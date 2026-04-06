//
//  NSCPlayerState.h
//  CanvasMedia
//
//  Created by Osei Fortune on 20/05/2025.
//
#import <Foundation/Foundation.h>

typedef NS_ENUM(NSUInteger, NSCPlayerState) {
    NSCPlayerStateIdle = 0,
    NSCPlayerStatePlaying = 1,
    NSCPlayerStatePaused = 2,
    NSCPlayerStateStopped = 3,
};


typedef NS_ENUM(NSUInteger, NSCPlayerReadyState) {
    NSCPlayerReadyStateHaveNothing = 0,
    NSCPlayerReadyStateHaveMetadata = 1,
    NSCPlayerReadyStateHaveCurrentData = 2,
    NSCPlayerReadyStateHaveFutureData = 3,
    NSCPlayerReadyStateHaveEnoughData = 4,
};