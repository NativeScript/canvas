#import "NSCAudioContext.h"

static NSInteger FilterType(NSString *type) {
    NSArray<NSString *> *types = @[ @"lowpass", @"highpass", @"bandpass", @"lowshelf", @"highshelf", @"peaking", @"notch", @"allpass" ];
    NSUInteger index = type ? [types indexOfObject:type] : NSNotFound;
    return index == NSNotFound ? -1 : (NSInteger)index;
}

@implementation NSCBiquadNode {
    NSString *_currentType;
}

- (instancetype)initWithContext:(NSCAudioContext *)context {
    return [self initWithContext:context type:@"lowpass" frequency:350.0 Q:1.0 gain:0.0 detune:0.0];
}

- (instancetype)initWithContext:(NSCAudioContext *)context type:(NSString *)type frequency:(double)frequency Q:(double)Q gain:(double)gain detune:(double)detune {
    NSCAudioParam *f = [[NSCAudioParam alloc] initWithContext:context defaultValue:frequency];
    NSCAudioParam *q = [[NSCAudioParam alloc] initWithContext:context defaultValue:Q];
    NSCAudioParam *g = [[NSCAudioParam alloc] initWithContext:context defaultValue:gain];
    NSCAudioParam *d = [[NSCAudioParam alloc] initWithContext:context defaultValue:detune];
    if (self = [super initWithContext:context kind:NSCParamEffectKindBiquad params:@[ f, q, g, d ] maxDelayTime:0.0]) {
        _frequencyParam = f;
        _qParam = q;
        _gainParam = g;
        _detuneParam = d;
        _currentType = @"lowpass";
        [self setType:type ?: @"lowpass"];
    }
    return self;
}

- (void)setType:(NSString *)t {
    NSInteger type = FilterType(t);
    if (type < 0) return;
    _currentType = [t copy];
    [self setFilterType:type];
}

- (NSString *)getType { return _currentType; }

- (void)setParams:(nullable NSString *)type :(NSNumber *)frequency :(NSNumber *)Q :(NSNumber *)gain {
    if (type) [self setType:type];
    [_frequencyParam setValue:frequency];
    [_qParam setValue:Q];
    [_gainParam setValue:gain];
}

@end
