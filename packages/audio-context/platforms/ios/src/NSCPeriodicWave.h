#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

@interface NSCPeriodicWave : NSObject

@property (nonatomic, readonly, copy) NSData *real;
@property (nonatomic, readonly, copy) NSData *imag;
@property (nonatomic, readonly) BOOL disableNormalization;

- (instancetype)initWithReal:(NSData *)real
                        imag:(NSData *)imag
        disableNormalization:(BOOL)disableNormalization NS_DESIGNATED_INITIALIZER;
- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
