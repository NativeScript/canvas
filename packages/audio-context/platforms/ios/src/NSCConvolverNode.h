#import <Foundation/Foundation.h>
#import "NSCProcessorNode.h"
#import "NSCAudioBuffer.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCConvolverNode : NSCProcessorNode

@property (nonatomic, strong, nullable) NSCAudioBuffer *buffer;
@property (nonatomic) BOOL normalize;

- (instancetype)initWithContext:(NSCAudioContext *)context;

@end

NS_ASSUME_NONNULL_END
