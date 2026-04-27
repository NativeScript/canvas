#import <Foundation/Foundation.h>
#import "NSCAudioNode.h"

NS_ASSUME_NONNULL_BEGIN

@interface NSCAudioScheduledSourceNode : NSCAudioNode

@property (nonatomic, copy, nullable) void (^onended)(void);

- (void)start;
- (void)stop;
- (void)startAt:(nullable NSNumber *)when;
- (void)stopAt:(nullable NSNumber *)when;

@end

NS_ASSUME_NONNULL_END
