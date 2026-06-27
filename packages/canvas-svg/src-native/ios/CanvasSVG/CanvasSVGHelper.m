//
//  CanvasSVGHelper.m
//  CanvasSVG
//
//  Created by Osei Fortune on 18/03/2024.
//

#import <Foundation/Foundation.h>
#import "CanvasSVGHelper.h"

// visionOS has no UIScreen; derive the display scale from the current trait collection
// (scene-derived) instead. iOS keeps using UIScreen.mainScreen.nativeScale.
static CGFloat nscSVGNativeScale(void) {
#if TARGET_OS_VISION
    return UITraitCollection.currentTraitCollection.displayScale;
#else
    return UIScreen.mainScreen.nativeScale;
#endif
}

@implementation CanvasSVGHelper

+ (void)drawFromString:(uint8_t *)data size:(uintptr_t)size width:(float)width height:(float)height svg:(NSString*)svg {
    canvas_native_svg_draw_from_string(data, size, width, height, nscSVGNativeScale(), [svg UTF8String]);
}

+ (void)drawFromPath:(uint8_t *)data size:(uintptr_t)size width:(float)width height:(float)height path:(NSString*)path {
    canvas_native_svg_draw_from_path(data, size, width, height, nscSVGNativeScale(), [path UTF8String]);
}

@end
