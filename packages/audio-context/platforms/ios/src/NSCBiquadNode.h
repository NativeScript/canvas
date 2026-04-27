#import <Foundation/Foundation.h>
#import "NSCAudioNode.h"
#import "NSCAudioParam.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCBiquadNode : NSCAudioNode

@property (nonatomic, strong) NSCAudioParam *frequencyParam;
@property (nonatomic, strong) NSCAudioParam *qParam;
@property (nonatomic, strong) NSCAudioParam *gainParam;
@property (nonatomic, strong) NSCAudioParam *detuneParam;

- (instancetype)initWithContext:(NSCAudioContext *)context;
- (instancetype)initWithContext:(NSCAudioContext *)context
                           type:(NSString *)type
                      frequency:(double)frequency
                              Q:(double)Q
                           gain:(double)gain
                         detune:(double)detune NS_DESIGNATED_INITIALIZER;

- (void)setType:(NSString *)type;
- (NSString *)getType;
- (void)setParams:(nullable NSString *)type :(NSNumber *)frequency :(NSNumber *)Q :(NSNumber *)gain;

@end

NS_ASSUME_NONNULL_END
