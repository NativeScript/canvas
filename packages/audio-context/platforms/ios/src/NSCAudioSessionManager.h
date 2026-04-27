#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

NS_ASSUME_NONNULL_BEGIN

@interface NSCAudioSessionManager : NSObject

+ (instancetype)sharedManager;

- (void)registerContext:(id)context sampleRate:(double)sampleRate latencyHint:(double)latencyHint;
- (void)unregisterContext:(id)context;

- (void)contextDidChangeActiveState:(id)context;


FOUNDATION_EXPORT NSNotificationName const NSCAudioSessionManagerDidUpdateNotification;
FOUNDATION_EXPORT NSString * const NSCAudioSessionManagerSampleRateKey;
FOUNDATION_EXPORT NSString * const NSCAudioSessionManagerLatencyKey;

@end

NS_ASSUME_NONNULL_END
