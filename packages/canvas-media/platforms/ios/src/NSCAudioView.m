//
//  NSCAudioView.m
//  CanvasMedia
//
#import "NSCAudioView.h"
#import "NSCAudioHelper.h"

@implementation NSCAudioView {
    __weak NSCAudioHelper *_helper;
    UIButton *_playButton;
    UISlider *_slider;
    UILabel *_timeLabel;
    BOOL _isSeeking;
    UIImage *_customPlayImage;
    UIImage *_customPauseImage;
    UIColor *_iconTintColor;
    double _durationSeconds;
}

- (instancetype)initWithHelper:(NSCAudioHelper *)helper {
    if (self = [super initWithFrame:CGRectZero]) {
        _helper = helper;
        self.backgroundColor = [UIColor clearColor];
        self.translatesAutoresizingMaskIntoConstraints = NO;

        _playButton = [UIButton buttonWithType:UIButtonTypeCustom];
        _playButton.translatesAutoresizingMaskIntoConstraints = YES;
        UIImage *playImage = nil;
        if (@available(iOS 13.0, *)) {
            playImage = [UIImage systemImageNamed:@"play.fill"];
            _playButton.tintColor = [UIColor systemBlueColor];
        }
        if (!playImage) {
            playImage = [self _playIconWithSize:CGSizeMake(40,40) color:[UIColor blackColor]];
            _playButton.tintColor = [UIColor blackColor];
        }
        if (playImage) {
            [_playButton setImage:[playImage imageWithRenderingMode:UIImageRenderingModeAlwaysTemplate] forState:UIControlStateNormal];
        }
        [_playButton addTarget:self action:@selector(playTapped:) forControlEvents:UIControlEventTouchUpInside];
        [self addSubview:_playButton];

        _slider = [[UISlider alloc] initWithFrame:CGRectZero];
        _slider.translatesAutoresizingMaskIntoConstraints = YES;
        _slider.enabled = NO;
        [_slider addTarget:self action:@selector(sliderChanged:) forControlEvents:UIControlEventValueChanged];
        [_slider addTarget:self action:@selector(sliderTouchDown:) forControlEvents:UIControlEventTouchDown];
        [_slider addTarget:self action:@selector(sliderTouchUp:) forControlEvents:UIControlEventTouchUpInside | UIControlEventTouchUpOutside];
        [self addSubview:_slider];

        _timeLabel = [[UILabel alloc] initWithFrame:CGRectZero];
        _timeLabel.translatesAutoresizingMaskIntoConstraints = YES;
        _durationSeconds = 0.0;
        _timeLabel.text = @"0:00 / 0:00";
        _slider.minimumValue = 0;
        _slider.maximumValue = 1.0f;
        [self addSubview:_timeLabel];
    }
    return self;
}

- (void)layoutSubviews {
    [super layoutSubviews];
    CGFloat pad = 8.0;
    CGFloat w = CGRectGetWidth(self.bounds);
    CGFloat h = CGRectGetHeight(self.bounds);
    if (w <= 0 || h <= 0) return;

    CGFloat btnW = 40.0;
    CGFloat btnH = 40.0;
    CGFloat btnY = floor((h - btnH) / 2.0);
    _playButton.frame = CGRectMake(pad, btnY, btnW, btnH);

    CGFloat maxLabelWidth = 120.0;
    CGSize labelSize = [_timeLabel sizeThatFits:CGSizeMake(maxLabelWidth, h)];
    CGFloat labelW = MIN(labelSize.width, maxLabelWidth);
    CGFloat labelH = MIN(labelSize.height, h);
    CGFloat labelX = w - pad - labelW;
    CGFloat labelY = floor((h - labelH) / 2.0);
    _timeLabel.frame = CGRectMake(labelX, labelY, labelW, labelH);

    CGFloat sliderX = CGRectGetMaxX(_playButton.frame) + pad;
    CGFloat sliderW = labelX - pad - sliderX;
    if (sliderW < 40.0) sliderW = 40.0;
    CGFloat sliderH = 30.0;
    CGFloat sliderY = floor((h - sliderH) / 2.0);
    _slider.frame = CGRectMake(sliderX, sliderY, sliderW, sliderH);
}

- (void)playTapped:(id)sender {
    if (!_helper) return;
    if ([_helper state] == 1) {
        [_helper pause];
    } else {
        [_helper play];
    }
}

- (void)sliderChanged:(UISlider *)s {
    if (!_isSeeking) return;
    double seconds = s.value;
    [self updateTimeLabelWithCurrent:seconds duration:_durationSeconds];
    if (_helper) {
        @try {
            [_helper setCurrentTime:seconds];
        } @catch (NSException *ex) {}
    }
}

- (void)sliderTouchDown:(UISlider *)s {
    _isSeeking = YES;
}

- (void)sliderTouchUp:(UISlider *)s {
    _isSeeking = NO;
    if (!_helper) return;
    double seconds = s.value;
    [_helper setCurrentTime:seconds];
}

- (void)updateCurrentTime:(double)seconds {
    if (_isSeeking) return;
    dispatch_async(dispatch_get_main_queue(), ^{
			self->_slider.value = seconds;
			[self updateTimeLabelWithCurrent:seconds duration:self->_durationSeconds];
    });
}

- (void)setPlaying:(BOOL)playing {
    __weak typeof(self) weakSelf = self;
    dispatch_async(dispatch_get_main_queue(), ^{
        __strong typeof(weakSelf) strongSelf = weakSelf;
        if (!strongSelf) return;
        UIImage *img = nil;
        if (playing ? strongSelf->_customPauseImage : strongSelf->_customPlayImage) {
            img = playing ? strongSelf->_customPauseImage : strongSelf->_customPlayImage;
            if (img) {
                UIImage *templ = [img imageWithRenderingMode:UIImageRenderingModeAlwaysTemplate];
                [strongSelf->_playButton setImage:templ forState:UIControlStateNormal];
                if (strongSelf->_iconTintColor) strongSelf->_playButton.tintColor = strongSelf->_iconTintColor;
                return;
            }
        }

        if (@available(iOS 13.0, *)) {
            img = [UIImage systemImageNamed:(playing ? @"pause.fill" : @"play.fill")];
            if (img) {
                UIImage *templ = [img imageWithRenderingMode:UIImageRenderingModeAlwaysTemplate];
                [strongSelf->_playButton setImage:templ forState:UIControlStateNormal];
                if (strongSelf->_iconTintColor) strongSelf->_playButton.tintColor = strongSelf->_iconTintColor;
                return;
            }
        }
			
        UIColor *c = strongSelf->_iconTintColor ?: [UIColor blackColor];
        UIImage *fallback = playing ? [strongSelf _pauseIconWithSize:CGSizeMake(40,40) color:c] : [strongSelf _playIconWithSize:CGSizeMake(40,40) color:c];
        if (fallback) {
            [strongSelf->_playButton setImage:[fallback imageWithRenderingMode:UIImageRenderingModeAlwaysTemplate] forState:UIControlStateNormal];
            if (strongSelf->_iconTintColor) strongSelf->_playButton.tintColor = strongSelf->_iconTintColor;
        }
    });
}

- (void)setPlayImage:(UIImage *)image {
    _customPlayImage = image;
    __weak typeof(self) weakSelf = self;
    dispatch_async(dispatch_get_main_queue(), ^{
        __strong typeof(weakSelf) strongSelf = weakSelf;
        if (!strongSelf) return;
        BOOL playing = NO;
        if (strongSelf->_helper) {
            playing = ([strongSelf->_helper state] == NSCPlayerStatePlaying);
        }
        if (!playing && strongSelf->_customPlayImage) {
            UIImage *img = [strongSelf->_customPlayImage imageWithRenderingMode:UIImageRenderingModeAlwaysTemplate];
            [strongSelf->_playButton setImage:img forState:UIControlStateNormal];
            if (strongSelf->_iconTintColor) strongSelf->_playButton.tintColor = strongSelf->_iconTintColor;
        }
    });
}

- (void)setPauseImage:(UIImage *)image {
    _customPauseImage = image;
    __weak typeof(self) weakSelf = self;
    dispatch_async(dispatch_get_main_queue(), ^{
        __strong typeof(weakSelf) strongSelf = weakSelf;
        if (!strongSelf) return;
        BOOL playing = NO;
        if (strongSelf->_helper) {
            playing = ([strongSelf->_helper state] == NSCPlayerStatePlaying);
        }
        if (playing && strongSelf->_customPauseImage) {
            UIImage *img = [strongSelf->_customPauseImage imageWithRenderingMode:UIImageRenderingModeAlwaysTemplate];
            [strongSelf->_playButton setImage:img forState:UIControlStateNormal];
            if (strongSelf->_iconTintColor) strongSelf->_playButton.tintColor = strongSelf->_iconTintColor;
        }
    });
}

- (void)setIconTintColor:(UIColor *)color {
    _iconTintColor = color;
    __weak typeof(self) weakSelf = self;
    dispatch_async(dispatch_get_main_queue(), ^{
        __strong typeof(weakSelf) strongSelf = weakSelf;
        if (!strongSelf) return;
        if (strongSelf->_iconTintColor) {
            strongSelf->_playButton.tintColor = strongSelf->_iconTintColor;
        }
        BOOL playing = NO;
        if (strongSelf->_helper) {
            playing = ([strongSelf->_helper state] == NSCPlayerStatePlaying);
        }
        [strongSelf setPlaying:playing];
    });
}

- (UIImage *)_playIconWithSize:(CGSize)size color:(UIColor *)color {
    if (!color) color = [UIColor blackColor];
    UIGraphicsImageRenderer *renderer = [[UIGraphicsImageRenderer alloc] initWithSize:size];
    UIImage *img = [renderer imageWithActions:^(UIGraphicsImageRendererContext * _Nonnull ctx) {
        CGContextRef context = ctx.CGContext;
        CGContextSetFillColorWithColor(context, color.CGColor);
        CGFloat w = size.width;
        CGFloat h = size.height;
        CGPoint p1 = CGPointMake(w * 0.32, h * 0.2);
        CGPoint p2 = CGPointMake(w * 0.32, h * 0.8);
        CGPoint p3 = CGPointMake(w * 0.78, h * 0.5);
        CGContextBeginPath(context);
        CGContextMoveToPoint(context, p1.x, p1.y);
        CGContextAddLineToPoint(context, p2.x, p2.y);
        CGContextAddLineToPoint(context, p3.x, p3.y);
        CGContextClosePath(context);
        CGContextFillPath(context);
    }];
    return img;
}

- (UIImage *)_pauseIconWithSize:(CGSize)size color:(UIColor *)color {
    if (!color) color = [UIColor blackColor];
    UIGraphicsImageRenderer *renderer = [[UIGraphicsImageRenderer alloc] initWithSize:size];
    UIImage *img = [renderer imageWithActions:^(UIGraphicsImageRendererContext * _Nonnull ctx) {
        CGContextRef context = ctx.CGContext;
        CGContextSetFillColorWithColor(context, color.CGColor);
        CGFloat w = size.width;
        CGFloat h = size.height;
        CGFloat barW = w * 0.16;
        CGFloat leftX = w * 0.3 - barW/2;
        CGFloat rightX = w * 0.7 - barW/2;
        CGRect leftRect = CGRectMake(leftX, h * 0.2, barW, h * 0.6);
        CGRect rightRect = CGRectMake(rightX, h * 0.2, barW, h * 0.6);
        CGContextFillRect(context, leftRect);
        CGContextFillRect(context, rightRect);
    }];
    return img;
}

- (void)setDuration:(double)seconds {
    __weak typeof(self) weakSelf = self;
    dispatch_async(dispatch_get_main_queue(), ^{
        __strong typeof(weakSelf) strongSelf = weakSelf;
        if (!strongSelf) return;
        strongSelf->_slider.minimumValue = 0;
        if (seconds > 0) {
            strongSelf->_slider.maximumValue = (float)seconds;
        } else {
            strongSelf->_slider.maximumValue = 1.0f;
        }
        strongSelf->_durationSeconds = seconds;
        strongSelf->_slider.enabled = (!isnan(seconds) && seconds > 0);
        if (strongSelf->_slider.value > strongSelf->_slider.maximumValue) strongSelf->_slider.value = strongSelf->_slider.maximumValue;
        [strongSelf updateTimeLabelWithCurrent:strongSelf->_slider.value duration:strongSelf->_durationSeconds];
    });
}

- (void)updateTimeLabelWithCurrent:(double)seconds duration:(double)duration {
    int total = (int)seconds;
    int mins = total / 60;
    int secs = total % 60;
    int durationInt = (int)duration;
    int dmins = durationInt / 60;
    int dsecs = durationInt % 60;
    _timeLabel.text = [NSString stringWithFormat:@"%d:%02d / %d:%02d", mins, secs, dmins, dsecs];
}

@end
