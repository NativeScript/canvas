#import "NSCAudioContext.h"

@implementation NSCChannelMergerNode

- (instancetype)initWithContext:(NSCAudioContext *)context numberOfInputs:(NSInteger)numberOfInputs {
    return [super initWithContext:context mode:NSCMixModeMerge channels:MAX(1, numberOfInputs) discrete:NO];
}

- (NSInteger)numberOfInputs {
    return self.channels;
}

@end
