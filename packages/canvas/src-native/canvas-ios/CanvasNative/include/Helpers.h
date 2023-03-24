//
//  Helpers.h
//  CanvasNative
//
//  Created by Osei Fortune on 22/03/2023.
//

#ifndef Helpers_h
#define Helpers_h
#include <stdint.h>

long long canvas_helper_init_gl(long long view, int width, int height, bool alpha, bool antialias, bool depth, bool fail_if_major_performance_caveat, const char* power_preference, bool premultiplied_alpha, bool preserve_drawing_buffer, bool stencil, bool desynchronized, bool xr_compatible, int version, bool is_canvas);

void canvas_helper_release_gl(long long context);

long long canvas_helper_get_gl_pointer(long long context);

void canvas_helper_release_gl_pointer(long long context);

long long canvas_helper_create_2d_context(long long context,int width, int height, bool alpha, float density, int samples, int font_color, float ppi, int direction);

void canvas_helper_update_gl_surface(long long view, int width, int height, long long context);

// Used for testing purposes
void canvas_helper_context_2d_test(long long context);


bool canvas_helper_imageasset_load_from_bytes(long long context, const unsigned char* bytes, size_t size);




long long canvas_helper_context_create_pattern(
                                               long long context,
                                               int width,
                                               int height,
                                               const unsigned char* bytes,
                                               size_t size,
                                               const char* repetition);

bool canvas_helper_context_draw_image_dx_dy_with_bytes(
                                                       long long context,
                                                       float width,
                                                       float height,
                                                       const unsigned char* bytes,
                                                       size_t size,
                                                       float dx,
                                                       float dy
                                                       );

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
                                                             );

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
                                                 );



#endif /* Helpers_h */
