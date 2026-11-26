#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import "NSCProcessorNode.h"
#import "NSCAudioParam.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCStereoPannerNode : NSCProcessorNode

@property (nonatomic, readonly) NSCAudioParam *panParam;

- (instancetype)initWithContext:(NSCAudioContext *)context pan:(double)pan;

@end

NS_ASSUME_NONNULL_END
