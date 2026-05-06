#import "NSCAudioContext.h"
#import <QuartzCore/QuartzCore.h>

@implementation NSCGainNode {
    dispatch_source_t _automationTimer;
    CFTimeInterval _automationAnchorHostTime;
    double _automationAnchorAudioTime;
    BOOL _automationUsesHostTime;
}

- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node {
    return [self initWithContext:context];
}

- (instancetype)initWithContext:(NSCAudioContext *)context {
    AVAudioMixerNode *m = [[AVAudioMixerNode alloc] init];
    [context.engine attachNode:m];
    if (self = [super initWithContext:context node:m]) {
        self.gainParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:m.outputVolume];
        _automationAnchorHostTime = 0;
        _automationAnchorAudioTime = 0.0;
        _automationUsesHostTime = NO;

        __weak typeof(self) weakSelf = self;
        void (^hook)(NSCAudioParam *) = ^(NSCAudioParam *p) {
            __strong typeof(weakSelf) s = weakSelf;
            if (!s) return;
            [s applyGainAtCurrentTime];
            [s ensureAutomationLink];
        };
        self.gainParam.onScheduleChanged = hook;
        [self applyGainAtCurrentTime];
    }
    return self;
}



- (float)gain {
    return (float)self.gainParam.value;
}

- (void)setGain:(float)gain {
    [self.gainParam setValueAtTime:@(gain) :@((double)(self.context ? self.context.currentTime : 0.0))];
    AVAudioMixerNode *m = (AVAudioMixerNode *)self.avNode;
    m.outputVolume = MAX(0.0f, MIN(1.0f, gain));
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

- (void)applyGainAtCurrentTime {
    NSCAudioParam *p = self.gainParam;
    AVAudioMixerNode *m = (AVAudioMixerNode *)self.avNode;
    if (!p || !m) return;
    double t = [self currentTimeForApply];
    double v = [p valueAtTime:t];
    if (v < 0.0) v = 0.0;
    if (v > 1.0) v = 1.0;
    m.outputVolume = (float)v;
}

- (BOOL)hasFutureAutomation {
    NSCAudioParam *p = self.gainParam;
    if (!p) return NO;
    double t = [self currentTimeForApply];
    return [p hasEventsAfter:t];
}

- (void)ensureAutomationLink {
    if (![NSThread isMainThread]) {
        __weak typeof(self) weakSelf = self;
        dispatch_async(dispatch_get_main_queue(), ^{ [weakSelf ensureAutomationLink]; });
        return;
    }
    if (_automationTimer) return;
    if (![self hasFutureAutomation]) return;
    dispatch_source_t timer = dispatch_source_create(DISPATCH_SOURCE_TYPE_TIMER, 0, 0, dispatch_get_main_queue());
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
        [s applyGainAtCurrentTime];
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

- (void)dealloc {
    [self stopAutomationLink];
}

@end
