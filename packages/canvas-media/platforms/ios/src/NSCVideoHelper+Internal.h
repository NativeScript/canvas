//
//  NSCVideoHelper+Internal.h
//  CanvasMedia
//
//  Created by Osei Fortune on 20/05/2025.
//
#import <Foundation/Foundation.h>
#import <AVKit/AVKit.h>
#import <AVFoundation/AVFoundation.h>
#import <UIKit/UIKit.h>
#import <CoreMedia/CoreMedia.h>

@interface NSCVideoHelper()
@property (nonatomic, strong) AVURLAsset *asset;
@property (nonatomic, copy) NSString *currentSrc;
@property (nonatomic, strong) id playEndNotificationId;
@property (nonatomic, strong) id resumeListenerId;
@property (nonatomic, strong) id suspendListenerId;
@property (nonatomic, strong) id playbackFramesObserver;
@property (nonatomic, strong) id playbackTimeObserver;
@property (nonatomic, assign) NSCPlayerState state;
@property (nonatomic, assign) NSCPlayerReadyState readyState;
@property (nonatomic, assign) BOOL inForeground;
@property (nonatomic, assign) BOOL isLoop;
@end
