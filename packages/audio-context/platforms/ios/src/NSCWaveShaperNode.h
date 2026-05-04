#import <Foundation/Foundation.h>
#import "NSCAudioNode.h"

NS_ASSUME_NONNULL_BEGIN

@class NSCAudioContext;

@interface NSCWaveShaperNode : NSCAudioNode

@property (nonatomic, copy) NSString *oversample; // "none" | "2x" | "4x"

- (instancetype)initWithContext:(NSCAudioContext *)context NS_DESIGNATED_INITIALIZER;
- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node NS_UNAVAILABLE;

- (void)setCurveFromData:(nullable NSData *)floatData;
- (void)setCurveFromDataWithByteOffset:(nullable NSData *)floatData :(NSInteger)byteOffset;
- (nullable NSData *)curve;

@end

NS_ASSUME_NONNULL_END
