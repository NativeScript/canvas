#import "NSCAudioContext.h"
#include <stdlib.h>

typedef NS_ENUM(NSInteger, ParamEventType) {
    ParamEventTypeSet = 0,
    ParamEventTypeLinearRamp = 1,
    ParamEventTypeExpRamp = 2,
    // value: target, aux: time constant.
    ParamEventTypeTarget = 3,
    // curve values from time for aux seconds.
    ParamEventTypeCurve = 4,
};

@interface ParamEvent : NSObject
@property (nonatomic) ParamEventType type;
@property (nonatomic) double time;
@property (nonatomic) double value;
@property (nonatomic) double aux;
@property (nonatomic, strong) NSData *curve;
- (instancetype)initWithType:(ParamEventType)type time:(double)time value:(double)value;
@end

@implementation ParamEvent
- (instancetype)initWithType:(ParamEventType)type time:(double)time value:(double)value {
    if (self = [super init]) {
        _type = type;
        _time = time;
        _value = value;
    }
    return self;
}
@end

typedef struct {
    ParamEventType type;
    double time;
    double value;
    double aux;
    const float *curve;
    NSUInteger curveLength;
} NSCParamEventSnapshot;

@implementation NSCAudioParam {
    __weak NSCAudioContext *_context;
    NSMutableArray<ParamEvent *> *_events;
    double _defaultValue;
    double _currentValueInternal;
    NSLock *_lock;
    NSMutableData *_inputData;
    _Atomic(NSCParamInputBlock *) _input;
}

- (instancetype)initWithContext:(NSCAudioContext *)context defaultValue:(double)defaultValue {
    if (self = [super init]) {
        _context = context;
        _defaultValue = defaultValue;
        _currentValueInternal = defaultValue;
        _events = [NSMutableArray array];
        _lock = [NSLock new];
        _automationRate = @"a-rate";
    }
    return self;
}

- (double)value {
    [_lock lock];
    double v = _currentValueInternal;
    [_lock unlock];
    return v;
}

- (void)setValue:(NSNumber *)value {
    double now = _context ? _context.currentTime : 0.0;
    [self setValueAtTime:value :@(now)];
}


static inline void NSCAudioParam_insertSorted(NSMutableArray<ParamEvent *> *events, ParamEvent *ev) {
    NSUInteger count = events.count;
    if (count == 0 || ev.time >= events[count - 1].time) {
        [events addObject:ev];
        return;
    }
    NSUInteger lo = 0, hi = count;
    double t = ev.time;
    while (lo < hi) {
        NSUInteger mid = (lo + hi) >> 1;
        if (events[mid].time <= t) lo = mid + 1;
        else hi = mid;
    }
    [events insertObject:ev atIndex:lo];
}

- (void)notifyScheduleChanged {
    void (^cb)(NSCAudioParam *) = self.onScheduleChanged;
    if (!cb) return;
    if ([NSThread isMainThread]) {
        cb(self);
    } else {
        __weak typeof(self) weakSelf = self;
        dispatch_async(dispatch_get_main_queue(), ^{
            __strong typeof(weakSelf) s = weakSelf;
            if (!s) return;
            void (^c)(NSCAudioParam *) = s.onScheduleChanged;
            if (c) c(s);
        });
    }
}

- (void)setValueAtTime:(NSNumber *)value :(NSNumber *)time {
    [_lock lock];
    ParamEvent *ev = [[ParamEvent alloc] initWithType:ParamEventTypeSet time:time.doubleValue value:value.doubleValue];
    NSCAudioParam_insertSorted(_events, ev);
    _currentValueInternal = value.doubleValue;
    [_lock unlock];
    [self notifyScheduleChanged];
}

- (void)linearRampToValueAtTime:(NSNumber *)value :(NSNumber *)time {
    [_lock lock];
    ParamEvent *ev = [[ParamEvent alloc] initWithType:ParamEventTypeLinearRamp time:time.doubleValue value:value.doubleValue];
    NSCAudioParam_insertSorted(_events, ev);
    [_lock unlock];
    [self notifyScheduleChanged];
}

- (void)exponentialRampToValueAtTime:(NSNumber *)value :(NSNumber *)time {
    [_lock lock];
    ParamEvent *ev = [[ParamEvent alloc] initWithType:ParamEventTypeExpRamp time:time.doubleValue value:value.doubleValue];
    NSCAudioParam_insertSorted(_events, ev);
    [_lock unlock];
    [self notifyScheduleChanged];
}

- (void)setTargetAtTime:(NSNumber *)target :(NSNumber *)startTime :(NSNumber *)timeConstant {
    [_lock lock];
    ParamEvent *ev = [[ParamEvent alloc] initWithType:ParamEventTypeTarget time:startTime.doubleValue value:target.doubleValue];
    ev.aux = timeConstant.doubleValue;
    NSCAudioParam_insertSorted(_events, ev);
    [_lock unlock];
    [self notifyScheduleChanged];
}

- (void)setValueCurveAtTime:(NSData *)values :(NSNumber *)startTime :(NSNumber *)duration {
    if (values.length < 2 * sizeof(float)) return;
    [_lock lock];
    const float *v = (const float *)values.bytes;
    ParamEvent *ev = [[ParamEvent alloc] initWithType:ParamEventTypeCurve time:startTime.doubleValue value:v[values.length / sizeof(float) - 1]];
    ev.aux = duration.doubleValue;
    ev.curve = [values copy];
    NSCAudioParam_insertSorted(_events, ev);
    // The curve's last value holds from its end, as a set would.
    ParamEvent *end = [[ParamEvent alloc] initWithType:ParamEventTypeSet time:startTime.doubleValue + duration.doubleValue value:ev.value];
    NSCAudioParam_insertSorted(_events, end);
    [_lock unlock];
    [self notifyScheduleChanged];
}

- (double)computedValue {
    double now = _context ? _context.currentTime : 0.0;
    return [self valueAtTime:now];
}

- (void)cancelScheduledValues:(NSNumber *)time {
    [_lock lock];
    double t = time.doubleValue;
    NSIndexSet *toRemove = [_events indexesOfObjectsPassingTest:^BOOL(ParamEvent *obj, NSUInteger idx, BOOL *stop) {
        return obj.time >= t;
    }];
    if (toRemove.count > 0) {
        [_events removeObjectsAtIndexes:toRemove];
    }
    [_lock unlock];
    [self notifyScheduleChanged];
}

// Holds the value automation reaches at `time`; a ramp running then ends there with it.
- (void)cancelAndHoldAtTime:(NSNumber *)heldValue :(NSNumber *)time {
    double t = time.doubleValue;
    double hv = [self scheduledValueAtTime:t];
    [_lock lock];
    ParamEventType cut = ParamEventTypeSet;
    for (ParamEvent *e in _events) {
        if (e.time >= t) { cut = e.type; break; }
    }
    NSIndexSet *toRemove = [_events indexesOfObjectsPassingTest:^BOOL(ParamEvent *obj, NSUInteger idx, BOOL *stop) {
        return obj.time >= t;
    }];
    if (toRemove.count > 0) [_events removeObjectsAtIndexes:toRemove];
    ParamEventType held = cut == ParamEventTypeLinearRamp || cut == ParamEventTypeExpRamp ? cut : ParamEventTypeSet;
    NSCAudioParam_insertSorted(_events, [[ParamEvent alloc] initWithType:held time:t value:hv]);
    [_lock unlock];
    [self notifyScheduleChanged];
}

- (double)scheduledValueAtTime:(double)t {
    double v = 0.0;
    double rate = _context ? _context.sampleRate : 0.0;
    [self fillScheduled:t byFrame:NO sampleRate:(rate > 0.0 ? rate : 1.0) frameCount:1 into:&v];
    return v;
}

- (NSMutableData *)inputData {
    if (_inputData) return _inputData;
    NSMutableData *data = [NSMutableData dataWithLength:sizeof(NSCParamInputBlock)];
    NSCParamInputBlock *block = (NSCParamInputBlock *)data.mutableBytes;
    atomic_init(&block->frame, -1.0);
    atomic_init(&block->frames, 0);
    _inputData = data;
    atomic_store_explicit(&_input, block, memory_order_release);
    return data;
}

// Adds the connected audio for frames first...; a frame just past the block holds its last value.
static BOOL AddInput(NSCParamInputBlock *in, int64_t first, NSInteger count, double *out, BOOL kRate) {
    if (!in) return NO;
    double frame = atomic_load_explicit(&in->frame, memory_order_acquire);
    uint32_t frames = atomic_load_explicit(&in->frames, memory_order_relaxed);
    if (frame < 0.0 || frames == 0) return NO;
    int64_t start = llround(frame);
    BOOL added = NO;
    for (NSInteger i = 0; i < count; i++) {
        int64_t at = (kRate ? first : first + i) - start;
        if (at < 0 || at >= 2 * (int64_t)frames) continue;
        out[i] += in->values[MIN(at, (int64_t)frames - 1)];
        added = YES;
    }
    return added;
}

- (double)currentScalarValue {
    [_lock lock];
    double v = _currentValueInternal;
    [_lock unlock];
    return v;
}

- (double)valueAtTime:(double)t {
    double v = 0.0;
    double rate = _context ? _context.sampleRate : 0.0;
    [self fillValuesForRange:t sampleRate:(rate > 0.0 ? rate : 1.0) frameCount:1 into:&v];
    return v;
}

- (BOOL)hasEventsAfter:(double)t {
    // Connected audio changes it every quantum.
    if (self.inputNode) return YES;
    [_lock lock];
    BOOL has = NO;
    for (ParamEvent *e in _events) {
        if (e.time > t) { has = YES; break; }
    }
    [_lock unlock];
    return has;
}

- (BOOL)fillValuesForRange:(double)startTime
                                sampleRate:(double)sampleRate
                                frameCount:(NSInteger)frameCount
                                            into:(double *)outValues {
    return [self fillValues:startTime byFrame:NO sampleRate:sampleRate frameCount:frameCount into:outValues];
}

- (BOOL)fillValuesFromFrame:(double)startFrame
                 sampleRate:(double)sampleRate
                 frameCount:(NSInteger)frameCount
                       into:(double *)outValues {
    if (sampleRate <= 0.0) return [self fillValuesForRange:0.0 sampleRate:1.0 frameCount:frameCount into:outValues];
    return [self fillValues:startFrame byFrame:YES sampleRate:sampleRate frameCount:frameCount into:outValues];
}

- (BOOL)fillValues:(double)start
           byFrame:(BOOL)byFrame
        sampleRate:(double)sampleRate
        frameCount:(NSInteger)frameCount
              into:(double *)outValues {
    BOOL automated = [self fillScheduled:start byFrame:byFrame sampleRate:sampleRate frameCount:frameCount into:outValues];
    NSCParamInputBlock *in = atomic_load_explicit(&_input, memory_order_acquire);
    if (!in || !outValues || frameCount <= 0) return automated;
    int64_t first = llround(byFrame ? start : start * sampleRate);
    BOOL kRate = [_automationRate isEqualToString:@"k-rate"];
    return AddInput(in, first, frameCount, outValues, kRate) || automated;
}

// The value at time t of the spec's automation timeline.
static double EvaluateAt(const NSCParamEventSnapshot *e, NSUInteger count, double def, double t) {
    // The last event at or before t, and the one after it.
    NSInteger prev = -1;
    NSUInteger lo = 0, hi = count;
    while (lo < hi) {
        NSUInteger mid = (lo + hi) >> 1;
        if (e[mid].time <= t) lo = mid + 1; else hi = mid;
    }
    prev = (NSInteger)lo - 1;
    NSUInteger next = lo;
    // The value an event leaves behind, at its own time.
    double base = def, baseTime = 0.0;
    for (NSInteger k = 0; k <= prev; k++) {
        const NSCParamEventSnapshot *ev = &e[k];
        switch (ev->type) {
            case ParamEventTypeTarget: {
                // Settles from the value before it; the next event starts from wherever it got.
                double endT = (NSUInteger)(k + 1) < count ? MIN(e[k + 1].time, t) : t;
                double v0 = base;
                base = ev->aux > 0.0 ? ev->value + (v0 - ev->value) * exp(-(endT - ev->time) / ev->aux) : ev->value;
                baseTime = endT;
                if ((NSUInteger)(k + 1) < count && e[k + 1].time <= t) continue;
                return base;
            }
            case ParamEventTypeCurve: {
                double dur = ev->aux;
                double endT = ev->time + dur;
                if (t < endT && k == prev) {
                    NSUInteger n = ev->curveLength;
                    double pos = dur > 0.0 ? (t - ev->time) / dur * (double)(n - 1) : (double)(n - 1);
                    NSUInteger i0 = (NSUInteger)MAX(0.0, floor(pos));
                    if (i0 >= n - 1) return ev->curve[n - 1];
                    double f = pos - i0;
                    return ev->curve[i0] + (ev->curve[i0 + 1] - ev->curve[i0]) * f;
                }
                base = ev->value;
                baseTime = endT;
                break;
            }
            default:
                base = ev->value;
                baseTime = ev->time;
                break;
        }
    }
    if (next < count) {
        const NSCParamEventSnapshot *n = &e[next];
        double t0 = prev >= 0 ? baseTime : 0.0, t1 = n->time;
        if (prev >= 0 && e[prev].type == ParamEventTypeTarget) t0 = e[prev].time;
        if (n->type == ParamEventTypeLinearRamp) {
            double r = t1 <= t0 ? 1.0 : MAX(0.0, MIN(1.0, (t - t0) / (t1 - t0)));
            return base + (n->value - base) * r;
        }
        if (n->type == ParamEventTypeExpRamp) {
            if (base * n->value <= 0.0) return base;
            double r = t1 <= t0 ? 1.0 : MAX(0.0, MIN(1.0, (t - t0) / (t1 - t0)));
            return base * pow(n->value / base, r);
        }
    }
    return base;
}

// byFrame: frame i is at (start + i) / sampleRate, so an event on a frame lands exactly on it.
- (BOOL)fillScheduled:(double)start
              byFrame:(BOOL)byFrame
           sampleRate:(double)sampleRate
           frameCount:(NSInteger)frameCount
                 into:(double *)outValues {
    if (!outValues || frameCount <= 0) return NO;
    double startTime = byFrame ? start / sampleRate : start;
    
    [_lock lock];
    NSUInteger evtCount = _events.count;
    BOOL kRate = (_automationRate == nil) ? NO : [_automationRate isEqualToString:@"k-rate"];
    double def = _defaultValue;
    if (evtCount == 0) {
        double cur = _currentValueInternal;
        [_lock unlock];
        for (NSInteger i = 0; i < frameCount; ++i) outValues[i] = cur;
        return NO;
    }

    const NSUInteger kStackSnapshotCapacity = 64;
    NSCParamEventSnapshot stackEvents[kStackSnapshotCapacity];
    NSCParamEventSnapshot *evts = stackEvents;
    if (evtCount > kStackSnapshotCapacity) {
        evts = (NSCParamEventSnapshot *)malloc(sizeof(NSCParamEventSnapshot) * evtCount);
        if (!evts) {
            double cur = _currentValueInternal;
            [_lock unlock];
            for (NSInteger i = 0; i < frameCount; ++i) outValues[i] = cur;
            return NO;
        }
    }
    for (NSUInteger i = 0; i < evtCount; ++i) {
        ParamEvent *e = _events[i];
        evts[i].type = e.type;
        evts[i].time = e.time;
        evts[i].value = e.value;
        evts[i].aux = e.aux;
        evts[i].curve = (const float *)e.curve.bytes;
        evts[i].curveLength = e.curve.length / sizeof(float);
    }
    [_lock unlock];
    
    // Curves and targets read their data, which the events keep; snapshot them under the lock.
    if (kRate) {
        double v = EvaluateAt(evts, evtCount, def, startTime);
        for (NSInteger i = 0; i < frameCount; ++i) outValues[i] = v;
    } else {
        for (NSInteger i = 0; i < frameCount; ++i) {
            double t = byFrame ? (start + (double)i) / sampleRate : startTime + (double)i / sampleRate;
            outValues[i] = EvaluateAt(evts, evtCount, def, t);
        }
    }
    if (evts != stackEvents) free(evts);
    return YES;
}

- (NSArray<NSNumber *> *)getValuesForRange:(double)startTime :(double)sampleRate :(NSInteger)frameCount {
    if (frameCount <= 0) return @[];

    NSMutableData *scratch = [NSMutableData dataWithLength:sizeof(double) * (NSUInteger)frameCount];
    double *values = (double *)scratch.mutableBytes;
    [self fillValuesForRange:startTime sampleRate:sampleRate frameCount:frameCount into:values];

    NSMutableArray<NSNumber *> *out = [NSMutableArray arrayWithCapacity:frameCount];
    for (NSInteger i = 0; i < frameCount; ++i) {
        [out addObject:@(values[i])];
    }
    return out;
}

@end
