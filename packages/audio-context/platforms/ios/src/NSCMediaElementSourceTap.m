#import "NSCMediaElementSourceTap.h"
#import "NSCAudioContext.h"
#import "NSCAudioNode.h"
#import "NSCAudioLog.h"

#import <MediaToolbox/MediaToolbox.h>
#import <stdatomic.h>
#import <string.h>

#pragma mark - SPSC byte ring

typedef struct {
    uint8_t * _Nonnull data;
    uint32_t mask;       
    uint32_t capacity;
    _Atomic uint32_t writeIdx;
    _Atomic uint32_t readIdx;
} NSCRing;

static void NSCRingInit(NSCRing *r, uint32_t capacityPow2) {
    r->capacity = capacityPow2;
    r->mask = capacityPow2 - 1;
    r->data = (uint8_t *)calloc(1, capacityPow2);
    atomic_store_explicit(&r->writeIdx, 0, memory_order_relaxed);
    atomic_store_explicit(&r->readIdx, 0, memory_order_relaxed);
}

static void NSCRingFree(NSCRing *r) {
    if (r->data) {
        free(r->data);
        r->data = NULL;
    }
}

static uint32_t NSCRingAvailableRead(NSCRing *r) {
    uint32_t w = atomic_load_explicit(&r->writeIdx, memory_order_acquire);
    uint32_t rd = atomic_load_explicit(&r->readIdx, memory_order_relaxed);
    return w - rd;
}

static uint32_t NSCRingAvailableWrite(NSCRing *r) {
    return r->capacity - NSCRingAvailableRead(r);
}


static uint32_t NSCRingWrite(NSCRing *r, const void * _Nonnull src, uint32_t len) {
    uint32_t avail = NSCRingAvailableWrite(r);
    if (len > avail) len = avail;
    uint32_t w = atomic_load_explicit(&r->writeIdx, memory_order_relaxed);
    uint32_t pos = w & r->mask;
    uint32_t firstChunk = r->capacity - pos;
    if (firstChunk > len) firstChunk = len;
    memcpy(r->data + pos, src, firstChunk);
    if (len > firstChunk) {
        memcpy(r->data, (const uint8_t *)src + firstChunk, len - firstChunk);
    }
    atomic_store_explicit(&r->writeIdx, w + len, memory_order_release);
    return len;
}


static uint32_t NSCRingRead(NSCRing *r, void * _Nonnull dst, uint32_t len) {
    uint32_t avail = NSCRingAvailableRead(r);
    if (len > avail) len = avail;
    uint32_t rd = atomic_load_explicit(&r->readIdx, memory_order_relaxed);
    uint32_t pos = rd & r->mask;
    uint32_t firstChunk = r->capacity - pos;
    if (firstChunk > len) firstChunk = len;
    memcpy(dst, r->data + pos, firstChunk);
    if (len > firstChunk) {
        memcpy((uint8_t *)dst + firstChunk, r->data, len - firstChunk);
    }
    atomic_store_explicit(&r->readIdx, rd + len, memory_order_release);
    return len;
}

#pragma mark - Tap context (heap-allocated, owned by the MTAudioProcessingTap)

typedef struct {
    NSCRing ring;                       
    AudioConverterRef converter;        
    AudioStreamBasicDescription tapASBD;
    AudioStreamBasicDescription outASBD;
    void * _Nullable conversionBuffer;  
    uint32_t conversionBufferBytes;
    uint32_t bytesPerEngineFrame;       
} NSCTapCore;


typedef struct {
    AudioBufferList *list;     
    BOOL consumed;
} NSCConverterInputState;

static OSStatus NSCConverterInputProc(AudioConverterRef inAudioConverter,
                                      UInt32 *ioNumberDataPackets,
                                      AudioBufferList *ioData,
                                      AudioStreamPacketDescription * _Nullable * _Nullable outDataPacketDescription,
                                      void *inUserData) {
    NSCConverterInputState *state = (NSCConverterInputState *)inUserData;
    if (state->consumed || !state->list || state->list->mNumberBuffers == 0) {
        *ioNumberDataPackets = 0;
        return 0;
    }
    UInt32 nBuffers = state->list->mNumberBuffers;
    if (ioData->mNumberBuffers < nBuffers) nBuffers = ioData->mNumberBuffers;
    for (UInt32 i = 0; i < nBuffers; i++) {
        ioData->mBuffers[i] = state->list->mBuffers[i];
    }
    ioData->mNumberBuffers = nBuffers;
    state->consumed = YES;
    if (outDataPacketDescription) *outDataPacketDescription = NULL;
    return 0;
}

static void NSCTapInit(MTAudioProcessingTapRef tap, void *clientInfo, void **tapStorageOut) {
    *tapStorageOut = clientInfo;
}

static void NSCTapFinalize(MTAudioProcessingTapRef tap) {
    NSCTapCore *core = (NSCTapCore *)MTAudioProcessingTapGetStorage(tap);
    if (!core) return;
    if (core->converter) {
        AudioConverterDispose(core->converter);
        core->converter = NULL;
    }
    if (core->conversionBuffer) {
        free(core->conversionBuffer);
        core->conversionBuffer = NULL;
    }
    NSCRingFree(&core->ring);
    free(core);
}

static void NSCTapPrepare(MTAudioProcessingTapRef tap, CMItemCount maxFrames,
                          const AudioStreamBasicDescription *processingFormat) {
    NSCTapCore *core = (NSCTapCore *)MTAudioProcessingTapGetStorage(tap);
    if (!core) return;
    core->tapASBD = *processingFormat;

    
    if (memcmp(&core->tapASBD, &core->outASBD, sizeof(AudioStreamBasicDescription)) != 0) {
        AudioConverterRef conv = NULL;
        OSStatus s = AudioConverterNew(&core->tapASBD, &core->outASBD, &conv);
        if (s == noErr) {
            core->converter = conv;
            UInt32 quality = kAudioConverterQuality_Max;
            AudioConverterSetProperty(conv, kAudioConverterSampleRateConverterQuality,
                                      sizeof(quality), &quality);
        } else {
            NSCLogError(@"NSCMediaElementSourceTap: AudioConverterNew failed (%d). Tap formats: in sr=%.0f ch=%u, out sr=%.0f ch=%u",
                  (int)s,
                  core->tapASBD.mSampleRate, core->tapASBD.mChannelsPerFrame,
                  core->outASBD.mSampleRate, core->outASBD.mChannelsPerFrame);
        }
    }

    double srRatio = core->outASBD.mSampleRate /
                     (core->tapASBD.mSampleRate > 0 ? core->tapASBD.mSampleRate : core->outASBD.mSampleRate);
    if (srRatio < 1.0) srRatio = 1.0;
    uint64_t outFrames = (uint64_t)((double)maxFrames * srRatio + 64.0);
    uint64_t bytes = outFrames * (uint64_t)core->outASBD.mBytesPerFrame;
    if (bytes < 8192) bytes = 8192;
    if (core->conversionBuffer) free(core->conversionBuffer);
    core->conversionBuffer = malloc((size_t)bytes);
    core->conversionBufferBytes = (uint32_t)bytes;
}

static void NSCTapUnprepare(MTAudioProcessingTapRef tap) {
    NSCTapCore *core = (NSCTapCore *)MTAudioProcessingTapGetStorage(tap);
    if (!core) return;
    if (core->converter) {
        AudioConverterDispose(core->converter);
        core->converter = NULL;
    }
}

static void NSCTapProcess(MTAudioProcessingTapRef tap, CMItemCount numberFrames,
                          MTAudioProcessingTapFlags flags,
                          AudioBufferList *bufferListInOut,
                          CMItemCount *numberFramesOut,
                          MTAudioProcessingTapFlags *flagsOut) {
    OSStatus status = MTAudioProcessingTapGetSourceAudio(tap, numberFrames,
                                                          bufferListInOut, flagsOut,
                                                          NULL, numberFramesOut);
    if (status != noErr) return;

    NSCTapCore *core = (NSCTapCore *)MTAudioProcessingTapGetStorage(tap);
    if (!core || !core->conversionBuffer) return;

    if (core->converter) {
        AudioBufferList outAbl;
        outAbl.mNumberBuffers = 1;
        outAbl.mBuffers[0].mNumberChannels = core->outASBD.mChannelsPerFrame;
        outAbl.mBuffers[0].mDataByteSize = core->conversionBufferBytes;
        outAbl.mBuffers[0].mData = core->conversionBuffer;

        UInt32 outPackets = core->conversionBufferBytes / core->bytesPerEngineFrame;
        NSCConverterInputState state;
        state.list = bufferListInOut;
        state.consumed = NO;
        OSStatus s = AudioConverterFillComplexBuffer(core->converter,
                                                     NSCConverterInputProc,
                                                     &state,
                                                     &outPackets,
                                                     &outAbl,
                                                     NULL);
        if (s == noErr && outPackets > 0) {
            uint32_t bytes = outPackets * core->bytesPerEngineFrame;
            NSCRingWrite(&core->ring, core->conversionBuffer, bytes);
        } else if (s != noErr) {
            NSCLogError(@"NSCMediaElementSourceTap: AudioConverterFillComplexBuffer failed (%d)", (int)s);
        }
    } else {
        if (bufferListInOut->mNumberBuffers > 0) {
            const AudioBuffer *b = &bufferListInOut->mBuffers[0];
            NSCRingWrite(&core->ring, b->mData, b->mDataByteSize);
        }
    }


    for (UInt32 i = 0; i < bufferListInOut->mNumberBuffers; i++) {
        memset(bufferListInOut->mBuffers[i].mData, 0,
               bufferListInOut->mBuffers[i].mDataByteSize);
    }
}

#pragma mark - Obj-C wrapper

@implementation NSCMediaElementSourceTap {
    MTAudioProcessingTapRef _tap;          
    NSCTapCore *_core;                     
    AVMutableAudioMix *_audioMix;
    AVPlayerItem *_attachedItem;
    BOOL _observingPlayer;
}

- (void)applyMixToCurrentItem {
    AVPlayer *p = self.player;
    if (!p) return;
    AVPlayerItem *item = p.currentItem;
    if (!item || !_audioMix) return;
    if (item == _attachedItem && item.audioMix == _audioMix) return;
    item.audioMix = _audioMix;
    _attachedItem = item;
}

- (void)observeValueForKeyPath:(NSString *)keyPath
                      ofObject:(id)object
                        change:(NSDictionary<NSKeyValueChangeKey,id> *)change
                       context:(void *)contextPtr {
    if ([keyPath isEqualToString:@"currentItem"]) {
        [self applyMixToCurrentItem];
    }
}

+ (nullable instancetype)attachToPlayer:(AVPlayer *)player
                                context:(NSCAudioContext *)context {
    if (!player || !context) return nil;
    AVPlayerItem *item = player.currentItem;
    if (!item) {
        NSCLogDebug(@"NSCMediaElementSourceTap: player has no current item; aborting attach.");
        return nil;
    }

    AVAssetTrack *audioTrack = nil;
    for (AVAssetTrack *t in item.asset.tracks) {
        if ([t.mediaType isEqualToString:AVMediaTypeAudio]) { audioTrack = t; break; }
    }
    if (!audioTrack) {
        NSCLogDebug(@"NSCMediaElementSourceTap: no audio track on current item.");
        return nil;
    }

    NSCMediaElementSourceTap *self_ = [[NSCMediaElementSourceTap alloc] init];
    if (!self_) return nil;

    self_->_player = player;
    self_->_context = context;
    self_->_attachedItem = item;

    AVAudioFormat *engineFormat = [context.engine.mainMixerNode outputFormatForBus:0];
    double engineSR = engineFormat.sampleRate;
    AVAudioFormat *outFormat = [[AVAudioFormat alloc]
                                 initWithCommonFormat:AVAudioPCMFormatFloat32
                                           sampleRate:engineSR
                                             channels:2
                                          interleaved:YES];

    NSCTapCore *core = (NSCTapCore *)calloc(1, sizeof(NSCTapCore));
    NSCRingInit(&core->ring, 1u << 18);
    core->outASBD = *outFormat.streamDescription;
    core->bytesPerEngineFrame = core->outASBD.mBytesPerFrame;
    self_->_core = core;

    MTAudioProcessingTapCallbacks callbacks = {
        .version = kMTAudioProcessingTapCallbacksVersion_0,
        .clientInfo = core,
        .init = NSCTapInit,
        .finalize = NSCTapFinalize,
        .prepare = NSCTapPrepare,
        .unprepare = NSCTapUnprepare,
        .process = NSCTapProcess,
    };
    MTAudioProcessingTapRef tap = NULL;
    OSStatus s = MTAudioProcessingTapCreate(kCFAllocatorDefault, &callbacks,
                                             kMTAudioProcessingTapCreationFlag_PostEffects,
                                             &tap);
    if (s != noErr || tap == NULL) {
        NSCLogError(@"NSCMediaElementSourceTap: MTAudioProcessingTapCreate failed (%d)", (int)s);
        NSCTapFinalize(NULL);
        if (core) {
            NSCRingFree(&core->ring);
            free(core);
        }
        return nil;
    }
    self_->_tap = tap;

    AVMutableAudioMixInputParameters *params =
        [AVMutableAudioMixInputParameters audioMixInputParametersWithTrack:audioTrack];
    params.audioTapProcessor = tap;

    AVMutableAudioMix *mix = [AVMutableAudioMix audioMix];
    mix.inputParameters = @[ params ];
    item.audioMix = mix;
    self_->_audioMix = mix;

    [player addObserver:self_ forKeyPath:@"currentItem"
                options:NSKeyValueObservingOptionNew context:NULL];
    self_->_observingPlayer = YES;

    NSCRing *ringPtr = &core->ring;
    uint32_t bytesPerFrame = core->bytesPerEngineFrame;

    AVAudioSourceNode *sourceNode = [[AVAudioSourceNode alloc]
        initWithFormat:outFormat
        renderBlock:^OSStatus(BOOL *isSilence,
                              const AudioTimeStamp *timestamp,
                              AVAudioFrameCount frameCount,
                              AudioBufferList *outputData) {
            uint32_t bytesNeeded = frameCount * bytesPerFrame;
            if (outputData->mNumberBuffers == 0) return noErr;
            void *dst = outputData->mBuffers[0].mData;
            outputData->mBuffers[0].mDataByteSize = bytesNeeded;

            uint32_t got = NSCRingRead(ringPtr, dst, bytesNeeded);
            if (got < bytesNeeded) {
                memset((uint8_t *)dst + got, 0, bytesNeeded - got);
                if (got == 0) *isSilence = YES;
            }
            return noErr;
        }];
    self_->_sourceNode = sourceNode;

    [context.engine attachNode:sourceNode];

    [context.engine connect:sourceNode
                         to:context.engine.mainMixerNode
                     format:outFormat];

    return self_;
}

- (void)detach {
    if (_observingPlayer && _player) {
        @try {
            [_player removeObserver:self forKeyPath:@"currentItem"];
        } @catch (NSException *ex) {
            NSCLogDebug(@"NSCMediaElementSourceTap: removeObserver threw: %@", ex);
        }
        _observingPlayer = NO;
    }
    if (_attachedItem) {
        _attachedItem.audioMix = nil;
        _attachedItem = nil;
    }
    if (_sourceNode) {
        @try {
            [_context.engine disconnectNodeOutput:_sourceNode];
            [_context detachNode:_sourceNode fromEngine:_context.engine];
        } @catch (NSException *ex) {
            NSCLogDebug(@"NSCMediaElementSourceTap: detach disconnect failed: %@", ex);
        }
        _sourceNode = nil;
    }
    if (_tap) {
        CFRelease(_tap);
        _tap = NULL;
        _core = NULL;
    }
    _audioMix = nil;
}

- (void)dealloc {
    [self detach];
}

@end
