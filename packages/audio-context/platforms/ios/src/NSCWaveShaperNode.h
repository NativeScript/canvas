#import <Foundation/Foundation.h>
#import "NSCProcessorNode.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCWaveShaperNode : NSCProcessorNode

@property (nonatomic, copy) NSString *oversample; // "none" | "2x" | "4x"

- (instancetype)initWithContext:(NSCAudioContext *)context;

- (void)setCurveFromData:(nullable NSData *)floatData;
- (void)setCurveFromDataWithByteOffset:(nullable NSData *)floatData :(NSInteger)byteOffset;
- (nullable NSData *)curve;

@end

NS_ASSUME_NONNULL_END
