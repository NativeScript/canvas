//
//  NSCAudioHelper.h
//  CanvasMedia
//
//
#import <Foundation/Foundation.h>
#import <UIKit/UIKit.h>
#import <AVFoundation/AVFoundation.h>
#import "NSCPlayerState.h"

@protocol NSCAudioHelperListener <NSObject>
- (void)onTimeUpdate:(double)seconds;
- (void)onStateChange:(NSCPlayerState)state;
- (void)onLoadedData;
- (void)onError:(NSString *)message;
- (void)onCanPlay;
- (void)onCanPlayThrough;
@end

@interface NSCAudioHelper : NSObject

@property (nonatomic, strong, readonly) AVPlayer *player;
@property (nonatomic, strong, readonly) AVPlayerItem *currentItem;
@property (nonatomic, readonly) NSCPlayerState state;
@property (nonatomic, readonly) NSCPlayerReadyState readyState;
@property (nonatomic, weak) id<NSCAudioHelperListener> listener;
@property (nonatomic) BOOL loop;
@property (nonatomic) BOOL autoplay;
@property (nonatomic) BOOL muted;
@property (nonatomic) BOOL controls;
@property (nonatomic, strong, readonly) UIView *view;
@property (nonatomic, copy) NSString *src;
@property (nonatomic, readonly) double duration;
@property (nonatomic) double currentTime;

- (void)play;
- (void)pause;
- (void)load;

- (NSString *)canPlayType:(NSString *)type;

- (void)setPlayIconName:(NSString *)name;
- (void)setPauseIconName:(NSString *)name;
- (void)setIconTintHex:(NSString *)hex;
- (void)setIconTintColor:(UIColor *)color;

@end
