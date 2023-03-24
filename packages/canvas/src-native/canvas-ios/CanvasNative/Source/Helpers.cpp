//
//  Helpers.mm
//  CanvasNative
//
//  Created by Osei Fortune on 22/03/2023.
//

#include "canvas-ios/src/lib.rs.h"

using namespace org::nativescript::canvas;

long long canvas_helper_init_gl(long long view, int width, int height, bool alpha, bool antialias, bool depth, bool fail_if_major_performance_caveat, char* power_preference, bool premultiplied_alpha, bool preserve_drawing_buffer, bool stencil, bool desynchronized, bool xr_compatible, int version, bool is_canvas) {
    auto pp = rust::Str(power_preference);
    
    return canvas_native_init_ios_gl(view, width, height, alpha, antialias, depth, fail_if_major_performance_caveat, pp, premultiplied_alpha, preserve_drawing_buffer, stencil, desynchronized, xr_compatible, version, is_canvas);
}

void canvas_helper_release_gl(long long context) {
    canvas_native_release_ios_gl(context);
}

long long canvas_helper_get_gl_pointer(long long context) {
    return canvas_native_get_gl_pointer(context);
}

void canvas_helper_release_gl_pointer(long long context) {
    canvas_native_release_gl_pointer(context);
}

void canvas_helper_update_gl_surface(long long view, int width, int height, long long context) {
    canvas_native_update_gl_surface(view, width, height, context);
}


long long canvas_helper_create_2d_context(long long context,int width, int height, bool alpha, float density, int samples, int font_color, float ppi, int direction) {
    
    return canvas_native_create_2d_context(context, width, height, alpha, density, samples, font_color, ppi, direction);
}

void canvas_helper_context_2d_test(long long context) {
    canvas_native_context_2d_test(context);
}


bool canvas_helper_imageasset_load_from_bytes(long long context, const unsigned char* bytes, size_t size) {
    rust::Slice<const std::uint8_t> buf(bytes, size);
    return canvas_native_imageasset_load_from_bytes(context, buf);
}




long long canvas_helper_context_create_pattern(
                                               long long context,
                                               int width,
                                               int height,
                                               const unsigned char* bytes,
                                               size_t size,
                                               const char* repetition) {
    rust::Slice<const std::uint8_t> buf(bytes, size);
    rust::Str rep(repetition);
    return canvas_native_context_create_pattern(context,width, height, buf, rep);
}

bool canvas_helper_context_draw_image_dx_dy_with_bytes(
                                                       long long context,
                                                       float width,
                                                       float height,
                                                       const unsigned char* bytes,
                                                       size_t size,
                                                       float dx,
                                                       float dy
                                                       ) {
    rust::Slice<const std::uint8_t> buf(bytes, size);
    return canvas_native_context_draw_image_dx_dy_with_bytes(
                                                             context, buf, width, height, dx, dy
                                                             );
}

bool canvas_helper_content_draw_image_dx_dy_dw_dh_with_bytes(
                                                             long long context,
                                                             float width,
                                                             float height,
                                                             const unsigned char* bytes,
                                                             size_t size,
                                                             float dx,
                                                             float dy,
                                                             float d_width,
                                                             float d_height
                                                             ) {
    rust::Slice<const std::uint8_t> buf(bytes, size);
    return canvas_native_content_draw_image_dx_dy_dw_dh_with_bytes(
                                                                   context, buf, width, height,dx, dy, d_width, d_height);
}

bool canvas_helper_context_draw_image_with_bytes(
                                                 long long context,
                                                 float width,
                                                 float height,
                                                 const unsigned char* bytes,
                                                 size_t size,
                                                 float sx,
                                                 float sy,
                                                 float s_width,
                                                 float s_height,
                                                 float dx,
                                                 float dy,
                                                 float d_width,
                                                 float d_height
                                                 ){
    
    rust::Slice<const std::uint8_t> buf(bytes, size);
    return canvas_native_context_draw_image_with_bytes(
                                                       context, buf, width, height, sx, sy, s_width, s_height, dx, dy, d_width, d_height);
}
