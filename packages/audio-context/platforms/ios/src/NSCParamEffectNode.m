#import "NSCParamEffectNode.h"
#import <AudioToolbox/AudioToolbox.h>
#import <math.h>
#import <stdatomic.h>
#import "NSCAudioContext.h"
#import "NSCAudioLog.h"

#define MAX_CHANNELS 32
#define MAX_FRAMES 4096
#define MAX_PARAMS 4

// Renders a param's inputs into its input block, outputting silence; not a public kind.
static const NSCParamEffectKind KindSink = (NSCParamEffectKind)99;

typedef struct {
    NSCParamEffectKind kind;
    CFTypeRef params[MAX_PARAMS];
    NSInteger paramCount;
    double maxDelayTime;
    double sampleRate;
    UInt32 maxFrames;
    UInt32 channels;
    float *pulled;
    float *rendered;
    double *values[MAX_PARAMS];
    BOOL automated[MAX_PARAMS];
    AudioBufferList *pullList;
    // Delay line: `lineLength` frames per channel, all channels written at `write`.
    float *line;
    int64_t lineLength;
    int64_t write;
    // Biquad: x1, x2, y1, y2 per channel.
    double *state;
    _Atomic NSInteger filterType;
    // The context's timeline cell (retained).
    CFTypeRef timelineCell;
    const double *timeline;
    double nextFrame;
    // Sink: the param input block it writes (retained NSMutableData).
    CFTypeRef inputData;
    NSCParamInputBlock *input;
} EffectRender;

static void ProcessGain(EffectRender *r, AUAudioFrameCount frames) {
    const double *values = r->values[0];
    for (UInt32 c = 0; c < r->channels; c++) {
        const float *in = r->pulled + c * r->maxFrames;
        float *out = r->rendered + c * r->maxFrames;
        for (AUAudioFrameCount k = 0; k < frames; k++) out[k] = in[k] * (float)values[k];
    }
}

// Linear interpolation between the two frames around delayTime; a whole number of frames is exact.
static void ProcessDelay(EffectRender *r, AUAudioFrameCount frames) {
    int64_t length = r->lineLength;
    double maxFrames = (double)(length - 2);
    double *values = r->values[0];
    for (AUAudioFrameCount k = 0; k < frames; k++) {
        double d = values[k] * r->sampleRate;
        d = isfinite(d) ? MAX(0.0, MIN(d, maxFrames)) : 0.0;
        double nearest = nearbyint(d);
        if (fabs(d - nearest) < 1e-6) d = nearest;
        values[k] = d;
    }
    for (UInt32 c = 0; c < r->channels; c++) {
        const float *in = r->pulled + c * r->maxFrames;
        float *out = r->rendered + c * r->maxFrames;
        float *line = r->line + c * length;
        int64_t w = r->write;
        for (AUAudioFrameCount k = 0; k < frames; k++) {
            line[w] = in[k];
            double d = values[k];
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

enum { LOWPASS, HIGHPASS, BANDPASS, LOWSHELF, HIGHSHELF, PEAKING, NOTCH, ALLPASS };

static void SetCoefficients(double *k, double b0, double b1, double b2, double a0, double a1, double a2) {
    double scale = 1.0 / a0;
    k[0] = b0 * scale; k[1] = b1 * scale; k[2] = b2 * scale; k[3] = a1 * scale; k[4] = a2 * scale;
}

// Chromium's biquad coefficients: the Audio EQ Cookbook, with its limits at 0 and Nyquist.
static void BiquadCoefficients(NSInteger type, double frequency, double Q, double gain, double detune, double rate, double *k) {
    double nyquist = rate * 0.5;
    double f = frequency * pow(2.0, detune / 1200.0) / nyquist;
    f = isfinite(f) ? MAX(0.0, MIN(f, 1.0)) : 0.0;
    double A = pow(10.0, gain / 40.0);
    double w0 = M_PI * f, cw = cos(w0), sw = sin(w0);
    BOOL inside = f > 0.0 && f < 1.0;
    switch (type) {
        case LOWPASS:
            if (f == 1.0) SetCoefficients(k, 1, 0, 0, 1, 0, 0);
            else if (f > 0.0) {
                double alpha = 0.5 * sw * pow(10.0, -0.05 * Q);
                SetCoefficients(k, 0.5 * (1 - cw), 1 - cw, 0.5 * (1 - cw), 1 + alpha, -2 * cw, 1 - alpha);
            } else SetCoefficients(k, 0, 0, 0, 1, 0, 0);
            break;
        case HIGHPASS:
            if (f == 1.0) SetCoefficients(k, 0, 0, 0, 1, 0, 0);
            else if (f > 0.0) {
                double alpha = 0.5 * sw * pow(10.0, -0.05 * Q);
                SetCoefficients(k, 0.5 * (1 + cw), -(1 + cw), 0.5 * (1 + cw), 1 + alpha, -2 * cw, 1 - alpha);
            } else SetCoefficients(k, 1, 0, 0, 1, 0, 0);
            break;
        case BANDPASS:
            Q = MAX(0.0, Q);
            if (inside && Q > 0.0) {
                double alpha = sw / (2 * Q);
                SetCoefficients(k, alpha, 0, -alpha, 1 + alpha, -2 * cw, 1 - alpha);
            } else SetCoefficients(k, inside ? 1 : 0, 0, 0, 1, 0, 0);
            break;
        case LOWSHELF:
            if (f == 1.0) SetCoefficients(k, A * A, 0, 0, 1, 0, 0);
            else if (f > 0.0) {
                double alpha = 0.5 * sw * M_SQRT2, s = 2 * sqrt(A) * alpha;
                SetCoefficients(k, A * ((A + 1) - (A - 1) * cw + s), 2 * A * ((A - 1) - (A + 1) * cw), A * ((A + 1) - (A - 1) * cw - s),
                                (A + 1) + (A - 1) * cw + s, -2 * ((A - 1) + (A + 1) * cw), (A + 1) + (A - 1) * cw - s);
            } else SetCoefficients(k, 1, 0, 0, 1, 0, 0);
            break;
        case HIGHSHELF:
            if (f == 1.0) SetCoefficients(k, 1, 0, 0, 1, 0, 0);
            else if (f > 0.0) {
                double alpha = 0.5 * sw * M_SQRT2, s = 2 * sqrt(A) * alpha;
                SetCoefficients(k, A * ((A + 1) + (A - 1) * cw + s), -2 * A * ((A - 1) + (A + 1) * cw), A * ((A + 1) + (A - 1) * cw - s),
                                (A + 1) - (A - 1) * cw + s, 2 * ((A - 1) - (A + 1) * cw), (A + 1) - (A - 1) * cw - s);
            } else SetCoefficients(k, A * A, 0, 0, 1, 0, 0);
            break;
        case PEAKING:
            Q = MAX(0.0, Q);
            if (inside && Q > 0.0) {
                double alpha = sw / (2 * Q);
                SetCoefficients(k, 1 + alpha * A, -2 * cw, 1 - alpha * A, 1 + alpha / A, -2 * cw, 1 - alpha / A);
            } else SetCoefficients(k, inside ? A * A : 1, 0, 0, 1, 0, 0);
            break;
        case NOTCH:
            Q = MAX(0.0, Q);
            if (inside && Q > 0.0) {
                double alpha = sw / (2 * Q);
                SetCoefficients(k, 1, -2 * cw, 1, 1 + alpha, -2 * cw, 1 - alpha);
            } else SetCoefficients(k, inside ? 0 : 1, 0, 0, 1, 0, 0);
            break;
        default:
            Q = MAX(0.0, Q);
            if (inside && Q > 0.0) {
                double alpha = sw / (2 * Q);
                SetCoefficients(k, 1 - alpha, -2 * cw, 1 + alpha, 1 + alpha, -2 * cw, 1 - alpha);
            } else SetCoefficients(k, inside ? -1 : 1, 0, 0, 1, 0, 0);
            break;
    }
}

// Coefficients per frame while a param changes, else once per call.
static void ProcessBiquad(EffectRender *r, AUAudioFrameCount frames) {
    NSInteger type = atomic_load_explicit(&r->filterType, memory_order_relaxed);
    BOOL perFrame = r->automated[0] || r->automated[1] || r->automated[2] || r->automated[3];
    double k[5];
    for (AUAudioFrameCount n = 0; n < frames; n++) {
        if (n == 0 || perFrame) BiquadCoefficients(type, r->values[0][n], r->values[1][n], r->values[2][n], r->values[3][n], r->sampleRate, k);
        for (UInt32 c = 0; c < r->channels; c++) {
            double *st = r->state + c * 4;
            double x = r->pulled[c * r->maxFrames + n];
            double y = k[0] * x + k[1] * st[0] + k[2] * st[1] - k[3] * st[2] - k[4] * st[3];
            if (!isfinite(y)) y = 0.0;
            st[1] = st[0]; st[0] = x; st[3] = st[2]; st[2] = y;
            r->rendered[c * r->maxFrames + n] = (float)y;
        }
    }
}

static void WriteParamInput(EffectRender *r, const AudioTimeStamp *ts, AUAudioFrameCount frames, BOOL silent) {
    NSCParamInputBlock *in = r->input;
    if (!in || !ts || !(ts->mFlags & kAudioTimeStampSampleTimeValid)) return;
    frames = MIN(frames, (AUAudioFrameCount)NSC_PARAM_INPUT_FRAMES);
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
    atomic_store_explicit(&in->frames, frames, memory_order_relaxed);
    atomic_store_explicit(&in->frame, NSCContextFrame(ts->mSampleTime, r->timeline), memory_order_release);
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
        r->pulled = calloc((size_t)channels * frames, sizeof(float));
        r->rendered = calloc((size_t)channels * frames, sizeof(float));
        for (NSInteger p = 0; p < MAX_PARAMS; p++) {
            free(r->values[p]);
            r->values[p] = calloc(frames, sizeof(double));
        }
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
    if (r->kind == NSCParamEffectKindBiquad && (!r->state || channels != r->channels)) {
        free(r->state);
        r->state = calloc((size_t)channels * 4, sizeof(double));
    }
    r->channels = channels;
    r->sampleRate = rate;
    r->pullList->mNumberBuffers = channels;
    for (UInt32 c = 0; c < channels; c++) r->pullList->mBuffers[c].mNumberChannels = 1;
    if (!r->pulled || !r->rendered || !r->values[MAX_PARAMS - 1]) return NO;
    if ((r->kind == NSCParamEffectKindDelay && !r->line) || (r->kind == NSCParamEffectKindBiquad && !r->state)) return NO;
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
        for (NSInteger p = 0; p < r->paramCount; p++) {
            NSCAudioParam *param = (__bridge NSCAudioParam *)r->params[p];
            r->automated[p] = [param fillValuesFromFrame:frame sampleRate:r->sampleRate frameCount:frames into:r->values[p]];
        }

        if (r->kind == NSCParamEffectKindDelay && r->line) ProcessDelay(r, frames);
        else if (r->kind == NSCParamEffectKindBiquad && r->state && r->paramCount == 4) ProcessBiquad(r, frames);
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
    for (NSInteger p = 0; p < MAX_PARAMS; p++) free(r->values[p]);
    free(r->line);
    free(r->state);
    free(r->pullList);
    for (NSInteger p = 0; p < r->paramCount; p++) CFRelease(r->params[p]);
    if (r->timelineCell) CFRelease(r->timelineCell);
    if (r->inputData) CFRelease(r->inputData);
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

static AVAudioUnitEffect *MakeUnit(NSCAudioContext *context, AVAudioFormat *format, NSCParamEffectKind kind, NSArray<NSCAudioParam *> *params, double maxDelayTime) {
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
    r->paramCount = MIN((NSInteger)params.count, (NSInteger)MAX_PARAMS);
    for (NSInteger p = 0; p < r->paramCount; p++) r->params[p] = CFBridgingRetain(params[p]);
    r->maxDelayTime = maxDelayTime;
    r->sampleRate = format.sampleRate;
    NSMutableData *cell = context.timelineCell;
    if (cell) {
        r->timelineCell = CFBridgingRetain(cell);
        r->timeline = (const double *)cell.bytes;
    }
    return unit;
}

NSCAudioNode *NSCMakeParamInput(NSCAudioContext *context, NSCAudioParam *param) {
    if (param.inputNode) return param.inputNode;
    AVAudioEngine *engine = context.engine;
    if (!engine) return nil;
    AVAudioFormat *format = EffectFormat(context);
    AVAudioUnitEffect *sink = MakeUnit(context, format, KindSink, @[], 0.0);
    NSCParamEffectUnit *au = (NSCParamEffectUnit *)sink.AUAudioUnit;
    if (![au isKindOfClass:[NSCParamEffectUnit class]]) return nil;
    NSMutableData *data = [param inputData];
    au.render->inputData = CFBridgingRetain(data);
    au.render->input = (NSCParamInputBlock *)data.mutableBytes;
    AVAudioMixerNode *mixer = [[AVAudioMixerNode alloc] init];
    if ([context isKindOfClass:[NSCOfflineAudioContext class]]) [(NSCOfflineAudioContext *)context willConnect];
    [engine attachNode:mixer];
    [engine attachNode:sink];
    @try {
        [engine connect:mixer to:sink format:format];
    } @catch (NSException *e) {
        NSCLogError(@"NSCParamEffectNode: the param input did not connect: %@", e);
        [engine detachNode:sink];
        [engine detachNode:mixer];
        return nil;
    }
    param.inputSink = sink;
    param.inputNode = [[NSCAudioNode alloc] initWithContext:context node:mixer];
    return param.inputNode;
}

#pragma mark - The node

@implementation NSCParamEffectNode {
    AVAudioUnitEffect *_unit;
    AVAudioMixerNode *_mixer;
    // Connections into this node go to this wrapper of its mixer.
    NSCAudioNode *_input;
}

- (instancetype)initWithContext:(NSCAudioContext *)context
                           kind:(NSCParamEffectKind)kind
                          param:(NSCAudioParam *)param
                   maxDelayTime:(double)maxDelayTime {
    return [self initWithContext:context kind:kind params:@[ param ] maxDelayTime:maxDelayTime];
}

- (instancetype)initWithContext:(NSCAudioContext *)context
                           kind:(NSCParamEffectKind)kind
                         params:(NSArray<NSCAudioParam *> *)params
                   maxDelayTime:(double)maxDelayTime {
    // Ports at the context's channel count avoid AVAudioMixerNode's equal-power pan law.
    AVAudioFormat *format = EffectFormat(context);
    AVAudioUnitEffect *unit = MakeUnit(context, format, kind, params, maxDelayTime);

    if (!(self = [super initWithContext:context node:unit])) return nil;
    _unit = unit;
    for (NSCAudioParam *param in params) param.owner = self;
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

- (void)setFilterType:(NSInteger)type {
    NSCParamEffectUnit *au = (NSCParamEffectUnit *)_unit.AUAudioUnit;
    if ([au isKindOfClass:[NSCParamEffectUnit class]]) atomic_store_explicit(&au.render->filterType, type, memory_order_relaxed);
}

// The sink feeds silence into this node's input mixer, so it renders before this unit reads the param.
- (void)attachParamSink:(AVAudioNode *)sink {
    NSCAudioContext *context = self.context;
    AVAudioEngine *engine = context.engine;
    if ([context isKindOfClass:[NSCOfflineAudioContext class]]) [(NSCOfflineAudioContext *)context willConnect];
    @try {
        [engine connect:sink to:_mixer fromBus:0 toBus:_mixer.nextAvailableInputBus format:EffectFormat(context)];
    } @catch (NSException *e) {
        NSCLogError(@"NSCParamEffectNode: the param input did not connect: %@", e);
    }
}

// Each connection takes the mixer's next free bus, so inputs sum.
- (void)handleConnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    [_input handleConnectFrom:source output:output input:nil];
}

- (void)handleDisconnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    [_input handleDisconnectFrom:source output:output input:nil];
}

@end
