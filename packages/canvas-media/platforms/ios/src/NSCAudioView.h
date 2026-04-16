//
//  NSCAudioView.h
//  CanvasMedia
//
#import <UIKit/UIKit.h>

@class NSCAudioHelper;

@interface NSCAudioView : UIView

- (instancetype)initWithHelper:(NSCAudioHelper *)helper;
- (void)updateCurrentTime:(double)seconds;
- (void)setPlaying:(BOOL)playing;
- (void)setDuration:(double)seconds;

- (void)setPlayImage:(UIImage *)image;
- (void)setPauseImage:(UIImage *)image;
- (void)setIconTintColor:(UIColor *)color;

@end
