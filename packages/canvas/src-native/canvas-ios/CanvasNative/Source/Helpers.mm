//
//  Helpers.mm
//  CanvasNative
//
//  Created by Osei Fortune on 22/03/2023.
//

#import <Foundation/Foundation.h>
#import "canvas-ios/src/lib.rs.h"

using namespace org::nativescript::canvas;

extern "C" int64_t canvas_native_init_gl(int64_t view, int32_t width, int32_t height, bool alpha, bool antialias, bool depth, bool fail_if_major_performance_caveat, char* power_preference, bool premultiplied_alpha, bool preserve_drawing_buffer, bool stencil, bool desynchronized, bool xr_compatible, int32_t version, bool is_canvas) noexcept {
    auto pp = rust::Str(power_preference);
    
    return canvas_native_init_ios_gl(view, width, height, alpha, antialias, depth, fail_if_major_performance_caveat, pp, premultiplied_alpha, preserve_drawing_buffer, stencil, desynchronized, xr_compatible, version, is_canvas);
}
