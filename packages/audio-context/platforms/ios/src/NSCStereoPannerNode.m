#import "NSCAudioContext.h"

@implementation NSCStereoPannerNode

- (instancetype)initWithContext:(NSCAudioContext *)context pan:(double)pan {
    AVAudioMixerNode *mixer = [[AVAudioMixerNode alloc] init];
    [context.engine attachNode:mixer];
    if (self = [super initWithContext:context node:mixer]) {
        mixer.pan = (float)MAX(-1.0, MIN(1.0, pan));
        _panParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:pan];
    }
    return self;
}

@end
