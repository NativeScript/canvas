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
    d.feedback = 0.0; d.lowPassCutoff = 15000.0; d.wetDryMix = 100.0;
    [context.engine attachNode:d];
    if (self = [super initWithContext:context node:d]) {
        _delay = d;
        _maxDelayTime = maxDelayTime > 0 ? maxDelayTime : 2.0;
        _delayTimeParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:clamped];
    }
    return self;
}


- (void)setDelayTimeScalar:(double)v {
    _delay.delayTime = MAX(0.0, MIN(v, _maxDelayTime));
}

@end
