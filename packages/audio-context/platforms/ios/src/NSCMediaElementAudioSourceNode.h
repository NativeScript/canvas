//
//  NSCMediaElementAudioSourceNode.h
//

#import "NSCAudioNode.h"

#import "NSCAudioParam.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCMediaElementSourceTap;

@interface NSCMediaElementAudioSourceNode : NSCAudioNode

@property (nonatomic, strong, nullable) NSCMediaElementSourceTap *tap;

- (instancetype)initWithContext:(NSCAudioContext *)context node:(AVAudioNode *)node tap:(NSCMediaElementSourceTap *)tap;
- (void)detach;

- (nullable NSCAudioParam *)getPlaybackRateParam;

@end

NS_ASSUME_NONNULL_END
