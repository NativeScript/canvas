#import <Foundation/Foundation.h>
#import "NSCAudioNode.h"
#import "NSCAudioParam.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCAudioPannerNode : NSCAudioNode

@property (nonatomic, strong) NSCAudioParam *positionXParam;
@property (nonatomic, strong) NSCAudioParam *positionYParam;
@property (nonatomic, strong) NSCAudioParam *positionZParam;
@property (nonatomic, strong) NSCAudioParam *orientationXParam;
@property (nonatomic, strong) NSCAudioParam *orientationYParam;
@property (nonatomic, strong) NSCAudioParam *orientationZParam;

- (instancetype)initWithContext:(NSCAudioContext *)context;
- (instancetype)initWithContextAndParams:(NSCAudioContext *)context
                                        :(NSNumber *)positionX
                                        :(NSNumber *)positionY
                                        :(NSNumber *)positionZ
                                        :(NSNumber *)orientationX
                                        :(NSNumber *)orientationY
                                        :(NSNumber *)orientationZ
                                        :(NSNumber *)pan
                                        :(NSNumber *)distanceModel
                                        :(NSNumber *)panningModel
                                        :(NSNumber *)refDistance
                                        :(NSNumber *)maxDistance
                                        :(NSNumber *)rolloffFactor
                                        :(NSNumber *)coneInnerAngle
                                        :(NSNumber *)coneOuterAngle
                                        :(NSNumber *)coneOuterGain;

- (void)attachSource:(NSCAudioNode *)source output:(nullable NSNumber *)output input:(nullable NSNumber *)input;
- (void)detachSource:(NSCAudioNode *)source;
- (void)disconnectPanner;

- (void)setPan:(NSNumber *)value;
- (void)setDistanceModel:(NSNumber *)value;
- (NSNumber *)getDistanceModel;
- (void)setPanningModel:(NSNumber *)value;
- (NSNumber *)getPanningModel;
- (void)setRefDistance:(NSNumber *)value;
- (NSNumber *)getRefDistance;
- (void)setMaxDistance:(NSNumber *)value;
- (NSNumber *)getMaxDistance;
- (void)setRolloffFactor:(NSNumber *)value;
- (NSNumber *)getRolloffFactor;
- (void)setConeInnerAngle:(NSNumber *)value;
- (NSNumber *)getConeInnerAngle;
- (void)setConeOuterAngle:(NSNumber *)value;
- (NSNumber *)getConeOuterAngle;
- (void)setConeOuterGain:(NSNumber *)value;
- (NSNumber *)getConeOuterGain;
- (void)setPosition:(NSNumber *)x :(NSNumber *)y :(NSNumber *)z;
- (void)setOrientation:(NSNumber *)x :(NSNumber *)y :(NSNumber *)z;

@end

NS_ASSUME_NONNULL_END
