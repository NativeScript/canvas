#import <QuartzCore/QuartzCore.h>
#import "NSCAudioContext.h"

@implementation NSCStereoPannerNode {
    dispatch_source_t _automationTimer;
    CFTimeInterval _automationAnchorHostTime;
    double _automationAnchorAudioTime;
    BOOL _automationUsesHostTime;
}

- (instancetype)initWithContext:(NSCAudioContext *)context pan:(double)pan {
    AVAudioMixerNode *mixer = [[AVAudioMixerNode alloc] init];
    [context.engine attachNode:mixer];
    if (self = [super initWithContext:context node:mixer]) {
        mixer.pan = (float)MAX(-1.0, MIN(1.0, pan));
        _panParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:pan];
        _automationAnchorHostTime = 0;
        _automationAnchorAudioTime = 0.0;
        _automationUsesHostTime = NO;

        __weak typeof(self) weakSelf = self;
        void (^hook)(NSCAudioParam *) = ^(NSCAudioParam *p) {
            __strong typeof(weakSelf) s = weakSelf;
            if (!s) return;
            double now = s.context ? s.context.currentTime : 0.0;
            double v = [p valueAtTime:now];
            if (v < -1.0) v = -1.0;
            if (v > 1.0) v = 1.0;
            ((AVAudioMixerNode *)s.avNode).pan = (float)v;
            [s ensureAutomationLink];
        };
        _panParam.onScheduleChanged = hook;
        double init = _panParam.value;
        mixer.pan = (float)MAX(-1.0, MIN(1.0, init));
    }
    return self;
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
    return [_panParam hasEventsAfter:t];
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
        double t = [s currentTimeForApply];
        double v = [s.panParam valueAtTime:t];
        if (v < -1.0) v = -1.0;
        if (v > 1.0) v = 1.0;
        ((AVAudioMixerNode *)s.avNode).pan = (float)v;
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

@end
