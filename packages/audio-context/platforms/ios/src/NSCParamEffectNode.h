#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import "NSCAudioNode.h"
#import "NSCAudioParam.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

typedef NS_ENUM(NSInteger, NSCParamEffectKind) {
    NSCParamEffectKindGain = 0,
    NSCParamEffectKindDelay = 1,
};

/// A node whose inputs sum in a mixer feeding an audio unit that applies `param` per frame:
/// a gain, or a delay line `maxDelayTime` long.
@interface NSCParamEffectNode : NSCAudioNode

- (instancetype)initWithContext:(NSCAudioContext *)context
                           kind:(NSCParamEffectKind)kind
                          param:(NSCAudioParam *)param
                   maxDelayTime:(double)maxDelayTime NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
