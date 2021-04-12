#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
  SourceOver = 0,
  SourceIn = 1,
  SourceOut = 2,
  SourceAtop = 3,
  DestinationOver = 4,
  DestinationIn = 5,
  DestinationOut = 6,
  DestinationAtop = 7,
  Lighter = 8,
  Copy = 9,
  Xor = 10,
  Multiply = 11,
  Screen = 12,
  Overlay = 13,
  Darken = 14,
  Lighten = 15,
  ColorDodge = 16,
  ColorBurn = 17,
  HardLight = 18,
  SoftLight = 19,
  Difference = 20,
  Exclusion = 21,
  Hue = 22,
  Saturation = 23,
  Color = 24,
  Luminosity = 25,
} CompositeOperationType;

typedef enum {
  NonZero = 0,
  EvenOdd = 1,
} FillRule;

typedef enum {
  Low = 0,
  Medium = 1,
  High = 2,
} ImageSmoothingQuality;

typedef enum {
  CapButt = 0,
  CapRound = 1,
  CapSquare = 2,
} LineCap;

typedef enum {
  JoinRound = 0,
  JoinBevel = 1,
  JoinMiter = 2,
} LineJoin;

typedef enum {
  PaintStyleValueTypeColor = 0,
  PaintStyleValueTypeGradient = 1,
  PaintStyleValueTypePattern = 2,
} PaintStyleValueType;

typedef enum {
  Repeat = 0,
  RepeatX = 1,
  RepeatY = 2,
  NoRepeat = 3,
} Repetition;

typedef enum {
  START = 0,
  LEFT = 1,
  CENTER = 2,
  RIGHT = 3,
  END = 4,
} TextAlign;

typedef enum {
  TOP = 0,
  HANGING = 1,
  MIDDLE = 2,
  ALPHABETIC = 3,
  IDEOGRAPHIC = 4,
  BOTTOM = 5,
} TextBaseLine;

typedef enum {
  LTR = 0,
  RTL = 1,
} TextDirection;

typedef struct Context Context;

typedef struct {
  long long value;
  PaintStyleValueType value_type;
} PaintStyleValue;

typedef struct {
  float *data;
  uintptr_t data_len;
} F32Array;

typedef struct {
  uint8_t *data;
  uintptr_t data_len;
} U8Array;

typedef struct {
  double *data;
  uintptr_t data_len;
} F64Array;

typedef struct {
  int16_t *data;
  uintptr_t data_len;
} I16Array;

typedef struct {
  int32_t *data;
  uintptr_t data_len;
} I32Array;

typedef struct {
  int8_t *data;
  uintptr_t data_len;
} I8Array;

typedef struct {
  uint16_t *data;
  uintptr_t data_len;
} U16Array;

typedef struct {
  uint32_t *data;
  uintptr_t data_len;
} U32Array;

void context_arc(long long context,
                 float x,
                 float y,
                 float radius,
                 float start_angle,
                 float end_angle,
                 bool anti_clockwise);

void context_arc_to(long long context, float x1, float y1, float x2, float y2, float radius);

void context_begin_path(long long context);

void context_bezier_curve_to(long long context,
                             float cp1x,
                             float cp1y,
                             float cp2x,
                             float cp2y,
                             float x,
                             float y);

void context_clear_rect(long long context, float x, float y, float width, float height);

void context_clip(long long context, long long path, FillRule rule);

void context_clip_rule(long long context, FillRule rule);

void context_close_path(long long context);

long long context_create_image_data(int width, int height);

long long context_create_linear_gradient(long long context, float x0, float y0, float x1, float y1);

long long context_create_pattern(long long context,
                                 const uint8_t *image_data,
                                 uintptr_t image_len,
                                 int width,
                                 int height,
                                 Repetition repetition);

long long context_create_pattern_asset(long long context, long long asset, Repetition repetition);

long long context_create_pattern_encoded(long long context,
                                         const uint8_t *image_data,
                                         uintptr_t image_len,
                                         Repetition repetition);

long long context_create_radial_gradient(long long context,
                                         float x0,
                                         float y0,
                                         float r0,
                                         float x1,
                                         float y1,
                                         float r1);

void context_custom_with_buffer_flush(long long context,
                                      uint8_t *buf,
                                      uintptr_t buf_size,
                                      float width,
                                      float height);

const char *context_data_url(long long context, const char *format, float quality);

void context_draw_image(long long context,
                        const uint8_t *image_data,
                        uintptr_t image_len,
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

void context_draw_image_asset(long long context,
                              long long asset,
                              float sx,
                              float sy,
                              float s_width,
                              float s_height,
                              float dx,
                              float dy,
                              float d_width,
                              float d_height);

void context_draw_image_dx_dy(long long context,
                              const uint8_t *image_data,
                              uintptr_t image_len,
                              float width,
                              float height,
                              float dx,
                              float dy);

void context_draw_image_dx_dy_asset(long long context, long long asset, float dx, float dy);

void context_draw_image_dx_dy_dw_dh(long long context,
                                    const uint8_t *image_data,
                                    uintptr_t image_len,
                                    float width,
                                    float height,
                                    float dx,
                                    float dy,
                                    float d_width,
                                    float d_height);

void context_draw_image_dx_dy_dw_dh_asset(long long context,
                                          long long asset,
                                          float dx,
                                          float dy,
                                          float d_width,
                                          float d_height);

void context_draw_image_encoded(long long context,
                                const uint8_t *image_data,
                                uintptr_t image_len,
                                float sx,
                                float sy,
                                float s_width,
                                float s_height,
                                float dx,
                                float dy,
                                float d_width,
                                float d_height);

void context_draw_image_encoded_dx_dy(long long context,
                                      const uint8_t *image_data,
                                      uintptr_t image_len,
                                      float dx,
                                      float dy);

void context_draw_image_encoded_dx_dy_dw_dh(long long context,
                                            const uint8_t *image_data,
                                            uintptr_t image_len,
                                            float dx,
                                            float dy,
                                            float d_width,
                                            float d_height);

void context_ellipse(long long context,
                     float x,
                     float y,
                     float radius_x,
                     float radius_y,
                     float rotation,
                     float start_angle,
                     float end_angle,
                     bool anticlockwise);

void context_fill(long long context, long long path, FillRule rule);

void context_fill_rect(long long context, float x, float y, float width, float height);

void context_fill_text(long long context, const char *text, float x, float y, float width);

void context_flush(long long context);

TextDirection context_get_direction(const Context *context);

PaintStyleValue *context_get_fill_style(long long context);

const char *context_get_filter(long long context);

const char *context_get_font(long long context);

float context_get_global_alpha(long long context);

CompositeOperationType context_get_global_composite_operation(long long context);

long long context_get_image_data(long long context, float sx, float sy, float sw, float sh);

bool context_get_image_smoothing_enabled(long long context);

ImageSmoothingQuality context_get_image_smoothing_quality(long long context);

LineCap context_get_line_cap(long long context);

F32Array *context_get_line_dash(long long context);

float context_get_line_dash_offset(long long context);

LineJoin context_get_line_join(long long context);

float context_get_line_width(long long context);

float context_get_miter_limit(long long context);

float context_get_shadow_blur(long long context);

const char *context_get_shadow_color(long long context);

float context_get_shadow_offset_x(long long context);

float context_get_shadow_offset_y(long long context);

PaintStyleValue *context_get_stroke_style(long long context);

TextAlign context_get_text_align(long long context);

TextBaseLine context_get_text_baseline(long long context);

long long context_get_transform(long long context);

long long context_init_context(float width,
                               float height,
                               float density,
                               int buffer_id,
                               uintptr_t samples,
                               bool alpha,
                               unsigned int font_color,
                               float ppi,
                               TextDirection direction);

long long context_init_context_with_custom_surface(float width,
                                                   float height,
                                                   float density,
                                                   bool alpha,
                                                   int font_color,
                                                   float ppi,
                                                   TextDirection direction);

bool context_is_point_in_path(long long context, long long path, float x, float y, FillRule rule);

bool context_is_point_in_stroke(long long context, long long path, float x, float y);

void context_line_to(long long context, float x, float y);

long long context_measure_text(long long context, const char *text);

void context_move_to(long long context, float x, float y);

void context_put_image_data(long long context,
                            long long image_data,
                            float dx,
                            float dy,
                            float dirty_x,
                            float dirty_y,
                            float dirty_width,
                            float dirty_height);

void context_quadratic_curve_to(long long context, float cpx, float cpy, float x, float y);

void context_rect(long long context, float x, float y, float width, float height);

void context_reset_transform(long long context);

void context_resize_custom_surface(long long context,
                                   float width,
                                   float height,
                                   float density,
                                   bool alpha,
                                   float ppi);

void context_resize_surface(long long context,
                            float width,
                            float height,
                            float density,
                            int buffer_id,
                            uintptr_t samples,
                            bool alpha,
                            float ppi);

void context_restore(long long context);

void context_rotate(long long context, float angle);

void context_save(long long context);

void context_scale(long long context, float x, float y);

void context_set_direction(long long context, TextDirection direction);

void context_set_fill_style(long long context, long long style);

void context_set_filter(long long context, const char *filter);

void context_set_font(long long context, const char *filter);

void context_set_global_alpha(long long context, float alpha);

void context_set_global_composite_operation(long long context, CompositeOperationType operation);

void context_set_image_smoothing_enabled(long long context, bool enabled);

void context_set_image_smoothing_quality(long long context, ImageSmoothingQuality quality);

void context_set_line_cap(long long context, LineCap cap);

void context_set_line_dash(long long context, const float *data, uintptr_t data_length);

void context_set_line_dash_offset(long long context, float offset);

void context_set_line_join(long long context, LineJoin join);

void context_set_line_width(long long context, float width);

void context_set_miter_limit(long long context, float limit);

void context_set_shadow_blur(long long context, float blur);

void context_set_shadow_color(long long context, uint8_t r, uint8_t g, uint8_t b, uint8_t a);

void context_set_shadow_color_string(long long context, const char *color);

void context_set_shadow_offset_x(long long context, float x);

void context_set_shadow_offset_y(long long context, float y);

void context_set_stroke_style(long long context, long long style);

void context_set_text_align(long long context, TextAlign align);

void context_set_text_baseline(long long context, TextBaseLine baseline);

void context_set_transform(long long context, float a, float b, float c, float d, float e, float f);

void context_set_transform_matrix(long long context, long long matrix);

U8Array *context_snapshot_canvas(long long context);

void context_stroke(long long context, long long path);

void context_stroke_rect(long long context, float x, float y, float width, float height);

void context_stroke_text(long long context, const char *text, float x, float y, float width);

void context_transform(long long context, float a, float b, float c, float d, float e, float f);

void context_translate(long long context, float x, float y);

void destroy_context(long long context);

void destroy_f32_array(F32Array *array);

void destroy_f64_array(F64Array *array);

void destroy_i16_array(I16Array *array);

void destroy_i32_array(I32Array *array);

void destroy_i8_array(I8Array *array);

void destroy_image_asset(long long asset);

void destroy_image_data(long long image_data);

void destroy_matrix(long long matrix);

void destroy_paint_style(long long style);

void destroy_paint_style_value(long long value);

void destroy_path(long long path);

void destroy_string(const char *string);

void destroy_text_decoder(long long decoder);

void destroy_text_encoder(long long encoder);

void destroy_text_metrics(long long metrics);

void destroy_u16_array(U16Array *array);

void destroy_u32_array(U32Array *array);

void destroy_u8_array(U8Array *array);

void flip_y_in_place(uint8_t *data, uintptr_t length, uintptr_t bytes_per_row, uintptr_t height);

void flip_y_in_place_3d(uint8_t *data,
                        uintptr_t length,
                        uintptr_t bytes_per_row,
                        uintptr_t height,
                        uintptr_t depth);

void flip_y_in_place_3d_f32(float *data,
                            uintptr_t length,
                            uintptr_t bytes_per_row,
                            uintptr_t height,
                            uintptr_t depth);

void flip_y_in_place_3d_f64(double *data,
                            uintptr_t length,
                            uintptr_t bytes_per_row,
                            uintptr_t height,
                            uintptr_t depth);

void flip_y_in_place_3d_i16(int16_t *data,
                            uintptr_t length,
                            uintptr_t bytes_per_row,
                            uintptr_t height,
                            uintptr_t depth);

void flip_y_in_place_3d_i32(int32_t *data,
                            uintptr_t length,
                            uintptr_t bytes_per_row,
                            uintptr_t height,
                            uintptr_t depth);

void flip_y_in_place_3d_i8(int8_t *data,
                           uintptr_t length,
                           uintptr_t bytes_per_row,
                           uintptr_t height,
                           uintptr_t depth);

void flip_y_in_place_3d_u16(uint16_t *data,
                            uintptr_t length,
                            uintptr_t bytes_per_row,
                            uintptr_t height,
                            uintptr_t depth);

void flip_y_in_place_3d_u32(uint32_t *data,
                            uintptr_t length,
                            uintptr_t bytes_per_row,
                            uintptr_t height,
                            uintptr_t depth);

void flip_y_in_place_f32(float *data, uintptr_t length, uintptr_t bytes_per_row, uintptr_t height);

void flip_y_in_place_f64(double *data, uintptr_t length, uintptr_t bytes_per_row, uintptr_t height);

void flip_y_in_place_i16(int16_t *data,
                         uintptr_t length,
                         uintptr_t bytes_per_row,
                         uintptr_t height);

void flip_y_in_place_i32(int32_t *data,
                         uintptr_t length,
                         uintptr_t bytes_per_row,
                         uintptr_t height);

void flip_y_in_place_i8(int8_t *data, uintptr_t length, uintptr_t bytes_per_row, uintptr_t height);

void flip_y_in_place_u16(uint16_t *data,
                         uintptr_t length,
                         uintptr_t bytes_per_row,
                         uintptr_t height);

void flip_y_in_place_u32(uint32_t *data,
                         uintptr_t length,
                         uintptr_t bytes_per_row,
                         uintptr_t height);

long long gl_get_vertex_attrib_offset(unsigned int index, unsigned int pname);

void gl_tex_image_2D_asset(unsigned int target,
                           int level,
                           int internalformat,
                           int border,
                           unsigned int format,
                           unsigned int image_type,
                           long long asset,
                           bool flip_y);

void gl_tex_image_3D_asset(unsigned int target,
                           int level,
                           int internalformat,
                           int width,
                           int height,
                           int depth,
                           int border,
                           unsigned int format,
                           unsigned int image_type,
                           long long asset,
                           bool flip_y);

void gl_tex_sub_image_2D_asset(unsigned int target,
                               int level,
                               int xoffset,
                               int yoffset,
                               unsigned int format,
                               unsigned int image_type,
                               long long asset,
                               bool flip_y);

void gl_tex_sub_image_3D_asset(unsigned int target,
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
                               bool flip_y);

void gl_vertex_attrib_pointer(unsigned int index,
                              int size,
                              unsigned int pointer_type,
                              bool normalized,
                              int stride,
                              long long offset);

void gradient_add_color_stop(long long style, float stop, const char *color);

long long image_asset_create(void);

bool image_asset_flip_x(long long asset);

bool image_asset_flip_x_in_place(long long asset);

void image_asset_flip_x_in_place_owned(uint8_t *buf, uintptr_t length);

bool image_asset_flip_y(long long asset);

bool image_asset_flip_y_in_place(long long asset);

void image_asset_flip_y_in_place_owned(uint8_t *buf, uintptr_t length);

U8Array *image_asset_get_bytes(long long asset);

const char *image_asset_get_error(long long asset);

U8Array *image_asset_get_rgb_bytes(long long asset);

U8Array *image_asset_get_rgba_bytes(long long asset);

bool image_asset_has_error(long long asset);

unsigned int image_asset_height(long long asset);

bool image_asset_load_from_path(long long asset, const char *path);

bool image_asset_load_from_raw(long long asset, const uint8_t *array, uintptr_t size);

bool image_asset_save_path(long long asset, const char *path, unsigned int format);

bool image_asset_scale(long long asset, unsigned int x, unsigned int y);

unsigned int image_asset_width(long long asset);

long long image_bitmap_create_from_bytes(const uint8_t *image_bytes,
                                         uintptr_t image_size,
                                         float image_width,
                                         float image_height,
                                         bool flip_y,
                                         int32_t premultiply_alpha,
                                         int32_t color_space_conversion,
                                         int32_t resize_quality,
                                         float resize_width,
                                         float resize_height);

long long image_bitmap_create_from_bytes_encoded(const uint8_t *image_bytes,
                                                 uintptr_t image_size,
                                                 bool flip_y,
                                                 int32_t premultiply_alpha,
                                                 int32_t color_space_conversion,
                                                 int32_t resize_quality,
                                                 float resize_width,
                                                 float resize_height);

long long image_bitmap_create_from_bytes_encoded_src_rect(const uint8_t *image_bytes,
                                                          uintptr_t image_size,
                                                          float sx,
                                                          float sy,
                                                          float s_width,
                                                          float s_height,
                                                          bool flip_y,
                                                          int32_t premultiply_alpha,
                                                          int32_t color_space_conversion,
                                                          int32_t resize_quality,
                                                          float resize_width,
                                                          float resize_height);

long long image_bitmap_create_from_bytes_src_rect(const uint8_t *image_bytes,
                                                  uintptr_t image_size,
                                                  float image_width,
                                                  float image_height,
                                                  float sx,
                                                  float sy,
                                                  float s_width,
                                                  float s_height,
                                                  bool flip_y,
                                                  int32_t premultiply_alpha,
                                                  int32_t color_space_conversion,
                                                  int32_t resize_quality,
                                                  float resize_width,
                                                  float resize_height);

long long image_bitmap_create_from_image_asset(long long asset,
                                               bool flip_y,
                                               int premultiply_alpha,
                                               int color_space_conversion,
                                               int resize_quality,
                                               float resize_width,
                                               float resize_height);

long long image_bitmap_create_from_image_asset_src_rect(long long asset,
                                                        float sx,
                                                        float sy,
                                                        float s_width,
                                                        float s_height,
                                                        bool flip_y,
                                                        int premultiply_alpha,
                                                        int color_space_conversion,
                                                        int resize_quality,
                                                        float resize_width,
                                                        float resize_height);

long long image_bitmap_create_from_image_data(long long image_data,
                                              bool flip_y,
                                              int premultiply_alpha,
                                              int color_space_conversion,
                                              int resize_quality,
                                              float resize_width,
                                              float resize_height);

long long image_bitmap_create_from_image_data_src_rect(long long image_data,
                                                       float sx,
                                                       float sy,
                                                       float s_width,
                                                       float s_height,
                                                       bool flip_y,
                                                       int premultiply_alpha,
                                                       int color_space_conversion,
                                                       int resize_quality,
                                                       float resize_width,
                                                       float resize_height);

long long image_data_create(int width, int height);

uint8_t *image_data_data(long long image_data);

uintptr_t image_data_data_length(long long image_data);

int image_data_height(long long image_data);

int image_data_width(long long image_data);

float matrix_a(long long matrix);

float matrix_b(long long matrix);

float matrix_c(long long matrix);

long long matrix_create(void);

float matrix_d(long long matrix);

float matrix_e(long long matrix);

float matrix_f(long long matrix);

float matrix_m11(long long matrix);

float matrix_m12(long long matrix);

float matrix_m13(long long matrix);

float matrix_m14(long long matrix);

float matrix_m21(long long matrix);

float matrix_m22(long long matrix);

float matrix_m23(long long matrix);

float matrix_m24(long long matrix);

float matrix_m31(long long matrix);

float matrix_m32(long long matrix);

float matrix_m33(long long matrix);

float matrix_m34(long long matrix);

float matrix_m41(long long matrix);

float matrix_m42(long long matrix);

float matrix_m43(long long matrix);

float matrix_m44(long long matrix);

void matrix_set_a(long long matrix, float a);

void matrix_set_b(long long matrix, float b);

void matrix_set_c(long long matrix, float c);

void matrix_set_d(long long matrix, float d);

void matrix_set_e(long long matrix, float e);

void matrix_set_f(long long matrix, float f);

void matrix_set_m11(long long matrix, float m11);

void matrix_set_m12(long long matrix, float m12);

void matrix_set_m13(long long matrix, float m13);

void matrix_set_m14(long long matrix, float m14);

void matrix_set_m21(long long matrix, float m21);

void matrix_set_m22(long long matrix, float m22);

void matrix_set_m23(long long matrix, float m23);

void matrix_set_m24(long long matrix, float m24);

void matrix_set_m31(long long matrix, float m31);

void matrix_set_m32(long long matrix, float m32);

void matrix_set_m33(long long matrix, float m33);

void matrix_set_m34(long long matrix, float m34);

void matrix_set_m41(long long matrix, float m41);

void matrix_set_m42(long long matrix, float m42);

void matrix_set_m43(long long matrix, float m43);

void matrix_set_m44(long long matrix, float m44);

void matrix_update(long long matrix, const float *data, uintptr_t data_len);

const char *paint_style_get_color_string(long long color);

void paint_style_set_fill_color_with_string(long long context, const char *color);

void paint_style_set_stroke_color_with_string(long long context, const char *color);

void path_add_path(long long path, long long path_to_add);

void path_add_path_with_matrix(long long path, long long path_to_add, long long matrix);

void path_arc(long long path,
              float x,
              float y,
              float radius,
              float start_angle,
              float end_angle,
              bool anti_clockwise);

void path_arc_to(long long path, float x1, float y1, float x2, float y2, float radius);

void path_bezier_curve_to(long long path,
                          float cp1x,
                          float cp1y,
                          float cp2x,
                          float cp2y,
                          float x,
                          float y);

void path_close_path(long long path);

long long path_create(void);

long long path_create_with_path(long long path);

long long path_create_with_string(const char *string);

void path_ellipse(long long path,
                  float x,
                  float y,
                  float radius_x,
                  float radius_y,
                  float rotation,
                  float start_angle,
                  float end_angle,
                  bool anticlockwise);

void path_line_to(long long path, float x, float y);

void path_move_to(long long path, float x, float y);

void path_quadratic_curve_to(long long path, float cpx, float cpy, float x, float y);

void path_rect(long long path, float x, float y, float width, float height);

void pattern_set_transform(long long pattern, long long matrix);

void svg_draw_from_path(long long context, const int8_t *path);

void svg_draw_from_string(long long context, const int8_t *svg);

long long text_decoder_create(const char *decoding);

const char *text_decoder_decode(long long decoder, const uint8_t *data, uintptr_t len);

const char *text_decoder_decode_i16(long long decoder, const int16_t *data, uintptr_t len);

const char *text_decoder_decode_i32(long long decoder, const int32_t *data, uintptr_t len);

const char *text_decoder_decode_u16(long long decoder, const uint16_t *data, uintptr_t len);

const char *text_decoder_get_encoding(long long decoder);

long long text_encoder_create(const char *encoding);

U8Array *text_encoder_encode(long long encoder, const char *text);

const char *text_encoder_get_encoding(long long encoder);

float text_metrics_get_actual_bounding_box_ascent(long long metrics);

float text_metrics_get_actual_bounding_box_descent(long long metrics);

float text_metrics_get_actual_bounding_box_left(long long metrics);

float text_metrics_get_actual_bounding_box_right(long long metrics);

float text_metrics_get_alphabetic_baseline(long long metrics);

float text_metrics_get_em_height_ascent(long long metrics);

float text_metrics_get_em_height_descent(long long metrics);

float text_metrics_get_font_bounding_box_ascent(long long metrics);

float text_metrics_get_font_bounding_box_descent(long long metrics);

float text_metrics_get_hanging_baseline(long long metrics);

float text_metrics_get_ideographic_baseline(long long metrics);

float text_metrics_get_width(long long metrics);
