#import "NSCAudioContext.h"

@implementation NSCChannelMergerNode {
    NSInteger _numberOfInputs;
}

- (instancetype)initWithContext:(NSCAudioContext *)context
                 numberOfInputs:(NSInteger)numberOfInputs {
    AVAudioMixerNode *mixer = [[AVAudioMixerNode alloc] init];
    [context.engine attachNode:mixer];

    if (self = [super initWithContext:context node:mixer]) {
        _numberOfInputs = MAX(1, numberOfInputs);
    }

    return self;
}

- (NSInteger)numberOfInputs {
    return _numberOfInputs;
}

- (void)connectTo:(NSCAudioNode *)destination {
    if (!destination) return;
    [destination handleConnectFrom:self output:@(0) input:@(0)];
}

- (void)connectTo:(NSCAudioNode *)destination output:(NSNumber *)output {
    if (!destination) return;
    [destination handleConnectFrom:self output:@(0) input:@(0)];
}

- (void)connectTo:(NSCAudioNode *)destination output:(NSNumber *)output input:(NSNumber *)input {
    if (!destination) return;
    [destination handleConnectFrom:self output:@(0) input:(input ?: @(0))];
}

- (void)disconnectTo:(NSCAudioNode *)destination {
    if (!destination) return;
    [destination handleDisconnectFrom:self output:@(0) input:@(0)];
}

- (void)disconnectTo:(NSCAudioNode *)destination output:(NSNumber *)output {
    if (!destination) return;
    [destination handleDisconnectFrom:self output:@(0) input:@(0)];
}

- (void)disconnectTo:(NSCAudioNode *)destination output:(NSNumber *)output input:(NSNumber *)input {
    if (!destination) return;
    [destination handleDisconnectFrom:self output:@(0) input:(input ?: @(0))];
}

@end
