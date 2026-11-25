#import <Foundation/Foundation.h>
#import <stdatomic.h>

NS_ASSUME_NONNULL_BEGIN

#define NSC_PARAM_INPUT_FRAMES 4096

/// The mono sum of the audio connected to a param for one render call, from context frame `frame`.
typedef struct {
    _Atomic double frame;
    _Atomic uint32_t frames;
    float values[NSC_PARAM_INPUT_FRAMES];
} NSCParamInputBlock;

@class NSCAudioContext;
@class NSCAudioParam;
@class NSCAudioNode;

@interface NSCAudioParam : NSObject

@property (nonatomic, copy) NSString *automationRate;
@property (nonatomic, readonly) double value;
/// The node that applies this param per frame; audio connected to the param sums in there.
@property (nonatomic, weak, nullable) NSCAudioNode *owner;

@property (nonatomic, copy, nullable) void (^onScheduleChanged)(NSCAudioParam *param);

/// Where audio connected to this param goes, and the sink that writes its sum into `inputBlock`.
@property (nonatomic, strong, nullable) NSCAudioNode *inputNode;
@property (nonatomic, strong, nullable) id inputSink;
/// Shared with the sink; made on first use. Values read for frames it covers include it.
- (NSMutableData *)inputData;

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
/// Values for frames `startFrame`..., frame i at (startFrame + i) / sampleRate.
- (BOOL)fillValuesFromFrame:(double)startFrame
                 sampleRate:(double)sampleRate
                 frameCount:(NSInteger)frameCount
                       into:(double *)outValues;

- (double)currentScalarValue;
- (double)valueAtTime:(double)time;
- (BOOL)hasEventsAfter:(double)time;

@end

NS_ASSUME_NONNULL_END
