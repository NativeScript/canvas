#import "NSCAudioWorkletNode.h"
#import <stdatomic.h>
#import <AudioToolbox/AudioToolbox.h>
#import "NSCAudioContext.h"
#import "NSCAudioLog.h"

#define QUANTUM 128
#define SLOTS 64
#define MAX_BACKLOG 48
#define SHRINK_WINDOW 128
// Channels per input or output; also the ring's per-input stride (WORKLET_MAX_INPUT_CHANNELS).
#define MAX_CHANNELS 8
// Offline: a Worker answering nothing for this long renders silence.
#define OFFLINE_STALL_SECONDS 5.0

enum { REQUESTED = 0, DONE = 1, READ = 2, CLOSED = 3 };
enum { FRAME = 0, ALIVE = 2 };

typedef struct {
    float *words;
    _Atomic uint32_t *header;
    NSInteger slotWords;
    NSInteger inputCounts;
    NSInteger inputData;
    NSInteger outputCounts;
    NSInteger paramLengths;
    NSInteger paramData;
    NSInteger inputs;
    NSInteger outputs;
    NSInteger inChannels[16];
    NSInteger outChannels[16];
    // Channels on the AVAudio side of each port; the context's count when the port's is 1 or 2.
    NSInteger inPort[16];
    NSInteger outPort[16];
    NSInteger outputData[16];
    // Render thread only. The quantum being filled from the inputs and played to the outputs.
    float *inQ;
    float *outQ;
    BOOL inSilent[16];
    // One render call's pulled inputs and rendered outputs, `maxFrames` per channel.
    float *pulled;
    float *rendered;
    AudioBufferList *pullLists[16];
    UInt32 maxFrames;
    double lastSampleTime;
    NSInteger position;
    uint64_t nextFrame;
    uint32_t lowWater;
    uint32_t window;
    BOOL finished;
    // Offline: requests run ahead of the render; the first is frame 0.
    BOOL offline;
    uint64_t requestFrame;
    CFTypeRef params;
    double sampleRate;
} WorkletRender;

#define MAX_PORTS 16

static float *InQ(WorkletRender *r, NSInteger i, NSInteger c) { return r->inQ + (i * MAX_CHANNELS + c) * QUANTUM; }
static float *OutQ(WorkletRender *r, NSInteger o, NSInteger c) { return r->outQ + (o * MAX_CHANNELS + c) * QUANTUM; }
static float *Pulled(WorkletRender *r, NSInteger i, NSInteger c) { return r->pulled + (i * MAX_CHANNELS + c) * r->maxFrames; }
static float *Rendered(WorkletRender *r, NSInteger o, NSInteger c) { return r->rendered + (o * MAX_CHANNELS + c) * r->maxFrames; }

static BOOL PushRequest(WorkletRender *r, uint64_t requestFrame, double time) {
    uint32_t requested = atomic_load_explicit(&r->header[REQUESTED], memory_order_relaxed);
    uint32_t read = atomic_load_explicit(&r->header[READ], memory_order_acquire);
    if (requested - read >= SLOTS) return NO;
    float *slot = r->words + (requested % SLOTS) * r->slotWords;
    double frame = (double)requestFrame;
    memcpy(slot + FRAME, &frame, sizeof frame);
    ((uint32_t *)slot)[ALIVE] = 1;
    for (NSInteger i = 0; i < r->inputs; i++) {
        uint32_t count = r->inSilent[i] ? 0 : (uint32_t)r->inChannels[i];
        ((uint32_t *)slot)[r->inputCounts + i] = count;
        for (uint32_t c = 0; c < count; c++) {
            memcpy(slot + r->inputData + (i * MAX_CHANNELS + c) * QUANTUM, InQ(r, i, c), QUANTUM * sizeof(float));
        }
    }
    for (NSInteger o = 0; o < r->outputs; o++) ((uint32_t *)slot)[r->outputCounts + o] = (uint32_t)r->outChannels[o];
    NSArray<NSCAudioParam *> *params = (__bridge NSArray<NSCAudioParam *> *)r->params;
    double values[QUANTUM];
    for (NSUInteger p = 0; p < params.count; p++) {
        float *dst = slot + r->paramData + p * QUANTUM;
        BOOL automated = [params[p] fillValuesForRange:time sampleRate:r->sampleRate frameCount:QUANTUM into:values];
        NSInteger length = automated ? QUANTUM : 1;
        for (NSInteger k = 0; k < length; k++) dst[k] = (float)values[k];
        ((uint32_t *)slot)[r->paramLengths + p] = (uint32_t)length;
    }
    atomic_store_explicit(&r->header[REQUESTED], requested + 1, memory_order_release);
    return YES;
}

static void ReadSlot(WorkletRender *r, uint32_t read) {
    float *slot = r->words + (read % SLOTS) * r->slotWords;
    for (NSInteger o = 0; o < r->outputs; o++) {
        uint32_t count = MIN(((uint32_t *)slot)[r->outputCounts + o], (uint32_t)r->outChannels[o]);
        for (NSInteger c = 0; c < r->outChannels[o]; c++) {
            if ((uint32_t)c < count) memcpy(OutQ(r, o, c), slot + r->outputData[o] + c * QUANTUM, QUANTUM * sizeof(float));
            else memset(OutQ(r, o, c), 0, QUANTUM * sizeof(float));
        }
    }
    if (((uint32_t *)slot)[ALIVE] == 0) r->finished = YES;
}

static void Silence(WorkletRender *r) {
    memset(r->outQ, 0, (size_t)MAX(r->outputs, 1) * MAX_CHANNELS * QUANTUM * sizeof(float));
}

static BOOL Closed(WorkletRender *r) {
    return atomic_load_explicit(&r->header[CLOSED], memory_order_acquire) != 0;
}

// Offline: the oldest response, never skipped.
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

// Realtime: the newest responses, shrinking a backlog a stall left behind.
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

// Offline with inputs: this quantum's inputs exist only now, so wait for the Worker's answer.
static void ProcessOfflineInputs(WorkletRender *r) {
    if (r->finished || Closed(r)) {
        PullOffline(r);
        return;
    }
    if (!PushRequest(r, r->nextFrame, r->nextFrame / r->sampleRate)) {
        Silence(r);
        return;
    }
    uint32_t target = atomic_load_explicit(&r->header[REQUESTED], memory_order_relaxed);
    CFAbsoluteTime deadline = CFAbsoluteTimeGetCurrent() + OFFLINE_STALL_SECONDS;
    while (atomic_load_explicit(&r->header[DONE], memory_order_acquire) != target && !Closed(r)) {
        if (CFAbsoluteTimeGetCurrent() > deadline) {
            NSCLogError(@"NSCAudioWorkletNode: the Worker stopped answering; rendering silence");
            r->finished = YES;
            atomic_store_explicit(&r->header[CLOSED], 1, memory_order_release);
            break;
        }
        usleep(100);
    }
    PullOffline(r);
}

static void EndQuantum(WorkletRender *r) {
    if (r->offline) {
        if (r->inputs > 0) ProcessOfflineInputs(r);
        else PullOffline(r);
    } else {
        if (!r->finished) PushRequest(r, r->nextFrame, r->nextFrame / r->sampleRate);
        PullResponse(r);
    }
    r->nextFrame += QUANTUM;
    r->position = 0;
    for (NSInteger i = 0; i < r->inputs; i++) r->inSilent[i] = YES;
}

// Web Audio "speakers" mixing from the port's channels; AVAudioMixerNode pans equal-power instead.
static void CopyInput(WorkletRender *r, NSInteger i, NSInteger at, NSInteger from, NSInteger n) {
    NSInteger want = r->inChannels[i], have = r->inPort[i];
    if (have == 2 && want == 1) {
        float *dst = InQ(r, i, 0) + at;
        const float *left = Pulled(r, i, 0) + from, *right = Pulled(r, i, 1) + from;
        for (NSInteger k = 0; k < n; k++) dst[k] = 0.5f * (left[k] + right[k]);
        return;
    }
    for (NSInteger c = 0; c < want; c++) {
        memcpy(InQ(r, i, c) + at, Pulled(r, i, MIN(c, have - 1)) + from, n * sizeof(float));
    }
}

static void DownmixOutputs(WorkletRender *r, AUAudioFrameCount frames) {
    for (NSInteger o = 0; o < r->outputs; o++) {
        if (r->outPort[o] != 1 || r->outChannels[o] != 2) continue;
        float *left = Rendered(r, o, 0);
        const float *right = Rendered(r, o, 1);
        for (AUAudioFrameCount k = 0; k < frames; k++) left[k] = 0.5f * (left[k] + right[k]);
    }
}

// Pulls `frames` of every input and renders `frames` of every output into `rendered`. Offline,
// a call is one quantum, so it is processed before it plays; realtime output trails by a quantum.
static void Process(WorkletRender *r, const AudioTimeStamp *ts, AUAudioFrameCount frames, AURenderPullInputBlock pull) {
    BOOL silent[MAX_PORTS];
    for (NSInteger i = 0; i < r->inputs; i++) {
        AudioBufferList *abl = r->pullLists[i];
        for (UInt32 c = 0; c < abl->mNumberBuffers; c++) {
            abl->mBuffers[c].mData = Pulled(r, i, c);
            abl->mBuffers[c].mDataByteSize = frames * sizeof(float);
        }
        AudioUnitRenderActionFlags flags = 0;
        OSStatus status = pull ? pull(&flags, ts, frames, (NSInteger)i, abl) : kAudioUnitErr_NoConnection;
        silent[i] = status != noErr || (flags & kAudioUnitRenderAction_OutputIsSilence) != 0;
        for (UInt32 c = 0; c < abl->mNumberBuffers; c++) {
            float *mine = Pulled(r, i, c);
            if (status != noErr || !abl->mBuffers[c].mData) memset(mine, 0, frames * sizeof(float));
            else if (abl->mBuffers[c].mData != mine) memcpy(mine, abl->mBuffers[c].mData, frames * sizeof(float));
        }
    }
    AUAudioFrameCount done = 0;
    if (r->offline) {
        // The context renders one quantum per call; a short last one is padded.
        for (NSInteger i = 0; i < r->inputs; i++) {
            r->inSilent[i] = silent[i];
            CopyInput(r, i, 0, 0, MIN(frames, QUANTUM));
            for (NSInteger c = 0; c < r->inChannels[i]; c++) {
                memset(InQ(r, i, c) + frames, 0, (QUANTUM - MIN(frames, QUANTUM)) * sizeof(float));
            }
        }
        EndQuantum(r);
        for (NSInteger o = 0; o < r->outputs; o++) {
            for (NSInteger c = 0; c < r->outChannels[o]; c++) memcpy(Rendered(r, o, c), OutQ(r, o, c), MIN(frames, QUANTUM) * sizeof(float));
        }
        DownmixOutputs(r, MIN(frames, QUANTUM));
        return;
    }
    while (done < frames) {
        AUAudioFrameCount n = (AUAudioFrameCount)MIN((NSInteger)(QUANTUM - r->position), (NSInteger)(frames - done));
        for (NSInteger i = 0; i < r->inputs; i++) {
            if (!silent[i]) r->inSilent[i] = NO;
            CopyInput(r, i, r->position, done, n);
        }
        for (NSInteger o = 0; o < r->outputs; o++) {
            for (NSInteger c = 0; c < r->outChannels[o]; c++) memcpy(Rendered(r, o, c) + done, OutQ(r, o, c) + r->position, n * sizeof(float));
        }
        r->position += n;
        done += n;
        if (r->position == QUANTUM) EndQuantum(r);
    }
    DownmixOutputs(r, frames);
}

static AVAudioFormat *ChannelFormat(double sampleRate, NSInteger channels) {
    if (channels <= 2) return [[AVAudioFormat alloc] initStandardFormatWithSampleRate:sampleRate channels:(AVAudioChannelCount)channels];
    AVAudioChannelLayout *layout = [AVAudioChannelLayout layoutWithLayoutTag:kAudioChannelLayoutTag_DiscreteInOrder | (AudioChannelLayoutTag)channels];
    return [[AVAudioFormat alloc] initStandardFormatWithSampleRate:sampleRate channelLayout:layout];
}

static NSInteger PortChannels(NSInteger channels, NSInteger contextChannels) {
    return channels <= 2 && contextChannels <= 2 ? contextChannels : channels;
}

#pragma mark - The audio unit

// Bus formats for the unit being instantiated; set around the synchronous instantiation.
static NSArray<AVAudioFormat *> *pendingInputs;
static NSArray<AVAudioFormat *> *pendingOutputs;

@interface NSCWorkletUnit : AUAudioUnit
@property (nonatomic, readonly) WorkletRender *render;
@end

@implementation NSCWorkletUnit {
    AUAudioUnitBusArray *_inputBusses;
    AUAudioUnitBusArray *_outputBusses;
    WorkletRender *_render;
}

- (instancetype)initWithComponentDescription:(AudioComponentDescription)description options:(AudioComponentInstantiationOptions)options error:(NSError **)outError {
    if (!(self = [super initWithComponentDescription:description options:options error:outError])) return nil;
    NSMutableArray<AUAudioUnitBus *> *inputs = [NSMutableArray array];
    for (AVAudioFormat *format in pendingInputs) {
        AUAudioUnitBus *bus = [[AUAudioUnitBus alloc] initWithFormat:format error:outError];
        if (!bus) return nil;
        [inputs addObject:bus];
    }
    NSMutableArray<AUAudioUnitBus *> *outputs = [NSMutableArray array];
    for (AVAudioFormat *format in pendingOutputs) {
        AUAudioUnitBus *bus = [[AUAudioUnitBus alloc] initWithFormat:format error:outError];
        if (!bus) return nil;
        [outputs addObject:bus];
    }
    _inputBusses = [[AUAudioUnitBusArray alloc] initWithAudioUnit:self busType:AUAudioUnitBusTypeInput busses:inputs];
    _outputBusses = [[AUAudioUnitBusArray alloc] initWithAudioUnit:self busType:AUAudioUnitBusTypeOutput busses:outputs];
    _render = calloc(1, sizeof(WorkletRender));
    _render->lastSampleTime = -1.0;
    self.maximumFramesToRender = 4096;
    return self;
}

- (AUAudioUnitBusArray *)inputBusses { return _inputBusses; }
- (AUAudioUnitBusArray *)outputBusses { return _outputBusses; }
- (BOOL)canProcessInPlace { return NO; }

- (BOOL)allocateRenderResourcesAndReturnError:(NSError **)outError {
    if (![super allocateRenderResourcesAndReturnError:outError]) return NO;
    WorkletRender *r = _render;
    UInt32 frames = MAX((UInt32)self.maximumFramesToRender, (UInt32)QUANTUM);
    if (frames > r->maxFrames) {
        free(r->pulled);
        free(r->rendered);
        r->pulled = calloc((size_t)MAX(r->inputs, 1) * MAX_CHANNELS * frames, sizeof(float));
        r->rendered = calloc((size_t)MAX(r->outputs, 1) * MAX_CHANNELS * frames, sizeof(float));
        r->maxFrames = frames;
    }
    return YES;
}

- (AUInternalRenderBlock)internalRenderBlock {
    WorkletRender *r = _render;
    return ^AUAudioUnitStatus(AudioUnitRenderActionFlags *flags, const AudioTimeStamp *ts, AUAudioFrameCount frames, NSInteger bus, AudioBufferList *out, const AURenderEvent *events, AURenderPullInputBlock pull) {
        if (!r->words || frames > r->maxFrames || bus >= r->outputs) return kAudioUnitErr_TooManyFramesToProcess;
        double sampleTime = ts && (ts->mFlags & kAudioTimeStampSampleTimeValid) ? ts->mSampleTime : -2.0;
        // Every output bus asks for the same timestamp; the first one processes them all.
        if (sampleTime != r->lastSampleTime || sampleTime < 0.0) {
            Process(r, ts, frames, pull);
            r->lastSampleTime = sampleTime;
        }
        for (UInt32 c = 0; c < out->mNumberBuffers; c++) {
            float *from = Rendered(r, bus, MIN((NSInteger)c, r->outChannels[bus] - 1));
            if (out->mBuffers[c].mData) memcpy(out->mBuffers[c].mData, from, frames * sizeof(float));
            else out->mBuffers[c].mData = from;
            out->mBuffers[c].mDataByteSize = frames * sizeof(float);
        }
        return noErr;
    };
}

- (void)dealloc {
    WorkletRender *r = _render;
    if (!r) return;
    for (NSInteger i = 0; i < MAX_PORTS; i++) free(r->pullLists[i]);
    free(r->pulled);
    free(r->rendered);
    free(r->inQ);
    free(r->outQ);
    if (r->params) CFRelease(r->params);
    free(r);
}

@end

static AudioComponentDescription WorkletDescription(BOOL generator) {
    AudioComponentDescription description = {0};
    description.componentType = generator ? kAudioUnitType_Generator : kAudioUnitType_Effect;
    description.componentSubType = 'nscw';
    description.componentManufacturer = 'NSCa';
    return description;
}

#pragma mark - The node

@implementation NSCAudioWorkletNode {
    NSData *_ring;
    AVAudioUnit *_unit;
    NSArray<AVAudioMixerNode *> *_inputMixers;
    // Connections into input i go to this wrapper of its mixer.
    NSArray<NSCAudioNode *> *_inputPorts;
    BOOL _active;
}

- (nullable instancetype)initWithContext:(NSCAudioContext *)context
                                    ring:(NSData *)ring
                           inputChannels:(NSArray<NSNumber *> *)inputChannels
                          outputChannels:(NSArray<NSNumber *> *)outputChannels
                               slotWords:(NSInteger)slotWords
                                  header:(NSInteger)header
                             inputCounts:(NSInteger)inputCounts
                               inputData:(NSInteger)inputData
                            outputCounts:(NSInteger)outputCounts
                              outputData:(NSArray<NSNumber *> *)outputData
                            paramLengths:(NSInteger)paramLengths
                               paramData:(NSInteger)paramData
                           paramDefaults:(NSArray<NSNumber *> *)paramDefaults {
    NSInteger inputs = MIN((NSInteger)inputChannels.count, MAX_PORTS);
    NSInteger outputs = MIN((NSInteger)outputChannels.count, MAX_PORTS);
    if (outputs != (NSInteger)outputData.count || inputs + outputs == 0) return nil;
    double sampleRate = context.format.sampleRate > 0 ? context.format.sampleRate : 48000.0;
    NSInteger contextChannels = context.format.channelCount > 0 ? (NSInteger)context.format.channelCount : 2;

    static dispatch_once_t once;
    dispatch_once(&once, ^{
        [AUAudioUnit registerSubclass:[NSCWorkletUnit class] asComponentDescription:WorkletDescription(YES) name:@"NativeScript: AudioWorklet" version:1];
        [AUAudioUnit registerSubclass:[NSCWorkletUnit class] asComponentDescription:WorkletDescription(NO) name:@"NativeScript: AudioWorklet" version:1];
    });

    NSInteger inCounts[MAX_PORTS], outCounts[MAX_PORTS];
    NSMutableArray<AVAudioFormat *> *inFormats = [NSMutableArray array];
    NSMutableArray<AVAudioFormat *> *outFormats = [NSMutableArray array];
    for (NSInteger i = 0; i < inputs; i++) {
        inCounts[i] = MAX(1, MIN(inputChannels[i].integerValue, MAX_CHANNELS));
        [inFormats addObject:ChannelFormat(sampleRate, PortChannels(inCounts[i], contextChannels))];
    }
    for (NSInteger o = 0; o < outputs; o++) {
        outCounts[o] = MAX(1, MIN(outputChannels[o].integerValue, MAX_CHANNELS));
        [outFormats addObject:ChannelFormat(sampleRate, PortChannels(outCounts[o], contextChannels))];
    }

    pendingInputs = inFormats;
    pendingOutputs = outFormats;
    // A registered in-process unit instantiates synchronously.
    AVAudioUnit *unit = inputs > 0
        ? [[AVAudioUnitEffect alloc] initWithAudioComponentDescription:WorkletDescription(NO)]
        : [[AVAudioUnitGenerator alloc] initWithAudioComponentDescription:WorkletDescription(YES)];
    pendingInputs = nil;
    pendingOutputs = nil;
    NSCWorkletUnit *au = (NSCWorkletUnit *)unit.AUAudioUnit;
    if (![au isKindOfClass:[NSCWorkletUnit class]]) {
        NSCLogError(@"NSCAudioWorkletNode: could not instantiate the worklet audio unit");
        return nil;
    }

    WorkletRender *r = au.render;
    r->words = (float *)ring.bytes;
    r->header = (_Atomic uint32_t *)((uint32_t *)ring.bytes + header);
    r->slotWords = slotWords;
    r->inputCounts = inputCounts;
    r->inputData = inputData;
    r->outputCounts = outputCounts;
    r->paramLengths = paramLengths;
    r->paramData = paramData;
    r->inputs = inputs;
    r->outputs = outputs;
    for (NSInteger i = 0; i < inputs; i++) {
        r->inChannels[i] = inCounts[i];
        r->inPort[i] = (NSInteger)inFormats[i].channelCount;
        r->inSilent[i] = YES;
        AudioBufferList *abl = calloc(1, sizeof(AudioBufferList) + (MAX_CHANNELS - 1) * sizeof(AudioBuffer));
        abl->mNumberBuffers = (UInt32)r->inPort[i];
        for (UInt32 c = 0; c < abl->mNumberBuffers; c++) abl->mBuffers[c].mNumberChannels = 1;
        r->pullLists[i] = abl;
    }
    for (NSInteger o = 0; o < outputs; o++) {
        r->outChannels[o] = outCounts[o];
        r->outPort[o] = (NSInteger)outFormats[o].channelCount;
        r->outputData[o] = outputData[o].integerValue;
    }
    r->inQ = calloc((size_t)MAX(inputs, 1) * MAX_CHANNELS * QUANTUM, sizeof(float));
    r->outQ = calloc((size_t)MAX(outputs, 1) * MAX_CHANNELS * QUANTUM, sizeof(float));
    r->position = 0;
    r->lowWater = UINT32_MAX;
    r->offline = context.engine.isInManualRenderingMode;
    r->sampleRate = sampleRate;

    NSMutableArray<NSCAudioParam *> *params = [NSMutableArray array];
    for (NSNumber *value in paramDefaults) {
        [params addObject:[[NSCAudioParam alloc] initWithContext:context defaultValue:value.doubleValue]];
    }
    r->params = CFBridgingRetain([params copy]);

    if (!(self = [super initWithContext:context node:unit])) return nil;
    _ring = ring;
    _unit = unit;
    _params = [params copy];
    _numberOfInputs = inputs;
    _numberOfOutputs = outputs;

    AVAudioEngine *engine = context.engine;
    if ([context isKindOfClass:[NSCOfflineAudioContext class]]) [(NSCOfflineAudioContext *)context willConnect];
    [engine attachNode:unit];
    NSMutableArray<AVAudioMixerNode *> *mixers = [NSMutableArray array];
    NSMutableArray<NSCAudioNode *> *ports = [NSMutableArray array];
    for (NSInteger i = 0; i < inputs; i++) {
        AVAudioMixerNode *mixer = [[AVAudioMixerNode alloc] init];
        [engine attachNode:mixer];
        @try {
            [engine connect:mixer to:unit fromBus:0 toBus:(AVAudioNodeBus)i format:inFormats[i]];
        } @catch (NSException *e) {
            NSCLogError(@"NSCAudioWorkletNode: input %ld did not connect: %@", (long)i, e);
        }
        [mixers addObject:mixer];
        [ports addObject:[[NSCAudioNode alloc] initWithContext:context node:mixer]];
    }
    _inputMixers = [mixers copy];
    _inputPorts = [ports copy];
    return self;
}

- (void)handleConnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    NSInteger index = input ? input.integerValue : 0;
    if (index < 0 || index >= (NSInteger)_inputPorts.count) return;
    [_inputPorts[index] handleConnectFrom:source output:output input:nil];
}

- (void)handleDisconnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    if (input) {
        NSInteger index = input.integerValue;
        if (index >= 0 && index < (NSInteger)_inputPorts.count) [_inputPorts[index] handleDisconnectFrom:source output:output input:nil];
        return;
    }
    for (NSCAudioNode *port in _inputPorts) [port handleDisconnectFrom:source output:output input:nil];
}

// A worklet has no start(): it plays once connected, as sources do after start().
- (void)handleConnectedTo:(NSCAudioNode *)destination output:(NSNumber *)output input:(NSNumber *)input {
    [super handleConnectedTo:destination output:output input:input];
    NSCAudioContext *ctx = self.context;
    if (!ctx || ctx.engine.isInManualRenderingMode) return;
    if (!_active) {
        _active = YES;
        [ctx incrementActiveCount];
    }
    if (!ctx.engine.isRunning) [NSCAudioContext startEngineWithRetry:ctx.engine attempts:3 label:@"worklet" asyncCompletion:nil];
}

- (WorkletRender *)render {
    return ((NSCWorkletUnit *)_unit.AUAudioUnit).render;
}

- (void)close {
    WorkletRender *r = [self render];
    if (r && r->header) atomic_store_explicit(&r->header[CLOSED], 1, memory_order_release);
}

// Called on the thread that renders, between quanta.
- (BOOL)prepareOfflineQuantumBefore:(uint64_t)limit {
    WorkletRender *r = [self render];
    if (!r || !r->offline || r->finished || r->inputs > 0) return YES;
    if (Closed(r)) return YES;
    while (r->requestFrame < limit && PushRequest(r, r->requestFrame, r->requestFrame / r->sampleRate)) r->requestFrame += QUANTUM;
    return atomic_load_explicit(&r->header[DONE], memory_order_acquire) != atomic_load_explicit(&r->header[READ], memory_order_relaxed);
}

- (void)abandonOffline {
    WorkletRender *r = [self render];
    if (!r) return;
    r->finished = YES;
    [self close];
}

- (void)dealloc {
    [self close];
    NSCAudioContext *ctx = self.context;
    if (_active) [ctx decrementActiveCount];
    AVAudioEngine *eng = ctx.engine;
    if (!ctx || !eng) return;
    NSMutableArray<AVAudioNode *> *nodes = [NSMutableArray arrayWithArray:_inputMixers ?: @[]];
    if (_unit) [nodes addObject:_unit];
    for (AVAudioNode *node in nodes) {
        if (![ctx isNode:node attachedToEngine:eng]) continue;
        @try { [eng disconnectNodeOutput:node]; } @catch (NSException *e) {}
        @try { [ctx detachNode:node fromEngine:eng]; } @catch (NSException *e) {}
    }
}

@end
