#import "NSCAudioContext.h"

@implementation NSCWaveShaperNode {
    AVAudioMixerNode *_mixer;
    NSData *_curve;
}

- (instancetype)initWithContext:(NSCAudioContext *)context {
    
    AVAudioMixerNode *mixer = [[AVAudioMixerNode alloc] init];
    [context.engine attachNode:mixer];
    if (self = [super initWithContext:context node:mixer]) {
        _mixer = mixer;
        _oversample = @"none";
    }
    return self;
}

- (void)setCurveFromData:(NSData *)floatData { _curve = [floatData copy]; }
- (NSData *)curve { return _curve; }

@end
