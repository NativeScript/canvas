//
//  NSCVideoHelper.h
//  CanvasMedia
//
//  Created by Osei Fortune on 20/05/2025.
//
#import <Foundation/Foundation.h>
#import <AVKit/AVKit.h>
#import <AVFoundation/AVFoundation.h>
#import <UIKit/UIKit.h>
#import <CoreMedia/CoreMedia.h>

#import "NSCPlayerState.h"


@protocol NSCVideoHelperListener <NSObject>
- (void)onVideoFrameCallback;
- (void)onTimeUpdate:(double)seconds;
- (void)onStateChange:(NSCPlayerState)state;
@end

@interface NSCVideoHelper : NSObject

@property (nonatomic, strong, readonly) AVPlayerViewController *controller;
@property (nonatomic, strong, readonly) AVPlayer *player;
@property (nonatomic, strong, readonly) AVPlayerItem *currentItem;
@property (nonatomic, readonly) BOOL isInForeground;
@property (nonatomic, readonly) NSCPlayerState state;
@property (nonatomic, readonly) NSCPlayerReadyState readyState;
@property (nonatomic, readonly) CGSize videoSize;
@property (nonatomic, strong, readonly) AVPlayerItemVideoOutput *assetOutput;
@property (nonatomic, weak) id<NSCVideoHelperListener> listener;
@property (nonatomic) BOOL loop;
@property (nonatomic) BOOL autoplay;
@property (nonatomic) BOOL muted;
@property (nonatomic) BOOL controls;
@property (nonatomic) BOOL playsinline;
@property (nonatomic, copy) NSString *src;
@property (nonatomic, readonly) double duration;
@property (nonatomic) double currentTime;

- (void)play;
- (void)pause;

@end
