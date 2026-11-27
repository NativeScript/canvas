#import "NSCProcessorNode.h"
#import <AudioToolbox/AudioToolbox.h>
#import "NSCAudioContext.h"
#import "NSCAudioLog.h"
#import "NSCChannelMixerNode.h"

#define PROC_IN_CHANNELS 8
#define PROC_MAX_FRAMES 4096

typedef struct {
    UInt32 maxFrames;
    UInt32 outChannels;
    float *pulled;
    float *outs;
    AudioBufferList *abl;
    const _Atomic UInt32 *inLogical;
    _Atomic UInt32 outLogical;
    _Atomic(void *) process;
    const double *timeline;
    double nextFrame;
    double lastSampleTime;
    BOOL lastSilent;
} ProcRender;

static AVAudioFormat *ProcFormat(double rate, NSInteger channels) {
    if (channels <= 2) return [[AVAudioFormat alloc] initStandardFormatWithSampleRate:rate channels:(AVAudioChannelCount)MAX(1, channels)];
    AVAudioChannelLayout *layout = [AVAudioChannelLayout layoutWithLayoutTag:kAudioChannelLayoutTag_DiscreteInOrder | (AudioChannelLayoutTag)channels];
    return [[AVAudioFormat alloc] initStandardFormatWithSampleRate:rate channelLayout:layout];
}

static AVAudioFormat *pendingIn;
static AVAudioFormat *pendingOut;

@interface NSCProcessorUnit : AUAudioUnit
@property (nonatomic, readonly) ProcRender *render;
@end

@implementation NSCProcessorUnit {
    AUAudioUnitBusArray *_inputBusses;
    AUAudioUnitBusArray *_outputBusses;
    ProcRender *_render;
}

- (instancetype)initWithComponentDescription:(AudioComponentDescription)description options:(AudioComponentInstantiationOptions)options error:(NSError **)outError {
    if (!(self = [super initWithComponentDescription:description options:options error:outError])) return nil;
    AUAudioUnitBus *input = [[AUAudioUnitBus alloc] initWithFormat:pendingIn error:outError];
    AUAudioUnitBus *output = [[AUAudioUnitBus alloc] initWithFormat:pendingOut error:outError];
    if (!input || !output) return nil;
    _inputBusses = [[AUAudioUnitBusArray alloc] initWithAudioUnit:self busType:AUAudioUnitBusTypeInput busses:@[ input ]];
    _outputBusses = [[AUAudioUnitBusArray alloc] initWithAudioUnit:self busType:AUAudioUnitBusTypeOutput busses:@[ output ]];
    _render = calloc(1, sizeof(ProcRender));
    _render->abl = calloc(1, sizeof(AudioBufferList) + (PROC_IN_CHANNELS - 1) * sizeof(AudioBuffer));
    _render->outChannels = pendingOut.channelCount;
    _render->lastSampleTime = -1.0;
    atomic_init(&_render->outLogical, 1);
    self.maximumFramesToRender = PROC_MAX_FRAMES;
    return self;
}

- (AUAudioUnitBusArray *)inputBusses { return _inputBusses; }
- (AUAudioUnitBusArray *)outputBusses { return _outputBusses; }
- (BOOL)canProcessInPlace { return NO; }

- (BOOL)shouldChangeToFormat:(AVAudioFormat *)format forBus:(AUAudioUnitBus *)bus {
    if (bus.busType == AUAudioUnitBusTypeInput) return format.channelCount == PROC_IN_CHANNELS;
    return format.channelCount == _render->outChannels;
}

- (BOOL)allocateRenderResourcesAndReturnError:(NSError **)outError {
    if (![super allocateRenderResourcesAndReturnError:outError]) return NO;
    ProcRender *r = _render;
    UInt32 frames = MAX((UInt32)self.maximumFramesToRender, 128u);
    if (frames > r->maxFrames) {
        free(r->pulled);
        free(r->outs);
        r->pulled = calloc((size_t)PROC_IN_CHANNELS * frames, sizeof(float));
        r->outs = calloc((size_t)MAX(r->outChannels, 2u) * frames, sizeof(float));
        r->maxFrames = frames;
    }
    return r->pulled && r->outs;
}

- (AUInternalRenderBlock)internalRenderBlock {
    ProcRender *r = _render;
    return ^AUAudioUnitStatus(AudioUnitRenderActionFlags *flags, const AudioTimeStamp *ts, AUAudioFrameCount frames, NSInteger bus, AudioBufferList *out, const AURenderEvent *events, AURenderPullInputBlock pull) {
        if (!r->pulled || frames > r->maxFrames) return kAudioUnitErr_TooManyFramesToProcess;
        BOOL timed = ts && (ts->mFlags & kAudioTimeStampSampleTimeValid);
        double sampleTime = timed ? ts->mSampleTime : -2.0;
        UInt32 C = r->outChannels;
        float *outs[16];
        for (UInt32 c = 0; c < C && c < 16; c++) outs[c] = r->outs + c * r->maxFrames;
        // A repeated pull for the same timestamp (fan-out) plays what the first rendered.
        if (sampleTime < 0.0 || sampleTime != r->lastSampleTime) {
            AudioBufferList *abl = r->abl;
            abl->mNumberBuffers = PROC_IN_CHANNELS;
            for (UInt32 c = 0; c < PROC_IN_CHANNELS; c++) {
                abl->mBuffers[c].mNumberChannels = 1;
                abl->mBuffers[c].mData = r->pulled + c * r->maxFrames;
                abl->mBuffers[c].mDataByteSize = frames * sizeof(float);
            }
            AudioUnitRenderActionFlags pullFlags = 0;
            OSStatus status = pull ? pull(&pullFlags, ts, frames, 0, abl) : kAudioUnitErr_NoConnection;
            BOOL silent = status != noErr || (pullFlags & kAudioUnitRenderAction_OutputIsSilence);
            float *ins[PROC_IN_CHANNELS];
            for (UInt32 c = 0; c < PROC_IN_CHANNELS; c++) {
                float *mine = r->pulled + c * r->maxFrames;
                if (silent || !abl->mBuffers[c].mData) memset(mine, 0, frames * sizeof(float));
                else if (abl->mBuffers[c].mData != mine) memcpy(mine, abl->mBuffers[c].mData, frames * sizeof(float));
                ins[c] = mine;
            }
            UInt32 inChannels = r->inLogical ? atomic_load_explicit(r->inLogical, memory_order_acquire) : 1;
            inChannels = MAX(1u, MIN(inChannels, (UInt32)PROC_IN_CHANNELS));
            double frame = timed ? NSCContextFrame(ts->mSampleTime, r->timeline) : r->nextFrame;
            r->nextFrame = frame + frames;
            for (UInt32 c = 0; c < C; c++) memset(outs[c], 0, frames * sizeof(float));
            NSCProcessBlock process = (__bridge NSCProcessBlock)atomic_load_explicit(&r->process, memory_order_acquire);
            UInt32 wrote = process ? process((int64_t)llround(frame), frames, ins, inChannels, outs, C) : 0;
            if (!process) {
                for (UInt32 c = 0; c < MIN(inChannels, C); c++) memcpy(outs[c], ins[c], frames * sizeof(float));
                wrote = MIN(inChannels, C);
            }
            wrote = MAX(1u, MIN(wrote, C));
            // Up-mix a mono result to the bus with the speakers rule; a wider bus gets silence.
            if (wrote == 1 && C == 2) memcpy(outs[1], outs[0], frames * sizeof(float));
            atomic_store_explicit(&r->outLogical, wrote, memory_order_release);
            r->lastSampleTime = sampleTime;
        }
        for (UInt32 c = 0; c < out->mNumberBuffers; c++) {
            float *from = outs[MIN(c, C - 1)];
            if (out->mBuffers[c].mData) memcpy(out->mBuffers[c].mData, from, frames * sizeof(float));
            else out->mBuffers[c].mData = from;
            out->mBuffers[c].mDataByteSize = frames * sizeof(float);
        }
        return noErr;
    };
}

- (void)dealloc {
    ProcRender *r = _render;
    if (!r) return;
    void *process = atomic_load(&r->process);
    if (process) CFRelease(process);
    free(r->pulled);
    free(r->outs);
    free(r->abl);
    free(r);
}

@end

static AudioComponentDescription ProcDescription(void) {
    AudioComponentDescription description = {0};
    description.componentType = kAudioUnitType_Effect;
    description.componentSubType = 'nscx';
    description.componentManufacturer = 'NSCa';
    return description;
}

@implementation NSCProcessorNode {
    AVAudioUnitEffect *_unit;
    NSCChannelMixerNode *_port;
    // Pulls the unit each quantum without playing it.
    NSCChannelMixerNode *_drain;
    NSMutableArray *_retired;
    NSMutableData *_timelineKeep;
}

- (instancetype)initWithContext:(NSCAudioContext *)context outputChannels:(NSInteger)outputChannels {
    static dispatch_once_t once;
    dispatch_once(&once, ^{
        [AUAudioUnit registerSubclass:[NSCProcessorUnit class] asComponentDescription:ProcDescription() name:@"NativeScript: processor" version:1];
    });
    double rate = context.sampleRate > 0 ? context.sampleRate : 44100.0;
    outputChannels = MAX(1, MIN(outputChannels, (NSInteger)16));
    AVAudioFormat *inFormat = ProcFormat(rate, PROC_IN_CHANNELS);
    pendingIn = inFormat;
    pendingOut = ProcFormat(rate, outputChannels);
    // A registered in-process unit instantiates synchronously.
    AVAudioUnitEffect *unit = [[AVAudioUnitEffect alloc] initWithAudioComponentDescription:ProcDescription()];
    pendingIn = nil;
    pendingOut = nil;
    NSCProcessorUnit *au = (NSCProcessorUnit *)unit.AUAudioUnit;
    if (![au isKindOfClass:[NSCProcessorUnit class]]) {
        NSCLogError(@"NSCProcessorNode: could not instantiate the processor audio unit");
        return nil;
    }
    NSCChannelMixerNode *port = [[NSCChannelMixerNode alloc] initWithContext:context mode:NSCMixModeSum channels:PROC_IN_CHANNELS discrete:NO];
    if (!port) return nil;
    if ([context isKindOfClass:[NSCOfflineAudioContext class]]) [(NSCOfflineAudioContext *)context willConnect];
    @try { [context.engine attachNode:unit]; } @catch (NSException *e) { return nil; }
    if (context.engine.isRunning) NSCEnsureAllocated(unit);
    if (!(self = [super initWithContext:context node:unit])) return nil;
    _unit = unit;
    _port = port;
    _outputChannels = outputChannels;
    _retired = [NSMutableArray array];
    _timelineKeep = context.timelineCell;
    au.render->inLogical = [port mixedChannels];
    au.render->timeline = (const double *)_timelineKeep.bytes;
    NSCConnectAdding(context, port.avNode, 0, unit, 0, inFormat);
    [self channelConfigChanged];
    return self;
}

- (ProcRender *)render {
    return ((NSCProcessorUnit *)_unit.AUAudioUnit).render;
}

- (void)setProcess:(NSCProcessBlock)process {
    ProcRender *r = [self render];
    if (!r) return;
    id copied = [process copy];
    void *old = atomic_exchange(&r->process, copied ? (void *)CFBridgingRetain(copied) : NULL);
    // The render thread may still be running the old block.
    if (old) [_retired addObject:CFBridgingRelease(old)];
}

- (NSCChannelMixerNode *)channelPort { return _port; }

- (const _Atomic UInt32 *)logicalChannelCellForOutput:(NSInteger)output {
    ProcRender *r = [self render];
    return r ? &r->outLogical : NULL;
}

- (void)handleConnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    [_port handleConnectFrom:source output:output input:nil];
}

- (void)handleDisconnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    [_port handleDisconnectFrom:source output:output input:nil];
}

// Param inputs render before this node reads its params.
- (void)attachParamSink:(AVAudioNode *)sink {
    [_port connectEarly:sink format:self.context.format];
}

- (void)keepPulled {
    NSCAudioContext *ctx = self.context;
    AVAudioEngine *engine = ctx.engine;
    if (!engine) return;
    if (!_drain) {
        _drain = [[NSCChannelMixerNode alloc] initWithContext:ctx mode:NSCMixModeSum channels:1 discrete:NO];
        AVAudioMixerNode *main = engine.mainMixerNode;
        if (!_drain || !NSCConnectAdding(ctx, _drain.avNode, 0, main, main.nextAvailableInputBus, ctx.format)) return;
    }
    [_drain connectEarly:_unit format:[_unit outputFormatForBus:0]];
}

- (void)disconnect {
    [super disconnect];
    if (_drain) [self keepPulled];
}

- (void)disconnectOutput:(NSNumber *)output {
    [super disconnectOutput:output];
    if (_drain) [self keepPulled];
}

- (void)detachHelpers {
    NSCAudioContext *ctx = self.context;
    AVAudioEngine *engine = ctx.engine;
    if (!engine) return;
    for (AVAudioNode *node in @[ _unit ?: (id)[NSNull null], _drain.avNode ?: (id)[NSNull null] ]) {
        if ((id)node == [NSNull null] || ![ctx isNode:node attachedToEngine:engine]) continue;
        @try { [engine disconnectNodeOutput:node]; } @catch (NSException *e) {}
        @try { [ctx detachNode:node fromEngine:engine]; } @catch (NSException *e) {}
    }
}

@end
