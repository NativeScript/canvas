#import "NSCAudioSessionManager.h"
#import <float.h>
#import <objc/message.h>

@interface NSObject (NSCAudioSessionManagerActiveAudio)
- (BOOL)hasActiveAudio;
@end

NSNotificationName const NSCAudioSessionManagerDidUpdateNotification = @"NSCAudioSessionManagerDidUpdateNotification";
NSString * const NSCAudioSessionManagerSampleRateKey = @"sampleRate";
NSString * const NSCAudioSessionManagerLatencyKey = @"latency";

@interface NSCAudioSessionManager ()
@property (nonatomic, strong) NSMapTable *requests;
@property (nonatomic) double appliedSampleRate;
@property (nonatomic) double appliedLatency;
@property (nonatomic) BOOL sessionActive;
@end

@implementation NSCAudioSessionManager

+ (instancetype)sharedManager {
    static NSCAudioSessionManager *mgr;
    static dispatch_once_t once;
    dispatch_once(&once, ^{
        mgr = [NSCAudioSessionManager new];
        mgr.requests = [NSMapTable weakToStrongObjectsMapTable];
        mgr.appliedSampleRate = 0.0;
        mgr.appliedLatency = 0.0;
        mgr.sessionActive = NO;
    });
    return mgr;
}

- (void)registerContext:(id)context sampleRate:(double)sampleRate latencyHint:(double)latencyHint {
    if (!context) return;
    NSNumber *sr = @(sampleRate);
    NSNumber *lat = @(latencyHint);
    NSDictionary *req = @{ @"sampleRate": sr, @"latency": lat };
    @synchronized(self) {
        [self.requests setObject:req forKey:context];
        [self _recomputeAndApply];
    }
}

- (void)unregisterContext:(id)context {
    if (!context) return;
    @synchronized(self) {
        [self.requests removeObjectForKey:context];
        [self _recomputeAndApply];
    }
}

- (void)contextDidChangeActiveState:(id)context {
    if (!context) return;
    @synchronized(self) {
        [self _recomputeAndApply];
    }
}

- (void)_recomputeAndApply {
    double bestSampleRate = 0.0;
    double bestLatency = 0.0;
    BOOL anyActive = NO;

    for (id key in self.requests) {
        NSDictionary *req = [self.requests objectForKey:key];
        if (!req) continue;
        double sr = [req[@"sampleRate"] doubleValue];
        double lat = [req[@"latency"] doubleValue];
        if (sr > bestSampleRate) bestSampleRate = sr;
        if (lat > 0.0) {
            if (bestLatency <= 0.0 || lat < bestLatency) bestLatency = lat;
        }
        if ([key respondsToSelector:@selector(hasActiveAudio)]) {
            BOOL active = ((BOOL (*)(id, SEL))objc_msgSend)(key, @selector(hasActiveAudio));
            if (active) anyActive = YES;
        }
    }

    if (bestLatency > 0.0) {
        if (bestLatency < 0.001) bestLatency = 0.001;
        if (bestLatency > 0.5) bestLatency = 0.5;
    }

    BOOL needUpdate = NO;
    if (bestLatency > 0.0) {
        if (fabs(self.appliedLatency - bestLatency) > DBL_EPSILON) needUpdate = YES;
    } else if (self.appliedLatency > 0.0) {

    }
    if (bestSampleRate > 0.0) {
        if (fabs(self.appliedSampleRate - bestSampleRate) > DBL_EPSILON) needUpdate = YES;
    } else if (self.appliedSampleRate > 0.0) {
    }

    AVAudioSession *session = [AVAudioSession sharedInstance];
    __block NSError *err = nil;

    if (![NSThread isMainThread]) {
        dispatch_sync(dispatch_get_main_queue(), ^{
            BOOL sessionOK = [session setCategory:AVAudioSessionCategoryPlayback error:&err];
            if (!sessionOK) {
                NSLog(@"NSCAudioSessionManager: setCategory failed: %@", err);
            }
        });
        if (err) return;
    } else {
        BOOL sessionOK = [session setCategory:AVAudioSessionCategoryPlayback error:&err];
        if (!sessionOK) {
            NSLog(@"NSCAudioSessionManager: setCategory failed: %@", err);
            return;
        }
    }

    if (anyActive && needUpdate) {
        NSLog(@"NSCAudioSessionManager: deferring AVAudioSession update while a context is active");
        return;
    }

    if (!self.sessionActive || needUpdate) {
        if (![NSThread isMainThread]) {
            dispatch_sync(dispatch_get_main_queue(), ^{
                if (bestLatency > 0.0) {
                    NSError *bufErr = nil;
                    if (![session setPreferredIOBufferDuration:bestLatency error:&bufErr]) {
                        NSLog(@"NSCAudioSessionManager: setPreferredIOBufferDuration(%.4f) failed: %@", bestLatency, bufErr);
                    }
                }
                if (bestSampleRate > 0.0) {
                    NSError *srErr = nil;
                    if (![session setPreferredSampleRate:bestSampleRate error:&srErr]) {
                        NSLog(@"NSCAudioSessionManager: setPreferredSampleRate(%.0f) failed: %@", bestSampleRate, srErr);
                    }
                }
                NSError *actErr = nil;
                if (![session setActive:YES error:&actErr]) {
                    NSLog(@"NSCAudioSessionManager: setActive:YES failed: %@", actErr);
                } else {
                    self.sessionActive = YES;
                }
            });
        } else {
            if (bestLatency > 0.0) {
                NSError *bufErr = nil;
                if (![session setPreferredIOBufferDuration:bestLatency error:&bufErr]) {
                    NSLog(@"NSCAudioSessionManager: setPreferredIOBufferDuration(%.4f) failed: %@", bestLatency, bufErr);
                }
            }
            if (bestSampleRate > 0.0) {
                NSError *srErr = nil;
                if (![session setPreferredSampleRate:bestSampleRate error:&srErr]) {
                    NSLog(@"NSCAudioSessionManager: setPreferredSampleRate(%.0f) failed: %@", bestSampleRate, srErr);
                }
            }
            NSError *actErr = nil;
            if (![session setActive:YES error:&actErr]) {
                NSLog(@"NSCAudioSessionManager: setActive:YES failed: %@", actErr);
            } else {
                self.sessionActive = YES;
            }
        }
        self.appliedSampleRate = bestSampleRate;
        self.appliedLatency = bestLatency;
        NSDictionary *info = @{ NSCAudioSessionManagerSampleRateKey: @(self.appliedSampleRate),
                                NSCAudioSessionManagerLatencyKey: @(self.appliedLatency) };
        dispatch_async(dispatch_get_main_queue(), ^{
            [[NSNotificationCenter defaultCenter] postNotificationName:NSCAudioSessionManagerDidUpdateNotification object:self userInfo:info];
        });
    }
}

@end
