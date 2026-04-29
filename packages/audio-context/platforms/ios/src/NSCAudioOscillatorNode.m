#import <stdatomic.h>
#import <math.h>
#import <objc/runtime.h>
#import "NSCAudioContext.h"

@implementation NSCAudioOscillatorNode {
    AVAudioSourceNode *_sourceNode;
    NSMutableData *_playingFlagData;
    NSMutableData *_startSampleData;
    atomic_uint_least8_t *_playingFlag;
    atomic_int_least64_t *_startSamplePtr;
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
}

- (instancetype)initWithContext:(NSCAudioContext *)context {
    return [self initWithContext:context type:@"sine" frequency:440.0];
}

- (instancetype)initWithContext:(NSCAudioContext *)context type:(NSString *)type frequency:(double)frequency {
    NSMutableData *playingData = [NSMutableData dataWithLength:sizeof(atomic_uint_least8_t)];
    NSMutableData *startSampleData = [NSMutableData dataWithLength:sizeof(atomic_int_least64_t)];
    atomic_uint_least8_t *playingCell = (atomic_uint_least8_t *)playingData.mutableBytes;
    atomic_int_least64_t *startSampleCell = (atomic_int_least64_t *)startSampleData.mutableBytes;
    atomic_init(playingCell, 0);
    atomic_init(startSampleCell, (int_least64_t)-1);

    typedef NS_ENUM(uint8_t, NSCOscWaveform) {
        NSCOscWaveformSine = 0,
        NSCOscWaveformSquare = 1,
        NSCOscWaveformSawtooth = 2,
    };
    NSCOscWaveform waveEnum = NSCOscWaveformSine;
    if ([type isEqualToString:@"square"]) waveEnum = NSCOscWaveformSquare;
    else if ([type isEqualToString:@"sawtooth"]) waveEnum = NSCOscWaveformSawtooth;
    
    NSCAudioParam *freqParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:frequency];
    AVAudioFormat *format = context.format
    ?: [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32 sampleRate:44100.0 channels:1 interleaved:NO];
    
    __block double localPhase = 0.0;
    
    NSMutableData *capturedPlayingData = playingData;
    NSMutableData *capturedStartSampleData = startSampleData;
    AVAudioSourceNode *source = [[AVAudioSourceNode alloc] initWithFormat:format renderBlock:^OSStatus(BOOL * _Nonnull isSilence, const AudioTimeStamp * _Nonnull timestamp, AVAudioFrameCount frameCount, AudioBufferList * _Nonnull audioBufferList) {
        atomic_uint_least8_t *playingCell = (atomic_uint_least8_t *)capturedPlayingData.mutableBytes;
        atomic_int_least64_t *startSampleCell = (atomic_int_least64_t *)capturedStartSampleData.mutableBytes;
        if (isSilence) *isSilence = NO;
        BOOL playing = atomic_load_explicit(playingCell, memory_order_acquire) == 1;
        int64_t startSample = atomic_load_explicit(startSampleCell, memory_order_acquire);
        if (!playing) {
            if (isSilence) *isSilence = YES;
            UInt32 bufCount = audioBufferList->mNumberBuffers;
            for (UInt32 b = 0; b < bufCount; ++b) {
                memset(audioBufferList->mBuffers[b].mData, 0, audioBufferList->mBuffers[b].mDataByteSize);
            }
            return noErr;
        }
        
        double sampleRate = format.sampleRate;
        double startTime = (timestamp && (timestamp->mFlags & kAudioTimeStampSampleTimeValid))
        ? timestamp->mSampleTime / sampleRate
        : 0.0;
        

        double freqValues[frameCount];
        BOOL hasAutomation = [freqParam fillValuesForRange:startTime sampleRate:sampleRate frameCount:(NSInteger)frameCount into:freqValues];
        
        const double TWO_PI = 2.0 * M_PI;
        double phase = localPhase;
        UInt32 bufCount = audioBufferList->mNumberBuffers;

        float *out0 = (bufCount > 0) ? (float *)audioBufferList->mBuffers[0].mData : NULL;
        if (!out0) return noErr;
        

        const double phaseIncConst = hasAutomation ? 0.0 : (TWO_PI * freqValues[0] / sampleRate);
        const double invSampleRate = 1.0 / sampleRate;
        
        AVAudioFrameCount firstRenderIndex = 0;
        if (startSample >= 0 && (timestamp && (timestamp->mFlags & kAudioTimeStampSampleTimeValid))) {
            int64_t blockStart = (int64_t)timestamp->mSampleTime;
            if (startSample >= blockStart + frameCount) {
                if (isSilence) *isSilence = YES;
                for (UInt32 b = 0; b < bufCount; ++b) memset(audioBufferList->mBuffers[b].mData, 0, audioBufferList->mBuffers[b].mDataByteSize);
                return noErr;
            }
            if (startSample > blockStart) {
                firstRenderIndex = (AVAudioFrameCount)(startSample - blockStart);
                for (AVAudioFrameCount i = 0; i < firstRenderIndex; ++i) out0[i] = 0.0f;
            }
        }
        
        switch (waveEnum) {
            case NSCOscWaveformSine: {
                if (hasAutomation) {
                    for (AVAudioFrameCount f = firstRenderIndex; f < frameCount; ++f) {
                        out0[f] = (float)sin(phase);
                        phase += TWO_PI * freqValues[f] * invSampleRate;
                        if (phase > TWO_PI) phase -= TWO_PI;
                    }
                } else {
                    for (AVAudioFrameCount f = firstRenderIndex; f < frameCount; ++f) {
                        out0[f] = (float)sin(phase);
                        phase += phaseIncConst;
                        if (phase > TWO_PI) phase -= TWO_PI;
                    }
                }
                break;
            }
            case NSCOscWaveformSquare: {
                if (hasAutomation) {
                    for (AVAudioFrameCount f = firstRenderIndex; f < frameCount; ++f) {
                        out0[f] = (fmod(phase, TWO_PI) < M_PI) ? 1.0f : -1.0f;
                        phase += TWO_PI * freqValues[f] * invSampleRate;
                        if (phase > TWO_PI) phase -= TWO_PI;
                    }
                } else {
                    for (AVAudioFrameCount f = firstRenderIndex; f < frameCount; ++f) {
                        out0[f] = (phase < M_PI) ? 1.0f : -1.0f;
                        phase += phaseIncConst;
                        if (phase > TWO_PI) phase -= TWO_PI;
                    }
                }
                break;
            }
            case NSCOscWaveformSawtooth: {
                const double invTwoPi = 1.0 / TWO_PI;
                if (hasAutomation) {
                    for (AVAudioFrameCount f = firstRenderIndex; f < frameCount; ++f) {
                        out0[f] = (float)((2.0 * phase * invTwoPi) - 1.0);
                        phase += TWO_PI * freqValues[f] * invSampleRate;
                        if (phase > TWO_PI) phase -= TWO_PI;
                    }
                } else {
                    for (AVAudioFrameCount f = firstRenderIndex; f < frameCount; ++f) {
                        out0[f] = (float)((2.0 * phase * invTwoPi) - 1.0);
                        phase += phaseIncConst;
                        if (phase > TWO_PI) phase -= TWO_PI;
                    }
                }
                break;
            }
        }
        localPhase = phase;
        
        for (UInt32 b = 1; b < bufCount; ++b) {
            float *dst = (float *)audioBufferList->mBuffers[b].mData;
            if (dst) memcpy(dst, out0, frameCount * sizeof(float));
        }
        return noErr;
    }];
    
    if (self = [super initWithContext:context node:source]) {
        _type = [type copy];
        _frequencyParam = freqParam;
        _sourceNode = source;
        _phase = 0.0;
        _playingFlagData = playingData;
        _startSampleData = startSampleData;
        _playingFlag = playingCell;
        _startSamplePtr = startSampleCell;
        [context.engine attachNode:source];
    }
    return self;
}

- (instancetype)initWithContext:(NSCAudioContext *)context node:(AVAudioNode *)node {
    return [self initWithContext:context];
}

- (void)setPlaying:(BOOL)v {
    if (_playingFlag) atomic_store_explicit(_playingFlag, v ? 1 : 0, memory_order_release);
}

- (BOOL)isPlaying {
    return _playingFlag ? atomic_load_explicit(_playingFlag, memory_order_acquire) == 1 : NO;
}

- (void)start {
    BOOL wasPlaying = [self isPlaying];
    if (wasPlaying) return;
    NSCAudioContext *ctx = self.context;
    if (!ctx) return;
    int64_t desiredStart = -1;
    double extra = [ctx extraLatencySeconds];
    if (extra > 0.0 && ctx.engine.isRunning) {
        AVAudioTime *now = ctx.engine.outputNode.lastRenderTime;
        if (now) {
            AVAudioFramePosition offset = (AVAudioFramePosition)llround(extra * now.sampleRate);
            desiredStart = (int64_t)(now.sampleTime + offset);
        }
    }
    [self setPlaying:YES];
    if (_startSamplePtr) atomic_store_explicit(_startSamplePtr, desiredStart, memory_order_release);
    [ctx incrementActiveCount];
    if (!ctx.engine.isRunning) {
        [NSCAudioContext startEngineWithRetry:ctx.engine attempts:3 label:@"oscillator" asyncCompletion:nil];
    }
}

- (void)stop {
    BOOL wasPlaying = [self isPlaying];
    if (!wasPlaying) return;
    [self setPlaying:NO];
    if (_startSamplePtr) atomic_store_explicit(_startSamplePtr, (int64_t)-1, memory_order_release);
    NSCAudioContext *ctx = self.context;
    if (ctx) [ctx decrementActiveCount];
}

- (double)frequency {
    return _frequencyParam.value;
}

- (void)setFrequency:(double)frequency {
    double now = self.context ? self.context.currentTime : 0.0;
    [_frequencyParam setValueAtTime:@(frequency) :@(now)];
}

- (void)setPeriodicWave:(NSCPeriodicWave *)wave {
    if (wave) self.type = @"custom";
    objc_setAssociatedObject(self, (__bridge const void *)@"periodicWave", wave, OBJC_ASSOCIATION_RETAIN_NONATOMIC);
}

@end
