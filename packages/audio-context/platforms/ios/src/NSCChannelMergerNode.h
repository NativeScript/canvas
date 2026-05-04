#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

#import "NSCAudioNode.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCChannelMergerNode : NSCAudioNode

@property (nonatomic, readonly) NSInteger numberOfInputs;

- (instancetype)initWithContext:(NSCAudioContext *)context
                 numberOfInputs:(NSInteger)numberOfInputs NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
