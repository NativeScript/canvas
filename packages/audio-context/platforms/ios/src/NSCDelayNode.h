#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import "NSCParamEffectNode.h"
#import "NSCAudioParam.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

/// Delays its input by `delayTimeParam` seconds, evaluated per frame, up to `maxDelayTime`.
@interface NSCDelayNode : NSCParamEffectNode

@property (nonatomic, readonly) NSCAudioParam *delayTimeParam;
@property (nonatomic, readonly) double maxDelayTime;

- (instancetype)initWithContext:(NSCAudioContext *)context
                      delayTime:(double)delayTime
                   maxDelayTime:(double)maxDelayTime;

@end

NS_ASSUME_NONNULL_END
