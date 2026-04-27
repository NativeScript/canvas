#import <Foundation/Foundation.h>
#import "NSCAudioNode.h"
#import "NSCAudioParam.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCGainNode : NSCAudioNode

@property (nonatomic, strong) NSCAudioParam *gainParam;
@property (nonatomic) float gain;

- (instancetype)initWithContext:(NSCAudioContext *)context;

@end

NS_ASSUME_NONNULL_END
