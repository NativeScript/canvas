#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

#import "NSCAudioNode.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCChannelSplitterNode : NSCAudioNode

@property (nonatomic, readonly) NSInteger numberOfOutputs;

- (instancetype)initWithContext:(NSCAudioContext *)context
                numberOfOutputs:(NSInteger)numberOfOutputs NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
