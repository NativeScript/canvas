#import "NSCAudioContext.h"
#import "NSCAudioLog.h"
#import <objc/message.h>
#import <objc/runtime.h>

@implementation NSCAudioBufferSourceNode {
  AVAudioPlayerNode *_player;
  NSCAudioBuffer *_buffer;
  BOOL _isPlaying;
  BOOL _pendingStart;
  int _retryCount;
  BOOL _didFallbackConnect;
  BOOL _gaveUp;
  __weak NSCAudioNode *_lastDestination;
  NSInteger _lastDestinationBus;
  NSInteger _lastSourceBus;
  int _transientRetryAttempts;
}

@synthesize buffer = _buffer;

- (instancetype)initWithContext:(NSCAudioContext *)context
                         buffer:(NSCAudioBuffer *)buffer {
  AVAudioPlayerNode *p = [[AVAudioPlayerNode alloc] init];
  if (self = [super initWithContext:context node:p]) {
    _player = p;
    if ([NSThread isMainThread]) {
      [context.engine attachNode:_player];
    } else {
      dispatch_sync(dispatch_get_main_queue(), ^{
        [context.engine attachNode:_player];
      });
    }
    _buffer = [self processBuffer:buffer context:context];
  }
  return self;
}

- (instancetype)initWithContext:(NSCAudioContext *)context
                           node:(AVAudioNode *)node {
  return [self initWithContext:context buffer:nil];
}

- (NSCAudioBuffer *)buffer {
  return _buffer;
}

- (void)setBuffer:(NSCAudioBuffer *)newBuffer {
  _buffer = [self processBuffer:newBuffer context:self.context];
}

- (BOOL)tryStartPlayer:(AVAudioPlayerNode *)player
               context:(NSCAudioContext *)ctx {
  if (!player || !ctx)
    return NO;
  if (_gaveUp) {
    NSCLogDebug(
        @"NSCAudioBufferSourceNode: tryStartPlayer skipping because _gaveUp=1");
    return NO;
  }

  if (!_isPlaying) {
    NSCLogDebug(@"NSCAudioBufferSourceNode: tryStartPlayer skipping because "
          @"_isPlaying=0");
    return NO;
  }
  AVAudioEngine *eng = ctx.engine;
  if (!eng)
    return NO;
  if (!eng.isRunning)
    return NO;

  @try {
    AVAudioTime *lrt = nil;
    @try {
      lrt = eng.outputNode.lastRenderTime;
    } @catch (NSException *e) {
      lrt = nil;
    }
    if (lrt) {
      NSCLogDebug(@"NSCAudioBufferSourceNode: tryStartPlayer initial engRunning=%d "
            @"lastRenderTime=%p sampleTime=%lld sampleRate=%f",
            (int)eng.isRunning, lrt, (long long)lrt.sampleTime, lrt.sampleRate);
    } else {
      NSCLogDebug(@"NSCAudioBufferSourceNode: tryStartPlayer initial engRunning=%d "
            @"lastRenderTime=NULL",
            (int)eng.isRunning);
    }
  } @catch (NSException *e) {
  }

  if (player.engine != eng) {
    @try {
      if (player.engine) {
        [ctx detachNode:player fromEngine:player.engine];
      }
    } @catch (NSException *e) {
    }
    @try {
      [eng attachNode:player];
    } @catch (NSException *e) {
    }
  }

  BOOL isConnected = NO;
  SEL ocpSel = NSSelectorFromString(@"outputConnectionPointsForNode:bus:");
  if (eng && [eng respondsToSelector:ocpSel]) {
    @try {
      typedef NSArray *(*MsgSendFn)(id, SEL, id, AVAudioNodeBus);
      MsgSendFn fn = (MsgSendFn)objc_msgSend;
      NSArray *pts = fn(eng, ocpSel, player, (AVAudioNodeBus)0);
      isConnected = pts && pts.count > 0;
      NSCLogDebug(@"NSCAudioBufferSourceNode: connectionPoints count=%lu",
            (unsigned long)(pts ? pts.count : 0));
    } @catch (NSException *e) {
      isConnected = NO;
    }
  } else {
    isConnected = (player.engine == eng);
  }

  if (!isConnected) {
    AVAudioNode *targetNode = nil;
    BOOL usingMainMixerFallback = NO;
    if (_lastDestination)
      targetNode = _lastDestination.avNode;
    if (!targetNode) {
      targetNode = eng.mainMixerNode;
      usingMainMixerFallback = YES;
    }
    __block BOOL didImmediateConnect = NO;
    void (^immediateConnectBlock)(void) = ^{
      @try {
        if (player.engine && player.engine != eng) {
          @try {
            [ctx detachNode:player fromEngine:player.engine];
          } @catch (NSException *e) {
          }
        }
        @try {
          [eng attachNode:player];
        } @catch (NSException *e) {
        }
        if (eng && player && targetNode) {
          @try {
            if (_lastDestination && targetNode) {
              AVAudioConnectionPoint *destPoint =
                  [[AVAudioConnectionPoint alloc]
                      initWithNode:targetNode
                               bus:(AVAudioNodeBus)MAX(
                                       0, (int)_lastDestinationBus)];
              [eng connect:player
                  toConnectionPoints:@[ destPoint ]
                             fromBus:(AVAudioNodeBus)MAX(0, (int)_lastSourceBus)
                              format:ctx.format];
            } else {
              [eng connect:(AVAudioNode *)player
                        to:targetNode
                    format:ctx.format];
            }
            @try {
              [eng prepare];
            } @catch (NSException *e) {
            }
            didImmediateConnect = YES;
            NSCLogDebug(@"NSCAudioBufferSourceNode: immediate fallback connect to %@ "
                  @"(usingMain=%d)",
                  NSStringFromClass([targetNode class]),
                  (int)usingMainMixerFallback);
          } @catch (NSException *e) {
            didImmediateConnect = NO;
            NSCLogDebug(@"NSCAudioBufferSourceNode: immediate fallback connect "
                  @"failed: %@",
                  e);
          }
        }
      } @catch (NSException *e) {
      }
    };
    if ([NSThread isMainThread])
      immediateConnectBlock();
    else
      dispatch_sync(dispatch_get_main_queue(), immediateConnectBlock);

    if (didImmediateConnect) {
      _transientRetryAttempts = 0;
      __weak typeof(self) weakSelf2 = self;
      __block int pollAttempts = 8;
      __block void (^pollBlock)(void) = nil;
      pollBlock = ^{
        __strong typeof(weakSelf2) strongSelf = weakSelf2;
        if (!strongSelf)
          return;
        BOOL nowConnected = NO;
        @try {
          SEL ocpSel2 =
              NSSelectorFromString(@"outputConnectionPointsForNode:bus:");
          if (eng && [eng respondsToSelector:ocpSel2]) {
            typedef NSArray *(*MsgSendFn)(id, SEL, id, AVAudioNodeBus);
            MsgSendFn fn = (MsgSendFn)objc_msgSend;
            NSArray *pts = fn(eng, ocpSel2, player, (AVAudioNodeBus)0);
            nowConnected = pts && pts.count > 0;
          } else {
            nowConnected = (player.engine == eng);
          }
        } @catch (NSException *e) {
          nowConnected = NO;
        }

        if (nowConnected) {
          BOOL safeToPlay = NO;
          @try {
            AVAudioTime *now = eng.outputNode.lastRenderTime;
            if (now) {
              AVAudioTime *playerTime = nil;
              @try {
                playerTime = [player playerTimeForNodeTime:now];
              } @catch (NSException *e) {
                playerTime = nil;
              }
              if (playerTime) {
                NSCLogDebug(@"NSCAudioBufferSourceNode: poll immediate now=%p "
                      @"sampleTime=%lld sampleRate=%f playerTime=%p "
                      @"player.sampleTime=%lld player.sampleRate=%f",
                      now, (long long)now.sampleTime, now.sampleRate,
                      playerTime, (long long)playerTime.sampleTime,
                      playerTime.sampleRate);
                safeToPlay = YES;
              } else {
                NSCLogDebug(@"NSCAudioBufferSourceNode: poll immediate now=%p "
                      @"playerTime=nil",
                      now);
#if TARGET_OS_SIMULATOR
                NSCLogDebug(@"NSCAudioBufferSourceNode: simulator forcing play "
                      @"despite nil playerTime (immediate)");
                safeToPlay = YES;
#endif
              }
            }
          } @catch (NSException *e) {
            safeToPlay = NO;
          }

          if (safeToPlay) {
            @try {
              const char *forceImpulseEnv =
                  getenv("NSC_ANALYSER_FORCE_IMPULSE");
              int diagEnv = getenv("NSC_ANALYSER_DIAG") ? 1 : 0;
              if (forceImpulseEnv || diagEnv) {
                AVAudioFormat *fmt = ctx.format;
                NSCLogDebug(@"NSCAudioBufferSourceNode: impulse env force=%s diag=%d "
                      @"ctx=%p fmt=%p",
                      forceImpulseEnv ? forceImpulseEnv : "NULL", diagEnv, ctx,
                      fmt);
                if (fmt) {
                  NSCLogDebug(@"NSCAudioBufferSourceNode: ctx.format channels=%u "
                        @"sampleRate=%f commonFormat=%ld",
                        (unsigned int)fmt.channelCount, fmt.sampleRate,
                        (long)fmt.commonFormat);
                } else {
                  NSCLogDebug(@"NSCAudioBufferSourceNode: ctx.format is NULL — "
                        @"cannot allocate impulse buffer");
                }
                if (fmt && ctx && ctx.engine) {
                  AVAudioFormat *useFmt = fmt;
                  const AudioStreamBasicDescription *u_asbd =
                      useFmt.streamDescription;
                  if (!u_asbd || u_asbd->mFormatID != kAudioFormatLinearPCM) {
                    useFmt = [[AVAudioFormat alloc]
                        initWithCommonFormat:AVAudioPCMFormatFloat32
                                  sampleRate:(useFmt.sampleRate > 0
                                                  ? useFmt.sampleRate
                                                  : 44100.0)
                                    channels:useFmt.channelCount
                                 interleaved:NO];
                  }
                  AVAudioFrameCount impFrames = 128;
                  AVAudioPCMBuffer *imp =
                      [[AVAudioPCMBuffer alloc] initWithPCMFormat:useFmt
                                                    frameCapacity:impFrames];
                  if (imp) {
                    imp.frameLength = impFrames;
                    if (imp.floatChannelData) {
                      for (AVAudioChannelCount c = 0; c < fmt.channelCount;
                           ++c) {
                        float *ch = imp.floatChannelData[c];
                        memset(ch, 0, (size_t)impFrames * sizeof(float));
                        ch[0] = 0.5f;
                      }
                    }
                 
                    NSObject *prodToken_imp = [[NSObject alloc] init];
                    objc_setAssociatedObject(imp, NSCProducerTokenKey,
                                             prodToken_imp,
                                             OBJC_ASSOCIATION_RETAIN_NONATOMIC);
                    NSCLogDebug(@"NSCAudioBufferSourceNode: attached prodToken=%p to "
                          @"imp=%p",
                          (__bridge void *)prodToken_imp, (__bridge void *)imp);
                    @try {
                      NSArray<NSCAnalyserNode *> *analysers =
                          [ctx allAnalyserWrappers];
                      for (NSCAnalyserNode *an in analysers) {
                        @try {
                          [an appendBufferToRing:imp];
                          NSCLogDebug(@"NSCAudioBufferSourceNode: injected impulse "
                                @"into analyser %@ (direct) token=%p buf=%p",
                                NSStringFromClass([an class]),
                                (__bridge void *)prodToken_imp,
                                (__bridge void *)imp);
                        } @catch (NSException *e) {
                          NSCLogDebug(@"NSCAudioBufferSourceNode: appendBufferToRing "
                                @"exception: %@",
                                e);
                        }
                      }
                    } @catch (NSException *e) {
                    } @
                    try {
                      AVAudioEngine *e = ctx.engine;
                      NSMutableArray *queue = [NSMutableArray
                          arrayWithObject:(AVAudioNode *)player];
                      NSMutableSet *seen = [NSMutableSet set];
                      while (queue.count > 0) {
                        AVAudioNode *n = queue.firstObject;
                        [queue removeObjectAtIndex:0];
                        if (!n)
                          continue;
                        NSCAudioNode *wrap = [ctx nodeWrapperForAVNode:n];
                        if (wrap &&
                            [wrap isKindOfClass:[NSCAnalyserNode class]]) {
                          NSCAnalyserNode *an = (NSCAnalyserNode *)wrap;
                          @try {
                            [an appendBufferToRing:imp];
                            NSCLogDebug(@"NSCAudioBufferSourceNode: injected impulse "
                                  @"into analyser %@ token=%p buf=%p",
                                  NSStringFromClass([an class]),
                                  (__bridge void *)prodToken_imp,
                                  (__bridge void *)imp);
                          } @catch (NSException *e) {
                            NSCLogDebug(@"NSCAudioBufferSourceNode: "
                                  @"appendBufferToRing exception: %@",
                                  e);
                          }
                        }
                        SEL ocpSel = NSSelectorFromString(
                            @"outputConnectionPointsForNode:bus:");
                        if (e && [e respondsToSelector:ocpSel]) {
                          typedef NSArray *(*MsgSendFn)(id, SEL, id,
                                                        AVAudioNodeBus);
                          MsgSendFn fn = (MsgSendFn)objc_msgSend;
                          NSArray *pts = fn(e, ocpSel, n, (AVAudioNodeBus)0);
                          for (AVAudioConnectionPoint *p in pts) {
                            if (!p.node)
                              continue;
                            NSValue *v =
                                [NSValue valueWithPointer:(__bridge const void
                                                               *)(p.node)];
                            if ([seen containsObject:v])
                              continue;
                            [seen addObject:v];
                            [queue addObject:p.node];
                          }
                        }
                      }
                    } @catch (NSException *e) {
                    }
                  }
                }
              }
#if TARGET_OS_SIMULATOR
              
              AVAudioPCMBuffer *localPcm = [_buffer getBuffer];
              if (localPcm && ctx && ctx.engine) {
                
                NSObject *prodToken_local = [[NSObject alloc] init];
                objc_setAssociatedObject(localPcm, NSCProducerTokenKey,
                                         prodToken_local,
                                         OBJC_ASSOCIATION_RETAIN_NONATOMIC);
                NSCLogDebug(@"NSCAudioBufferSourceNode: attached prodToken=%p to "
                      @"localPcm=%p",
                      (__bridge void *)prodToken_local,
                      (__bridge void *)localPcm);
                
                AVAudioFrameCount lf = localPcm.frameLength;
                float *const *ldata = localPcm.floatChannelData;
                AVAudioChannelCount lch = localPcm.format.channelCount;
                int snapN = (int)MIN((AVAudioFrameCount)8, lf);
                NSMutableString *ls = [NSMutableString
                    stringWithFormat:@"NSCAudioBufferSourceNode: localPcm "
                                     @"frames=%u ch=%u buf=%p chData=%p first:",
                                     (unsigned)lf, (unsigned)lch,
                                     (void *)localPcm, (void *)ldata];
                if (ldata && ldata[0]) {
                  for (int i = 0; i < snapN; ++i)
                    [ls appendFormat:@" %f", ldata[0][i]];
                } else {
                  [ls appendString:@" (NULL)"];
                }
                NSCLogDebug(@"%@", ls);

                AVAudioEngine *e = ctx.engine;
                NSMutableArray *queue =
                    [NSMutableArray arrayWithObject:(AVAudioNode *)player];
                NSMutableSet *seen = [NSMutableSet set];
                while (queue.count > 0) {
                  AVAudioNode *n = queue.firstObject;
                  [queue removeObjectAtIndex:0];
                  if (!n)
                    continue;
                  NSCAudioNode *wrap = [ctx nodeWrapperForAVNode:n];
                  NSCLogDebug(@"NSCAudioBufferSourceNode: traverse node %p class=%@ "
                        @"-> wrap=%p wrapClass=%@",
                        n, NSStringFromClass([n class]), wrap,
                        (wrap ? NSStringFromClass([wrap class]) : @"(nil)"));
                  if (wrap && [wrap isKindOfClass:[NSCAnalyserNode class]]) {
                    NSCAnalyserNode *an = (NSCAnalyserNode *)wrap;
                    @try {
                      [an appendBufferToRing:localPcm];
                      NSCLogDebug(@"NSCAudioBufferSourceNode: appended buffer to "
                            @"analyser %@ token=%p buf=%p (direct)",
                            NSStringFromClass([an class]),
                            (__bridge void *)prodToken_local,
                            (__bridge void *)localPcm);
                    } @catch (NSException *e) {
                      NSCLogDebug(@"NSCAudioBufferSourceNode: appendBufferToRing "
                            @"exception: %@",
                            e);
                    }
                  }
                  SEL ocpSel = NSSelectorFromString(
                      @"outputConnectionPointsForNode:bus:");
                  if (e && [e respondsToSelector:ocpSel]) {
                    typedef NSArray *(*MsgSendFn)(id, SEL, id, AVAudioNodeBus);
                    MsgSendFn fn = (MsgSendFn)objc_msgSend;
                    NSArray *pts = fn(e, ocpSel, n, (AVAudioNodeBus)0);
                    for (AVAudioConnectionPoint *p in pts) {
                      if (!p.node)
                        continue;
                      NSValue *v = [NSValue
                          valueWithPointer:(__bridge const void *)(p.node)];
                      if ([seen containsObject:v])
                        continue;
                      [seen addObject:v];
                      [queue addObject:p.node];
                    }
                  }
                }
              }
#endif
              [player play];
              if (_pendingStart) {
                _pendingStart = NO;
                if (ctx)
                  [ctx unregisterPendingNode:strongSelf];
              }
              _retryCount = 0;
            } @catch (NSException *ex) {
              @try {
                [player stop];
              } @catch (NSException *inner) {
              }
              [strongSelf scheduleRetryWithContext:ctx];
            }
            return;
          }
        }

        pollAttempts -= 1;
        if (pollAttempts > 0) {
          dispatch_after(
              dispatch_time(DISPATCH_TIME_NOW, (int64_t)(0.01 * NSEC_PER_SEC)),
              dispatch_get_main_queue(), pollBlock);
          return;
        }

        [strongSelf scheduleRetryWithContext:ctx];
      };
      dispatch_after(
          dispatch_time(DISPATCH_TIME_NOW, (int64_t)(0.01 * NSEC_PER_SEC)),
          dispatch_get_main_queue(), pollBlock);
      return YES;
    }

    if (_transientRetryAttempts < 3) {
      _transientRetryAttempts += 1;
      NSCLogDebug(@"NSCAudioBufferSourceNode: transient recheck %d waiting for "
            @"connection",
            _transientRetryAttempts);
      dispatch_after(
          dispatch_time(DISPATCH_TIME_NOW, (int64_t)(0.02 * NSEC_PER_SEC)),
          dispatch_get_main_queue(), ^{
            @try {
              [self tryStartPlayer:player context:ctx];
            } @catch (NSException *e) {
              NSCLogDebug(@"NSCAudioBufferSourceNode: transient tryStartPlayer "
                    @"threw: %@",
                    e);
            }
          });
      return NO;
    }
    _transientRetryAttempts = 0;
    [self scheduleRetryWithContext:ctx];
    return NO;
  }

  _transientRetryAttempts = 0;
  __weak typeof(self) weakSelf3 = self;
  __block int pollAttempts2 = 8;
  __block void (^pollBlock2)(void) = nil;
  pollBlock2 = ^{
    __strong typeof(weakSelf3) strongSelf = weakSelf3;
    if (!strongSelf)
      return;
    BOOL nowConnected = NO;
    @try {
      SEL ocpSel3 = NSSelectorFromString(@"outputConnectionPointsForNode:bus:");
      if (eng && [eng respondsToSelector:ocpSel3]) {
        typedef NSArray *(*MsgSendFn)(id, SEL, id, AVAudioNodeBus);
        MsgSendFn fn = (MsgSendFn)objc_msgSend;
        NSArray *pts = fn(eng, ocpSel3, player, (AVAudioNodeBus)0);
        nowConnected = pts && pts.count > 0;
      } else {
        nowConnected = (player.engine == eng);
      }
    } @catch (NSException *e) {
      nowConnected = NO;
    }

    if (nowConnected) {
      BOOL safeToPlay = NO;
      @try {
        AVAudioTime *now = eng.outputNode.lastRenderTime;
        if (now) {
          AVAudioTime *playerTime = nil;
          @try {
            playerTime = [player playerTimeForNodeTime:now];
          } @catch (NSException *e) {
            playerTime = nil;
          }
          if (playerTime) {
            NSCLogDebug(@"NSCAudioBufferSourceNode: poll now=%p sampleTime=%lld "
                  @"sampleRate=%f playerTime=%p player.sampleTime=%lld "
                  @"player.sampleRate=%f",
                  now, (long long)now.sampleTime, now.sampleRate, playerTime,
                  (long long)playerTime.sampleTime, playerTime.sampleRate);
            safeToPlay = YES;
          } else {
            NSCLogDebug(@"NSCAudioBufferSourceNode: poll now=%p playerTime=nil", now);
#if TARGET_OS_SIMULATOR
            NSCLogDebug(@"NSCAudioBufferSourceNode: simulator forcing play despite "
                  @"nil playerTime");
            safeToPlay = YES;
#endif
          }
        }
      } @catch (NSException *e) {
        safeToPlay = NO;
      }

      if (safeToPlay) {
        @try {
          const char *forceImpulseEnv = getenv("NSC_ANALYSER_FORCE_IMPULSE");
          int diagEnv = getenv("NSC_ANALYSER_DIAG") ? 1 : 0;
          if (forceImpulseEnv || diagEnv) {
            AVAudioFormat *fmt = ctx.format;
            NSCLogDebug(@"NSCAudioBufferSourceNode: impulse env force=%s diag=%d "
                  @"ctx=%p fmt=%p",
                  forceImpulseEnv ? forceImpulseEnv : "NULL", diagEnv, ctx,
                  fmt);
            if (fmt) {
              NSCLogDebug(@"NSCAudioBufferSourceNode: ctx.format channels=%u "
                    @"sampleRate=%f commonFormat=%ld",
                    (unsigned int)fmt.channelCount, fmt.sampleRate,
                    (long)fmt.commonFormat);
            } else {
              NSCLogDebug(@"NSCAudioBufferSourceNode: ctx.format is NULL — cannot "
                    @"allocate impulse buffer");
            }
            if (fmt && ctx && ctx.engine) {
              AVAudioFormat *useFmt2 = fmt;
              const AudioStreamBasicDescription *u2_asbd =
                  useFmt2.streamDescription;
              if (!u2_asbd || u2_asbd->mFormatID != kAudioFormatLinearPCM) {
                useFmt2 = [[AVAudioFormat alloc]
                    initWithCommonFormat:AVAudioPCMFormatFloat32
                              sampleRate:(useFmt2.sampleRate > 0
                                              ? useFmt2.sampleRate
                                              : 44100.0)
                                channels:useFmt2.channelCount
                             interleaved:NO];
              }
              AVAudioFrameCount impFrames = 128;
              AVAudioPCMBuffer *imp =
                  [[AVAudioPCMBuffer alloc] initWithPCMFormat:useFmt2
                                                frameCapacity:impFrames];
              if (imp) {
                imp.frameLength = impFrames;
                if (imp.floatChannelData) {
                  for (AVAudioChannelCount c = 0; c < fmt.channelCount; ++c) {
                    float *ch = imp.floatChannelData[c];
                    memset(ch, 0, (size_t)impFrames * sizeof(float));
                    ch[0] = 0.5f;
                  }
                }
               

                NSObject *prodToken_imp2 = [[NSObject alloc] init];
                objc_setAssociatedObject(imp, NSCProducerTokenKey,
                                         prodToken_imp2,
                                         OBJC_ASSOCIATION_RETAIN_NONATOMIC);
                NSCLogDebug(@"NSCAudioBufferSourceNode: attached prodToken=%p to "
                      @"imp=%p (branch2)",
                      (__bridge void *)prodToken_imp2, (__bridge void *)imp);
                @try {
                  AVAudioEngine *e = ctx.engine;
                  NSMutableArray *queue =
                      [NSMutableArray arrayWithObject:(AVAudioNode *)player];
                  NSMutableSet *seen = [NSMutableSet set];
                  while (queue.count > 0) {
                    AVAudioNode *n = queue.firstObject;
                    [queue removeObjectAtIndex:0];
                    if (!n)
                      continue;
                    NSCAudioNode *wrap = [ctx nodeWrapperForAVNode:n];
                    NSCLogDebug(@"NSCAudioBufferSourceNode: traverse node %p "
                          @"class=%@ -> wrap=%p wrapClass=%@",
                          n, NSStringFromClass([n class]), wrap,
                          (wrap ? NSStringFromClass([wrap class]) : @"(nil)"));
                    if (wrap && [wrap isKindOfClass:[NSCAnalyserNode class]]) {
                      NSCAnalyserNode *an = (NSCAnalyserNode *)wrap;
                      @try {
                        [an appendBufferToRing:imp];
                        NSCLogDebug(@"NSCAudioBufferSourceNode: injected impulse "
                              @"into analyser %@ token=%p buf=%p",
                              NSStringFromClass([an class]),
                              (__bridge void *)prodToken_imp2,
                              (__bridge void *)imp);
                      } @catch (NSException *e) {
                        NSCLogDebug(@"NSCAudioBufferSourceNode: appendBufferToRing "
                              @"exception: %@",
                              e);
                      }
                    }
                    SEL ocpSel = NSSelectorFromString(
                        @"outputConnectionPointsForNode:bus:");
                    if (e && [e respondsToSelector:ocpSel]) {
                      typedef NSArray *(*MsgSendFn)(id, SEL, id,
                                                    AVAudioNodeBus);
                      MsgSendFn fn = (MsgSendFn)objc_msgSend;
                      NSArray *pts = fn(e, ocpSel, n, (AVAudioNodeBus)0);
                      for (AVAudioConnectionPoint *p in pts) {
                        if (!p.node)
                          continue;
                        NSValue *v = [NSValue
                            valueWithPointer:(__bridge const void *)(p.node)];
                        if ([seen containsObject:v])
                          continue;
                        [seen addObject:v];
                        [queue addObject:p.node];
                      }
                    }
                  }
                } @catch (NSException *e) {
                }
              }
            }
          }
          [player play];
          if (_pendingStart) {
            _pendingStart = NO;
            if (ctx)
              [ctx unregisterPendingNode:strongSelf];
          }
          _retryCount = 0;
#if TARGET_OS_SIMULATOR
          
          AVAudioPCMBuffer *localPcm = [_buffer getBuffer];
          if (localPcm && ctx && ctx.engine) {
            AVAudioEngine *e = ctx.engine;
            NSObject *prodToken_local2 = [[NSObject alloc] init];
            objc_setAssociatedObject(localPcm, NSCProducerTokenKey,
                                     prodToken_local2,
                                     OBJC_ASSOCIATION_RETAIN_NONATOMIC);
            NSCLogDebug(@"NSCAudioBufferSourceNode: attached prodToken=%p to "
                  @"localPcm=%p (post-play)",
                  (__bridge void *)prodToken_local2, (__bridge void *)localPcm);
            NSMutableArray *queue =
                [NSMutableArray arrayWithObject:(AVAudioNode *)player];
            NSMutableSet *seen = [NSMutableSet set];
            while (queue.count > 0) {
              AVAudioNode *n = queue.firstObject;
              [queue removeObjectAtIndex:0];
              if (!n)
                continue;
              NSCAudioNode *wrap = [ctx nodeWrapperForAVNode:n];
              NSCLogDebug(@"NSCAudioBufferSourceNode: traverse node %p class=%@ -> "
                    @"wrap=%p wrapClass=%@",
                    n, NSStringFromClass([n class]), wrap,
                    (wrap ? NSStringFromClass([wrap class]) : @"(nil)"));
              if (wrap && [wrap isKindOfClass:[NSCAnalyserNode class]]) {
                NSCAnalyserNode *an = (NSCAnalyserNode *)wrap;
                @try {
                  [an appendBufferToRing:localPcm];
                  NSCLogDebug(@"NSCAudioBufferSourceNode: appended buffer to "
                        @"analyser %@ token=%p buf=%p (post-play)",
                        NSStringFromClass([an class]),
                        (__bridge void *)prodToken_local2,
                        (__bridge void *)localPcm);
                } @catch (NSException *e) {
                  NSCLogDebug(@"NSCAudioBufferSourceNode: appendBufferToRing "
                        @"exception: %@",
                        e);
                }
              }
              SEL ocpSel =
                  NSSelectorFromString(@"outputConnectionPointsForNode:bus:");
              if (e && [e respondsToSelector:ocpSel]) {
                typedef NSArray *(*MsgSendFn)(id, SEL, id, AVAudioNodeBus);
                MsgSendFn fn = (MsgSendFn)objc_msgSend;
                NSArray *pts = fn(e, ocpSel, n, (AVAudioNodeBus)0);
                for (AVAudioConnectionPoint *p in pts) {
                  if (!p.node)
                    continue;
                  NSValue *v = [NSValue
                      valueWithPointer:(__bridge const void *)(p.node)];
                  if ([seen containsObject:v])
                    continue;
                  [seen addObject:v];
                  [queue addObject:p.node];
                }
              }
            }
          }
#endif
        } @catch (NSException *ex) {
          AVAudioEngine *eng2 = ctx.engine;
          BOOL engRunning = eng2 ? eng2.isRunning : NO;
          id attachedNodes = nil;
          if (eng2 && [eng2 respondsToSelector:@selector(attachedNodes)]) {
            @try {
              attachedNodes = [eng2 attachedNodes];
            } @catch (NSException *e) {
              attachedNodes = nil;
            }
          }
          NSCLogDebug(@"NSCAudioBufferSourceNode: deferred play exception: %@; "
                @"ctx=%p eng=%p engRunning=%d player=%p player.engine=%p "
                @"attachedNodesCount=%lu",
                ex, ctx, eng2, engRunning, player, player.engine,
                attachedNodes ? (unsigned long)[attachedNodes count] : 0ul);
          @try {
            [player stop];
          } @catch (NSException *inner) {
          }
          [strongSelf scheduleRetryWithContext:ctx];
        }
        return;
      }
    }

    pollAttempts2 -= 1;
    if (pollAttempts2 > 0) {
      dispatch_after(
          dispatch_time(DISPATCH_TIME_NOW, (int64_t)(0.01 * NSEC_PER_SEC)),
          dispatch_get_main_queue(), pollBlock2);
      return;
    }

    [strongSelf scheduleRetryWithContext:ctx];
  };
  dispatch_after(
      dispatch_time(DISPATCH_TIME_NOW, (int64_t)(0.01 * NSEC_PER_SEC)),
      dispatch_get_main_queue(), pollBlock2);
  return YES;
}

- (void)registerPendingIfNeededWithContext:(NSCAudioContext *)ctx {
  if (_gaveUp)
    return;
  if (!_pendingStart) {
    _pendingStart = YES;
    if (ctx)
      [ctx registerPendingNode:self];
  }
  if (ctx && ctx.engine) {
    NSCAudioContext_scheduleResumeOnEngineStart(ctx.engine, 0.05);
    NSCLogDebug(@"NSCAudioBufferSourceNode: registered pending node %@ (retry=%d)",
          self, _retryCount);
  }
}

- (void)scheduleRetryWithContext:(NSCAudioContext *)ctx {
  if (!ctx)
    return;
  if (_gaveUp)
    return;
  if (_retryCount >= 6) {
    NSCLogError(@"NSCAudioBufferSourceNode: giving up after %d retries", _retryCount);
    _pendingStart = NO;
    _gaveUp = YES;
    if (ctx)
      [ctx unregisterPendingNode:self];
    if (_isPlaying) {
      _isPlaying = NO;
      if (self.context)
        [self.context decrementActiveCount];
    }
    if (self.onended)
      self.onended();
    return;
  }
  double delay = 0.05 * pow(2.0, (double)_retryCount);
  if (delay < 0.05)
    delay = 0.05;
  if (delay > 1.0)
    delay = 1.0;
  _retryCount += 1;
  if (_retryCount >= 3 && !_didFallbackConnect) {
    _didFallbackConnect = YES;
    AVAudioEngine *eng = ctx.engine;
    AVAudioPlayerNode *player = (AVAudioPlayerNode *)self.avNode;
    AVAudioNode *targetNode = nil;
    BOOL usingMainMixerFallback = NO;
    if (_lastDestination)
      targetNode = _lastDestination.avNode;
    if (!targetNode) {
      targetNode = eng.mainMixerNode;
      usingMainMixerFallback = YES;
    }
    NSCLogDebug(@"NSCAudioBufferSourceNode: fallback path: _lastDestination=%@ "
          @"targetNode=%@ usingMainMixer=%d",
          _lastDestination ? NSStringFromClass([_lastDestination class])
                           : @"(nil)",
          NSStringFromClass([targetNode class]), (int)usingMainMixerFallback);
    void (^fallbackConnectBlock)(void) = ^{
      @try {
        if (eng && player && targetNode) {
                    if (player.engine && player.engine != eng) {
            @try {
              [ctx detachNode:player fromEngine:player.engine];
            } @catch (NSException *e) {
            }
          }
          @try {
            [eng attachNode:player];
          } @catch (NSException *e) {
          }
          if (_lastDestination && targetNode) {
            AVAudioConnectionPoint *destPoint = [[AVAudioConnectionPoint alloc]
                initWithNode:targetNode
                         bus:(AVAudioNodeBus)MAX(0, (int)_lastDestinationBus)];
            @try {
              [eng connect:player
                  toConnectionPoints:@[ destPoint ]
                             fromBus:(AVAudioNodeBus)MAX(0, (int)_lastSourceBus)
                              format:ctx.format];
            } @catch (NSException *e) {
              @try {
                [eng connect:(AVAudioNode *)player
                          to:targetNode
                      format:ctx.format];
              } @catch (NSException *e2) {
              }
            }
          } else {
            @try {
              [eng connect:(AVAudioNode *)player
                        to:targetNode
                    format:ctx.format];
            } @catch (NSException *e) {
            }
          }
          @try {
            [eng prepare];
          } @catch (NSException *e) {
          }
          NSCLogDebug(@"NSCAudioBufferSourceNode: fallback connected player to %@",
                targetNode);
        }
      } @catch (NSException *e) {
        NSCLogDebug(@"NSCAudioBufferSourceNode: fallback connect failed: %@", e);
      }
    };
    if ([NSThread isMainThread])
      fallbackConnectBlock();
    else
      dispatch_sync(dispatch_get_main_queue(), fallbackConnectBlock);
  }
  if (!_pendingStart) {
    _pendingStart = YES;
    if (ctx)
      [ctx registerPendingNode:self];
  }
  @try {
    AVAudioTime *lrt =
        ctx && ctx.engine ? ctx.engine.outputNode.lastRenderTime : nil;
    if (lrt) {
      NSCLogDebug(@"NSCAudioBufferSourceNode: scheduling retry %d after %.3fs "
            @"engRunning=%d lastRender=%p sampleTime=%lld sampleRate=%f",
            _retryCount, delay, (int)(ctx.engine.isRunning), lrt,
            (long long)lrt.sampleTime, lrt.sampleRate);
    } else {
      NSCLogDebug(@"NSCAudioBufferSourceNode: scheduling retry %d after %.3fs "
            @"engRunning=%d lastRender=NULL",
            _retryCount, delay, (int)(ctx.engine ? ctx.engine.isRunning : 0));
    }
  } @catch (NSException *e) {
    NSCLogDebug(@"NSCAudioBufferSourceNode: scheduling retry %d after %.3fs",
          _retryCount, delay);
  }
  dispatch_after(
      dispatch_time(DISPATCH_TIME_NOW, (int64_t)(delay * NSEC_PER_SEC)),
      dispatch_get_main_queue(), ^{
        @try {
          [self handleConnectedTo:nil output:nil input:nil];
        } @catch (NSException *e) {
          NSCLogDebug(@"NSCAudioBufferSourceNode: scheduled retry threw: %@", e);
        }
      });
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

- (void)start {
  NSCAudioContext *ctx = self.context;
  if (!ctx)
    return;
  _gaveUp = NO;
  AVAudioPlayerNode *player = (AVAudioPlayerNode *)self.avNode;
  AVAudioPCMBuffer *pcm = [_buffer getBuffer];
  if (pcm != nil) {
    __weak typeof(self) weakSelf = self;

    if (!_isPlaying) {
      _isPlaying = YES;
      [ctx incrementActiveCount];
    }

    AVAudioTime *startTime = nil;
    double extra = [ctx extraLatencySeconds];
    if (extra > 0.0 && ctx.engine.isRunning) {
      AVAudioTime *now = ctx.engine.outputNode.lastRenderTime;
      if (now) {
        AVAudioTime *playerTime = [player playerTimeForNodeTime:now];
        if (playerTime) {
          AVAudioFramePosition offset =
              (AVAudioFramePosition)llround(extra * playerTime.sampleRate);
          AVAudioFramePosition startSample = playerTime.sampleTime + offset;
          startTime = [AVAudioTime timeWithSampleTime:startSample
                                               atRate:playerTime.sampleRate];
        }
      }
    }

    [player scheduleBuffer:pcm
                    atTime:startTime
                   options:(self.loop ? AVAudioPlayerNodeBufferLoops
                                      : 0)completionHandler:^{
                     dispatch_async(dispatch_get_main_queue(), ^{
                       __strong typeof(weakSelf) strongSelf = weakSelf;
                       if (!strongSelf)
                         return;
                       if (strongSelf->_isPlaying) {
                         strongSelf->_isPlaying = NO;
                         [strongSelf.context decrementActiveCount];
                       }
                       if (strongSelf.onended)
                         strongSelf.onended();
                     });
                   }];
  }
  __weak typeof(self) weakSelf = self;

  if (ctx.engine.isRunning) {
    if ([self tryStartPlayer:player context:ctx]) {
      return;
    } else {
      [self registerPendingIfNeededWithContext:ctx];
      return;
    }
  }

  BOOL syncStarted = [NSCAudioContext
      startEngineWithRetry:ctx.engine
                  attempts:3
                     label:@"bufferSource"
           asyncCompletion:^(BOOL ok) {
             if (!ok)
               return;
             __strong typeof(weakSelf) strongSelf = weakSelf;
             if (!strongSelf)
               return;
             NSCAudioContext *sctx = strongSelf.context;
             AVAudioPlayerNode *splayer =
                 (AVAudioPlayerNode *)strongSelf.avNode;
             if (![strongSelf tryStartPlayer:splayer context:sctx]) {
               [strongSelf registerPendingIfNeededWithContext:sctx];
             }
           }];
  if (syncStarted) {
    if (![self tryStartPlayer:player context:ctx]) {
      [self registerPendingIfNeededWithContext:ctx];
    }
  }
}

- (void)stop {
  AVAudioPlayerNode *player = (AVAudioPlayerNode *)self.avNode;
  [player stop];
  if (_isPlaying) {
    _isPlaying = NO;
    if (self.context)
      [self.context decrementActiveCount];
  }
  _gaveUp = YES;
  NSCLogDebug(@"NSCAudioBufferSourceNode: stop called — _gaveUp=1, _isPlaying=%d",
        (int)_isPlaying);

  if (_pendingStart) {
    _pendingStart = NO;
    if (self.context)
      [self.context unregisterPendingNode:self];
  }
  _retryCount = 0;

  AVAudioEngine *eng = self.context ? self.context.engine : nil;
  if (eng) {
    void (^detachBlock)(void) = ^{
      AVAudioPlayerNode *p = _player;
      if (p) {
        @try {
          if (p.engine && p.engine != eng) {
            @try {
              [self.context detachNode:p fromEngine:p.engine];
            } @catch (NSException *e) {
            }
          }
        } @catch (NSException *e) {
        }
        @try {
          // Only detach from `eng` if the player is currently attached to it.
          if (p.engine == eng) {
            [self.context detachNode:p fromEngine:eng];
          }
        } @catch (NSException *e) {
        }
      }
    };
    if ([NSThread isMainThread]) {
      detachBlock();
    } else {
      dispatch_sync(dispatch_get_main_queue(), detachBlock);
    }
    NSCLogDebug(@"NSCAudioBufferSourceNode: stop requested "
          @"context.stopManualRenderer and detached player");
  }
}

- (void)handleConnectedTo:(NSCAudioNode *)destination
                   output:(NSNumber *)output
                    input:(NSNumber *)input {
  if (destination) {
    _lastDestination = destination;
    _didFallbackConnect = NO;
    _lastDestinationBus = input ? input.intValue : 0;
    _lastSourceBus = output ? output.intValue : 0;
  }
  dispatch_async(dispatch_get_main_queue(), ^{
    if (!_pendingStart)
      return;
    NSCLogDebug(@"NSCAudioBufferSourceNode: handleConnectedTo dest=%p pending=%d",
          destination, _pendingStart);
    NSCAudioContext *ctx = self.context;
    AVAudioPlayerNode *player = (AVAudioPlayerNode *)self.avNode;
    AVAudioPCMBuffer *pcm = [_buffer getBuffer];
    if (!pcm) {
      if (_isPlaying) {
        _isPlaying = NO;
        if (self.context)
          [self.context decrementActiveCount];
      }
      _pendingStart = NO;
      if (self.context)
        [self.context unregisterPendingNode:self];
      return;
    }

    @try {
      [player stop];
    } @catch (NSException *e) {
    }

    __weak typeof(self) weakSelf = self;

    AVAudioTime *startTime = nil;
    double extra = [ctx extraLatencySeconds];
    if (extra > 0.0 && ctx.engine.isRunning) {
      AVAudioTime *now = ctx.engine.outputNode.lastRenderTime;
      if (now) {
        AVAudioTime *playerTime = [player playerTimeForNodeTime:now];
        if (playerTime) {
          AVAudioFramePosition offset =
              (AVAudioFramePosition)llround(extra * playerTime.sampleRate);
          AVAudioFramePosition startSample = playerTime.sampleTime + offset;
          startTime = [AVAudioTime timeWithSampleTime:startSample
                                               atRate:playerTime.sampleRate];
        }
      }
    }

    [player scheduleBuffer:pcm
                    atTime:startTime
                   options:(self.loop ? AVAudioPlayerNodeBufferLoops
                                      : 0)completionHandler:^{
                     dispatch_async(dispatch_get_main_queue(), ^{
                       __strong typeof(weakSelf) strongSelf = weakSelf;
                       if (!strongSelf)
                         return;
                       if (strongSelf->_isPlaying) {
                         strongSelf->_isPlaying = NO;
                         [strongSelf.context decrementActiveCount];
                       }
                       if (strongSelf.onended)
                         strongSelf.onended();
                     });
                   }];

    if (ctx.engine.isRunning) {
      if ([self tryStartPlayer:player context:ctx]) {
        return;
      } else {
        [self registerPendingIfNeededWithContext:ctx];
        return;
      }
    }

    BOOL syncStarted = [NSCAudioContext
        startEngineWithRetry:ctx.engine
                    attempts:3
                       label:@"bufferSource"
             asyncCompletion:^(BOOL ok) {
               if (!ok)
                 return;
               __strong typeof(weakSelf) strongSelf = weakSelf;
               if (!strongSelf)
                 return;
               NSCAudioContext *sctx = strongSelf.context;
               AVAudioPlayerNode *splayer =
                   (AVAudioPlayerNode *)strongSelf.avNode;
               if (![strongSelf tryStartPlayer:splayer context:sctx]) {
                 [strongSelf registerPendingIfNeededWithContext:sctx];
               }
             }];
    if (syncStarted) {
      if (![self tryStartPlayer:player context:ctx]) {
        [self registerPendingIfNeededWithContext:ctx];
      }
    }
  });
}

@end
