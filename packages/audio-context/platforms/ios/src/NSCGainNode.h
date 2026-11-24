#import <Foundation/Foundation.h>
#import "NSCParamEffectNode.h"
#import "NSCAudioParam.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

/// Multiplies its input by `gainParam`, evaluated per frame.
@interface NSCGainNode : NSCParamEffectNode

@property (nonatomic, strong, readonly) NSCAudioParam *gainParam;
@property (nonatomic) float gain;

- (instancetype)initWithContext:(NSCAudioContext *)context;

@end

NS_ASSUME_NONNULL_END
