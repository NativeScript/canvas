#import <math.h>
#import "NSCAudioContext.h"

@implementation NSCIIRFilterNode {
    NSArray<NSNumber *> *_feedforward;
    NSArray<NSNumber *> *_feedback;
    AVAudioMixerNode *_mixer;
}

- (instancetype)initWithContext:(NSCAudioContext *)context
                                        feedforward:(NSArray<NSNumber *> *)feedforward
                                             feedback:(NSArray<NSNumber *> *)feedback {
    AVAudioMixerNode *mixer = [[AVAudioMixerNode alloc] init];
    [context.engine attachNode:mixer];
    if (self = [super initWithContext:context node:mixer]) {
        _mixer = mixer;
        _feedforward = [feedforward copy];
        _feedback = [feedback copy];
    }
    return self;
}

- (void)getFrequencyResponse:(NSData *)frequencyHzData
                                 magResponse:(NSMutableData *)magResponse
                             phaseResponse:(NSMutableData *)phaseResponse {
    if (!frequencyHzData || !magResponse || !phaseResponse) return;
    const float *freqs = (const float *)frequencyHzData.bytes;
    float *mag = (float *)magResponse.mutableBytes;
    float *phase = (float *)phaseResponse.mutableBytes;
    if (!freqs || !mag || !phase) return;
    NSInteger n = frequencyHzData.length / sizeof(float);
    NSInteger maxOut = MIN((NSInteger)(magResponse.length / sizeof(float)),
                                                 (NSInteger)(phaseResponse.length / sizeof(float)));
    n = MIN(n, maxOut);
    if (n <= 0) return;

    double sr = self.context ? self.context.sampleRate : 48000.0;

    NSUInteger ffCount = _feedforward.count, fbCount = _feedback.count;
    double fb0 = fbCount > 0 ? [_feedback[0] doubleValue] : 1.0;
    if (fb0 == 0.0) fb0 = 1.0;

    double *ff = (double *)malloc(ffCount * sizeof(double));
    double *fb = (double *)malloc(fbCount * sizeof(double));
    if (!ff || !fb) { if (ff) free(ff); if (fb) free(fb); return; }
    for (NSUInteger k = 0; k < ffCount; ++k) ff[k] = [_feedforward[k] doubleValue] / fb0;
    for (NSUInteger k = 0; k < fbCount; ++k) fb[k] = [_feedback[k] doubleValue] / fb0;

    const double twoPiOverSr = 2.0 * M_PI / sr;
    for (NSInteger i = 0; i < n; ++i) {
        double w = twoPiOverSr * (double)freqs[i];
        double numR = 0, numI = 0, denR = 0, denI = 0;
        for (NSUInteger k = 0; k < ffCount; ++k) {
            double wk = w * (double)k;
            numR += ff[k] * cos(wk);
            numI -= ff[k] * sin(wk);
        }
        for (NSUInteger k = 0; k < fbCount; ++k) {
            double wk = w * (double)k;
            denR += fb[k] * cos(wk);
            denI -= fb[k] * sin(wk);
        }
        double dMag = sqrt(denR * denR + denI * denI);
        double nMag = sqrt(numR * numR + numI * numI);
        mag[i] = (float)(dMag == 0.0 ? 0.0 : nMag / dMag);
        phase[i] = (float)(atan2(numI, numR) - atan2(denI, denR));
    }
    free(ff);
    free(fb);
}

@end
