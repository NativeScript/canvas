#import "NSCAudioContext.h"

static inline NSUInteger NSCPeriodicWaveNormalizeFloatByteOffset(NSInteger byteOffset, NSUInteger length) {
    NSUInteger offset = byteOffset > 0 ? (NSUInteger)byteOffset : 0;
    if (offset > length) offset = length;
    offset -= (offset % sizeof(float));
    return offset;
}

#import <Accelerate/Accelerate.h>

@implementation NSCPeriodicWave {
    float *_tables[NSC_WAVE_TABLES];
    dispatch_once_t _built;
}

+ (instancetype)waveForType:(NSString *)type {
    static NSMutableDictionary<NSString *, NSCPeriodicWave *> *cache;
    static dispatch_once_t once;
    dispatch_once(&once, ^{ cache = [NSMutableDictionary dictionary]; });
    @synchronized (cache) {
        NSCPeriodicWave *wave = cache[type];
        if (wave) return wave;
        // The Fourier series the spec gives for each built-in type.
        NSUInteger n = NSC_WAVE_TABLE_SIZE / 2;
        NSMutableData *real = [NSMutableData dataWithLength:n * sizeof(float)];
        NSMutableData *imag = [NSMutableData dataWithLength:n * sizeof(float)];
        float *b = (float *)imag.mutableBytes;
        for (NSUInteger k = 1; k < n; k++) {
            double pk = M_PI * k;
            if ([type isEqualToString:@"square"]) b[k] = (float)(k % 2 ? 4.0 / pk : 0.0);
            else if ([type isEqualToString:@"sawtooth"]) b[k] = (float)((k % 2 ? 2.0 : -2.0) / pk);
            else if ([type isEqualToString:@"triangle"]) b[k] = (float)(8.0 * sin(pk / 2.0) / (pk * pk));
            else return nil;
        }
        wave = [[NSCPeriodicWave alloc] initWithReal:real imag:imag disableNormalization:NO];
        cache[type] = wave;
        return wave;
    }
}

- (const float *const *)tables {
    dispatch_once(&_built, ^{ [self buildTables]; });
    return (const float *const *)_tables;
}

// Inverse DFTs of the coefficients up to each table's harmonic limit, scaled by the full table's peak.
- (void)buildTables {
    NSUInteger count = MIN(_real.length, _imag.length) / sizeof(float);
    const float *a = (const float *)_real.bytes, *b = (const float *)_imag.bytes;
    NSUInteger N = NSC_WAVE_TABLE_SIZE, half = N / 2;
    vDSP_DFT_Setup setup = vDSP_DFT_zrop_CreateSetup(NULL, N, vDSP_DFT_INVERSE);
    float *re = calloc(half, sizeof(float)), *im = calloc(half, sizeof(float));
    float *ore = calloc(half, sizeof(float)), *oim = calloc(half, sizeof(float));
    float scale = 1.0f;
    for (int pass = 0; pass < 2; pass++) {
        for (int j = NSC_WAVE_TABLES - 1; j >= 0; j--) {
            NSUInteger limit = (NSUInteger)1 << j;
            if (pass == 0 && j != NSC_WAVE_TABLES - 1) continue;
            memset(re, 0, half * sizeof(float));
            memset(im, 0, half * sizeof(float));
            // x[n] = sum a_k cos + b_k sin: packed so the inverse real DFT returns it.
            for (NSUInteger k = 1; k < MIN(count, half) && k <= limit; k++) {
                re[k] = a[k];
                im[k] = -b[k];
            }
            float *table = _tables[j] ?: (_tables[j] = calloc(N, sizeof(float)));
            vDSP_DFT_Execute(setup, re, im, ore, oim);
            DSPSplitComplex split = { ore, oim };
            vDSP_ztoc(&split, 1, (DSPComplex *)table, 2, half);
            // vDSP's inverse real DFT returns twice the series.
            float half_ = 0.5f;
            vDSP_vsmul(table, 1, &half_, table, 1, N);
            if (pass == 0) {
                float peak = 0.0f;
                vDSP_maxmgv(table, 1, &peak, N);
                scale = !_disableNormalization && peak > 0.0f ? 1.0f / peak : 1.0f;
            } else {
                vDSP_vsmul(table, 1, &scale, table, 1, N);
            }
        }
    }
    free(re); free(im); free(ore); free(oim);
    vDSP_DFT_DestroySetup(setup);
}

- (void)dealloc {
    for (int j = 0; j < NSC_WAVE_TABLES; j++) free(_tables[j]);
}

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
