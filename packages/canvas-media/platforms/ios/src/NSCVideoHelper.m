//
//  NSCVideoHelper.m
//  CanvasMedia
//
//  Created by Osei Fortune on 20/05/2025.
//
#import "NSCVideoHelper.h"
#import "NSCVideoHelper+Internal.h"

@implementation NSCVideoHelper

- (instancetype)init {
	_isLoop = NO;
    if (self = [super init]) {
        _controller = [[AVPlayerViewController alloc] init];
        _player = [[AVPlayer alloc] init];
				_inForeground = YES;
        _readyState = NSCPlayerReadyStateHaveNothing;
        _state = NSCPlayerStateIdle;
        _controller.player = _player;
			
        __weak typeof(self) weakSelf = self;
        _suspendListenerId = [[NSNotificationCenter defaultCenter]
                              addObserverForName:UIApplicationDidEnterBackgroundNotification
                              object:nil queue:nil
                              usingBlock:^(NSNotification *note) {
            weakSelf.inForeground = NO;
        }];

        _resumeListenerId = [[NSNotificationCenter defaultCenter]
                             addObserverForName:UIApplicationDidBecomeActiveNotification
                             object:nil queue:nil
                             usingBlock:^(NSNotification *note) {
            weakSelf.inForeground = YES;
        }];
    }
    return self;
}

-(BOOL) loop {
	return _isLoop;
}


- (void)setLoop:(BOOL)loop {
	_isLoop = loop;
}



-(BOOL) isInForeground {
	return _inForeground;
}

- (void)setSrc:(NSString *)src {
    _currentSrc = src;
    _readyState = NSCPlayerReadyStateHaveNothing;
    self.loadedDataFired = NO;
    
    if (!src) return;

    NSURL *url = [src hasPrefix:@"/"] ? [NSURL fileURLWithPath:src] : [NSURL URLWithString:src];
    if (!url) return;

    if (_playEndNotificationId) {
        [[NSNotificationCenter defaultCenter] removeObserver:_playEndNotificationId];
        _playEndNotificationId = nil;
    }

    self.asset = [AVURLAsset URLAssetWithURL:url options:nil];
    NSArray *keys = @[@"duration", @"tracks", @"commonMetadata"];

    __weak typeof(self) weakSelf = self;
    [self.asset loadValuesAsynchronouslyForKeys:keys completionHandler:^{
        dispatch_async(dispatch_get_main_queue(), ^{
					__strong typeof(self) strongSelf = weakSelf;
					if (!strongSelf) return;
					
            AVAssetTrack *videoTrack = [[weakSelf.asset tracksWithMediaType:AVMediaTypeVideo] firstObject];
					strongSelf->_videoSize = videoTrack.naturalSize;

            float fps = videoTrack.nominalFrameRate ?: 30.0;
            CMTime interval = CMTimeMake(1, (int32_t)fps);
            weakSelf.playbackFramesObserver = [weakSelf.player addPeriodicTimeObserverForInterval:interval queue:nil usingBlock:^(CMTime time) {
                if (weakSelf.state == NSCPlayerStatePlaying) {
                    [weakSelf.listener onVideoFrameCallback];
                }
            }];

            AVPlayerItem *item = [AVPlayerItem playerItemWithAsset:weakSelf.asset];
					strongSelf->_currentItem = item;

            NSDictionary *outputSettings = @{
                (NSString *)kCVPixelBufferPixelFormatTypeKey: @(kCVPixelFormatType_32BGRA),
#if !TARGET_OS_SIMULATOR
                (NSString *)kCVPixelBufferIOSurfacePropertiesKey:  @{},
                (NSString *)kCVPixelBufferOpenGLESCompatibilityKey: @YES,
                (NSString *)kCVPixelBufferOpenGLCompatibilityKey: @YES,
#endif
                (NSString *)kCVPixelBufferMetalCompatibilityKey: @YES,
            };

					strongSelf->_assetOutput = [[AVPlayerItemVideoOutput alloc] initWithPixelBufferAttributes:outputSettings];

            [item addObserver:strongSelf forKeyPath:@"status" options:NSKeyValueObservingOptionNew context:nil];
            [item addObserver:strongSelf forKeyPath:@"loadedTimeRanges" options:NSKeyValueObservingOptionNew | NSKeyValueObservingOptionInitial context:nil];

					strongSelf->_playEndNotificationId = [[NSNotificationCenter defaultCenter]
                                                 addObserverForName:AVPlayerItemDidPlayToEndTimeNotification
                                                 object:item queue:nil usingBlock:^(NSNotification *note) {
                if (weakSelf != NULL && weakSelf.isLoop) {
                    [weakSelf.player seekToTime:kCMTimeZero];
                    [weakSelf.player play];
                    weakSelf.state = NSCPlayerStatePlaying;
                    [weakSelf.listener onStateChange:NSCPlayerStatePlaying];
                }
            }];

            [weakSelf.player replaceCurrentItemWithPlayerItem:item];
            weakSelf.readyState = NSCPlayerReadyStateHaveMetadata;
            [weakSelf performSelectorOnMainThread:@selector(_scheduleFirstFrameCheck) withObject:nil waitUntilDone:NO];
            if (weakSelf.autoplay) {
                [weakSelf play];
            }
        });
    }];
}

- (NSString *)src {
    return _currentSrc;
}

- (void)observeValueForKeyPath:(NSString *)keyPath
                      ofObject:(id)object
                        change:(NSDictionary<NSKeyValueChangeKey,id> *)change
                       context:(void *)context {

    if ([keyPath isEqualToString:@"status"]) {
        if (self.player.currentItem.status == AVPlayerItemStatusReadyToPlay) {
            self->_videoSize = [[self.asset tracksWithMediaType:AVMediaTypeVideo] firstObject].naturalSize;
            [self.player.currentItem addOutput:self.assetOutput];
            self.readyState = NSCPlayerReadyStateHaveCurrentData;
        }
    }
}

- (void)_scheduleFirstFrameCheck {
   [self _checkForFirstFrameWithAttempts:10 delayMs:50];
}

- (void)_checkForFirstFrameWithAttempts:(int)attemptsRemaining delayMs:(int)delayMs {
    if (self.loadedDataFired || attemptsRemaining <= 0) return;

    CMTime current = [self.player currentTime];
    CVPixelBufferRef buffer = NULL;
    if (self.assetOutput) {
        buffer = [self.assetOutput copyPixelBufferForItemTime:current itemTimeForDisplay:NULL];
    }

    if (buffer != NULL) {
        CFRelease(buffer);
        self.loadedDataFired = YES;
        dispatch_async(dispatch_get_main_queue(), ^{
            if ([self.listener respondsToSelector:@selector(onLoadedData)]) {
                [self.listener onLoadedData];
            }
        });
        return;
    }

    dispatch_after(dispatch_time(DISPATCH_TIME_NOW, (int64_t)(delayMs * NSEC_PER_MSEC)), dispatch_get_main_queue(), ^{
        [self _checkForFirstFrameWithAttempts:attemptsRemaining - 1 delayMs:delayMs];
    });
}

- (double)duration {
    if (!_currentItem) return NAN;
    return CMTimeGetSeconds(_currentItem.asset.duration);
}

- (double)currentTime {
    return CMTimeGetSeconds(self.player.currentTime);
}

- (void)setCurrentTime:(double)currentTime {
    CMTime time = CMTimeMakeWithSeconds(currentTime, self.player.currentTime.timescale);
    [self.player seekToTime:time];
}

- (BOOL)controls {
    return self.controller.showsPlaybackControls;
}

- (void)setControls:(BOOL)controls {
    self.controller.showsPlaybackControls = controls;
}

- (BOOL)muted {
    return self.player.muted;
}

- (void)setMuted:(BOOL)muted {
    self.player.muted = muted;
}

- (BOOL)playsinline {
    return !self.controller.entersFullScreenWhenPlaybackBegins;
}

- (void)setPlaysinline:(BOOL)playsinline {
    self.controller.entersFullScreenWhenPlaybackBegins = !playsinline;
}

- (void)play {
    if (self.state == NSCPlayerStatePlaying) return;
    [self addTimeObserver];
    [self.player play];
    self.state = NSCPlayerStatePlaying;
    [self.listener onStateChange:self.state];
}

- (void)addTimeObserver {
    CMTime interval = CMTimeMake(1, 1);
    __weak typeof(self) weakSelf = self;
    self.playbackTimeObserver = [self.player addPeriodicTimeObserverForInterval:interval queue:nil usingBlock:^(CMTime time) {
        if (weakSelf.state == NSCPlayerStatePlaying) {
            [weakSelf.listener onTimeUpdate:CMTimeGetSeconds(time)];
        }
    }];
}

- (void)pause {
    if (self.state != NSCPlayerStatePlaying) return;
    if (self.playbackTimeObserver) {
        [self.player removeTimeObserver:self.playbackTimeObserver];
        self.playbackTimeObserver = nil;
    }
    [self.player pause];
    self.state = NSCPlayerStatePaused;
    [self.listener onStateChange:self.state];
}

- (void)dealloc {
    if (self.state == NSCPlayerStatePlaying) {
        [self.player pause];
        self.state = NSCPlayerStateStopped;
        [self.listener onStateChange:self.state];
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

@end
