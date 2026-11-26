#import <stdatomic.h>
#import <Accelerate/Accelerate.h>
#import "NSCAudioContext.h"

@implementation NSCConstantSourceNode {
    AVAudioSourceNode *_sourceNode;
    NSMutableData *_playingFlagData;
    NSMutableData *_offsetData;
    NSMutableData *_scheduleData;
    atomic_uint_least8_t *_playingFlag;
    double *_offsetCell;
    // [0] start sample, [1] stop sample; -1 when unscheduled.
    atomic_int_least64_t *_schedule;
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
    _schedule = NULL;
}

- (instancetype)initWithContext:(NSCAudioContext *)context offset:(double)offset {
    NSMutableData *playingData = [NSMutableData dataWithLength:sizeof(atomic_uint_least8_t)];
    NSMutableData *offsetData = [NSMutableData dataWithLength:sizeof(double)];
    atomic_uint_least8_t *playingCell = (atomic_uint_least8_t *)playingData.mutableBytes;
    double *offsetCell = (double *)offsetData.mutableBytes;
    atomic_init(playingCell, 0);
    *offsetCell = offset;
    NSMutableData *scheduleData = [NSMutableData dataWithLength:sizeof(atomic_int_least64_t) * 2];
    atomic_int_least64_t *scheduleCells = (atomic_int_least64_t *)scheduleData.mutableBytes;
    atomic_init(&scheduleCells[0], (int_least64_t)-1);
    atomic_init(&scheduleCells[1], (int_least64_t)-1);

    AVAudioFormat *format = context.format
    ?: [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32 sampleRate:44100.0 channels:1 interleaved:NO];

    NSMutableData *capturedPlayingData = playingData;
    NSMutableData *capturedOffsetData = offsetData;
    NSMutableData *capturedScheduleData = scheduleData;
    NSMutableData *timelineCell = context.timelineCell ?: [NSMutableData dataWithLength:NSCTimelineSlots * sizeof(double)];
    __weak NSCConstantSourceNode *weakSelf = self;
    NSCAudioParam *offsetParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:offset];
    AVAudioSourceNode *source = [[AVAudioSourceNode alloc] initWithFormat:format renderBlock:^OSStatus(BOOL *isSilence, const AudioTimeStamp *ts, AVAudioFrameCount frameCount, AudioBufferList *abl) {
        atomic_uint_least8_t *playingCell = (atomic_uint_least8_t *)capturedPlayingData.mutableBytes;
        double *offsetCell = (double *)capturedOffsetData.mutableBytes;
        atomic_int_least64_t *schedule = (atomic_int_least64_t *)capturedScheduleData.mutableBytes;
        BOOL playing = atomic_load_explicit(playingCell, memory_order_acquire) == 1;
        AVAudioFrameCount first = 0;
        AVAudioFrameCount end = playing ? frameCount : 0;
        BOOL hitStop = NO;
        BOOL hasSampleTime = ts && (ts->mFlags & kAudioTimeStampSampleTimeValid);
        double blockFrame = hasSampleTime ? NSCContextFrame(ts->mSampleTime, (const double *)timelineCell.bytes) : 0.0;
        if (playing && hasSampleTime) {
            int64_t blockStart = llround(blockFrame);
            int64_t blockEnd = blockStart + (int64_t)frameCount;
            int64_t startSample = atomic_load_explicit(&schedule[0], memory_order_acquire);
            int64_t stopSample = atomic_load_explicit(&schedule[1], memory_order_acquire);
            if (startSample > blockStart) first = (AVAudioFrameCount)MIN((int64_t)frameCount, startSample - blockStart);
            if (stopSample >= 0 && stopSample < blockEnd) {
                end = (AVAudioFrameCount)MAX((int64_t)0, stopSample - blockStart);
                hitStop = YES;
            }
        }
        (void)offsetCell;
        double values[frameCount];
        BOOL automated = end > first && [offsetParam fillValuesFromFrame:blockFrame sampleRate:format.sampleRate frameCount:(NSInteger)frameCount into:values];
        float v = end > first ? (float)values[0] : 0.0f;
        float zero = 0.0f;
        UInt32 bufCount = abl->mNumberBuffers;
        for (UInt32 b = 0; b < bufCount; ++b) {
            float *dst = (float *)abl->mBuffers[b].mData;
            if (!dst) continue;
            vDSP_vfill(&zero, dst, 1, (vDSP_Length)frameCount);
            if (end <= first) continue;
            if (automated) for (AVAudioFrameCount k = first; k < end; ++k) dst[k] = (float)values[k];
            else vDSP_vfill(&v, dst + first, 1, (vDSP_Length)(end - first));
        }
        if (end <= first && isSilence) *isSilence = YES;
        if (hitStop) {
            atomic_store_explicit(playingCell, 0, memory_order_release);
            atomic_store_explicit(&schedule[0], (int_least64_t)-1, memory_order_release);
            atomic_store_explicit(&schedule[1], (int_least64_t)-1, memory_order_release);
            dispatch_async(dispatch_get_main_queue(), ^{
                NSCConstantSourceNode *s = weakSelf;
                if (!s) return;
                if (s.context) [s.context decrementActiveCount];
                if (s.onended) s.onended();
            });
        }
        return noErr;
    }];

    if (self = [super initWithContext:context node:source]) {
        _sourceNode = source;
        _playingFlagData = playingData;
        _offsetData = offsetData;
        _scheduleData = scheduleData;
        _schedule = scheduleCells;
        _playingFlag = playingCell;
        _offsetCell = offsetCell;
        _offsetParam = offsetParam;
        offsetParam.owner = self;
        [self setLogicalChannels:1];
        [context.engine attachNode:source];
    }
    return self;
}

- (void)setOffsetScalar:(double)v {
    [_offsetParam setValue:@(v)];
}

- (BOOL)usesOutputStage { return YES; }

- (BOOL)isPlaying {
    return _playingFlag ? atomic_load_explicit(_playingFlag, memory_order_acquire) == 1 : NO;
}

- (void)start {
    [self startAt:nil];
}

- (void)startAt:(NSNumber *)when {
    NSCAudioContext *ctx = self.context;
    if (!ctx || !_playingFlag || [self isPlaying]) return;
    double t = when ? when.doubleValue : 0.0;
    int64_t startSample = -1;
    if (t > ctx.currentTime) startSample = [ctx frameForTime:t];
    atomic_store_explicit(&_schedule[1], (int_least64_t)-1, memory_order_release);
    atomic_store_explicit(&_schedule[0], startSample, memory_order_release);
    atomic_store_explicit(_playingFlag, 1, memory_order_release);
    [ctx incrementActiveCount];
    if (!ctx.engine.isRunning) {
        [NSCAudioContext startEngineWithRetry:ctx.engine attempts:3 label:@"constantSource" asyncCompletion:nil];
    }
}

- (void)stop {
    if (![self isPlaying]) return;
    atomic_store_explicit(_playingFlag, 0, memory_order_release);
    atomic_store_explicit(&_schedule[0], (int_least64_t)-1, memory_order_release);
    atomic_store_explicit(&_schedule[1], (int_least64_t)-1, memory_order_release);
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
    if (![self isPlaying]) return;
    atomic_store_explicit(&_schedule[1], [ctx frameForTime:when.doubleValue], memory_order_release);
}

@end
