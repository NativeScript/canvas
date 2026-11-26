#import <Foundation/Foundation.h>
#import <math.h>

#define NSC_WAVE_TABLE_SIZE 4096
#define NSC_WAVE_TABLES 12

NS_ASSUME_NONNULL_BEGIN

@interface NSCPeriodicWave : NSObject

@property (nonatomic, readonly, copy) NSData *real;
@property (nonatomic, readonly, copy) NSData *imag;
@property (nonatomic, readonly) BOOL disableNormalization;

- (instancetype)initWithReal:(NSData *)real
                        imag:(NSData *)imag
          disableNormalization:(BOOL)disableNormalization;
- (instancetype)initWithReal:(NSData *)real
                                imag:(NSData *)imag
          disableNormalization:(BOOL)disableNormalization
                  realByteOffset:(NSInteger)realByteOffset
                  imagByteOffset:(NSInteger)imagByteOffset NS_DESIGNATED_INITIALIZER;
- (instancetype)init NS_UNAVAILABLE;

/// A built-in oscillator type's wave: square, sawtooth or triangle.
+ (nullable instancetype)waveForType:(NSString *)type;
/// Band-limited tables: table j holds harmonics up to 2^j; read on the render thread.
- (const float *_Nonnull const *_Nonnull)tables;

@end

NS_ASSUME_NONNULL_END

/// The wave at `phase` (radians, 0 to 2 pi) with no harmonic above `nyquist / frequency`.
static inline float NSCWaveSample(const float *const *tables, double phase, double frequency, double nyquist) {
    double limit = fabs(frequency) > 0.0 ? nyquist / fabs(frequency) : 1e9;
    int j = limit >= 1.0 ? (int)floor(log2(limit)) : 0;
    j = j < 0 ? 0 : (j >= NSC_WAVE_TABLES ? NSC_WAVE_TABLES - 1 : j);
    const float *t = tables[j];
    double at = phase / (2.0 * M_PI) * NSC_WAVE_TABLE_SIZE;
    int i0 = (int)at;
    double f = at - i0;
    i0 &= NSC_WAVE_TABLE_SIZE - 1;
    int i1 = (i0 + 1) & (NSC_WAVE_TABLE_SIZE - 1);
    return (float)(t[i0] + (t[i1] - t[i0]) * f);
}
