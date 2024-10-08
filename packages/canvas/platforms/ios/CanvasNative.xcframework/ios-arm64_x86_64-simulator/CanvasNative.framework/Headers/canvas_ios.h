#ifndef CANVAS_IOS_H
#define CANVAS_IOS_H

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

long long canvas_native_init_ios_webgpu(int64_t instance,
                                        int64_t view,
                                        uint32_t width,
                                        uint32_t height);

#if defined(TARGET_OS_IOS)
long long canvas_native_init_ios_webgpu_uiview(int64_t instance,
                                               int64_t view,
                                               uint32_t width,
                                               uint32_t height);
#endif

#if defined(TARGET_OS_IOS)
void canvas_native_resize_ios_webgpu_uiview(int64_t context,
                                            int64_t view,
                                            uint32_t width,
                                            uint32_t height);
#endif

long long canvas_native_init_ios_gl(int64_t view,
                                    bool alpha,
                                    bool antialias,
                                    bool depth,
                                    bool fail_if_major_performance_caveat,
                                    int32_t power_preference,
                                    bool premultiplied_alpha,
                                    bool preserve_drawing_buffer,
                                    bool stencil,
                                    bool desynchronized,
                                    bool xr_compatible,
                                    int32_t version,
                                    bool is_canvas);

long long canvas_native_init_ios_gl_with_shared_gl(int64_t view,
                                                   bool alpha,
                                                   bool antialias,
                                                   bool depth,
                                                   bool fail_if_major_performance_caveat,
                                                   int32_t power_preference,
                                                   bool premultiplied_alpha,
                                                   bool preserve_drawing_buffer,
                                                   bool stencil,
                                                   bool desynchronized,
                                                   bool xr_compatible,
                                                   int32_t version,
                                                   bool is_canvas,
                                                   int64_t shared_context);

long long canvas_native_init_offscreen_ios_gl(int32_t width,
                                              int32_t height,
                                              bool alpha,
                                              bool antialias,
                                              bool depth,
                                              bool fail_if_major_performance_caveat,
                                              int32_t power_preference,
                                              bool premultiplied_alpha,
                                              bool preserve_drawing_buffer,
                                              bool stencil,
                                              bool desynchronized,
                                              bool xr_compatible,
                                              int32_t version,
                                              bool is_canvas);

long long canvas_native_init_offscreen_ios_gl_with_shared_gl(int32_t width,
                                                             int32_t height,
                                                             bool alpha,
                                                             bool antialias,
                                                             bool depth,
                                                             bool fail_if_major_performance_caveat,
                                                             int32_t power_preference,
                                                             bool premultiplied_alpha,
                                                             bool preserve_drawing_buffer,
                                                             bool stencil,
                                                             bool desynchronized,
                                                             bool xr_compatible,
                                                             int32_t version,
                                                             bool is_canvas,
                                                             int64_t shared_context);

bool canvas_native_ios_flush_gl(int64_t context);

void canvas_native_ios_flush_2d_context(int64_t context);

void canvas_native_resize_context_2d(int64_t context, float width, float height);

int64_t canvas_native_create_2d_context(int64_t context,
                                        int32_t width,
                                        int32_t height,
                                        bool alpha,
                                        float density,
                                        int32_t samples,
                                        int32_t font_color,
                                        float ppi,
                                        int32_t direction);

void canvas_native_update_gl_surface(int64_t view, int32_t width, int32_t height, int64_t context);

void canvas_native_release_ios_gl(int64_t context);

int64_t canvas_native_get_gl_pointer(int64_t gl_context);

void canvas_native_release_gl_pointer(int64_t gl_context);

void canvas_native_context_2d_test(int64_t context);

void canvas_native_gl_make_current(int64_t gl_context);

char *canvas_native_context_2d_test_to_data_url(int64_t context);

void canvas_native_context_2d_destroy_string(char *string);

bool canvas_native_imageasset_load_from_bytes(int64_t asset, uint8_t *bytes, uintptr_t size);

int64_t canvas_native_context_create_pattern_raw(int64_t context,
                                                 int32_t width,
                                                 int32_t height,
                                                 uint8_t *bytes,
                                                 uintptr_t size,
                                                 const char *repetition);

bool canvas_native_context_draw_image_dx_dy_with_bytes(int64_t context,
                                                       uint8_t *bytes,
                                                       uintptr_t size,
                                                       float width,
                                                       float height,
                                                       float dx,
                                                       float dy);

bool canvas_native_context_draw_image_dx_dy_dw_dh_with_bytes(int64_t context,
                                                             uint8_t *bytes,
                                                             uintptr_t size,
                                                             float width,
                                                             float height,
                                                             float dx,
                                                             float dy,
                                                             float d_width,
                                                             float d_height);

bool canvas_native_context_draw_image_with_bytes(int64_t context,
                                                 uint8_t *bytes,
                                                 uintptr_t size,
                                                 float width,
                                                 float height,
                                                 float sx,
                                                 float sy,
                                                 float s_width,
                                                 float s_height,
                                                 float dx,
                                                 float dy,
                                                 float d_width,
                                                 float d_height);

void canvas_native_context_custom_with_buffer_flush(int64_t context,
                                                    uint8_t *bytes,
                                                    uintptr_t size,
                                                    float width,
                                                    float height,
                                                    bool alpha);

long long canvas_native_context_init_context_with_custom_surface(float width,
                                                                 float height,
                                                                 float density,
                                                                 bool alpha,
                                                                 int font_color,
                                                                 float ppi,
                                                                 int direction);

int64_t canvas_native_context_get_texture_from_2d(int64_t context);

uint32_t canvas_native_context_backend_texture_get_id(int64_t texture);

void canvas_native_context_backend_texture_destroy(int64_t texture);

void canvas_native_webgl_tex_image_2d(int64_t context,
                                      int32_t target,
                                      int32_t level,
                                      int32_t internalformat,
                                      int32_t format,
                                      int32_t type_,
                                      uint8_t *bytes,
                                      uintptr_t size,
                                      float width,
                                      float height,
                                      bool flip_y);

void canvas_native_webgl_tex_sub_image_2d(int64_t context,
                                          int32_t target,
                                          int32_t level,
                                          int32_t xoffset,
                                          int32_t yoffset,
                                          int32_t format,
                                          int32_t type_,
                                          uint8_t *bytes,
                                          uintptr_t size,
                                          float width,
                                          float height,
                                          bool flip_y);

#endif /* CANVAS_IOS_H */
