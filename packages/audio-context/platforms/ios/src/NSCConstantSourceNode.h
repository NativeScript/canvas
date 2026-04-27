#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import "NSCAudioScheduledSourceNode.h"
#import "NSCAudioParam.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCConstantSourceNode : NSCAudioScheduledSourceNode

@property (nonatomic, readonly) NSCAudioParam *offsetParam;

- (instancetype)initWithContext:(NSCAudioContext *)context offset:(double)offset NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
