#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import "NSCAudioNode.h"
#import "NSCAudioParam.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCDelayNode : NSCAudioNode

@property (nonatomic, readonly) NSCAudioParam *delayTimeParam;
@property (nonatomic, readonly) double maxDelayTime;

- (instancetype)initWithContext:(NSCAudioContext *)context
                      delayTime:(double)delayTime
                   maxDelayTime:(double)maxDelayTime NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
