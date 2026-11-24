#import "NSCAudioContext.h"

// Web Audio's limit for maxDelayTime.
static const double NSCMaxDelaySeconds = 180.0;

@implementation NSCDelayNode

- (instancetype)initWithContext:(NSCAudioContext *)context
                      delayTime:(double)delayTime
                   maxDelayTime:(double)maxDelayTime {
    double max = isfinite(maxDelayTime) && maxDelayTime > 0.0 ? MIN(maxDelayTime, NSCMaxDelaySeconds) : 1.0;
    double initial = isfinite(delayTime) ? MAX(0.0, MIN(delayTime, max)) : 0.0;
    NSCAudioParam *param = [[NSCAudioParam alloc] initWithContext:context defaultValue:initial];
    if (self = [super initWithContext:context kind:NSCParamEffectKindDelay param:param maxDelayTime:max]) {
        _delayTimeParam = param;
        _maxDelayTime = max;
    }
    return self;
}

@end
