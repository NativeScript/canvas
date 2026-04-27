#import <Foundation/Foundation.h>
#import "NSCAudioNode.h"
#import "NSCAudioBuffer.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCConvolverNode : NSCAudioNode

@property (nonatomic, strong, nullable) NSCAudioBuffer *buffer;
@property (nonatomic) BOOL normalize;

- (instancetype)initWithContext:(NSCAudioContext *)context NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
