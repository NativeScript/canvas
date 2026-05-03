#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import "NSCAudioParam.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;
@class NSCAudioNode;

@interface NSCMediaElementSourceTap : NSObject

@property (nonatomic, weak, readonly, nullable) NSCAudioContext *context;
@property (nonatomic, strong, readonly, nullable) AVPlayer *player;
@property (nonatomic, strong, readonly, nullable) AVAudioSourceNode *sourceNode;
@property (nonatomic, strong, readonly, nullable) AVAudioNode *outputNode;


+ (nullable instancetype)attachToPlayer:(AVPlayer *)player
                                context:(NSCAudioContext *)context;

- (void)detach;

- (nullable NSCAudioParam *)getPlaybackRateParam;

@end

NS_ASSUME_NONNULL_END
