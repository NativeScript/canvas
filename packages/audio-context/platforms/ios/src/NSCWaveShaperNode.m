#import "NSCAudioContext.h"

static inline NSUInteger NSCWaveShaperNormalizeFloatByteOffset(NSInteger byteOffset, NSUInteger length) {
    NSUInteger offset = byteOffset > 0 ? (NSUInteger)byteOffset : 0;
    if (offset > length) offset = length;
    offset -= (offset % sizeof(float));
    return offset;
}

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

- (void)setCurveFromData:(NSData *)floatData {
    [self setCurveFromDataWithByteOffset:floatData :0];
}

- (void)setCurveFromDataWithByteOffset:(NSData *)floatData :(NSInteger)byteOffset {
    if (!floatData) {
        _curve = nil;
        return;
    }
    NSUInteger offsetBytes = NSCWaveShaperNormalizeFloatByteOffset(byteOffset, floatData.length);
    _curve = [floatData subdataWithRange:NSMakeRange(offsetBytes, floatData.length - offsetBytes)];
}

- (NSData *)curve { return _curve; }

@end
