#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

#import "NSCChannelMixerNode.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

/// Input N, down-mixed to mono, plays as output channel N.
@interface NSCChannelMergerNode : NSCChannelMixerNode

@property (nonatomic, readonly) NSInteger numberOfInputs;

- (instancetype)initWithContext:(NSCAudioContext *)context
                 numberOfInputs:(NSInteger)numberOfInputs;

@end

NS_ASSUME_NONNULL_END
