#import <objc/message.h>
#import <QuartzCore/QuartzCore.h>
#import "NSCAudioLog.h"
#import "NSCAudioContext.h"

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

@implementation NSCAudioPannerNode {
    AVAudioUnit *_au;
    AVAudioMixerNode *_panMixer;
    AVAudioUnitEQ *_occlusionEq;
    AVAudioEnvironmentNode *_envNode;
    BOOL _envNodeOwned;
    AVAudioFormat *_monoFormat;
    BOOL _connectedDownstream;
    float _positionX, _positionY, _positionZ;
    float _orientationX, _orientationY, _orientationZ;
    float _pan;
    int _distanceModel;
    int _panningModel;
    float _refDistance, _maxDistance, _rolloffFactor;
    float _coneInnerAngle, _coneOuterAngle, _coneOuterGain;
    NSMutableDictionary<NSValue *, NSCAudioNode *> *_attachedSourcesMap;
    NSMutableDictionary<NSValue *, NSString *> *_sourceRouting;
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
    BOOL useHRTF = (panningModel == 1);
    double sr = context.sampleRate > 0 ? context.sampleRate : 44100.0;
    AVAudioFormat *monoFmt = [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32
                                                              sampleRate:sr
                                                                channels:1
                                                             interleaved:NO];

    AVAudioNode *avNode = nil;
    AVAudioMixerNode *mixer = [[AVAudioMixerNode alloc] init];
    @try { [context.engine attachNode:mixer]; } @catch (NSException *e) { mixer = nil; }
    AVAudioUnitEQ *occlEq = nil;
    AVAudioEnvironmentNode *env = nil;

    if (useHRTF) {
        [context ensureEnvironmentNodeAttached];
        env = context.environmentNode;
        avNode = env ?: mixer;
    } else {
        avNode = mixer;
    }

    if (!avNode) return nil;

    if (self = [super initWithContext:context node:avNode]) {
        _au = nil;
        _panMixer = mixer;
        _occlusionEq = occlEq;
        _envNode = env;
        _envNodeOwned = (env && env != context.environmentNode) ? YES : NO;
        _monoFormat = monoFmt;
        _connectedDownstream = NO;
        _positionX = (float)positionX; _positionY = (float)positionY; _positionZ = (float)positionZ;
        _pan = 0.0f;
        _positionXParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:positionX];
        _positionYParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:positionY];
        _positionZParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:positionZ];
        _orientationXParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:1.0];
        _orientationYParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:0.0];
        _orientationZParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:0.0];
        _orientationX = 1.0f; _orientationY = 0.0f; _orientationZ = 0.0f;
        _attachedSourcesMap = [NSMutableDictionary dictionary];
        _sourceRouting = [NSMutableDictionary dictionary];
        _panningModel = panningModel;
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
    if (_occlusionEq) {
        AVAudioEngine *engine = self.context.engine;
        if (engine) {
            @try {
                [engine disconnectNodeInput:_occlusionEq];
                [engine disconnectNodeOutput:_occlusionEq];
                [self.context detachNode:_occlusionEq fromEngine:engine];
            } @catch (NSException *e) {}
        }
        _occlusionEq = nil;
    }
    if (_panMixer) {
        AVAudioEngine *engine = self.context.engine;
        if (engine) {
            @try {
                [engine disconnectNodeInput:_panMixer];
                [engine disconnectNodeOutput:_panMixer];
                [self.context detachNode:_panMixer fromEngine:engine];
            } @catch (NSException *e) {}
        }
        _panMixer = nil;
    }
    if (_envNode) {
        if (_envNodeOwned) {
            AVAudioEngine *engine = self.context.engine;
            if (engine) {
                @try {
                    [engine disconnectNodeInput:_envNode];
                    [engine disconnectNodeOutput:_envNode];
                    [self.context detachNode:_envNode fromEngine:engine];
                } @catch (NSException *e) {}
            }
        }
        _envNode = nil;
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

- (void)ensureEnvironmentNodeForCurrentModel {
    if (_panningModel != 1 || _envNode) return;
    NSCAudioContext *ctx = self.context;
    if (!ctx) return;
    [ctx ensureEnvironmentNodeAttached];
    _envNode = ctx.environmentNode;
    _envNodeOwned = NO;
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
    [self ensureEnvironmentNodeForCurrentModel];

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

    AVAudio3DPoint pos = AVAudioMake3DPoint(_positionX, _positionY, _positionZ);
    AVAudio3DMixingRenderingAlgorithm renderAlgo = _panningModel == 1
        ? AVAudio3DMixingRenderingAlgorithmHRTF
        : AVAudio3DMixingRenderingAlgorithmEqualPowerPanning;

    if (_envNode) {
        for (NSValue *idv in _attachedSourcesMap) {
            NSCAudioNode *src = _attachedSourcesMap[idv];
            if ([src.avNode conformsToProtocol:@protocol(AVAudio3DMixing)]) {
                id<AVAudio3DMixing> mix3d = (id<AVAudio3DMixing>)src.avNode;
                mix3d.position = pos;
                mix3d.renderingAlgorithm = renderAlgo;
            }
        }
        _envNode.outputVolume = 1.0f;
    }

    if (_panMixer) {
        _panMixer.pan = (float)panVal;
        _panMixer.outputVolume = (float)gain;
        NSCLogDebug(@"NSCAudioPannerNode: applySpatial pan=%f gain=%f model=%d", panVal, gain, _panningModel);
    }

    if (_occlusionEq) {
        double g = NSCClampDouble(gain, 0.0, 1.0);
        double lowFreq = 800.0;
        double highFreq = 22050.0;
        double cutoff = lowFreq + (highFreq - lowFreq) * pow(g, 0.5);
        AVAudioUnitEQFilterParameters *band = _occlusionEq.bands[0];
        band.filterType = AVAudioUnitEQFilterTypeLowPass;
        band.frequency = (float)cutoff;
        band.bandwidth = 0.5f;
        band.bypass = (g >= 0.999);
    }
}

#pragma mark - Source attach / connect

- (AVAudioNode *)activeAVNode {
    return (AVAudioNode *)(_envNode ?: _panMixer);
}

- (void)attachSource:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    NSCAudioContext *ctx = self.context;
    AVAudioNode *target = [self activeAVNode];
    if (!ctx || !target) return;
    NSCLogDebug(@"NSCAudioPannerNode: attachSource called source=%@(avNode=%@) attachedCount=%lu", NSStringFromClass([source class]), NSStringFromClass([source.avNode class]), (unsigned long)(_attachedSourcesMap ? _attachedSourcesMap.count : 0));
    NSValue *idv = [NSValue valueWithPointer:(__bridge const void *)(source)];
    if (_attachedSourcesMap[idv]) return;
    [ctx.engine disconnectNodeOutput:source.avNode];
    AVAudioNodeBus fromBus = (AVAudioNodeBus)MAX(0, output ? output.intValue : 0);
    AVAudioNodeBus toBus = 0;
    if (input) toBus = (AVAudioNodeBus)MAX(0, input.intValue);
    else if ([target isKindOfClass:[AVAudioMixerNode class]]) {
        toBus = ((AVAudioMixerNode *)target).nextAvailableInputBus;
    }
    AVAudioConnectionPoint *destPoint = [[AVAudioConnectionPoint alloc] initWithNode:target bus:toBus];

        if (_envNode) {
        if ([source.avNode conformsToProtocol:@protocol(AVAudio3DMixing)] && ![source.avNode isKindOfClass:[AVAudioSourceNode class]]) {
            BOOL connected = NO;
            @try {
                [ctx.engine connect:source.avNode toConnectionPoints:@[destPoint] fromBus:fromBus format:_monoFormat];
                connected = YES;
            } @catch (NSException *e) {
                NSCLogDebug(@"NSCAudioPannerNode: attachSource env connect threw: %@", e);
                @try {
                    [ctx.engine connect:source.avNode toConnectionPoints:@[destPoint] fromBus:fromBus format:nil];
                    connected = YES;
                } @catch (NSException *e2) {
                    NSCLogDebug(@"NSCAudioPannerNode: attachSource env connect fallback threw: %@", e2);
                }
            }
            if (connected) {
                NSCLogDebug(@"NSCAudioPannerNode: attachSource connected source=%@ -> env dest=%@ fromBus=%d toBus=%d", NSStringFromClass([source.avNode class]), NSStringFromClass([((AVAudioNode *)destPoint.node) class]), (int)fromBus, (int)toBus);
            }
            if (!connected) {
                NSCLogDebug(@"NSCAudioPannerNode: attachSource failed env route source=%@", NSStringFromClass([source.avNode class]));
                return;
            }
            id<AVAudio3DMixing> mix3d = (id<AVAudio3DMixing>)source.avNode;
            mix3d.position = AVAudioMake3DPoint(_positionX, _positionY, _positionZ);
            mix3d.renderingAlgorithm = _panningModel == 1
                ? AVAudio3DMixingRenderingAlgorithmHRTF
                : AVAudio3DMixingRenderingAlgorithmEqualPowerPanning;
            _sourceRouting[idv] = @"env";
        } else if (_panMixer) {
            NSCLogDebug(@"NSCAudioPannerNode: attachSource: source does not support 3D mixing; falling back to mixer (source=%@)", NSStringFromClass([source.avNode class]));
            AVAudioConnectionPoint *fallbackDest = [[AVAudioConnectionPoint alloc] initWithNode:_panMixer bus:((AVAudioMixerNode *)_panMixer).nextAvailableInputBus];
            BOOL connected = NO;
            @try {
                [ctx.engine connect:source.avNode toConnectionPoints:@[fallbackDest] fromBus:fromBus format:nil];
                connected = YES;
            } @catch (NSException *e) {
                NSCLogDebug(@"NSCAudioPannerNode: attachSource mixer connect threw: %@", e);
                @try {
                    [ctx.engine connect:source.avNode toConnectionPoints:@[fallbackDest] fromBus:fromBus format:_monoFormat];
                    connected = YES;
                } @catch (NSException *e2) {
                    NSCLogDebug(@"NSCAudioPannerNode: attachSource mixer connect fallback threw: %@", e2);
                }
            }
            if (connected) NSCLogDebug(@"NSCAudioPannerNode: attachSource connected source=%@ -> panMixer (bus=%lu)", NSStringFromClass([source.avNode class]), (unsigned long)(((AVAudioMixerNode *)_panMixer).nextAvailableInputBus - 1));
            if (!connected) {
                NSCLogDebug(@"NSCAudioPannerNode: attachSource failed mixer fallback source=%@", NSStringFromClass([source.avNode class]));
                return;
            }
            _sourceRouting[idv] = @"mix";
        } else {
            BOOL connected = NO;
            @try {
                [ctx.engine connect:source.avNode toConnectionPoints:@[destPoint] fromBus:fromBus format:nil];
                connected = YES;
            } @catch (NSException *e) {
                NSCLogDebug(@"NSCAudioPannerNode: attachSource generic connect threw: %@", e);
                @try {
                    [ctx.engine connect:source.avNode toConnectionPoints:@[destPoint] fromBus:fromBus format:_monoFormat];
                    connected = YES;
                } @catch (NSException *e2) {
                    NSCLogDebug(@"NSCAudioPannerNode: attachSource generic connect fallback threw: %@", e2);
                }
            }
            if (connected) NSCLogDebug(@"NSCAudioPannerNode: attachSource connected source=%@ -> dest=%@", NSStringFromClass([source.avNode class]), NSStringFromClass([((AVAudioNode *)destPoint.node) class]));
            if (!connected) {
                NSCLogDebug(@"NSCAudioPannerNode: attachSource failed generic env source=%@", NSStringFromClass([source.avNode class]));
                return;
            }
            _sourceRouting[idv] = @"env";
        }
    } else {
        BOOL connected = NO;
        @try {
            [ctx.engine connect:source.avNode toConnectionPoints:@[destPoint] fromBus:fromBus format:nil];
            connected = YES;
        } @catch (NSException *e) {
            NSCLogDebug(@"NSCAudioPannerNode: attachSource default connect threw: %@", e);
            @try {
                [ctx.engine connect:source.avNode toConnectionPoints:@[destPoint] fromBus:fromBus format:ctx.format];
                connected = YES;
            } @catch (NSException *e2) {
                NSCLogDebug(@"NSCAudioPannerNode: attachSource ctx.format fallback threw: %@", e2);
                @try {
                    [ctx.engine connect:source.avNode toConnectionPoints:@[destPoint] fromBus:fromBus format:_monoFormat];
                    connected = YES;
                } @catch (NSException *e3) {
                    NSCLogDebug(@"NSCAudioPannerNode: attachSource mono fallback threw: %@", e3);
                }
            }
        }
        if (connected) {
            NSCLogDebug(@"NSCAudioPannerNode: attachSource connected source=%@ -> panMixer/default (fromBus=%d toBus=%d)", NSStringFromClass([source.avNode class]), (int)fromBus, (int)toBus);
        }
        if (!connected) {
            NSCLogDebug(@"NSCAudioPannerNode: attachSource failed default route source=%@", NSStringFromClass([source.avNode class]));
            return;
        }
        _sourceRouting[idv] = @"mix";
    }

    _attachedSourcesMap[idv] = source;
    NSMutableDictionary *pmap = ctx.voicePannerByOutput[idv];
    if (!pmap) pmap = [NSMutableDictionary dictionary];
    pmap[@(output ? output.intValue : 0)] = self;
    ctx.voicePannerByOutput[idv] = pmap;

    [self applySpatial];
}


- (void)connectTo:(NSCAudioNode *)destination {
    [self connectTo:destination output:nil input:nil];
}

- (void)connectTo:(NSCAudioNode *)destination output:(NSNumber *)output {
    [self connectTo:destination output:output input:@(0)];
}

- (void)connectTo:(NSCAudioNode *)destination output:(NSNumber *)output input:(NSNumber *)input {
    NSCAudioContext *ctx = self.context ?: destination.context;
    AVAudioEngine *engine = ctx.engine;
    AVAudioNode *src = [self activeAVNode];
    if (!engine || !src || !destination.avNode) return;

    AVAudioNodeBus fromBus = (AVAudioNodeBus)MAX(0, output ? output.intValue : 0);
    AVAudioNodeBus toBus = 0;
    if (input) toBus = (AVAudioNodeBus)MAX(0, input.intValue);
    else if ([destination.avNode isKindOfClass:[AVAudioMixerNode class]]) {
        toBus = ((AVAudioMixerNode *)destination.avNode).nextAvailableInputBus;
    }
    AVAudioConnectionPoint *destPoint = [[AVAudioConnectionPoint alloc] initWithNode:destination.avNode bus:toBus];

    if (_envNode) {
        BOOL connected = NO;
        @try {
            [engine connect:_envNode toConnectionPoints:@[destPoint] fromBus:fromBus format:nil];
            NSCLogDebug(@"NSCAudioPannerNode: connected envNode -> %@ (fromBus=%d toBus=%d)", NSStringFromClass([destination.avNode class]), (int)fromBus, (int)toBus);
            connected = YES;
        } @catch (NSException *e) {
            NSCLogDebug(@"NSCAudioPannerNode: envNode connect to destination threw: %@", e);
            @try {
                [engine connect:_envNode toConnectionPoints:@[destPoint] fromBus:fromBus format:_monoFormat];
                connected = YES;
            } @catch (NSException *e2) { NSCLogDebug(@"NSCAudioPannerNode: envNode connect fallback also threw: %@", e2); }
        }

        if (_panMixer) {
            @try {
                [engine connect:_panMixer toConnectionPoints:@[destPoint] fromBus:0 format:_monoFormat];
                NSCLogDebug(@"NSCAudioPannerNode: connected panMixer -> %@ (fromBus=0 toBus=%d)", NSStringFromClass([destination.avNode class]), (int)toBus);
                connected = YES;
            } @catch (NSException *e) {
                NSCLogDebug(@"NSCAudioPannerNode: panMixer connect to destination threw: %@", e);
                @try {
                    [engine connect:_panMixer toConnectionPoints:@[destPoint] fromBus:0 format:nil];
                    connected = YES;
                } @catch (NSException *e2) { NSCLogDebug(@"NSCAudioPannerNode: panMixer connect fallback also threw: %@", e2); }
            }
        }

        if (!connected) {
            NSCLogDebug(@"NSCAudioPannerNode: connectTo failed env src path dest=%@", NSStringFromClass([destination.avNode class]));
            return;
        }
    } else {
        BOOL connected = NO;
        @try {
            [engine connect:src toConnectionPoints:@[destPoint] fromBus:fromBus format:nil];
            connected = YES;
        } @catch (NSException *e) {
            NSCLogDebug(@"NSCAudioPannerNode: connectTo default connect threw: %@", e);
            @try {
                [engine connect:src toConnectionPoints:@[destPoint] fromBus:fromBus format:ctx ? ctx.format : nil];
                connected = YES;
            } @catch (NSException *e2) {
                NSCLogDebug(@"NSCAudioPannerNode: connectTo ctx.format fallback threw: %@", e2);
                @try {
                    [engine connect:src toConnectionPoints:@[destPoint] fromBus:fromBus format:_monoFormat];
                    connected = YES;
                } @catch (NSException *e3) {
                    NSCLogDebug(@"NSCAudioPannerNode: connectTo mono fallback threw: %@", e3);
                }
            }
        }
        if (!connected) {
            NSCLogDebug(@"NSCAudioPannerNode: connectTo failed src=%@ dest=%@", NSStringFromClass([src class]), NSStringFromClass([destination.avNode class]));
            return;
        }
    }
    _connectedDownstream = YES;
    [self applySpatial];
}

- (void)detachSource:(NSCAudioNode *)source {
    NSCAudioContext *ctx = self.context;
    if (!ctx) return;
    NSValue *idv = [NSValue valueWithPointer:(__bridge const void *)(source)];
    [ctx.engine disconnectNodeOutput:source.avNode];
    [_attachedSourcesMap removeObjectForKey:idv];
    [_sourceRouting removeObjectForKey:idv];
    NSMutableDictionary *pmap = ctx.voicePannerByOutput[idv];
    if (pmap) {
        NSMutableArray *keysToRemove = [NSMutableArray array];
        for (NSNumber *k in pmap) {
            NSCAudioNode *v = pmap[k];
            if (v == self) [keysToRemove addObject:k];
        }
        for (NSNumber *k in keysToRemove) [pmap removeObjectForKey:k];
        ctx.voicePannerByOutput[idv] = pmap;
    }
}

- (void)disconnectPanner {
    NSCAudioContext *ctx = self.context;
    if (!ctx) return;
    for (NSValue *v in _attachedSourcesMap) {
        NSCAudioNode *src = _attachedSourcesMap[v];
        [ctx.engine disconnectNodeOutput:src.avNode];
    }
    for (NSValue *idv in _attachedSourcesMap) {
        NSMutableDictionary *pmap = ctx.voicePannerByOutput[idv];
        if (pmap) {
            NSMutableArray *keysToRemove = [NSMutableArray array];
            for (NSNumber *k in pmap) {
                NSCAudioNode *val = pmap[k];
                if (val == self) [keysToRemove addObject:k];
            }
            for (NSNumber *k in keysToRemove) [pmap removeObjectForKey:k];
            ctx.voicePannerByOutput[idv] = pmap;
        }
    }
    [_attachedSourcesMap removeAllObjects];
    [_sourceRouting removeAllObjects];
    [ctx.engine disconnectNodeOutput:self.avNode];
}

- (void)handleConnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    [self attachSource:source output:output input:input];
}

- (void)handleDisconnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    NSCAudioContext *ctx = self.context;
    if (!ctx) return;
    NSValue *idv = [NSValue valueWithPointer:(__bridge const void *)(source)];
    if (output) {
        [ctx.engine disconnectNodeOutput:source.avNode];
        NSMutableDictionary *pmap = ctx.voicePannerByOutput[idv];
        if (pmap) [pmap removeObjectForKey:@(output.intValue)];
        [_attachedSourcesMap removeObjectForKey:idv];
        [_sourceRouting removeObjectForKey:idv];
    } else {
        [self detachSource:source];
    }
}

#pragma mark - Setters / getters

- (void)setPan:(NSNumber *)p {
    _pan = (float)NSCClampDouble([p doubleValue], -1.0, 1.0);
    [self applySpatial];
    NSCLogDebug(@"NSCAudioPannerNode: setPan -> %f env=%@ panMixer=%@", _pan, _envNode ? @"YES" : @"NO", _panMixer ? @"YES" : @"NO");
}

- (void)setDistanceModel:(NSNumber *)v { _distanceModel = v.intValue; [self applySpatial]; }
- (NSNumber *)getDistanceModel { return @(_distanceModel); }

- (void)setPanningModel:(NSNumber *)v {
    int newPM = v.intValue == 1 ? 1 : 0;
    if (newPM == _panningModel) return;

    _panningModel = newPM;
    if (_panningModel == 1) {
        [self ensureEnvironmentNodeForCurrentModel];
    }

    if (_attachedSourcesMap.count > 0) {
        NSArray<NSCAudioNode *> *sources = _attachedSourcesMap.allValues.copy;
        for (NSCAudioNode *source in sources) {
            [self detachSource:source];
        }
        for (NSCAudioNode *source in sources) {
            [self attachSource:source output:nil input:nil];
        }
    }

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

@end
