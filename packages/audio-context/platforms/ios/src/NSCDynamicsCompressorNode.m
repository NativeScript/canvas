#import "NSCDynamicsCompressorNode.h"
#import "NSCAudioContext.h"

@implementation NSCDynamicsCompressorNode

- (instancetype)initWithContext:(NSCAudioContext *)context {
    NSCAudioParam *threshold = [[NSCAudioParam alloc] initWithContext:context defaultValue:-24.0];
    NSCAudioParam *knee = [[NSCAudioParam alloc] initWithContext:context defaultValue:30.0];
    NSCAudioParam *ratio = [[NSCAudioParam alloc] initWithContext:context defaultValue:12.0];
    NSCAudioParam *attack = [[NSCAudioParam alloc] initWithContext:context defaultValue:0.003];
    NSCAudioParam *release = [[NSCAudioParam alloc] initWithContext:context defaultValue:0.25];
    if (self = [super initWithContext:context kind:NSCParamEffectKindCompressor params:@[ threshold, knee, ratio, attack, release ] maxDelayTime:0.0]) {
        _thresholdParam = threshold;
        _kneeParam = knee;
        _ratioParam = ratio;
        _attackParam = attack;
        _releaseParam = release;
    }
    return self;
}

- (double)reduction { return [self meteredReduction]; }

@end
