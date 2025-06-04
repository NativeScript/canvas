//
//  NSCVideoHelperListener.h
//  CanvasMedia
//
//  Created by Osei Fortune on 20/05/2025.
//

#import <Foundation/Foundation.h>
#import "NSCPlayerState.h"

NS_ASSUME_NONNULL_BEGIN

@protocol NSCVideoHelperListener <NSObject>

- (void)onStateChange:(NSCPlayerState)state;
- (void)onTimeUpdate:(double)time;
- (void)onVideoFrameCallback;

@end

NS_ASSUME_NONNULL_END