#import "NSCAudioContext.h"

static inline NSUInteger NSCPeriodicWaveNormalizeFloatByteOffset(NSInteger byteOffset, NSUInteger length) {
    NSUInteger offset = byteOffset > 0 ? (NSUInteger)byteOffset : 0;
    if (offset > length) offset = length;
    offset -= (offset % sizeof(float));
    return offset;
}

@implementation NSCPeriodicWave

- (instancetype)initWithReal:(NSData *)real imag:(NSData *)imag disableNormalization:(BOOL)disableNormalization {
    return [self initWithReal:real
                         imag:imag
         disableNormalization:disableNormalization
               realByteOffset:0
               imagByteOffset:0];
}

- (instancetype)initWithReal:(NSData *)real
                        imag:(NSData *)imag
        disableNormalization:(BOOL)disableNormalization
              realByteOffset:(NSInteger)realByteOffset
              imagByteOffset:(NSInteger)imagByteOffset {
    if (self = [super init]) {
        NSUInteger realOffset = NSCPeriodicWaveNormalizeFloatByteOffset(realByteOffset, real.length);
        NSUInteger imagOffset = NSCPeriodicWaveNormalizeFloatByteOffset(imagByteOffset, imag.length);
        _real = [real subdataWithRange:NSMakeRange(realOffset, real.length - realOffset)];
        _imag = [imag subdataWithRange:NSMakeRange(imagOffset, imag.length - imagOffset)];
        _disableNormalization = disableNormalization;
    }
    return self;
}

@end
