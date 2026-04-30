#import <Accelerate/Accelerate.h>
#import "NSCAudioLog.h"
#import <objc/runtime.h>
#import <objc/message.h>
#import <stdatomic.h>
#import <math.h>
#import <AudioToolbox/AudioToolbox.h>

#import "NSCAudioContext.h"
#import "NSCMediaElementSourceTap.h"
#import "NSCMediaElementAudioSourceNode.h"

#import "NSCAudioSessionManager.h"
#import "NSCAnalyserNode.h"
#import "NSCAudioBufferSourceNode.h"
#import "NSCAudioPannerNode.h"
#import "NSCGainNode.h"
#import "NSCBiquadNode.h"
#import "NSCStereoPannerNode.h"
#import "NSCDelayNode.h"
#import "NSCConstantSourceNode.h"

static void *bufferKey = &bufferKey;
void *NSCProducerTokenKey = &NSCProducerTokenKey;

@interface NSCAudioContext ()
- (void)registerPendingNode:(NSCAudioBufferSourceNode *)node;
- (void)unregisterPendingNode:(NSCAudioBufferSourceNode *)node;
- (void)resumeAllPendingNodes;
- (void)ensureMainMixerConnectedToOutput;
@end

#import "NSCAudioContext.h"

static NSMapTable<AVAudioEngine *, NSCAudioContext *> *gEngineContextMap = nil;

static void NSCAudioContext_registerEngine(AVAudioEngine *engine, NSCAudioContext *ctx) {
    static dispatch_once_t onceToken;
    dispatch_once(&onceToken, ^{
        gEngineContextMap = [NSMapTable strongToWeakObjectsMapTable];
    });
    if (!engine) return;
    @synchronized (gEngineContextMap) { [gEngineContextMap setObject:ctx forKey:engine]; }
}

#import <dispatch/dispatch.h>
#import <CoreFoundation/CoreFoundation.h>

static NSMapTable<AVAudioEngine *, dispatch_block_t> *gScheduledResumeMap = nil;

void NSCAudioContext_cancelScheduledResume(AVAudioEngine *engine) {
    if (!engine) return;
    static dispatch_once_t onceToken;
    dispatch_once(&onceToken, ^{
        gScheduledResumeMap = [NSMapTable strongToStrongObjectsMapTable];
    });
    @synchronized (gScheduledResumeMap) {
        dispatch_block_t b = [gScheduledResumeMap objectForKey:engine];
        if (b) {
            dispatch_block_cancel(b);
            [gScheduledResumeMap removeObjectForKey:engine];
        }
    }
}

void NSCAudioContext_scheduleResumeOnEngineStart(AVAudioEngine *engine, double delay) {
    if (!engine) return;
    static dispatch_once_t onceToken;
    dispatch_once(&onceToken, ^{
        if (!gEngineContextMap) gEngineContextMap = [NSMapTable strongToWeakObjectsMapTable];
        if (!gScheduledResumeMap) gScheduledResumeMap = [NSMapTable strongToStrongObjectsMapTable];
    });

    @synchronized (gScheduledResumeMap) {
        dispatch_block_t prev = [gScheduledResumeMap objectForKey:engine];
        if (prev) {
            return;
        }

        __weak AVAudioEngine *weakEngine = engine;
        dispatch_block_t block = dispatch_block_create(0, ^{
            AVAudioEngine *eng = weakEngine;
            if (!eng) return;
            NSCAudioContext *ctx = nil;
            @synchronized (gEngineContextMap) { ctx = [gEngineContextMap objectForKey:eng]; }
            if (ctx) {
                [ctx resumeAllPendingNodes];
            }
            @synchronized (gScheduledResumeMap) { [gScheduledResumeMap removeObjectForKey:eng]; }
        });

        [gScheduledResumeMap setObject:block forKey:engine];
        dispatch_after(dispatch_time(DISPATCH_TIME_NOW, (int64_t)(delay * NSEC_PER_SEC)), dispatch_get_main_queue(), block);
    }
}


@implementation NSCAudioContext {
    AVAudioEngine *_engine;
    AVAudioEnvironmentNode *_environmentNode;
    NSCAudioNode *_destination;
    AVAudioFormat *_format;
    NSMutableSet<NSCAudioBufferSourceNode *> *_pendingNodes;
    dispatch_queue_t _pendingQueue;
    atomic_int _activeCount;
    NSMapTable<NSValue *, NSCAudioNode *> *_nodeWrappers;
    NSCAudioParam *_listenerPositionXParam;
    NSCAudioParam *_listenerPositionYParam;
    NSCAudioParam *_listenerPositionZParam;
    NSCAudioParam *_listenerForwardXParam;
    NSCAudioParam *_listenerForwardYParam;
    NSCAudioParam *_listenerForwardZParam;
    NSCAudioParam *_listenerUpXParam;
    NSCAudioParam *_listenerUpYParam;
    NSCAudioParam *_listenerUpZParam;
    dispatch_source_t _listenerAutomationTimer;
}

- (instancetype)initWithSampleRate:(double)sampleRate {
    return [self initWithSampleRate:sampleRate latencyHint:0.0];
}

- (instancetype)initWithSampleRate:(double)sampleRate latencyHint:(double)latencyHint {
    if (self = [super init]) {
        _engine = [[AVAudioEngine alloc] init];
        _environmentNode = nil;
        _destination = nil;
        _pendingNodes = [NSMutableSet set];
        _pendingQueue = dispatch_queue_create("nscaudio.pending", DISPATCH_QUEUE_SERIAL);
        atomic_init(&_activeCount, 0);

        _nodeWrappers = [NSMapTable strongToWeakObjectsMapTable];

        _format = [_engine.mainMixerNode outputFormatForBus:0];

        if (!_format || _format.sampleRate <= 0 || _format.channelCount == 0) {
            _format = [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32 sampleRate:(sampleRate > 0.0 ? sampleRate : 44100.0) channels:2 interleaved:NO];
        } else {
            const AudioStreamBasicDescription *asd = _format.streamDescription;
            if (!asd || asd->mFormatID != kAudioFormatLinearPCM) {
                _format = [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32 sampleRate:(_format.sampleRate > 0.0 ? _format.sampleRate : (sampleRate > 0.0 ? sampleRate : 44100.0)) channels:_format.channelCount interleaved:NO];
            }
        }

          NSCAudioContext_registerEngine(_engine, self);
        @try {
            if (_engine.mainMixerNode) {
                _destination = [[NSCAudioNode alloc] initWithContext:self node:_engine.mainMixerNode];
            }
        } @catch (NSException *e) {
            _destination = nil;
        }
        @try {
            [[NSCAudioSessionManager sharedManager] registerContext:self sampleRate:_format.sampleRate latencyHint:latencyHint];
        } @catch (NSException *e) {}

    }
    return self;
}

- (AVAudioEngine *)engine { return _engine; }
- (AVAudioEnvironmentNode *)environmentNode { return _environmentNode; }
- (NSCAudioNode *)destination { return _destination; }
- (double)sampleRate { return _format ? _format.sampleRate : 44100.0; }
- (double)currentTime {
    if (!_engine) return 0.0;
    AVAudioTime *now = _engine.outputNode.lastRenderTime;
    if (!now) return 0.0;
    return (double)now.sampleTime / now.sampleRate;
}
- (AVAudioFormat *)format { return _format; }

- (void)registerPendingNode:(NSCAudioBufferSourceNode *)node {
    if (!node) return;
    dispatch_async(_pendingQueue, ^{
        [_pendingNodes addObject:node];
    });
}

- (void)unregisterPendingNode:(NSCAudioBufferSourceNode *)node {
    if (!node) return;
    dispatch_async(_pendingQueue, ^{
        [_pendingNodes removeObject:node];
    });
}

- (void)resumeAllPendingNodes {
    __block NSSet<NSCAudioBufferSourceNode *> *copy = nil;
    dispatch_sync(_pendingQueue, ^{
        copy = [_pendingNodes copy];
        [_pendingNodes removeAllObjects];
    });
    for (NSCAudioBufferSourceNode *n in copy) {
        dispatch_async(dispatch_get_main_queue(), ^{
            @try { [n handleConnectedTo:nil output:nil input:nil]; } @catch (NSException *e) {}
        });
    }
}

- (void)ensureMainMixerConnectedToOutput {
    if (!_engine) return;
}

- (void)registerNodeWrapper:(NSCAudioNode *)node {
    if (!node || !node.avNode) return;
    NSCLogDebug(@"NSCAudioContext: registerNodeWrapper node=%p nodeClass=%@ avNode=%p avNodeClass=%@",
          node, NSStringFromClass([node class]), node.avNode, NSStringFromClass([node.avNode class]));
    NSValue *key = [NSValue valueWithPointer:(__bridge const void *)(node.avNode)];
    @synchronized(_nodeWrappers) {
        if (!_nodeWrappers) _nodeWrappers = [NSMapTable strongToWeakObjectsMapTable];
        [_nodeWrappers setObject:node forKey:key];
        NSUInteger cnt = _nodeWrappers ? [_nodeWrappers count] : 0;
        NSCLogDebug(@"NSCAudioContext: node wrappers count=%lu", (unsigned long)cnt);
    }
}

- (NSCAudioNode *)nodeWrapperForAVNode:(AVAudioNode *)avNode {
    if (!avNode) return nil;
    NSValue *key = [NSValue valueWithPointer:(__bridge const void *)(avNode)];
    @synchronized(_nodeWrappers) {
        NSCAudioNode *wrap = _nodeWrappers ? [_nodeWrappers objectForKey:key] : nil;
        NSCLogDebug(@"NSCAudioContext: nodeWrapperForAVNode avNode=%p class=%@ -> wrap=%p wrapClass=%@",
              avNode, NSStringFromClass([avNode class]), wrap, (wrap ? NSStringFromClass([wrap class]) : @"(nil)"));
        return wrap;
    }
}

- (NSArray<NSCAnalyserNode *> *)allAnalyserWrappers {
    NSMutableArray<NSCAnalyserNode *> *out = [NSMutableArray array];
    @synchronized(_nodeWrappers) {
        if (!_nodeWrappers) return out;
        NSEnumerator *en = [_nodeWrappers objectEnumerator];
        NSCAudioNode *n = nil;
        while ((n = [en nextObject])) {
            if (n && [n isKindOfClass:[NSCAnalyserNode class]]) {
                [out addObject:(NSCAnalyserNode *)n];
            }
        }
    }
    return out;
}

- (void)resume {
    if (!_engine) return;
    [NSCAudioContext startEngineWithRetry:_engine attempts:3 label:@"context" asyncCompletion:nil];
}

- (void)suspend {
    if (!_engine) return;
    @try { [_engine pause]; } @catch (NSException *e) {}
}

- (void)resumeAsync:(void (^)(BOOL))completion {
    if (!_engine) {
        if (completion) completion(NO);
        return;
    }

    CFRunLoopRef rl = completion ? CFRunLoopGetCurrent() : NULL;
    if (rl) CFRetain(rl);

    [NSCAudioContext startEngineWithRetry:_engine
                                 attempts:3
                                    label:@"context"
                          asyncCompletion:^(BOOL ok) {
        if (completion) {
            CFRunLoopPerformBlock(rl, kCFRunLoopCommonModes, ^{
                completion(ok);
                CFRelease(rl);
            });
            CFRunLoopWakeUp(rl);
        } else if (rl) {
            CFRelease(rl);
        }
    }];
}

- (void)suspendAsync:(void (^)(BOOL))completion {
    if (!_engine) {
        if (completion) completion(NO);
        return;
    }
    CFRunLoopRef rl = completion ? CFRunLoopGetCurrent() : NULL;
    if (rl) CFRetain(rl);
    AVAudioEngine *engine = _engine;
    dispatch_async(dispatch_get_global_queue(QOS_CLASS_USER_INITIATED, 0), ^{
        BOOL ok = YES;
        @try { [engine pause]; }
        @catch (NSException *e) { ok = NO; }
        if (completion) {
            CFRunLoopPerformBlock(rl, kCFRunLoopCommonModes, ^{
                completion(ok);
                CFRelease(rl);
            });
            CFRunLoopWakeUp(rl);
        } else if (rl) {
            CFRelease(rl);
        }
    });
}

- (void)closeAsync:(void (^)(void))completion {
    if (!_engine) {
        if (completion) completion();
        return;
    }
    CFRunLoopRef rl = completion ? CFRunLoopGetCurrent() : NULL;
    if (rl) CFRetain(rl);
    AVAudioEngine *engine = _engine;
    dispatch_async(dispatch_get_global_queue(QOS_CLASS_USER_INITIATED, 0), ^{
        @try { [engine pause]; } @catch (NSException *e) {}
        @try { [engine stop]; } @catch (NSException *e) {}
        if (completion) {
            CFRunLoopPerformBlock(rl, kCFRunLoopCommonModes, ^{
                completion();
                CFRelease(rl);
            });
            CFRunLoopWakeUp(rl);
        } else if (rl) {
            CFRelease(rl);
        }
    });
}

- (void)incrementActiveCount {
    int prev = atomic_fetch_add(&_activeCount, 1);
    if (prev == 0) {
        @try { [[NSCAudioSessionManager sharedManager] contextDidChangeActiveState:self]; } @catch (NSException *e) {}
    }
}

- (void)decrementActiveCount {
    int prev = atomic_fetch_sub(&_activeCount, 1);
    if (prev == 1) {
        @try { [[NSCAudioSessionManager sharedManager] contextDidChangeActiveState:self]; } @catch (NSException *e) {}
    }
}

- (BOOL)hasActiveAudio {
    return atomic_load(&_activeCount) > 0;
}

- (double)extraLatencySeconds { return 0.0; }


- (nullable NSCAudioBuffer *)_decodeAudioFileAtURL:(NSURL *)url {
    if (!url) return nil;
    NSError *err = nil;
    AVAudioFile *afile = [[AVAudioFile alloc] initForReading:url error:&err];
    if (!afile) {
        NSCLogError(@"NSCAudioContext: AVAudioFile init failed for %@: %@", url, err);
        return nil;
    }

    AVAudioFramePosition fLen = afile.length;
    if (fLen <= 0) {
        NSCLogDebug(@"NSCAudioContext: AVAudioFile has zero length: %@", url);
        return nil;
    }

    AVAudioFormat *fileFmt = afile.processingFormat;
    const AudioStreamBasicDescription *f_asbd = fileFmt ? fileFmt.streamDescription : NULL;
    if (!f_asbd || f_asbd->mFormatID != kAudioFormatLinearPCM) {
        fileFmt = [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32 sampleRate:afile.processingFormat.sampleRate channels:afile.processingFormat.channelCount interleaved:NO];
    }
    AVAudioFrameCount capacity = (AVAudioFrameCount)fLen;
    AVAudioPCMBuffer *srcBuf = [[AVAudioPCMBuffer alloc] initWithPCMFormat:fileFmt frameCapacity:capacity];
    NSError *readErr = nil;
    @try {
        [afile readIntoBuffer:srcBuf error:&readErr];
    } @catch (NSException *ex) {
        readErr = [NSError errorWithDomain:@"NSCAudioContext" code:-1 userInfo:@{NSLocalizedDescriptionKey: ex.reason ?: @"exception reading file"}];
    }
    if (readErr) {
        NSCLogError(@"NSCAudioContext: readIntoBuffer failed for %@: %@", url, readErr);
        return nil;
    }

    AVAudioPCMBuffer *outBuf = nil;
    if (fileFmt.commonFormat == AVAudioPCMFormatFloat32 && !fileFmt.isInterleaved) {
        outBuf = srcBuf;
    } else {
        AVAudioFormat *destFmt = [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32 sampleRate:fileFmt.sampleRate channels:fileFmt.channelCount interleaved:NO];
        AVAudioConverter *converter = [[AVAudioConverter alloc] initFromFormat:fileFmt toFormat:destFmt];
        if (!converter) {
            NSCLogError(@"NSCAudioContext: failed to create AVAudioConverter from %@ to %@", fileFmt, destFmt);
            return nil;
        }
        AVAudioPCMBuffer *destBuf = [[AVAudioPCMBuffer alloc] initWithPCMFormat:destFmt frameCapacity:srcBuf.frameLength];
        __block BOOL supplied = NO;
        AVAudioConverterInputBlock inputBlock = ^AVAudioBuffer * _Nullable(AVAudioPacketCount inNumberOfPackets, AVAudioConverterInputStatus *outStatus) {
            if (supplied) { *outStatus = AVAudioConverterInputStatus_NoDataNow; return nil; }
            supplied = YES;
            *outStatus = AVAudioConverterInputStatus_HaveData;
            return srcBuf;
        };
        NSError *convErr = nil;
        BOOL ok = [converter convertToBuffer:destBuf error:&convErr withInputFromBlock:inputBlock];
        if (!ok || convErr) {
            NSCLogError(@"NSCAudioContext: conversion failed: %@", convErr);
            return nil;
        }
        outBuf = destBuf;
    }

    NSCAudioBuffer *wrapper = [[NSCAudioBuffer alloc] initWithContext:self id:[[NSUUID UUID] UUIDString] buffer:outBuf];
    return wrapper;
}

- (BOOL)isNode:(AVAudioNode *)node attachedToEngine:(AVAudioEngine *)engine {
    if (!node || !engine) return NO;
    @try {
        if ([engine respondsToSelector:@selector(attachedNodes)]) {
            NSSet<AVAudioNode *> *attached = [engine attachedNodes];
            if (attached && [attached containsObject:node]) return YES;
        }
    } @catch (NSException *e) {}
    return node.engine == engine;
}

- (void)detachNode:(AVAudioNode *)node fromEngine:(AVAudioEngine *)engine {
    if (!node || !engine) return;
    if (![NSThread isMainThread]) {
        dispatch_sync(dispatch_get_main_queue(), ^{
            [self detachNode:node fromEngine:engine];
        });
        return;
    }
    @try {
        BOOL shouldDetach = NO;
        NSArray *attached = nil;
        if ([engine respondsToSelector:@selector(attachedNodes)]) {
            @try {
                attached = [engine attachedNodes];
                if (attached && [attached containsObject:node]) shouldDetach = YES;
            } @catch (NSException *e) {
                attached = nil;
            }
        }
        if (!shouldDetach) {
            if (node.engine == engine) shouldDetach = YES;
        }
        if (shouldDetach) {
            @try {
                NSCLogDebug(@"NSCAudioContext: detachNode: attempting detach node=%p engine=%p attachedCount=%lu node.engine=%p stack:%@",
                            node, engine, (unsigned long)(attached ? attached.count : 0), node.engine, [NSThread callStackSymbols]);
                [engine detachNode:node];
            } @catch (NSException *e) {
                NSCLogDebug(@"NSCAudioContext: detachNode: detach threw: %@\nstack:%@", e, [NSThread callStackSymbols]);
            }
        } else {
            NSCLogDebug(@"NSCAudioContext: detachNode: skip detach — node not attached to engine node=%p engine=%p node.engine=%p attachedCount=%lu stack:%@",
                        node, engine, node.engine, (unsigned long)(attached ? attached.count : 0), [NSThread callStackSymbols]);
        }
    } @catch (NSException *e) {
    }
}

- (nullable NSCAudioBuffer *)decodeAudioData:(NSString *)base64 {
    if (!base64) return nil;
    NSString *b64 = base64;
    NSRange comma = [b64 rangeOfString:@"," options:NSBackwardsSearch];
    if (comma.location != NSNotFound && [b64 hasPrefix:@"data:"]) {
        b64 = [b64 substringFromIndex:comma.location + 1];
    }
    NSData *data = [[NSData alloc] initWithBase64EncodedString:b64 options:NSDataBase64DecodingIgnoreUnknownCharacters];
    if (!data) return nil;
    return [self decodeAudioDataFromData:data];
}

- (nullable NSCAudioBuffer *)decodeAudioDataFromFile:(NSString *)path {
    if (!path) return nil;
    NSURL *url = nil;
    if ([path hasPrefix:@"file://"]) url = [NSURL URLWithString:path];
    else url = [NSURL fileURLWithPath:path];
    return [self _decodeAudioFileAtURL:url];
}

- (nullable NSCAudioBuffer *)decodeAudioDataFromData:(NSData *)data {
    if (!data || data.length == 0) return nil;
    NSString *tmpName = [NSTemporaryDirectory() stringByAppendingPathComponent:[NSString stringWithFormat:@"nsc_decode_%@.tmp", [[NSUUID UUID] UUIDString]]];
    BOOL ok = [data writeToFile:tmpName atomically:YES];
    if (!ok) return nil;
    NSURL *url = [NSURL fileURLWithPath:tmpName];
    NSCAudioBuffer *buf = [self _decodeAudioFileAtURL:url];
    @try { [[NSFileManager defaultManager] removeItemAtPath:tmpName error:NULL]; } @catch (NSException *e) {}
    return buf;
}

- (void)decodeAudioDataAsync:(NSString *)base64 :(void (^)(NSCAudioBuffer * _Nullable))completion {
    if (!completion) return;
    __weak typeof(self) weakSelf = self;
    dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
        __strong typeof(weakSelf) s = weakSelf;
        NSCAudioBuffer *b = [s decodeAudioData:base64];
        dispatch_async(dispatch_get_main_queue(), ^{ completion(b); });
    });
}

- (void)decodeAudioDataFromFileAsync:(NSString *)path :(void (^)(NSCAudioBuffer * _Nullable))completion {
    if (!completion) return;
    __weak typeof(self) weakSelf = self;
    dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
        __strong typeof(weakSelf) s = weakSelf;
        NSCAudioBuffer *b = [s decodeAudioDataFromFile:path];
        dispatch_async(dispatch_get_main_queue(), ^{ completion(b); });
    });
}

- (void)decodeAudioDataFromDataAsync:(NSData *)data :(void (^)(NSCAudioBuffer * _Nullable))completion {
    if (!completion) return;
    __weak typeof(self) weakSelf = self;
    dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
        __strong typeof(weakSelf) s = weakSelf;
        NSCAudioBuffer *b = [s decodeAudioDataFromData:data];
        dispatch_async(dispatch_get_main_queue(), ^{ completion(b); });
    });
}

- (NSCGainNode *)createGainNode { return [[NSCGainNode alloc] initWithContext:self]; }
- (NSCBiquadNode *)createBiquadNode:(nullable NSString *)type frequency:(double)frequency Q:(double)Q gain:(double)gain {
    NSString *t = type ? type : @"peaking";
    return [[NSCBiquadNode alloc] initWithContext:self type:t frequency:frequency Q:Q gain:gain detune:0.0];
}
- (NSCBiquadNode *)createBiquadNode { return [[NSCBiquadNode alloc] initWithContext:self]; }
- (NSCAudioPannerNode *)createPannerNode { return [[NSCAudioPannerNode alloc] initWithContext:self]; }
- (NSCAudioOscillatorNode *)createOscillatorNode:(nullable NSString *)type frequency:(double)frequency { return [[NSCAudioOscillatorNode alloc] initWithContext:self]; }
- (nullable NSCAudioBufferSourceNode *)createBufferSourceNode:(nullable NSCAudioBuffer *)buffer { return [[NSCAudioBufferSourceNode alloc] initWithContext:self buffer:buffer]; }
- (NSCStereoPannerNode *)createStereoPannerNode:(double)pan { return [[NSCStereoPannerNode alloc] initWithContext:self pan:pan]; }
- (NSCDelayNode *)createDelayNode:(double)delayTime maxDelayTime:(double)maxDelayTime { return [[NSCDelayNode alloc] initWithContext:self delayTime:delayTime maxDelayTime:maxDelayTime]; }
- (NSCConstantSourceNode *)createConstantSourceNode:(double)offset { return [[NSCConstantSourceNode alloc] initWithContext:self offset:offset]; }
- (NSCAnalyserNode *)createAnalyserNode { return [[NSCAnalyserNode alloc] initWithContext:self]; }
- (NSCWaveShaperNode *)createWaveShaperNode { return [[NSCWaveShaperNode alloc] initWithContext:self]; }
- (NSCIIRFilterNode *)createIIRFilterNode:(NSArray<NSNumber *> *)feedforward feedback:(NSArray<NSNumber *> *)feedback { return [[NSCIIRFilterNode alloc] initWithContext:self feedforward:feedforward feedback:feedback]; }
- (NSCConvolverNode *)createConvolverNode { return [[NSCConvolverNode alloc] initWithContext:self]; }

- (nullable NSCAudioNode *)createSourceNodeFromMediaPlayer:(AVPlayer *)player {
    if (!player) return nil;
    NSCMediaElementSourceTap *tap = [NSCMediaElementSourceTap attachToPlayer:player context:self];
    if (!tap) return nil;
    NSCMediaElementAudioSourceNode *wrapper = [[NSCMediaElementAudioSourceNode alloc] initWithContext:self node:tap.sourceNode tap:tap];
    [self registerNodeWrapper:wrapper];
    return wrapper;
}

- (void)detachSource:(NSCAudioNode *)source {
    if (!source) return;
    @try {
        if ([source respondsToSelector:@selector(detach)]) {
            @try { [(id)source detach]; } @catch (NSException *ex) { NSCLogDebug(@"NSCAudioContext: detachSource: source.detach threw: %@", ex); }
        } else {
            id tap = objc_getAssociatedObject(source, "NSCMediaElementSourceTap");
            if (tap && [tap respondsToSelector:@selector(detach)]) {
                @try { [(NSCMediaElementSourceTap *)tap detach]; } @catch (NSException *ex) { NSCLogDebug(@"NSCAudioContext: detachSource: tap detach threw: %@", ex); }
            }
            objc_setAssociatedObject(source, "NSCMediaElementSourceTap", nil, OBJC_ASSOCIATION_ASSIGN);
        }

        AVAudioNode *av = source.avNode;
        if (av && av.engine) {
            @try { [self detachNode:av fromEngine:av.engine]; } @catch (NSException *ex) { NSCLogDebug(@"NSCAudioContext: detachSource: detachNode threw: %@", ex); }
        }
        if (av) {
            NSValue *key = [NSValue valueWithPointer:(__bridge const void *)(av)];
            @synchronized(_nodeWrappers) {
                if (_nodeWrappers) [_nodeWrappers removeObjectForKey:key];
            }
        }
    } @catch (NSException *e) {}
}

- (void)ensureEnvironmentNodeAttached {
    if (_environmentNode) return;
    if (!_engine) return;
    @try {
        _environmentNode = [[AVAudioEnvironmentNode alloc] init];
        [_engine attachNode:_environmentNode];
    } @catch (NSException *e) {
        _environmentNode = nil;
    }
}

- (void)setListenerPosition:(double)x :(double)y :(double)z {
    [self ensureEnvironmentNodeAttached];
    if (!_environmentNode) return;
    if (_listenerPositionXParam) [_listenerPositionXParam setValue:@(x)];
    if (_listenerPositionYParam) [_listenerPositionYParam setValue:@(y)];
    if (_listenerPositionZParam) [_listenerPositionZParam setValue:@(z)];
    dispatch_async(dispatch_get_main_queue(), ^{
        @try {
            _environmentNode.listenerPosition = AVAudioMake3DPoint(x, y, z);
        } @catch (NSException *e) {}
    });
}

- (void)setListenerOrientation:(double)forwardX :(double)forwardY :(double)forwardZ :(double)upX :(double)upY :(double)upZ {
    [self ensureEnvironmentNodeAttached];
    if (!_environmentNode) return;
    if (_listenerForwardXParam) [_listenerForwardXParam setValue:@(forwardX)];
    if (_listenerForwardYParam) [_listenerForwardYParam setValue:@(forwardY)];
    if (_listenerForwardZParam) [_listenerForwardZParam setValue:@(forwardZ)];
    if (_listenerUpXParam) [_listenerUpXParam setValue:@(upX)];
    if (_listenerUpYParam) [_listenerUpYParam setValue:@(upY)];
    if (_listenerUpZParam) [_listenerUpZParam setValue:@(upZ)];
    dispatch_async(dispatch_get_main_queue(), ^{
        @try {
            AVAudio3DVector f = AVAudioMake3DVector(forwardX, forwardY, forwardZ);
            AVAudio3DVector u = AVAudioMake3DVector(upX, upY, upZ);
            _environmentNode.listenerVectorOrientation = AVAudioMake3DVectorOrientation(f, u);
        } @catch (NSException *e) {}
    });
}

#pragma mark - Listener automation params

- (NSCAudioParam *)getListenerPositionXParam {
    if (!_listenerPositionXParam) {
        _listenerPositionXParam = [[NSCAudioParam alloc] initWithContext:self defaultValue:0.0];
        __weak typeof(self) weakSelf = self;
        _listenerPositionXParam.onScheduleChanged = ^(NSCAudioParam *p) {
            __strong typeof(weakSelf) s = weakSelf;
            if (!s) return;
            [s ensureListenerAutomationLink];
        };
    }
    return _listenerPositionXParam;
}

- (NSCAudioParam *)getListenerPositionYParam {
    if (!_listenerPositionYParam) {
        _listenerPositionYParam = [[NSCAudioParam alloc] initWithContext:self defaultValue:0.0];
        __weak typeof(self) weakSelf = self;
        _listenerPositionYParam.onScheduleChanged = ^(NSCAudioParam *p) {
            __strong typeof(weakSelf) s = weakSelf;
            if (!s) return;
            [s ensureListenerAutomationLink];
        };
    }
    return _listenerPositionYParam;
}

- (NSCAudioParam *)getListenerPositionZParam {
    if (!_listenerPositionZParam) {
        _listenerPositionZParam = [[NSCAudioParam alloc] initWithContext:self defaultValue:0.0];
        __weak typeof(self) weakSelf = self;
        _listenerPositionZParam.onScheduleChanged = ^(NSCAudioParam *p) {
            __strong typeof(weakSelf) s = weakSelf;
            if (!s) return;
            [s ensureListenerAutomationLink];
        };
    }
    return _listenerPositionZParam;
}

- (NSCAudioParam *)getListenerForwardXParam {
    if (!_listenerForwardXParam) {
        _listenerForwardXParam = [[NSCAudioParam alloc] initWithContext:self defaultValue:0.0];
        __weak typeof(self) weakSelf = self;
        _listenerForwardXParam.onScheduleChanged = ^(NSCAudioParam *p) {
            __strong typeof(weakSelf) s = weakSelf;
            if (!s) return;
            [s ensureListenerAutomationLink];
        };
    }
    return _listenerForwardXParam;
}

- (NSCAudioParam *)getListenerForwardYParam {
    if (!_listenerForwardYParam) {
        _listenerForwardYParam = [[NSCAudioParam alloc] initWithContext:self defaultValue:0.0];
        __weak typeof(self) weakSelf = self;
        _listenerForwardYParam.onScheduleChanged = ^(NSCAudioParam *p) {
            __strong typeof(weakSelf) s = weakSelf;
            if (!s) return;
            [s ensureListenerAutomationLink];
        };
    }
    return _listenerForwardYParam;
}

- (NSCAudioParam *)getListenerForwardZParam {
    if (!_listenerForwardZParam) {
        _listenerForwardZParam = [[NSCAudioParam alloc] initWithContext:self defaultValue:-1.0];
        __weak typeof(self) weakSelf = self;
        _listenerForwardZParam.onScheduleChanged = ^(NSCAudioParam *p) {
            __strong typeof(weakSelf) s = weakSelf;
            if (!s) return;
            [s ensureListenerAutomationLink];
        };
    }
    return _listenerForwardZParam;
}

- (NSCAudioParam *)getListenerUpXParam {
    if (!_listenerUpXParam) {
        _listenerUpXParam = [[NSCAudioParam alloc] initWithContext:self defaultValue:0.0];
        __weak typeof(self) weakSelf = self;
        _listenerUpXParam.onScheduleChanged = ^(NSCAudioParam *p) {
            __strong typeof(weakSelf) s = weakSelf;
            if (!s) return;
            [s ensureListenerAutomationLink];
        };
    }
    return _listenerUpXParam;
}

- (NSCAudioParam *)getListenerUpYParam {
    if (!_listenerUpYParam) {
        _listenerUpYParam = [[NSCAudioParam alloc] initWithContext:self defaultValue:1.0];
        __weak typeof(self) weakSelf = self;
        _listenerUpYParam.onScheduleChanged = ^(NSCAudioParam *p) {
            __strong typeof(weakSelf) s = weakSelf;
            if (!s) return;
            [s ensureListenerAutomationLink];
        };
    }
    return _listenerUpYParam;
}

- (NSCAudioParam *)getListenerUpZParam {
    if (!_listenerUpZParam) {
        _listenerUpZParam = [[NSCAudioParam alloc] initWithContext:self defaultValue:0.0];
        __weak typeof(self) weakSelf = self;
        _listenerUpZParam.onScheduleChanged = ^(NSCAudioParam *p) {
            __strong typeof(weakSelf) s = weakSelf;
            if (!s) return;
            [s ensureListenerAutomationLink];
        };
    }
    return _listenerUpZParam;
}

- (BOOL)_listenerHasFutureEvents {
    double t = self.currentTime;
    if (_listenerPositionXParam && [_listenerPositionXParam hasEventsAfter:t]) return YES;
    if (_listenerPositionYParam && [_listenerPositionYParam hasEventsAfter:t]) return YES;
    if (_listenerPositionZParam && [_listenerPositionZParam hasEventsAfter:t]) return YES;
    if (_listenerForwardXParam && [_listenerForwardXParam hasEventsAfter:t]) return YES;
    if (_listenerForwardYParam && [_listenerForwardYParam hasEventsAfter:t]) return YES;
    if (_listenerForwardZParam && [_listenerForwardZParam hasEventsAfter:t]) return YES;
    if (_listenerUpXParam && [_listenerUpXParam hasEventsAfter:t]) return YES;
    if (_listenerUpYParam && [_listenerUpYParam hasEventsAfter:t]) return YES;
    if (_listenerUpZParam && [_listenerUpZParam hasEventsAfter:t]) return YES;
    return NO;
}

- (void)ensureListenerAutomationLink {
    if ([NSThread isMainThread] == NO) {
        __weak typeof(self) weakSelf = self;
        dispatch_async(dispatch_get_main_queue(), ^{ __strong typeof(weakSelf) s = weakSelf; if (s) [s ensureListenerAutomationLink]; });
        return;
    }
    if (_listenerAutomationTimer) return;
    if (![self _listenerHasFutureEvents]) return;
    uint64_t intervalNs = (uint64_t)(NSEC_PER_SEC / 60.0);
    uint64_t leeway = (uint64_t)(NSEC_PER_SEC / 240.0);
    dispatch_source_t timer = dispatch_source_create(DISPATCH_SOURCE_TYPE_TIMER, 0, 0, dispatch_get_main_queue());
    dispatch_source_set_timer(timer, dispatch_time(DISPATCH_TIME_NOW, 0), intervalNs, leeway);
    __weak typeof(self) weakSelf = self;
    dispatch_source_set_event_handler(timer, ^{
        __strong typeof(weakSelf) s = weakSelf;
        if (!s) return;
        [s _applyListenerAutomationOnce];
        if (![s _listenerHasFutureEvents]) [s stopListenerAutomationLink];
    });
    _listenerAutomationTimer = timer;
    dispatch_resume(_listenerAutomationTimer);
}

- (void)stopListenerAutomationLink {
    if (!_listenerAutomationTimer) return;
    dispatch_source_cancel(_listenerAutomationTimer);
    _listenerAutomationTimer = nil;
}

- (void)_applyListenerAutomationOnce {
    double t = self.currentTime;
    double px = _listenerPositionXParam ? [_listenerPositionXParam valueAtTime:t] : 0.0;
    double py = _listenerPositionYParam ? [_listenerPositionYParam valueAtTime:t] : 0.0;
    double pz = _listenerPositionZParam ? [_listenerPositionZParam valueAtTime:t] : 0.0;
    double fx = _listenerForwardXParam ? [_listenerForwardXParam valueAtTime:t] : 0.0;
    double fy = _listenerForwardYParam ? [_listenerForwardYParam valueAtTime:t] : 0.0;
    double fz = _listenerForwardZParam ? [_listenerForwardZParam valueAtTime:t] : -1.0;
    double ux = _listenerUpXParam ? [_listenerUpXParam valueAtTime:t] : 0.0;
    double uy = _listenerUpYParam ? [_listenerUpYParam valueAtTime:t] : 1.0;
    double uz = _listenerUpZParam ? [_listenerUpZParam valueAtTime:t] : 0.0;

    [self ensureEnvironmentNodeAttached];
    if (!_environmentNode) return;
    @try {
        _environmentNode.listenerPosition = AVAudioMake3DPoint(px, py, pz);
        AVAudio3DVector f = AVAudioMake3DVector(fx, fy, fz);
        AVAudio3DVector u = AVAudioMake3DVector(ux, uy, uz);
        _environmentNode.listenerVectorOrientation = AVAudioMake3DVectorOrientation(f, u);
    } @catch (NSException *e) {}
}


+ (BOOL)startEngineWithRetry:(AVAudioEngine *)engine
                    attempts:(NSInteger)attempts
                       label:(NSString *)label
             asyncCompletion:(nullable void (^)(BOOL))completion {
    if (!engine) {
        if (completion) completion(NO);
        return NO;
    }
    __block NSError *err = nil;
    __block BOOL ok = NO;
    @try {
        if ([NSThread isMainThread]) {
            ok = [engine startAndReturnError:&err];
        } else {
            dispatch_sync(dispatch_get_main_queue(), ^{
                NSError *innerErr = nil;
                ok = [engine startAndReturnError:&innerErr];
                if (innerErr) err = innerErr;
            });
        }
        if (ok) {
            if (completion) dispatch_async(dispatch_get_main_queue(), ^{ completion(YES); });
            return YES;
        }
        AVAudioTime *lrt = nil;
        @try { lrt = engine.outputNode.lastRenderTime; } @catch (NSException *e) { lrt = nil; }
        if (lrt) {
            NSCLogDebug(@"NSCAudioContext: engine start failed initial attempt: %@ (label=%@) engRunning=%d lastRenderTime=%p sampleTime=%lld sampleRate=%f",
                  err, label ?: @"<nil>", (int)engine.isRunning, lrt, (long long)lrt.sampleTime, lrt.sampleRate);
        } else {
            NSCLogDebug(@"NSCAudioContext: engine start failed initial attempt: %@ (label=%@) engRunning=%d lastRenderTime=NULL",
                  err, label ?: @"<nil>", (int)engine.isRunning);
        }
    } @catch (NSException *e) {
        NSCLogDebug(@"NSCAudioContext: engine start initial attempt threw exception: %@ (label=%@)", e, label ?: @"<nil>");
    }

    if (attempts <= 0) {
        if (completion) dispatch_async(dispatch_get_main_queue(), ^{ completion(NO); });
        return NO;
    }

    __block int remaining = (int)attempts;
    __block double delay = 0.02;
    __weak AVAudioEngine *weakEngine = engine;
    __block void (^attemptBlock)(void) = nil;
    attemptBlock = ^{
        AVAudioEngine *eng = weakEngine;
        if (!eng) { if (completion) completion(NO); return; }
        __block NSError *e = nil;
        __block BOOL ok2 = NO;
        @try {
            if ([NSThread isMainThread]) {
                ok2 = [eng startAndReturnError:&e];
            } else {
                dispatch_sync(dispatch_get_main_queue(), ^{
                    NSError *inner = nil;
                    ok2 = [eng startAndReturnError:&inner];
                    if (inner) e = inner;
                });
            }
            if (ok2) { if (completion) completion(YES); return; }
            AVAudioTime *lrt = nil;
            @try { lrt = eng.outputNode.lastRenderTime; } @catch (NSException *ex) { lrt = nil; }
            if (lrt) {
                NSCLogDebug(@"NSCAudioContext: engine start retry failed: %@ (label=%@, remaining=%d) engRunning=%d lastRenderTime=%p sampleTime=%lld sampleRate=%f",
                      e, label ?: @"<nil>", remaining, (int)eng.isRunning, lrt, (long long)lrt.sampleTime, lrt.sampleRate);
            } else {
                NSCLogDebug(@"NSCAudioContext: engine start retry failed: %@ (label=%@, remaining=%d) engRunning=%d lastRenderTime=NULL",
                      e, label ?: @"<nil>", remaining, (int)eng.isRunning);
            }
        } @catch (NSException *ex) {
            NSCLogDebug(@"NSCAudioContext: engine start retry threw exception: %@ (label=%@, remaining=%d)", ex, label ?: @"<nil>", remaining);
        }
        remaining -= 1;
        if (remaining <= 0) { if (completion) completion(NO); return; }
        delay = MIN(1.0, delay * 2.0);
        dispatch_after(dispatch_time(DISPATCH_TIME_NOW, (int64_t)(delay * NSEC_PER_SEC)), dispatch_get_main_queue(), attemptBlock);
    };
    dispatch_after(dispatch_time(DISPATCH_TIME_NOW, (int64_t)(delay * NSEC_PER_SEC)), dispatch_get_main_queue(), attemptBlock);
    return NO;
}

+ (NSMutableData *)marshalMutableData:(NSMutableData *)data { return data; }

- (void)dealloc {
    @try { [[NSCAudioSessionManager sharedManager] unregisterContext:self]; } @catch (NSException *e) {}
}

#pragma mark - setSinkId

- (BOOL)setSinkId:(NSString *)deviceId {
    AVAudioSession *session = [AVAudioSession sharedInstance];
    NSError *err = nil;

    NSString *normalized = deviceId.length > 0 ? deviceId : @"default";

    if ([normalized isEqualToString:@"default"] || [normalized isEqualToString:@""]) {
        BOOL ok = [session overrideOutputAudioPort:AVAudioSessionPortOverrideNone error:&err];
        if (!ok) NSCLogError(@"NSCAudioContext: setSinkId(default) failed: %@", err);
        return ok;
    }

    if ([normalized isEqualToString:@"speaker"]) {
        BOOL ok = [session overrideOutputAudioPort:AVAudioSessionPortOverrideSpeaker error:&err];
        if (!ok) NSCLogError(@"NSCAudioContext: setSinkId(speaker) failed: %@", err);
        return ok;
    }

    for (AVAudioSessionPortDescription *p in session.currentRoute.outputs) {
        if ([p.UID isEqualToString:normalized]) {
            BOOL ok = [session overrideOutputAudioPort:AVAudioSessionPortOverrideNone error:&err];
            if (!ok) NSCLogError(@"NSCAudioContext: setSinkId(uid=%@) failed: %@", normalized, err);
            return ok;
        }
    }
    NSCLogError(@"NSCAudioContext: setSinkId(%@) — UID not present in current route", normalized);
    return NO;
}

- (NSString *)currentSinkId {
    AVAudioSession *session = [AVAudioSession sharedInstance];
    AVAudioSessionPortDescription *p = session.currentRoute.outputs.firstObject;
    if (!p) return @"default";
    if ([p.portType isEqualToString:AVAudioSessionPortBuiltInSpeaker]) return @"speaker";
    return p.UID ?: @"default";
}


@end
