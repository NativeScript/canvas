#import <Foundation/Foundation.h>
#import "NSCParamEffectNode.h"
#import "NSCAudioParam.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

/// Web Audio's biquad filter, its params applied per frame.
@interface NSCBiquadNode : NSCParamEffectNode

@property (nonatomic, readonly) NSCAudioParam *frequencyParam;
@property (nonatomic, readonly) NSCAudioParam *qParam;
@property (nonatomic, readonly) NSCAudioParam *gainParam;
@property (nonatomic, readonly) NSCAudioParam *detuneParam;

- (instancetype)initWithContext:(NSCAudioContext *)context;
- (instancetype)initWithContext:(NSCAudioContext *)context
                           type:(NSString *)type
                      frequency:(double)frequency
                              Q:(double)Q
                           gain:(double)gain
                         detune:(double)detune;

- (void)setType:(NSString *)type;
- (NSString *)getType;
- (void)setParams:(nullable NSString *)type :(NSNumber *)frequency :(NSNumber *)Q :(NSNumber *)gain;

@end

NS_ASSUME_NONNULL_END
