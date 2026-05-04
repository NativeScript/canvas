#import <QuartzCore/QuartzCore.h>
#import <AudioToolbox/AudioToolbox.h>

#import "NSCAudioContext.h"

@implementation NSCDynamicsCompressorNode {
    AVAudioUnitEffect *_compressor;
    dispatch_source_t _automationTimer;
    CFTimeInterval _automationAnchorHostTime;
    double _automationAnchorAudioTime;
    BOOL _automationUsesHostTime;
}

- (instancetype)initWithContext:(NSCAudioContext *)context {
    AudioComponentDescription description;
    description.componentType = kAudioUnitType_Effect;
    description.componentSubType = kAudioUnitSubType_DynamicsProcessor;
    description.componentManufacturer = kAudioUnitManufacturer_Apple;
    description.componentFlags = 0;
    description.componentFlagsMask = 0;

    AVAudioUnitEffect *compressor = [[AVAudioUnitEffect alloc] initWithAudioComponentDescription:description];
    AVAudioNode *node = compressor;
    if (compressor) {
        [context.engine attachNode:compressor];
    } else {
        AVAudioMixerNode *fallback = [[AVAudioMixerNode alloc] init];
        [context.engine attachNode:fallback];
        node = fallback;
    }

    if (self = [super initWithContext:context node:node]) {
        _compressor = compressor;

        _thresholdParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:-24.0];
        _kneeParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:30.0];
        _ratioParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:12.0];
        _attackParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:0.003];
        _releaseParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:0.25];
        _reductionParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:0.0];

        _automationAnchorHostTime = 0;
        _automationAnchorAudioTime = 0.0;
        _automationUsesHostTime = NO;

        [self installAudioParamHooks];
        [self applyCompressorParams];
    }

    return self;
}

- (BOOL)setCompressorParam:(AudioUnitParameterID)param value:(AudioUnitParameterValue)value {
    if (!_compressor) return NO;
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
    AudioUnit unit = _compressor.audioUnit;
#pragma clang diagnostic pop
    if (!unit) return NO;
    OSStatus status = AudioUnitSetParameter(unit, param, kAudioUnitScope_Global, 0, value, 0);
    return status == noErr;
}

- (BOOL)getCompressorParam:(AudioUnitParameterID)param value:(AudioUnitParameterValue *)value {
    if (!_compressor || !value) return NO;
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
    AudioUnit unit = _compressor.audioUnit;
#pragma clang diagnostic pop
    if (!unit) return NO;
    OSStatus status = AudioUnitGetParameter(unit, param, kAudioUnitScope_Global, 0, value);
    return status == noErr;
}

- (void)dealloc {
    [self stopAutomationLink];
    [self clearAudioParamHooks];
}

- (void)installAudioParamHooks {
    __weak typeof(self) weakSelf = self;
    void (^hook)(NSCAudioParam *) = ^(NSCAudioParam *p) {
        __strong typeof(weakSelf) s = weakSelf;
        if (!s) return;
        [s applyCompressorParams];
        [s ensureAutomationLink];
    };

    _thresholdParam.onScheduleChanged = hook;
    _kneeParam.onScheduleChanged = hook;
    _ratioParam.onScheduleChanged = hook;
    _attackParam.onScheduleChanged = hook;
    _releaseParam.onScheduleChanged = hook;
}

- (void)clearAudioParamHooks {
    _thresholdParam.onScheduleChanged = nil;
    _kneeParam.onScheduleChanged = nil;
    _ratioParam.onScheduleChanged = nil;
    _attackParam.onScheduleChanged = nil;
    _releaseParam.onScheduleChanged = nil;
}

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

- (BOOL)hasFutureAutomation {
    double t = [self currentTimeForApply];
    return [_thresholdParam hasEventsAfter:t]
        || [_kneeParam hasEventsAfter:t]
        || [_ratioParam hasEventsAfter:t]
        || [_attackParam hasEventsAfter:t]
        || [_releaseParam hasEventsAfter:t];
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
        [s applyCompressorParams];
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

- (void)applyCompressorParams {
    if (!_compressor) return;

    double t = [self currentTimeForApply];

    double threshold = [_thresholdParam valueAtTime:t];
    double knee = [_kneeParam valueAtTime:t];
    double ratio = [_ratioParam valueAtTime:t];
    double attack = [_attackParam valueAtTime:t];
    double release = [_releaseParam valueAtTime:t];

    if (threshold < -100.0) threshold = -100.0;
    if (threshold > 0.0) threshold = 0.0;

    if (knee < 0.0) knee = 0.0;
    if (knee > 40.0) knee = 40.0;

    if (ratio < 1.0) ratio = 1.0;
    if (ratio > 50.0) ratio = 50.0;

    if (attack < 0.0001) attack = 0.0001;
    if (attack > 1.0) attack = 1.0;

    if (release < 0.001) release = 0.001;
    if (release > 5.0) release = 5.0;

    double effectiveHeadroom = knee;
    if (ratio > 1.0) {
        double normalized = (ratio - 1.0) / 19.0;
        if (normalized < 0.0) normalized = 0.0;
        if (normalized > 1.0) normalized = 1.0;
        effectiveHeadroom = knee * (1.0 - (0.7 * normalized));
        if (effectiveHeadroom < 1.0) effectiveHeadroom = 1.0;
    }

    [self setCompressorParam:kDynamicsProcessorParam_Threshold value:(AudioUnitParameterValue)threshold];
    [self setCompressorParam:kDynamicsProcessorParam_HeadRoom value:(AudioUnitParameterValue)effectiveHeadroom];
    [self setCompressorParam:kDynamicsProcessorParam_ExpansionRatio value:(AudioUnitParameterValue)1.0f];
    [self setCompressorParam:kDynamicsProcessorParam_ExpansionThreshold value:(AudioUnitParameterValue)-120.0f];
    [self setCompressorParam:kDynamicsProcessorParam_AttackTime value:(AudioUnitParameterValue)attack];
    [self setCompressorParam:kDynamicsProcessorParam_ReleaseTime value:(AudioUnitParameterValue)release];

    AudioUnitParameterValue compressionAmount = 0;
    if ([self getCompressorParam:kDynamicsProcessorParam_CompressionAmount value:&compressionAmount]) {
        double reductionDb = -(double)compressionAmount;
        [_reductionParam setValueAtTime:@(reductionDb) :@(t)];
    }
}

@end
