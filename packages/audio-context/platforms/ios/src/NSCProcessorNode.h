#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import <stdatomic.h>
#import "NSCAudioNode.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;
@class NSCChannelMixerNode;

/// Renders `frames` from context frame `frame`: `in` holds the input's own channels (its
/// computedNumberOfChannels), `out` has room for `outChannels`. Returns the channels it wrote.
typedef UInt32 (^NSCProcessBlock)(int64_t frame, UInt32 frames, float *const _Nonnull *_Nonnull in, UInt32 inChannels, float *const _Nonnull *_Nonnull out, UInt32 outChannels);

/// A node whose inputs mix per its channel config into a block run on the render thread.
@interface NSCProcessorNode : NSCAudioNode

- (nullable instancetype)initWithContext:(NSCAudioContext *)context outputChannels:(NSInteger)outputChannels NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

@property (nonatomic, readonly) NSCChannelMixerNode *port;
@property (nonatomic, readonly) NSInteger outputChannels;

/// Swaps the block the render thread runs.
- (void)setProcess:(NSCProcessBlock)process;
/// Renders even with nothing connected to the output, as an AnalyserNode does.
- (void)keepPulled;

@end

NS_ASSUME_NONNULL_END
