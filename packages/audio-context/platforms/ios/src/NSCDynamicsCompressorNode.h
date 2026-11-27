#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

#import "NSCParamEffectNode.h"
#import "NSCAudioParam.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

/// Web Audio's compressor: Chromium's kernel with 6 ms of look-ahead and makeup gain.
@interface NSCDynamicsCompressorNode : NSCParamEffectNode

@property (nonatomic, readonly) NSCAudioParam *thresholdParam;
@property (nonatomic, readonly) NSCAudioParam *kneeParam;
@property (nonatomic, readonly) NSCAudioParam *ratioParam;
@property (nonatomic, readonly) NSCAudioParam *attackParam;
@property (nonatomic, readonly) NSCAudioParam *releaseParam;
/// The gain reduction in dB the last render applied.
@property (nonatomic, readonly) double reduction;

- (instancetype)initWithContext:(NSCAudioContext *)context;

@end

NS_ASSUME_NONNULL_END
