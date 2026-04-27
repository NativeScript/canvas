#import "NSCAudioContext.h"

@implementation NSCConvolverNode {
    AVAudioUnitReverb *_reverb;
}

- (instancetype)initWithContext:(NSCAudioContext *)context {
    AVAudioUnitReverb *r = [[AVAudioUnitReverb alloc] init];
    [r loadFactoryPreset:AVAudioUnitReverbPresetMediumHall];
    r.wetDryMix = 50.0;
    [context.engine attachNode:r];
    if (self = [super initWithContext:context node:r]) {
        _reverb = r;
        _normalize = YES;
    }
    return self;
}

@end
