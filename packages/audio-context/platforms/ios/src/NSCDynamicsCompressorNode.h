#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

#import "NSCAudioNode.h"
#import "NSCAudioParam.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCDynamicsCompressorNode : NSCAudioNode

@property (nonatomic, readonly) NSCAudioParam *thresholdParam;
@property (nonatomic, readonly) NSCAudioParam *kneeParam;
@property (nonatomic, readonly) NSCAudioParam *ratioParam;
@property (nonatomic, readonly) NSCAudioParam *attackParam;
@property (nonatomic, readonly) NSCAudioParam *releaseParam;
@property (nonatomic, readonly) NSCAudioParam *reductionParam;

- (instancetype)initWithContext:(NSCAudioContext *)context NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
