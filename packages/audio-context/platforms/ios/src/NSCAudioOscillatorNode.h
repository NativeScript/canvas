#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import "NSCAudioScheduledSourceNode.h"
#import "NSCAudioParam.h"
#import "NSCPeriodicWave.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCAudioOscillatorNode : NSCAudioScheduledSourceNode

@property (nonatomic, copy) NSString *type;
@property (nonatomic, readonly) NSCAudioParam *frequencyParam;
@property (nonatomic) double frequency;

- (instancetype)initWithContext:(NSCAudioContext *)context;
- (instancetype)initWithContext:(NSCAudioContext *)context
                           type:(NSString *)type
                      frequency:(double)frequency NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

- (void)setPeriodicWave:(NSCPeriodicWave *)wave;

@end

NS_ASSUME_NONNULL_END
