#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  const void *array;
  size_t length;
} CanvasArray;

typedef struct {
  float width;
} CanvasTextMetrics;

typedef struct {
  uint8_t *array;
  size_t length;
} NativeByteArray;

typedef struct {
  const void *device;
  const void *queue;
  const void *drawable;
} CanvasDevice;

long long native_arc(long long canvas_native_ptr,
                     float x,
                     float y,
                     float radius,
                     float start_angle,
                     float end_angle,
                     bool anticlockwise);

long long native_arc_to(long long canvas_native_ptr,
                        float x1,
                        float y1,
                        float x2,
                        float y2,
                        float radius);

long long native_begin_path(long long canvas_native_ptr);

long long native_bezier_curve_to(long long canvas_native_ptr,
                                 float cp1x,
                                 float cp1y,
                                 float cp2x,
                                 float cp2y,
                                 float x,
                                 float y);

long long native_clear_canvas(long long canvas_native_ptr, void *view);

long long native_clear_rect(long long canvas_native_ptr,
                            float x,
                            float y,
                            float width,
                            float height,
                            void *view);

long long native_clip(long long canvas_native_ptr, void *view);

long long native_clip_path_rule(long long canvas_native_ptr,
                                long long path,
                                const char *fill_rule,
                                void *view);

long long native_clip_rule(long long canvas_native_ptr, const char *fill_rule, void *view);

long long native_close_path(long long canvas_native_ptr);

long long native_create_image_asset(void);

CanvasArray *native_create_image_data(size_t width, size_t height);

long long native_create_matrix(void);

long long native_create_path_2d(void);

long long native_create_path_2d_from_path_data(const char *data);

long long native_create_path_from_path(long long path);

long long native_create_pattern(uint8_t *image_array,
                                size_t image_size,
                                int original_width,
                                int original_height,
                                const char *repetition);

long long native_create_pattern_encoded(uint8_t *image_array,
                                        size_t image_size,
                                        const char *repetition);

long long native_create_text_decoder(const char *decoding);

long long native_create_text_encoder(const char *encoding);

void native_destroy(long long canvas_ptr);

long long native_draw_image(long long canvas_native_ptr,
                            const uint8_t *image_array,
                            size_t image_size,
                            int original_width,
                            int original_height,
                            float dx,
                            float dy,
                            void *view);

long long native_draw_image_dw(long long canvas_native_ptr,
                               const uint8_t *image_array,
                               size_t image_size,
                               int original_width,
                               int original_height,
                               float dx,
                               float dy,
                               float d_width,
                               float d_height,
                               void *view);

long long native_draw_image_dw_raw(long long canvas_native_ptr,
                                   const uint8_t *image_array,
                                   size_t image_size,
                                   int original_width,
                                   int original_height,
                                   float dx,
                                   float dy,
                                   float d_width,
                                   float d_height,
                                   void *view);

long long native_draw_image_raw(long long canvas_native_ptr,
                                const uint8_t *image_array,
                                size_t image_size,
                                int original_width,
                                int original_height,
                                float dx,
                                float dy,
                                void *view);

long long native_draw_image_sw(long long canvas_native_ptr,
                               const uint8_t *image_array,
                               size_t image_size,
                               int original_width,
                               int original_height,
                               float sx,
                               float sy,
                               float s_width,
                               float s_height,
                               float dx,
                               float dy,
                               float d_width,
                               float d_height,
                               void *view);

long long native_draw_image_sw_raw(long long canvas_native_ptr,
                                   const uint8_t *image_array,
                                   size_t image_size,
                                   int original_width,
                                   int original_height,
                                   float sx,
                                   float sy,
                                   float s_width,
                                   float s_height,
                                   float dx,
                                   float dy,
                                   float d_width,
                                   float d_height,
                                   void *view);

void native_drop_image_data(CanvasArray *data);

void native_drop_text_metrics(CanvasTextMetrics *data);

long long native_ellipse(long long canvas_native_ptr,
                         float x,
                         float y,
                         float radius_x,
                         float radius_y,
                         float rotation,
                         float start_angle,
                         float end_angle,
                         bool anticlockwise);

long long native_fill(long long canvas_native_ptr, void *view);

long long native_fill_path_rule(long long canvas_native_ptr,
                                long long path_ptr,
                                const char *rule,
                                void *view);

long long native_fill_rect(long long canvas_native_ptr,
                           float x,
                           float y,
                           float width,
                           float height,
                           void *view);

long long native_fill_rule(long long canvas_native_ptr, const char *rule, void *view);

long long native_fill_text(long long canvas_native_ptr,
                           const char *text,
                           float x,
                           float y,
                           float width,
                           void *view);

void native_flip_y_in_place(uint8_t *data,
                            uintptr_t length,
                            uintptr_t bytes_per_row,
                            uintptr_t height);

void native_flip_y_in_place_3d(uint8_t *data,
                               uintptr_t length,
                               uintptr_t bytes_per_row,
                               uintptr_t height,
                               uintptr_t depth);

void native_flip_y_in_place_3d_f32(float *data,
                                   uintptr_t length,
                                   uintptr_t bytes_per_row,
                                   uintptr_t height,
                                   uintptr_t depth);

void native_flip_y_in_place_3d_f64(double *data,
                                   uintptr_t length,
                                   uintptr_t bytes_per_row,
                                   uintptr_t height,
                                   uintptr_t depth);

void native_flip_y_in_place_3d_i16(int16_t *data,
                                   uintptr_t length,
                                   uintptr_t bytes_per_row,
                                   uintptr_t height,
                                   uintptr_t depth);

void native_flip_y_in_place_3d_i32(int32_t *data,
                                   uintptr_t length,
                                   uintptr_t bytes_per_row,
                                   uintptr_t height,
                                   uintptr_t depth);

void native_flip_y_in_place_3d_i8(int8_t *data,
                                  uintptr_t length,
                                  uintptr_t bytes_per_row,
                                  uintptr_t height,
                                  uintptr_t depth);

void native_flip_y_in_place_3d_u16(uint16_t *data,
                                   uintptr_t length,
                                   uintptr_t bytes_per_row,
                                   uintptr_t height,
                                   uintptr_t depth);

void native_flip_y_in_place_3d_u32(uint32_t *data,
                                   uintptr_t length,
                                   uintptr_t bytes_per_row,
                                   uintptr_t height,
                                   uintptr_t depth);

void native_flip_y_in_place_f32(float *data,
                                uintptr_t length,
                                uintptr_t bytes_per_row,
                                uintptr_t height);

void native_flip_y_in_place_f64(double *data,
                                uintptr_t length,
                                uintptr_t bytes_per_row,
                                uintptr_t height);

void native_flip_y_in_place_i16(int16_t *data,
                                uintptr_t length,
                                uintptr_t bytes_per_row,
                                uintptr_t height);

void native_flip_y_in_place_i32(int32_t *data,
                                uintptr_t length,
                                uintptr_t bytes_per_row,
                                uintptr_t height);

void native_flip_y_in_place_i8(int8_t *data,
                               uintptr_t length,
                               uintptr_t bytes_per_row,
                               uintptr_t height);

void native_flip_y_in_place_u16(uint16_t *data,
                                uintptr_t length,
                                uintptr_t bytes_per_row,
                                uintptr_t height);

void native_flip_y_in_place_u32(uint32_t *data,
                                uintptr_t length,
                                uintptr_t bytes_per_row,
                                uintptr_t height);

long long native_flush(long long canvas_ptr);

void native_free_byte_array(NativeByteArray *array);

void native_free_char(const char *text);

void native_free_matrix_data(CanvasArray *data);

void native_free_path_2d(long long path);

void native_free_pattern(long long pattern);

long long native_get_current_transform(long long canvas_native_ptr);

const char *native_get_direction(long long canvas_native_ptr);

CanvasArray *native_get_image_data(long long canvas_native_ptr,
                                   float sx,
                                   float sy,
                                   size_t sw,
                                   size_t sh);

CanvasDevice *native_get_ios_device(long long canvas_native_ptr);

CanvasArray *native_get_matrix(long long matrix);

long long native_get_vertex_attrib_offset(unsigned int index, unsigned int pname);

void native_gl_tex_image_2D_asset(unsigned int target,
                                  int level,
                                  int internalformat,
                                  int width,
                                  int height,
                                  int border,
                                  unsigned int format,
                                  unsigned int image_type,
                                  long long asset,
                                  uint8_t flip_y);

void native_gl_tex_sub_image_2D_asset(unsigned int target,
                                      int level,
                                      int xoffset,
                                      int yoffset,
                                      int width,
                                      int height,
                                      unsigned int format,
                                      unsigned int image_type,
                                      long long asset,
                                      uint8_t flip_y);

long long native_image_asset_flip_x(long long asset);

void native_image_asset_flip_x_in_place_owned(uint32_t width,
                                              uint32_t height,
                                              uint8_t *buf,
                                              uintptr_t length);

long long native_image_asset_flip_y(long long asset);

void native_image_asset_flip_y_in_place_owned(uint32_t width,
                                              uint32_t height,
                                              uint8_t *buf,
                                              uintptr_t length);

void native_image_asset_free_bytes(NativeByteArray data);

NativeByteArray native_image_asset_get_bytes(long long asset);

const char *native_image_asset_get_error(long long asset);

unsigned int native_image_asset_get_height(long long asset);

unsigned int native_image_asset_get_width(long long asset);

unsigned int native_image_asset_has_error(long long asset);

unsigned int native_image_asset_load_from_path(long long asset, const char *path);

unsigned int native_image_asset_load_from_raw(long long asset, const uint8_t *array, size_t size);

void native_image_asset_release(long long asset);

long long native_image_asset_scale(long long asset, unsigned int x, unsigned int y);

long long native_image_smoothing_enabled(long long canvas_native_ptr, bool enabled);

long long native_image_smoothing_quality(long long canvas_native_ptr, const char *quality);

long long native_init(void *device, void *queue, void *view, float scale, const char *direction);

long long native_init_legacy(int width,
                             int height,
                             int buffer_id,
                             float scale,
                             uintptr_t stencil,
                             uintptr_t samples,
                             uint8_t alpha,
                             const char *direction);

unsigned char native_is_point_in_path(int64_t canvas_ptr, float x, float y);

unsigned char native_is_point_in_path_with_path_rule(int64_t canvas_ptr,
                                                     int64_t path,
                                                     float x,
                                                     float y,
                                                     const char *fill_rule);

unsigned char native_is_point_in_path_with_rule(int64_t canvas_ptr,
                                                float x,
                                                float y,
                                                const char *fill_rule);

unsigned char native_is_point_in_stroke(int64_t canvas_ptr, float x, float y);

unsigned char native_is_point_in_stroke_with_path(int64_t canvas_ptr,
                                                  int64_t path,
                                                  float x,
                                                  float y);

long long native_line_dash_offset(long long canvas_native_ptr, float offset);

long long native_line_join(long long canvas_native_ptr, const char *line_join);

long long native_line_to(long long canvas_native_ptr, float x, float y);

CanvasTextMetrics *native_measure_text(long long canvas_native_ptr, const char *text);

long long native_miter_limit(long long canvas_native_ptr, float limit);

long long native_move_to(long long canvas_native_ptr, float x, float y);

unsigned int native_native_image_asset_save_path(long long asset,
                                                 const char *path,
                                                 unsigned int format);

long long native_path_2d_add_path(long long path, long long path_to_add, long long matrix);

long long native_path_2d_arc(long long path,
                             float x,
                             float y,
                             float radius,
                             float start_angle,
                             float end_angle,
                             bool anticlockwise);

long long native_path_2d_arc_to(long long path,
                                float x1,
                                float y1,
                                float x2,
                                float y2,
                                float radius);

long long native_path_2d_bezier_curve_to(long long path,
                                         float cp1x,
                                         float cp1y,
                                         float cp2x,
                                         float cp2y,
                                         float x,
                                         float y);

long long native_path_2d_close_path(long long path);

long long native_path_2d_ellipse(long long path,
                                 float x,
                                 float y,
                                 float radius_x,
                                 float radius_y,
                                 float rotation,
                                 float start_angle,
                                 float end_angle,
                                 bool anticlockwise);

long long native_path_2d_line_to(long long path, float x, float y);

long long native_path_2d_move_to(long long path, float x, float y);

long long native_path_2d_quadratic_curve_to(long long path, float cpx, float cpy, float x, float y);

long long native_path_2d_rect(long long path, float x, float y, float width, float height);

long long native_put_image_data(long long canvas_native_ptr,
                                size_t width,
                                size_t height,
                                const uint8_t *array,
                                size_t array_size,
                                float x,
                                float y,
                                float dirty_x,
                                float dirty_y,
                                size_t dirty_width,
                                size_t dirty_height);

long long native_quadratic_curve_to(long long canvas_native_ptr,
                                    float cpx,
                                    float cpy,
                                    float x,
                                    float y);

long long native_rect(long long canvas_native_ptr, float x, float y, float width, float height);

long long native_reset_transform(long long canvas_native_ptr);

long long native_restore(long long canvas_native_ptr);

long long native_rotate(long long canvas_native_ptr, float angle, void *view);

long long native_save(long long canvas_native_ptr);

long long native_scale(long long canvas_native_ptr, float x, float y, void *view);

long long native_set_current_transform(long long canvas_native_ptr, long long matrix);

long long native_set_direction(long long canvas_native_ptr, const char *direction);

long long native_set_fill_color(long long canvas_native_ptr, uint32_t color);

long long native_set_fill_color_rgba(long long canvas_native_ptr,
                                     uint8_t red,
                                     uint8_t green,
                                     uint8_t blue,
                                     uint8_t alpha);

long long native_set_fill_gradient_linear(long long canvas_native_ptr,
                                          float x0,
                                          float y0,
                                          float x1,
                                          float y1,
                                          size_t colors_size,
                                          const unsigned int *colors_array,
                                          size_t positions_size,
                                          const float *positions_array);

long long native_set_fill_gradient_radial(long long canvas_native_ptr,
                                          float x0,
                                          float y0,
                                          float radius_0,
                                          float x1,
                                          float y1,
                                          float radius_1,
                                          size_t colors_size,
                                          const unsigned int *colors_array,
                                          size_t positions_size,
                                          const float *positions_array);

long long native_set_fill_pattern(long long canvas_native_ptr, long long pattern);

long long native_set_font(long long canvas_native_ptr, const char *font);

long long native_set_global_alpha(long long canvas_native_ptr, uint8_t alpha);

long long native_set_global_composite_operation(long long canvas_native_ptr, const char *composite);

long long native_set_line_cap(long long canvas_native_ptr, const char *line_cap);

long long native_set_line_dash(long long canvas_native_ptr, size_t size, const float *array);

long long native_set_line_width(long long canvas_native_ptr, float line_width);

long long native_set_matrix(long long matrix, const void *array, size_t length);

long long native_set_pattern_transform(long long pattern, long long matrix);

long long native_set_stroke_color(long long canvas_native_ptr, uint32_t color);

long long native_set_stroke_color_rgba(long long canvas_native_ptr,
                                       uint8_t red,
                                       uint8_t green,
                                       uint8_t blue,
                                       uint8_t alpha);

long long native_set_stroke_gradient_linear(long long canvas_native_ptr,
                                            float x0,
                                            float y0,
                                            float x1,
                                            float y1,
                                            size_t colors_size,
                                            const unsigned int *colors_array,
                                            size_t positions_size,
                                            const float *positions_array);

long long native_set_stroke_gradient_radial(long long canvas_native_ptr,
                                            float x0,
                                            float y0,
                                            float radius_0,
                                            float x1,
                                            float y1,
                                            float radius_1,
                                            size_t colors_size,
                                            const unsigned int *colors_array,
                                            size_t positions_size,
                                            const float *positions_array);

long long native_set_stroke_pattern(long long canvas_native_ptr, long long pattern);

long long native_set_transform(long long canvas_native_ptr,
                               float a,
                               float b,
                               float c,
                               float d,
                               float e,
                               float f,
                               void *view);

long long native_shadow_blur(long long canvas_native_ptr, float limit);

long long native_shadow_color(long long canvas_native_ptr, uint32_t color);

long long native_shadow_offset_x(long long canvas_native_ptr, float x);

long long native_shadow_offset_y(long long canvas_native_ptr, float y);

NativeByteArray *native_snapshot_canvas(long long canvas_native_ptr);

long long native_stroke(long long canvas_native_ptr, void *view);

long long native_stroke_path(long long canvas_native_ptr, long long path, void *view);

long long native_stroke_rect(long long canvas_native_ptr,
                             float x,
                             float y,
                             float width,
                             float height,
                             void *view);

long long native_stroke_text(long long canvas_native_ptr,
                             const char *text,
                             float x,
                             float y,
                             float width,
                             void *view);

long long native_surface_resized(int _width,
                                 int _height,
                                 void *_device,
                                 void *_queue,
                                 float _scale,
                                 long long current_canvas);

long long native_surface_resized_legacy(int width,
                                        int height,
                                        int buffer_id,
                                        float _scale,
                                        uintptr_t stencil,
                                        uintptr_t samples,
                                        uint8_t alpha,
                                        long long canvas_native_ptr);

void native_tex_image_3D_asset(unsigned int target,
                               int level,
                               int internalformat,
                               int width,
                               int height,
                               int depth,
                               int border,
                               unsigned int format,
                               unsigned int image_type,
                               long long asset,
                               uint8_t flip_y);

void native_tex_sub_image_3D_asset(unsigned int target,
                                   int level,
                                   int xoffset,
                                   int yoffset,
                                   int zoffset,
                                   int width,
                                   int height,
                                   int depth,
                                   unsigned int format,
                                   unsigned int image_type,
                                   long long asset,
                                   uint8_t flip_y);

long long native_text_align(long long canvas_native_ptr, const char *alignment);

const char *native_text_decoder_decode(int64_t decoder, const uint8_t *data, size_t len);

const char *native_text_decoder_decode_i16(int64_t decoder, const int16_t *data, size_t len);

const char *native_text_decoder_decode_i32(int64_t decoder, const int32_t *data, size_t len);

const char *native_text_decoder_decode_u16(int64_t decoder, const uint16_t *data, size_t len);

void native_text_decoder_free(int64_t decoder);

const char *native_text_decoder_get_encoding(int64_t decoder);

NativeByteArray *native_text_encoder_encode(int64_t encoder, const char *text);

void native_text_encoder_free(int64_t encoder);

const char *native_text_encoder_get_encoding(int64_t encoder);

char *native_to_data_url(long long canvas_ptr, const char *format, float quality);

long long native_transform(long long canvas_native_ptr,
                           float a,
                           float b,
                           float c,
                           float d,
                           float e,
                           float f,
                           void *_view);

long long native_translate(long long canvas_native_ptr, float x, float y, void *view);

void native_vertex_attrib_pointer(unsigned int index,
                                  int size,
                                  unsigned int pointer_type,
                                  uint8_t normalized,
                                  int stride,
                                  long long offset);
