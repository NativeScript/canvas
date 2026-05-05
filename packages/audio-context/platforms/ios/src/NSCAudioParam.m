#import "NSCAudioContext.h"
#include <stdlib.h>

typedef NS_ENUM(NSInteger, ParamEventType) {
    ParamEventTypeSet = 0,
    ParamEventTypeLinearRamp = 1
};

@interface ParamEvent : NSObject
@property (nonatomic) ParamEventType type;
@property (nonatomic) double time;
@property (nonatomic) double value;
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
} NSCParamEventSnapshot;

@implementation NSCAudioParam {
    __strong NSCAudioContext *_context;
    NSMutableArray<ParamEvent *> *_events;
    double _defaultValue;
    double _currentValueInternal;
    NSLock *_lock;
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

- (void)cancelAndHoldAtTime:(NSNumber *)heldValue :(NSNumber *)time {

    [_lock lock];
    double t = time.doubleValue;
    double hv = heldValue.doubleValue;
    NSIndexSet *toRemove = [_events indexesOfObjectsPassingTest:^BOOL(ParamEvent *obj, NSUInteger idx, BOOL *stop) {
        return obj.time >= t;
    }];
    if (toRemove.count > 0) {
        [_events removeObjectsAtIndexes:toRemove];
    }

    ParamEvent *ev = [[ParamEvent alloc] initWithType:ParamEventTypeSet time:t value:hv];
    [_events addObject:ev];
    _currentValueInternal = hv;
    [_lock unlock];
    [self notifyScheduleChanged];
}

- (double)currentScalarValue {
    [_lock lock];
    double v = _currentValueInternal;
    [_lock unlock];
    return v;
}

- (double)valueAtTime:(double)t {
    double v = 0.0;
    BOOL ok = [self fillValuesForRange:t sampleRate:1.0 frameCount:1 into:&v];
    if (!ok) {
        [_lock lock];
        v = _currentValueInternal;
        [_lock unlock];
    }
    return v;
}

- (BOOL)hasEventsAfter:(double)t {
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
    if (!outValues || frameCount <= 0) return NO;
    
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
    }
    [_lock unlock];
    
    if (kRate) {
        double v = def;
        NSInteger firstAfter = (NSInteger)evtCount;
        for (NSUInteger i = 0; i < evtCount; ++i) {
            if (evts[i].time > startTime) { firstAfter = (NSInteger)i; break; }
        }
        if (firstAfter == 0) v = def;
        else if (firstAfter >= (NSInteger)evtCount) v = evts[evtCount - 1].value;
        else {
            double prevT = evts[firstAfter - 1].time;
            double prevV = evts[firstAfter - 1].value;
            ParamEventType nextType = evts[firstAfter].type;
            double nextT = evts[firstAfter].time;
            double nextV = evts[firstAfter].value;
            if (nextType == ParamEventTypeLinearRamp) {
                double ratio = nextT <= prevT ? 1.0 : (startTime - prevT) / (nextT - prevT);
                if (ratio < 0.0) ratio = 0.0; if (ratio > 1.0) ratio = 1.0;
                v = prevV + (nextV - prevV) * ratio;
            } else {
                v = prevV;
            }
        }
        for (NSInteger i = 0; i < frameCount; ++i) outValues[i] = v;
        if (evts != stackEvents) free(evts);
        return YES;
    }

    NSInteger nextIdx = 0;
    NSInteger prevIdx = -1;
    for (NSInteger i = 0; i < frameCount; ++i) {
        double t = startTime + (double)i / sampleRate;
        while (nextIdx < (NSInteger)evtCount && evts[nextIdx].time <= t) {
            prevIdx = nextIdx;
            nextIdx += 1;
        }
        double value;
        if (prevIdx < 0) value = def;
        else if (nextIdx >= (NSInteger)evtCount) value = evts[prevIdx].value;
        else {
            double prevT = evts[prevIdx].time;
            double prevV = evts[prevIdx].value;
            ParamEventType nextType = evts[nextIdx].type;
            double nextT = evts[nextIdx].time;
            double nextV = evts[nextIdx].value;
            if (nextType == ParamEventTypeLinearRamp) {
                double ratio = nextT <= prevT ? 1.0 : (t - prevT) / (nextT - prevT);
                if (ratio < 0.0) ratio = 0.0; if (ratio > 1.0) ratio = 1.0;
                value = prevV + (nextV - prevV) * ratio;
            } else {
                value = prevV;
            }
        }
        outValues[i] = value;
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
