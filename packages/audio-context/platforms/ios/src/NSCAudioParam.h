#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;
@class NSCAudioParam;

@interface NSCAudioParam : NSObject

@property (nonatomic, copy) NSString *automationRate;
@property (nonatomic, readonly) double value;

@property (nonatomic, copy, nullable) void (^onScheduleChanged)(NSCAudioParam *param);

- (instancetype)initWithContext:(nullable NSCAudioContext *)context
                   defaultValue:(double)defaultValue NS_DESIGNATED_INITIALIZER;
- (instancetype)init NS_UNAVAILABLE;

- (void)setValue:(NSNumber *)value;
- (void)setValueAtTime:(NSNumber *)value :(NSNumber *)time;
- (void)linearRampToValueAtTime:(NSNumber *)value :(NSNumber *)time;
- (void)cancelScheduledValues:(NSNumber *)time;
- (void)cancelAndHoldAtTime:(NSNumber *)heldValue :(NSNumber *)time;
- (NSArray<NSNumber *> *)getValuesForRange:(double)startTime :(double)sampleRate :(NSInteger)frameCount;

- (BOOL)fillValuesForRange:(double)startTime
                sampleRate:(double)sampleRate
                frameCount:(NSInteger)frameCount
                      into:(double *)outValues;

- (double)currentScalarValue;
- (double)valueAtTime:(double)time;
- (BOOL)hasEventsAfter:(double)time;

@end

NS_ASSUME_NONNULL_END
