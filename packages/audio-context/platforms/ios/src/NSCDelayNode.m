#import "NSCAudioContext.h"

@implementation NSCDelayNode {
    AVAudioUnitDelay *_delay;
}

- (instancetype)initWithContext:(NSCAudioContext *)context
                                            delayTime:(double)delayTime
                                        maxDelayTime:(double)maxDelayTime {
    AVAudioUnitDelay *d = [[AVAudioUnitDelay alloc] init];
    double clamped = MAX(0.0, MIN(delayTime, maxDelayTime > 0 ? maxDelayTime : 2.0));
    d.delayTime = clamped;
    // Open filter: a DelayNode only delays.
    double nyquist = context.sampleRate > 0 ? context.sampleRate / 2.0 : 22050.0;
    d.feedback = 0.0; d.lowPassCutoff = nyquist; d.wetDryMix = 100.0;
    [context.engine attachNode:d];
    if (self = [super initWithContext:context node:d]) {
        _delay = d;
        _maxDelayTime = maxDelayTime > 0 ? maxDelayTime : 2.0;
        _delayTimeParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:clamped];
        __weak typeof(self) weakSelf = self;
        _delayTimeParam.onScheduleChanged = ^(NSCAudioParam *p) {
            [weakSelf applyAutomation];
        };
    }
    return self;
}


- (void)setDelayTimeScalar:(double)v {
    _delay.delayTime = MAX(0.0, MIN(v, _maxDelayTime));
}

- (void)applyAutomation {
    NSCAudioContext *ctx = self.context;
    [self setDelayTimeScalar:[_delayTimeParam valueAtTime:ctx ? ctx.currentTime : 0.0]];
}

@end
