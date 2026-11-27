#import <Accelerate/Accelerate.h>
#import <stdatomic.h>
#import "NSCAudioLog.h"
#import "NSCAudioContext.h"
#import "NSCChannelMixerNode.h"
#import "NSCHRTFKemar.h"

@interface NSCAudioContext (NSCPannerNodes)
- (NSArray<NSCAudioNode *> *)allNodeWrappers;
@end

typedef struct { double x, y, z; } V3;

static inline V3 Sub(V3 a, V3 b) { return (V3){ a.x - b.x, a.y - b.y, a.z - b.z }; }
static inline double Dot(V3 a, V3 b) { return a.x * b.x + a.y * b.y + a.z * b.z; }
static inline V3 Cross(V3 a, V3 b) { return (V3){ a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x }; }
static inline double Len(V3 a) { return sqrt(Dot(a, a)); }
static inline V3 Norm(V3 a) {
    double l = Len(a);
    return l > 0.0 ? (V3){ a.x / l, a.y / l, a.z / l } : a;
}

#pragma mark - HRTF database

// The KEMAR set resampled to the context rate, scaled so an ear averages half the input's power.
typedef struct {
    double rate;
    int taps;
    int positions;
    float *data;
    int offsets[NSC_HRIR_ELEVATIONS];
} HRTFDatabase;

static HRTFDatabase *HRTFDatabaseForRate(double rate) {
    static NSMutableDictionary<NSNumber *, NSValue *> *cache;
    static dispatch_once_t once;
    dispatch_once(&once, ^{ cache = [NSMutableDictionary dictionary]; });
    @synchronized (cache) {
        NSValue *found = cache[@(rate)];
        if (found) return (HRTFDatabase *)found.pointerValue;
        HRTFDatabase *db = calloc(1, sizeof(HRTFDatabase));
        db->rate = rate;
        double ratio = rate / NSCHRIRRate;
        db->taps = (int)ceil(NSCHRIRTaps * ratio);
        int positions = 0;
        for (int e = 0; e < NSC_HRIR_ELEVATIONS; e++) {
            db->offsets[e] = positions;
            positions += NSCHRIRAzimuthCount[e];
        }
        db->positions = positions;
        db->data = calloc((size_t)positions * 2 * db->taps, sizeof(float));
        double energy = 0.0;
        for (int p = 0; p < positions; p++) {
            for (int ear = 0; ear < 2; ear++) {
                const int16_t *src = NSCHRIRData + ((size_t)p * 2 + ear) * NSCHRIRTaps;
                float *dst = db->data + ((size_t)p * 2 + ear) * db->taps;
                for (int k = 0; k < db->taps; k++) {
                    double at = k / ratio;
                    int i = (int)at;
                    double f = at - i;
                    double a = i < NSCHRIRTaps ? src[i] : 0.0, b = i + 1 < NSCHRIRTaps ? src[i + 1] : 0.0;
                    // Resampling up keeps the response's gain; down needs the rate ratio.
                    dst[k] = (float)(((1.0 - f) * a + f * b) / 32768.0 * MIN(1.0, 1.0 / ratio));
                    energy += (double)dst[k] * dst[k];
                }
            }
        }
        float scale = (float)sqrt(0.5 / (energy / (positions * 2)));
        vDSP_vsmul(db->data, 1, &scale, db->data, 1, (vDSP_Length)positions * 2 * db->taps);
        cache[@(rate)] = [NSValue valueWithPointer:db];
        return db;
    }
}

static const float *HRIR(HRTFDatabase *db, int position, int ear) {
    return db->data + ((size_t)position * 2 + ear) * db->taps;
}

// Adds `weight` times the HRIR pair nearest azimuth `az` (0 to 180) at elevation row `e`, interpolated.
static void AddRow(HRTFDatabase *db, int e, double az, double weight, BOOL swap, float *left, float *right) {
    int count = NSCHRIRAzimuthCount[e], first = db->offsets[e];
    double step = 360.0 / NSCHRIRCircle[e];
    int lo = MAX(0, MIN((int)(az / step), count - 1));
    int hi = MIN(lo + 1, count - 1);
    double f = hi > lo ? MAX(0.0, MIN(1.0, az / step - lo)) : 0.0;
    float wl = (float)(weight * (1.0 - f)), wh = (float)(weight * f);
    vDSP_vsma(HRIR(db, first + lo, swap ? 1 : 0), 1, &wl, left, 1, left, 1, db->taps);
    vDSP_vsma(HRIR(db, first + lo, swap ? 0 : 1), 1, &wl, right, 1, right, 1, db->taps);
    if (wh > 0.0f) {
        vDSP_vsma(HRIR(db, first + hi, swap ? 1 : 0), 1, &wh, left, 1, left, 1, db->taps);
        vDSP_vsma(HRIR(db, first + hi, swap ? 0 : 1), 1, &wh, right, 1, right, 1, db->taps);
    }
}

// The ear responses for a Web Audio azimuth (-180 to 180, right positive) and elevation.
static void HRTFKernel(HRTFDatabase *db, double azimuth, double elevation, float *left, float *right) {
    memset(left, 0, db->taps * sizeof(float));
    memset(right, 0, db->taps * sizeof(float));
    BOOL swap = azimuth < 0.0;
    double az = fabs(azimuth);
    double el = MAX(NSCHRIRElevation[0], MIN(elevation, NSCHRIRElevation[NSC_HRIR_ELEVATIONS - 1]));
    int e = 0;
    while (e + 1 < NSC_HRIR_ELEVATIONS && NSCHRIRElevation[e + 1] <= el) e++;
    int e2 = MIN(e + 1, NSC_HRIR_ELEVATIONS - 1);
    double span = NSCHRIRElevation[e2] - NSCHRIRElevation[e];
    double f = span > 0.0 ? (el - NSCHRIRElevation[e]) / span : 0.0;
    AddRow(db, e, az, 1.0 - f, swap, left, right);
    if (f > 0.0) AddRow(db, e2, az, f, swap, left, right);
}

#pragma mark - Render state

#define PAN_MAX_TAPS 512
#define PAN_FRAMES 128

enum { PX, PY, PZ, OX, OY, OZ, LPX, LPY, LPZ, LFX, LFY, LFZ, LUX, LUY, LUZ, PARAMS };

// Owned by the render block; settings are written on the main thread.
@interface NSCPannerState : NSObject {
@public
    NSCAudioParam *params[PARAMS];
    double rate;
    _Atomic int distanceModel;
    _Atomic int panningModel;
    _Atomic double refDistance, maxDistance, rolloffFactor;
    _Atomic double coneInnerAngle, coneOuterAngle, coneOuterGain;
    _Atomic double pan;
    HRTFDatabase *hrtf;
    // HRTF: the kernels in use, the ones being faded to, and input history per channel.
    float kernel[2][PAN_MAX_TAPS], next[2][PAN_MAX_TAPS];
    double kernelAzimuth, kernelElevation;
    BOOL hasKernel;
    float history[2][PAN_MAX_TAPS + PAN_FRAMES];
}
@end

@implementation NSCPannerState
@end

// Spec: distance gain.
static double DistanceGain(NSCPannerState *s, double distance) {
    double ref = atomic_load_explicit(&s->refDistance, memory_order_relaxed);
    double max = atomic_load_explicit(&s->maxDistance, memory_order_relaxed);
    double rolloff = atomic_load_explicit(&s->rolloffFactor, memory_order_relaxed);
    switch (atomic_load_explicit(&s->distanceModel, memory_order_relaxed)) {
        case 1: {
            double d = MAX(MIN(distance, max), ref);
            rolloff = MIN(rolloff, 1.0);
            return max == ref ? 1.0 - rolloff : 1.0 - rolloff * (d - ref) / (max - ref);
        }
        case 2:
            return ref == 0.0 ? 0.0 : pow(MAX(distance, ref) / ref, -rolloff);
        default:
            return ref / (ref + rolloff * (MAX(distance, ref) - ref));
    }
}

// Spec: cone gain.
static double ConeGain(NSCPannerState *s, V3 source, V3 orientation, V3 listener) {
    double inner = atomic_load_explicit(&s->coneInnerAngle, memory_order_relaxed);
    double outer = atomic_load_explicit(&s->coneOuterAngle, memory_order_relaxed);
    if ((orientation.x == 0.0 && orientation.y == 0.0 && orientation.z == 0.0) || (inner == 360.0 && outer == 360.0)) return 1.0;
    V3 toListener = Norm(Sub(listener, source));
    double angle = fabs(180.0 * acos(MAX(-1.0, MIN(1.0, Dot(toListener, Norm(orientation))))) / M_PI);
    double absInner = fabs(inner) / 2.0, absOuter = fabs(outer) / 2.0;
    double outerGain = atomic_load_explicit(&s->coneOuterGain, memory_order_relaxed);
    if (angle <= absInner) return 1.0;
    if (angle >= absOuter) return outerGain;
    double x = (angle - absInner) / (absOuter - absInner);
    return (1.0 - x) + outerGain * x;
}

// Spec: azimuth and elevation of the source as the listener hears it.
static void AzimuthElevation(V3 source, V3 listener, V3 forward, V3 up, double *azimuth, double *elevation) {
    V3 sl = Sub(source, listener);
    if (sl.x == 0.0 && sl.y == 0.0 && sl.z == 0.0) {
        *azimuth = 0.0;
        *elevation = 0.0;
        return;
    }
    sl = Norm(sl);
    V3 right = Norm(Cross(forward, up));
    V3 f = Norm(forward);
    V3 u = Cross(right, f);
    double upProjection = Dot(sl, u);
    V3 projected = Norm(Sub(sl, (V3){ u.x * upProjection, u.y * upProjection, u.z * upProjection }));
    double az = 180.0 * acos(MAX(-1.0, MIN(1.0, Dot(projected, right)))) / M_PI;
    if (Dot(projected, f) < 0.0) az = 360.0 - az;
    az = az >= 0.0 && az <= 270.0 ? 90.0 - az : 450.0 - az;
    double el = 90.0 - 180.0 * acos(MAX(-1.0, MIN(1.0, Dot(sl, u)))) / M_PI;
    if (el > 90.0) el = 180.0 - el;
    else if (el < -90.0) el = -180.0 - el;
    *azimuth = az;
    *elevation = el;
}

static void Spatial(NSCPannerState *s, double v[PARAMS], double *gain, double *azimuth, double *elevation) {
    V3 source = { v[PX], v[PY], v[PZ] }, orientation = { v[OX], v[OY], v[OZ] };
    V3 listener = { v[LPX], v[LPY], v[LPZ] }, forward = { v[LFX], v[LFY], v[LFZ] }, up = { v[LUX], v[LUY], v[LUZ] };
    AzimuthElevation(source, listener, forward, up, azimuth, elevation);
    double pan = atomic_load_explicit(&s->pan, memory_order_relaxed);
    if (pan != 0.0) *azimuth = MAX(-1.0, MIN(1.0, pan)) * 90.0;
    *gain = DistanceGain(s, Len(Sub(source, listener))) * ConeGain(s, source, orientation, listener);
}

static void EqualPower(double azimuth, double gain, BOOL mono, float inL, float inR, float *outL, float *outR) {
    azimuth = MAX(-180.0, MIN(180.0, azimuth));
    if (azimuth < -90.0) azimuth = -180.0 - azimuth;
    else if (azimuth > 90.0) azimuth = 180.0 - azimuth;
    if (mono) {
        double x = (azimuth + 90.0) / 180.0;
        *outL = (float)(inL * cos(x * M_PI_2) * gain);
        *outR = (float)(inL * sin(x * M_PI_2) * gain);
    } else if (azimuth <= 0.0) {
        double x = (azimuth + 90.0) / 90.0;
        *outL = (float)((inL + inR * cos(x * M_PI_2)) * gain);
        *outR = (float)(inR * sin(x * M_PI_2) * gain);
    } else {
        double x = azimuth / 90.0;
        *outL = (float)(inL * cos(x * M_PI_2) * gain);
        *outR = (float)((inR + inL * sin(x * M_PI_2)) * gain);
    }
}

// Each ear convolves its input channel (both the mono one) with its HRIR; a kernel change fades
// across the quantum. Chromium's HRTFPanner works the same way.
static void RenderHRTF(NSCPannerState *s, UInt32 frames, float *const *in, UInt32 inChannels, float *const *out, const double *gain, double azimuth, double elevation) {
    HRTFDatabase *db = s->hrtf;
    int taps = MIN(db->taps, PAN_MAX_TAPS);
    BOOL change = !s->hasKernel || fabs(azimuth - s->kernelAzimuth) > 1e-3 || fabs(elevation - s->kernelElevation) > 1e-3;
    if (change) {
        HRTFKernel(db, azimuth, elevation, s->next[0], s->next[1]);
        if (!s->hasKernel) memcpy(s->kernel, s->next, sizeof(s->kernel));
        s->kernelAzimuth = azimuth;
        s->kernelElevation = elevation;
        s->hasKernel = YES;
    }
    float a[PAN_FRAMES], b[PAN_FRAMES];
    for (int ear = 0; ear < 2; ear++) {
        float *h = s->history[ear];
        memcpy(h + taps - 1, in[MIN((UInt32)ear, inChannels - 1)], frames * sizeof(float));
        vDSP_conv(h, 1, s->kernel[ear] + taps - 1, -1, a, 1, frames, (vDSP_Length)taps);
        if (change) {
            vDSP_conv(h, 1, s->next[ear] + taps - 1, -1, b, 1, frames, (vDSP_Length)taps);
            for (UInt32 k = 0; k < frames; k++) {
                float x = (float)(k + 1) / frames;
                a[k] = a[k] * (1.0f - x) + b[k] * x;
            }
        }
        for (UInt32 k = 0; k < frames; k++) out[ear][k] = (float)(a[k] * gain[k]);
        memmove(h, h + frames, (taps - 1) * sizeof(float));
    }
    if (change) memcpy(s->kernel, s->next, sizeof(s->kernel));
}

#pragma mark - The listener's params

// Owns the AudioListener params: audio connected to them renders before every panner.
@interface NSCListenerOwner : NSCAudioNode
@end

@implementation NSCListenerOwner

- (void)attachParamSink:(AVAudioNode *)sink {
    for (NSCAudioNode *node in [self.context allNodeWrappers]) {
        if ([node isKindOfClass:[NSCAudioPannerNode class]]) [((NSCAudioPannerNode *)node).port connectEarly:sink format:self.context.format];
    }
}

@end

NSCAudioNode *NSCMakeListenerOwner(NSCAudioContext *context) {
    return [[NSCListenerOwner alloc] initWithContext:context node:nil];
}

#pragma mark - The node

@implementation NSCAudioPannerNode {
    NSCPannerState *_state;
}

- (instancetype)initWithContext:(NSCAudioContext *)context {
    return [self initWithContextAndParams:context :nil :nil :nil :nil :nil :nil :nil :nil :nil :nil :nil :nil :nil :nil :nil];
}

- (instancetype)initWithContextAndParams:(NSCAudioContext *)context
                                        :(NSNumber *)positionX
                                        :(NSNumber *)positionY
                                        :(NSNumber *)positionZ
                                        :(NSNumber *)orientationX
                                        :(NSNumber *)orientationY
                                        :(NSNumber *)orientationZ
                                        :(NSNumber *)pan
                                        :(NSNumber *)distanceModel
                                        :(NSNumber *)panningModel
                                        :(NSNumber *)refDistance
                                        :(NSNumber *)maxDistance
                                        :(NSNumber *)rolloffFactor
                                        :(NSNumber *)coneInnerAngle
                                        :(NSNumber *)coneOuterAngle
                                        :(NSNumber *)coneOuterGain {
    if (!(self = [super initWithContext:context outputChannels:2])) return nil;
    NSCPannerState *s = [NSCPannerState new];
    _state = s;
    s->rate = context.sampleRate;
    _positionXParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:positionX ? positionX.doubleValue : 0.0];
    _positionYParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:positionY ? positionY.doubleValue : 0.0];
    _positionZParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:positionZ ? positionZ.doubleValue : 0.0];
    _orientationXParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:orientationX ? orientationX.doubleValue : 1.0];
    _orientationYParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:orientationY ? orientationY.doubleValue : 0.0];
    _orientationZParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:orientationZ ? orientationZ.doubleValue : 0.0];
    NSArray<NSCAudioParam *> *own = @[ _positionXParam, _positionYParam, _positionZParam, _orientationXParam, _orientationYParam, _orientationZParam ];
    NSArray<NSCAudioParam *> *listener = @[ [context getListenerPositionXParam], [context getListenerPositionYParam], [context getListenerPositionZParam],
                                            [context getListenerForwardXParam], [context getListenerForwardYParam], [context getListenerForwardZParam],
                                            [context getListenerUpXParam], [context getListenerUpYParam], [context getListenerUpZParam] ];
    for (NSUInteger i = 0; i < own.count; i++) {
        own[i].owner = self;
        s->params[i] = own[i];
    }
    for (NSUInteger i = 0; i < listener.count; i++) {
        s->params[own.count + i] = listener[i];
        if (listener[i].inputSink) [self.port connectEarly:(AVAudioNode *)listener[i].inputSink format:context.format];
    }
    atomic_init(&s->distanceModel, distanceModel ? distanceModel.intValue : 0);
    atomic_init(&s->panningModel, panningModel && panningModel.intValue == 1 ? 1 : 0);
    atomic_init(&s->refDistance, refDistance ? refDistance.doubleValue : 1.0);
    atomic_init(&s->maxDistance, maxDistance ? maxDistance.doubleValue : 10000.0);
    atomic_init(&s->rolloffFactor, rolloffFactor ? rolloffFactor.doubleValue : 1.0);
    atomic_init(&s->coneInnerAngle, coneInnerAngle ? coneInnerAngle.doubleValue : 360.0);
    atomic_init(&s->coneOuterAngle, coneOuterAngle ? coneOuterAngle.doubleValue : 360.0);
    atomic_init(&s->coneOuterGain, coneOuterGain ? coneOuterGain.doubleValue : 0.0);
    atomic_init(&s->pan, pan ? MAX(-1.0, MIN(1.0, pan.doubleValue)) : 0.0);
    if (atomic_load(&s->panningModel) == 1) s->hrtf = HRTFDatabaseForRate(s->rate);
    [self setChannelConfig:2 mode:1 interpretation:0];
    [self setProcess:^UInt32(int64_t start, UInt32 total, float *const *inAll, UInt32 inChannels, float *const *outAll, UInt32 outChannels) {
      for (UInt32 at = 0; at < total; at += PAN_FRAMES) {
        UInt32 frames = MIN(total - at, (UInt32)PAN_FRAMES);
        int64_t frame = start + at;
        float *in[2] = { inAll[0] + at, inAll[MIN(1u, inChannels - 1)] + at };
        float *out[2] = { outAll[0] + at, outAll[1] + at };
        double values[PARAMS][frames];
        BOOL automated = NO;
        for (int p = 0; p < PARAMS; p++) automated |= [s->params[p] fillValuesFromFrame:(double)frame sampleRate:s->rate frameCount:frames into:values[p]];
        double gains[frames], azimuth = 0.0, elevation = 0.0;
        double v[PARAMS];
        for (int p = 0; p < PARAMS; p++) v[p] = values[p][0];
        Spatial(s, v, &gains[0], &azimuth, &elevation);
        BOOL hrtf = atomic_load_explicit(&s->panningModel, memory_order_relaxed) == 1 && s->hrtf;
        BOOL mono = inChannels == 1;
        if (hrtf) {
            // The HRIRs follow the source per quantum; distance and cone gain per frame.
            for (UInt32 k = 1; k < frames; k++) {
                if (!automated) { gains[k] = gains[0]; continue; }
                double az, el;
                for (int p = 0; p < PARAMS; p++) v[p] = values[p][k];
                Spatial(s, v, &gains[k], &az, &el);
            }
            RenderHRTF(s, frames, in, inChannels, out, gains, azimuth, elevation);
            continue;
        }
        double az = azimuth, gain = gains[0];
        for (UInt32 k = 0; k < frames; k++) {
            if (automated && k > 0) {
                double el;
                for (int p = 0; p < PARAMS; p++) v[p] = values[p][k];
                Spatial(s, v, &gain, &az, &el);
            }
            EqualPower(az, gain, mono, in[0][k], in[mono ? 0 : 1][k], &out[0][k], &out[1][k]);
        }
      }
        return 2;
    }];
    return self;
}

#pragma mark - Source attach / connect

- (void)attachSource:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    [self handleConnectFrom:source output:output input:nil];
}

- (void)detachSource:(NSCAudioNode *)source {
    [self handleDisconnectFrom:source output:nil input:nil];
}

- (void)disconnectPanner {
    [self disconnect];
}

#pragma mark - Setters / getters

- (void)setPan:(NSNumber *)p { atomic_store(&_state->pan, MAX(-1.0, MIN(1.0, p.doubleValue))); }
- (void)setDistanceModel:(NSNumber *)v { atomic_store(&_state->distanceModel, v.intValue); }
- (NSNumber *)getDistanceModel { return @(atomic_load(&_state->distanceModel)); }

- (void)setPanningModel:(NSNumber *)v {
    int model = v.intValue == 1 ? 1 : 0;
    if (model == 1 && !_state->hrtf) _state->hrtf = HRTFDatabaseForRate(_state->rate);
    atomic_store(&_state->panningModel, model);
}
- (NSNumber *)getPanningModel { return @(atomic_load(&_state->panningModel)); }

- (void)setRefDistance:(NSNumber *)v { atomic_store(&_state->refDistance, v.doubleValue); }
- (NSNumber *)getRefDistance { return @(atomic_load(&_state->refDistance)); }
- (void)setMaxDistance:(NSNumber *)v { atomic_store(&_state->maxDistance, v.doubleValue); }
- (NSNumber *)getMaxDistance { return @(atomic_load(&_state->maxDistance)); }
- (void)setRolloffFactor:(NSNumber *)v { atomic_store(&_state->rolloffFactor, v.doubleValue); }
- (NSNumber *)getRolloffFactor { return @(atomic_load(&_state->rolloffFactor)); }
- (void)setConeInnerAngle:(NSNumber *)v { atomic_store(&_state->coneInnerAngle, v.doubleValue); }
- (NSNumber *)getConeInnerAngle { return @(atomic_load(&_state->coneInnerAngle)); }
- (void)setConeOuterAngle:(NSNumber *)v { atomic_store(&_state->coneOuterAngle, v.doubleValue); }
- (NSNumber *)getConeOuterAngle { return @(atomic_load(&_state->coneOuterAngle)); }
- (void)setConeOuterGain:(NSNumber *)v { atomic_store(&_state->coneOuterGain, v.doubleValue); }
- (NSNumber *)getConeOuterGain { return @(atomic_load(&_state->coneOuterGain)); }

- (void)setPosition:(NSNumber *)x :(NSNumber *)y :(NSNumber *)z {
    [self.positionXParam setValue:x];
    [self.positionYParam setValue:y];
    [self.positionZParam setValue:z];
}

- (void)setOrientation:(NSNumber *)x :(NSNumber *)y :(NSNumber *)z {
    [self.orientationXParam setValue:x];
    [self.orientationYParam setValue:y];
    [self.orientationZParam setValue:z];
}

@end
