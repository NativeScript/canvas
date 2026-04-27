#import <objc/message.h>
#import "NSCAudioContext.h"

@implementation NSCAudioPannerNode {
    AVAudioUnit *_au;
    AVAudioMixerNode *_panMixer;
    AVAudioEnvironmentNode *_envNode;
    BOOL _envNodeOwned;
    AVAudioFormat *_monoFormat;
    BOOL _connectedDownstream;
    float _positionX, _positionY, _positionZ;
    float _orientationX, _orientationY, _orientationZ;
    int _distanceModel;
    int _panningModel;
    float _refDistance, _maxDistance, _rolloffFactor;
    float _coneInnerAngle, _coneOuterAngle, _coneOuterGain;
    NSMutableDictionary<NSValue *, NSCAudioNode *> *_attachedSourcesMap;
    NSMutableDictionary<NSValue *, NSString *> *_sourceRouting;
    dispatch_source_t _automationTimer;
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

    AVAudioNode *avNode;
    AVAudioMixerNode *mixer = nil;
    AVAudioEnvironmentNode *env = nil;

    if (useHRTF) {
        [context ensureEnvironmentNodeAttached];
        env = context.environmentNode;
        avNode = env;
    } else {
        mixer = [[AVAudioMixerNode alloc] init];
        [context.engine attachNode:mixer];
        avNode = mixer;
    }

    if (self = [super initWithContext:context node:avNode]) {
        _au = nil;
        _panMixer = mixer;
        _envNode = env;
        _envNodeOwned = (env && env != context.environmentNode) ? YES : NO;
        _monoFormat = monoFmt;
        _connectedDownstream = NO;
        _positionX = (float)positionX; _positionY = (float)positionY; _positionZ = (float)positionZ;
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

        [self installAudioParamHooks];
        [self applySpatial];
    }
    return self;
}

- (void)dealloc {
    [self stopAutomationLink];
    [self clearAudioParamHooks];
    if (_panMixer) {
        AVAudioEngine *engine = self.context.engine;
        if (engine) {
            @try {
                [engine disconnectNodeInput:_panMixer];
                [engine disconnectNodeOutput:_panMixer];
                [engine detachNode:_panMixer];
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
                    [engine detachNode:_envNode];
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
    NSCAudioContext *ctx = self.context;
    double t = ctx ? ctx.currentTime : 0.0;
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
}

#pragma mark - Spatial state apply

- (double)currentTimeForApply {
    NSCAudioContext *ctx = self.context;
    return ctx ? ctx.currentTime : 0.0;
}

- (double)coneAndDistanceGain {
    double dx = _positionX, dy = _positionY, dz = _positionZ;
    double dist = sqrt(dx * dx + dy * dy + dz * dz);

    double distGain = 1.0;
    double ref = _refDistance > 0.0f ? (double)_refDistance : 1.0;
    double maxd = _maxDistance > 0.0f ? (double)_maxDistance : 10000.0;
    double rf = _rolloffFactor > 0.0f ? (double)_rolloffFactor : 1.0;
    if (dist > ref) {
        switch (_distanceModel) {
            default:
            case 0: // inverse
                distGain = ref / (ref + rf * (dist - ref));
                break;
            case 1: // linear
                if (dist >= maxd) distGain = 0.0;
                else distGain = 1.0 - rf * (dist - ref) / (maxd - ref);
                break;
            case 2: // exponential
                distGain = pow(dist / ref, -rf);
                break;
        }
    }
    if (distGain < 0.0) distGain = 0.0;
    if (distGain > 1.0) distGain = 1.0;

    double coneGain = 1.0;
    double sx = -dx, sy = -dy, sz = -dz;
    double slen = sqrt(sx * sx + sy * sy + sz * sz);
    if (slen > 1e-12) {
        double ox = _orientationX, oy = _orientationY, oz = _orientationZ;
        double olen = sqrt(ox * ox + oy * oy + oz * oz);
        if (olen <= 1e-12) { ox = 1.0; oy = 0.0; oz = 0.0; olen = 1.0; }
        double dot = (ox * sx + oy * sy + oz * sz) / (olen * slen);
        if (dot > 1.0) dot = 1.0;
        if (dot < -1.0) dot = -1.0;
        double angDeg = acos(dot) * 180.0 / M_PI;
        double inner = _coneInnerAngle, outer = _coneOuterAngle;
        double outGainV = _coneOuterGain;
        if (angDeg <= inner) {
            coneGain = 1.0;
        } else if (angDeg >= outer) {
            coneGain = outGainV;
        } else {
            double denom = outer - inner;
            if (denom <= 0.0) coneGain = outGainV;
            else {
                double tt = (angDeg - inner) / denom;
                coneGain = (1.0 - tt) + tt * outGainV;
            }
        }
    }
    return distGain * coneGain;
}

- (void)applySpatial {
    double t = [self currentTimeForApply];

    _positionX = (float)[_positionXParam valueAtTime:t];
    _positionY = (float)[_positionYParam valueAtTime:t];
    _positionZ = (float)[_positionZParam valueAtTime:t];
    _orientationX = (float)[_orientationXParam valueAtTime:t];
    _orientationY = (float)[_orientationYParam valueAtTime:t];
    _orientationZ = (float)[_orientationZParam valueAtTime:t];

    double gain = [self coneAndDistanceGain];

    if (_envNode) {
        AVAudio3DPoint pos = AVAudioMake3DPoint(_positionX, _positionY, _positionZ);
        for (NSValue *idv in _attachedSourcesMap) {
            NSCAudioNode *src = _attachedSourcesMap[idv];
            if ([src.avNode conformsToProtocol:@protocol(AVAudio3DMixing)]) {
                id<AVAudio3DMixing> mix3d = (id<AVAudio3DMixing>)src.avNode;
                mix3d.position = pos;
            }
        }
        _envNode.outputVolume = (float)gain;
    } else if (_panMixer) {
        double panVal;
        if (_positionX != 0.0f || _positionZ != 0.0f) {
            double az = atan2((double)_positionX, (double)_positionZ);
            panVal = sin(az);
        } else {
            panVal = (double)_positionX;
        }
        if (panVal < -1.0) panVal = -1.0;
        if (panVal > 1.0) panVal = 1.0;
        _panMixer.pan = (float)panVal;
        _panMixer.outputVolume = (float)gain;
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
        @try {
            [ctx.engine connect:source.avNode toConnectionPoints:@[destPoint] fromBus:fromBus format:_monoFormat];
        } @catch (NSException *e) {
            [ctx.engine connect:source.avNode toConnectionPoints:@[destPoint] fromBus:fromBus format:nil];
        }
        if ([source.avNode conformsToProtocol:@protocol(AVAudio3DMixing)]) {
            id<AVAudio3DMixing> mix3d = (id<AVAudio3DMixing>)source.avNode;
            mix3d.position = AVAudioMake3DPoint(_positionX, _positionY, _positionZ);
            mix3d.renderingAlgorithm = AVAudio3DMixingRenderingAlgorithmHRTF;
        }
        _sourceRouting[idv] = @"env";
    } else {
        [ctx.engine connect:source.avNode toConnectionPoints:@[destPoint] fromBus:fromBus format:nil];
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
        [engine connect:src toConnectionPoints:@[destPoint] fromBus:fromBus format:nil];
    } else {
        @try {
            [engine connect:src toConnectionPoints:@[destPoint] fromBus:fromBus format:_monoFormat];
        } @catch (NSException *e) {
            [engine connect:src toConnectionPoints:@[destPoint] fromBus:fromBus format:nil];
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
    float val = [p floatValue];
    if (val < -1.0f) val = -1.0f;
    if (val > 1.0f) val = 1.0f;
    if (_panMixer) _panMixer.pan = val;
    if (_envNode) {
        AVAudio3DPoint pos = AVAudioMake3DPoint(val, 0.0f, 0.0f);
        for (NSValue *idv in _attachedSourcesMap) {
            NSCAudioNode *src = _attachedSourcesMap[idv];
            if ([src.avNode conformsToProtocol:@protocol(AVAudio3DMixing)]) {
                id<AVAudio3DMixing> mix3d = (id<AVAudio3DMixing>)src.avNode;
                mix3d.position = pos;
            }
        }
    }
}

- (void)setDistanceModel:(NSNumber *)v { _distanceModel = v.intValue; [self applySpatial]; }
- (NSNumber *)getDistanceModel { return @(_distanceModel); }

- (void)setPanningModel:(NSNumber *)v {
    int newPM = v.intValue;
    if (newPM == _panningModel) return;
    NSLog(@"NSCAudioPannerNode: panningModel can only be set at construction; "
          @"runtime change ignored. Recreate the panner with createPanner({ panningModel: ... }) to switch.");
}
- (NSNumber *)getPanningModel { return @(_panningModel); }

- (void)setRefDistance:(NSNumber *)v { _refDistance = [v floatValue]; [self applySpatial]; }
- (NSNumber *)getRefDistance { return @((double)_refDistance); }
- (void)setMaxDistance:(NSNumber *)v { _maxDistance = [v floatValue]; [self applySpatial]; }
- (NSNumber *)getMaxDistance { return @((double)_maxDistance); }
- (void)setRolloffFactor:(NSNumber *)v { _rolloffFactor = [v floatValue]; [self applySpatial]; }
- (NSNumber *)getRolloffFactor { return @((double)_rolloffFactor); }
- (void)setConeInnerAngle:(NSNumber *)v { _coneInnerAngle = [v floatValue]; [self applySpatial]; }
- (NSNumber *)getConeInnerAngle { return @((double)_coneInnerAngle); }
- (void)setConeOuterAngle:(NSNumber *)v { _coneOuterAngle = [v floatValue]; [self applySpatial]; }
- (NSNumber *)getConeOuterAngle { return @((double)_coneOuterAngle); }
- (void)setConeOuterGain:(NSNumber *)v { _coneOuterGain = [v floatValue]; [self applySpatial]; }
- (NSNumber *)getConeOuterGain { return @((double)_coneOuterGain); }

- (void)setPosition:(NSNumber *)x :(NSNumber *)y :(NSNumber *)z {
    double now = self.context ? self.context.currentTime : 0.0;
    [self.positionXParam setValueAtTime:x :@(now)];
    [self.positionYParam setValueAtTime:y :@(now)];
    [self.positionZParam setValueAtTime:z :@(now)];
    [self applySpatial];
}

- (void)setOrientation:(NSNumber *)x :(NSNumber *)y :(NSNumber *)z {
    double now = self.context ? self.context.currentTime : 0.0;
    [self.orientationXParam setValueAtTime:x :@(now)];
    [self.orientationYParam setValueAtTime:y :@(now)];
    [self.orientationZParam setValueAtTime:z :@(now)];
    [self applySpatial];
}

@end
