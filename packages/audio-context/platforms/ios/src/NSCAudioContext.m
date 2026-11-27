#import <Accelerate/Accelerate.h>
#import "NSCAudioLog.h"
#import <objc/runtime.h>
#import <objc/message.h>
#import <stdatomic.h>
#import <sched.h>
#import <math.h>
#import <AudioToolbox/AudioToolbox.h>

#import "NSCAudioContext.h"
#import "NSCAudioFileStreamDecoder.h"
#import <AudioContextNative/AudioContextNative.h>
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
#import "NSCDynamicsCompressorNode.h"
#import "NSCChannelSplitterNode.h"
#import "NSCChannelMergerNode.h"
#import "AudioModule.h"
#import "NSCAudioWorkletNode.h"
#import "NSCChannelMixerNode.h"

static void *bufferKey = &bufferKey;

// Web Audio's render quantum.
static const AVAudioFrameCount NSCQuantum = 128;
#define NSC_DRIVER_CHANNELS 2

// Shared with the pump's render block: one rendered quantum and how much of it is left.
typedef struct {
    _Atomic int64_t frames;
    // Set while the engine is reconfigured; the pump plays silence and time holds.
    _Atomic int hold;
    _Atomic int rendering;
    UInt32 left;
    float quantum[NSC_DRIVER_CHANNELS][128];
    AudioBufferList list;
    AudioBuffer second;
} NSCDriver;
void *NSCProducerTokenKey = &NSCProducerTokenKey;

@interface NSCAudioContext ()
- (void)registerPendingNode:(NSCAudioBufferSourceNode *)node;
- (void)unregisterPendingNode:(NSCAudioBufferSourceNode *)node;
- (void)resumeAllPendingNodes;
- (void)ensureMainMixerConnectedToOutput;
- (void)adoptFormat:(AVAudioFormat *)format;
- (NSArray<NSCAudioNode *> *)allNodeWrappers;
@end

#import "NSCAudioContext.h"

static NSMapTable<AVAudioEngine *, NSCAudioContext *> *gEngineContextMap = nil;

static void NSCAudioContext_registerEngine(AVAudioEngine *engine, NSCAudioContext *ctx) {
    static dispatch_once_t onceToken;
    dispatch_once(&onceToken, ^{
        gEngineContextMap = [NSMapTable weakToWeakObjectsMapTable];
    });
    if (!engine) return;
    @synchronized (gEngineContextMap) { [gEngineContextMap setObject:ctx forKey:engine]; }
}

#import <dispatch/dispatch.h>
#import <CoreFoundation/CoreFoundation.h>

static NSMapTable<AVAudioEngine *, dispatch_block_t> *gScheduledResumeMap = nil;

static void NSCAudioContext_prepareAudioSessionForPlayback(void) {
    AVAudioSession *session = [AVAudioSession sharedInstance];
    if (!session) return;

    void (^configure)(void) = ^{
        // DefaultToSpeaker / AllowBluetoothA2DP / AllowAirPlay are invalid with
        // AVAudioSessionCategoryPlayback (OSStatus -50 BadParam); preserve only valid mixing flags.
        AVAudioSessionCategoryOptions options = 0;
        if ([session.category isEqualToString:AVAudioSessionCategoryPlayback]) {
            AVAudioSessionCategoryOptions validPlaybackMask = AVAudioSessionCategoryOptionMixWithOthers | AVAudioSessionCategoryOptionDuckOthers;
            options = session.categoryOptions & validPlaybackMask;
        }
        NSError *catErr = nil;
        BOOL catOK = (options != 0)
            ? [session setCategory:AVAudioSessionCategoryPlayback withOptions:options error:&catErr]
            : [session setCategory:AVAudioSessionCategoryPlayback error:&catErr];
        if (!catOK) {
            NSCLogDebug(@"NSCAudioContext: prepare session setCategory failed: %@", catErr);
        }

        NSError *actErr = nil;
        if (![session setActive:YES error:&actErr]) {
            NSCLogDebug(@"NSCAudioContext: prepare session setActive failed: %@", actErr);
        }
    };

    if ([NSThread isMainThread]) {
        configure();
    } else {
        dispatch_sync(dispatch_get_main_queue(), configure);
    }
}

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
        if (!gEngineContextMap) gEngineContextMap = [NSMapTable weakToWeakObjectsMapTable];
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
    NSMutableData *_timelineCell;
    id _configurationObserver;
    NSCAudioNode *_listenerOwner;
    // Realtime: the engine renders manually, a quantum at a time, pulled by _output's pump.
    AVAudioEngine *_output;
    AVAudioSourceNode *_pump;
    NSMutableData *_driverCell;
    id _outputObserver;
    BOOL _userSuspended;
    BOOL _closed;
}

- (instancetype)initWithSampleRate:(double)sampleRate {
    return [self initWithSampleRate:sampleRate latencyHint:0.0];
}

- (instancetype)initWithSampleRate:(double)sampleRate latencyHint:(double)latencyHint {
    if (self = [super init]) {
        @try {
            [AudioModule install];
        } @catch (NSException *e) {}

        _engine = [[AVAudioEngine alloc] init];
        _environmentNode = nil;
        _destination = nil;
        _pendingNodes = [NSMutableSet set];
        _pendingQueue = dispatch_queue_create("nscaudio.pending", DISPATCH_QUEUE_SERIAL);
        atomic_init(&_activeCount, 0);

        _nodeWrappers = [NSMapTable strongToWeakObjectsMapTable];

        BOOL offline = [self isKindOfClass:[NSCOfflineAudioContext class]];
        double hardwareRate = 0.0;
        @try { hardwareRate = [_engine.outputNode outputFormatForBus:0].sampleRate; } @catch (NSException *e) {}
        double rate = sampleRate >= 3000.0 && sampleRate <= 768000.0 ? sampleRate : (hardwareRate > 0.0 ? hardwareRate : 48000.0);
        _format = [[AVAudioFormat alloc] initStandardFormatWithSampleRate:rate channels:2];
        if (!offline) [self attachDriver];

          NSCAudioContext_registerEngine(_engine, self);
        _timelineCell = [NSMutableData dataWithLength:NSCTimelineSlots * sizeof(double)];
        [self refreshTimelineRate];
        __weak NSCAudioContext *weakSelf = self;
        _configurationObserver = [[NSNotificationCenter defaultCenter] addObserverForName:AVAudioEngineConfigurationChangeNotification object:_engine queue:nil usingBlock:^(NSNotification *note) {
            [weakSelf refreshTimelineRate];
        }];
        if (![self isKindOfClass:[NSCOfflineAudioContext class]]) {
            @try {
                [[NSCAudioSessionManager sharedManager] registerContext:self sampleRate:_format.sampleRate latencyHint:latencyHint];
            } @catch (NSException *e) {}
        }

    }
    return self;
}

// Connections to the destination mix with Web Audio's rules before the engine's mixer.
- (void)makeDestination {
    AVAudioMixerNode *main = _engine.mainMixerNode;
    if (!main) return;
    NSInteger channels = _format.channelCount;
    if (channels <= 8) {
        NSCChannelMixerNode *port = [[NSCChannelMixerNode alloc] initWithContext:self mode:NSCMixModeSum channels:channels discrete:NO];
        if (port && NSCConnectAdding(self, port.avNode, 0, main, main.nextAvailableInputBus, _format)) {
            _destination = port;
            return;
        }
    }
    _destination = [[NSCAudioNode alloc] initWithContext:self node:main];
}

- (AVAudioEngine *)engine { return _engine; }
- (AVAudioEnvironmentNode *)environmentNode { return _environmentNode; }
- (NSCAudioNode *)destination { return _destination; }
- (double)sampleRate { return _format ? _format.sampleRate : 44100.0; }
- (BOOL)isOffline { return NO; }
- (double)currentTime {
    if (!_driverCell) return 0.0;
    NSCDriver *d = (NSCDriver *)_driverCell.mutableBytes;
    return (double)atomic_load_explicit(&d->frames, memory_order_acquire) / self.sampleRate;
}
// One quantum buffered between the engine and the device, plus the device buffer.
- (double)baseLatency {
    return NSCQuantum / self.sampleRate + [AVAudioSession sharedInstance].IOBufferDuration;
}
- (double)outputLatency {
    // Hardware output latency reported by the OS.
    return [AVAudioSession sharedInstance].outputLatency;
}
- (AVAudioFormat *)format { return _format; }
- (void)adoptFormat:(AVAudioFormat *)format { _format = format; }

- (void)registerPendingNode:(NSCAudioBufferSourceNode *)node {
    if (!node) return;
    dispatch_async(_pendingQueue, ^{
        [self->_pendingNodes addObject:node];
    });
}

- (void)unregisterPendingNode:(NSCAudioBufferSourceNode *)node {
    if (!node) return;
    dispatch_async(_pendingQueue, ^{
        [self->_pendingNodes removeObject:node];
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

- (NSArray<NSCAudioNode *> *)allNodeWrappersForHealing { return [self allNodeWrappers]; }

- (NSArray<NSCAudioNode *> *)allNodeWrappers {
    @synchronized(_nodeWrappers) {
        return _nodeWrappers ? [[_nodeWrappers objectEnumerator] allObjects] : @[];
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

#pragma mark - Realtime driver

// The device pulls the pump, which renders the engine a quantum at a time, so every node
// sees 128-frame render calls and context frames count from 0, as in Web Audio.
- (void)attachDriver {
    NSError *err = nil;
    if (![_engine enableManualRenderingMode:AVAudioEngineManualRenderingModeRealtime format:_format maximumFrameCount:NSCQuantum error:&err]) {
        NSCLogError(@"NSCAudioContext: realtime manual rendering failed: %@", err);
        return;
    }
    @try { [_engine connect:_engine.mainMixerNode to:_engine.outputNode format:_format]; } @catch (NSException *e) {}
    _driverCell = [NSMutableData dataWithLength:sizeof(NSCDriver)];
    NSMutableData *cell = _driverCell;
    AVAudioEngineManualRenderingBlock render = _engine.manualRenderingBlock;
    _pump = [[AVAudioSourceNode alloc] initWithFormat:_format renderBlock:^OSStatus(BOOL *isSilence, const AudioTimeStamp *ts, AVAudioFrameCount frames, AudioBufferList *out) {
        NSCDriver *d = (NSCDriver *)cell.mutableBytes;
        UInt32 channels = MIN(out->mNumberBuffers, (UInt32)NSC_DRIVER_CHANNELS);
        AVAudioFrameCount done = 0;
        atomic_store_explicit(&d->rendering, 1, memory_order_seq_cst);
        while (done < frames && !atomic_load_explicit(&d->hold, memory_order_seq_cst)) {
            if (d->left == 0) {
                d->list.mNumberBuffers = NSC_DRIVER_CHANNELS;
                for (UInt32 c = 0; c < NSC_DRIVER_CHANNELS; c++) {
                    d->list.mBuffers[c].mNumberChannels = 1;
                    d->list.mBuffers[c].mData = d->quantum[c];
                    d->list.mBuffers[c].mDataByteSize = NSCQuantum * sizeof(float);
                }
                OSStatus status = noErr;
                if (render(NSCQuantum, &d->list, &status) != AVAudioEngineManualRenderingStatusSuccess) break;
                for (UInt32 c = 0; c < NSC_DRIVER_CHANNELS; c++) {
                    if (d->list.mBuffers[c].mData != d->quantum[c]) memcpy(d->quantum[c], d->list.mBuffers[c].mData, NSCQuantum * sizeof(float));
                }
                d->left = NSCQuantum;
                atomic_fetch_add_explicit(&d->frames, NSCQuantum, memory_order_release);
            }
            AVAudioFrameCount n = MIN(frames - done, d->left);
            for (UInt32 c = 0; c < channels; c++) {
                memcpy((float *)out->mBuffers[c].mData + done, d->quantum[c] + (NSCQuantum - d->left), n * sizeof(float));
            }
            d->left -= n;
            done += n;
        }
        atomic_store_explicit(&d->rendering, 0, memory_order_release);
        for (UInt32 c = 0; c < out->mNumberBuffers; c++) {
            if (done < frames) memset((float *)out->mBuffers[c].mData + done, 0, (frames - done) * sizeof(float));
            if (c >= channels) memcpy(out->mBuffers[c].mData, out->mBuffers[0].mData, frames * sizeof(float));
        }
        return noErr;
    }];
    _output = [[AVAudioEngine alloc] init];
    [_output attachNode:_pump];
    [_output connect:_pump to:_output.mainMixerNode format:_format];
    [self makeDestination];
    __weak NSCAudioContext *weakSelf = self;
    _outputObserver = [[NSNotificationCenter defaultCenter] addObserverForName:AVAudioEngineConfigurationChangeNotification object:_output queue:[NSOperationQueue mainQueue] usingBlock:^(NSNotification *note) {
        [weakSelf startDriver];
    }];
    [self startDriver];
}

- (void)whileStopped:(void (NS_NOESCAPE ^)(void))block {
    NSCDriver *d = _driverCell ? (NSCDriver *)_driverCell.mutableBytes : NULL;
    if (d) {
        atomic_store(&d->hold, 1);
        while (atomic_load(&d->rendering)) sched_yield();
    }
    BOOL wasRunning = _engine.isRunning;
    @try { [_engine stop]; } @catch (NSException *e) {}
    block();
    NSError *err = nil;
    if (wasRunning && ![_engine startAndReturnError:&err]) NSCLogError(@"NSCAudioContext: the engine did not restart: %@", err);
    if (d) atomic_store(&d->hold, 0);
}

- (BOOL)startDriver {
    if (!_output || _closed) return NO;
    NSError *err = nil;
    if (!_engine.isRunning && ![_engine startAndReturnError:&err]) {
        NSCLogError(@"NSCAudioContext: the engine did not start: %@", err);
        return NO;
    }
    if (_userSuspended || _output.isRunning) return YES;
    NSCAudioContext_prepareAudioSessionForPlayback();
    if (![_output startAndReturnError:&err]) {
        NSCLogError(@"NSCAudioContext: the output did not start: %@", err);
        return NO;
    }
    return YES;
}

- (void)resume {
    if (!_engine) return;
    _userSuspended = NO;
    [NSCAudioContext startEngineWithRetry:_engine attempts:3 label:@"context" asyncCompletion:nil];
}

- (void)suspend {
    if (!_engine) return;
    _userSuspended = YES;
    @try { [_output pause]; } @catch (NSException *e) {}
}

- (void)resumeAsync:(void (^)(BOOL))completion {
    if (!_engine) {
        if (completion) completion(NO);
        return;
    }

    _userSuspended = NO;
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
    _userSuspended = YES;
    AVAudioEngine *engine = _output ?: _engine;
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
    _closed = YES;
    AVAudioEngine *engine = _engine;
    AVAudioEngine *output = _output;
    if (_outputObserver) [[NSNotificationCenter defaultCenter] removeObserver:_outputObserver];
    _outputObserver = nil;
    dispatch_async(dispatch_get_global_queue(QOS_CLASS_USER_INITIATED, 0), ^{
        @try { [output stop]; } @catch (NSException *e) {}
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
        NSSet<AVAudioNode *> *attached = nil;
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

    NSString *lower = [path.lowercaseString stringByTrimmingCharactersInSet:[NSCharacterSet whitespaceAndNewlineCharacterSet]];
    if ([lower hasSuffix:@".ogg"] || [lower hasSuffix:@".opus"]) {
        AVAudioPCMBuffer *pcm = [NSCOpusDecoder decodeOpusFile:path];
        if (pcm) return [[NSCAudioBuffer alloc] initWithContext:self id:[[NSUUID UUID] UUIDString] buffer:pcm];
    }
    NSURL *url = nil;
    if ([path hasPrefix:@"file://"]) url = [NSURL URLWithString:path];
    else url = [NSURL fileURLWithPath:path];
    return [self _decodeAudioFileAtURL:url];
}


- (nullable NSCAudioBuffer *)decodeAudioDataFromData:(NSData *)data {
    if (!data || data.length == 0) return nil;

    @autoreleasepool {


    if (data.length > 4) {
        const uint8_t *bytes = (const uint8_t *)data.bytes;
        if (bytes[0] == 'O' && bytes[1] == 'g' && bytes[2] == 'g' && bytes[3] == 'S') {
            AVAudioPCMBuffer *pcm = [NSCOpusDecoder decodeOpusData:data];
            if (pcm) return [[NSCAudioBuffer alloc] initWithContext:self id:[[NSUUID UUID] UUIDString] buffer:pcm];

            NSData *d = data;
            NSData *vorbisNeedle = [NSData dataWithBytes:"vorbis" length:6];
            NSRange searchRange = NSMakeRange(0, (NSUInteger)MIN((size_t)1024, d.length));
            NSRange found = [d rangeOfData:vorbisNeedle options:0 range:searchRange];
            if (found.location != NSNotFound) {
                AVAudioPCMBuffer *vpcm = [NSCVorbisDecoder decodeVorbisData:data];
                if (vpcm) return [[NSCAudioBuffer alloc] initWithContext:self id:[[NSUUID UUID] UUIDString] buffer:vpcm];
            }
        }
    }

    NSError *streamErr = nil;
    AVAudioPCMBuffer *streamPCM = [NSCAudioFileStreamDecoder decodeDataToPCMBuffer:data error:&streamErr];
    if (streamPCM) {
        NSCAudioBuffer *wrapper = [[NSCAudioBuffer alloc] initWithContext:self id:[[NSUUID UUID] UUIDString] buffer:streamPCM];
        return wrapper;
    }

    NSString *tmpName = [NSTemporaryDirectory() stringByAppendingPathComponent:[NSString stringWithFormat:@"nsc_decode_%@.tmp", [[NSUUID UUID] UUIDString]]];
    BOOL ok = [data writeToFile:tmpName atomically:YES];
    if (!ok) return nil;
    NSURL *url = [NSURL fileURLWithPath:tmpName];
    NSCAudioBuffer *buf = [self _decodeAudioFileAtURL:url];
    @try { [[NSFileManager defaultManager] removeItemAtPath:tmpName error:NULL]; } @catch (NSException *e) {}
    return buf;
    }
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
    NSData *retainedData = [data copy];
    dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
        @autoreleasepool {
        __strong typeof(weakSelf) s = weakSelf;
        NSCAudioBuffer *b = [s decodeAudioDataFromData:retainedData];
        dispatch_async(dispatch_get_main_queue(), ^{ completion(b); });
        }
    });
}

- (NSCGainNode *)createGainNode { return [[NSCGainNode alloc] initWithContext:self]; }
- (NSCBiquadNode *)createBiquadNode:(nullable NSString *)type frequency:(double)frequency Q:(double)Q gain:(double)gain {
    NSString *t = type ? type : @"peaking";
    return [[NSCBiquadNode alloc] initWithContext:self type:t frequency:frequency Q:Q gain:gain detune:0.0];
}
- (NSCBiquadNode *)createBiquadNode { return [[NSCBiquadNode alloc] initWithContext:self]; }
- (NSCAudioPannerNode *)createPannerNode { return [[NSCAudioPannerNode alloc] initWithContext:self]; }
- (NSCAudioOscillatorNode *)createOscillatorNode:(nullable NSString *)type frequency:(double)frequency {
    NSString *resolvedType = (type.length > 0) ? type : @"sine";
    double resolvedFreq = (isfinite(frequency) && frequency > 0.0) ? frequency : 440.0;
    return [[NSCAudioOscillatorNode alloc] initWithContext:self type:resolvedType frequency:resolvedFreq];
}
- (nullable NSCAudioBufferSourceNode *)createBufferSourceNode:(nullable NSCAudioBuffer *)buffer { return [[NSCAudioBufferSourceNode alloc] initWithContext:self buffer:buffer]; }
- (NSCStereoPannerNode *)createStereoPannerNode:(double)pan { return [[NSCStereoPannerNode alloc] initWithContext:self pan:pan]; }
- (NSCDelayNode *)createDelayNode:(double)delayTime maxDelayTime:(double)maxDelayTime { return [[NSCDelayNode alloc] initWithContext:self delayTime:delayTime maxDelayTime:maxDelayTime]; }
- (NSCConstantSourceNode *)createConstantSourceNode:(double)offset { return [[NSCConstantSourceNode alloc] initWithContext:self offset:offset]; }
- (NSCAnalyserNode *)createAnalyserNode { return [[NSCAnalyserNode alloc] initWithContext:self]; }
- (NSCWaveShaperNode *)createWaveShaperNode { return [[NSCWaveShaperNode alloc] initWithContext:self]; }
- (NSCIIRFilterNode *)createIIRFilterNode:(NSArray<NSNumber *> *)feedforward feedback:(NSArray<NSNumber *> *)feedback { return [[NSCIIRFilterNode alloc] initWithContext:self feedforward:feedforward feedback:feedback]; }
- (NSCConvolverNode *)createConvolverNode { return [[NSCConvolverNode alloc] initWithContext:self]; }
- (NSCDynamicsCompressorNode *)createDynamicsCompressorNode { return [[NSCDynamicsCompressorNode alloc] initWithContext:self]; }
- (NSCChannelSplitterNode *)createChannelSplitterNode:(NSInteger)numberOfOutputs { return [[NSCChannelSplitterNode alloc] initWithContext:self numberOfOutputs:numberOfOutputs]; }
- (NSCChannelMergerNode *)createChannelMergerNode:(NSInteger)numberOfInputs { return [[NSCChannelMergerNode alloc] initWithContext:self numberOfInputs:numberOfInputs]; }

- (nullable NSCAudioNode *)createSourceNodeFromMediaPlayer:(AVPlayer *)player {
    if (!player) return nil;
    NSCMediaElementSourceTap *tap = [NSCMediaElementSourceTap attachToPlayer:player context:self];
    if (!tap) return nil;
    AVAudioNode *node = tap.outputNode ?: tap.sourceNode;
    if (!node) {
        @try { [tap detach]; } @catch (NSException *ex) {}
        return nil;
    }
    NSCMediaElementAudioSourceNode *wrapper = [[NSCMediaElementAudioSourceNode alloc] initWithContext:self node:node tap:tap];
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
    if (![NSThread isMainThread]) {
        dispatch_sync(dispatch_get_main_queue(), ^{
            [self ensureEnvironmentNodeAttached];
        });
        return;
    }
    @try {
        _environmentNode = [[AVAudioEnvironmentNode alloc] init];
        [_engine attachNode:_environmentNode];

        @try {
            AVAudioMixerNode *mainMixer = _engine.mainMixerNode;
            AVAudioFormat *fmt = nil;
            @try { fmt = [mainMixer outputFormatForBus:0]; } @catch (NSException *inner) { fmt = _format; }
            if (!fmt) fmt = _format;
            if (mainMixer) {
                @try {
                    [_engine connect:_environmentNode to:mainMixer format:fmt];
                } @catch (NSException *e2) {
                    @try { [_engine connect:_environmentNode to:mainMixer format:nil]; } @catch (NSException *e3) {}
                }
            }
        } @catch (NSException *e) {}
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
            self->_environmentNode.listenerPosition = AVAudioMake3DPoint(x, y, z);
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
            self->_environmentNode.listenerVectorOrientation = AVAudioMake3DVectorOrientation(f, u);
        } @catch (NSException *e) {}
    });
}

#pragma mark - Listener automation params

- (NSCAudioNode *)listenerOwner {
    if (!_listenerOwner) _listenerOwner = NSCMakeListenerOwner(self);
    return _listenerOwner;
}

- (NSCAudioParam *)getListenerPositionXParam {
    if (!_listenerPositionXParam) {
        _listenerPositionXParam = [[NSCAudioParam alloc] initWithContext:self defaultValue:0.0];
        _listenerPositionXParam.owner = [self listenerOwner];
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
        _listenerPositionYParam.owner = [self listenerOwner];
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
        _listenerPositionZParam.owner = [self listenerOwner];
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
        _listenerForwardXParam.owner = [self listenerOwner];
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
        _listenerForwardYParam.owner = [self listenerOwner];
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
        _listenerForwardZParam.owner = [self listenerOwner];
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
        _listenerUpXParam.owner = [self listenerOwner];
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
        _listenerUpYParam.owner = [self listenerOwner];
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
        _listenerUpZParam.owner = [self listenerOwner];
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
    if (engine.isInManualRenderingMode && engine.manualRenderingMode == AVAudioEngineManualRenderingModeRealtime) {
        __block NSCAudioContext *ctx = nil;
        @synchronized (gEngineContextMap) { ctx = [gEngineContextMap objectForKey:engine]; }
        __block BOOL ok = NO;
        void (^start)(void) = ^{ ok = [ctx startDriver]; };
        if ([NSThread isMainThread]) start(); else dispatch_sync(dispatch_get_main_queue(), start);
        if (completion) dispatch_async(dispatch_get_main_queue(), ^{ completion(ok); });
        return ok;
    }
    // An offline engine runs only while it renders.
    if (engine.isInManualRenderingMode) {
        BOOL running = engine.isRunning;
        if (completion) dispatch_async(dispatch_get_main_queue(), ^{ completion(running); });
        return running;
    }
    __block NSError *err = nil;
    __block BOOL ok = NO;
    @try {
        if (!engine.isInManualRenderingMode) NSCAudioContext_prepareAudioSessionForPlayback();
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
    __block __weak void (^weakAttemptBlock)(void) = nil;
    attemptBlock = ^{
        AVAudioEngine *eng = weakEngine;
        if (!eng) { if (completion) completion(NO); return; }
        __block NSError *e = nil;
        __block BOOL ok2 = NO;
        @try {
            if (!eng.isInManualRenderingMode) NSCAudioContext_prepareAudioSessionForPlayback();
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
        void (^nextAttempt)(void) = weakAttemptBlock;
        if (!nextAttempt) { if (completion) completion(NO); return; }
        dispatch_after(dispatch_time(DISPATCH_TIME_NOW, (int64_t)(delay * NSEC_PER_SEC)), dispatch_get_main_queue(), nextAttempt);
    };
    weakAttemptBlock = attemptBlock;
    if (weakAttemptBlock) {
        dispatch_after(dispatch_time(DISPATCH_TIME_NOW, (int64_t)(delay * NSEC_PER_SEC)), dispatch_get_main_queue(), weakAttemptBlock);
    }
    return NO;
}

+ (NSMutableData *)marshalMutableData:(NSMutableData *)data { return data; }

- (void)dealloc {
    if (_configurationObserver) [[NSNotificationCenter defaultCenter] removeObserver:_configurationObserver];
    if (_outputObserver) [[NSNotificationCenter defaultCenter] removeObserver:_outputObserver];
    @try { [_output stop]; } @catch (NSException *e) {}
    @try { [_engine stop]; } @catch (NSException *e) {}
    @try { [[NSCAudioSessionManager sharedManager] unregisterContext:self]; } @catch (NSException *e) {}
}

#pragma mark - Render timeline

- (NSMutableData *)timelineCell { return _timelineCell; }

static void UpdateTimelineOffset(double *timeline, double contextRate) {
    double device = timeline[NSCTimelineDeviceRate];
    timeline[NSCTimelineOffset] = device > 0.0 && contextRate > 0.0 ? timeline[NSCTimelineStart] * (1.0 - contextRate / device) : 0.0;
}

- (void)refreshTimelineRate {
    AVAudioEngine *engine = _engine;
    if (!engine || !_timelineCell) return;
    double *timeline = (double *)_timelineCell.mutableBytes;
    timeline[NSCTimelineDeviceRate] = engine.isInManualRenderingMode ? 0.0 : [engine.outputNode outputFormatForBus:0].sampleRate;
    UpdateTimelineOffset(timeline, self.sampleRate);
}

- (int64_t)frameForTime:(double)when {
    double rate = self.sampleRate;
    double now = self.currentTime;
    return (int64_t)llround(MAX(when, now) * rate);
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
#if TARGET_OS_TV
        // tvOS has no built-in speaker to override to; output follows the HDMI/AirPlay route.
        NSCLogError(@"NSCAudioContext: setSinkId(speaker) is unavailable on tvOS");
        return NO;
#else
        BOOL ok = [session overrideOutputAudioPort:AVAudioSessionPortOverrideSpeaker error:&err];
        if (!ok) NSCLogError(@"NSCAudioContext: setSinkId(speaker) failed: %@", err);
        return ok;
#endif
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

#pragma mark - NSCOfflineAudioContext

// One render quantum per renderOffline call, so automation and worklets advance per quantum.
static const AVAudioFrameCount NSCOfflineQuantum = 128;
// Main-thread time per turn before yielding to the run loop.
static const CFTimeInterval NSCOfflineSliceSeconds = 0.008;
// A worklet whose Worker answers nothing for this long renders silence.
static const CFTimeInterval NSCOfflineWorkletStallSeconds = 5.0;

@implementation NSCOfflineAudioContext {
    AVAudioFrameCount _length;
    BOOL _started;
    AVAudioPCMBuffer *_rendered;
    AVAudioPCMBuffer *_chunk;
    CFAbsoluteTime _lastProgress;
    void (^_completion)(NSCAudioBuffer * _Nullable);
    NSMutableIndexSet *_suspends;
    BOOL _paused;
    BOOL _finished;
}

- (instancetype)init {
    return [super initWithSampleRate:44100.0 latencyHint:0.0];
}

- (void)configure:(NSNumber *)channels :(NSNumber *)lengthInFrames :(NSNumber *)sampleRate {
    AVAudioEngine *engine = self.engine;
    if (!engine || _started) return;
    AVAudioChannelCount count = (AVAudioChannelCount)MAX(1, MIN(32, channels.integerValue));
    double rate = sampleRate.doubleValue > 0.0 ? sampleRate.doubleValue : 44100.0;
    _length = (AVAudioFrameCount)MAX(0LL, MIN((long long)UINT32_MAX, lengthInFrames.longLongValue));
    AVAudioFormat *format = count <= 2
        ? [[AVAudioFormat alloc] initStandardFormatWithSampleRate:rate channels:count]
        : [[AVAudioFormat alloc] initStandardFormatWithSampleRate:rate channelLayout:[AVAudioChannelLayout layoutWithLayoutTag:kAudioChannelLayoutTag_DiscreteInOrder | count]];
    @try { [engine stop]; } @catch (NSException *e) {}
    NSError *err = nil;
    if (![engine enableManualRenderingMode:AVAudioEngineManualRenderingModeOffline format:format maximumFrameCount:NSCOfflineQuantum error:&err]) {
        NSCLogError(@"NSCOfflineAudioContext: enableManualRenderingMode failed: %@", err);
        return;
    }
    // The mixer keeps the hardware format it was created with otherwise.
    [engine connect:engine.mainMixerNode to:engine.outputNode format:format];
    [self adoptFormat:format];
    [self makeDestination];
    [self refreshTimelineRate];
}

- (NSInteger)length { return _length; }
- (BOOL)isOffline { return YES; }

- (NSInteger)scheduleSuspend:(double)time {
    double rate = self.sampleRate;
    if (!isfinite(time) || time < 0.0 || rate <= 0.0) return NSCOfflineSuspendErrorInvalidTime;
    if (_finished) return NSCOfflineSuspendErrorFinished;
    // The epsilon keeps an exact boundary from rounding up a quantum.
    NSUInteger frame = (NSUInteger)MAX(0.0, ceil(time * rate / NSCOfflineQuantum - 1e-9)) * NSCOfflineQuantum;
    if (frame >= _length) return NSCOfflineSuspendErrorPastEnd;
    NSUInteger current = _started ? _rendered.frameLength : 0;
    if (frame < current || (_started && frame <= current)) return NSCOfflineSuspendErrorNotAfterCurrent;
    if (!_suspends) _suspends = [NSMutableIndexSet indexSet];
    if ([_suspends containsIndex:frame]) return NSCOfflineSuspendErrorDuplicate;
    [_suspends addIndex:frame];
    return (NSInteger)frame;
}

// An initialized mixer keeps a new input at its default format (44.1 kHz stereo: resampled,
// +3 dB), so a render stops the engine before a connect and restarts it for the next quantum.
// Stopping keeps the sample time but clears effect state (delay lines, filters, reverb tails).
- (void)whileStopped:(void (NS_NOESCAPE ^)(void))block {
    [self willConnect];
    block();
}

- (void)willConnect {
    // A paused engine is still initialized.
    if (_started && !_finished) {
        @try { [self.engine stop]; } @catch (NSException *e) {}
    }
}

- (void)resumeRendering {
    if (!_paused || _finished) return;
    _paused = NO;
    // Time spent held is not a worklet stall.
    _lastProgress = CFAbsoluteTimeGetCurrent();
    dispatch_async(dispatch_get_main_queue(), ^{ [self renderSlice]; });
}

- (double)currentTime {
    AVAudioEngine *engine = self.engine;
    double rate = self.sampleRate;
    if (!engine.isInManualRenderingMode || rate <= 0.0) return 0.0;
    return (double)engine.manualRenderingSampleTime / rate;
}

- (double)baseLatency { return 0.0; }
- (double)outputLatency { return 0.0; }

// Nothing here touches the audio session.
- (void)incrementActiveCount {}
- (void)decrementActiveCount {}
- (BOOL)hasActiveAudio { return NO; }

- (void)decodeAudioDataFromFileAsyncOffline:(NSString *)path :(void (^)(NSCAudioBuffer * _Nullable))completion {
    [self decodeAudioDataFromFileAsync:path :completion];
}

- (void)decodeAudioDataFromDataAsyncOffline:(NSData *)data :(void (^)(NSCAudioBuffer * _Nullable))completion {
    [self decodeAudioDataFromDataAsync:data :completion];
}

- (nullable NSCAudioBufferSourceNode *)createBufferSourceNodeOffline:(nullable NSCAudioBuffer *)buffer {
    return [self createBufferSourceNode:buffer];
}

- (void)startRenderingAsync:(void (^)(NSCAudioBuffer * _Nullable))completion {
    AVAudioEngine *engine = self.engine;
    if (_started || !engine.isInManualRenderingMode) {
        if (completion) completion(nil);
        return;
    }
    _started = YES;
    _completion = [completion copy];
    _rendered = [[AVAudioPCMBuffer alloc] initWithPCMFormat:engine.manualRenderingFormat frameCapacity:MAX(_length, 1u)];
    _rendered.frameLength = 0;
    _chunk = [[AVAudioPCMBuffer alloc] initWithPCMFormat:engine.manualRenderingFormat frameCapacity:engine.manualRenderingMaximumFrameCount];
    NSError *err = nil;
    if (!_rendered || !_chunk || (!engine.isRunning && ![engine startAndReturnError:&err])) {
        NSCLogError(@"NSCOfflineAudioContext: could not start rendering: %@", err);
        [self finishRendering:NO];
        return;
    }
    _lastProgress = CFAbsoluteTimeGetCurrent();
    dispatch_async(dispatch_get_main_queue(), ^{ [self renderSlice]; });
}

// Renders on the main thread, where nodes apply automation, a quantum at a time.
- (void)renderSlice {
    if (_paused || !_rendered) return;
    AVAudioEngine *engine = self.engine;
    NSError *startErr = nil;
    if (!engine.isRunning && ![engine startAndReturnError:&startErr]) {
        NSCLogError(@"NSCOfflineAudioContext: could not restart rendering: %@", startErr);
        [self finishRendering:NO];
        return;
    }
    CFAbsoluteTime deadline = CFAbsoluteTimeGetCurrent() + NSCOfflineSliceSeconds;
    while (_rendered.frameLength < _length) {
        if ([_suspends containsIndex:_rendered.frameLength]) {
            [_suspends removeIndex:_rendered.frameLength];
            _paused = YES;
            @try { [engine pause]; } @catch (NSException *e) {}
            if (self.onSuspend) self.onSuspend((NSInteger)_rendered.frameLength);
            return;
        }
        NSArray<NSCAudioNode *> *nodes = [self allNodeWrappers];
        // Worklets don't run past a suspend, where the graph may change.
        NSUInteger next = _suspends ? [_suspends indexGreaterThanIndex:_rendered.frameLength] : NSNotFound;
        uint64_t limit = next == NSNotFound ? UINT64_MAX : next;
        BOOL ready = YES;
        for (NSCAudioNode *node in nodes) {
            if ([node isKindOfClass:[NSCAudioWorkletNode class]] && ![(NSCAudioWorkletNode *)node prepareOfflineQuantumBefore:limit]) ready = NO;
        }
        if (!ready) {
            if (CFAbsoluteTimeGetCurrent() - _lastProgress < NSCOfflineWorkletStallSeconds) {
                [self renderSliceAfter:0.001];
                return;
            }
            NSCLogError(@"NSCOfflineAudioContext: an AudioWorklet stopped answering; rendering it as silence");
            for (NSCAudioNode *node in nodes) {
                if ([node isKindOfClass:[NSCAudioWorkletNode class]]) [(NSCAudioWorkletNode *)node abandonOffline];
            }
        }
        for (NSCAudioNode *node in nodes) [node applyAutomation];

        AVAudioFrameCount frames = MIN(_chunk.frameCapacity, _length - _rendered.frameLength);
        NSError *err = nil;
        AVAudioEngineManualRenderingStatus status = [engine renderOffline:frames toBuffer:_chunk error:&err];
        if (status == AVAudioEngineManualRenderingStatusCannotDoInCurrentContext) {
            [self renderSliceAfter:0.001];
            return;
        }
        if (status != AVAudioEngineManualRenderingStatusSuccess) {
            NSCLogError(@"NSCOfflineAudioContext: renderOffline failed (%ld): %@", (long)status, err);
            [self finishRendering:NO];
            return;
        }
        AVAudioFrameCount offset = _rendered.frameLength;
        AVAudioFrameCount got = MIN(_chunk.frameLength, _length - offset);
        for (AVAudioChannelCount c = 0; c < _rendered.format.channelCount; c++) {
            memcpy(_rendered.floatChannelData[c] + offset, _chunk.floatChannelData[c], got * sizeof(float));
        }
        _rendered.frameLength = offset + got;
        _lastProgress = CFAbsoluteTimeGetCurrent();
        if (_lastProgress > deadline && _rendered.frameLength < _length) {
            dispatch_async(dispatch_get_main_queue(), ^{ [self renderSlice]; });
            return;
        }
    }
    [self finishRendering:YES];
}

- (void)renderSliceAfter:(double)seconds {
    dispatch_after(dispatch_time(DISPATCH_TIME_NOW, (int64_t)(seconds * NSEC_PER_SEC)), dispatch_get_main_queue(), ^{ [self renderSlice]; });
}

- (void)finishRendering:(BOOL)ok {
    _finished = YES;
    _paused = NO;
    [_suspends removeAllIndexes];
    @try { [self.engine stop]; } @catch (NSException *e) {}
    NSCAudioBuffer *buffer = nil;
    if (ok && _rendered) {
        _rendered.frameLength = _length;
        buffer = [[NSCAudioBuffer alloc] initWithContext:self id:[[NSUUID UUID] UUIDString] buffer:_rendered];
    }
    _rendered = nil;
    _chunk = nil;
    void (^completion)(NSCAudioBuffer * _Nullable) = _completion;
    _completion = nil;
    if (completion) completion(buffer);
}

@end
