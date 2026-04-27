#import "NSCAudioContext.h"

@implementation NSCGainNode

- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node {
    return [self initWithContext:context];
}

- (instancetype)initWithContext:(NSCAudioContext *)context {
    AVAudioMixerNode *m = [[AVAudioMixerNode alloc] init];
    [context.engine attachNode:m];
    if (self = [super initWithContext:context node:m]) {
        self.gainParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:m.outputVolume];
    }
    return self;
}



- (float)gain {
    return (float)self.gainParam.value;
}

- (void)setGain:(float)gain {
    [self.gainParam setValueAtTime:@(gain) :@((double)(self.context ? self.context.currentTime : 0.0))];
    AVAudioMixerNode *m = (AVAudioMixerNode *)self.avNode;
    m.outputVolume = gain;
}

@end
