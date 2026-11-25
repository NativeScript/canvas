#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

#import "NSCChannelMixerNode.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

/// Output N plays input channel N, the input mixed to numberOfOutputs channels discretely.
/// Each output is its own node: AVAudioEngine drops a unit's other outputs when one is connected.
@interface NSCChannelSplitterNode : NSCChannelMixerNode

@property (nonatomic, readonly) NSInteger numberOfOutputs;

- (instancetype)initWithContext:(NSCAudioContext *)context
                numberOfOutputs:(NSInteger)numberOfOutputs;

@end

NS_ASSUME_NONNULL_END
