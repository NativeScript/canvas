#import "NSCAudioContext.h"
#import <stdatomic.h>

#define WS_CHANNELS 8
#define WS_TAPS 32

static inline NSUInteger NSCWaveShaperNormalizeFloatByteOffset(NSInteger byteOffset, NSUInteger length) {
    NSUInteger offset = byteOffset > 0 ? (NSUInteger)byteOffset : 0;
    if (offset > length) offset = length;
    offset -= (offset % sizeof(float));
    return offset;
}

// One 2x resampling stage: a windowed-sinc half-band low-pass, with history per channel.
typedef struct {
    float up[WS_CHANNELS][WS_TAPS];
    float down[WS_CHANNELS][WS_TAPS];
} WSStage;

typedef struct {
    // A curve: its length in the first 4 bytes, then the floats from byte 16.
    _Atomic(const void *) curve;
    _Atomic int oversample;
    WSStage stages[2];
    float kernel[WS_TAPS];
} WSState;

static void MakeKernel(float *k) {
    double sum = 0.0;
    for (int i = 0; i < WS_TAPS; i++) {
        double n = i - (WS_TAPS - 1) / 2.0;
        double sinc = n == 0.0 ? 1.0 : sin(M_PI * n * 0.5) / (M_PI * n * 0.5);
        double w = 0.42 - 0.5 * cos(2 * M_PI * i / (WS_TAPS - 1)) + 0.08 * cos(4 * M_PI * i / (WS_TAPS - 1));
        k[i] = (float)(sinc * w);
        sum += k[i];
    }
    for (int i = 0; i < WS_TAPS; i++) k[i] = (float)(k[i] / sum);
}

static float Fir(float *hist, const float *k, float x) {
    memmove(hist, hist + 1, (WS_TAPS - 1) * sizeof(float));
    hist[WS_TAPS - 1] = x;
    float y = 0.0f;
    for (int i = 0; i < WS_TAPS; i++) y += hist[i] * k[i];
    return y;
}

static float Shape(const float *curve, UInt32 n, float x) {
    if (!curve || n == 0) return x;
    double v = (n - 1) * 0.5 * ((double)x + 1.0);
    if (!(v > 0.0)) return curve[0];
    if (v >= n - 1) return curve[n - 1];
    UInt32 k = (UInt32)v;
    double f = v - k;
    return (float)((1.0 - f) * curve[k] + f * curve[k + 1]);
}

// Up-samples x by 2^level, shapes, and down-samples back.
static float ShapeOversampled(WSState *s, int level, int c, const float *curve, UInt32 n, float x) {
    if (level == 0) return Shape(curve, n, x);
    WSStage *st = &s->stages[level - 1];
    float a = Fir(st->up[c], s->kernel, 2.0f * x);
    float b = Fir(st->up[c], s->kernel, 0.0f);
    float ya = ShapeOversampled(s, level - 1, c, curve, n, a);
    float yb = ShapeOversampled(s, level - 1, c, curve, n, b);
    Fir(st->down[c], s->kernel, ya);
    return Fir(st->down[c], s->kernel, yb);
}

@implementation NSCWaveShaperNode {
    NSMutableData *_state;
    NSData *_curve;
    NSMutableArray<NSMutableData *> *_curves;
}

- (instancetype)initWithContext:(NSCAudioContext *)context {
    if (!(self = [super initWithContext:context outputChannels:2])) return nil;
    _oversample = @"none";
    _curves = [NSMutableArray array];
    _state = [NSMutableData dataWithLength:sizeof(WSState)];
    WSState *s = (WSState *)_state.mutableBytes;
    MakeKernel(s->kernel);
    NSMutableData *keep = _state;
    [self setProcess:^UInt32(int64_t frame, UInt32 frames, float *const *in, UInt32 inChannels, float *const *out, UInt32 outChannels) {
        WSState *st = (WSState *)keep.mutableBytes;
        const void *packed = atomic_load_explicit(&st->curve, memory_order_acquire);
        UInt32 n = packed ? *(const UInt32 *)packed : 0;
        const float *curve = packed ? (const float *)((const uint8_t *)packed + 16) : NULL;
        int level = atomic_load_explicit(&st->oversample, memory_order_relaxed);
        UInt32 channels = MIN(inChannels, MIN(outChannels, (UInt32)WS_CHANNELS));
        for (UInt32 c = 0; c < channels; c++) {
            for (UInt32 k = 0; k < frames; k++) out[c][k] = curve ? ShapeOversampled(st, level, (int)c, curve, n, in[c][k]) : in[c][k];
        }
        return channels;
    }];
    return self;
}

- (void)setOversample:(NSString *)oversample {
    _oversample = [oversample isEqualToString:@"2x"] || [oversample isEqualToString:@"4x"] ? [oversample copy] : @"none";
    WSState *s = (WSState *)_state.mutableBytes;
    int level = [_oversample isEqualToString:@"4x"] ? 2 : [_oversample isEqualToString:@"2x"] ? 1 : 0;
    atomic_store(&s->oversample, level);
}

- (void)setCurveFromData:(NSData *)floatData {
    [self setCurveFromDataWithByteOffset:floatData :0];
}

- (void)setCurveFromDataWithByteOffset:(NSData *)floatData :(NSInteger)byteOffset {
    WSState *s = (WSState *)_state.mutableBytes;
    if (!floatData) {
        _curve = nil;
        atomic_store(&s->curve, NULL);
        return;
    }
    NSUInteger offsetBytes = NSCWaveShaperNormalizeFloatByteOffset(byteOffset, floatData.length);
    NSData *curve = [floatData subdataWithRange:NSMakeRange(offsetBytes, floatData.length - offsetBytes)];
    _curve = curve;
    NSMutableData *packed = [NSMutableData dataWithLength:16];
    *(UInt32 *)packed.mutableBytes = (UInt32)(curve.length / sizeof(float));
    [packed appendData:curve];
    // The render thread may still read an earlier curve.
    [_curves addObject:packed];
    atomic_store(&s->curve, packed.bytes);
}

- (NSData *)curve { return _curve; }

@end
