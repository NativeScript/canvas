#import "NSCAudioContext.h"

@implementation NSCAudioScheduledSourceNode

- (void)start {}
- (void)stop {}

- (void)startAt:(NSNumber *)when {
    double now = self.context ? self.context.currentTime : 0.0;
    double t = when ? when.doubleValue : 0.0;
    double delay = MAX(0, t - now);
    if (delay <= 0) {
        [self start];
    } else {
        dispatch_after(dispatch_time(DISPATCH_TIME_NOW, (int64_t)(delay * NSEC_PER_SEC)), dispatch_get_global_queue(QOS_CLASS_USER_INITIATED, 0), ^{
            dispatch_async(dispatch_get_main_queue(), ^{ [self start]; });
        });
    }
}

- (void)stopAt:(NSNumber *)when {
    double now = self.context ? self.context.currentTime : 0.0;
    double t = when ? when.doubleValue : 0.0;
    double delay = MAX(0, t - now);
    if (delay <= 0) {
        [self stop];
        if (self.onended) self.onended();
    } else {
        dispatch_after(dispatch_time(DISPATCH_TIME_NOW, (int64_t)(delay * NSEC_PER_SEC)), dispatch_get_global_queue(QOS_CLASS_USER_INITIATED, 0), ^{
            dispatch_async(dispatch_get_main_queue(), ^{ [self stop]; if (self.onended) self.onended(); });
        });
    }
}

@end
