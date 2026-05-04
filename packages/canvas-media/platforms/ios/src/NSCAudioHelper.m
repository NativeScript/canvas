//
//  NSCAudioHelper.m
//  CanvasMedia
//
#import "NSCAudioHelper.h"
#if __has_include(<UniformTypeIdentifiers/UniformTypeIdentifiers.h>)
#import <UniformTypeIdentifiers/UniformTypeIdentifiers.h>
#endif
#import "NSCAudioView.h"

@implementation NSCAudioHelper {
    AVPlayer *_playerInternal;
    AVPlayerItem *_currentItemInternal;
    BOOL _inForeground;
    NSCPlayerState _stateInternal;
    NSCPlayerReadyState _readyStateInternal;
    NSString *_src;
    id _playEndNotificationId;
    id _resumeListenerId;
    id _suspendListenerId;
    id _playbackTimeObserver;
    BOOL _loadedDataFired;
    BOOL _canPlayFired;
    BOOL _canPlayThroughFired;
    NSCAudioView *_audioView;
    BOOL _controlsInternal;
    NSInteger _durationRetryCount;
    BOOL _httpFallbackTried;
}

- (instancetype)init {
    if (self = [super init]) {
        _playerInternal = [[AVPlayer alloc] init];
        _inForeground = YES;
        _readyStateInternal = NSCPlayerReadyStateHaveNothing;
        _stateInternal = NSCPlayerStateIdle;

        @try {
            AVAudioSession *session = [AVAudioSession sharedInstance];
            NSError *sessionErr = nil;
            [session setCategory:AVAudioSessionCategoryPlayback error:&sessionErr];
            if (sessionErr) NSLog(@"NSCAudioHelper: setCategory error: %@", sessionErr);
            [session setActive:YES error:&sessionErr];
            if (sessionErr) NSLog(@"NSCAudioHelper: setActive error: %@", sessionErr);
        } @catch (NSException *ex) {
            NSLog(@"NSCAudioHelper: audio session setup failed: %@", ex);
        }

        __weak typeof(self) weakSelf = self;
        _suspendListenerId = [[NSNotificationCenter defaultCenter]
                              addObserverForName:UIApplicationDidEnterBackgroundNotification
                              object:nil queue:nil
                              usingBlock:^(NSNotification *note) {
            __strong typeof(weakSelf) strongSelf = weakSelf;
            if (!strongSelf) return;
            strongSelf->_inForeground = NO;
        }];

        _resumeListenerId = [[NSNotificationCenter defaultCenter]
                             addObserverForName:UIApplicationDidBecomeActiveNotification
                             object:nil queue:nil
                             usingBlock:^(NSNotification *note) {
            __strong typeof(weakSelf) strongSelf = weakSelf;
            if (!strongSelf) return;
            strongSelf->_inForeground = YES;
        }];

        @try {
            _audioView = [[NSCAudioView alloc] initWithHelper:self];
            _audioView.hidden = YES;
            _controlsInternal = NO;
        } @catch (NSException *ex) {
            _audioView = nil;
            _controlsInternal = NO;
        }
        _durationRetryCount = 0;
        _httpFallbackTried = NO;
    }
    return self;
}

- (BOOL)isInForeground {
    return _inForeground;
}

- (void)setSrc:(NSString *)src {
    NSLog(@"NSCAudioHelper.setSrc: %@", src);
    _loadedDataFired = NO;
    _canPlayFired = NO;
    _canPlayThroughFired = NO;
    _currentItemInternal = nil;
    _playerInternal = _playerInternal ?: [[AVPlayer alloc] init];

    _src = [src copy];
    if (!_src) return;
    _durationRetryCount = 0;
    _httpFallbackTried = NO;
    NSURL *url = [_src hasPrefix:@"/"] ? [NSURL fileURLWithPath:_src] : [NSURL URLWithString:_src];
    if (!url) return;

    if (_playEndNotificationId) {
        [[NSNotificationCenter defaultCenter] removeObserver:_playEndNotificationId];
        _playEndNotificationId = nil;
    }

    NSDictionary *options = @{ AVURLAssetPreferPreciseDurationAndTimingKey: @YES };
    AVURLAsset *asset = [AVURLAsset URLAssetWithURL:url options:options];
    NSArray *keys = @[@"duration", @"tracks", @"commonMetadata"];
    __weak typeof(self) weakSelf = self;
    [asset loadValuesAsynchronouslyForKeys:keys completionHandler:^{
        dispatch_async(dispatch_get_main_queue(), ^{
            __strong typeof(weakSelf) strongSelf = weakSelf;
            if (!strongSelf) return;

            NSError *keyError = nil;
            AVKeyValueStatus tracksStatus = [asset statusOfValueForKey:@"tracks" error:&keyError];
            NSError *durationErr = nil;
            AVKeyValueStatus durationStatus = [asset statusOfValueForKey:@"duration" error:&durationErr];
            if (durationStatus != AVKeyValueStatusLoaded) {
                NSLog(@"NSCAudioHelper: duration key status=%ld err=%@", (long)durationStatus, durationErr);
            }

            AVPlayerItem *item = [AVPlayerItem playerItemWithAsset:asset];

            if (strongSelf.currentItem) {
                @try { [strongSelf.currentItem removeObserver:strongSelf forKeyPath:@"status"]; } @catch (NSException *e) {}
                @try { [strongSelf.currentItem removeObserver:strongSelf forKeyPath:@"loadedTimeRanges"]; } @catch (NSException *e) {}
                @try { [strongSelf.currentItem removeObserver:strongSelf forKeyPath:@"playbackLikelyToKeepUp"]; } @catch (NSException *e) {}
            }
            if (strongSelf->_playbackTimeObserver) {
                strongSelf->_playbackTimeObserver = nil;
            }

            strongSelf->_currentItemInternal = item;

            [item addObserver:strongSelf forKeyPath:@"status" options:NSKeyValueObservingOptionNew context:nil];
            [item addObserver:strongSelf forKeyPath:@"loadedTimeRanges" options:NSKeyValueObservingOptionNew | NSKeyValueObservingOptionInitial context:nil];
            [item addObserver:strongSelf forKeyPath:@"playbackLikelyToKeepUp" options:NSKeyValueObservingOptionNew context:nil];

            strongSelf->_playEndNotificationId = [[NSNotificationCenter defaultCenter]
                addObserverForName:AVPlayerItemDidPlayToEndTimeNotification
                object:item queue:nil usingBlock:^(NSNotification *note) {
                    __strong typeof(strongSelf) innerSelf = strongSelf;
                    if (!innerSelf) return;
                    if (innerSelf->_loop) {
                        [innerSelf->_playerInternal seekToTime:kCMTimeZero];
                        [innerSelf->_playerInternal play];
                        innerSelf->_stateInternal = NSCPlayerStatePlaying;
                        if ([innerSelf.listener respondsToSelector:@selector(onStateChange:)]) {
                            [innerSelf.listener onStateChange:innerSelf->_stateInternal];
                        }
                    }
                }];

            [strongSelf->_playerInternal replaceCurrentItemWithPlayerItem:item];

            if (tracksStatus == AVKeyValueStatusLoaded) {
                strongSelf->_readyStateInternal = NSCPlayerReadyStateHaveMetadata;
            }

            if (strongSelf->_autoplay) {
                [strongSelf play];
            }
        });
    }];
}

- (NSString *)src { return _src; }

- (void)observeValueForKeyPath:(NSString *)keyPath ofObject:(id)object change:(NSDictionary<NSKeyValueChangeKey,id> *)change context:(void *)context {
    if ([keyPath isEqualToString:@"status"]) {
        AVPlayerItemStatus status = self.player.currentItem.status;
        if (status == AVPlayerItemStatusReadyToPlay) {
            _readyStateInternal = NSCPlayerReadyStateHaveCurrentData;

            CMTime itemDuration = self.player.currentItem.duration;
            BOOL haveDuration = CMTIME_IS_NUMERIC(itemDuration) && !CMTIME_IS_INDEFINITE(itemDuration) && CMTimeCompare(itemDuration, kCMTimeZero) > 0;

            if (haveDuration) {
                double duration = CMTimeGetSeconds(itemDuration);
                if (_audioView) {
                    [_audioView setDuration:duration];
                }
            } else {
                NSLog(@"NSCAudioHelper: ReadyToPlay without numeric duration");
                if (![self updateDurationFromItemOrAsset:self.player.currentItem]) {
                    [self tryUpdateDurationWithDelay:0.5];
                }
            }

            if ([self.listener respondsToSelector:@selector(onLoadedData)]) {
                [self.listener onLoadedData];
            }
        } else if (status == AVPlayerItemStatusFailed) {
            _readyStateInternal = NSCPlayerReadyStateHaveNothing;
            NSError *err = self.player.currentItem.error;
            NSString *msg = err ? [err localizedDescription] : @"Playback error";
            if ([self.listener respondsToSelector:@selector(onError:)]) {
                [self.listener onError:msg];
            }
        }
    } else if ([keyPath isEqualToString:@"loadedTimeRanges"]) {
        if (self.player.currentItem) {
            NSArray *ranges = [self.player.currentItem.loadedTimeRanges copy];
            if (ranges.count > 0) {
                CMTimeRange timeRange = [[ranges objectAtIndex:0] CMTimeRangeValue];
                CMTime bufferEnd = CMTimeRangeGetEnd(timeRange);
                double bufferedEndSeconds = CMTimeGetSeconds(bufferEnd);
                double current = CMTimeGetSeconds(self.player.currentTime);
                CMTime itemDuration = self.player.currentItem.duration;
                double duration = CMTIME_IS_NUMERIC(itemDuration) ? CMTimeGetSeconds(itemDuration) : NAN;

                BOOL likely = NO;
                @try { likely = self.player.currentItem.playbackLikelyToKeepUp; } @catch (NSException *ex) { likely = NO; }

                if (!_canPlayFired && (likely || bufferedEndSeconds > 0)) {
                    _canPlayFired = YES;
                    dispatch_async(dispatch_get_main_queue(), ^{
                        if ([self.listener respondsToSelector:@selector(onCanPlay)]) {
                            [self.listener onCanPlay];
                        }
											if (self->_audioView) {
												[self->_audioView updateCurrentTime:current];
                        }
                    });
                }

                if (!isnan(duration) && duration > 0 && !_canPlayThroughFired) {
                    if (bufferedEndSeconds >= duration - 1.0) {
                        _canPlayThroughFired = YES;
                        dispatch_async(dispatch_get_main_queue(), ^{
                            if ([self.listener respondsToSelector:@selector(onCanPlayThrough)]) {
                                [self.listener onCanPlayThrough];
                            }
													if (self->_audioView) {
														[self->_audioView setDuration:duration];
                            }
                        });
                    }
                }
            }
        }
    } else if ([keyPath isEqualToString:@"playbackLikelyToKeepUp"]) {
        BOOL likely = NO;
        @try { likely = self.player.currentItem.playbackLikelyToKeepUp; } @catch (NSException *ex) { likely = NO; }
        if (!_canPlayFired && likely) {
            _canPlayFired = YES;
            dispatch_async(dispatch_get_main_queue(), ^{
                if ([self.listener respondsToSelector:@selector(onCanPlay)]) {
                    [self.listener onCanPlay];
                }
            });
        }
    }
}

- (double)duration {
    if (!_currentItemInternal) return NAN;
    CMTime t = _currentItemInternal.duration;
    BOOL numeric = CMTIME_IS_NUMERIC(t);
    BOOL indefinite = CMTIME_IS_INDEFINITE(t);
    double seconds = numeric ? CMTimeGetSeconds(t) : NAN;
    if (!numeric || indefinite || CMTimeCompare(t, kCMTimeZero) <= 0) return NAN;
    return seconds;
}


- (void)setDurationSeconds:(double)seconds fromSource:(NSString *)source estimated:(BOOL)estimated {
    if (!(seconds > 0 && seconds < 24.0 * 60.0 * 60.0)) return;
    dispatch_async(dispatch_get_main_queue(), ^{
        if (self->_audioView) {
            [self->_audioView setDuration:seconds];
        }
    });
    if (estimated) {
        NSLog(@"NSCAudioHelper: duration estimated (%@) = %.3f", source, seconds);
    }
}


- (BOOL)updateDurationFromItemOrAsset:(AVPlayerItem *)item {
    if (!item) return NO;

    CMTime itemDur = item.duration;
    if (CMTIME_IS_NUMERIC(itemDur) && !CMTIME_IS_INDEFINITE(itemDur) && CMTimeCompare(itemDur, kCMTimeZero) > 0) {
        double s = CMTimeGetSeconds(itemDur);
        [self setDurationSeconds:s fromSource:@"item" estimated:NO];
        return YES;
    }

    @try {
        NSArray *seekable = item.seekableTimeRanges;
        if (seekable && seekable.count > 0) {
            CMTimeRange range = [[seekable lastObject] CMTimeRangeValue];
            CMTime end = CMTimeRangeGetEnd(range);
            if (CMTIME_IS_NUMERIC(end) && !CMTIME_IS_INDEFINITE(end) && CMTimeCompare(end, kCMTimeZero) > 0) {
                double s = CMTimeGetSeconds(end);
                [self setDurationSeconds:s fromSource:@"seekable" estimated:NO];
                return YES;
            }
        }
    } @catch (NSException *ex) {}

    AVAsset *asset = item.asset;
    if (asset) {
        CMTime assetDur = asset.duration;
        if (CMTIME_IS_NUMERIC(assetDur) && !CMTIME_IS_INDEFINITE(assetDur) && CMTimeCompare(assetDur, kCMTimeZero) > 0) {
            double s = CMTimeGetSeconds(assetDur);
            [self setDurationSeconds:s fromSource:@"asset" estimated:NO];
            return YES;
        }

        @try {
            NSArray<AVAssetTrack *> *tracks = [asset tracksWithMediaType:AVMediaTypeAudio];
            for (AVAssetTrack *track in tracks) {
                CMTimeRange tr = track.timeRange;
                CMTime td = tr.duration;
                if (CMTIME_IS_NUMERIC(td) && !CMTIME_IS_INDEFINITE(td) && CMTimeCompare(td, kCMTimeZero) > 0) {
                    double s = CMTimeGetSeconds(td);
                    [self setDurationSeconds:s fromSource:@"audioTrack" estimated:NO];
                    return YES;
                }
            }
            tracks = [asset tracksWithMediaType:AVMediaTypeVideo];
            for (AVAssetTrack *track in tracks) {
                CMTimeRange tr = track.timeRange;
                CMTime td = tr.duration;
                if (CMTIME_IS_NUMERIC(td) && !CMTIME_IS_INDEFINITE(td) && CMTimeCompare(td, kCMTimeZero) > 0) {
                    double s = CMTimeGetSeconds(td);
                    [self setDurationSeconds:s fromSource:@"videoTrack" estimated:NO];
                    return YES;
                }
            }
        } @catch (NSException *ex) {}
    }

    [self attemptHTTPContentLengthFallbackForItem:item];
    return NO;
}


- (void)attemptHTTPContentLengthFallbackForItem:(AVPlayerItem *)item {
    if (!item) return;
    if (_httpFallbackTried) return;
    _httpFallbackTried = YES;
    AVAsset *asset = item.asset;
    if (![asset isKindOfClass:[AVURLAsset class]]) return;
    NSURL *url = ((AVURLAsset *)asset).URL;
    if (!url) return;
    NSString *scheme = [[url scheme] lowercaseString];
    if (!([scheme isEqualToString:@"http"] || [scheme isEqualToString:@"https"])) return;

    NSMutableURLRequest *req = [NSMutableURLRequest requestWithURL:url];
    req.HTTPMethod = @"HEAD";
    req.timeoutInterval = 5.0;

    __weak typeof(self) weakSelf = self;
    NSURLSessionDataTask *task = [[NSURLSession sharedSession] dataTaskWithRequest:req completionHandler:^(NSData *data, NSURLResponse *response, NSError *error) {
        __strong typeof(weakSelf) strongSelf = weakSelf;
        if (!strongSelf) return;
        if (error) {
            NSLog(@"NSCAudioHelper: HTTP HEAD fallback error: %@", error);
            return;
        }
        long long contentLength = -1;
        if ([response isKindOfClass:[NSHTTPURLResponse class]]) {
            NSHTTPURLResponse *http = (NSHTTPURLResponse *)response;
            NSDictionary *headers = http.allHeaderFields;
            for (NSString *k in headers) {
                if ([k caseInsensitiveCompare:@"Content-Length"] == NSOrderedSame) {
                    contentLength = [headers[k] longLongValue];
                    break;
                }
            }
        } else {
            contentLength = response.expectedContentLength;
        }
        if (contentLength <= 0) {
            NSLog(@"NSCAudioHelper: HTTP HEAD provided no Content-Length");
            return;
        }

        double bitrate = 0.0;
        @try {
            NSArray *tracks = [asset tracksWithMediaType:AVMediaTypeAudio];
            if (tracks.count > 0) {
                AVAssetTrack *track = [tracks objectAtIndex:0];
                bitrate = track.estimatedDataRate;
            }
        } @catch (NSException *ex) {}

        if (bitrate <= 0) {
            if ([response isKindOfClass:[NSHTTPURLResponse class]]) {
                NSHTTPURLResponse *http = (NSHTTPURLResponse *)response;
                NSDictionary *headers = http.allHeaderFields;
                for (NSString *k in headers) {
                    if ([k caseInsensitiveCompare:@"icy-br"] == NSOrderedSame || [k caseInsensitiveCompare:@"x-bitrate"] == NSOrderedSame || [k caseInsensitiveCompare:@"x-audio-bitrate"] == NSOrderedSame) {
                        NSString *val = [headers objectForKey:k];
                        double v = [val doubleValue];
                        if (v > 0) {
                            bitrate = v * 1000.0;
                            break;
                        }
                    }
                }
            }
        }

        if (bitrate <= 0) bitrate = 128000.0;

        double seconds = ((double)contentLength * 8.0) / bitrate;
        if (seconds > 0 && seconds < 24.0 * 60.0 * 60.0) {
            [strongSelf setDurationSeconds:seconds fromSource:@"http" estimated:YES];
        } else {
            NSLog(@"NSCAudioHelper: HTTP content-length fallback produced invalid duration=%f", seconds);
        }
    }];
    [task resume];
}

- (void)tryUpdateDurationWithDelay:(NSTimeInterval)delay {
    __weak typeof(self) weakSelf = self;
    dispatch_after(dispatch_time(DISPATCH_TIME_NOW, (int64_t)(delay * NSEC_PER_SEC)), dispatch_get_main_queue(), ^{
        __strong typeof(weakSelf) strongSelf = weakSelf;
        if (!strongSelf) return;

        if ([strongSelf updateDurationFromItemOrAsset:strongSelf.player.currentItem]) return;
        strongSelf->_durationRetryCount++;
        if (strongSelf->_durationRetryCount > 6) {
            NSLog(@"NSCAudioHelper: duration retry limit reached");
            return;
        }

        AVPlayerItem *item = strongSelf.player.currentItem;
        if (!item) return;

        CMTime itemDur = item.duration;
        if (CMTIME_IS_NUMERIC(itemDur) && !CMTIME_IS_INDEFINITE(itemDur) && CMTimeCompare(itemDur, kCMTimeZero) > 0) {
            double seconds = CMTimeGetSeconds(itemDur);
            [strongSelf setDurationSeconds:seconds fromSource:@"item" estimated:NO];
            return;
        }

        CMTime assetDur = item.asset.duration;
        if (CMTIME_IS_NUMERIC(assetDur) && !CMTIME_IS_INDEFINITE(assetDur) && CMTimeCompare(assetDur, kCMTimeZero) > 0) {
            double seconds = CMTimeGetSeconds(assetDur);
            [strongSelf setDurationSeconds:seconds fromSource:@"asset" estimated:NO];
            return;
        }

        [item.asset loadValuesAsynchronouslyForKeys:@[@"duration"] completionHandler:^{
            dispatch_async(dispatch_get_main_queue(), ^{
                NSError *err = nil;
                AVKeyValueStatus ds = [item.asset statusOfValueForKey:@"duration" error:&err];
                    if (ds == AVKeyValueStatusLoaded) {
                        CMTime ad = item.asset.duration;
                        if (CMTIME_IS_NUMERIC(ad) && !CMTIME_IS_INDEFINITE(ad) && CMTimeCompare(ad, kCMTimeZero) > 0) {
                            double s = CMTimeGetSeconds(ad);
                            [strongSelf setDurationSeconds:s fromSource:@"asset" estimated:NO];
                            return;
                        }
                    } else {
                        NSLog(@"NSCAudioHelper: retry load duration status=%ld err=%@", (long)ds, err);
                    }
                NSTimeInterval next = MAX(0.5, delay * 2.0);
                [strongSelf tryUpdateDurationWithDelay:next];
            });
        }];
    });
}

- (double)currentTime {
    return CMTimeGetSeconds(self.player.currentTime);
}

- (void)setCurrentTime:(double)currentTime {
    CMTime time = CMTimeMakeWithSeconds(currentTime, self.player.currentTime.timescale);
    [self.player seekToTime:time];
}
- (BOOL)controls { return _controlsInternal; }
- (void)setControls:(BOOL)controls {
    _controlsInternal = controls;
    if (_audioView) {
        dispatch_async(dispatch_get_main_queue(), ^{
					self->_audioView.hidden = !controls;
        });
    }
}

- (BOOL)muted { return self.player.muted; }
- (void)setMuted:(BOOL)muted { self.player.muted = muted; }

- (void)play {
    NSLog(@"NSCAudioHelper.play called");
    if (_stateInternal == NSCPlayerStatePlaying) return;
    [self addTimeObserver];
    [self.player play];
    _stateInternal = NSCPlayerStatePlaying;
    if ([self.listener respondsToSelector:@selector(onStateChange:)]) {
        [self.listener onStateChange:_stateInternal];
    }
    if (_audioView) {
        [_audioView setPlaying:YES];
    }
}

- (void)addTimeObserver {
    if (_playbackTimeObserver) {
        @try { [self.player removeTimeObserver:_playbackTimeObserver]; } @catch (NSException *ex) {}
        _playbackTimeObserver = nil;
    }
    CMTime interval = CMTimeMake(1, 1);
    __weak typeof(self) weakSelf = self;
    _playbackTimeObserver = [self.player addPeriodicTimeObserverForInterval:interval queue:nil usingBlock:^(CMTime time) {
        __strong typeof(weakSelf) strongSelf = weakSelf;
        if (!strongSelf) return;
        if (strongSelf->_stateInternal == NSCPlayerStatePlaying) {
            if ([strongSelf.listener respondsToSelector:@selector(onTimeUpdate:)]) {
                [strongSelf.listener onTimeUpdate:CMTimeGetSeconds(time)];
            }
            if (strongSelf->_audioView) {
                [strongSelf->_audioView updateCurrentTime:CMTimeGetSeconds(time)];
            }
        }
    }];
}

- (void)pause {
    if (_stateInternal != NSCPlayerStatePlaying) return;
    if (_playbackTimeObserver) {
        [self.player removeTimeObserver:_playbackTimeObserver];
        _playbackTimeObserver = nil;
    }
    [self.player pause];
    _stateInternal = NSCPlayerStatePaused;
    if ([self.listener respondsToSelector:@selector(onStateChange:)]) {
        [self.listener onStateChange:_stateInternal];
    }
    if (_audioView) {
        [_audioView setPlaying:NO];
    }
}

- (void)dealloc {
    if (_stateInternal == NSCPlayerStatePlaying) {
        [self.player pause];
        _stateInternal = NSCPlayerStateStopped;
        if ([self.listener respondsToSelector:@selector(onStateChange:)]) {
            [self.listener onStateChange:_stateInternal];
        }
    }
    if (_playbackTimeObserver) {
        @try { [self.player removeTimeObserver:_playbackTimeObserver]; } @catch (NSException *ex) {}
        _playbackTimeObserver = nil;
    }
    if (self.currentItem) {
        @try { [self.currentItem removeObserver:self forKeyPath:@"status"]; } @catch (NSException *exception) {}
        @try { [self.currentItem removeObserver:self forKeyPath:@"loadedTimeRanges"]; } @catch (NSException *exception) {}
        @try { [self.currentItem removeObserver:self forKeyPath:@"playbackLikelyToKeepUp"]; } @catch (NSException *exception) {}
    }
    if (_playEndNotificationId) {
        [[NSNotificationCenter defaultCenter] removeObserver:_playEndNotificationId];
    }
    if (_resumeListenerId) {
        [[NSNotificationCenter defaultCenter] removeObserver:_resumeListenerId];
    }
    if (_suspendListenerId) {
        [[NSNotificationCenter defaultCenter] removeObserver:_suspendListenerId];
    }
}

- (void)load {
    dispatch_async(dispatch_get_main_queue(), ^{
        @try {
            [self pause];
					if (self->_playbackTimeObserver) {
						[self.player removeTimeObserver:self->_playbackTimeObserver];
						self->_playbackTimeObserver = nil;
            }
					if (self->_playEndNotificationId) {
						[[NSNotificationCenter defaultCenter] removeObserver:self->_playEndNotificationId];
						self->_playEndNotificationId = nil;
            }
            if (self.currentItem) {
                @try { [self.currentItem removeObserver:self forKeyPath:@"status"]; } @catch (NSException *exception) {}
                @try { [self.currentItem removeObserver:self forKeyPath:@"loadedTimeRanges"]; } @catch (NSException *exception) {}
                @try { [self.currentItem removeObserver:self forKeyPath:@"playbackLikelyToKeepUp"]; } @catch (NSException *exception) {}
            }
                [self.player replaceCurrentItemWithPlayerItem:nil];
                    self->_loadedDataFired = NO;
                    self->_durationRetryCount = 0;
            if (self.src && self.src.length > 0) {
                NSString *src = [self.src copy];
                self.src = nil;
                [self setSrc:src];
            }
        } @catch (NSException *ex) {
            NSLog(@"NSCAudioHelper load() error: %@", ex);
        }
    });
}

- (AVPlayer *)player { return _playerInternal; }
- (AVPlayerItem *)currentItem { return _currentItemInternal; }
- (NSCPlayerState)state { return _stateInternal; }
- (NSCPlayerReadyState)readyState { return _readyStateInternal; }

- (UIView *)view { return _audioView; }

- (NSString *)canPlayType:(NSString *)type {
    if (!type) return @"";
    NSString *mime = [type lowercaseString];
    NSRange sc = [mime rangeOfString:@";"];
    if (sc.location != NSNotFound) {
        mime = [mime substringToIndex:sc.location];
    }
    mime = [mime stringByTrimmingCharactersInSet:[NSCharacterSet whitespaceAndNewlineCharacterSet]];

#if __has_include(<UniformTypeIdentifiers/UniformTypeIdentifiers.h>)
    if (@available(iOS 14.0, *)) {
        UTType *utType = [UTType typeWithMIMEType:mime];
        if (utType && ([utType conformsToType:UTTypeAudio] || [utType conformsToType:UTTypeAudiovisualContent] || [utType conformsToType:UTTypeMovie])) {
            if ([mime containsString:@"ogg"] || [mime containsString:@"webm"] || [mime containsString:@"opus"]) return @"maybe";
            return @"probably";
        }
    }
#endif

    if ([mime containsString:@"mp3"] || [mime containsString:@"mpeg"] || [mime containsString:@"wav"] || [mime containsString:@"aac"] || [mime containsString:@"m4a"] || [mime containsString:@"flac"]) return @"probably";
    if ([mime containsString:@"ogg"] || [mime containsString:@"opus"] || [mime containsString:@"webm"]) return @"maybe";
    return @"";
}


- (void)setPlayIconName:(NSString *)name {
    if (!name || name.length == 0) return;
    UIImage *img = nil;
    if (@available(iOS 13.0, *)) {
        img = [UIImage systemImageNamed:name];
    }
    if (!img) {
        img = [UIImage imageNamed:name];
    }
    if (!img) return;
    img = [img imageWithRenderingMode:UIImageRenderingModeAlwaysTemplate];
    dispatch_async(dispatch_get_main_queue(), ^{
        if (self->_audioView) {
            [self->_audioView setPlayImage:img];
        }
    });
}

- (void)setPauseIconName:(NSString *)name {
    if (!name || name.length == 0) return;
    UIImage *img = nil;
    if (@available(iOS 13.0, *)) {
        img = [UIImage systemImageNamed:name];
    }
    if (!img) {
        img = [UIImage imageNamed:name];
    }
    if (!img) return;
    img = [img imageWithRenderingMode:UIImageRenderingModeAlwaysTemplate];
    dispatch_async(dispatch_get_main_queue(), ^{
        if (self->_audioView) {
            [self->_audioView setPauseImage:img];
        }
    });
}

- (UIColor *)_colorFromHexString:(NSString *)hexString {
    if (!hexString) return nil;
    NSString *cString = [[hexString stringByTrimmingCharactersInSet:[NSCharacterSet whitespaceAndNewlineCharacterSet]] uppercaseString];
    if ([cString hasPrefix:@"#"]) cString = [cString substringFromIndex:1];
    NSUInteger length = [cString length];
    unsigned int rgba = 0;
    NSScanner *scanner = [NSScanner scannerWithString:cString];
    if (![scanner scanHexInt:&rgba]) return nil;
    if (length == 6) {
        CGFloat r = ((rgba & 0xFF0000) >> 16) / 255.0f;
        CGFloat g = ((rgba & 0x00FF00) >> 8) / 255.0f;
        CGFloat b = (rgba & 0x0000FF) / 255.0f;
        return [UIColor colorWithRed:r green:g blue:b alpha:1.0f];
    } else if (length == 8) {
        CGFloat r = ((rgba & 0xFF000000) >> 24) / 255.0f;
        CGFloat g = ((rgba & 0x00FF0000) >> 16) / 255.0f;
        CGFloat b = ((rgba & 0x0000FF00) >> 8) / 255.0f;
        CGFloat a = (rgba & 0x000000FF) / 255.0f;
        return [UIColor colorWithRed:r green:g blue:b alpha:a];
    }
    return nil;
}

- (void)setIconTintHex:(NSString *)hex {
    UIColor *c = [self _colorFromHexString:hex];
    if (!c) return;
    dispatch_async(dispatch_get_main_queue(), ^{
        if (self->_audioView) {
            [self->_audioView setIconTintColor:c];
        }
    });
}

- (void)setIconTintColor:(UIColor *)color {
    if (!color) return;
    dispatch_async(dispatch_get_main_queue(), ^{
        if (self->_audioView) {
            [self->_audioView setIconTintColor:color];
        }
    });
}

@end
