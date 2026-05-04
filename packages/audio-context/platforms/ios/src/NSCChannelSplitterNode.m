#import "NSCAudioContext.h"
#import "NSCAudioLog.h"
#import <objc/message.h>

@implementation NSCChannelSplitterNode {
    NSInteger _numberOfOutputs;
    AVAudioMixerNode *_inputMixer;
    NSMutableArray<NSCAudioNode *> *_outputNodes;
}

- (instancetype)initWithContext:(NSCAudioContext *)context
                numberOfOutputs:(NSInteger)numberOfOutputs {
    AVAudioMixerNode *inputMixer = [[AVAudioMixerNode alloc] init];
    [context.engine attachNode:inputMixer];

    if (self = [super initWithContext:context node:inputMixer]) {
        _inputMixer = inputMixer;
        _numberOfOutputs = MAX(1, numberOfOutputs);
        _outputNodes = [NSMutableArray arrayWithCapacity:(NSUInteger)_numberOfOutputs];

        for (NSInteger i = 0; i < _numberOfOutputs; i++) {
            AVAudioMixerNode *outMixer = [[AVAudioMixerNode alloc] init];
            [context.engine attachNode:outMixer];
            NSCAudioNode *wrapper =
                [[NSCAudioNode alloc] initWithContext:context node:outMixer];
            [_outputNodes addObject:wrapper];
        }
    }

    return self;
}

- (NSInteger)numberOfOutputs {
    return _numberOfOutputs;
}

- (NSNumber *)normalizedOutput:(nullable NSNumber *)output {
    NSInteger out = output ? output.integerValue : 0;
    if (out < 0) out = 0;
    if (out >= _numberOfOutputs) out = _numberOfOutputs - 1;
    return @(out);
}

- (NSCAudioNode *)outputNodeForOutput:(nullable NSNumber *)output {
    if (_outputNodes.count == 0) return self;
    NSInteger out = [self normalizedOutput:output].integerValue;
    if (out < 0) out = 0;
    if (out >= (NSInteger)_outputNodes.count) out = (NSInteger)_outputNodes.count - 1;
    [self ensureOutputConnectedAtIndex:out];
    NSCAudioNode *node = _outputNodes[(NSUInteger)out];
    return node ?: self;
}

- (void)ensureOutputConnectedAtIndex:(NSInteger)index {
    if (index < 0 || index >= (NSInteger)_outputNodes.count) return;
    NSCAudioContext *ctx = self.context;
    AVAudioEngine *engine = ctx.engine;
    if (!ctx || !engine || !_inputMixer) return;

    NSCAudioNode *wrapper = _outputNodes[(NSUInteger)index];
    AVAudioNode *outNode = wrapper.avNode;
    if (!outNode) return;

    BOOL alreadyConnected = NO;
    SEL pointsSelector = NSSelectorFromString(@"outputConnectionPointsForNode:outputBus:");
    NSMutableArray<AVAudioConnectionPoint *> *targetPoints = [NSMutableArray array];

    if ([engine respondsToSelector:pointsSelector]) {
        @try {
            typedef NSArray *(*MsgSendFn)(id, SEL, id, AVAudioNodeBus);
            MsgSendFn fn = (MsgSendFn)objc_msgSend;
            NSArray *existingPoints = fn(engine, pointsSelector, _inputMixer, (AVAudioNodeBus)0);
            if (existingPoints.count > 0) {
                [targetPoints addObjectsFromArray:existingPoints];
            }
        } @catch (NSException *e) {
            NSCLogDebug(@"NSCChannelSplitterNode: existing points read failed: %@", e);
        }
    }

    for (AVAudioConnectionPoint *point in targetPoints) {
        if (point.node == outNode && point.bus == 0) {
            alreadyConnected = YES;
            break;
        }
    }
    if (alreadyConnected) return;

    AVAudioConnectionPoint *destPoint =
        [[AVAudioConnectionPoint alloc] initWithNode:outNode bus:0];
    [targetPoints addObject:destPoint];

    BOOL connected = NO;
    @try {
        [engine connect:_inputMixer
            toConnectionPoints:targetPoints
                       fromBus:0
                        format:nil];
        connected = YES;
    } @catch (NSException *e) {
        @try {
            [engine connect:_inputMixer
                toConnectionPoints:targetPoints
                           fromBus:0
                            format:ctx.format];
            connected = YES;
        } @catch (NSException *e2) {
            NSCLogError(@"NSCChannelSplitterNode: failed to connect output %ld: %@ / %@",
                        (long)index, e, e2);
        }
    }

    if (connected) {
        NSCLogDebug(@"NSCChannelSplitterNode: connected output branch %ld",
                    (long)index);
    }
}

- (void)connectTo:(NSCAudioNode *)destination {
    if (!destination) return;
    NSCAudioNode *source = [self outputNodeForOutput:@(0)];
    [destination handleConnectFrom:source output:@(0) input:@(0)];
}

- (void)connectTo:(NSCAudioNode *)destination output:(NSNumber *)output {
    if (!destination) return;
    NSCAudioNode *source = [self outputNodeForOutput:output];
    [destination handleConnectFrom:source output:@(0) input:@(0)];
}

- (void)connectTo:(NSCAudioNode *)destination output:(NSNumber *)output input:(NSNumber *)input {
    if (!destination) return;
    NSCAudioNode *source = [self outputNodeForOutput:output];
    [destination handleConnectFrom:source output:@(0) input:(input ?: @(0))];
}

- (void)disconnectTo:(NSCAudioNode *)destination {
    if (!destination) return;
    NSCAudioNode *source = [self outputNodeForOutput:@(0)];
    [destination handleDisconnectFrom:source output:@(0) input:@(0)];
}

- (void)disconnectTo:(NSCAudioNode *)destination output:(NSNumber *)output {
    if (!destination) return;
    NSCAudioNode *source = [self outputNodeForOutput:output];
    [destination handleDisconnectFrom:source output:@(0) input:@(0)];
}

- (void)disconnectTo:(NSCAudioNode *)destination output:(NSNumber *)output input:(NSNumber *)input {
    if (!destination) return;
    NSCAudioNode *source = [self outputNodeForOutput:output];
    [destination handleDisconnectFrom:source output:@(0)
                                input:(input ?: @(0))];
}

@end
