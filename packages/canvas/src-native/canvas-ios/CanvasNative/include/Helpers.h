//
//  Helpers.h
//  CanvasNative
//
//  Created by Osei Fortune on 22/03/2023.
//

#ifndef Helpers_h
#define Helpers_h

#import "canvas-ios/src/lib.rs.h"

extern "C" int64_t canvas_native_init_gl(int64_t view, int32_t width, int32_t height, bool alpha, bool antialias, bool depth, bool fail_if_major_performance_caveat, char* power_preference, bool premultiplied_alpha, bool preserve_drawing_buffer, bool stencil, bool desynchronized, bool xr_compatible, int32_t version, bool is_canvas) noexcept;

#endif /* Helpers_h */
