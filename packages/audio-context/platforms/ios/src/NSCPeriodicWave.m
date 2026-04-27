#import "NSCAudioContext.h"

@implementation NSCPeriodicWave

- (instancetype)initWithReal:(NSData *)real imag:(NSData *)imag disableNormalization:(BOOL)disableNormalization {
    if (self = [super init]) {
        _real = [real copy];
        _imag = [imag copy];
        _disableNormalization = disableNormalization;
    }
    return self;
}

@end
