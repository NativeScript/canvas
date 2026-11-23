#import "NSCAudioWorkletNode.h"
#import <stdatomic.h>
#import "NSCAudioContext.h"

#define QUANTUM 128
#define SLOTS 64
#define MAX_BACKLOG 48
#define SHRINK_WINDOW 128
#define MAX_CHANNELS 8

enum { REQUESTED = 0, DONE = 1, READ = 2, CLOSED = 3 };
enum { FRAME = 0, ALIVE = 2 };

typedef struct {
    float *words;
    _Atomic uint32_t *header;
    NSInteger channels;
    NSInteger slotWords;
    NSInteger outputCounts;
    NSInteger outputData;
    NSInteger paramLengths;
    NSInteger paramData;
    // Render thread only.
    float out[MAX_CHANNELS][QUANTUM];
    NSInteger position;
    uint64_t nextFrame;
    uint32_t lowWater;
    uint32_t window;
    BOOL finished;
    // Offline: requests run ahead of the render; the first is frame 0.
    BOOL offline;
    uint64_t requestFrame;
} WorkletRender;

static BOOL PushRequest(WorkletRender *r, NSArray<NSCAudioParam *> *params, uint64_t requestFrame, double time, double sampleRate) {
    uint32_t requested = atomic_load_explicit(&r->header[REQUESTED], memory_order_relaxed);
    uint32_t read = atomic_load_explicit(&r->header[READ], memory_order_acquire);
    if (requested - read >= SLOTS) return NO;
    float *slot = r->words + (requested % SLOTS) * r->slotWords;
    double frame = (double)requestFrame;
    memcpy(slot + FRAME, &frame, sizeof frame);
    ((uint32_t *)slot)[ALIVE] = 1;
    ((uint32_t *)slot)[r->outputCounts] = (uint32_t)r->channels;
    double values[QUANTUM];
    for (NSUInteger p = 0; p < params.count; p++) {
        float *dst = slot + r->paramData + p * QUANTUM;
        BOOL automated = [params[p] fillValuesForRange:time sampleRate:sampleRate frameCount:QUANTUM into:values];
        NSInteger length = automated ? QUANTUM : 1;
        for (NSInteger i = 0; i < length; i++) dst[i] = (float)values[i];
        ((uint32_t *)slot)[r->paramLengths + p] = (uint32_t)length;
    }
    atomic_store_explicit(&r->header[REQUESTED], requested + 1, memory_order_release);
    return YES;
}

static void ReadSlot(WorkletRender *r, uint32_t read) {
    float *slot = r->words + (read % SLOTS) * r->slotWords;
    uint32_t count = MIN(((uint32_t *)slot)[r->outputCounts], (uint32_t)r->channels);
    for (NSInteger c = 0; c < r->channels; c++) {
        if ((uint32_t)c < count) memcpy(r->out[c], slot + r->outputData + c * QUANTUM, sizeof r->out[c]);
        else memset(r->out[c], 0, sizeof r->out[c]);
    }
    if (((uint32_t *)slot)[ALIVE] == 0) r->finished = YES;
}

static void Silence(WorkletRender *r) {
    for (NSInteger c = 0; c < r->channels; c++) memset(r->out[c], 0, sizeof r->out[c]);
}

// Offline: the oldest response, never skipped. The context waits until it is in.
static void PullOffline(WorkletRender *r) {
    uint32_t read = atomic_load_explicit(&r->header[READ], memory_order_relaxed);
    uint32_t done = atomic_load_explicit(&r->header[DONE], memory_order_acquire);
    if (done == read) {
        Silence(r);
        return;
    }
    ReadSlot(r, read);
    atomic_store_explicit(&r->header[READ], read + 1, memory_order_release);
}

static void PullResponse(WorkletRender *r) {
    uint32_t read = atomic_load_explicit(&r->header[READ], memory_order_relaxed);
    uint32_t done = atomic_load_explicit(&r->header[DONE], memory_order_acquire);
    uint32_t backlog = done - read;
    if (backlog > MAX_BACKLOG) {
        read = done - MAX_BACKLOG;
    } else {
        uint32_t low = MIN(r->lowWater, backlog);
        if (++r->window >= SHRINK_WINDOW) {
            if (low >= 2) read += MAX((low - 1) / 2, 1u);
            r->window = 0;
            r->lowWater = UINT32_MAX;
        } else {
            r->lowWater = low;
        }
    }
    if (done != read) {
        ReadSlot(r, read);
        read++;
    } else {
        Silence(r);
    }
    atomic_store_explicit(&r->header[READ], read, memory_order_release);
}

@implementation NSCAudioWorkletNode {
    NSData *_ring;
    NSMutableData *_renderData;
    WorkletRender *_render;
    double _sampleRate;
}

- (instancetype)initWithContext:(NSCAudioContext *)context
                           ring:(NSData *)ring
                       channels:(NSInteger)channels
                      slotWords:(NSInteger)slotWords
                         header:(NSInteger)header
                   outputCounts:(NSInteger)outputCounts
                     outputData:(NSInteger)outputData
                   paramLengths:(NSInteger)paramLengths
                      paramData:(NSInteger)paramData
                  paramDefaults:(NSArray<NSNumber *> *)paramDefaults {
    channels = MAX(1, MIN(channels, MAX_CHANNELS));
    NSMutableData *renderData = [NSMutableData dataWithLength:sizeof(WorkletRender)];
    WorkletRender *render = (WorkletRender *)renderData.mutableBytes;
    render->words = (float *)ring.bytes;
    render->header = (_Atomic uint32_t *)((uint32_t *)ring.bytes + header);
    render->channels = channels;
    render->slotWords = slotWords;
    render->outputCounts = outputCounts;
    render->outputData = outputData;
    render->paramLengths = paramLengths;
    render->paramData = paramData;
    render->position = QUANTUM;
    render->lowWater = UINT32_MAX;
    render->offline = context.engine.isInManualRenderingMode;

    NSMutableArray<NSCAudioParam *> *params = [NSMutableArray array];
    for (NSNumber *value in paramDefaults) {
        [params addObject:[[NSCAudioParam alloc] initWithContext:context defaultValue:value.doubleValue]];
    }
    NSArray<NSCAudioParam *> *renderParams = [params copy];

    double sampleRate = context.format.sampleRate > 0 ? context.format.sampleRate : 48000.0;
    AVAudioFormat *format = [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32 sampleRate:sampleRate channels:(AVAudioChannelCount)channels interleaved:NO];
    NSData *keptRing = ring;
    NSMutableData *keptRender = renderData;
    AVAudioSourceNode *source = [[AVAudioSourceNode alloc] initWithFormat:format renderBlock:^OSStatus(BOOL *isSilence, const AudioTimeStamp *ts, AVAudioFrameCount frameCount, AudioBufferList *abl) {
        (void)keptRing;
        WorkletRender *r = (WorkletRender *)keptRender.mutableBytes;
        double blockTime = ts && (ts->mFlags & kAudioTimeStampSampleTimeValid) ? ts->mSampleTime / sampleRate : 0.0;
        AVAudioFrameCount written = 0;
        while (written < frameCount) {
            if (r->position == QUANTUM) {
                if (r->offline) {
                    PullOffline(r);
                } else {
                    if (!r->finished) PushRequest(r, renderParams, r->nextFrame, blockTime + written / sampleRate, sampleRate);
                    PullResponse(r);
                }
                r->nextFrame += QUANTUM;
                r->position = 0;
            }
            AVAudioFrameCount n = (AVAudioFrameCount)MIN((NSInteger)(QUANTUM - r->position), (NSInteger)(frameCount - written));
            for (UInt32 b = 0; b < abl->mNumberBuffers; b++) {
                float *dst = (float *)abl->mBuffers[b].mData;
                if (!dst) continue;
                NSInteger c = MIN((NSInteger)b, r->channels - 1);
                memcpy(dst + written, r->out[c] + r->position, n * sizeof(float));
            }
            r->position += n;
            written += n;
        }
        if (isSilence) *isSilence = NO;
        return noErr;
    }];

    if (self = [super initWithContext:context node:source]) {
        _ring = ring;
        _renderData = renderData;
        _render = render;
        _params = renderParams;
        _sampleRate = sampleRate;
        [context.engine attachNode:source];
    }
    return self;
}

- (void)close {
    if (_render) atomic_store_explicit(&_render->header[CLOSED], 1, memory_order_release);
}

// Called on the thread that renders, between quanta.
- (BOOL)prepareOfflineQuantumBefore:(uint64_t)limit {
    WorkletRender *r = _render;
    if (!r || !r->offline || r->finished) return YES;
    if (atomic_load_explicit(&r->header[CLOSED], memory_order_acquire)) return YES;
    while (r->requestFrame < limit && PushRequest(r, _params, r->requestFrame, r->requestFrame / _sampleRate, _sampleRate)) r->requestFrame += QUANTUM;
    return atomic_load_explicit(&r->header[DONE], memory_order_acquire) != atomic_load_explicit(&r->header[READ], memory_order_relaxed);
}

- (void)abandonOffline {
    if (!_render) return;
    _render->finished = YES;
    [self close];
}

- (void)dealloc {
    [self close];
    NSCAudioContext *ctx = self.context;
    AVAudioNode *node = self.avNode;
    if (ctx && node) {
        AVAudioEngine *eng = ctx.engine;
        if (eng && [ctx isNode:node attachedToEngine:eng]) {
            @try { [eng disconnectNodeOutput:node]; } @catch (NSException *e) {}
            @try { [ctx detachNode:node fromEngine:eng]; } @catch (NSException *e) {}
        }
    }
}

@end
