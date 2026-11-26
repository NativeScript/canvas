#import <Accelerate/Accelerate.h>
#import "NSCAudioLog.h"
#import <stdatomic.h>
#import <math.h>

#import "NSCAudioContext.h"

static inline void NSCAnalyser_appendToRing(float *ring,
                                            NSInteger cap,
                                            atomic_int_least64_t *widxPtr,
                                            atomic_uint_least64_t *framesWrittenPtr,
                                            float * const *channelData,
                                            AVAudioChannelCount channels,
                                            AVAudioFrameCount frames) {
    if (!ring || cap <= 0 || frames == 0 || !channelData) return;
    NSInteger capMask = cap - 1;
    int64_t widx = atomic_load_explicit(widxPtr, memory_order_relaxed);

    AVAudioFrameCount remaining = frames;
    AVAudioFrameCount srcOffset = 0;

    while (remaining > 0) {
        NSInteger idx = (NSInteger)(widx & (int64_t)capMask);
        NSInteger room = cap - idx;
        NSInteger chunk = (NSInteger)remaining < room ? (NSInteger)remaining : room;
        float *dest = ring + idx;

        if (channels == 1 || !channelData[1]) {
            memcpy(dest, channelData[0] + srcOffset, (size_t)chunk * sizeof(float));
        } else {
            vDSP_vadd(channelData[0] + srcOffset, 1,
                      channelData[1] + srcOffset, 1,
                      dest, 1, (vDSP_Length)chunk);
            for (AVAudioChannelCount c = 2; c < channels; ++c) {
                if (!channelData[c]) continue;
                vDSP_vadd(dest, 1, channelData[c] + srcOffset, 1,
                          dest, 1, (vDSP_Length)chunk);
            }
            float scale = 1.0f / (float)channels;
            vDSP_vsmul(dest, 1, &scale, dest, 1, (vDSP_Length)chunk);
        }

        widx += chunk;
        srcOffset += chunk;
        remaining -= chunk;
    }

    atomic_store_explicit(widxPtr, widx, memory_order_release);
    if (framesWrittenPtr) {
        atomic_fetch_add_explicit(framesWrittenPtr, frames, memory_order_relaxed);
    }
}

static inline NSUInteger NSCAnalyserNormalizeFloatByteOffset(NSInteger byteOffset, NSUInteger length) {
    NSUInteger offset = byteOffset > 0 ? (NSUInteger)byteOffset : 0;
    if (offset > length) offset = length;
    offset -= (offset % sizeof(float));
    return offset;
}

#define ANALYSER_RING 32768

// The last 32768 frames, down-mixed to mono; written on the render thread.
@interface NSCAnalyserRing : NSObject {
@public
    float ring[ANALYSER_RING];
    atomic_int_least64_t writeIndex;
    atomic_uint_least64_t framesWritten;
}
@end

@implementation NSCAnalyserRing
@end

@implementation NSCAnalyserNode {
    NSCAnalyserRing *_ring;
    float *_timeRing;
    NSInteger _timeRingCapacity;

    float *_smoothedMag;
    float *_dbCache;
    NSInteger _smoothedMagCount;
    NSLock *_magLock;

    vDSP_DFT_Setup _dftSetup;
    NSInteger _dftSetupSize;
    float *_window;
    float *_samplesScratch;
    float *_realIn;
    float *_imagIn;
    float *_realOut;
    float *_imagOut;
    float *_magScratch;

    int64_t _spectrumFrameStamp;
    int64_t _dbCacheFrameStamp;
}

#pragma mark - Setup / teardown

- (instancetype)initWithContext:(NSCAudioContext *)context {
    if (!(self = [super initWithContext:context outputChannels:2])) return nil;
    _fftSize = 2048;
    _smoothingTimeConstant = 0.8;
    _minDecibels = -100.0;
    _maxDecibels = -30.0;
    _ring = [NSCAnalyserRing new];
    _timeRing = _ring->ring;
    _timeRingCapacity = ANALYSER_RING;
    _smoothedMagCount = _fftSize / 2;
    _smoothedMag = (float *)calloc((size_t)_smoothedMagCount, sizeof(float));
    _dbCache = (float *)calloc((size_t)_smoothedMagCount, sizeof(float));
    _magLock = [NSLock new];
    _spectrumFrameStamp = -1;
    _dbCacheFrameStamp = -1;
    [self rebuildFftCachesForSize:_fftSize];
    NSCAnalyserRing *ring = _ring;
    [self setProcess:^UInt32(int64_t frame, UInt32 frames, float *const *in, UInt32 inChannels, float *const *out, UInt32 outChannels) {
        NSCAnalyser_appendToRing(ring->ring, ANALYSER_RING, &ring->writeIndex, &ring->framesWritten, in, inChannels, frames);
        UInt32 channels = MIN(inChannels, outChannels);
        for (UInt32 c = 0; c < channels; c++) memcpy(out[c], in[c], frames * sizeof(float));
        return channels;
    }];
    [self keepPulled];
    return self;
}

- (void)dealloc {
    if (_smoothedMag) { free(_smoothedMag); _smoothedMag = NULL; }
    if (_dbCache)     { free(_dbCache); _dbCache = NULL; }
    [self freeFftCaches];
}

- (void)freeFftCaches {
    if (_dftSetup)       { vDSP_DFT_DestroySetup(_dftSetup); _dftSetup = NULL; }
    if (_window)         { free(_window); _window = NULL; }
    if (_samplesScratch) { free(_samplesScratch); _samplesScratch = NULL; }
    if (_realIn)         { free(_realIn); _realIn = NULL; }
    if (_imagIn)         { free(_imagIn); _imagIn = NULL; }
    if (_realOut)        { free(_realOut); _realOut = NULL; }
    if (_imagOut)        { free(_imagOut); _imagOut = NULL; }
    if (_magScratch)     { free(_magScratch); _magScratch = NULL; }
    _dftSetupSize = 0;
}

- (void)rebuildFftCachesForSize:(NSInteger)n {
    [self freeFftCaches];
    NSInteger half = n / 2;
    _dftSetup = vDSP_DFT_zrop_CreateSetup(NULL, (vDSP_Length)n, vDSP_DFT_FORWARD);
    _dftSetupSize = n;
    _window         = (float *)malloc((size_t)n * sizeof(float));
    _samplesScratch = (float *)malloc((size_t)n * sizeof(float));
    _realIn         = (float *)malloc((size_t)half * sizeof(float));
    _imagIn         = (float *)malloc((size_t)half * sizeof(float));
    _realOut        = (float *)malloc((size_t)half * sizeof(float));
    _imagOut        = (float *)malloc((size_t)half * sizeof(float));
    _magScratch     = (float *)malloc((size_t)half * sizeof(float));
    if (_window) vDSP_blkman_window(_window, (vDSP_Length)n, 0);
    _spectrumFrameStamp = -1;
    _dbCacheFrameStamp = -1;
}

#pragma mark - Properties

- (NSInteger)frequencyBinCount { return _fftSize / 2; }

- (void)setFftSize:(NSInteger)fftSize {
    if (fftSize < 32) fftSize = 32;
    if (fftSize > 32768) fftSize = 32768;
    NSInteger p = 1; while (p < fftSize) p <<= 1;
    if (p == _fftSize && _dftSetupSize == p) return;
    [_magLock lock];
    _fftSize = p;
    if (_smoothedMag) free(_smoothedMag);
    if (_dbCache) free(_dbCache);
    _smoothedMagCount = p / 2;
    _smoothedMag = (float *)calloc((size_t)_smoothedMagCount, sizeof(float));
    _dbCache = (float *)calloc((size_t)_smoothedMagCount, sizeof(float));
    _spectrumFrameStamp = -1;
    _dbCacheFrameStamp = -1;
    [self rebuildFftCachesForSize:p];
    [_magLock unlock];
}

- (void)setSmoothingTimeConstant:(double)smoothingTimeConstant {
    if (smoothingTimeConstant < 0.0) smoothingTimeConstant = 0.0;
    if (smoothingTimeConstant > 1.0) smoothingTimeConstant = 1.0;
    _smoothingTimeConstant = smoothingTimeConstant;
    _spectrumFrameStamp = -1;
    _dbCacheFrameStamp = -1;
}

#pragma mark - Read-out helpers

- (void)copyLatestTimeSamplesInto:(float *)dst count:(NSInteger)n {
    NSInteger cap = _timeRingCapacity;
    if (cap <= 0 || n <= 0) return;
    int64_t widx = atomic_load_explicit(&_ring->writeIndex, memory_order_acquire);
    int64_t start = widx - n;
    NSInteger leadZeros = (start < 0) ? (NSInteger)MIN((int64_t)n, -start) : 0;
    if (leadZeros > 0) memset(dst, 0, (size_t)leadZeros * sizeof(float));
    NSInteger filled = leadZeros;
    int64_t firstSrc = (start < 0) ? 0 : start;
    NSInteger idx = (NSInteger)(firstSrc & (cap - 1));
    NSInteger remaining = n - filled;
    while (remaining > 0) {
        NSInteger chunk = MIN(remaining, cap - idx);
        memcpy(dst + filled, _timeRing + idx, (size_t)chunk * sizeof(float));
        filled += chunk;
        remaining -= chunk;
        idx = (idx + chunk) & (cap - 1);
    }
}

- (void)getFloatTimeDomainData:(NSMutableData *)data {
    [self getFloatTimeDomainDataWithByteOffset:data :0];
}

- (void)getFloatTimeDomainDataWithByteOffset:(NSMutableData *)data :(NSInteger)byteOffset {
    if (!data) return;
    NSUInteger offsetBytes = NSCAnalyserNormalizeFloatByteOffset(byteOffset, data.length);
    if (offsetBytes >= data.length) return;
    uint8_t *bytes = (uint8_t *)data.mutableBytes;
    if (!bytes) return;
    float *dst = (float *)(bytes + offsetBytes);
    NSInteger n = MIN((NSInteger)((data.length - offsetBytes) / sizeof(float)), _fftSize);
    if (n <= 0) return;
    [self copyLatestTimeSamplesInto:dst count:n];
}

- (void)getByteTimeDomainData:(NSMutableData *)data {
    if (!data) return;
    uint8_t *dst = (uint8_t *)data.mutableBytes;
    if (!dst) return;
    NSInteger n = MIN((NSInteger)data.length, _fftSize);
    if (n <= 0) return;
    float *tmp = _samplesScratch;
    if (!tmp || _dftSetupSize < n) {
        [self rebuildFftCachesForSize:_fftSize];
        tmp = _samplesScratch;
        if (!tmp) return;
    }
    [self copyLatestTimeSamplesInto:tmp count:n];
    float scale = 127.5f, bias = 127.5f;
    vDSP_vsmsa(tmp, 1, &scale, &bias, tmp, 1, (vDSP_Length)n);
    float lo = 0.0f, hi = 255.0f;
    vDSP_vclip(tmp, 1, &lo, &hi, tmp, 1, (vDSP_Length)n);
    vDSP_vfixu8(tmp, 1, dst, 1, (vDSP_Length)n);
}

#pragma mark - Frequency-domain

- (void)computeMagnitudeSpectrumSmoothed {
    NSInteger n = _fftSize;
    NSInteger half = n / 2;
    if (n <= 0 || !_dftSetup || _dftSetupSize != n || !_window || !_samplesScratch) {
        [self rebuildFftCachesForSize:n];
        if (!_dftSetup) return;
    }
    int64_t widx = atomic_load_explicit(&_ring->writeIndex, memory_order_acquire);

    [_magLock lock];
    if (_spectrumFrameStamp == widx) { [_magLock unlock]; return; }

    [self copyLatestTimeSamplesInto:_samplesScratch count:n];
    vDSP_vmul(_samplesScratch, 1, _window, 1, _samplesScratch, 1, (vDSP_Length)n);

    DSPSplitComplex inSplit = { _realIn, _imagIn };
    vDSP_ctoz((const DSPComplex *)_samplesScratch, 2, &inSplit, 1, (vDSP_Length)half);
    vDSP_DFT_Execute(_dftSetup, _realIn, _imagIn, _realOut, _imagOut);
    DSPSplitComplex c = { _realOut, _imagOut };
    vDSP_zvmags(&c, 1, _magScratch, 1, (vDSP_Length)half);
    // Bin 0's imaginary slot holds the Nyquist bin.
    _magScratch[0] = _realOut[0] * _realOut[0];
    int count = (int)half;
    vvsqrtf(_magScratch, _magScratch, &count);
    // |X[k]| / N; vDSP's real DFT is twice the DFT.
    float scale = 0.5f / (float)n;
    vDSP_vsmul(_magScratch, 1, &scale, _magScratch, 1, (vDSP_Length)half);

    float tau = (float)MAX(0.0, MIN(1.0, _smoothingTimeConstant));
    float oneMinus = 1.0f - tau;
    NSInteger lim = MIN(_smoothedMagCount, half);
    if (tau == 0.0f) {
        memcpy(_smoothedMag, _magScratch, (size_t)lim * sizeof(float));
    } else {
        vDSP_vsmul(_smoothedMag, 1, &tau, _smoothedMag, 1, (vDSP_Length)lim);
        vDSP_vsma(_magScratch, 1, &oneMinus, _smoothedMag, 1, _smoothedMag, 1, (vDSP_Length)lim);
    }
    _spectrumFrameStamp = widx;
    _dbCacheFrameStamp = -1;
    [_magLock unlock];
}

- (void)ensureDbCacheLocked {
    if (_dbCacheFrameStamp == _spectrumFrameStamp || !_dbCache) return;
    NSInteger lim = _smoothedMagCount;
    float floor_ = 1e-20f;
    vDSP_vthr(_smoothedMag, 1, &floor_, _dbCache, 1, (vDSP_Length)lim);
    float ref = 1.0f;
    vDSP_vdbcon(_dbCache, 1, &ref, _dbCache, 1, (vDSP_Length)lim, 1);
    _dbCacheFrameStamp = _spectrumFrameStamp;
}

- (void)getFloatFrequencyData:(NSMutableData *)data {
    [self getFloatFrequencyDataWithByteOffset:data :0];
}

- (void)getFloatFrequencyDataWithByteOffset:(NSMutableData *)data :(NSInteger)byteOffset {
    if (!data) return;
    NSUInteger offsetBytes = NSCAnalyserNormalizeFloatByteOffset(byteOffset, data.length);
    if (offsetBytes >= data.length) return;
    uint8_t *bytes = (uint8_t *)data.mutableBytes;
    if (!bytes) return;
    float *dst = (float *)(bytes + offsetBytes);
    [self computeMagnitudeSpectrumSmoothed];
    NSInteger n = MIN((NSInteger)((data.length - offsetBytes) / sizeof(float)), _smoothedMagCount);
    if (n <= 0) return;
    [_magLock lock];
    [self ensureDbCacheLocked];
    memcpy(dst, _dbCache, (size_t)n * sizeof(float));
    [_magLock unlock];
}

- (void)getByteFrequencyData:(NSMutableData *)data {
    if (!data) return;
    uint8_t *dst = (uint8_t *)data.mutableBytes;
    if (!dst) return;
    [self computeMagnitudeSpectrumSmoothed];
    NSInteger n = MIN((NSInteger)data.length, _smoothedMagCount);
    if (n <= 0) return;
    float lo = (float)_minDecibels, hi = (float)_maxDecibels;
    float range = hi - lo;
    if (range <= 0.0f) range = 1.0f;
    const float invRange = 1.0f / range;
    const float scale = 255.0f * invRange;
    const float bias = -lo * scale;
    float clipLo = 0.0f, clipHi = 255.0f;

    float *tmp = _magScratch;
    [_magLock lock];
    [self ensureDbCacheLocked];
    vDSP_vsmsa(_dbCache, 1, &scale, &bias, tmp, 1, (vDSP_Length)n);
    vDSP_vclip(tmp, 1, &clipLo, &clipHi, tmp, 1, (vDSP_Length)n);
    vDSP_vfixu8(tmp, 1, dst, 1, (vDSP_Length)n);
    [_magLock unlock];
}

- (void)appendBufferToRing:(AVAudioPCMBuffer *)buffer {
    if (!buffer.floatChannelData || buffer.frameLength == 0) return;
    NSCAnalyser_appendToRing(_ring->ring, ANALYSER_RING, &_ring->writeIndex, &_ring->framesWritten,
                             buffer.floatChannelData, buffer.format.channelCount, buffer.frameLength);
}

@end
