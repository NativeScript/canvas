#import "NSCAudioContext.h"

@implementation NSCBiquadNode {
    NSString *_currentType;
}

- (instancetype)initWithContext:(NSCAudioContext *)context node:(nullable AVAudioNode *)node {
    return [self initWithContext:context];
}

- (instancetype)initWithContext:(NSCAudioContext *)context {
    return [self initWithContext:context type:@"peaking" frequency:350.0 Q:1.0 gain:0.0 detune:0.0];
}

- (instancetype)initWithContext:(NSCAudioContext *)context type:(NSString *)type frequency:(double)frequency Q:(double)Q gain:(double)gain detune:(double)detune {
    AVAudioUnitEQ *eq = [[AVAudioUnitEQ alloc] initWithNumberOfBands:1];
    AVAudioUnitEQFilterParameters *band = eq.bands[0];
    band.bypass = NO;
    band.frequency = (float)frequency;
    band.gain = (float)gain;
    band.bandwidth = (float)MAX(0.01, 1.0 / MAX(0.0001, Q));
    if ([type isEqualToString:@"lowpass"]) band.filterType = AVAudioUnitEQFilterTypeLowPass;
    else if ([type isEqualToString:@"highpass"]) band.filterType = AVAudioUnitEQFilterTypeHighPass;
    else if ([type isEqualToString:@"bandpass"]) band.filterType = AVAudioUnitEQFilterTypeBandPass;
    else if ([type isEqualToString:@"notch"]) band.filterType = AVAudioUnitEQFilterTypeParametric;
    else if ([type isEqualToString:@"lowshelf"]) band.filterType = AVAudioUnitEQFilterTypeLowShelf;
    else if ([type isEqualToString:@"highshelf"]) band.filterType = AVAudioUnitEQFilterTypeHighShelf;
    else band.filterType = AVAudioUnitEQFilterTypeParametric;
    
    if (self = [super initWithContext:context node:eq]) {
        _frequencyParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:frequency];
        _qParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:Q];
        _gainParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:gain];
        _detuneParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:detune];
        _currentType = [type copy];
        [context.engine attachNode:eq];
    }
    return self;
}

- (void)setType:(NSString *)t {
    AVAudioUnitEQ *eq = (AVAudioUnitEQ *)self.avNode;
    AVAudioUnitEQFilterParameters *band = eq.bands[0];
    if ([t isEqualToString:@"lowpass"]) band.filterType = AVAudioUnitEQFilterTypeLowPass;
    else if ([t isEqualToString:@"highpass"]) band.filterType = AVAudioUnitEQFilterTypeHighPass;
    else if ([t isEqualToString:@"bandpass"]) band.filterType = AVAudioUnitEQFilterTypeBandPass;
    else if ([t isEqualToString:@"notch"]) band.filterType = AVAudioUnitEQFilterTypeParametric;
    else if ([t isEqualToString:@"lowshelf"]) band.filterType = AVAudioUnitEQFilterTypeLowShelf;
    else if ([t isEqualToString:@"highshelf"]) band.filterType = AVAudioUnitEQFilterTypeHighShelf;
    _currentType = [t copy];
}

- (NSString *)getType { return _currentType; }

- (void)setParams:(nullable NSString *)type :(NSNumber *)frequency :(NSNumber *)Q :(NSNumber *)gain {
    AVAudioUnitEQ *eq = (AVAudioUnitEQ *)self.avNode;
    AVAudioUnitEQFilterParameters *band = eq.bands[0];
    if (type) [self setType:type];
    band.frequency = (float)frequency.doubleValue;
    band.gain = (float)gain.doubleValue;
    band.bandwidth = (float)MAX(0.01, 1.0 / MAX(0.0001, Q.doubleValue));
}

@end
