#import "NSCAudioContext.h"

// Web Audio's equal-power stereo panning, with pan read per frame.
@implementation NSCStereoPannerNode

- (instancetype)initWithContext:(NSCAudioContext *)context pan:(double)pan {
    if (!(self = [super initWithContext:context outputChannels:2])) return nil;
    NSCAudioParam *param = [[NSCAudioParam alloc] initWithContext:context defaultValue:pan];
    _panParam = param;
    param.owner = self;
    double rate = context.sampleRate;
    [self setProcess:^UInt32(int64_t frame, UInt32 frames, float *const *in, UInt32 inChannels, float *const *out, UInt32 outChannels) {
        double p[frames];
        BOOL automated = [param fillValuesFromFrame:(double)frame sampleRate:rate frameCount:frames into:p];
        BOOL mono = inChannels == 1;
        for (UInt32 k = 0; k < frames; k++) {
            double v = automated ? p[k] : p[0];
            v = isfinite(v) ? MAX(-1.0, MIN(1.0, v)) : 0.0;
            if (mono) {
                double x = (v + 1.0) * 0.5;
                out[0][k] = in[0][k] * (float)cos(x * M_PI_2);
                out[1][k] = in[0][k] * (float)sin(x * M_PI_2);
            } else if (v <= 0.0) {
                double x = v + 1.0;
                out[0][k] = in[0][k] + in[1][k] * (float)cos(x * M_PI_2);
                out[1][k] = in[1][k] * (float)sin(x * M_PI_2);
            } else {
                double x = v;
                out[0][k] = in[0][k] * (float)cos(x * M_PI_2);
                out[1][k] = in[1][k] + in[0][k] * (float)sin(x * M_PI_2);
            }
        }
        return 2;
    }];
    return self;
}

@end
