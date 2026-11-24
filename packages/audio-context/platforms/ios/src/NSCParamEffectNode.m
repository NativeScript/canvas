#import "NSCParamEffectNode.h"
#import <AudioToolbox/AudioToolbox.h>
#import <math.h>
#import "NSCAudioContext.h"
#import "NSCAudioLog.h"

#define MAX_CHANNELS 32

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
    // Rate of the timestamps' sample time: the output device's in a realtime context.
    double timelineRate;
    double nextFrame;
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
    self.maximumFramesToRender = 4096;
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

        double timelineRate = r->timelineRate > 0.0 ? r->timelineRate : r->sampleRate;
        double frame = ts && (ts->mFlags & kAudioTimeStampSampleTimeValid)
            ? (timelineRate == r->sampleRate ? ts->mSampleTime : ts->mSampleTime * (r->sampleRate / timelineRate))
            : r->nextFrame;
        r->nextFrame = frame + frames;
        NSCAudioParam *param = (__bridge NSCAudioParam *)r->param;
        if (param) [param fillValuesFromFrame:frame sampleRate:r->sampleRate frameCount:frames into:r->values];
        else for (AUAudioFrameCount k = 0; k < frames; k++) r->values[k] = r->kind == NSCParamEffectKindGain ? 1.0 : 0.0;

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
    id _configurationObserver;
}

// currentTime counts the output node's samples, which run at the device's rate.
static void UpdateTimelineRate(AVAudioUnitEffect *unit, AVAudioEngine *engine) {
    NSCParamEffectUnit *au = (NSCParamEffectUnit *)unit.AUAudioUnit;
    if (![au isKindOfClass:[NSCParamEffectUnit class]] || !engine) return;
    double rate = engine.isInManualRenderingMode ? engine.manualRenderingFormat.sampleRate : [engine.outputNode outputFormatForBus:0].sampleRate;
    if (rate > 0.0) au.render->timelineRate = rate;
}

- (instancetype)initWithContext:(NSCAudioContext *)context
                           kind:(NSCParamEffectKind)kind
                          param:(NSCAudioParam *)param
                   maxDelayTime:(double)maxDelayTime {
    static dispatch_once_t once;
    dispatch_once(&once, ^{
        [AUAudioUnit registerSubclass:[NSCParamEffectUnit class] asComponentDescription:EffectDescription() name:@"NativeScript: param effect" version:1];
    });
    // Ports at the context's channel count avoid AVAudioMixerNode's equal-power pan law.
    AVAudioFormat *format = EffectFormat(context);
    pendingFormat = format;
    // A registered in-process unit instantiates synchronously.
    AVAudioUnitEffect *unit = [[AVAudioUnitEffect alloc] initWithAudioComponentDescription:EffectDescription()];
    pendingFormat = nil;
    NSCParamEffectUnit *au = (NSCParamEffectUnit *)unit.AUAudioUnit;
    if ([au isKindOfClass:[NSCParamEffectUnit class]]) {
        EffectRender *r = au.render;
        r->kind = kind;
        r->param = CFBridgingRetain(param);
        r->maxDelayTime = maxDelayTime;
        r->sampleRate = format.sampleRate;
    } else {
        NSCLogError(@"NSCParamEffectNode: could not instantiate the effect audio unit");
    }

    if (!(self = [super initWithContext:context node:unit])) return nil;
    _unit = unit;
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
    UpdateTimelineRate(unit, engine);
    __weak AVAudioUnitEffect *weakUnit = unit;
    __weak AVAudioEngine *weakEngine = engine;
    _configurationObserver = [[NSNotificationCenter defaultCenter] addObserverForName:AVAudioEngineConfigurationChangeNotification object:engine queue:nil usingBlock:^(NSNotification *note) {
        UpdateTimelineRate(weakUnit, weakEngine);
    }];
    return self;
}

- (void)dealloc {
    if (_configurationObserver) [[NSNotificationCenter defaultCenter] removeObserver:_configurationObserver];
}

// Each connection takes the mixer's next free bus, so inputs sum.
- (void)handleConnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    [_input handleConnectFrom:source output:output input:nil];
}

- (void)handleDisconnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    [_input handleDisconnectFrom:source output:output input:nil];
}

@end
