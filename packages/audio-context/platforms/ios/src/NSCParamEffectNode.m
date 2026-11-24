#import "NSCParamEffectNode.h"
#import <AudioToolbox/AudioToolbox.h>
#import <math.h>
#import <stdatomic.h>
#import "NSCAudioContext.h"
#import "NSCAudioLog.h"

#define MAX_CHANNELS 32
#define MAX_FRAMES 4096

// Mono downmix of the audio connected to a param, written by its sink and read by its effect.
typedef struct {
    _Atomic double sampleTime;
    UInt32 frames;
    float values[MAX_FRAMES];
} ParamInput;

// Renders a param's inputs into a ParamInput, outputting silence; not a public kind.
static const NSCParamEffectKind KindSink = (NSCParamEffectKind)2;

typedef struct {
    NSCParamEffectKind kind;
    CFTypeRef param;
    double maxDelayTime;
    double sampleRate;
    UInt32 maxFrames;
    UInt32 channels;
    float *pulled;
    float *rendered;
    double *values;
    AudioBufferList *pullList;
    // Delay line: `lineLength` frames per channel, all channels written at `write`.
    float *line;
    int64_t lineLength;
    int64_t write;
    // The context's timeline cell (retained).
    CFTypeRef timelineCell;
    const double *timeline;
    double nextFrame;
    // Shared with the param's sink (retained NSMutableData), NULL until something connects to the param.
    CFTypeRef paramInputData;
    _Atomic(ParamInput *) paramInput;
} EffectRender;

static void ProcessGain(EffectRender *r, AUAudioFrameCount frames) {
    for (UInt32 c = 0; c < r->channels; c++) {
        const float *in = r->pulled + c * r->maxFrames;
        float *out = r->rendered + c * r->maxFrames;
        for (AUAudioFrameCount k = 0; k < frames; k++) out[k] = in[k] * (float)r->values[k];
    }
}

// Linear interpolation between the two frames around delayTime; a whole number of frames is exact.
static void ProcessDelay(EffectRender *r, AUAudioFrameCount frames) {
    int64_t length = r->lineLength;
    double maxFrames = (double)(length - 2);
    for (AUAudioFrameCount k = 0; k < frames; k++) {
        double d = r->values[k] * r->sampleRate;
        d = isfinite(d) ? MAX(0.0, MIN(d, maxFrames)) : 0.0;
        double nearest = nearbyint(d);
        if (fabs(d - nearest) < 1e-6) d = nearest;
        r->values[k] = d;
    }
    for (UInt32 c = 0; c < r->channels; c++) {
        const float *in = r->pulled + c * r->maxFrames;
        float *out = r->rendered + c * r->maxFrames;
        float *line = r->line + c * length;
        int64_t w = r->write;
        for (AUAudioFrameCount k = 0; k < frames; k++) {
            line[w] = in[k];
            double d = r->values[k];
            int64_t whole = (int64_t)d;
            double frac = d - (double)whole;
            int64_t r0 = w - whole;
            if (r0 < 0) r0 += length;
            if (frac == 0.0) {
                out[k] = line[r0];
            } else {
                int64_t r1 = r0 == 0 ? length - 1 : r0 - 1;
                out[k] = (float)((1.0 - frac) * line[r0] + frac * line[r1]);
            }
            if (++w == length) w = 0;
        }
    }
    r->write = (r->write + frames) % length;
}

// The sink renders in the same cycle, before this, as it feeds this unit's input mixer.
static void AddParamInput(EffectRender *r, const AudioTimeStamp *ts, AUAudioFrameCount frames) {
    ParamInput *in = atomic_load_explicit(&r->paramInput, memory_order_acquire);
    if (!in || !ts || atomic_load_explicit(&in->sampleTime, memory_order_acquire) != ts->mSampleTime || in->frames != frames) return;
    for (AUAudioFrameCount k = 0; k < frames; k++) r->values[k] += in->values[k];
}

static void WriteParamInput(EffectRender *r, const AudioTimeStamp *ts, AUAudioFrameCount frames, BOOL silent) {
    ParamInput *in = atomic_load_explicit(&r->paramInput, memory_order_acquire);
    if (!in || !ts) return;
    if (silent) {
        memset(in->values, 0, frames * sizeof(float));
    } else {
        float scale = 1.0f / (float)r->channels;
        for (AUAudioFrameCount k = 0; k < frames; k++) {
            float sum = 0.0f;
            for (UInt32 c = 0; c < r->channels; c++) sum += r->pulled[c * r->maxFrames + k];
            in->values[k] = sum * scale;
        }
    }
    in->frames = frames;
    atomic_store_explicit(&in->sampleTime, ts->mSampleTime, memory_order_release);
}

#pragma mark - The audio unit

// Bus format for the unit being instantiated; set around the synchronous instantiation.
static AVAudioFormat *pendingFormat;

@interface NSCParamEffectUnit : AUAudioUnit
@property (nonatomic, readonly) EffectRender *render;
@end

@implementation NSCParamEffectUnit {
    AUAudioUnitBusArray *_inputBusses;
    AUAudioUnitBusArray *_outputBusses;
    EffectRender *_render;
}

- (instancetype)initWithComponentDescription:(AudioComponentDescription)description options:(AudioComponentInstantiationOptions)options error:(NSError **)outError {
    if (!(self = [super initWithComponentDescription:description options:options error:outError])) return nil;
    AVAudioFormat *format = pendingFormat ?: [[AVAudioFormat alloc] initStandardFormatWithSampleRate:44100.0 channels:2];
    AUAudioUnitBus *input = [[AUAudioUnitBus alloc] initWithFormat:format error:outError];
    AUAudioUnitBus *output = [[AUAudioUnitBus alloc] initWithFormat:format error:outError];
    if (!input || !output) return nil;
    _inputBusses = [[AUAudioUnitBusArray alloc] initWithAudioUnit:self busType:AUAudioUnitBusTypeInput busses:@[ input ]];
    _outputBusses = [[AUAudioUnitBusArray alloc] initWithAudioUnit:self busType:AUAudioUnitBusTypeOutput busses:@[ output ]];
    _render = calloc(1, sizeof(EffectRender));
    _render->pullList = calloc(1, sizeof(AudioBufferList) + (MAX_CHANNELS - 1) * sizeof(AudioBuffer));
    self.maximumFramesToRender = MAX_FRAMES;
    return self;
}

- (AUAudioUnitBusArray *)inputBusses { return _inputBusses; }
- (AUAudioUnitBusArray *)outputBusses { return _outputBusses; }
- (BOOL)canProcessInPlace { return NO; }

// The delay line outlives render resources: an offline context stops the engine to connect.
- (BOOL)allocateRenderResourcesAndReturnError:(NSError **)outError {
    if (![super allocateRenderResourcesAndReturnError:outError]) return NO;
    EffectRender *r = _render;
    AVAudioFormat *format = _inputBusses[0].format;
    UInt32 channels = MAX(1u, MIN((UInt32)format.channelCount, (UInt32)MAX_CHANNELS));
    UInt32 frames = MAX((UInt32)self.maximumFramesToRender, 128u);
    double rate = format.sampleRate > 0.0 ? format.sampleRate : 44100.0;
    if (frames != r->maxFrames || channels != r->channels) {
        free(r->pulled);
        free(r->rendered);
        free(r->values);
        r->pulled = calloc((size_t)channels * frames, sizeof(float));
        r->rendered = calloc((size_t)channels * frames, sizeof(float));
        r->values = calloc(frames, sizeof(double));
        r->maxFrames = frames;
    }
    if (r->kind == NSCParamEffectKindDelay) {
        int64_t length = (int64_t)ceil(r->maxDelayTime * rate) + 2;
        if (!r->line || length != r->lineLength || channels != r->channels || rate != r->sampleRate) {
            free(r->line);
            r->line = calloc((size_t)channels * (size_t)length, sizeof(float));
            r->lineLength = length;
            r->write = 0;
        }
    }
    r->channels = channels;
    r->sampleRate = rate;
    r->pullList->mNumberBuffers = channels;
    for (UInt32 c = 0; c < channels; c++) r->pullList->mBuffers[c].mNumberChannels = 1;
    if (!r->pulled || !r->rendered || !r->values || (r->kind == NSCParamEffectKindDelay && !r->line)) return NO;
    return YES;
}

- (AUInternalRenderBlock)internalRenderBlock {
    EffectRender *r = _render;
    return ^AUAudioUnitStatus(AudioUnitRenderActionFlags *flags, const AudioTimeStamp *ts, AUAudioFrameCount frames, NSInteger bus, AudioBufferList *out, const AURenderEvent *events, AURenderPullInputBlock pull) {
        if (!r->pulled || frames > r->maxFrames) return kAudioUnitErr_TooManyFramesToProcess;
        AudioBufferList *abl = r->pullList;
        for (UInt32 c = 0; c < r->channels; c++) {
            abl->mBuffers[c].mData = r->pulled + c * r->maxFrames;
            abl->mBuffers[c].mDataByteSize = frames * sizeof(float);
        }
        AudioUnitRenderActionFlags pullFlags = 0;
        OSStatus status = pull ? pull(&pullFlags, ts, frames, 0, abl) : kAudioUnitErr_NoConnection;
        BOOL silent = status != noErr || (pullFlags & kAudioUnitRenderAction_OutputIsSilence) != 0;
        for (UInt32 c = 0; c < r->channels; c++) {
            float *mine = r->pulled + c * r->maxFrames;
            if (silent || !abl->mBuffers[c].mData) memset(mine, 0, frames * sizeof(float));
            else if (abl->mBuffers[c].mData != mine) memcpy(mine, abl->mBuffers[c].mData, frames * sizeof(float));
        }

        if (r->kind == KindSink) {
            WriteParamInput(r, ts, frames, silent);
            for (UInt32 c = 0; c < out->mNumberBuffers; c++) {
                if (!out->mBuffers[c].mData) out->mBuffers[c].mData = r->rendered + MIN(c, r->channels - 1) * r->maxFrames;
                memset(out->mBuffers[c].mData, 0, frames * sizeof(float));
                out->mBuffers[c].mDataByteSize = frames * sizeof(float);
            }
            *flags |= kAudioUnitRenderAction_OutputIsSilence;
            return noErr;
        }

        double frame = ts && (ts->mFlags & kAudioTimeStampSampleTimeValid)
            ? NSCContextFrame(ts->mSampleTime, r->timeline)
            : r->nextFrame;
        r->nextFrame = frame + frames;
        NSCAudioParam *param = (__bridge NSCAudioParam *)r->param;
        if (param) [param fillValuesFromFrame:frame sampleRate:r->sampleRate frameCount:frames into:r->values];
        else for (AUAudioFrameCount k = 0; k < frames; k++) r->values[k] = r->kind == NSCParamEffectKindGain ? 1.0 : 0.0;
        AddParamInput(r, ts, frames);

        if (r->kind == NSCParamEffectKindDelay && r->line) ProcessDelay(r, frames);
        else ProcessGain(r, frames);

        for (UInt32 c = 0; c < out->mNumberBuffers; c++) {
            float *from = r->rendered + MIN(c, r->channels - 1) * r->maxFrames;
            if (out->mBuffers[c].mData) memcpy(out->mBuffers[c].mData, from, frames * sizeof(float));
            else out->mBuffers[c].mData = from;
            out->mBuffers[c].mDataByteSize = frames * sizeof(float);
        }
        return noErr;
    };
}

- (void)dealloc {
    EffectRender *r = _render;
    if (!r) return;
    free(r->pulled);
    free(r->rendered);
    free(r->values);
    free(r->line);
    free(r->pullList);
    if (r->param) CFRelease(r->param);
    if (r->timelineCell) CFRelease(r->timelineCell);
    if (r->paramInputData) CFRelease(r->paramInputData);
    free(r);
}

@end

static AudioComponentDescription EffectDescription(void) {
    AudioComponentDescription description = {0};
    description.componentType = kAudioUnitType_Effect;
    description.componentSubType = 'nscp';
    description.componentManufacturer = 'NSCa';
    return description;
}

static AVAudioFormat *EffectFormat(NSCAudioContext *context) {
    double rate = context.format.sampleRate > 0 ? context.format.sampleRate : 44100.0;
    AVAudioChannelCount channels = context.format.channelCount > 0 ? context.format.channelCount : 2;
    if (channels <= 2) return [[AVAudioFormat alloc] initStandardFormatWithSampleRate:rate channels:channels];
    AVAudioChannelLayout *layout = [AVAudioChannelLayout layoutWithLayoutTag:kAudioChannelLayoutTag_DiscreteInOrder | (AudioChannelLayoutTag)channels];
    return [[AVAudioFormat alloc] initStandardFormatWithSampleRate:rate channelLayout:layout];
}

#pragma mark - The node

@implementation NSCParamEffectNode {
    AVAudioUnitEffect *_unit;
    AVAudioMixerNode *_mixer;
    // Connections into this node go to this wrapper of its mixer.
    NSCAudioNode *_input;
    NSCAudioNode *_paramInput;
    AVAudioUnitEffect *_paramSink;
}

static AVAudioUnitEffect *MakeUnit(NSCAudioContext *context, AVAudioFormat *format, NSCParamEffectKind kind, NSCAudioParam *param, double maxDelayTime) {
    static dispatch_once_t once;
    dispatch_once(&once, ^{
        [AUAudioUnit registerSubclass:[NSCParamEffectUnit class] asComponentDescription:EffectDescription() name:@"NativeScript: param effect" version:1];
    });
    pendingFormat = format;
    // A registered in-process unit instantiates synchronously.
    AVAudioUnitEffect *unit = [[AVAudioUnitEffect alloc] initWithAudioComponentDescription:EffectDescription()];
    pendingFormat = nil;
    NSCParamEffectUnit *au = (NSCParamEffectUnit *)unit.AUAudioUnit;
    if (![au isKindOfClass:[NSCParamEffectUnit class]]) {
        NSCLogError(@"NSCParamEffectNode: could not instantiate the effect audio unit");
        return unit;
    }
    EffectRender *r = au.render;
    r->kind = kind;
    r->param = param ? CFBridgingRetain(param) : NULL;
    r->maxDelayTime = maxDelayTime;
    r->sampleRate = format.sampleRate;
    NSMutableData *cell = context.timelineCell;
    if (cell) {
        r->timelineCell = CFBridgingRetain(cell);
        r->timeline = (const double *)cell.bytes;
    }
    return unit;
}

static void ShareParamInput(AVAudioUnitEffect *unit, NSMutableData *data) {
    NSCParamEffectUnit *au = (NSCParamEffectUnit *)unit.AUAudioUnit;
    if (![au isKindOfClass:[NSCParamEffectUnit class]]) return;
    au.render->paramInputData = CFBridgingRetain(data);
    atomic_store_explicit(&au.render->paramInput, (ParamInput *)data.mutableBytes, memory_order_release);
}

- (instancetype)initWithContext:(NSCAudioContext *)context
                           kind:(NSCParamEffectKind)kind
                          param:(NSCAudioParam *)param
                   maxDelayTime:(double)maxDelayTime {
    // Ports at the context's channel count avoid AVAudioMixerNode's equal-power pan law.
    AVAudioFormat *format = EffectFormat(context);
    AVAudioUnitEffect *unit = MakeUnit(context, format, kind, param, maxDelayTime);

    if (!(self = [super initWithContext:context node:unit])) return nil;
    _unit = unit;
    param.owner = self;
    AVAudioEngine *engine = context.engine;
    if ([context isKindOfClass:[NSCOfflineAudioContext class]]) [(NSCOfflineAudioContext *)context willConnect];
    AVAudioMixerNode *mixer = [[AVAudioMixerNode alloc] init];
    [engine attachNode:unit];
    [engine attachNode:mixer];
    @try {
        [engine connect:mixer to:unit format:format];
    } @catch (NSException *e) {
        NSCLogError(@"NSCParamEffectNode: the input mixer did not connect: %@", e);
    }
    _mixer = mixer;
    _input = [[NSCAudioNode alloc] initWithContext:context node:mixer];
    return self;
}

// Audio connected to the param sums in a mixer feeding a sink unit, which feeds silence into this
// node's input mixer, so it renders first in each cycle and leaves its downmix for this unit.
- (BOOL)hasParamInput { return _paramInput != nil; }

- (NSCAudioNode *)paramInput {
    if (_paramInput) return _paramInput;
    NSCAudioContext *context = self.context;
    AVAudioEngine *engine = context.engine;
    if (!engine || ![_unit.AUAudioUnit isKindOfClass:[NSCParamEffectUnit class]]) return nil;
    AVAudioFormat *format = EffectFormat(context);
    AVAudioUnitEffect *sink = MakeUnit(context, format, KindSink, nil, 0.0);
    if (![sink.AUAudioUnit isKindOfClass:[NSCParamEffectUnit class]]) return nil;
    NSMutableData *shared = [NSMutableData dataWithLength:sizeof(ParamInput)];
    atomic_init(&((ParamInput *)shared.mutableBytes)->sampleTime, -1.0);
    ShareParamInput(sink, shared);
    AVAudioMixerNode *mixer = [[AVAudioMixerNode alloc] init];
    if ([context isKindOfClass:[NSCOfflineAudioContext class]]) [(NSCOfflineAudioContext *)context willConnect];
    [engine attachNode:mixer];
    [engine attachNode:sink];
    @try {
        [engine connect:mixer to:sink format:format];
        [engine connect:sink to:_mixer fromBus:0 toBus:_mixer.nextAvailableInputBus format:format];
    } @catch (NSException *e) {
        NSCLogError(@"NSCParamEffectNode: the param input did not connect: %@", e);
        [engine detachNode:sink];
        [engine detachNode:mixer];
        return nil;
    }
    ShareParamInput(_unit, shared);
    _paramSink = sink;
    _paramInput = [[NSCAudioNode alloc] initWithContext:context node:mixer];
    return _paramInput;
}

// Each connection takes the mixer's next free bus, so inputs sum.
- (void)handleConnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    [_input handleConnectFrom:source output:output input:nil];
}

- (void)handleDisconnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    [_input handleDisconnectFrom:source output:output input:nil];
}

@end
