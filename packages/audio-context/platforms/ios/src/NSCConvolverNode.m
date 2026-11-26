#import "NSCAudioContext.h"
#import <Accelerate/Accelerate.h>
#import <stdatomic.h>

#define CONV_BLOCK 128
#define CONV_FFT 256
#define CONV_LOG2 8

// Uniformly partitioned overlap-save convolution, a quantum per partition, so no latency.
// Spectra are vDSP packed: 128 real then 128 imaginary, DC and Nyquist in bin 0.
typedef struct {
    int irChannels;
    int parts;
    float *H;
    float *fdl;
    float prev[2][CONV_BLOCK];
    int head;
} ConvKernel;

static float *Spectrum(float *base, int index) { return base + (size_t)index * CONV_FFT; }

static void ForwardFFT(FFTSetup setup, const float *time, float *spectrum) {
    DSPSplitComplex split = { spectrum, spectrum + CONV_BLOCK };
    vDSP_ctoz((const DSPComplex *)time, 2, &split, 1, CONV_BLOCK);
    vDSP_fft_zrip(setup, &split, 1, CONV_LOG2, kFFTDirection_Forward);
}

// acc += a * b, keeping the packed DC and Nyquist bins apart.
static void MultiplyAdd(const float *a, const float *b, float *acc) {
    float dc = acc[0] + a[0] * b[0], ny = acc[CONV_BLOCK] + a[CONV_BLOCK] * b[CONV_BLOCK];
    DSPSplitComplex A = { (float *)a, (float *)a + CONV_BLOCK }, B = { (float *)b, (float *)b + CONV_BLOCK }, C = { acc, acc + CONV_BLOCK };
    vDSP_zvma(&A, 1, &B, 1, &C, 1, &C, 1, CONV_BLOCK);
    acc[0] = dc;
    acc[CONV_BLOCK] = ny;
}

static void FreeKernel(ConvKernel *k) {
    if (!k) return;
    free(k->H);
    free(k->fdl);
    free(k);
}

// The spec's equal-power normalization scale.
static float NormalizationScale(AVAudioPCMBuffer *pcm) {
    double power = 0.0;
    AVAudioChannelCount channels = pcm.format.channelCount;
    AVAudioFrameCount length = pcm.frameLength;
    for (AVAudioChannelCount c = 0; c < channels; c++) {
        float sum = 0.0f;
        vDSP_svesq(pcm.floatChannelData[c], 1, &sum, length);
        power += sum;
    }
    power = sqrt(power / ((double)channels * MAX(1u, length)));
    if (!isfinite(power) || power < 0.000125) power = 0.000125;
    double scale = 1.0 / power * 0.00125 * 44100.0 / pcm.format.sampleRate;
    if (channels == 4) scale *= 0.5;
    return (float)scale;
}

static ConvKernel *MakeKernel(FFTSetup setup, AVAudioPCMBuffer *pcm, BOOL normalize) {
    AVAudioChannelCount channels = pcm.format.channelCount;
    if (!pcm.floatChannelData || (channels != 1 && channels != 2 && channels != 4)) return NULL;
    ConvKernel *k = calloc(1, sizeof(ConvKernel));
    k->irChannels = (int)channels;
    k->parts = (int)MAX(1u, (pcm.frameLength + CONV_BLOCK - 1) / CONV_BLOCK);
    k->H = calloc((size_t)channels * k->parts * CONV_FFT, sizeof(float));
    k->fdl = calloc((size_t)2 * k->parts * CONV_FFT, sizeof(float));
    if (!k->H || !k->fdl) {
        FreeKernel(k);
        return NULL;
    }
    // vDSP's forward transforms are twice the DFT and its inverse N times the inverse DFT.
    float scale = (normalize ? NormalizationScale(pcm) : 1.0f) / (4.0f * CONV_FFT);
    float block[CONV_FFT];
    for (AVAudioChannelCount c = 0; c < channels; c++) {
        const float *ir = pcm.floatChannelData[c];
        for (int p = 0; p < k->parts; p++) {
            memset(block, 0, sizeof(block));
            AVAudioFrameCount from = (AVAudioFrameCount)p * CONV_BLOCK;
            AVAudioFrameCount n = MIN((AVAudioFrameCount)CONV_BLOCK, pcm.frameLength - from);
            vDSP_vsmul(ir + from, 1, &scale, block, 1, n);
            ForwardFFT(setup, block, Spectrum(k->H, (int)c * k->parts + p));
        }
    }
    return k;
}

// One quantum: in[0..inputs) through the response into out[0..outputs).
static UInt32 Convolve(FFTSetup setup, ConvKernel *k, const float *const *in, UInt32 inputs, float *const *out) {
    float window[CONV_FFT], acc[2][CONV_FFT], time[CONV_FFT];
    int P = k->parts;
    k->head = (k->head + 1) % P;
    for (UInt32 c = 0; c < 2; c++) {
        const float *x = in[MIN(c, inputs - 1)];
        memcpy(window, k->prev[c], sizeof(k->prev[c]));
        memcpy(window + CONV_BLOCK, x, CONV_BLOCK * sizeof(float));
        memcpy(k->prev[c], x, CONV_BLOCK * sizeof(float));
        ForwardFFT(setup, window, Spectrum(k->fdl, (int)c * P + k->head));
    }
    // Output channel o sums input i through response channel `paths[o][i]`.
    int irs = k->irChannels;
    UInt32 outputs = irs == 1 && inputs == 1 ? 1 : 2;
    int paths[2][2] = { { -1, -1 }, { -1, -1 } };
    if (irs == 1) { paths[0][0] = 0; paths[1][1] = 0; }
    else if (irs == 2) { paths[0][0] = 0; paths[1][inputs == 1 ? 0 : 1] = 1; }
    else { paths[0][0] = 0; paths[0][1] = 2; paths[1][0] = 1; paths[1][1] = 3; }
    for (UInt32 o = 0; o < outputs; o++) {
        memset(acc[o], 0, sizeof(acc[o]));
        for (UInt32 i = 0; i < 2; i++) {
            int h = paths[o][i];
            if (h < 0) continue;
            for (int p = 0; p < P; p++) {
                int slot = (k->head - p + P) % P;
                MultiplyAdd(Spectrum(k->fdl, (int)i * P + slot), Spectrum(k->H, h * P + p), acc[o]);
            }
        }
        DSPSplitComplex split = { acc[o], acc[o] + CONV_BLOCK };
        vDSP_fft_zrip(setup, &split, 1, CONV_LOG2, kFFTDirection_Inverse);
        vDSP_ztoc(&split, 1, (DSPComplex *)time, 2, CONV_BLOCK);
        memcpy(out[o], time + CONV_BLOCK, CONV_BLOCK * sizeof(float));
    }
    return outputs;
}

// Owned by the render block, so kernels outlive any render still using them.
@interface NSCConvolverState : NSObject {
@public
    _Atomic(ConvKernel *) kernel;
    FFTSetup setup;
    NSMutableArray<NSValue *> *kernels;
}
@end

@implementation NSCConvolverState
- (instancetype)init {
    if ((self = [super init])) {
        setup = vDSP_create_fftsetup(CONV_LOG2, kFFTRadix2);
        kernels = [NSMutableArray array];
    }
    return self;
}
- (void)dealloc {
    for (NSValue *v in kernels) FreeKernel((ConvKernel *)v.pointerValue);
    if (setup) vDSP_destroy_fftsetup(setup);
}
@end

@implementation NSCConvolverNode {
    NSCConvolverState *_state;
}

- (instancetype)initWithContext:(NSCAudioContext *)context {
    if (!(self = [super initWithContext:context outputChannels:2])) return nil;
    _normalize = YES;
    _state = [NSCConvolverState new];
    [self setChannelConfig:2 mode:1 interpretation:0];
    NSCConvolverState *st = _state;
    [self setProcess:^UInt32(int64_t frame, UInt32 frames, float *const *in, UInt32 inChannels, float *const *out, UInt32 outChannels) {
        ConvKernel *k = atomic_load_explicit(&st->kernel, memory_order_acquire);
        if (!k) return 1;
        UInt32 inputs = MIN(inChannels, 2u), wrote = 1;
        for (UInt32 at = 0; at + CONV_BLOCK <= frames; at += CONV_BLOCK) {
            const float *ins[2] = { in[0] + at, in[MIN(1u, inputs - 1)] + at };
            float *outs[2] = { out[0] + at, out[1] + at };
            wrote = Convolve(st->setup, k, ins, inputs, outs);
        }
        return wrote;
    }];
    return self;
}

- (void)setBuffer:(NSCAudioBuffer *)buffer {
    _buffer = buffer;
    AVAudioPCMBuffer *pcm = [buffer getBuffer];
    ConvKernel *kernel = pcm ? MakeKernel(_state->setup, pcm, _normalize) : NULL;
    if (kernel) [_state->kernels addObject:[NSValue valueWithPointer:kernel]];
    atomic_store_explicit(&_state->kernel, kernel, memory_order_release);
}

@end
