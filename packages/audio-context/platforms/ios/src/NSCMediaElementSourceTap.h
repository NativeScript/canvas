#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;
@class NSCAudioNode;

@interface NSCMediaElementSourceTap : NSObject

@property (nonatomic, weak, readonly, nullable) NSCAudioContext *context;
@property (nonatomic, strong, readonly, nullable) AVPlayer *player;
@property (nonatomic, strong, readonly, nullable) AVAudioSourceNode *sourceNode;


+ (nullable instancetype)attachToPlayer:(AVPlayer *)player
                                context:(NSCAudioContext *)context;

- (void)detach;

@end

NS_ASSUME_NONNULL_END
