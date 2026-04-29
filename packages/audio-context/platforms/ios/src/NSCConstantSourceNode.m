#import <stdatomic.h>
#import <Accelerate/Accelerate.h>
#import "NSCAudioContext.h"

@implementation NSCConstantSourceNode {
    AVAudioSourceNode *_sourceNode;
    NSMutableData *_playingFlagData;
    NSMutableData *_offsetData;
    atomic_uint_least8_t *_playingFlag;
    double *_offsetCell;
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
    _offsetCell = NULL;
}

- (instancetype)initWithContext:(NSCAudioContext *)context offset:(double)offset {
    NSMutableData *playingData = [NSMutableData dataWithLength:sizeof(atomic_uint_least8_t)];
    NSMutableData *offsetData = [NSMutableData dataWithLength:sizeof(double)];
    atomic_uint_least8_t *playingCell = (atomic_uint_least8_t *)playingData.mutableBytes;
    double *offsetCell = (double *)offsetData.mutableBytes;
    atomic_init(playingCell, 0);
    *offsetCell = offset;

    AVAudioFormat *format = context.format
    ?: [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32 sampleRate:44100.0 channels:1 interleaved:NO];

    NSMutableData *capturedPlayingData = playingData;
    NSMutableData *capturedOffsetData = offsetData;
    AVAudioSourceNode *source = [[AVAudioSourceNode alloc] initWithFormat:format renderBlock:^OSStatus(BOOL *isSilence, const AudioTimeStamp *ts, AVAudioFrameCount frameCount, AudioBufferList *abl) {
        atomic_uint_least8_t *playingCell = (atomic_uint_least8_t *)capturedPlayingData.mutableBytes;
        double *offsetCell = (double *)capturedOffsetData.mutableBytes;
        BOOL playing = atomic_load_explicit(playingCell, memory_order_acquire) == 1;
        float v = playing ? (float)(*offsetCell) : 0.0f;
        if (!playing && isSilence) *isSilence = YES;
        UInt32 bufCount = abl->mNumberBuffers;
        for (UInt32 b = 0; b < bufCount; ++b) {
            float *dst = (float *)abl->mBuffers[b].mData;
            if (!dst) continue;
            vDSP_vfill(&v, dst, 1, (vDSP_Length)frameCount);
        }
        return noErr;
    }];

    if (self = [super initWithContext:context node:source]) {
        _sourceNode = source;
        _playingFlagData = playingData;
        _offsetData = offsetData;
        _playingFlag = playingCell;
        _offsetCell = offsetCell;
        _offsetParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:offset];
        [context.engine attachNode:source];
    }
    return self;
}

- (void)setOffsetScalar:(double)v {
    if (_offsetCell) *_offsetCell = v;
}

- (void)start {
    if (_playingFlag) atomic_store_explicit(_playingFlag, 1, memory_order_release);
    NSCAudioContext *ctx = self.context;
    if (ctx) {
        [ctx incrementActiveCount];
        if (!ctx.engine.isRunning) {
            [NSCAudioContext startEngineWithRetry:ctx.engine attempts:3 label:@"constantSource" asyncCompletion:nil];
        }
    }
}

- (void)stop {
    if (_playingFlag) atomic_store_explicit(_playingFlag, 0, memory_order_release);
    NSCAudioContext *ctx = self.context;
    if (ctx) [ctx decrementActiveCount];
    if (self.onended) self.onended();
}

@end
