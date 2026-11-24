#import <stdatomic.h>
#import <math.h>
#import "NSCAudioContext.h"
#import "NSCAudioLog.h"

// Shared with the render block; the block keeps it alive after the node is gone.
typedef struct {
  atomic_uint_least8_t playing;
  atomic_uint_least8_t loop;
  atomic_int_least64_t startSample;
  atomic_int_least64_t stopSample;
  // Set with the start offset (in buffer frames) before `playing`; the render thread
  // consumes it and owns `position` from then on.
  atomic_uint_least8_t resetPending;
  double pendingOffsetFrames;
  double position;
  _Atomic(void *) pcm;
} NSCBufferSourceState;

@implementation NSCAudioBufferSourceNode {
  AVAudioSourceNode *_sourceNode;
  NSMutableData *_stateData;
  NSCBufferSourceState *_state;
  // Every buffer the render block may still be reading; the block holds this array.
  NSMutableArray<AVAudioPCMBuffer *> *_keepAlive;
  NSCAudioBuffer *_buffer;
  NSCAudioParam *_playbackRateParam;
}

@synthesize buffer = _buffer;

static void NSCBufferSourceSilence(AudioBufferList *abl) {
  for (UInt32 b = 0; b < abl->mNumberBuffers; ++b) {
    if (abl->mBuffers[b].mData) memset(abl->mBuffers[b].mData, 0, abl->mBuffers[b].mDataByteSize);
  }
}

- (instancetype)initWithContext:(NSCAudioContext *)context
                         buffer:(NSCAudioBuffer *)buffer {
  NSMutableData *stateData = [NSMutableData dataWithLength:sizeof(NSCBufferSourceState)];
  NSCBufferSourceState *state = (NSCBufferSourceState *)stateData.mutableBytes;
  atomic_init(&state->playing, 0);
  atomic_init(&state->loop, 0);
  atomic_init(&state->startSample, (int_least64_t)-1);
  atomic_init(&state->stopSample, (int_least64_t)-1);
  atomic_init(&state->resetPending, 0);
  atomic_init(&state->pcm, NULL);

  NSMutableArray<AVAudioPCMBuffer *> *keepAlive = [NSMutableArray array];
  NSCAudioParam *rateParam = [[NSCAudioParam alloc] initWithContext:context defaultValue:1.0];
  AVAudioFormat *format = context.format
      ?: [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32 sampleRate:44100.0 channels:2 interleaved:NO];
  NSMutableData *timelineCell = context.timelineCell ?: [NSMutableData dataWithLength:NSCTimelineSlots * sizeof(double)];
  __weak NSCAudioBufferSourceNode *weakSelf = self;

  AVAudioSourceNode *source = [[AVAudioSourceNode alloc] initWithFormat:format renderBlock:^OSStatus(BOOL *_Nonnull isSilence, const AudioTimeStamp *_Nonnull timestamp, AVAudioFrameCount frameCount, AudioBufferList *_Nonnull abl) {
    (void)keepAlive;
    NSCBufferSourceState *st = (NSCBufferSourceState *)stateData.mutableBytes;
    if (atomic_load_explicit(&st->playing, memory_order_acquire) != 1) {
      if (isSilence) *isSilence = YES;
      NSCBufferSourceSilence(abl);
      return noErr;
    }
    if (atomic_load_explicit(&st->resetPending, memory_order_acquire)) {
      st->position = st->pendingOffsetFrames;
      atomic_store_explicit(&st->resetPending, 0, memory_order_release);
    }

    int64_t startSample = atomic_load_explicit(&st->startSample, memory_order_acquire);
    int64_t stopSample = atomic_load_explicit(&st->stopSample, memory_order_acquire);
    BOOL loop = atomic_load_explicit(&st->loop, memory_order_acquire) == 1;
    AVAudioPCMBuffer *pcm = (__bridge AVAudioPCMBuffer *)atomic_load_explicit(&st->pcm, memory_order_acquire);

    AVAudioFrameCount first = 0;
    AVAudioFrameCount end = frameCount;
    BOOL finished = NO;
    BOOL hasSampleTime = timestamp && (timestamp->mFlags & kAudioTimeStampSampleTimeValid);
    double sampleRate = format.sampleRate;
    double blockFrame = hasSampleTime ? NSCContextFrame(timestamp->mSampleTime, (const double *)timelineCell.bytes) : 0.0;
    double blockTime = blockFrame / sampleRate;
    if (hasSampleTime) {
      int64_t blockStart = llround(blockFrame);
      int64_t blockEnd = blockStart + (int64_t)frameCount;
      if (stopSample >= 0 && stopSample <= blockStart) {
        end = 0;
        finished = YES;
      } else if (stopSample >= 0 && stopSample < blockEnd) {
        end = (AVAudioFrameCount)(stopSample - blockStart);
        finished = YES;
      }
      if (startSample >= blockEnd && !finished) {
        if (isSilence) *isSilence = YES;
        NSCBufferSourceSilence(abl);
        return noErr;
      }
      if (startSample > blockStart) first = (AVAudioFrameCount)MIN((int64_t)frameCount, startSample - blockStart);
    }

    NSCBufferSourceSilence(abl);
    AVAudioFrameCount frames = pcm ? pcm.frameLength : 0;
    AVAudioChannelCount pcmChannels = pcm ? pcm.format.channelCount : 0;
    float *const *src = pcm ? pcm.floatChannelData : NULL;

    if (first < end && src && frames > 0) {
      double rates[frameCount];
      BOOL rateAutomated = [rateParam fillValuesForRange:blockTime sampleRate:sampleRate frameCount:(NSInteger)frameCount into:rates];
      double pos = st->position;
      AVAudioFrameCount f = first;
      for (; f < end; ++f) {
        if (pos >= (double)frames) {
          if (loop) {
            pos = fmod(pos, (double)frames);
          } else {
            finished = YES;
            break;
          }
        }
        AVAudioFrameCount i0 = (AVAudioFrameCount)pos;
        AVAudioFrameCount i1 = i0 + 1 < frames ? i0 + 1 : (loop ? 0 : i0);
        float frac = (float)(pos - (double)i0);
        for (UInt32 b = 0; b < abl->mNumberBuffers; ++b) {
          float *dst = (float *)abl->mBuffers[b].mData;
          if (!dst) continue;
          const float *s = src[pcmChannels == 1 ? 0 : MIN(b, pcmChannels - 1)];
          dst[f] = s[i0] + (s[i1] - s[i0]) * frac;
        }
        double rate = rateAutomated ? rates[f] : rates[0];
        if (!(rate >= 0.0)) rate = 0.0;
        pos += rate;
      }
      st->position = pos;
    }

    if (finished) {
      atomic_store_explicit(&st->playing, 0, memory_order_release);
      atomic_store_explicit(&st->startSample, (int_least64_t)-1, memory_order_release);
      atomic_store_explicit(&st->stopSample, (int_least64_t)-1, memory_order_release);
      dispatch_async(dispatch_get_main_queue(), ^{
        NSCAudioBufferSourceNode *s = weakSelf;
        if (!s) return;
        if (s.context) [s.context decrementActiveCount];
        if (s.onended) s.onended();
      });
    }
    return noErr;
  }];

  if (self = [super initWithContext:context node:source]) {
    _sourceNode = source;
    _stateData = stateData;
    _state = state;
    _keepAlive = keepAlive;
    _playbackRateParam = rateParam;
    if ([NSThread isMainThread]) {
      [context.engine attachNode:source];
    } else {
      dispatch_sync(dispatch_get_main_queue(), ^{
        [context.engine attachNode:source];
      });
    }
    [self setBuffer:buffer];
  }
  return self;
}

- (instancetype)initWithContext:(NSCAudioContext *)context
                           node:(AVAudioNode *)node {
  return [self initWithContext:context buffer:nil];
}

- (void)dealloc {
  NSCAudioContext *ctx = self.context;
  if (ctx && _sourceNode) {
    AVAudioEngine *eng = ctx.engine;
    if (eng && [ctx isNode:_sourceNode attachedToEngine:eng]) {
      @try { [eng disconnectNodeOutput:_sourceNode]; } @catch (NSException *e) {}
      @try { [ctx detachNode:_sourceNode fromEngine:eng]; } @catch (NSException *e) {}
    }
  }
  _state = NULL;
}

- (NSCAudioBuffer *)buffer {
  return _buffer;
}

- (void)setBuffer:(NSCAudioBuffer *)newBuffer {
  _buffer = newBuffer ? [self processBuffer:newBuffer context:self.context] : nil;
  AVAudioPCMBuffer *pcm = [_buffer getBuffer];
  if (pcm && pcm.floatChannelData == NULL) pcm = nil;
  if (pcm) [_keepAlive addObject:pcm];
  if (_state) atomic_store_explicit(&_state->pcm, (__bridge void *)pcm, memory_order_release);
}

- (BOOL)loop {
  return _state ? atomic_load_explicit(&_state->loop, memory_order_acquire) == 1 : NO;
}

- (void)setLoop:(BOOL)loop {
  if (_state) atomic_store_explicit(&_state->loop, loop ? 1 : 0, memory_order_release);
}

- (NSCAudioParam *)getPlaybackRateParam {
  return _playbackRateParam;
}

- (BOOL)isPlaying {
  return _state ? atomic_load_explicit(&_state->playing, memory_order_acquire) == 1 : NO;
}

- (void)beginAtSample:(int64_t)startSample offset:(double)offset {
  NSCAudioContext *ctx = self.context;
  if (!ctx || !_state || [self isPlaying]) return;
  AVAudioPCMBuffer *pcm = (__bridge AVAudioPCMBuffer *)atomic_load_explicit(&_state->pcm, memory_order_acquire);
  double rate = pcm ? pcm.format.sampleRate : ctx.sampleRate;
  _state->pendingOffsetFrames = (isfinite(offset) && offset > 0.0) ? offset * rate : 0.0;
  atomic_store_explicit(&_state->resetPending, 1, memory_order_release);
  atomic_store_explicit(&_state->stopSample, (int_least64_t)-1, memory_order_release);
  atomic_store_explicit(&_state->startSample, startSample, memory_order_release);
  atomic_store_explicit(&_state->playing, 1, memory_order_release);
  [ctx incrementActiveCount];
  if (!ctx.engine.isRunning) {
    [NSCAudioContext startEngineWithRetry:ctx.engine attempts:3 label:@"bufferSource" asyncCompletion:nil];
  }
}

- (void)start {
  [self startAt:nil offset:0.0];
}

- (void)startAt:(NSNumber *)when {
  [self startAt:when offset:0.0];
}

- (void)startAt:(NSNumber *)when offset:(double)offset {
  NSCAudioContext *ctx = self.context;
  if (!ctx) return;
  int64_t startSample = -1;
  double extra = [ctx extraLatencySeconds];
  double t = when ? when.doubleValue : 0.0;
  if (t > ctx.currentTime || extra > 0.0) {
    startSample = [ctx frameForTime:MAX(t, ctx.currentTime) + extra];
  }
  [self beginAtSample:startSample offset:offset];
}

- (void)stop {
  if (!_state || ![self isPlaying]) return;
  atomic_store_explicit(&_state->playing, 0, memory_order_release);
  atomic_store_explicit(&_state->startSample, (int_least64_t)-1, memory_order_release);
  atomic_store_explicit(&_state->stopSample, (int_least64_t)-1, memory_order_release);
  NSCAudioContext *ctx = self.context;
  if (ctx) [ctx decrementActiveCount];
}

- (void)stopAt:(NSNumber *)when {
  NSCAudioContext *ctx = self.context;
  if (!when || !ctx || when.doubleValue <= ctx.currentTime) {
    BOOL wasPlaying = [self isPlaying];
    [self stop];
    if (wasPlaying && self.onended) self.onended();
    return;
  }
  if (!_state || ![self isPlaying]) return;
  int64_t stopSample = [ctx frameForTime:when.doubleValue];
  atomic_store_explicit(&_state->stopSample, stopSample, memory_order_release);
}

- (NSCAudioBuffer *)processBuffer:(NSCAudioBuffer *)input
                          context:(NSCAudioContext *)context {
  if (!context)
    return nil;
  AVAudioPCMBuffer *srcBuf = [input getBuffer];
  if (!srcBuf)
    return nil;
  AVAudioFormat *mixerFmt = context.format;
  AVAudioChannelCount targetChannels = mixerFmt.channelCount;
  double targetSampleRate = mixerFmt.sampleRate;
  AVAudioCommonFormat targetCommon = mixerFmt.commonFormat;

  if (srcBuf.format.sampleRate == targetSampleRate &&
      srcBuf.format.channelCount == targetChannels &&
      srcBuf.format.commonFormat == targetCommon) {
    return input;
  }

  if (srcBuf.format.sampleRate == targetSampleRate &&
      srcBuf.format.commonFormat == targetCommon) {
    AVAudioChannelCount srcChannels =
        (AVAudioChannelCount)srcBuf.format.channelCount;
    AVAudioChannelCount dstChannels = targetChannels;
    AVAudioFormat *outFmt =
        [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32
                                         sampleRate:srcBuf.format.sampleRate
                                           channels:targetChannels
                                        interleaved:NO];
    AVAudioPCMBuffer *outBuf =
        [[AVAudioPCMBuffer alloc] initWithPCMFormat:outFmt
                                      frameCapacity:srcBuf.frameCapacity];
    outBuf.frameLength = srcBuf.frameLength;
    if (srcBuf.floatChannelData && outBuf.floatChannelData) {
      AVAudioFrameCount frames = srcBuf.frameLength;
      if (srcChannels == 1 && dstChannels >= 1) {
        float *s = srcBuf.floatChannelData[0];
        for (AVAudioChannelCount c = 0; c < dstChannels; ++c) {
          float *d = outBuf.floatChannelData[c];
          for (AVAudioFrameCount i = 0; i < frames; ++i)
            d[i] = s[i];
        }
      } else if (srcChannels >= 1 && dstChannels == 1) {
        float *d = outBuf.floatChannelData[0];
        for (AVAudioFrameCount i = 0; i < frames; ++i) {
          float acc = 0.0f;
          for (AVAudioChannelCount c = 0; c < srcChannels; ++c)
            acc += srcBuf.floatChannelData[c][i];
          d[i] = acc / (float)srcChannels;
        }
      } else {
        AVAudioChannelCount minCh = MIN(srcChannels, dstChannels);
        for (AVAudioChannelCount c = 0; c < minCh; ++c) {
          float *s = srcBuf.floatChannelData[c];
          float *d = outBuf.floatChannelData[c];
          AVAudioFrameCount frames = srcBuf.frameLength;
          for (AVAudioFrameCount i = 0; i < frames; ++i)
            d[i] = s[i];
        }
        if (dstChannels > srcChannels) {
          for (AVAudioChannelCount c = srcChannels; c < dstChannels; ++c) {
            float *d = outBuf.floatChannelData[c];
            for (AVAudioFrameCount i = 0; i < outBuf.frameLength; ++i)
              d[i] = 0.0f;
          }
        }
      }
    }
    return [[NSCAudioBuffer alloc] initWithContext:context
                                                id:[[NSUUID UUID] UUIDString]
                                            buffer:outBuf];
  }

  AVAudioFormat *tgt =
      [[AVAudioFormat alloc] initWithCommonFormat:AVAudioPCMFormatFloat32
                                       sampleRate:targetSampleRate
                                         channels:targetChannels
                                      interleaved:NO];
  AVAudioConverter *conv =
      [[AVAudioConverter alloc] initFromFormat:srcBuf.format toFormat:tgt];
  if (!conv)
    return input;
  double ratio = tgt.sampleRate / srcBuf.format.sampleRate;
  AVAudioFrameCount outFrames =
      (AVAudioFrameCount)((double)srcBuf.frameLength * ratio + 0.5);
  AVAudioPCMBuffer *outBuf =
      [[AVAudioPCMBuffer alloc] initWithPCMFormat:tgt
                                    frameCapacity:MAX(1, outFrames)];
  __block BOOL inputDone = NO;
  __block NSError *convErr = nil;
  AVAudioConverterInputBlock inputBlock =
      ^AVAudioPCMBuffer *_Nullable(AVAudioPacketCount inNumberOfPackets,
                                   AVAudioConverterInputStatus *outStatus) {
    if (inputDone) {
      if (outStatus)
        *outStatus = AVAudioConverterInputStatus_EndOfStream;
      return nil;
    }
    inputDone = YES;
    if (outStatus)
      *outStatus = AVAudioConverterInputStatus_HaveData;
    return srcBuf;
  };
  AVAudioConverterOutputStatus status = [conv convertToBuffer:outBuf
                                                        error:&convErr
                                           withInputFromBlock:inputBlock];
  if (convErr || status == AVAudioConverterOutputStatus_Error) {
    NSCLogError(@"NSCAudioBufferSourceNode: converter error: %@", convErr);
    return input;
  }
  outBuf.frameLength = outFrames;
  return [[NSCAudioBuffer alloc] initWithContext:context
                                              id:[[NSUUID UUID] UUIDString]
                                          buffer:outBuf];
}

@end
