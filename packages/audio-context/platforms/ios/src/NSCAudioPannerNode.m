#import <objc/message.h>
#import <QuartzCore/QuartzCore.h>
#import "NSCAudioLog.h"
#import "NSCAudioContext.h"
#import "NSCChannelMixerNode.h"
#import <stdatomic.h>

//#ifdef NSCLogDebug
//#undef NSCLogDebug
//#endif
//#define NSCLogDebug(...)

static inline double NSCClampDouble(double value, double minValue, double maxValue) {
    if (value < minValue) return minValue;
    if (value > maxValue) return maxValue;
    return value;
}

static inline void NSCNormalizeVector3(double *x, double *y, double *z,
                                       double fallbackX, double fallbackY, double fallbackZ) {
    double len = sqrt((*x) * (*x) + (*y) * (*y) + (*z) * (*z));
    if (len <= 1e-12) {
        *x = fallbackX;
        *y = fallbackY;
        *z = fallbackZ;
        return;
    }
    *x /= len;
    *y /= len;
    *z /= len;
}

static inline double NSCWrapPannerAzimuthDegrees(double azimuth) {
    azimuth = NSCClampDouble(azimuth, -180.0, 180.0);
    if (azimuth < -90.0) azimuth = -180.0 - azimuth;
    else if (azimuth > 90.0) azimuth = 180.0 - azimuth;
    return azimuth;
}

#pragma mark - The pan unit

typedef struct {
    _Atomic float gain;
    // Azimuth in degrees, -90 to 90.
    _Atomic float azimuth;
    const UInt32 *inputChannels;
    float *pulled;
    // A mono output's right channel before the down-mix.
    float *right;
    UInt32 maxFrames;
    AudioBufferList *pullList;
} PanRender;

static AVAudioFormat *pendingPanFormat;

// Web Audio's equal-power panning of a mono or stereo input, times the distance and cone gain.
@interface NSCPanUnit : AUAudioUnit
@property (nonatomic, readonly) PanRender *render;
@end

@implementation NSCPanUnit {
    AUAudioUnitBusArray *_inputBusses;
    AUAudioUnitBusArray *_outputBusses;
    PanRender *_render;
}

- (instancetype)initWithComponentDescription:(AudioComponentDescription)description options:(AudioComponentInstantiationOptions)options error:(NSError **)outError {
    if (!(self = [super initWithComponentDescription:description options:options error:outError])) return nil;
    AUAudioUnitBus *input = [[AUAudioUnitBus alloc] initWithFormat:pendingPanFormat error:outError];
    AUAudioUnitBus *output = [[AUAudioUnitBus alloc] initWithFormat:pendingPanFormat error:outError];
    if (!input || !output) return nil;
    _inputBusses = [[AUAudioUnitBusArray alloc] initWithAudioUnit:self busType:AUAudioUnitBusTypeInput busses:@[ input ]];
    _outputBusses = [[AUAudioUnitBusArray alloc] initWithAudioUnit:self busType:AUAudioUnitBusTypeOutput busses:@[ output ]];
    _render = calloc(1, sizeof(PanRender));
    _render->pullList = calloc(1, sizeof(AudioBufferList) + sizeof(AudioBuffer));
    atomic_init(&_render->gain, 1.0f);
    self.maximumFramesToRender = 4096;
    return self;
}

- (AUAudioUnitBusArray *)inputBusses { return _inputBusses; }
- (AUAudioUnitBusArray *)outputBusses { return _outputBusses; }
- (BOOL)canProcessInPlace { return NO; }

// Two channels in; one or two out, a mono output taking the down-mix.
- (BOOL)shouldChangeToFormat:(AVAudioFormat *)format forBus:(AUAudioUnitBus *)bus {
    if (bus.busType == AUAudioUnitBusTypeInput) return format.channelCount == 2;
    return format.channelCount == 1 || format.channelCount == 2;
}

- (BOOL)allocateRenderResourcesAndReturnError:(NSError **)outError {
    if (![super allocateRenderResourcesAndReturnError:outError]) return NO;
    PanRender *r = _render;
    UInt32 frames = MAX((UInt32)self.maximumFramesToRender, 128u);
    if (frames > r->maxFrames) {
        free(r->pulled);
        free(r->right);
        r->pulled = calloc((size_t)2 * frames, sizeof(float));
        r->right = calloc(frames, sizeof(float));
        r->maxFrames = frames;
    }
    return r->pulled && r->right;
}

- (AUInternalRenderBlock)internalRenderBlock {
    PanRender *r = _render;
    return ^AUAudioUnitStatus(AudioUnitRenderActionFlags *flags, const AudioTimeStamp *ts, AUAudioFrameCount frames, NSInteger bus, AudioBufferList *out, const AURenderEvent *events, AURenderPullInputBlock pull) {
        if (!r->pulled || frames > r->maxFrames || out->mNumberBuffers < 1) return kAudioUnitErr_TooManyFramesToProcess;
        AudioBufferList *abl = r->pullList;
        abl->mNumberBuffers = 2;
        for (UInt32 c = 0; c < 2; c++) {
            abl->mBuffers[c].mNumberChannels = 1;
            abl->mBuffers[c].mData = r->pulled + c * r->maxFrames;
            abl->mBuffers[c].mDataByteSize = frames * sizeof(float);
        }
        AudioUnitRenderActionFlags pullFlags = 0;
        OSStatus status = pull ? pull(&pullFlags, ts, frames, 0, abl) : kAudioUnitErr_NoConnection;
        BOOL monoOut = out->mNumberBuffers == 1;
        float *outL = (float *)out->mBuffers[0].mData, *outR = monoOut ? r->right : (float *)out->mBuffers[1].mData;
        if (!outL) outL = out->mBuffers[0].mData = r->pulled;
        if (!outR) outR = out->mBuffers[1].mData = r->pulled + r->maxFrames;
        for (UInt32 c = 0; c < out->mNumberBuffers; c++) out->mBuffers[c].mDataByteSize = frames * sizeof(float);
        if (status != noErr || (pullFlags & kAudioUnitRenderAction_OutputIsSilence)) {
            memset(outL, 0, frames * sizeof(float));
            memset(outR, 0, frames * sizeof(float));
            *flags |= kAudioUnitRenderAction_OutputIsSilence;
            return noErr;
        }
        const float *inL = abl->mBuffers[0].mData ?: r->pulled, *inR = abl->mBuffers[1].mData ?: r->pulled + r->maxFrames;
        float g = atomic_load_explicit(&r->gain, memory_order_relaxed);
        double az = atomic_load_explicit(&r->azimuth, memory_order_relaxed);
        // Sources here play at the context's channel count, so identical channels are taken as mono.
        BOOL mono = r->inputChannels && *r->inputChannels == 1;
        if (!mono) {
            mono = YES;
            for (AUAudioFrameCount k = 0; k < frames && mono; k++) mono = inL[k] == inR[k];
        }
        if (mono) {
            double x = (az + 90.0) / 180.0;
            float gl = (float)cos(x * M_PI_2) * g, gr = (float)sin(x * M_PI_2) * g;
            for (AUAudioFrameCount k = 0; k < frames; k++) {
                float m = inL[k];
                outL[k] = m * gl;
                outR[k] = m * gr;
            }
        } else if (az <= 0.0) {
            double x = (az + 90.0) / 90.0;
            float gl = (float)cos(x * M_PI_2), gr = (float)sin(x * M_PI_2);
            for (AUAudioFrameCount k = 0; k < frames; k++) {
                float l = inL[k], rr = inR[k];
                outL[k] = (l + rr * gl) * g;
                outR[k] = rr * gr * g;
            }
        } else {
            double x = az / 90.0;
            float gl = (float)cos(x * M_PI_2), gr = (float)sin(x * M_PI_2);
            for (AUAudioFrameCount k = 0; k < frames; k++) {
                float l = inL[k], rr = inR[k];
                outL[k] = l * gl * g;
                outR[k] = (rr + l * gr) * g;
            }
        }
        if (monoOut) for (AUAudioFrameCount k = 0; k < frames; k++) outL[k] = 0.5f * (outL[k] + outR[k]);
        return noErr;
    };
}

- (void)dealloc {
    free(_render->pulled);
    free(_render->right);
    free(_render->pullList);
    free(_render);
}

@end

static AudioComponentDescription PanDescription(void) {
    AudioComponentDescription description = {0};
    description.componentType = kAudioUnitType_Effect;
    description.componentSubType = 'nscn';
    description.componentManufacturer = 'NSCa';
    return description;
}

@implementation NSCAudioPannerNode {
    AVAudioUnitEffect *_unit;
    // Connections into the panner mix here: clamped-max, two channels.
    NSCChannelMixerNode *_input;
    float _positionX, _positionY, _positionZ;
    float _orientationX, _orientationY, _orientationZ;
    float _pan;
    int _distanceModel;
    int _panningModel;
    float _refDistance, _maxDistance, _rolloffFactor;
    float _coneInnerAngle, _coneOuterAngle, _coneOuterGain;
    dispatch_source_t _automationTimer;
    CFTimeInterval _automationAnchorHostTime;
    double _automationAnchorAudioTime;
    BOOL _automationUsesHostTime;
}

- (instancetype)initWithContext:(NSCAudioContext *)context {
    return [self initWithContext:context positionX:0.0 positionY:0.0 positionZ:0.0 panningModel:0];
}

- (instancetype)initWithContext:(NSCAudioContext *)context positionX:(double)positionX positionY:(double)positionY positionZ:(double)positionZ {
    return [self initWithContext:context positionX:positionX positionY:positionY positionZ:positionZ panningModel:0];
}

- (instancetype)initWithContext:(NSCAudioContext *)context
                      positionX:(double)positionX
                      positionY:(double)positionY
                      positionZ:(double)positionZ
                   panningModel:(int)panningModel {
    static dispatch_once_t once;
    dispatch_once(&once, ^{
        [AUAudioUnit registerSubclass:[NSCPanUnit class] asComponentDescription:PanDescription() name:@"NativeScript: panner" version:1];
    });
    double sr = context.sampleRate > 0 ? context.sampleRate : 44100.0;
    AVAudioFormat *stereo = [[AVAudioFormat alloc] initStandardFormatWithSampleRate:sr channels:2];
    pendingPanFormat = stereo;
    // A registered in-process unit instantiates synchronously.
    AVAudioUnitEffect *unit = [[AVAudioUnitEffect alloc] initWithAudioComponentDescription:PanDescription()];
    pendingPanFormat = nil;
    if (![unit.AUAudioUnit isKindOfClass:[NSCPanUnit class]]) return nil;
    NSCChannelMixerNode *input = [[NSCChannelMixerNode alloc] initWithContext:context mode:NSCMixModeSum channels:2 discrete:NO];
    if (!input) return nil;
    [input setClamp:YES];
    if ([context isKindOfClass:[NSCOfflineAudioContext class]]) [(NSCOfflineAudioContext *)context willConnect];
    @try { [context.engine attachNode:unit]; } @catch (NSException *e) { return nil; }

    if (self = [super initWithContext:context node:unit]) {
        _unit = unit;
        _input = input;
        ((NSCPanUnit *)unit.AUAudioUnit).render->inputChannels = [input mixedChannels];
        NSCConnectAdding(context, input.avNode, 0, unit, 0, stereo);
        _positionX = (float)positionX; _positionY = (float)positionY; _positionZ = (float)positionZ;
        _pan = 0.0f;
        _positionXParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:positionX];
        _positionYParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:positionY];
        _positionZParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:positionZ];
        _orientationXParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:1.0];
        _orientationYParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:0.0];
        _orientationZParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:0.0];
        for (NSCAudioParam *param in @[ _positionXParam, _positionYParam, _positionZParam, _orientationXParam, _orientationYParam, _orientationZParam ]) param.owner = self;
        _orientationX = 1.0f; _orientationY = 0.0f; _orientationZ = 0.0f;
        _panningModel = panningModel == 1 ? 1 : 0;
        _distanceModel = 0;
        _refDistance = 1.0f;
        _maxDistance = 10000.0f;
        _rolloffFactor = 1.0f;
        _coneInnerAngle = 360.0f;
        _coneOuterAngle = 360.0f;
        _coneOuterGain = 0.0f;
        _automationAnchorHostTime = 0;
        _automationAnchorAudioTime = 0.0;
        _automationUsesHostTime = NO;

        [self installAudioParamHooks];
        [self applySpatial];
    }
    return self;
}

- (void)dealloc {
    [self stopAutomationLink];
    [self clearAudioParamHooks];
    NSCAudioContext *ctx = self.context;
    AVAudioEngine *engine = ctx.engine;
    if (engine && _unit && [ctx isNode:_unit attachedToEngine:engine]) {
        @try { [engine disconnectNodeOutput:_unit]; } @catch (NSException *e) {}
        @try { [ctx detachNode:_unit fromEngine:engine]; } @catch (NSException *e) {}
    }
}

- (instancetype)initWithContextAndParams:(NSCAudioContext *)context
                                                                                :(NSNumber *)positionX
                                                                                :(NSNumber *)positionY
                                                                                :(NSNumber *)positionZ
                                                                                :(NSNumber *)orientationX
                                                                                :(NSNumber *)orientationY
                                                                                :(NSNumber *)orientationZ
                                                                                :(NSNumber *)pan
                                                                                :(NSNumber *)distanceModel
                                                                                :(NSNumber *)panningModel
                                                                                :(NSNumber *)refDistance
                                                                                :(NSNumber *)maxDistance
                                                                                :(NSNumber *)rolloffFactor
                                                                                :(NSNumber *)coneInnerAngle
                                                                                :(NSNumber *)coneOuterAngle
                                                                                :(NSNumber *)coneOuterGain {
    double px = positionX ? positionX.doubleValue : 0.0;
    double py = positionY ? positionY.doubleValue : 0.0;
    double pz = positionZ ? positionZ.doubleValue : 0.0;
    int pm = panningModel ? panningModel.intValue : 0;
    if (self = [self initWithContext:context positionX:px positionY:py positionZ:pz panningModel:pm]) {
        if (orientationX || orientationY || orientationZ) {
            [self setOrientation:orientationX :orientationY :orientationZ];
        }
        if (pan) [self setPan:pan];
        if (distanceModel) [self setDistanceModel:distanceModel];
        if (refDistance) [self setRefDistance:refDistance];
        if (maxDistance) [self setMaxDistance:maxDistance];
        if (rolloffFactor) [self setRolloffFactor:rolloffFactor];
        if (coneInnerAngle) [self setConeInnerAngle:coneInnerAngle];
        if (coneOuterAngle) [self setConeOuterAngle:coneOuterAngle];
        if (coneOuterGain) [self setConeOuterGain:coneOuterGain];
        [self applySpatial];
    }
    return self;
}

#pragma mark - AudioParam hooks (param-driven automation)

- (void)installAudioParamHooks {
    __weak typeof(self) weakSelf = self;
    void (^hook)(NSCAudioParam *) = ^(NSCAudioParam *p) {
        __strong typeof(weakSelf) s = weakSelf;
        if (!s) return;
        [s applySpatial];
        [s ensureAutomationLink];
    };
    _positionXParam.onScheduleChanged = hook;
    _positionYParam.onScheduleChanged = hook;
    _positionZParam.onScheduleChanged = hook;
    _orientationXParam.onScheduleChanged = hook;
    _orientationYParam.onScheduleChanged = hook;
    _orientationZParam.onScheduleChanged = hook;
}

- (void)clearAudioParamHooks {
    _positionXParam.onScheduleChanged = nil;
    _positionYParam.onScheduleChanged = nil;
    _positionZParam.onScheduleChanged = nil;
    _orientationXParam.onScheduleChanged = nil;
    _orientationYParam.onScheduleChanged = nil;
    _orientationZParam.onScheduleChanged = nil;
}

- (BOOL)hasFutureAutomation {
    double t = [self currentTimeForApply];
    return [_positionXParam hasEventsAfter:t]
        || [_positionYParam hasEventsAfter:t]
        || [_positionZParam hasEventsAfter:t]
        || [_orientationXParam hasEventsAfter:t]
        || [_orientationYParam hasEventsAfter:t]
        || [_orientationZParam hasEventsAfter:t];
}

- (void)ensureAutomationLink {
    if (![NSThread isMainThread]) {
        __weak typeof(self) weakSelf = self;
        dispatch_async(dispatch_get_main_queue(), ^{ [weakSelf ensureAutomationLink]; });
        return;
    }
    if (_automationTimer) return;
    if (![self hasFutureAutomation]) return;
    dispatch_source_t timer = dispatch_source_create(DISPATCH_SOURCE_TYPE_TIMER, 0, 0,
        dispatch_get_main_queue());
    uint64_t intervalNs = (uint64_t)(NSEC_PER_SEC / 60.0);
    uint64_t leewayNs = (uint64_t)(NSEC_PER_SEC / 240.0);
    NSCAudioContext *ctx = self.context;
    double now = ctx ? ctx.currentTime : 0.0;
    if (now <= 0.0) {
        _automationUsesHostTime = YES;
        _automationAnchorHostTime = CACurrentMediaTime();
        _automationAnchorAudioTime = 0.0;
    } else {
        _automationUsesHostTime = NO;
        _automationAnchorHostTime = CACurrentMediaTime();
        _automationAnchorAudioTime = now;
    }
    dispatch_source_set_timer(timer, dispatch_time(DISPATCH_TIME_NOW, 0), intervalNs, leewayNs);
    __weak typeof(self) weakSelf = self;
    dispatch_source_set_event_handler(timer, ^{
        __strong typeof(weakSelf) s = weakSelf;
        if (!s) return;
        [s applySpatial];
        if (![s hasFutureAutomation]) [s stopAutomationLink];
    });
    _automationTimer = timer;
    dispatch_resume(timer);
}

- (void)stopAutomationLink {
    if (_automationTimer) {
        dispatch_source_cancel(_automationTimer);
        _automationTimer = nil;
    }
    _automationUsesHostTime = NO;
    _automationAnchorHostTime = 0;
    _automationAnchorAudioTime = 0.0;
}

#pragma mark - Spatial state apply

- (double)currentTimeForApply {
    NSCAudioContext *ctx = self.context;
    double t = ctx ? ctx.currentTime : 0.0;
    if (t > 0.0) {
        _automationUsesHostTime = NO;
        _automationAnchorHostTime = CACurrentMediaTime();
        _automationAnchorAudioTime = t;
        return t;
    }

    if (!_automationTimer) return t;

    double hostNow = CACurrentMediaTime();
    if (!_automationUsesHostTime) {
        _automationUsesHostTime = YES;
        _automationAnchorHostTime = hostNow;
        _automationAnchorAudioTime = 0.0;
        return 0.0;
    }

    double synthetic = _automationAnchorAudioTime + (hostNow - _automationAnchorHostTime);
    return synthetic >= 0.0 ? synthetic : 0.0;
}

- (void)readListenerStateAtTime:(double)t
                      positionX:(double *)listenerX
                      positionY:(double *)listenerY
                      positionZ:(double *)listenerZ
                       forwardX:(double *)forwardX
                       forwardY:(double *)forwardY
                       forwardZ:(double *)forwardZ
                            upX:(double *)upX
                            upY:(double *)upY
                            upZ:(double *)upZ {
    *listenerX = 0.0;
    *listenerY = 0.0;
    *listenerZ = 0.0;
    *forwardX = 0.0;
    *forwardY = 0.0;
    *forwardZ = -1.0;
    *upX = 0.0;
    *upY = 1.0;
    *upZ = 0.0;

    NSCAudioContext *ctx = self.context;
    if (!ctx) return;

    *listenerX = [[ctx getListenerPositionXParam] valueAtTime:t];
    *listenerY = [[ctx getListenerPositionYParam] valueAtTime:t];
    *listenerZ = [[ctx getListenerPositionZParam] valueAtTime:t];
    *forwardX = [[ctx getListenerForwardXParam] valueAtTime:t];
    *forwardY = [[ctx getListenerForwardYParam] valueAtTime:t];
    *forwardZ = [[ctx getListenerForwardZParam] valueAtTime:t];
    *upX = [[ctx getListenerUpXParam] valueAtTime:t];
    *upY = [[ctx getListenerUpYParam] valueAtTime:t];
    *upZ = [[ctx getListenerUpZParam] valueAtTime:t];
}

- (double)coneAndDistanceGainForListenerX:(double)listenerX
                                 listenerY:(double)listenerY
                                 listenerZ:(double)listenerZ {
    double dx = (double)_positionX - listenerX;
    double dy = (double)_positionY - listenerY;
    double dz = (double)_positionZ - listenerZ;
    double dist = sqrt(dx * dx + dy * dy + dz * dz);

    double ref = fmax(1e-6, (double)_refDistance);
    double maxd = fmax(ref, (double)_maxDistance);
    double rf = fmax(0.0, (double)_rolloffFactor);
    double d = fmin(fmax(dist, ref), maxd);

    double distGain = 1.0;
    switch (_distanceModel) {
        default:
        case 0: // inverse
            distGain = ref / (ref + rf * (d - ref));
            break;
        case 1: // linear
            if (maxd <= ref) distGain = dist > maxd ? 0.0 : 1.0;
            else distGain = 1.0 - rf * (d - ref) / (maxd - ref);
            break;
        case 2: // exponential
            distGain = pow(d / ref, -rf);
            break;
    }
    distGain = NSCClampDouble(distGain, 0.0, 1.0);

    double sx = listenerX - (double)_positionX;
    double sy = listenerY - (double)_positionY;
    double sz = listenerZ - (double)_positionZ;
    double slen = sqrt(sx * sx + sy * sy + sz * sz);
    if (slen <= 1e-12) return distGain;

    double ox = (double)_orientationX;
    double oy = (double)_orientationY;
    double oz = (double)_orientationZ;
    NSCNormalizeVector3(&ox, &oy, &oz, 1.0, 0.0, 0.0);

    double dot = (ox * sx + oy * sy + oz * sz) / slen;
    dot = NSCClampDouble(dot, -1.0, 1.0);
    double angDeg = acos(dot) * 180.0 / M_PI;

    double inner = NSCClampDouble((double)_coneInnerAngle, 0.0, 360.0) * 0.5;
    double outer = NSCClampDouble((double)_coneOuterAngle, 0.0, 360.0) * 0.5;
    if (outer < inner) inner = outer;
    double outGainV = NSCClampDouble((double)_coneOuterGain, 0.0, 1.0);

    double coneGain = 1.0;
    if (angDeg <= inner) {
        coneGain = 1.0;
    } else if (angDeg >= outer) {
        coneGain = outGainV;
    } else {
        double denom = outer - inner;
        if (denom <= 1e-12) coneGain = outGainV;
        else {
            double tt = (angDeg - inner) / denom;
            coneGain = (1.0 - tt) + tt * outGainV;
        }
    }

    return distGain * coneGain;
}

- (double)resolvedPanForListenerX:(double)listenerX
                         listenerY:(double)listenerY
                         listenerZ:(double)listenerZ
                          forwardX:(double)forwardX
                          forwardY:(double)forwardY
                          forwardZ:(double)forwardZ
                               upX:(double)upX
                               upY:(double)upY
                               upZ:(double)upZ {
    if (fabs((double)_pan) > 1e-12) {
        return NSCClampDouble((double)_pan, -1.0, 1.0);
    }

    double fx = forwardX;
    double fy = forwardY;
    double fz = forwardZ;
    NSCNormalizeVector3(&fx, &fy, &fz, 0.0, 0.0, -1.0);

    double ux = upX;
    double uy = upY;
    double uz = upZ;
    NSCNormalizeVector3(&ux, &uy, &uz, 0.0, 1.0, 0.0);

    double rxv = fy * uz - fz * uy;
    double ryv = fz * ux - fx * uz;
    double rzv = fx * uy - fy * ux;
    NSCNormalizeVector3(&rxv, &ryv, &rzv, 1.0, 0.0, 0.0);

    double relX = (double)_positionX - listenerX;
    double relY = (double)_positionY - listenerY;
    double relZ = (double)_positionZ - listenerZ;

    double localX = relX * rxv + relY * ryv + relZ * rzv;
    double localZ = relX * fx + relY * fy + relZ * fz;
    double azimuth = NSCWrapPannerAzimuthDegrees(atan2(localX, localZ) * 180.0 / M_PI);
    return NSCClampDouble(azimuth / 90.0, -1.0, 1.0);
}

- (void)applySpatial {
    double t = [self currentTimeForApply];

    _positionX = (float)[_positionXParam valueAtTime:t];
    _positionY = (float)[_positionYParam valueAtTime:t];
    _positionZ = (float)[_positionZParam valueAtTime:t];
    _orientationX = (float)[_orientationXParam valueAtTime:t];
    _orientationY = (float)[_orientationYParam valueAtTime:t];
    _orientationZ = (float)[_orientationZParam valueAtTime:t];

    double listenerX = 0.0;
    double listenerY = 0.0;
    double listenerZ = 0.0;
    double forwardX = 0.0;
    double forwardY = 0.0;
    double forwardZ = -1.0;
    double upX = 0.0;
    double upY = 1.0;
    double upZ = 0.0;
    [self readListenerStateAtTime:t
                        positionX:&listenerX
                        positionY:&listenerY
                        positionZ:&listenerZ
                         forwardX:&forwardX
                         forwardY:&forwardY
                         forwardZ:&forwardZ
                              upX:&upX
                              upY:&upY
                              upZ:&upZ];

    double gain = [self coneAndDistanceGainForListenerX:listenerX
                                              listenerY:listenerY
                                              listenerZ:listenerZ];
    double panVal = [self resolvedPanForListenerX:listenerX
                                        listenerY:listenerY
                                        listenerZ:listenerZ
                                         forwardX:forwardX
                                         forwardY:forwardY
                                         forwardZ:forwardZ
                                              upX:upX
                                              upY:upY
                                              upZ:upZ];

    PanRender *r = ((NSCPanUnit *)_unit.AUAudioUnit).render;
    atomic_store_explicit(&r->gain, (float)gain, memory_order_relaxed);
    atomic_store_explicit(&r->azimuth, (float)(panVal * 90.0), memory_order_relaxed);
}

#pragma mark - Source attach / connect

- (void)attachSource:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    [_input handleConnectFrom:source output:output input:nil];
}

- (void)detachSource:(NSCAudioNode *)source {
    [_input handleDisconnectFrom:source output:nil input:nil];
}

- (void)disconnectPanner {
    [self disconnect];
}

- (void)handleConnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    [_input handleConnectFrom:source output:output input:nil];
}

- (void)handleDisconnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    [_input handleDisconnectFrom:source output:output input:nil];
}

#pragma mark - Setters / getters

- (void)setPan:(NSNumber *)p {
    _pan = (float)NSCClampDouble([p doubleValue], -1.0, 1.0);
    [self applySpatial];
}

- (void)setDistanceModel:(NSNumber *)v { _distanceModel = v.intValue; [self applySpatial]; }
- (NSNumber *)getDistanceModel { return @(_distanceModel); }

- (void)setPanningModel:(NSNumber *)v {
    int newPM = v.intValue == 1 ? 1 : 0;
    if (newPM == _panningModel) return;

    // HRTF renders as equalpower.
    _panningModel = newPM;
    [self applySpatial];
    NSCLogDebug(@"NSCAudioPannerNode: setPanningModel -> %d", _panningModel);
}
- (NSNumber *)getPanningModel { return @(_panningModel); }

- (void)setRefDistance:(NSNumber *)v { _refDistance = [v floatValue]; [self applySpatial]; }
- (NSNumber *)getRefDistance { return @((double)_refDistance); }
- (void)setMaxDistance:(NSNumber *)v { _maxDistance = [v floatValue]; [self applySpatial]; }
- (NSNumber *)getMaxDistance { return @((double)_maxDistance); }
- (void)setRolloffFactor:(NSNumber *)v { _rolloffFactor = [v floatValue]; [self applySpatial]; }
- (NSNumber *)getRolloffFactor { return @((double)_rolloffFactor); }
 

 - (void)setConeInnerAngle:(NSNumber *)v { _coneInnerAngle = [v floatValue]; NSCLogDebug(@"NSCAudioPannerNode: setConeInnerAngle -> %f", _coneInnerAngle); [self applySpatial]; }
 - (NSNumber *)getConeInnerAngle { return @((double)_coneInnerAngle); }
 - (void)setConeOuterAngle:(NSNumber *)v { _coneOuterAngle = [v floatValue]; NSCLogDebug(@"NSCAudioPannerNode: setConeOuterAngle -> %f", _coneOuterAngle); [self applySpatial]; }
 - (NSNumber *)getConeOuterAngle { return @((double)_coneOuterAngle); }
 - (void)setConeOuterGain:(NSNumber *)v { _coneOuterGain = [v floatValue]; NSCLogDebug(@"NSCAudioPannerNode: setConeOuterGain -> %f", _coneOuterGain); [self applySpatial]; }
 - (NSNumber *)getConeOuterGain { return @((double)_coneOuterGain); }

- (void)setPosition:(NSNumber *)x :(NSNumber *)y :(NSNumber *)z {
    double now = self.context ? self.context.currentTime : 0.0;
    [self.positionXParam setValueAtTime:x :@(now)];
    [self.positionYParam setValueAtTime:y :@(now)];
    [self.positionZParam setValueAtTime:z :@(now)];
    NSCLogDebug(@"NSCAudioPannerNode: setPosition -> x=%f y=%f z=%f", [x doubleValue], [y doubleValue], [z doubleValue]);
    [self applySpatial];
}

- (void)setOrientation:(NSNumber *)x :(NSNumber *)y :(NSNumber *)z {
    double now = self.context ? self.context.currentTime : 0.0;
    [self.orientationXParam setValueAtTime:x :@(now)];
    [self.orientationYParam setValueAtTime:y :@(now)];
    [self.orientationZParam setValueAtTime:z :@(now)];
    NSCLogDebug(@"NSCAudioPannerNode: setOrientation -> x=%f y=%f z=%f", [x doubleValue], [y doubleValue], [z doubleValue]);
    [self applySpatial];
}

- (void)attachParamSink:(AVAudioNode *)sink {
    [super attachParamSink:sink];
    [self ensureAutomationLink];
}

- (void)applyAutomation {
    [self applySpatial];
}

@end
