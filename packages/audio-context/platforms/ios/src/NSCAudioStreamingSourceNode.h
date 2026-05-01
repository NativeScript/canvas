// NSCAudioStreamingSourceNode.h

#import <Foundation/Foundation.h>
#import "NSCAudioScheduledSourceNode.h"

NS_ASSUME_NONNULL_BEGIN

@interface NSCAudioStreamingSourceNode : NSCAudioScheduledSourceNode

- (instancetype)initWithContext:(NSCAudioContext *)context NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

- (BOOL)appendData:(NSData *)data error:(NSError * _Nullable * _Nullable)error;
- (void)endStream;

@end

NS_ASSUME_NONNULL_END
