#import "NSCMediaElementSourceTap.h"
#import "NSCAudioContext.h"
#import "NSCAudioNode.h"
#import "NSCAudioLog.h"

#import "NSCAudioParam.h"

#import <MediaToolbox/MediaToolbox.h>
#import <stdatomic.h>
#import <stdint.h>
#import <string.h>
#import <objc/runtime.h>

static void *kNSCMediaElementSourceTapPlayerKey = &kNSCMediaElementSourceTapPlayerKey;

static BOOL NSCEnsureContextEngineRunning(NSCAudioContext *context,
                      NSString *label) {
  if (!context) return NO;
  AVAudioEngine *engine = context.engine;
  if (!engine) return NO;
  if (engine.isRunning) return YES;

  BOOL started = [NSCAudioContext startEngineWithRetry:engine
                        attempts:2
                           label:label ?: @"media-tap-engine-start"
                     asyncCompletion:nil];
  return started || engine.isRunning;
}

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

static void NSCRingClear(NSCRing *r) {
    if (!r || !r->data) return;
    atomic_store_explicit(&r->readIdx, 0, memory_order_release);
    atomic_store_explicit(&r->writeIdx, 0, memory_order_release);
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
    void * _Nullable renderBuffer;
    uint32_t conversionBufferBytes;
    uint32_t renderBufferBytes;
    uint32_t bytesPerEngineFrame;
    uint32_t bytesPerChannelFrame;
    uint32_t channelCount;
} NSCTapCore;


typedef struct {
    AudioBufferList *list;     
    AudioStreamBasicDescription asbd;
    UInt32 availablePackets;
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
    UInt32 requestedPackets = ioNumberDataPackets ? *ioNumberDataPackets : 0;
    UInt32 packetBytes = state->asbd.mBytesPerPacket;
    if (packetBytes == 0) packetBytes = state->asbd.mBytesPerFrame;
    if (packetBytes == 0 && state->availablePackets > 0 && state->list->mNumberBuffers > 0) {
        packetBytes = state->list->mBuffers[0].mDataByteSize / state->availablePackets;
    }

    UInt32 packets = state->availablePackets > 0 ? state->availablePackets : requestedPackets;
    if (requestedPackets > 0 && packets > requestedPackets) packets = requestedPackets;
    if (packetBytes > 0) {
        for (UInt32 i = 0; i < state->list->mNumberBuffers; i++) {
            UInt32 bufferPackets = state->list->mBuffers[i].mDataByteSize / packetBytes;
            if (packets == 0 || bufferPackets < packets) packets = bufferPackets;
        }
    }
    if (packets == 0) {
        *ioNumberDataPackets = 0;
        return 0;
    }

    UInt32 nBuffers = state->list->mNumberBuffers;
    if (ioData->mNumberBuffers < nBuffers) nBuffers = ioData->mNumberBuffers;
    for (UInt32 i = 0; i < nBuffers; i++) {
        ioData->mBuffers[i] = state->list->mBuffers[i];
        if (packetBytes > 0) {
            UInt32 byteCount = packets * packetBytes;
            if (ioData->mBuffers[i].mDataByteSize > byteCount) {
                ioData->mBuffers[i].mDataByteSize = byteCount;
            }
        }
    }
    ioData->mNumberBuffers = nBuffers;
    *ioNumberDataPackets = packets;
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
    if (core->renderBuffer) {
        free(core->renderBuffer);
        core->renderBuffer = NULL;
    }
    NSCRingFree(&core->ring);
    free(core);
}

static void NSCTapPrepare(MTAudioProcessingTapRef tap, CMItemCount maxFrames,
                          const AudioStreamBasicDescription *processingFormat) {
    NSCTapCore *core = (NSCTapCore *)MTAudioProcessingTapGetStorage(tap);
    if (!core) return;
    core->tapASBD = *processingFormat;

    if (core->converter) {
        AudioConverterDispose(core->converter);
        core->converter = NULL;
    }

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
        state.asbd = core->tapASBD;
        state.availablePackets = numberFramesOut ? (UInt32)MIN((CMItemCount)UINT32_MAX, *numberFramesOut) : (UInt32)MIN((CMItemCount)UINT32_MAX, numberFrames);
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
        AudioBuffer *b = &bufferListInOut->mBuffers[i];
        if (b->mData && b->mDataByteSize > 0) {
            memset(b->mData, 0, b->mDataByteSize);
        }
    }
}

#pragma mark - Obj-C wrapper

@implementation NSCMediaElementSourceTap {
  MTAudioProcessingTapRef _tap;
  NSCTapCore *_core;
  AVMutableAudioMix *_audioMix;
  AVPlayerItem *_attachedItem;
  AVPlayerItem *_observedItem;
  BOOL _observingPlayer;
  BOOL _observingItem;

  NSCAudioParam *_playbackRateParam;
  AVAudioUnitVarispeed *_varispeed;
  AVAudioNode *_outputNode;
  AVAudioPlayerNode *_playerNode;
  dispatch_source_t _playerSchedulingSource;
  dispatch_source_t _playbackRateAutomationTimer;
}

- (void)applyMixToCurrentItem {
  AVPlayer *p = self.player;
  if (!p)
    return;
  AVPlayerItem *item = p.currentItem;
  if (!item || !_audioMix)
    return;
  if (item == _attachedItem && item.audioMix == _audioMix)
    return;
  item.audioMix = _audioMix;
  _attachedItem = item;
}

- (void)removeItemObserverIfNeeded {
  if (_observingItem && _observedItem) {
    @try {
      [_observedItem removeObserver:self forKeyPath:@"status"];
    } @catch (NSException *ex) {
      NSCLogDebug(@"NSCMediaElementSourceTap: removeObserver threw: %@", ex);
    }
    _observingItem = NO;
    _observedItem = nil;
  }
}

- (BOOL)installTapForItem:(AVPlayerItem *)item {
  if (!item || !self->_context)
    return NO;

  if (!self->_tap)
    return NO;

  if (self->_attachedItem == item && item.audioMix == self->_audioMix) {
    [self applyMixToCurrentItem];
    return YES;
  }

  if (self->_attachedItem && self->_attachedItem != item) {
    @try {
      if (self->_attachedItem.audioMix == self->_audioMix) {
        self->_attachedItem.audioMix = nil;
      }
    } @catch (NSException *ex) {
    }
    self->_attachedItem = nil;
    self->_audioMix = nil;
  }

  if (item.status != AVPlayerItemStatusReadyToPlay) {
    if (_observingItem && _observedItem != item) {
      [self removeItemObserverIfNeeded];
    }
    if (!_observingItem) {
      @try {
        [item addObserver:self
               forKeyPath:@"status"
                  options:NSKeyValueObservingOptionNew
                  context:NULL];
        _observingItem = YES;
        _observedItem = item;
      } @catch (NSException *ex) {
        NSCLogDebug(@"NSCMediaElementSourceTap: addObserver(status) threw: %@",
                    ex);
      }
    }

    AVAsset *asset = item.asset;
    __weak typeof(self) weakSelf = self;
    [asset loadValuesAsynchronouslyForKeys:@[ @"tracks" ]
                         completionHandler:^{
                           __strong typeof(weakSelf) s = weakSelf;
                           if (!s)
                             return;
                           dispatch_async(dispatch_get_main_queue(), ^{
                             if (s.player.currentItem == item) {
                               [s installTapForItem:item];
                             }
                           });
                         }];

    return NO;
  }

  AVAsset *asset = item.asset;
  NSError *statusErr = nil;
  AVKeyValueStatus tracksStatus = [asset statusOfValueForKey:@"tracks"
                                                       error:&statusErr];
  if (tracksStatus != AVKeyValueStatusLoaded) {
    __weak typeof(self) weakSelf = self;
    [asset loadValuesAsynchronouslyForKeys:@[ @"tracks" ]
                         completionHandler:^{
                           __strong typeof(weakSelf) s = weakSelf;
                           if (!s)
                             return;
                           NSError *err = nil;
                           AVKeyValueStatus st =
                               [asset statusOfValueForKey:@"tracks" error:&err];
                           if (st == AVKeyValueStatusLoaded) {
                             dispatch_async(dispatch_get_main_queue(), ^{
                               if (s.player.currentItem == item) {
                                 [s installTapForItem:item];
                               }
                             });
                           } else {
                             NSCLogDebug(@"NSCMediaElementSourceTap: asset "
                                         @"tracks not loadable (status=%d)",
                                         (int)st);
                           }
                         }];
    return NO;
  }

  AVAssetTrack *audioTrack = nil;
  for (AVAssetTrack *t in asset.tracks) {
    if ([t.mediaType isEqualToString:AVMediaTypeAudio]) {
      audioTrack = t;
      break;
    }
  }
  if (!audioTrack) {
    NSCLogDebug(@"NSCMediaElementSourceTap: no audio track on provided item.");
    return NO;
  }

  AVMutableAudioMixInputParameters *params = [AVMutableAudioMixInputParameters
      audioMixInputParametersWithTrack:audioTrack];
  params.audioTapProcessor = self->_tap;

  AVMutableAudioMix *mix = [AVMutableAudioMix audioMix];
  mix.inputParameters = @[ params ];
  item.audioMix = mix;
  self->_audioMix = mix;

  [self removeItemObserverIfNeeded];

  self->_attachedItem = item;
  if (self->_core)
    NSCRingClear(&self->_core->ring);

  return YES;
}

- (void)observeValueForKeyPath:(NSString *)keyPath
                      ofObject:(id)object
                        change:(NSDictionary<NSKeyValueChangeKey, id> *)change
                       context:(void *)contextPtr {
  if ([keyPath isEqualToString:@"currentItem"]) {
    AVPlayerItem *item = self.player.currentItem;
    if (item) {
      [self installTapForItem:item];
    } else {
      if (_attachedItem) {
        @try {
          if (_attachedItem.audioMix == _audioMix)
            _attachedItem.audioMix = nil;
        } @catch (NSException *ex) {
        }
        _attachedItem = nil;
      }
      [self removeItemObserverIfNeeded];
      _audioMix = nil;
    }
  } else if ([keyPath isEqualToString:@"status"]) {
    AVPlayerItem *item = (AVPlayerItem *)object;
    if (item != _attachedItem && item != self.player.currentItem)
      return;
    AVPlayerItemStatus st = item.status;
    if (st == AVPlayerItemStatusReadyToPlay) {
      [self removeItemObserverIfNeeded];
      dispatch_async(dispatch_get_main_queue(), ^{
        if (self.player.currentItem == item)
          [self installTapForItem:item];
      });
    } else if (st == AVPlayerItemStatusFailed) {
      NSCLogDebug(@"NSCMediaElementSourceTap: player item status failed: %@",
                  item.error);
      [self removeItemObserverIfNeeded];
    }
  }
}

+ (nullable instancetype)attachToPlayer:(AVPlayer *)player
                                context:(NSCAudioContext *)context {
  if (!player || !context)
    return nil;
  if (![NSThread isMainThread]) {
    __block NSCMediaElementSourceTap *created = nil;
    dispatch_sync(dispatch_get_main_queue(), ^{
      created = [NSCMediaElementSourceTap attachToPlayer:player
                                                 context:context];
    });
    return created;
  }

  id existing =
      objc_getAssociatedObject(player, kNSCMediaElementSourceTapPlayerKey);
  if (existing) {
    NSCLogDebug(@"NSCMediaElementSourceTap: player already has a tap attached; "
                @"refusing second attach.");
    return nil;
  }

  NSCMediaElementSourceTap *self_ = [[NSCMediaElementSourceTap alloc] init];
  if (!self_)
    return nil;

  self_->_player = player;
  self_->_context = context;

  AVAudioFormat *engineFormat = nil;
  @try {
    engineFormat = [context.engine.mainMixerNode outputFormatForBus:0];
  } @catch (NSException *ex) {
    engineFormat = nil;
  }
  double engineSR = engineFormat && engineFormat.sampleRate > 0.0
                        ? engineFormat.sampleRate
                        : context.sampleRate;
  if (engineSR <= 0.0)
    engineSR = 44100.0;
  AVAudioFormat *ringFormat =
      [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32
                                       sampleRate:engineSR
                                         channels:2
                                      interleaved:YES];
  AVAudioFormat *nodeFormat =
      [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32
                                       sampleRate:engineSR
                                         channels:2
                                      interleaved:NO];
  if (!ringFormat || !nodeFormat)
    return nil;

  NSCTapCore *core = (NSCTapCore *)calloc(1, sizeof(NSCTapCore));
  if (!core)
    return nil;
  NSCRingInit(&core->ring, 1u << 18);
  if (!core->ring.data) {
    free(core);
    return nil;
  }
  core->outASBD = *ringFormat.streamDescription;
  core->bytesPerEngineFrame = core->outASBD.mBytesPerFrame;
  core->channelCount = MAX(1, core->outASBD.mChannelsPerFrame);
  core->bytesPerChannelFrame =
      core->channelCount > 0 ? core->bytesPerEngineFrame / core->channelCount : 0;
  if (core->bytesPerEngineFrame == 0 || core->bytesPerChannelFrame == 0) {
    NSCRingFree(&core->ring);
    free(core);
    return nil;
  }
  core->renderBufferBytes = core->bytesPerEngineFrame * 8192;
  core->renderBuffer = malloc(core->renderBufferBytes);
  if (!core->renderBuffer) {
    NSCRingFree(&core->ring);
    free(core);
    return nil;
  }

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
  OSStatus s = MTAudioProcessingTapCreate(
      kCFAllocatorDefault, &callbacks,
      kMTAudioProcessingTapCreationFlag_PostEffects, &tap);
  if (s != noErr || tap == NULL) {
    NSCLogError(
        @"NSCMediaElementSourceTap: MTAudioProcessingTapCreate failed (%d)",
        (int)s);
    if (core->renderBuffer)
      free(core->renderBuffer);
    NSCRingFree(&core->ring);
    free(core);
    return nil;
  }

  NSCRing *ringPtr = &core->ring;
  uint8_t *renderBuffer = (uint8_t *)core->renderBuffer;
  uint32_t renderBufferBytes = core->renderBufferBytes;
  uint32_t bytesPerInterleavedFrame = core->bytesPerEngineFrame;
  uint32_t bytesPerChannelFrame = core->bytesPerChannelFrame;
  uint32_t channelCount = core->channelCount;
  AVAudioSourceNode *sourceNode = [[AVAudioSourceNode alloc]
      initWithFormat:nodeFormat
         renderBlock:^OSStatus(BOOL *isSilence, const AudioTimeStamp *timestamp,
                               AVAudioFrameCount frameCount,
                               AudioBufferList *outputData) {
           if (isSilence)
             *isSilence = NO;
           if (!outputData || outputData->mNumberBuffers == 0)
             return noErr;

           BOOL planarOutput = outputData->mNumberBuffers >= channelCount;
           if (!renderBuffer || renderBufferBytes < bytesPerInterleavedFrame ||
               frameCount == 0 || bytesPerInterleavedFrame == 0 ||
               bytesPerChannelFrame == 0) {
             for (UInt32 i = 0; i < outputData->mNumberBuffers; i++) {
               if (outputData->mBuffers[i].mData &&
                   outputData->mBuffers[i].mDataByteSize > 0) {
                 memset(outputData->mBuffers[i].mData, 0,
                        outputData->mBuffers[i].mDataByteSize);
               }
             }
             if (isSilence)
               *isSilence = YES;
             return noErr;
           }

           if (planarOutput) {
             for (UInt32 ch = 0; ch < outputData->mNumberBuffers; ch++) {
               AudioBuffer *buffer = &outputData->mBuffers[ch];
               if (buffer->mData) {
                 uint64_t byteCount64 =
                     (uint64_t)frameCount * (uint64_t)bytesPerChannelFrame;
                 buffer->mDataByteSize =
                     byteCount64 > UINT32_MAX ? UINT32_MAX : (UInt32)byteCount64;
                 if (ch >= channelCount)
                   memset(buffer->mData, 0, buffer->mDataByteSize);
               }
             }
           } else {
             AudioBuffer *buffer = &outputData->mBuffers[0];
             if (buffer->mData) {
               uint64_t byteCount64 =
                   (uint64_t)frameCount * (uint64_t)bytesPerInterleavedFrame;
               buffer->mDataByteSize =
                   byteCount64 > UINT32_MAX ? UINT32_MAX : (UInt32)byteCount64;
             }
           }

           BOOL anyAudio = NO;
           uint32_t maxChunkFrames = renderBufferBytes / bytesPerInterleavedFrame;
           AVAudioFrameCount framesDone = 0;
           while (framesDone < frameCount && maxChunkFrames > 0) {
             uint32_t framesThis =
                 (uint32_t)MIN((AVAudioFrameCount)maxChunkFrames,
                               frameCount - framesDone);
             uint32_t bytesNeeded = framesThis * bytesPerInterleavedFrame;
             uint32_t got = NSCRingRead(ringPtr, renderBuffer, bytesNeeded);
             uint32_t gotFrames = got / bytesPerInterleavedFrame;
             if (gotFrames > 0)
               anyAudio = YES;

             if (planarOutput) {
               for (UInt32 ch = 0; ch < channelCount; ch++) {
                 AudioBuffer *buffer = &outputData->mBuffers[ch];
                 if (!buffer->mData)
                   continue;
                 uint8_t *dst =
                     (uint8_t *)buffer->mData + framesDone * bytesPerChannelFrame;
                 if (bytesPerChannelFrame == sizeof(float) && channelCount == 2) {
                   float *dstFloat = (float *)dst;
                   float *srcFloat = (float *)renderBuffer;
                   for (uint32_t frame = 0; frame < gotFrames; frame++) {
                     dstFloat[frame] = srcFloat[(frame * 2) + ch];
                   }
                 } else {
                   for (uint32_t frame = 0; frame < gotFrames; frame++) {
                     memcpy(dst + (frame * bytesPerChannelFrame),
                            renderBuffer + (frame * bytesPerInterleavedFrame) +
                                (ch * bytesPerChannelFrame),
                            bytesPerChannelFrame);
                   }
                 }
                 if (gotFrames < framesThis) {
                   memset(dst + (gotFrames * bytesPerChannelFrame), 0,
                          (framesThis - gotFrames) * bytesPerChannelFrame);
                 }
               }
             } else {
               AudioBuffer *buffer = &outputData->mBuffers[0];
               if (buffer->mData) {
                 uint8_t *dst = (uint8_t *)buffer->mData +
                                framesDone * bytesPerInterleavedFrame;
                 memcpy(dst, renderBuffer, got);
                 if (got < bytesNeeded)
                   memset(dst + got, 0, bytesNeeded - got);
               }
             }

             framesDone += framesThis;
           }
           if (!anyAudio && isSilence)
             *isSilence = YES;
           return noErr;
         }];
  if (!sourceNode) {
    CFRelease(tap);
    return nil;
  }

  self_->_core = core;
  self_->_tap = tap;
  self_->_sourceNode = sourceNode;
  self_->_outputNode = sourceNode;

  @try {
    [context.engine attachNode:sourceNode];
  } @catch (NSException *ex) {
    NSCLogDebug(@"NSCMediaElementSourceTap: attach source node failed: %@", ex);
    self_->_tap = NULL;
    self_->_sourceNode = nil;
    self_->_outputNode = nil;
    CFRelease(tap);
    self_->_core = NULL;
    return nil;
  }


  AVAudioPlayerNode *pnode = [[AVAudioPlayerNode alloc] init];
  if (pnode) {
    @try {
      [context.engine attachNode:pnode];
      self_->_playerNode = pnode;
      self_->_outputNode = pnode;

      if (NSCEnsureContextEngineRunning(context, @"media-tap-initial-player-play")) {
        [pnode play];
      }

      dispatch_queue_t scheduleQ = dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0);
      dispatch_source_t sched = dispatch_source_create(DISPATCH_SOURCE_TYPE_TIMER, 0, 0, scheduleQ);
      if (sched) {
        uint64_t interval = (uint64_t)(NSEC_PER_SEC / 50);
        dispatch_source_set_timer(sched, dispatch_time(DISPATCH_TIME_NOW, 0), interval, (uint64_t)(NSEC_PER_MSEC * 5));
        __weak typeof(pnode) weakP = pnode;
        __weak typeof(self_) weakSelfTap = self_;
        dispatch_source_set_event_handler(sched, ^{
          __strong typeof(weakP) strongP = weakP;
          __strong typeof(weakSelfTap) strongTap = weakSelfTap;
          if (!strongP || !strongTap) return;
          NSCTapCore *coreLocal = core;
          if (!coreLocal) return;
          uint32_t avail = NSCRingAvailableRead(&coreLocal->ring);
          uint32_t bytesPerFrameLocal = bytesPerInterleavedFrame;
          if (bytesPerFrameLocal == 0) return;
          uint32_t framesAvailable = avail / bytesPerFrameLocal;

          if (framesAvailable == 0) return;

       
          @try {
            if (![strongP isPlaying]) {
              BOOL engineReady = NSCEnsureContextEngineRunning(
                  strongTap.context, @"media-tap-player-restart");
              if (!engineReady) {
                return;
              }
              [strongP play];
            }
          } @catch (NSException *ex) {
            NSCLogDebug(@"NSCMediaElementSourceTap: playerNode play retry failed: %@", ex);
          }

          const uint32_t framesPerBuffer = 1024;
          while (framesAvailable > 0) {
            uint32_t framesThis = framesAvailable >= framesPerBuffer ? framesPerBuffer : framesAvailable;
            uint32_t bytesNeeded = framesThis * bytesPerFrameLocal;
            uint8_t *tmpBuf = (uint8_t *)malloc((size_t)bytesNeeded);
            if (!tmpBuf) break;
            uint32_t got = NSCRingRead(&coreLocal->ring, tmpBuf, bytesNeeded);
            uint32_t gotFrames = got / bytesPerFrameLocal;
            if (gotFrames == 0) { free(tmpBuf); break; }
            AVAudioFrameCount fc = (AVAudioFrameCount)gotFrames;

            AVAudioFormat *pnodeFormat = nil;
            @try { pnodeFormat = [strongP outputFormatForBus:0]; } @catch (NSException *e) { pnodeFormat = nil; }
            AVAudioFormat *useFormat = pnodeFormat ?: nodeFormat;

            AVAudioPCMBuffer *pcm = [[AVAudioPCMBuffer alloc] initWithPCMFormat:useFormat frameCapacity:fc];
            if (pcm) {
              pcm.frameLength = fc;
              float *srcFloat = (float *)tmpBuf;
              uint32_t srcChannels = channelCount;
              uint32_t dstChannels = (uint32_t)(useFormat.channelCount);

              if (dstChannels == srcChannels) {
                for (UInt32 ch = 0; ch < dstChannels; ch++) {
                  float *dst = pcm.floatChannelData[ch];
                  for (UInt32 f = 0; f < gotFrames; f++) dst[f] = srcFloat[(f * srcChannels) + ch];
                }
              } else if (dstChannels == 1) {
                float *dst = pcm.floatChannelData[0];
                for (UInt32 f = 0; f < gotFrames; f++) {
                  float sum = 0.0f;
                  for (UInt32 ch = 0; ch < srcChannels; ch++) sum += srcFloat[(f * srcChannels) + ch];
                  dst[f] = sum / (float)srcChannels;
                }
              } else if (srcChannels == 1) {
                for (UInt32 ch = 0; ch < dstChannels; ch++) {
                  float *dst = pcm.floatChannelData[ch];
                  for (UInt32 f = 0; f < gotFrames; f++) dst[f] = srcFloat[f];
                }
              } else {
                for (UInt32 ch = 0; ch < dstChannels; ch++) {
                  float *dst = pcm.floatChannelData[ch];
                  if (ch < srcChannels) {
                    for (UInt32 f = 0; f < gotFrames; f++) dst[f] = srcFloat[(f * srcChannels) + ch];
                  } else {
                    for (UInt32 f = 0; f < gotFrames; f++) dst[f] = 0.0f;
                  }
                }
              }

              [strongP scheduleBuffer:pcm completionHandler:NULL];
            }
            free(tmpBuf);
            avail = NSCRingAvailableRead(&coreLocal->ring);
            framesAvailable = avail / bytesPerFrameLocal;
            if (framesThis < framesPerBuffer) break;
          }
        });
        dispatch_resume(sched);
        self_->_playerSchedulingSource = sched;
      }
    } @catch (NSException *ex) {
      NSCLogDebug(@"NSCMediaElementSourceTap: player node attach failed: %@", ex);
      @try {
        if (pnode.engine == context.engine) {
          [context detachNode:pnode fromEngine:context.engine];
        }
      } @catch (NSException *detachEx) {}
      self_->_playerNode = nil;
      self_->_outputNode = sourceNode;
    }
  }

  if (!self_->_playerNode) {
    AVAudioUnitVarispeed *vs = [[AVAudioUnitVarispeed alloc] init];
    if (vs) {
      @try {
        [context.engine attachNode:vs];
        [context.engine connect:sourceNode to:vs format:nodeFormat];
        self_->_varispeed = vs;
        self_->_outputNode = vs;
      } @catch (NSException *ex) {
        NSCLogDebug(@"NSCMediaElementSourceTap: varispeed setup failed: %@", ex);
        @try {
          if (vs.engine == context.engine) {
            [context detachNode:vs fromEngine:context.engine];
          }
        } @catch (NSException *detachEx) {
        }
        self_->_varispeed = nil;
        self_->_outputNode = sourceNode;
      }
    }
  }

  objc_setAssociatedObject(player, kNSCMediaElementSourceTapPlayerKey, self_,
                           OBJC_ASSOCIATION_RETAIN_NONATOMIC);

  @try {
    [player addObserver:self_
             forKeyPath:@"currentItem"
                options:NSKeyValueObservingOptionNew
                context:NULL];
    self_->_observingPlayer = YES;
  } @catch (NSException *ex) {
    NSCLogDebug(@"NSCMediaElementSourceTap: addObserver(currentItem) threw: %@",
                ex);
  }

  AVPlayerItem *item = player.currentItem;
  if (item) {
    [self_ installTapForItem:item];
  }

  return self_;
}

- (nullable NSCAudioParam *)getPlaybackRateParam {
  if (!_playbackRateParam) {
    _playbackRateParam = [[NSCAudioParam alloc] initWithContext:self.context
                                                   defaultValue:1.0];
    __weak typeof(self) weakSelf = self;
    _playbackRateParam.onScheduleChanged = ^(NSCAudioParam *p) {
      __strong typeof(weakSelf) s = weakSelf;
      if (!s)
        return;
      [s ensurePlaybackRateAutomationLink];
    };
  }
  return _playbackRateParam;
}

- (BOOL)_playbackRateHasFutureEvents {
  NSCAudioContext *ctx = self.context;
  double t = ctx ? ctx.currentTime : 0.0;
  return _playbackRateParam && [_playbackRateParam hasEventsAfter:t];
}

- (void)ensurePlaybackRateAutomationLink {
  if ([NSThread isMainThread] == NO) {
    __weak typeof(self) weakSelf = self;
    dispatch_async(dispatch_get_main_queue(), ^{
      __strong typeof(weakSelf) s = weakSelf;
      if (s)
        [s ensurePlaybackRateAutomationLink];
    });
    return;
  }
  if (_playbackRateAutomationTimer)
    return;
  if (![self _playbackRateHasFutureEvents])
    return;
  uint64_t intervalNs = (uint64_t)(NSEC_PER_SEC / 60.0);
  uint64_t leeway = (uint64_t)(NSEC_PER_SEC / 240.0);
  dispatch_source_t timer = dispatch_source_create(
      DISPATCH_SOURCE_TYPE_TIMER, 0, 0, dispatch_get_main_queue());
  dispatch_source_set_timer(timer, dispatch_time(DISPATCH_TIME_NOW, 0),
                            intervalNs, leeway);
  __weak typeof(self) weakSelf = self;
  dispatch_source_set_event_handler(timer, ^{
    __strong typeof(weakSelf) s = weakSelf;
    if (!s)
      return;
    [s _applyPlaybackRateAutomationOnce];
    if (![s _playbackRateHasFutureEvents])
      [s stopPlaybackRateAutomationLink];
  });
  _playbackRateAutomationTimer = timer;
  dispatch_resume(_playbackRateAutomationTimer);
}

- (void)stopPlaybackRateAutomationLink {
  if (!_playbackRateAutomationTimer)
    return;
  dispatch_source_cancel(_playbackRateAutomationTimer);
  _playbackRateAutomationTimer = nil;
}

- (void)_applyPlaybackRateAutomationOnce {
  NSCAudioContext *ctx = self.context;
  double t = ctx ? ctx.currentTime : 0.0;
  double pr = _playbackRateParam ? [_playbackRateParam valueAtTime:t] : 1.0;
  AVAudioUnitVarispeed *vs = _varispeed;
  if (!vs)
    return;
  dispatch_async(dispatch_get_main_queue(), ^{
    @try {
      float v = (float)pr;
      if (isnan(v))
        v = 1.0f;
      if (v < 0.0f)
        v = 0.0f;
      vs.rate = v;
    } @catch (NSException *e) {
    }
  });
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
    @try {
      if (_attachedItem.audioMix == _audioMix)
        _attachedItem.audioMix = nil;
    } @catch (NSException *ex) {
    }
    _attachedItem = nil;
  }
  [self removeItemObserverIfNeeded];

  if (_player) {
    objc_setAssociatedObject(_player, kNSCMediaElementSourceTapPlayerKey, nil,
                             OBJC_ASSOCIATION_ASSIGN);
  }
  if (_playbackRateAutomationTimer) {
    dispatch_source_cancel(_playbackRateAutomationTimer);
    _playbackRateAutomationTimer = nil;
  }

  AVAudioEngine *engine = _context ? _context.engine : nil;
  AVAudioNode *sourceNode = _sourceNode;
  AVAudioNode *varispeedNode = _varispeed;
  if (sourceNode && engine) {
    @try {
      if ([_context isNode:sourceNode attachedToEngine:engine]) {
        [engine disconnectNodeOutput:sourceNode];
      }
    } @catch (NSException *ex) {
      NSCLogDebug(@"NSCMediaElementSourceTap: detach disconnect failed: %@",
                  ex);
    }
  }
  if (varispeedNode && engine) {
    @try {
      if ([_context isNode:varispeedNode attachedToEngine:engine]) {
        [engine disconnectNodeOutput:varispeedNode];
        [_context detachNode:varispeedNode fromEngine:engine];
      }
    } @catch (NSException *ex) {
      NSCLogDebug(
          @"NSCMediaElementSourceTap: detach varispeed disconnect failed: %@",
          ex);
    }
  }

  if (_playerSchedulingSource) {
    dispatch_source_cancel(_playerSchedulingSource);
    _playerSchedulingSource = nil;
  }
  if (_playerNode && engine) {
    @try {
      if ([_context isNode:_playerNode attachedToEngine:engine]) {
        [_playerNode stop];
        [engine disconnectNodeOutput:_playerNode];
        [_context detachNode:_playerNode fromEngine:engine];
      }
    } @catch (NSException *ex) {
      NSCLogDebug(@"NSCMediaElementSourceTap: detach playerNode disconnect failed: %@", ex);
    }
  }
  if (sourceNode && engine) {
    @try {
      if ([_context isNode:sourceNode attachedToEngine:engine]) {
        [_context detachNode:sourceNode fromEngine:engine];
      }
    } @catch (NSException *ex) {
      NSCLogDebug(@"NSCMediaElementSourceTap: detach source detach failed: %@",
                  ex);
    }
  }
  _sourceNode = nil;
  _varispeed = nil;
  _playerNode = nil;
  _outputNode = nil;
  _playbackRateParam = nil;
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
