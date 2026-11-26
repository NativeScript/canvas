#import "NSCChannelMixerNode.h"
#import <AudioToolbox/AudioToolbox.h>
#import <stdatomic.h>
#import <math.h>
#import "NSCAudioContext.h"
#import "NSCAudioLog.h"

#define MIX_BUSSES 32
#define MIX_CHANNELS 8

typedef struct {
    NSCMixMode mode;
    _Atomic BOOL discrete;
    _Atomic UInt32 channels;
    // Sum mode: mix to at most the widest connection's channels (clamped-max).
    _Atomic int clamp;
    // Pick mode: the channel of bus 0 this node plays.
    _Atomic int pick;
    // Per input bus: connected, its format's channels, the merger channel it feeds, rendered first.
    _Atomic int used[MIX_BUSSES];
    _Atomic int busChannels[MIX_BUSSES];
    _Atomic int target[MIX_BUSSES];
    _Atomic int early[MIX_BUSSES];
    // The source's own channel count when it plays fewer than its bus carries.
    _Atomic(const _Atomic UInt32 *) logical[MIX_BUSSES];
    UInt32 maxFrames;
    float *pulled;
    float *mixed;
    float *outs;
    AudioBufferList *abl;
    double lastSampleTime;
    _Atomic UInt32 mixedChannels;
    BOOL silent;
} MixRender;

// Web Audio's speakers gains from n to m channels; NO for counts it leaves to the discrete rules.
static BOOL SpeakerGains(UInt32 n, UInt32 m, float g[MIX_CHANNELS][MIX_CHANNELS]) {
    memset(g, 0, sizeof(float) * MIX_CHANNELS * MIX_CHANNELS);
    BOOL nKnown = n == 1 || n == 2 || n == 4 || n == 6, mKnown = m == 1 || m == 2 || m == 4 || m == 6;
    if (!nKnown || !mKnown) return NO;
    const float h = (float)M_SQRT1_2;
    if (n == m) {
        for (UInt32 i = 0; i < n; i++) g[i][i] = 1.0f;
    } else if (n == 1) {
        if (m == 6) g[2][0] = 1.0f;
        else g[0][0] = g[1][0] = 1.0f;
    } else if (m == 1) {
        if (n == 2) g[0][0] = g[0][1] = 0.5f;
        else if (n == 4) g[0][0] = g[0][1] = g[0][2] = g[0][3] = 0.25f;
        else { g[0][0] = g[0][1] = h; g[0][2] = 1.0f; g[0][4] = g[0][5] = 0.5f; }
    } else if (n == 2) {
        g[0][0] = g[1][1] = 1.0f;
    } else if (n == 4 && m == 2) {
        g[0][0] = g[0][2] = 0.5f; g[1][1] = g[1][3] = 0.5f;
    } else if (n == 4 && m == 6) {
        g[0][0] = g[1][1] = 1.0f; g[4][2] = g[5][3] = 1.0f;
    } else if (n == 6 && m == 2) {
        g[0][0] = 1.0f; g[0][2] = g[0][4] = h; g[1][1] = 1.0f; g[1][2] = g[1][5] = h;
    } else {
        g[0][0] = 1.0f; g[0][2] = h; g[1][1] = 1.0f; g[1][2] = h; g[2][4] = g[3][5] = 1.0f;
    }
    return YES;
}

static void MixAdd(float *const *src, UInt32 n, float *const *dst, UInt32 m, UInt32 frames, BOOL discrete) {
    float g[MIX_CHANNELS][MIX_CHANNELS];
    if (discrete || !SpeakerGains(n, m, g)) {
        for (UInt32 c = 0; c < MIN(n, m); c++) {
            for (UInt32 k = 0; k < frames; k++) dst[c][k] += src[c][k];
        }
        return;
    }
    for (UInt32 j = 0; j < m; j++) {
        for (UInt32 i = 0; i < n; i++) {
            float w = g[j][i];
            if (w == 0.0f) continue;
            if (w == 1.0f) for (UInt32 k = 0; k < frames; k++) dst[j][k] += src[i][k];
            else for (UInt32 k = 0; k < frames; k++) dst[j][k] += w * src[i][k];
        }
    }
}

static float *Mixed(MixRender *r, UInt32 c) { return r->mixed + c * r->maxFrames; }

// A bus's channels as its source plays them.
static UInt32 BusChannels(MixRender *r, int b) {
    UInt32 n = (UInt32)MAX(1, MIN(atomic_load_explicit(&r->busChannels[b], memory_order_relaxed), MIX_CHANNELS));
    const _Atomic UInt32 *cell = atomic_load_explicit(&r->logical[b], memory_order_acquire);
    UInt32 logical = cell ? atomic_load_explicit(cell, memory_order_relaxed) : 0;
    return logical >= 1 && logical < n ? logical : n;
}

static void Mix(MixRender *r, const AudioTimeStamp *ts, AUAudioFrameCount frames, AURenderPullInputBlock pull) {
    UInt32 C = atomic_load_explicit(&r->channels, memory_order_relaxed);
    if (r->mode == NSCMixModeSum && atomic_load_explicit(&r->clamp, memory_order_relaxed)) {
        UInt32 widest = 1;
        for (int b = 0; b < MIX_BUSSES; b++) {
            if (!atomic_load_explicit(&r->used[b], memory_order_acquire) || atomic_load_explicit(&r->early[b], memory_order_relaxed)) continue;
            widest = MAX(widest, BusChannels(r, b));
        }
        C = MIN(C, widest);
    }
    if (r->mode == NSCMixModePick) C = 1;
    for (UInt32 c = 0; c < MIX_CHANNELS; c++) memset(Mixed(r, c), 0, frames * sizeof(float));
    UInt32 mixedCount = r->mode == NSCMixModePass ? 0 : C;
    r->silent = YES;
    float *mixed[MIX_CHANNELS];
    for (UInt32 c = 0; c < MIX_CHANNELS; c++) mixed[c] = Mixed(r, c);
    for (int pass = 0; pass < 2; pass++) {
        for (int b = 0; b < MIX_BUSSES; b++) {
            if (!atomic_load_explicit(&r->used[b], memory_order_acquire)) continue;
            BOOL early = atomic_load_explicit(&r->early[b], memory_order_relaxed) != 0;
            if (early != (pass == 0)) continue;
            UInt32 n = (UInt32)MAX(1, MIN(atomic_load_explicit(&r->busChannels[b], memory_order_relaxed), MIX_CHANNELS));
            AudioBufferList *abl = r->abl;
            abl->mNumberBuffers = n;
            for (UInt32 c = 0; c < n; c++) {
                abl->mBuffers[c].mNumberChannels = 1;
                abl->mBuffers[c].mData = r->pulled + c * r->maxFrames;
                abl->mBuffers[c].mDataByteSize = frames * sizeof(float);
            }
            AudioUnitRenderActionFlags flags = 0;
            OSStatus status = pull ? pull(&flags, ts, frames, b, abl) : kAudioUnitErr_NoConnection;
            UInt32 played = BusChannels(r, b);
            if (r->mode == NSCMixModePass && b == 0) mixedCount = played;
            if (early || status != noErr || (flags & kAudioUnitRenderAction_OutputIsSilence)) continue;
            r->silent = NO;
            float *src[MIX_CHANNELS];
            for (UInt32 c = 0; c < n; c++) src[c] = abl->mBuffers[c].mData ? (float *)abl->mBuffers[c].mData : r->pulled + c * r->maxFrames;
            n = played;
            if (r->mode == NSCMixModePass) {
                if (b == 0) MixAdd(src, n, mixed, n, frames, YES);
            } else if (r->mode == NSCMixModePick) {
                int p = atomic_load_explicit(&r->pick, memory_order_relaxed);
                if (b == 0 && p >= 0 && (UInt32)p < n) MixAdd(&src[p], 1, mixed, 1, frames, YES);
            } else if (r->mode == NSCMixModeMerge) {
                int t = atomic_load_explicit(&r->target[b], memory_order_relaxed);
                if (t >= 0 && (UInt32)t < C) MixAdd(src, n, &mixed[t], 1, frames, NO);
            } else {
                MixAdd(src, n, mixed, C, frames, r->discrete);
            }
        }
    }
    atomic_store_explicit(&r->mixedChannels, mixedCount, memory_order_release);
}

// Bus formats for the unit being instantiated; set around the synchronous instantiation.
static AVAudioFormat *pendingInput;
static AVAudioFormat *pendingOutput;

@interface NSCChannelMixUnit : AUAudioUnit
@property (nonatomic, readonly) MixRender *render;
@end

@implementation NSCChannelMixUnit {
    AUAudioUnitBusArray *_inputBusses;
    AUAudioUnitBusArray *_outputBusses;
    MixRender *_render;
}

- (instancetype)initWithComponentDescription:(AudioComponentDescription)description options:(AudioComponentInstantiationOptions)options error:(NSError **)outError {
    if (!(self = [super initWithComponentDescription:description options:options error:outError])) return nil;
    NSMutableArray<AUAudioUnitBus *> *inputs = [NSMutableArray array];
    for (NSInteger i = 0; i < MIX_BUSSES; i++) {
        AUAudioUnitBus *bus = [[AUAudioUnitBus alloc] initWithFormat:pendingInput error:outError];
        if (!bus) return nil;
        [inputs addObject:bus];
    }
    AUAudioUnitBus *output = [[AUAudioUnitBus alloc] initWithFormat:pendingOutput error:outError];
    if (!output) return nil;
    _inputBusses = [[AUAudioUnitBusArray alloc] initWithAudioUnit:self busType:AUAudioUnitBusTypeInput busses:inputs];
    _outputBusses = [[AUAudioUnitBusArray alloc] initWithAudioUnit:self busType:AUAudioUnitBusTypeOutput busses:@[ output ]];
    _render = calloc(1, sizeof(MixRender));
    _render->abl = calloc(1, sizeof(AudioBufferList) + (MIX_CHANNELS - 1) * sizeof(AudioBuffer));
    _render->lastSampleTime = -1.0;
    for (NSInteger i = 0; i < MIX_BUSSES; i++) atomic_init(&_render->busChannels[i], (int)pendingInput.channelCount);
    self.maximumFramesToRender = 4096;
    return self;
}

- (AUAudioUnitBusArray *)inputBusses { return _inputBusses; }
- (AUAudioUnitBusArray *)outputBusses { return _outputBusses; }
- (BOOL)canProcessInPlace { return NO; }

// Connections set a bus's format while the engine renders; the render reads the channel count.
- (BOOL)shouldChangeToFormat:(AVAudioFormat *)format forBus:(AUAudioUnitBus *)bus {
    if (format.channelCount == 0 || format.channelCount > MIX_CHANNELS) return NO;
    if (bus.busType == AUAudioUnitBusTypeInput && bus.index < MIX_BUSSES) {
        atomic_store_explicit(&_render->busChannels[bus.index], (int)format.channelCount, memory_order_release);
    }
    return YES;
}

- (BOOL)allocateRenderResourcesAndReturnError:(NSError **)outError {
    if (![super allocateRenderResourcesAndReturnError:outError]) return NO;
    MixRender *r = _render;
    UInt32 frames = MAX((UInt32)self.maximumFramesToRender, 128u);
    if (frames > r->maxFrames) {
        free(r->pulled);
        free(r->mixed);
        free(r->outs);
        r->pulled = calloc((size_t)MIX_CHANNELS * frames, sizeof(float));
        r->mixed = calloc((size_t)MIX_CHANNELS * frames, sizeof(float));
        r->outs = calloc((size_t)MIX_CHANNELS * frames, sizeof(float));
        r->maxFrames = frames;
    }
    for (NSUInteger i = 0; i < MIN(_inputBusses.count, (NSUInteger)MIX_BUSSES); i++) {
        atomic_store_explicit(&r->busChannels[i], (int)_inputBusses[i].format.channelCount, memory_order_release);
    }
    return r->pulled && r->mixed && r->outs;
}

- (AUInternalRenderBlock)internalRenderBlock {
    MixRender *r = _render;
    return ^AUAudioUnitStatus(AudioUnitRenderActionFlags *flags, const AudioTimeStamp *ts, AUAudioFrameCount frames, NSInteger bus, AudioBufferList *out, const AURenderEvent *events, AURenderPullInputBlock pull) {
        if (!r->pulled || frames > r->maxFrames || bus != 0) return kAudioUnitErr_TooManyFramesToProcess;
        double sampleTime = ts && (ts->mFlags & kAudioTimeStampSampleTimeValid) ? ts->mSampleTime : -2.0;
        // A repeated pull for the same timestamp plays the same mix.
        if (sampleTime != r->lastSampleTime || sampleTime < 0.0) {
            Mix(r, ts, frames, pull);
            r->lastSampleTime = sampleTime;
        }
        UInt32 k = MIN(out->mNumberBuffers, (UInt32)MIX_CHANNELS);
        float *dst[MIX_CHANNELS];
        for (UInt32 c = 0; c < out->mNumberBuffers; c++) {
            float *mine = r->outs + MIN(c, (UInt32)MIX_CHANNELS - 1) * r->maxFrames;
            if (!out->mBuffers[c].mData) out->mBuffers[c].mData = mine;
            out->mBuffers[c].mDataByteSize = frames * sizeof(float);
            memset(out->mBuffers[c].mData, 0, frames * sizeof(float));
            if (c < MIX_CHANNELS) dst[c] = (float *)out->mBuffers[c].mData;
        }
        float *src[MIX_CHANNELS];
        UInt32 n = atomic_load_explicit(&r->mixedChannels, memory_order_relaxed);
        for (UInt32 c = 0; c < n; c++) src[c] = Mixed(r, c);
        if (n > 0 && !r->silent) MixAdd(src, n, dst, k, frames, NO);
        else *flags |= kAudioUnitRenderAction_OutputIsSilence;
        return noErr;
    };
}

- (void)dealloc {
    MixRender *r = _render;
    if (!r) return;
    free(r->pulled);
    free(r->mixed);
    free(r->outs);
    free(r->abl);
    free(r);
}

@end

static AudioComponentDescription MixDescription(void) {
    AudioComponentDescription description = {0};
    description.componentType = kAudioUnitType_Effect;
    description.componentSubType = 'nscm';
    description.componentManufacturer = 'NSCa';
    return description;
}

static AVAudioFormat *MixFormat(double rate, NSInteger channels) {
    channels = MAX(1, MIN(channels, (NSInteger)MIX_CHANNELS));
    if (channels <= 2) return [[AVAudioFormat alloc] initStandardFormatWithSampleRate:rate channels:(AVAudioChannelCount)channels];
    AVAudioChannelLayout *layout = [AVAudioChannelLayout layoutWithLayoutTag:kAudioChannelLayoutTag_DiscreteInOrder | (AudioChannelLayoutTag)channels];
    return [[AVAudioFormat alloc] initStandardFormatWithSampleRate:rate channelLayout:layout];
}

@implementation NSCChannelMixerNode {
    AVAudioUnit *_unit;
    NSCMixMode _mode;
    // One per connection: source node, its bus, this node's bus.
    NSMutableArray<NSArray *> *_links;
    AVAudioSourceNode *_silence;
}

- (instancetype)initWithContext:(NSCAudioContext *)context mode:(NSCMixMode)mode channels:(NSInteger)channels discrete:(BOOL)discrete {
    static dispatch_once_t once;
    dispatch_once(&once, ^{
        [AUAudioUnit registerSubclass:[NSCChannelMixUnit class] asComponentDescription:MixDescription() name:@"NativeScript: channel mixer" version:1];
    });
    channels = MAX(1, MIN(channels, (NSInteger)MIX_CHANNELS));
    double rate = context.format.sampleRate > 0 ? context.format.sampleRate : 44100.0;
    pendingInput = MixFormat(rate, 1);
    pendingOutput = MixFormat(rate, channels);
    // A registered in-process unit instantiates synchronously.
    AVAudioUnitEffect *unit = [[AVAudioUnitEffect alloc] initWithAudioComponentDescription:MixDescription()];
    pendingInput = nil;
    pendingOutput = nil;
    NSCChannelMixUnit *au = (NSCChannelMixUnit *)unit.AUAudioUnit;
    if (![au isKindOfClass:[NSCChannelMixUnit class]]) {
        NSCLogError(@"NSCChannelMixerNode: could not instantiate the mixer audio unit");
        return nil;
    }
    MixRender *r = au.render;
    r->mode = mode;
    r->discrete = discrete;
    r->channels = (UInt32)channels;
    atomic_init(&r->pick, 0);
    for (NSInteger i = 0; i < MIX_BUSSES; i++) atomic_init(&r->target[i], -1);

    if (!(self = [super initWithContext:context node:unit])) return nil;
    _unit = unit;
    _mode = mode;
    _channels = channels;
    _links = [NSMutableArray array];
    if ([context isKindOfClass:[NSCOfflineAudioContext class]]) [(NSCOfflineAudioContext *)context willConnect];
    [context.engine attachNode:unit];
    if (context.engine.isRunning) NSCEnsureAllocated(unit);
    // The engine skips an effect with no input.
    _silence = [[AVAudioSourceNode alloc] initWithFormat:context.format renderBlock:^OSStatus(BOOL *isSilence, const AudioTimeStamp *ts, AVAudioFrameCount frames, AudioBufferList *out) {
        for (UInt32 c = 0; c < out->mNumberBuffers; c++) memset(out->mBuffers[c].mData, 0, out->mBuffers[c].mDataByteSize);
        *isSilence = YES;
        return noErr;
    }];
    [context.engine attachNode:_silence];
    [self connectNode:_silence bus:0 toBus:MIX_BUSSES - 1 format:context.format target:-1 early:YES];
    return self;
}

- (MixRender *)render {
    return ((NSCChannelMixUnit *)_unit.AUAudioUnit).render;
}

- (const _Atomic UInt32 *)mixedChannels {
    MixRender *r = [self render];
    return r ? &r->mixedChannels : NULL;
}

- (const _Atomic UInt32 *)logicalChannelCellForOutput:(NSInteger)output {
    return [self mixedChannels];
}

- (void)setMixChannels:(NSInteger)channels {
    MixRender *r = [self render];
    if (r) atomic_store(&r->channels, (UInt32)MAX(1, MIN(channels, (NSInteger)MIX_CHANNELS)));
}

- (void)configureCount:(NSInteger)count mode:(NSInteger)mode discrete:(BOOL)discrete {
    [self setMixChannels:mode == 0 ? MIX_CHANNELS : count];
    [self setClamp:mode != 2];
    [self setDiscrete:discrete];
}

- (void)setDiscrete:(BOOL)discrete {
    MixRender *r = [self render];
    if (r) atomic_store(&r->discrete, discrete);
}

- (void)setClamp:(BOOL)clamp {
    MixRender *r = [self render];
    if (r) atomic_store(&r->clamp, clamp ? 1 : 0);
}

- (void)setPick:(NSInteger)pick {
    MixRender *r = [self render];
    if (r) atomic_store(&r->pick, (int)pick);
}

// A bus whose connection went away with its source's disconnect() is free again.
- (NSInteger)freeBus {
    MixRender *r = [self render];
    AVAudioEngine *engine = self.context.engine;
    for (NSInteger b = _mode == NSCMixModePass ? 1 : 0; b < MIX_BUSSES; b++) {
        if (!atomic_load(&r->used[b])) return b;
        if (atomic_load(&r->early[b])) continue;
        if (![self bus:b connectedIn:engine]) {
            [self forgetBus:b];
            return b;
        }
    }
    return -1;
}

// Asks the source: inputConnectionPointForNode: can miss a live connection.
- (BOOL)bus:(NSInteger)b connectedIn:(AVAudioEngine *)engine {
    for (NSArray *link in _links) {
        if ([link[2] integerValue] != b) continue;
        NSArray<AVAudioConnectionPoint *> *points = nil;
        @try { points = [engine outputConnectionPointsForNode:link[0] outputBus:[link[1] unsignedIntValue]]; } @catch (NSException *e) {}
        for (AVAudioConnectionPoint *point in points) {
            if (point.node == _unit && point.bus == (AVAudioNodeBus)b) return YES;
        }
    }
    return NO;
}

- (void)forgetBus:(NSInteger)b {
    MixRender *r = [self render];
    atomic_store(&r->used[b], 0);
    atomic_store(&r->early[b], 0);
    atomic_store(&r->target[b], -1);
    atomic_store(&r->logical[b], NULL);
    for (NSInteger i = (NSInteger)_links.count - 1; i >= 0; i--) {
        if ([_links[i][2] integerValue] == b) [_links removeObjectAtIndex:(NSUInteger)i];
    }
}

- (BOOL)connectNode:(AVAudioNode *)node bus:(AVAudioNodeBus)from toBus:(NSInteger)b format:(AVAudioFormat *)format target:(NSInteger)target early:(BOOL)early {
    return [self connectNode:node bus:from toBus:b format:format target:target early:early logical:NULL];
}

- (BOOL)connectNode:(AVAudioNode *)node bus:(AVAudioNodeBus)from toBus:(NSInteger)b format:(AVAudioFormat *)format target:(NSInteger)target early:(BOOL)early logical:(const _Atomic UInt32 *)logical {
    MixRender *r = [self render];
    if (b < 0 || !r) return NO;
    atomic_store(&r->logical[b], logical);
    atomic_store(&r->busChannels[b], (int)MAX(1u, MIN(format.channelCount, (AVAudioChannelCount)MIX_CHANNELS)));
    atomic_store(&r->target[b], (int)target);
    atomic_store(&r->early[b], early ? 1 : 0);
    if (!NSCConnectAdding(self.context, node, from, _unit, (AVAudioNodeBus)b, format)) {
        atomic_store(&r->early[b], 0);
        return NO;
    }
    atomic_store_explicit(&r->used[b], 1, memory_order_release);
    [_links addObject:@[ node, @(from), @(b) ]];
    return YES;
}

- (void)connectEarly:(AVAudioNode *)node format:(AVAudioFormat *)format {
    [self connectNode:node bus:0 toBus:[self freeBus] format:format target:-1 early:YES];
}

- (void)connectMain:(AVAudioNode *)node bus:(AVAudioNodeBus)bus format:(AVAudioFormat *)format {
    [self connectMain:node bus:bus format:format logical:NULL];
}

- (void)connectMain:(AVAudioNode *)node bus:(AVAudioNodeBus)bus format:(AVAudioFormat *)format logical:(const _Atomic UInt32 *)logical {
    [self forgetBus:0];
    [self connectNode:node bus:bus toBus:0 format:format target:-1 early:NO logical:logical];
}

- (void)handleConnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    AVAudioNodeBus from = 0;
    AVAudioNode *node = [source outputNodeFor:output bus:&from];
    if (!node || _mode == NSCMixModePass || _mode == NSCMixModePick) return;
    if (from >= node.numberOfOutputs) return;
    NSInteger target = _mode == NSCMixModeMerge ? MAX(0, input ? input.integerValue : 0) : -1;
    if (target >= _channels) return;
    MixRender *r = [self render];
    for (NSArray *link in _links) {
        NSInteger b = [link[2] integerValue];
        // Web Audio ignores a repeated connection.
        if (link[0] == node && [link[1] unsignedIntValue] == from && atomic_load(&r->target[b]) == target) return;
    }
    AVAudioFormat *format = [source formatForOutput:(AVAudioNodeBus)MAX(0, output ? output.intValue : 0)];
    if (format.channelCount > MIX_CHANNELS) format = self.context.format;
    NSInteger outputIndex = MAX(0, output ? output.intValue : 0);
    if ([self connectNode:node bus:from toBus:[self freeBus] format:format target:target early:NO logical:[source logicalChannelCellForOutput:outputIndex]]) {
        [source handleConnectedTo:self output:output input:input];
    }
}

- (void)handleDisconnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    MixRender *r = [self render];
    [source eachOutputNode:output block:^(AVAudioNode *node, NSNumber *bus) {
        for (NSArray *link in [self->_links copy]) {
            NSInteger b = [link[2] integerValue];
            if (link[0] != node) continue;
            if (bus && [link[1] integerValue] != bus.integerValue) continue;
            if (input && self->_mode == NSCMixModeMerge && atomic_load(&r->target[b]) != input.integerValue) continue;
            NSCDisconnectRemoving(self.context, node, link[1], self->_unit, @(b));
            [self forgetBus:b];
        }
    }];
    [source handleDisconnectedFrom:self output:output input:input];
}

- (void)dealloc {
    NSCAudioContext *ctx = self.context;
    AVAudioEngine *engine = ctx.engine;
    if (!engine || !_unit || ![ctx isNode:_unit attachedToEngine:engine]) return;
    @try { [engine disconnectNodeOutput:_unit]; } @catch (NSException *e) {}
    @try { [ctx detachNode:_unit fromEngine:engine]; } @catch (NSException *e) {}
}

@end
