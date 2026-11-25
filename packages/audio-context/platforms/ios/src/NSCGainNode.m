#import "NSCAudioContext.h"

@implementation NSCGainNode

- (instancetype)initWithContext:(NSCAudioContext *)context {
    NSCAudioParam *param = [[NSCAudioParam alloc] initWithContext:context defaultValue:1.0];
    if (self = [super initWithContext:context kind:NSCParamEffectKindGain param:param maxDelayTime:0.0]) {
        _gainParam = param;
    }
    return self;
}

- (float)gain {
    return (float)self.gainParam.value;
}

- (void)setGain:(float)gain {
    [self.gainParam setValueAtTime:@(gain) :@((double)(self.context ? self.context.currentTime : 0.0))];
}

@end
