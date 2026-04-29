#import "NSCAudioContext.h"
#import "NSCAudioLog.h"

@interface NSCAudioNode ()
@property (nonatomic, readwrite) AVAudioNode *avNode;
@end

@implementation NSCAudioNode

- (instancetype)initWithContext:(NSCAudioContext *)context node:(AVAudioNode *)node {
    if (self = [super init]) {
        _context = context;
        _avNode = node;
        if (_context) {
            @try { [_context registerNodeWrapper:self]; } @catch (NSException *e) {}
        }
    }
    return self;
}

- (void)handleConnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    AVAudioEngine *engine = self.context.engine ?: source.context.engine;
    if (!engine) return;

    AVAudioNodeBus fromBus = (AVAudioNodeBus)MAX(0, output ? output.intValue : 0);

    AVAudioNodeBus toBus = 0;
    if (input) {
        toBus = (AVAudioNodeBus)MAX(0, input.intValue);
    } else if ([self.avNode isKindOfClass:[AVAudioMixerNode class]]) {
        AVAudioMixerNode *m = (AVAudioMixerNode *)self.avNode;
        toBus = m.nextAvailableInputBus;
    } else {
        toBus = 0;
    }

    AVAudioConnectionPoint *destPoint = [[AVAudioConnectionPoint alloc] initWithNode:self.avNode bus:toBus];

    NSCAudioContext *ctx = self.context ?: source.context;
        BOOL sourceIsPlayer = [source.avNode isKindOfClass:[AVAudioPlayerNode class]];
        __block BOOL connectOk = NO;
        NSCLogDebug(@"NSCAudioNode: connecting source=%@(avNode=%@) -> dest=%@(avNode=%@) fromBus=%d toBus=%d",
            NSStringFromClass([source class]), NSStringFromClass([source.avNode class]),
            NSStringFromClass([self class]), NSStringFromClass([self.avNode class]),
            (int)fromBus, (int)toBus);
    if (sourceIsPlayer) {
        AVAudioNode *playerNode = source.avNode;
        void (^connectBlock)(void) = ^{
            @try {
                if (playerNode.engine == engine) {
                    if (ctx) {
                        @try { [ctx detachNode:playerNode fromEngine:engine]; } @catch (NSException *e) {}
                    } else {
                        @try { [engine detachNode:playerNode]; } @catch (NSException *e) {
                            NSCLogDebug(@"NSCAudioNode: detachNode for player threw: %@", e);
                        }
                    }
                }
            } @catch (NSException *e) {
                NSCLogDebug(@"NSCAudioNode: detachNode for player threw: %@", e);
            }
            @try {
                [engine attachNode:playerNode];
            } @catch (NSException *e) {
                NSCLogDebug(@"NSCAudioNode: attachNode for player threw: %@", e);
            }
            @try {
                [engine connect:playerNode to:self.avNode fromBus:fromBus toBus:toBus format:nil];
                connectOk = YES;
            } @catch (NSException *e) {
                NSCLogDebug(@"NSCAudioNode: player connect:to:format:nil threw: %@; trying ctx.format", e);
                @try {
                    [engine connect:playerNode to:self.avNode fromBus:fromBus toBus:toBus format:ctx ? ctx.format : nil];
                    connectOk = YES;
                } @catch (NSException *e2) {
                    NSCLogDebug(@"NSCAudioNode: player connect:to:format:ctx also threw: %@", e2);
                }
            }
        };
        if ([NSThread isMainThread]) connectBlock(); else dispatch_sync(dispatch_get_main_queue(), connectBlock);
    } else {
        void (^connectBlock)(void) = ^{
            @try {
                [engine connect:source.avNode toConnectionPoints:@[destPoint] fromBus:fromBus format:nil];
                connectOk = YES;
            } @catch (NSException *e) {
                NSCLogDebug(@"NSCAudioNode: connect with nil format threw: %@; trying ctx.format", e);
                @try {
                    [engine connect:source.avNode toConnectionPoints:@[destPoint] fromBus:fromBus format:ctx ? ctx.format : nil];
                    connectOk = YES;
                } @catch (NSException *e2) {
                    NSCLogDebug(@"NSCAudioNode: connect with ctx.format also threw: %@", e2);
                }
            }
        };
        if ([NSThread isMainThread]) connectBlock(); else dispatch_sync(dispatch_get_main_queue(), connectBlock);
    }
    if (connectOk && sourceIsPlayer) {
        void (^prepareBlock)(void) = ^{
            @try { [engine prepare]; } @catch (NSException *e) {
                NSCLogDebug(@"NSCAudioNode: engine prepare after player connect threw: %@", e);
            }
        };
        if ([NSThread isMainThread]) prepareBlock(); else dispatch_sync(dispatch_get_main_queue(), prepareBlock);
    }
    if (ctx) {
        NSValue *key = [NSValue valueWithPointer:(__bridge const void *)(source)];
        if ([self isKindOfClass:[NSCGainNode class]]) {
            NSMutableDictionary<NSNumber *, NSCAudioNode *> *map = ctx.voiceGainByOutput[key];
            if (!map) map = [NSMutableDictionary dictionary];
            map[@(output ? output.intValue : 0)] = self;
            ctx.voiceGainByOutput[key] = map;
        } else if ([self isKindOfClass:[NSCBiquadNode class]]) {
            NSMutableDictionary<NSNumber *, NSCAudioNode *> *map = ctx.voiceFilterByOutput[key];
            if (!map) map = [NSMutableDictionary dictionary];
            map[@(output ? output.intValue : 0)] = self;
            ctx.voiceFilterByOutput[key] = map;
        }

        @try {
            [source handleConnectedTo:self output:output input:(input ?: @(toBus))];
        } @catch (NSException *e) {}
    }
}

- (void)handleDisconnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    AVAudioEngine *engine = self.context.engine ?: source.context.engine;
    if (!engine) return;
    NSCAudioContext *ownerCtx = self.context ?: source.context;
    if (source.avNode && [ownerCtx isNode:source.avNode attachedToEngine:engine]) {
        @try { [engine disconnectNodeOutput:source.avNode]; }
        @catch (NSException *e) { NSCLogDebug(@"NSCAudioNode: disconnectNodeOutput threw: %@", e); }
    }
    
    NSCAudioContext *ctx = self.context ?: source.context;
    if (ctx) {
        NSValue *key = [NSValue valueWithPointer:(__bridge const void *)(source)];
        NSMutableDictionary<NSNumber *, NSCAudioNode *> *gmap = ctx.voiceGainByOutput[key];
        if (gmap) {
            if (output) { [gmap removeObjectForKey:@(output.intValue)]; } else { [gmap removeAllObjects]; }
            ctx.voiceGainByOutput[key] = gmap;
        }
        NSMutableDictionary<NSNumber *, NSCAudioNode *> *fmap = ctx.voiceFilterByOutput[key];
        if (fmap) {
            if (output) { [fmap removeObjectForKey:@(output.intValue)]; } else { [fmap removeAllObjects]; }
            ctx.voiceFilterByOutput[key] = fmap;
        }
        NSMutableDictionary<NSNumber *, NSCAudioNode *> *pmap = ctx.voicePannerByOutput[key];
        if (pmap) {
            if (output) { [pmap removeObjectForKey:@(output.intValue)]; } else { [pmap removeAllObjects]; }
            ctx.voicePannerByOutput[key] = pmap;
        }
        
        @try {
            [source handleDisconnectedFrom:self output:output input:input];
        } @catch (NSException *e) {}
    }
}

- (void)handleConnectedTo:(NSCAudioNode *)destination output:(NSNumber *)output input:(NSNumber *)input {
}

- (void)handleDisconnectedFrom:(NSCAudioNode *)destination output:(NSNumber *)output input:(NSNumber *)input {
  
}
- (void)connectTo:(NSCAudioNode *)destination {
    [destination handleConnectFrom:self output:nil input:nil];
}

- (void)connectTo:(NSCAudioNode *)destination output:(NSNumber *)output {
    [destination handleConnectFrom:self output:output input:@(0)];
}

- (void)connectTo:(NSCAudioNode *)destination output:(NSNumber *)output input:(NSNumber *)input {
    [destination handleConnectFrom:self output:output input:input];
}

- (void)disconnect {
    AVAudioEngine *engine = self.context.engine;
    if (!engine) return;
    if (self.avNode && [self.context isNode:self.avNode attachedToEngine:engine]) {
        @try { [engine disconnectNodeOutput:self.avNode]; }
        @catch (NSException *e) { NSCLogDebug(@"NSCAudioNode: disconnect threw: %@", e); }
    }
}

- (void)disconnectOutput:(NSNumber *)output {
    [self disconnect];
}

- (void)disconnectTo:(NSCAudioNode *)destination {
    [destination handleDisconnectFrom:self output:nil input:nil];
}

- (void)disconnectTo:(NSCAudioNode *)destination output:(NSNumber *)output {
    [destination handleDisconnectFrom:self output:output input:nil];
}

- (void)disconnectTo:(NSCAudioNode *)destination output:(NSNumber *)output input:(NSNumber *)input {
    [destination handleDisconnectFrom:self output:output input:input];
}

@end
