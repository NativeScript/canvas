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

@implementation NSCAnalyserNode {
    AVAudioMixerNode *_mixer;

    float *_timeRing;
    NSInteger _timeRingCapacity;
    atomic_int_least64_t _timeRingWriteIdx;
    atomic_uint_least64_t _framesWritten;

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

    BOOL _tapInstalled;
    AVAudioFormat *_tapFormat;
    _Atomic(bool) _acceptingFrames;

    int _quantumFrames;
}

#pragma mark - Setup / teardown

- (instancetype)initWithContext:(NSCAudioContext *)context {
    AVAudioMixerNode *mixer = [[AVAudioMixerNode alloc] init];
    [context.engine attachNode:mixer];
    if (self = [super initWithContext:context node:mixer]) {
        _mixer = mixer;
        _fftSize = 2048;
        _smoothingTimeConstant = 0.8;
        _minDecibels = -100.0;
        _maxDecibels = -30.0;
        _quantumFrames = 1024;
        _timeRingCapacity = _fftSize;
        _timeRing = (float *)calloc((size_t)_timeRingCapacity, sizeof(float));
        _smoothedMagCount = _fftSize / 2;
        _smoothedMag = (float *)calloc((size_t)_smoothedMagCount, sizeof(float));
        _dbCache = (float *)calloc((size_t)_smoothedMagCount, sizeof(float));
        _magLock = [NSLock new];
        atomic_init(&_timeRingWriteIdx, 0);
        atomic_init(&_framesWritten, 0);
        atomic_store(&_acceptingFrames, false);
        _spectrumFrameStamp = -1;
        _dbCacheFrameStamp = -1;
        [self rebuildFftCachesForSize:_fftSize];
    }
    return self;
}

- (void)dealloc {
    atomic_store_explicit(&_acceptingFrames, false, memory_order_relaxed);
    [self removeTapIfInstalled];
    if (_timeRing)    { free(_timeRing); _timeRing = NULL; }
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
    if (_window) vDSP_hann_window(_window, (vDSP_Length)n, vDSP_HANN_NORM);
    _spectrumFrameStamp = -1;
    _dbCacheFrameStamp = -1;
}

- (AVAudioNode *)mixerNode { return _mixer; }

- (void)setAcceptingFrames:(BOOL)accept {
    atomic_store_explicit(&_acceptingFrames, accept, memory_order_relaxed);
}

#pragma mark - Tap lifecycle

- (BOOL)hasAnyInputConnection {
    if (!_mixer) return NO;
    AVAudioEngine *engine = self.context.engine;
    if (!engine) return NO;
    if (![engine respondsToSelector:@selector(inputConnectionPointForNode:inputBus:)]) return YES;
    for (AVAudioNodeBus bus = 0; bus < (AVAudioNodeBus)_mixer.numberOfInputs; ++bus) {
        AVAudioConnectionPoint *p = nil;
        @try { p = [engine inputConnectionPointForNode:_mixer inputBus:bus]; }
        @catch (NSException *e) { p = nil; }
        if (p && p.node) return YES;
    }
    return NO;
}

- (AVAudioFormat *)resolveTapFormat {
    if (!_mixer) return nil;
    AVAudioFormat *fmt = nil;
    @try { fmt = [_mixer outputFormatForBus:0]; }
    @catch (NSException *e) { fmt = nil; }
    if (!fmt || fmt.sampleRate <= 0 || fmt.channelCount == 0) {
        AVAudioEngine *engine = self.context.engine;
        @try { fmt = engine ? [engine.mainMixerNode outputFormatForBus:0] : nil; }
        @catch (NSException *e) { fmt = nil; }
    }
    if (!fmt || fmt.sampleRate <= 0 || fmt.channelCount == 0) return nil;
    return fmt;
}

- (void)installTapIfNeeded {
    if (!_mixer || _tapInstalled) return;
    AVAudioFormat *fmt = [self resolveTapFormat];
    if (!fmt) return;

    __weak typeof(self) weakSelf = self;
    AVAudioMixerNode *mixer = _mixer;
    AVAudioNodeTapBlock tapBlock = ^(AVAudioPCMBuffer *buf, AVAudioTime *when) {
        __strong typeof(weakSelf) s = weakSelf;
        if (!s) return;
        if (!atomic_load_explicit(&s->_acceptingFrames, memory_order_relaxed)) return;
        float * const *cd = buf.floatChannelData;
        if (!cd) return;
        AVAudioFrameCount frames = buf.frameLength;
        if (frames == 0) return;
        AVAudioChannelCount channels = buf.format.channelCount;
        NSCAnalyser_appendToRing(s->_timeRing, s->_timeRingCapacity,
                                 &s->_timeRingWriteIdx, &s->_framesWritten,
                                 cd, channels, frames);
    };

    @try {
        [_mixer installTapOnBus:0
                     bufferSize:(AVAudioFrameCount)_quantumFrames
                         format:fmt
                          block:tapBlock];
        _tapInstalled = YES;
        _tapFormat = fmt;
        atomic_store_explicit(&_acceptingFrames, true, memory_order_release);
    } @catch (NSException *e) {
        NSCLogDebug(@"NSCAnalyserNode: installTapOnBus failed: %@ (mixer=%p sr=%.0f ch=%u)",
              e, mixer, fmt.sampleRate, (unsigned)fmt.channelCount);
    }
}

- (void)removeTapIfInstalled {
    atomic_store_explicit(&_acceptingFrames, false, memory_order_relaxed);
    if (!_tapInstalled || !_mixer) return;
    @try { [_mixer removeTapOnBus:0]; } @catch (NSException *e) {}
    _tapInstalled = NO;
    _tapFormat = nil;
}

#pragma mark - Connection hooks

- (void)handleConnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    [super handleConnectFrom:source output:output input:input];
    [self installTapIfNeeded];
}

- (void)handleConnectedTo:(NSCAudioNode *)destination output:(NSNumber *)output input:(NSNumber *)input {
    [super handleConnectedTo:destination output:output input:input];
    [self installTapIfNeeded];
}

- (void)handleDisconnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    [super handleDisconnectFrom:source output:output input:input];
    if (![self hasAnyInputConnection]) {
        [self resetAfterDisconnect];
    }
}

- (void)disconnect {
    [super disconnect];
    if (![self hasAnyInputConnection]) {
        [self resetAfterDisconnect];
    }
}

- (void)resetAfterDisconnect {
    [self removeTapIfInstalled];
    atomic_store_explicit(&_timeRingWriteIdx, 0, memory_order_relaxed);
    atomic_store_explicit(&_framesWritten, 0, memory_order_relaxed);
    _spectrumFrameStamp = -1;
    _dbCacheFrameStamp = -1;
    if (_smoothedMag) memset(_smoothedMag, 0, (size_t)_smoothedMagCount * sizeof(float));
    if (_dbCache)     memset(_dbCache, 0, (size_t)_smoothedMagCount * sizeof(float));
    if (_timeRing)    memset(_timeRing, 0, (size_t)_timeRingCapacity * sizeof(float));
}

#pragma mark - Properties

- (NSInteger)frequencyBinCount { return _fftSize / 2; }

- (void)setFftSize:(NSInteger)fftSize {
    if (fftSize < 32) fftSize = 32;
    if (fftSize > 32768) fftSize = 32768;
    NSInteger p = 1; while (p < fftSize) p <<= 1;
    if (p == _fftSize && _dftSetupSize == p) return;

    BOOL hadTap = _tapInstalled;
    if (hadTap) [self removeTapIfInstalled];

    [_magLock lock];
    _fftSize = p;
    _timeRingCapacity = p;
    if (_timeRing) free(_timeRing);
    _timeRing = (float *)calloc((size_t)_timeRingCapacity, sizeof(float));
    atomic_store_explicit(&_timeRingWriteIdx, 0, memory_order_relaxed);
    atomic_store_explicit(&_framesWritten, 0, memory_order_relaxed);
    if (_smoothedMag) free(_smoothedMag);
    if (_dbCache) free(_dbCache);
    _smoothedMagCount = p / 2;
    _smoothedMag = (float *)calloc((size_t)_smoothedMagCount, sizeof(float));
    _dbCache = (float *)calloc((size_t)_smoothedMagCount, sizeof(float));
    _spectrumFrameStamp = -1;
    _dbCacheFrameStamp = -1;
    [self rebuildFftCachesForSize:p];
    [_magLock unlock];

    if (hadTap) [self installTapIfNeeded];
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
    int64_t widx = atomic_load_explicit(&_timeRingWriteIdx, memory_order_acquire);
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
    if (!data) return;
    float *dst = (float *)data.mutableBytes;
    if (!dst) return;
    NSInteger n = MIN((NSInteger)(data.length / sizeof(float)), _fftSize);
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
    int64_t widx = atomic_load_explicit(&_timeRingWriteIdx, memory_order_acquire);

    [_magLock lock];
    if (_spectrumFrameStamp == widx) { [_magLock unlock]; return; }

    [self copyLatestTimeSamplesInto:_samplesScratch count:n];
    vDSP_vmul(_samplesScratch, 1, _window, 1, _samplesScratch, 1, (vDSP_Length)n);

    DSPSplitComplex inSplit = { _realIn, _imagIn };
    vDSP_ctoz((const DSPComplex *)_samplesScratch, 2, &inSplit, 1, (vDSP_Length)half);
    vDSP_DFT_Execute(_dftSetup, _realIn, _imagIn, _realOut, _imagOut);
    DSPSplitComplex c = { _realOut, _imagOut };
    vDSP_zvmags(&c, 1, _magScratch, 1, (vDSP_Length)half);

    float scale = 1.0f / (float)(n * n);
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
    vDSP_vdbcon(_dbCache, 1, &ref, _dbCache, 1, (vDSP_Length)lim, 0);
    _dbCacheFrameStamp = _spectrumFrameStamp;
}

- (void)getFloatFrequencyData:(NSMutableData *)data {
    if (!data) return;
    float *dst = (float *)data.mutableBytes;
    if (!dst) return;
    [self computeMagnitudeSpectrumSmoothed];
    NSInteger n = MIN((NSInteger)(data.length / sizeof(float)), _smoothedMagCount);
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
    if (!buffer) return;
    if (!atomic_load_explicit(&_acceptingFrames, memory_order_relaxed)) return;
    float * const *cd = buffer.floatChannelData;
    if (!cd) return;
    AVAudioFrameCount frames = buffer.frameLength;
    if (frames == 0) return;
    NSCAnalyser_appendToRing(_timeRing, _timeRingCapacity,
                             &_timeRingWriteIdx, &_framesWritten,
                             cd, buffer.format.channelCount, frames);
}

@end
