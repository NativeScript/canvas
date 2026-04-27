#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import "NSCAudioScheduledSourceNode.h"
#import "NSCAudioBuffer.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCAudioBufferSourceNode : NSCAudioScheduledSourceNode

@property (nonatomic, strong, nullable) NSCAudioBuffer *buffer;
@property (nonatomic) BOOL loop;

- (instancetype)initWithContext:(NSCAudioContext *)context
                         buffer:(nullable NSCAudioBuffer *)buffer NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
