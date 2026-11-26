#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import "NSCAudioNode.h"
#import "NSCAudioParam.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

typedef NS_ENUM(NSInteger, NSCParamEffectKind) {
    NSCParamEffectKindGain = 0,
    NSCParamEffectKindDelay = 1,
    /// params: frequency, Q, gain, detune.
    NSCParamEffectKindBiquad = 2,
};

/// A node whose inputs sum in a mixer feeding an audio unit that applies its params per frame:
/// a gain, a delay line `maxDelayTime` long, or a biquad filter.
@interface NSCParamEffectNode : NSCAudioNode

- (instancetype)initWithContext:(NSCAudioContext *)context
                           kind:(NSCParamEffectKind)kind
                         params:(NSArray<NSCAudioParam *> *)params
                   maxDelayTime:(double)maxDelayTime NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context
                           kind:(NSCParamEffectKind)kind
                          param:(NSCAudioParam *)param
                   maxDelayTime:(double)maxDelayTime;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

/// A delay only: moves the delay's input onto a writer so it can sit in a cycle; the delay then
/// reads only earlier render calls, clamping delayTime to at least one call. YES once split.
- (BOOL)breakCycle;
/// Undoes breakCycle once the delay is no longer on a cycle.
- (void)healCycle;
@property (nonatomic, readonly) BOOL isSplit;

/// The biquad's filter type, read by the render thread.
- (void)setFilterType:(NSInteger)type;

@end

/// Makes `param`'s input: a mixer whose mono sum a sink unit writes into the param each render.
/// The sink outputs silence; connect it where it renders before the param is read.
NSCAudioNode *_Nullable NSCMakeParamInput(NSCAudioContext *context, NSCAudioParam *param);

NS_ASSUME_NONNULL_END
