#import <stdatomic.h>
#import <math.h>
#import <objc/runtime.h>
#import "NSCAudioContext.h"

typedef NS_ENUM(uint8_t, NSCOscWaveform) {
    NSCOscWaveformSine = 0,
    NSCOscWaveformSquare = 1,
    NSCOscWaveformSawtooth = 2,
    NSCOscWaveformTriangle = 3,
};

static inline NSCOscWaveform NSCOscWaveformFromString(NSString * _Nullable type) {
    if ([type isEqualToString:@"square"]) return NSCOscWaveformSquare;
    if ([type isEqualToString:@"sawtooth"]) return NSCOscWaveformSawtooth;
    if ([type isEqualToString:@"triangle"]) return NSCOscWaveformTriangle;
    return NSCOscWaveformSine;
}

@implementation NSCAudioOscillatorNode {
    AVAudioSourceNode *_sourceNode;
    NSMutableData *_playingFlagData;
    NSMutableData *_startSampleData;
    NSMutableData *_stopSampleData;
    NSMutableData *_waveTypeData;
    atomic_uint_least8_t *_playingFlag;
    atomic_int_least64_t *_startSamplePtr;
    atomic_int_least64_t *_stopSamplePtr;
    atomic_uint_least8_t *_waveTypePtr;
    double _phase;
}

- (void)dealloc {
    NSCAudioContext *ctx = self.context;
    if (ctx && _sourceNode) {
        AVAudioEngine *eng = ctx.engine;
        if (eng && [ctx isNode:_sourceNode attachedToEngine:eng]) {
            @try { [eng disconnectNodeOutput:_sourceNode]; } @catch (NSException *e) {}
            @try { [eng disconnectNodeInput:_sourceNode]; } @catch (NSException *e) {}
            @try { [ctx detachNode:_sourceNode fromEngine:eng]; } @catch (NSException *e) {}
        }
    }
    _playingFlag = NULL;
    _startSamplePtr = NULL;
    _stopSamplePtr = NULL;
    _waveTypePtr = NULL;
}

- (instancetype)initWithContext:(NSCAudioContext *)context {
    return [self initWithContext:context type:@"sine" frequency:440.0];
}

- (instancetype)initWithContext:(NSCAudioContext *)context type:(NSString *)type frequency:(double)frequency {
    NSMutableData *playingData = [NSMutableData dataWithLength:sizeof(atomic_uint_least8_t)];
    NSMutableData *startSampleData = [NSMutableData dataWithLength:sizeof(atomic_int_least64_t)];
    NSMutableData *stopSampleData = [NSMutableData dataWithLength:sizeof(atomic_int_least64_t)];
    NSMutableData *waveTypeData = [NSMutableData dataWithLength:sizeof(atomic_uint_least8_t)];
    atomic_uint_least8_t *playingCell = (atomic_uint_least8_t *)playingData.mutableBytes;
    atomic_int_least64_t *startSampleCell = (atomic_int_least64_t *)startSampleData.mutableBytes;
    atomic_int_least64_t *stopSampleCell = (atomic_int_least64_t *)stopSampleData.mutableBytes;
    atomic_uint_least8_t *waveTypeCell = (atomic_uint_least8_t *)waveTypeData.mutableBytes;
    atomic_init(playingCell, 0);
    atomic_init(startSampleCell, (int_least64_t)-1);
    atomic_init(stopSampleCell, (int_least64_t)-1);
    atomic_init(waveTypeCell, (uint_least8_t)NSCOscWaveformFromString(type));

    NSCAudioParam *freqParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:frequency];
    NSCAudioParam *detuneParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:0.0];
    AVAudioFormat *format = context.format
    ?: [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32 sampleRate:44100.0 channels:1 interleaved:NO];

    __block double localPhase = 0.0;

    NSMutableData *capturedPlayingData = playingData;
    NSMutableData *capturedStartSampleData = startSampleData;
    NSMutableData *capturedStopSampleData = stopSampleData;
    NSMutableData *capturedWaveTypeData = waveTypeData;
    NSMutableData *timelineCell = context.timelineCell ?: [NSMutableData dataWithLength:NSCTimelineSlots * sizeof(double)];
    __weak NSCAudioOscillatorNode *weakSelf = self;

    AVAudioSourceNode *source = [[AVAudioSourceNode alloc] initWithFormat:format renderBlock:^OSStatus(BOOL * _Nonnull isSilence, const AudioTimeStamp * _Nonnull timestamp, AVAudioFrameCount frameCount, AudioBufferList * _Nonnull audioBufferList) {
        atomic_uint_least8_t *playingPtr = (atomic_uint_least8_t *)capturedPlayingData.mutableBytes;
        atomic_int_least64_t *startPtr = (atomic_int_least64_t *)capturedStartSampleData.mutableBytes;
        atomic_int_least64_t *stopPtr = (atomic_int_least64_t *)capturedStopSampleData.mutableBytes;
        atomic_uint_least8_t *wavePtr = (atomic_uint_least8_t *)capturedWaveTypeData.mutableBytes;

        if (isSilence) *isSilence = NO;
        BOOL playing = atomic_load_explicit(playingPtr, memory_order_acquire) == 1;
        if (!playing) {
            if (isSilence) *isSilence = YES;
            UInt32 bufCount = audioBufferList->mNumberBuffers;
            for (UInt32 b = 0; b < bufCount; ++b) {
                memset(audioBufferList->mBuffers[b].mData, 0, audioBufferList->mBuffers[b].mDataByteSize);
            }
            return noErr;
        }

        int64_t startSample = atomic_load_explicit(startPtr, memory_order_acquire);
        int64_t stopSample = atomic_load_explicit(stopPtr, memory_order_acquire);
        NSCOscWaveform waveEnum = (NSCOscWaveform)atomic_load_explicit(wavePtr, memory_order_acquire);

        double sampleRate = format.sampleRate;
        BOOL hasSampleTime = (timestamp && (timestamp->mFlags & kAudioTimeStampSampleTimeValid));
        double blockFrame = hasSampleTime ? NSCContextFrame(timestamp->mSampleTime, (const double *)timelineCell.bytes) : 0.0;

        UInt32 bufCount = audioBufferList->mNumberBuffers;
        float *out0 = (bufCount > 0) ? (float *)audioBufferList->mBuffers[0].mData : NULL;
        if (!out0) return noErr;

        AVAudioFrameCount firstRenderIndex = 0;
        AVAudioFrameCount endRenderIndex = frameCount;
        BOOL hitStop = NO;

        if (hasSampleTime) {
            int64_t blockStart = llround(blockFrame);
            int64_t blockEnd = blockStart + (int64_t)frameCount;
            if (startSample >= 0) {
                if (startSample >= blockEnd) {
                    if (isSilence) *isSilence = YES;
                    for (UInt32 b = 0; b < bufCount; ++b) memset(audioBufferList->mBuffers[b].mData, 0, audioBufferList->mBuffers[b].mDataByteSize);
                    return noErr;
                }
                if (startSample > blockStart) {
                    firstRenderIndex = (AVAudioFrameCount)(startSample - blockStart);
                    for (AVAudioFrameCount i = 0; i < firstRenderIndex; ++i) out0[i] = 0.0f;
                }
            }
            if (stopSample >= 0) {
                if (stopSample <= blockStart) {
                    endRenderIndex = 0;
                    hitStop = YES;
                } else if (stopSample < blockEnd) {
                    endRenderIndex = (AVAudioFrameCount)(stopSample - blockStart);
                    hitStop = YES;
                }
            }
        }

        if (endRenderIndex <= firstRenderIndex) {
            for (UInt32 b = 0; b < bufCount; ++b) {
                memset(audioBufferList->mBuffers[b].mData, 0, audioBufferList->mBuffers[b].mDataByteSize);
            }
            if (hitStop) {
                atomic_store_explicit(playingPtr, 0, memory_order_release);
                atomic_store_explicit(stopPtr, (int_least64_t)-1, memory_order_release);
                dispatch_async(dispatch_get_main_queue(), ^{
                    NSCAudioOscillatorNode *s = weakSelf;
                    if (!s) return;
                    if (s.context) [s.context decrementActiveCount];
                    if (s.onended) s.onended();
                });
            }
            if (isSilence) *isSilence = YES;
            return noErr;
        }

        double freqValues[frameCount];
        double detuneValues[frameCount];
        BOOL freqAutomated = [freqParam fillValuesFromFrame:blockFrame sampleRate:sampleRate frameCount:(NSInteger)frameCount into:freqValues];
        BOOL detuneAutomated = [detuneParam fillValuesFromFrame:blockFrame sampleRate:sampleRate frameCount:(NSInteger)frameCount into:detuneValues];
        BOOL hasAutomation = freqAutomated || detuneAutomated;
        if (hasAutomation) {
            for (AVAudioFrameCount f = 0; f < frameCount; ++f) {
                if (detuneValues[f] != 0.0) freqValues[f] *= exp2(detuneValues[f] / 1200.0);
            }
        } else if (detuneValues[0] != 0.0) {
            freqValues[0] *= exp2(detuneValues[0] / 1200.0);
        }

        const double TWO_PI = 2.0 * M_PI;
        const double invTwoPi = 1.0 / TWO_PI;
        double phase = localPhase;
        const double phaseIncConst = hasAutomation ? 0.0 : (TWO_PI * freqValues[0] / sampleRate);
        const double invSampleRate = 1.0 / sampleRate;

        switch (waveEnum) {
            case NSCOscWaveformSine: {
                for (AVAudioFrameCount f = firstRenderIndex; f < endRenderIndex; ++f) {
                    out0[f] = (float)sin(phase);
                    phase += hasAutomation ? (TWO_PI * freqValues[f] * invSampleRate) : phaseIncConst;
                    if (phase >= TWO_PI) phase = fmod(phase, TWO_PI);
                }
                break;
            }
            case NSCOscWaveformSquare: {
                for (AVAudioFrameCount f = firstRenderIndex; f < endRenderIndex; ++f) {
                    out0[f] = (phase < M_PI) ? 1.0f : -1.0f;
                    phase += hasAutomation ? (TWO_PI * freqValues[f] * invSampleRate) : phaseIncConst;
                    if (phase >= TWO_PI) phase = fmod(phase, TWO_PI);
                }
                break;
            }
            case NSCOscWaveformSawtooth: {
                for (AVAudioFrameCount f = firstRenderIndex; f < endRenderIndex; ++f) {
                    out0[f] = (float)((2.0 * phase * invTwoPi) - 1.0);
                    phase += hasAutomation ? (TWO_PI * freqValues[f] * invSampleRate) : phaseIncConst;
                    if (phase >= TWO_PI) phase = fmod(phase, TWO_PI);
                }
                break;
            }
            case NSCOscWaveformTriangle: {
                for (AVAudioFrameCount f = firstRenderIndex; f < endRenderIndex; ++f) {
                    out0[f] = (float)(1.0 - 4.0 * fabs((phase * invTwoPi) - 0.5));
                    phase += hasAutomation ? (TWO_PI * freqValues[f] * invSampleRate) : phaseIncConst;
                    if (phase >= TWO_PI) phase = fmod(phase, TWO_PI);
                }
                break;
            }
        }
        for (AVAudioFrameCount f = endRenderIndex; f < frameCount; ++f) {
            out0[f] = 0.0f;
        }
        localPhase = phase;

        for (UInt32 b = 1; b < bufCount; ++b) {
            float *dst = (float *)audioBufferList->mBuffers[b].mData;
            if (dst) memcpy(dst, out0, frameCount * sizeof(float));
        }

        if (hitStop) {
            atomic_store_explicit(playingPtr, 0, memory_order_release);
            atomic_store_explicit(stopPtr, (int_least64_t)-1, memory_order_release);
            dispatch_async(dispatch_get_main_queue(), ^{
                NSCAudioOscillatorNode *s = weakSelf;
                if (!s) return;
                if (s.context) [s.context decrementActiveCount];
                if (s.onended) s.onended();
            });
        }
        return noErr;
    }];

    if (self = [super initWithContext:context node:source]) {
        _type = [type ?: @"sine" copy];
        _frequencyParam = freqParam;
        _detuneParam = detuneParam;
        freqParam.owner = self;
        detuneParam.owner = self;
        _sourceNode = source;
        _phase = 0.0;
        _playingFlagData = playingData;
        _startSampleData = startSampleData;
        _stopSampleData = stopSampleData;
        _waveTypeData = waveTypeData;
        _playingFlag = playingCell;
        _startSamplePtr = startSampleCell;
        _stopSamplePtr = stopSampleCell;
        _waveTypePtr = waveTypeCell;
        [context.engine attachNode:source];
    }
    return self;
}

- (instancetype)initWithContext:(NSCAudioContext *)context node:(AVAudioNode *)node {
    return [self initWithContext:context];
}

- (void)setType:(NSString *)type {
    NSString *resolved = (type.length > 0) ? [type copy] : @"sine";
    _type = resolved;
    if (_waveTypePtr) {
        atomic_store_explicit(_waveTypePtr, (uint_least8_t)NSCOscWaveformFromString(resolved), memory_order_release);
    }
}

- (void)setPlaying:(BOOL)v {
    if (_playingFlag) atomic_store_explicit(_playingFlag, v ? 1 : 0, memory_order_release);
}

- (BOOL)isPlaying {
    return _playingFlag ? atomic_load_explicit(_playingFlag, memory_order_acquire) == 1 : NO;
}

- (void)start {
    [self beginAtFrame:-1];
}

- (void)beginAtFrame:(int64_t)frame {
    NSCAudioContext *ctx = self.context;
    if (!ctx || [self isPlaying]) return;
    if (_stopSamplePtr) atomic_store_explicit(_stopSamplePtr, (int64_t)-1, memory_order_release);
    if (_startSamplePtr) atomic_store_explicit(_startSamplePtr, frame, memory_order_release);
    [self setPlaying:YES];
    [ctx incrementActiveCount];
    if (!ctx.engine.isRunning && !ctx.engine.isInManualRenderingMode) {
        [NSCAudioContext startEngineWithRetry:ctx.engine attempts:3 label:@"oscillator" asyncCompletion:nil];
    }
}

- (void)startAt:(NSNumber *)when {
    NSCAudioContext *ctx = self.context;
    if (!ctx) return;
    double t = when ? when.doubleValue : 0.0;
    [self beginAtFrame:t > ctx.currentTime ? [ctx frameForTime:t] : -1];
}

- (void)stop {
    BOOL wasPlaying = [self isPlaying];
    if (!wasPlaying) return;
    [self setPlaying:NO];
    if (_startSamplePtr) atomic_store_explicit(_startSamplePtr, (int64_t)-1, memory_order_release);
    if (_stopSamplePtr) atomic_store_explicit(_stopSamplePtr, (int64_t)-1, memory_order_release);
    NSCAudioContext *ctx = self.context;
    if (ctx) [ctx decrementActiveCount];
}

- (void)stopAt:(NSNumber *)when {
    NSCAudioContext *ctx = self.context;
    if (!when || !ctx || when.doubleValue <= ctx.currentTime) {
        BOOL wasPlaying = [self isPlaying];
        [self stop];
        if (wasPlaying && self.onended) self.onended();
        return;
    }
    if ([self isPlaying] && _stopSamplePtr) atomic_store_explicit(_stopSamplePtr, [ctx frameForTime:when.doubleValue], memory_order_release);
}

- (double)frequency {
    return _frequencyParam.value;
}

- (void)setFrequency:(double)frequency {
    double now = self.context ? self.context.currentTime : 0.0;
    [_frequencyParam setValueAtTime:@(frequency) :@(now)];
}

- (BOOL)usesOutputStage { return YES; }

- (void)setPeriodicWave:(NSCPeriodicWave *)wave {
    if (wave) _type = @"custom";
    objc_setAssociatedObject(self, (__bridge const void *)@"periodicWave", wave, OBJC_ASSOCIATION_RETAIN_NONATOMIC);
}

@end
