#import <Foundation/Foundation.h>

#ifndef NSC_AUDIO_DEBUG
#define NSC_AUDIO_DEBUG 0
#endif

static inline BOOL NSCAudioDebugEnabled(void) {
#if NSC_AUDIO_DEBUG
    return YES;
#else
    static BOOL enabled;
    static dispatch_once_t onceToken;
    dispatch_once(&onceToken, ^{
        const char *env = getenv("NSC_AUDIO_DEBUG");
        enabled = env && env[0] && env[0] != '0';
    });
    return enabled;
#endif
}

#define NSCLogDebug(fmt, ...)                                  \
    do {                                                       \
        if (NSCAudioDebugEnabled()) NSLog((fmt), ##__VA_ARGS__); \
    } while (0)

#define NSCLogError(fmt, ...) NSLog((fmt), ##__VA_ARGS__)
