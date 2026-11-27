#import "NSCAudioContext.h"
#import "NSCAudioLog.h"
#import <objc/message.h>
#import "NSCParamEffectNode.h"
#import "NSCChannelMixerNode.h"

@interface NSCAudioNode ()
@property (nonatomic, readwrite) AVAudioNode *avNode;
@end

@interface NSCAudioContext (NSCHealing)
- (NSArray<NSCAudioNode *> *)allNodeWrappersForHealing;
@end

static void OnMain(void (^block)(void)) {
    if ([NSThread isMainThread]) block();
    else dispatch_sync(dispatch_get_main_queue(), block);
}

static void WillConnect(NSCAudioContext *context) {
    if ([context isKindOfClass:[NSCOfflineAudioContext class]]) [(NSCOfflineAudioContext *)context willConnect];
}

// A running engine does not always allocate a unit connected upstream of a new connection.
void NSCEnsureAllocated(AVAudioNode *node) {
    if (![node isKindOfClass:[AVAudioUnit class]]) return;
    AUAudioUnit *au = ((AVAudioUnit *)node).AUAudioUnit;
    if (au.renderResourcesAllocated) return;
    NSError *err = nil;
    if (![au allocateRenderResourcesAndReturnError:&err]) NSCLogError(@"NSCAudioNode: could not allocate %@: %@", NSStringFromClass([au class]), err);
}

BOOL NSCConnectAdding(NSCAudioContext *context, AVAudioNode *src, AVAudioNodeBus fromBus, AVAudioNode *dst, AVAudioNodeBus toBus, AVAudioFormat *format) {
    AVAudioEngine *engine = context.engine;
    if (!engine || !src || !dst) return NO;
    __block BOOL ok = NO;
    OnMain(^{
        WillConnect(context);
        NSMutableArray<AVAudioConnectionPoint *> *points = [NSMutableArray array];
        @try {
            for (AVAudioConnectionPoint *point in [engine outputConnectionPointsForNode:src outputBus:fromBus]) {
                if (!(point.node == dst && point.bus == toBus)) [points addObject:point];
            }
        } @catch (NSException *e) {}
        [points addObject:[[AVAudioConnectionPoint alloc] initWithNode:dst bus:toBus]];
        if (engine.isRunning) {
            NSCEnsureAllocated(src);
            NSCEnsureAllocated(dst);
        }
        @try {
            [engine connect:src toConnectionPoints:points fromBus:fromBus format:format];
            ok = YES;
        } @catch (NSException *e) {
            // A running engine only fans out to mixer nodes, and an allocated unit keeps its
            // bus formats; a stopped engine takes either.
            if (engine.isRunning) {
                [context whileStopped:^{
                    @try {
                        [engine connect:src toConnectionPoints:points fromBus:fromBus format:format];
                        ok = YES;
                    } @catch (NSException *e2) {}
                }];
            }
            if (!ok) NSCLogError(@"NSCAudioNode: connect %@ -> %@ (%@ -> %@, %u ch to bus %u) threw: %@", NSStringFromClass([src class]), NSStringFromClass([dst class]), [src isKindOfClass:[AVAudioUnit class]] ? NSStringFromClass([((AVAudioUnit *)src).AUAudioUnit class]) : @"-", [dst isKindOfClass:[AVAudioUnit class]] ? NSStringFromClass([((AVAudioUnit *)dst).AUAudioUnit class]) : @"-", format.channelCount, toBus, e);
        }
    });
    return ok;
}

void NSCDisconnectRemoving(NSCAudioContext *context, AVAudioNode *src, NSNumber *fromBus, AVAudioNode *dst, NSNumber *toBus) {
    AVAudioEngine *engine = context.engine;
    if (!engine || !src || ![context isNode:src attachedToEngine:engine]) return;
    OnMain(^{
        NSUInteger first = fromBus ? fromBus.unsignedIntegerValue : 0;
        NSUInteger end = fromBus ? first + 1 : src.numberOfOutputs;
        for (NSUInteger bus = first; bus < end && bus < src.numberOfOutputs; bus++) {
            NSArray<AVAudioConnectionPoint *> *points = nil;
            @try { points = [engine outputConnectionPointsForNode:src outputBus:bus]; } @catch (NSException *e) { continue; }
            NSMutableArray<AVAudioConnectionPoint *> *keep = [NSMutableArray array];
            for (AVAudioConnectionPoint *point in points) {
                BOOL match = (!dst || point.node == dst) && (!toBus || point.bus == toBus.unsignedIntegerValue);
                if (!match) [keep addObject:point];
            }
            if (keep.count == points.count) continue;
            AVAudioFormat *format = nil;
            @try { format = [src outputFormatForBus:bus]; } @catch (NSException *e) {}
            WillConnect(context);
            @try {
                [engine disconnectNodeOutput:src bus:bus];
                if (keep.count > 0) [engine connect:src toConnectionPoints:keep fromBus:bus format:format];
            } @catch (NSException *e) {
                NSCLogError(@"NSCAudioNode: disconnect threw: %@", e);
            }
        }
    });
}

// A path of nodes from one of `starts` downstream to one of `targets`, or nil.
static NSArray<AVAudioNode *> *DownstreamPath(AVAudioEngine *engine, NSArray<AVAudioNode *> *starts, NSHashTable<AVAudioNode *> *targets) {
    NSMapTable<AVAudioNode *, id> *parent = [NSMapTable strongToStrongObjectsMapTable];
    NSMutableArray<AVAudioNode *> *stack = [NSMutableArray array];
    for (AVAudioNode *node in starts) {
        if ([parent objectForKey:node]) continue;
        [parent setObject:[NSNull null] forKey:node];
        [stack addObject:node];
    }
    while (stack.count > 0) {
        AVAudioNode *node = stack.lastObject;
        [stack removeLastObject];
        if ([targets containsObject:node]) {
            NSMutableArray<AVAudioNode *> *path = [NSMutableArray array];
            for (id at = node; at != [NSNull null]; at = [parent objectForKey:at]) [path insertObject:at atIndex:0];
            return path;
        }
        for (AVAudioNodeBus bus = 0; bus < node.numberOfOutputs; bus++) {
            NSArray<AVAudioConnectionPoint *> *points = nil;
            @try { points = [engine outputConnectionPointsForNode:node outputBus:bus]; } @catch (NSException *e) {}
            for (AVAudioConnectionPoint *point in points) {
                AVAudioNode *next = point.node;
                if (!next || [parent objectForKey:next]) continue;
                [parent setObject:node forKey:next];
                [stack addObject:next];
            }
        }
    }
    return nil;
}

BOOL NSCHasPath(AVAudioEngine *engine, AVAudioNode *from, AVAudioNode *to) {
    NSHashTable<AVAudioNode *> *targets = [NSHashTable hashTableWithOptions:NSPointerFunctionsObjectPointerPersonality];
    [targets addObject:to];
    return DownstreamPath(engine, @[ from ], targets) != nil;
}

static void HealCycles(NSCAudioContext *context) {
    for (NSCAudioNode *node in [context allNodeWrappersForHealing]) {
        if ([node isKindOfClass:[NSCParamEffectNode class]] && [(NSCParamEffectNode *)node isSplit]) [(NSCParamEffectNode *)node healCycle];
    }
}

static BOOL BreakAt(NSCAudioNode *node) {
    return [node isKindOfClass:[NSCDelayNode class]] && [(NSCDelayNode *)node breakCycle];
}

@implementation NSCAudioNode {
    NSCChannelMixerNode *_stage;
    NSMutableData *_logical;
}

- (const _Atomic UInt32 *)logicalChannelCellForOutput:(NSInteger)output {
    if (_stage) return [_stage mixedChannels];
    return _logical ? (const _Atomic UInt32 *)_logical.bytes : NULL;
}

- (void)setLogicalChannels:(NSInteger)channels {
    if (!_logical) {
        _logical = [NSMutableData dataWithLength:sizeof(_Atomic UInt32)];
        atomic_init((_Atomic UInt32 *)_logical.mutableBytes, 0);
    }
    atomic_store((_Atomic UInt32 *)_logical.mutableBytes, (UInt32)MAX(0, channels));
}

- (NSCChannelMixerNode *)channelPort { return nil; }

- (void)detachHelpers {}

- (AVAudioNode *)outputAVNode {
    return _stage ? _stage.avNode : self.avNode;
}

- (AVAudioNode *)outputNodeFor:(NSNumber *)output bus:(AVAudioNodeBus *)bus {
    *bus = (AVAudioNodeBus)MAX(0, output ? output.intValue : 0);
    return [self outputAVNode];
}

- (void)eachOutputNode:(NSNumber *)output block:(void (NS_NOESCAPE ^)(AVAudioNode *, NSNumber *))block {
    AVAudioNode *node = [self outputAVNode];
    if (node) block(node, output);
}

// An unconnected AVAudioSourceNode or mixer reports the engine's default format, not what it renders.
- (AVAudioFormat *)formatForOutput:(AVAudioNodeBus)bus {
    NSCAudioContext *ctx = self.context;
    AVAudioNode *node = [self outputAVNode];
    AVAudioFormat *format = nil;
    BOOL connected = NO;
    @try { connected = [ctx.engine outputConnectionPointsForNode:node outputBus:bus].count > 0; } @catch (NSException *e) {}
    if (connected || [node isKindOfClass:[AVAudioUnit class]]) {
        @try { format = [node outputFormatForBus:bus]; } @catch (NSException *e) {}
    } else {
        format = self.nativeFormat;
    }
    double rate = ctx.format.sampleRate;
    if (!format || format.channelCount == 0 || (rate > 0 && format.sampleRate != rate)) format = self.nativeFormat ?: ctx.format;
    return format ?: ctx.format;
}

- (void)replaceAVNode:(AVAudioNode *)node {
    self.avNode = node;
    if (self.context) [self.context registerNodeWrapper:self];
    if (_stage) [_stage connectMain:node bus:0 format:self.nativeFormat ?: self.context.format logical:_logical ? (const _Atomic UInt32 *)_logical.bytes : NULL];
}

- (BOOL)usesOutputStage { return NO; }

// Moves this node's connections onto a stage fed by it.
- (void)ensureOutputStageWithChannels:(NSInteger)channels {
    if (_stage) return;
    NSCAudioContext *ctx = self.context;
    AVAudioEngine *engine = ctx.engine;
    AVAudioNode *node = self.avNode;
    if (!engine || !node) return;
    NSCChannelMixerNode *stage = [[NSCChannelMixerNode alloc] initWithContext:ctx mode:NSCMixModePass channels:channels discrete:NO];
    if (!stage) return;
    NSArray<AVAudioConnectionPoint *> *points = @[];
    @try { points = [engine outputConnectionPointsForNode:node outputBus:0]; } @catch (NSException *e) {}
    AVAudioFormat *format = [self formatForOutput:0];
    if (points.count > 0) NSCDisconnectRemoving(ctx, node, @0, nil, nil);
    [stage connectMain:node bus:0 format:self.nativeFormat ?: ctx.format logical:_logical ? (const _Atomic UInt32 *)_logical.bytes : NULL];
    _stage = stage;
    if (points.count > 0) {
        OnMain(^{
            WillConnect(ctx);
            @try {
                [engine connect:stage.avNode toConnectionPoints:points fromBus:0 format:format];
            } @catch (NSException *e) {
                NSCLogError(@"NSCAudioNode: the output stage did not reconnect: %@", e);
            }
        });
    }
}

- (NSCAudioNode *)inputForParam:(NSCAudioParam *)param {
    if (param.inputNode) return param.inputNode;
    NSCAudioContext *ctx = self.context;
    if (!ctx) return nil;
    NSCAudioNode *input = NSCMakeParamInput(ctx, param);
    if (!input || !param.inputSink) return nil;
    [self attachParamSink:(AVAudioNode *)param.inputSink];
    return input;
}

// Sources render after the stage pulls the sink; other nodes read the param once per quantum, after it.
- (void)attachParamSink:(AVAudioNode *)sink {
    NSCAudioContext *ctx = self.context;
    if (self.usesOutputStage) {
        AVAudioFormat *format = self.nativeFormat ?: ctx.format;
        [self ensureOutputStageWithChannels:format.channelCount];
        if (_stage) {
            [_stage connectEarly:sink format:ctx.format];
            return;
        }
    }
    AVAudioMixerNode *mixer = ctx.engine.mainMixerNode;
    if (mixer) NSCConnectAdding(ctx, sink, 0, mixer, mixer.nextAvailableInputBus, ctx.format);
}

- (instancetype)initWithContext:(NSCAudioContext *)context node:(AVAudioNode *)node {
    if (self = [super init]) {
        _context = context;
        _avNode = node;
        _channelCount = 2;
        if (_context) {
            @try { [_context registerNodeWrapper:self]; } @catch (NSException *e) {}
        }
    }
    return self;
}

- (void)handleConnectFrom:(NSCAudioNode *)source
                   output:(NSNumber *)output
                    input:(NSNumber *)input {
  if (!source)
    return;
  AVAudioEngine *engine = self.context.engine ?: source.context.engine;
  if (!engine)
    return;
  AVAudioNodeBus fromBus = 0;
  AVAudioNode *sourceNode = [source outputNodeFor:output bus:&fromBus];
  AVAudioNode *destinationNode = self.avNode;
  if (!sourceNode || !destinationNode) {
    NSCLogDebug(
        @"NSCAudioNode: skip connect because source or destination AVAudioNode "
        @"is nil (source=%p source.avNode=%p dest=%p dest.avNode=%p)",
        source, sourceNode, self, destinationNode);
    return;
  }


  AVAudioNodeBus toBus = 0;
  BOOL multiInput = [self isKindOfClass:[NSCChannelMergerNode class]];
  if (input && (multiInput || ![destinationNode isKindOfClass:[AVAudioMixerNode class]])) {
    toBus = (AVAudioNodeBus)MAX(0, input.intValue);
  } else if ([destinationNode isKindOfClass:[AVAudioMixerNode class]]) {
    // A mixer-backed node has one input; each source takes its own bus so they sum.
    AVAudioMixerNode *m = (AVAudioMixerNode *)destinationNode;
    toBus = m.nextAvailableInputBus;
    AVAudioConnectionPoint *existing = nil;
    @try {
      for (AVAudioConnectionPoint *point in [engine outputConnectionPointsForNode:sourceNode outputBus:fromBus]) {
        if (point.node == destinationNode) { existing = point; break; }
      }
    } @catch (NSException *e) {}
    if (existing) toBus = existing.bus;
  }

  AVAudioConnectionPoint *destPoint =
      [[AVAudioConnectionPoint alloc] initWithNode:destinationNode bus:toBus];

  NSCAudioContext *ctx = self.context ?: source.context;
  BOOL sourceIsPlayer = [sourceNode isKindOfClass:[AVAudioPlayerNode class]];
  __block BOOL connectOk = NO;
  if ([ctx isKindOfClass:[NSCOfflineAudioContext class]]) {
    [(NSCOfflineAudioContext *)ctx willConnect];
  }
  NSCLogDebug(
      @"NSCAudioNode: connecting source=%@(avNode=%@) -> dest=%@(avNode=%@) "
      @"fromBus=%d toBus=%d",
      NSStringFromClass([source class]), NSStringFromClass([sourceNode class]),
      NSStringFromClass([self class]),
      NSStringFromClass([destinationNode class]), (int)fromBus, (int)toBus);
  if (sourceIsPlayer) {
    AVAudioNode *playerNode = sourceNode;
    void (^connectBlock)(void) = ^{
      @try {
        if (playerNode.engine == engine) {
          if (ctx) {
            @try {
              [ctx detachNode:playerNode fromEngine:engine];
            } @catch (NSException *e) {
            }
          } else {
            @try {
              [engine detachNode:playerNode];
            } @catch (NSException *e) {
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
        [engine connect:playerNode
                     to:destinationNode
                fromBus:fromBus
                  toBus:toBus
                 format:nil];
        connectOk = YES;
      } @catch (NSException *e) {
        NSCLogDebug(@"NSCAudioNode: player connect:to:format:nil threw: %@; "
                    @"trying ctx.format",
                    e);
        @try {
          [engine connect:playerNode
                       to:destinationNode
                  fromBus:fromBus
                    toBus:toBus
                   format:ctx ? ctx.format : nil];
          connectOk = YES;
        } @catch (NSException *e2) {
          NSCLogDebug(
              @"NSCAudioNode: player connect:to:format:ctx also threw: %@", e2);
        }
      }
    };
    if ([NSThread isMainThread])
      connectBlock();
    else
      dispatch_sync(dispatch_get_main_queue(), connectBlock);
  } else {
    void (^connectBlock)(void) = ^{
      // Offline, nil leaves mixers at stereo: mono graphs drop 3 dB and worklets go silent.
      // A channel mixer maps its channels to the format it is given with the speakers rules.
      BOOL mixesOutput = [sourceNode.AUAudioUnit isKindOfClass:NSClassFromString(@"NSCChannelMixUnit")];
      AVAudioFormat *firstFormat =
          (ctx && (engine.isInManualRenderingMode || mixesOutput)) ? ctx.format : nil;
      AVAudioFormat *sourceFormat = nil;
      @try {
        sourceFormat = [sourceNode outputFormatForBus:fromBus];
      } @catch (NSException *e) {
        sourceFormat = nil;
      }
      NSMutableArray<AVAudioConnectionPoint *> *targetPoints =
          [NSMutableArray array];
      SEL pointsSelector =
          NSSelectorFromString(@"outputConnectionPointsForNode:outputBus:");
      if (engine && [engine respondsToSelector:pointsSelector]) {
        @try {
          typedef NSArray *(*MsgSendFn)(id, SEL, id, AVAudioNodeBus);
          MsgSendFn fn = (MsgSendFn)objc_msgSend;
          NSArray *existingPoints =
              fn(engine, pointsSelector, sourceNode, fromBus);
          if (existingPoints.count > 0) {
            [targetPoints addObjectsFromArray:existingPoints];
          }
        } @catch (NSException *e) {
          NSCLogDebug(@"NSCAudioNode: read existing connection points threw: %@",
                      e);
        }
      }

      BOOL hasDestination = NO;
      for (AVAudioConnectionPoint *point in targetPoints) {
        if (point.node == destinationNode && point.bus == toBus) {
          hasDestination = YES;
          break;
        }
      }
      if (!hasDestination) {
        [targetPoints addObject:destPoint];
      }
      NSArray<AVAudioConnectionPoint *> *pointsToConnect =
          targetPoints.count > 0 ? [targetPoints copy] : @[ destPoint ];

      @try {
        [engine connect:sourceNode
            toConnectionPoints:pointsToConnect
                       fromBus:fromBus
                        format:firstFormat];
        connectOk = YES;
      } @catch (NSException *e) {
        NSCLogDebug(@"NSCAudioNode: connect with nil format threw: %@; trying "
                    @"ctx.format",
                    e);
        @try {
          [engine connect:sourceNode
              toConnectionPoints:pointsToConnect
                         fromBus:fromBus
                          format:ctx ? ctx.format : nil];
          connectOk = YES;
        } @catch (NSException *e2) {
          NSCLogDebug(@"NSCAudioNode: connect with ctx.format also threw: %@; "
                      @"retrying with destination-only point",
                      e2);
          @try {
            [engine connect:sourceNode
                toConnectionPoints:@[ destPoint ]
                           fromBus:fromBus
                            format:nil];
            connectOk = YES;
          } @catch (NSException *e3) {
            NSCLogDebug(@"NSCAudioNode: destination-only nil-format connect "
                        @"threw: %@; trying source format",
                        e3);
            @try {
              [engine connect:sourceNode
                  toConnectionPoints:@[ destPoint ]
                             fromBus:fromBus
                              format:sourceFormat];
              connectOk = YES;
            } @catch (NSException *e4) {
              NSCLogDebug(@"NSCAudioNode: destination-only source-format "
                          @"connect threw: %@; trying ctx.format",
                          e4);
              @try {
                [engine connect:sourceNode
                    toConnectionPoints:@[ destPoint ]
                               fromBus:fromBus
                                format:ctx ? ctx.format : nil];
                connectOk = YES;
              } @catch (NSException *e5) {
                NSCLogDebug(@"NSCAudioNode: destination-only connect also threw: %@",
                            e5);
              }
            }
          }
        }
      }
    };
    if ([NSThread isMainThread])
      connectBlock();
    else
      dispatch_sync(dispatch_get_main_queue(), connectBlock);
  }
  if (connectOk && sourceIsPlayer) {
    void (^prepareBlock)(void) = ^{
      @try {
        [engine prepare];
      } @catch (NSException *e) {
        NSCLogDebug(
            @"NSCAudioNode: engine prepare after player connect threw: %@", e);
      }
    };
    if ([NSThread isMainThread])
      prepareBlock();
    else
      dispatch_sync(dispatch_get_main_queue(), prepareBlock);
  }
  if (ctx) {
    NSValue *key = [NSValue valueWithPointer:(__bridge const void *)(source)];
    if ([self isKindOfClass:[NSCGainNode class]]) {
      NSMutableDictionary<NSNumber *, NSCAudioNode *> *map =
          ctx.voiceGainByOutput[key];
      if (!map)
        map = [NSMutableDictionary dictionary];
      map[@(output ? output.intValue : 0)] = self;
      ctx.voiceGainByOutput[key] = map;
    } else if ([self isKindOfClass:[NSCBiquadNode class]]) {
      NSMutableDictionary<NSNumber *, NSCAudioNode *> *map =
          ctx.voiceFilterByOutput[key];
      if (!map)
        map = [NSMutableDictionary dictionary];
      map[@(output ? output.intValue : 0)] = self;
      ctx.voiceFilterByOutput[key] = map;
    }

    @try {
      [source handleConnectedTo:self output:output input:(input ?: @(toBus))];
    } @catch (NSException *e) {
    }
  }
}

- (void)handleDisconnectFrom:(NSCAudioNode *)source output:(NSNumber *)output input:(NSNumber *)input {
    if (!source) return;
    NSCAudioContext *owner = self.context ?: source.context;
    // A mixer-backed node's buses are its own, not the input index.
    BOOL mixer = [self.avNode isKindOfClass:[AVAudioMixerNode class]];
    if (self.avNode) {
        [source eachOutputNode:output block:^(AVAudioNode *node, NSNumber *bus) {
            NSCDisconnectRemoving(owner, node, bus, self.avNode, mixer ? nil : input);
        }];
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
// AVAudioEngine can't render a cycle: one through a DelayNode splits it, any other is refused,
// which Web Audio's muting of a cycle without a delay matches.
- (BOOL)allowsConnectionTo:(NSCAudioNode *)destination {
    AVAudioEngine *engine = self.context.engine;
    if (!engine) return YES;
    NSMutableArray<AVAudioNode *> *starts = [NSMutableArray array];
    [destination eachOutputNode:nil block:^(AVAudioNode *node, NSNumber *bus) { [starts addObject:node]; }];
    NSHashTable<AVAudioNode *> *targets = [NSHashTable hashTableWithOptions:NSPointerFunctionsObjectPointerPersonality];
    [self eachOutputNode:nil block:^(AVAudioNode *node, NSNumber *bus) { [targets addObject:node]; }];
    if (self.avNode) [targets addObject:self.avNode];
    for (NSInteger attempt = 0; attempt < 16; attempt++) {
        NSArray<AVAudioNode *> *path = DownstreamPath(engine, starts, targets);
        if (!path) return YES;
        // The new connection's ends are on every cycle it closes.
        if (BreakAt(destination) || BreakAt(self)) return YES;
        BOOL broke = NO;
        for (AVAudioNode *node in path) {
            if (BreakAt([self.context nodeWrapperForAVNode:node])) { broke = YES; break; }
        }
        if (!broke) break;
    }
    NSCLogError(@"NSCAudioNode: %@ -> %@ would close a cycle without a DelayNode; not connected", NSStringFromClass([self class]), NSStringFromClass([destination class]));
    return NO;
}

- (void)connectTo:(NSCAudioNode *)destination {
    [self connectTo:destination output:nil input:nil];
}

- (void)connectTo:(NSCAudioNode *)destination output:(NSNumber *)output {
    [self connectTo:destination output:output input:nil];
}

- (void)connectTo:(NSCAudioNode *)destination output:(NSNumber *)output input:(NSNumber *)input {
    if (!destination || ![self allowsConnectionTo:destination]) return;
    [destination handleConnectFrom:self output:output input:input];
}

- (void)connectToParam:(NSCAudioParam *)param output:(NSNumber *)output {
    NSCAudioNode *input = [param.owner inputForParam:param];
    if (!input) {
        NSCLogDebug(@"NSCAudioNode: this AudioParam does not take audio inputs on iOS");
        return;
    }
    if (output) [self connectTo:input output:output];
    else [self connectTo:input];
}

- (void)disconnectFromParam:(NSCAudioParam *)param output:(NSNumber *)output {
    NSCAudioNode *input = param.inputNode;
    if (!input) return;
    if (output) [self disconnectTo:input output:output];
    else [self disconnectTo:input];
}

- (void)disconnect {
    [self eachOutputNode:nil block:^(AVAudioNode *node, NSNumber *bus) {
        NSCDisconnectRemoving(self.context, node, bus, nil, nil);
    }];
    HealCycles(self.context);
}

- (void)disconnectOutput:(NSNumber *)output {
    if (!output) return;
    [self eachOutputNode:output block:^(AVAudioNode *node, NSNumber *bus) {
        NSCDisconnectRemoving(self.context, node, bus, nil, nil);
    }];
    HealCycles(self.context);
}

- (void)disconnectTo:(NSCAudioNode *)destination {
    [self disconnectTo:destination output:nil input:nil];
}

- (void)disconnectTo:(NSCAudioNode *)destination output:(NSNumber *)output {
    [self disconnectTo:destination output:output input:nil];
}

- (void)disconnectTo:(NSCAudioNode *)destination output:(NSNumber *)output input:(NSNumber *)input {
    if (!destination) return;
    [destination handleDisconnectFrom:self output:output input:input];
    HealCycles(self.context);
}

- (void)setChannelConfig:(NSInteger)count mode:(NSInteger)mode interpretation:(NSInteger)interpretation {
    if (count >= 1) _channelCount = count;
    if (mode >= 0 && mode <= 2) _channelCountMode = mode;
    if (interpretation >= 0 && interpretation <= 1) _channelInterpretation = interpretation;
    [self channelConfigChanged];
}

- (void)channelConfigChanged {
    [[self channelPort] configureCount:self.channelCount mode:self.channelCountMode discrete:self.channelInterpretation == 1];
}

- (void)applyAutomation {}

@end
