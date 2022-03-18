#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum CompositeOperationType {
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

typedef enum FillRule {
  NonZero = 0,
  EvenOdd = 1,
} FillRule;

typedef enum ImageSmoothingQuality {
  Low = 0,
  Medium = 1,
  High = 2,
} ImageSmoothingQuality;

typedef enum LineCap {
  CapButt = 0,
  CapRound = 1,
  CapSquare = 2,
} LineCap;

typedef enum LineJoin {
  JoinRound = 0,
  JoinBevel = 1,
  JoinMiter = 2,
} LineJoin;

typedef enum PaintStyleValueType {
  PaintStyleValueTypeColor = 0,
  PaintStyleValueTypeGradient = 1,
  PaintStyleValueTypePattern = 2,
} PaintStyleValueType;

typedef enum Repetition {
  Repeat = 0,
  RepeatX = 1,
  RepeatY = 2,
  NoRepeat = 3,
} Repetition;

typedef enum TextAlign {
  START = 0,
  LEFT = 1,
  CENTER = 2,
  RIGHT = 3,
  END = 4,
} TextAlign;

typedef enum TextBaseLine {
  TOP = 0,
  HANGING = 1,
  MIDDLE = 2,
  ALPHABETIC = 3,
  IDEOGRAPHIC = 4,
  BOTTOM = 5,
} TextBaseLine;

typedef enum TextDirection {
  LTR = 0,
  RTL = 1,
} TextDirection;

typedef struct Context Context;

typedef struct F32Array {
  float *data;
  uintptr_t data_len;
} F32Array;

typedef struct F64Array {
  double *data;
  uintptr_t data_len;
} F64Array;

typedef struct I16Array {
  int16_t *data;
  uintptr_t data_len;
} I16Array;

typedef struct I32Array {
  int32_t *data;
  uintptr_t data_len;
} I32Array;

typedef struct I8Array {
  int8_t *data;
  uintptr_t data_len;
} I8Array;

typedef struct U16Array {
  uint16_t *data;
  uintptr_t data_len;
} U16Array;

typedef struct U32Array {
  uint32_t *data;
  uintptr_t data_len;
} U32Array;

typedef struct U8Array {
  uint8_t *data;
  uintptr_t data_len;
} U8Array;

typedef struct PaintStyleValue {
  long long value;
  enum PaintStyleValueType value_type;
} PaintStyleValue;

/**
 * AndroidBitmap functions result code.
 */
typedef int32_t _bindgen_ty_1;

/**
 * Bitmap pixel format.
 */
typedef uint32_t AndroidBitmapFormat;

/**
 * Operation was successful.
 */
#define ANDROID_BITMAP_RESULT_SUCCESS 0

/**
 * Bad parameter.
 */
#define ANDROID_BITMAP_RESULT_BAD_PARAMETER -1

/**
 * JNI exception occured.
 */
#define ANDROID_BITMAP_RESULT_JNI_EXCEPTION -2

/**
 * Allocation failed.
 */
#define ANDROID_BITMAP_RESULT_ALLOCATION_FAILED -3

/**
 * No format.
 */
#define AndroidBitmapFormat_ANDROID_BITMAP_FORMAT_NONE 0

/**
 * Red: 8 bits, Green: 8 bits, Blue: 8 bits, Alpha: 8 bits.
 */
#define AndroidBitmapFormat_ANDROID_BITMAP_FORMAT_RGBA_8888 1

/**
 * Red: 5 bits, Green: 6 bits, Blue: 5 bits.
 */
#define AndroidBitmapFormat_ANDROID_BITMAP_FORMAT_RGB_565 4

/**
 * Deprecated in API level 13. Because of the poor quality of this configuration, it is advised to use ARGB_8888 instead.
 */
#define AndroidBitmapFormat_ANDROID_BITMAP_FORMAT_RGBA_4444 7

/**
 * Alpha: 8 bits.
 */
#define AndroidBitmapFormat_ANDROID_BITMAP_FORMAT_A_8 8

void destroy_f32_array(struct F32Array *array);

void destroy_f64_array(struct F64Array *array);

void destroy_i16_array(struct I16Array *array);

void destroy_i32_array(struct I32Array *array);

void destroy_i8_array(struct I8Array *array);

void destroy_paint_style_value(long long value);

void destroy_u16_array(struct U16Array *array);

void destroy_u32_array(struct U32Array *array);

void destroy_u8_array(struct U8Array *array);

void destroy_string(const char *string);

void destroy_context(long long context);

void destroy_paint_style(long long style);

void destroy_text_metrics(long long metrics);

long long context_init_context(float width,
                               float height,
                               float density,
                               int buffer_id,
                               uintptr_t samples,
                               bool alpha,
                               unsigned int font_color,
                               float ppi,
                               enum TextDirection direction);

long long context_init_context_with_custom_surface(float width,
                                                   float height,
                                                   float density,
                                                   bool alpha,
                                                   int font_color,
                                                   float ppi,
                                                   enum TextDirection direction);

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

const char *context_data_url(long long context, const char *format, float quality);

struct U8Array *context_snapshot_canvas(long long context);

void context_flush(long long context);

void context_custom_with_buffer_flush(long long context,
                                      uint8_t *buf,
                                      uintptr_t buf_size,
                                      float width,
                                      float height);

void context_set_direction(long long context, enum TextDirection direction);

enum TextDirection context_get_direction(const struct Context *context);

void context_set_fill_style(long long context, long long style);

struct PaintStyleValue *context_get_fill_style(long long context);

void context_set_filter(long long context, const char *filter);

const char *context_get_filter(long long context);

void context_set_font(long long context, const char *filter);

const char *context_get_font(long long context);

void context_set_global_alpha(long long context, float alpha);

float context_get_global_alpha(long long context);

void context_set_global_composite_operation(long long context,
                                            enum CompositeOperationType operation);

enum CompositeOperationType context_get_global_composite_operation(long long context);

void context_set_image_smoothing_enabled(long long context, bool enabled);

bool context_get_image_smoothing_enabled(long long context);

void context_set_image_smoothing_quality(long long context, enum ImageSmoothingQuality quality);

enum ImageSmoothingQuality context_get_image_smoothing_quality(long long context);

void context_set_line_cap(long long context, enum LineCap cap);

enum LineCap context_get_line_cap(long long context);

void context_set_line_dash_offset(long long context, float offset);

float context_get_line_dash_offset(long long context);

void context_set_line_join(long long context, enum LineJoin join);

enum LineJoin context_get_line_join(long long context);

void context_set_line_width(long long context, float width);

float context_get_line_width(long long context);

void context_set_miter_limit(long long context, float limit);

float context_get_miter_limit(long long context);

void context_set_shadow_blur(long long context, float blur);

float context_get_shadow_blur(long long context);

void context_set_shadow_color(long long context, uint8_t r, uint8_t g, uint8_t b, uint8_t a);

void context_set_shadow_color_string(long long context, const char *color);

const char *context_get_shadow_color(long long context);

void context_set_shadow_offset_x(long long context, float x);

float context_get_shadow_offset_x(long long context);

void context_set_shadow_offset_y(long long context, float y);

float context_get_shadow_offset_y(long long context);

void context_set_stroke_style(long long context, long long style);

struct PaintStyleValue *context_get_stroke_style(long long context);

void context_set_text_align(long long context, enum TextAlign align);

enum TextAlign context_get_text_align(long long context);

void context_set_text_baseline(long long context, enum TextBaseLine baseline);

enum TextBaseLine context_get_text_baseline(long long context);

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

void context_clip(long long context, long long path, enum FillRule rule);

void context_clip_rule(long long context, enum FillRule rule);

void context_close_path(long long context);

long long context_create_image_data(int width, int height);

long long context_create_linear_gradient(long long context, float x0, float y0, float x1, float y1);

long long context_create_pattern(long long context,
                                 const uint8_t *image_data,
                                 uintptr_t image_len,
                                 int width,
                                 int height,
                                 enum Repetition repetition);

long long context_create_pattern_asset(long long context,
                                       long long asset,
                                       enum Repetition repetition);

long long context_create_pattern_encoded(long long context,
                                         const uint8_t *image_data,
                                         uintptr_t image_len,
                                         enum Repetition repetition);

long long context_create_radial_gradient(long long context,
                                         float x0,
                                         float y0,
                                         float r0,
                                         float x1,
                                         float y1,
                                         float r1);

void context_draw_image_dx_dy(long long context,
                              const uint8_t *image_data,
                              uintptr_t image_len,
                              float width,
                              float height,
                              float dx,
                              float dy);

void context_draw_image_dx_dy_dw_dh(long long context,
                                    const uint8_t *image_data,
                                    uintptr_t image_len,
                                    float width,
                                    float height,
                                    float dx,
                                    float dy,
                                    float d_width,
                                    float d_height);

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

void context_draw_image_dx_dy_asset(long long context, long long asset, float dx, float dy);

void context_draw_image_dx_dy_dw_dh_asset(long long context,
                                          long long asset,
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

void context_ellipse(long long context,
                     float x,
                     float y,
                     float radius_x,
                     float radius_y,
                     float rotation,
                     float start_angle,
                     float end_angle,
                     bool anticlockwise);

void context_fill(long long context, long long path, enum FillRule rule);

void context_fill_rect(long long context, float x, float y, float width, float height);

void context_fill_text(long long context, const char *text, float x, float y, float width);

long long context_get_image_data(long long context, float sx, float sy, float sw, float sh);

struct F32Array *context_get_line_dash(long long context);

long long context_get_transform(long long context);

bool context_is_point_in_path(long long context,
                              long long path,
                              float x,
                              float y,
                              enum FillRule rule);

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

void context_restore(long long context);

void context_rotate(long long context, float angle);

void context_save(long long context);

void context_scale(long long context, float x, float y);

void context_set_line_dash(long long context, const float *data, uintptr_t data_length);

void context_set_transform(long long context, float a, float b, float c, float d, float e, float f);

void context_set_transform_matrix(long long context, long long matrix);

void context_stroke(long long context, long long path);

void context_stroke_rect(long long context, float x, float y, float width, float height);

void context_stroke_text(long long context, const char *text, float x, float y, float width);

void context_transform(long long context, float a, float b, float c, float d, float e, float f);

void context_translate(long long context, float x, float y);

void gl_tex_image_2D_asset(unsigned int target,
                           int level,
                           int internalformat,
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

long long gl_get_vertex_attrib_offset(unsigned int index, unsigned int pname);

void gradient_add_color_stop(long long style, float stop, const char *color);

long long image_asset_create(void);

bool image_asset_load_from_path(long long asset, const char *path);

bool image_asset_load_from_raw(long long asset, const uint8_t *array, uintptr_t size);

struct U8Array *image_asset_get_bytes(long long asset);

struct U8Array *image_asset_get_rgba_bytes(long long asset);

struct U8Array *image_asset_get_rgb_bytes(long long asset);

unsigned int image_asset_width(long long asset);

unsigned int image_asset_height(long long asset);

const char *image_asset_get_error(long long asset);

bool image_asset_has_error(long long asset);

bool image_asset_scale(long long asset, unsigned int x, unsigned int y);

bool image_asset_flip_x(long long asset);

bool image_asset_flip_x_in_place(long long asset);

bool image_asset_flip_y(long long asset);

void image_asset_flip_y_in_place_owned(uint8_t *buf, uintptr_t length);

void image_asset_flip_x_in_place_owned(uint8_t *buf, uintptr_t length);

bool image_asset_flip_y_in_place(long long asset);

bool image_asset_save_path(long long asset, const char *path, unsigned int format);

void destroy_image_asset(long long asset);

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

int image_data_width(long long image_data);

int image_data_height(long long image_data);

uint8_t *image_data_data(long long image_data);

uintptr_t image_data_data_length(long long image_data);

void destroy_image_data(long long image_data);

long long matrix_create(void);

void matrix_update(long long matrix, const float *data, uintptr_t data_len);

float matrix_a(long long matrix);

void matrix_set_a(long long matrix, float a);

float matrix_b(long long matrix);

void matrix_set_b(long long matrix, float b);

float matrix_c(long long matrix);

void matrix_set_c(long long matrix, float c);

float matrix_d(long long matrix);

void matrix_set_d(long long matrix, float d);

float matrix_e(long long matrix);

void matrix_set_e(long long matrix, float e);

float matrix_f(long long matrix);

void matrix_set_f(long long matrix, float f);

float matrix_m11(long long matrix);

void matrix_set_m11(long long matrix, float m11);

float matrix_m12(long long matrix);

void matrix_set_m12(long long matrix, float m12);

float matrix_m13(long long matrix);

void matrix_set_m13(long long matrix, float m13);

float matrix_m14(long long matrix);

void matrix_set_m14(long long matrix, float m14);

float matrix_m21(long long matrix);

void matrix_set_m21(long long matrix, float m21);

float matrix_m22(long long matrix);

void matrix_set_m22(long long matrix, float m22);

float matrix_m23(long long matrix);

void matrix_set_m23(long long matrix, float m23);

float matrix_m24(long long matrix);

void matrix_set_m24(long long matrix, float m24);

float matrix_m31(long long matrix);

void matrix_set_m31(long long matrix, float m31);

float matrix_m32(long long matrix);

void matrix_set_m32(long long matrix, float m32);

float matrix_m33(long long matrix);

void matrix_set_m33(long long matrix, float m33);

float matrix_m34(long long matrix);

void matrix_set_m34(long long matrix, float m34);

float matrix_m41(long long matrix);

void matrix_set_m41(long long matrix, float m41);

float matrix_m42(long long matrix);

void matrix_set_m42(long long matrix, float m42);

float matrix_m43(long long matrix);

void matrix_set_m43(long long matrix, float m43);

float matrix_m44(long long matrix);

void matrix_set_m44(long long matrix, float m44);

void destroy_matrix(long long matrix);

void paint_style_set_fill_color_with_string(long long context, const char *color);

void paint_style_set_stroke_color_with_string(long long context, const char *color);

const char *paint_style_get_color_string(long long color);

long long path_create(void);

long long path_create_with_path(long long path);

long long path_create_with_string(const char *string);

void path_add_path(long long path, long long path_to_add);

void path_add_path_with_matrix(long long path, long long path_to_add, long long matrix);

void path_close_path(long long path);

void path_move_to(long long path, float x, float y);

void path_line_to(long long path, float x, float y);

void path_bezier_curve_to(long long path,
                          float cp1x,
                          float cp1y,
                          float cp2x,
                          float cp2y,
                          float x,
                          float y);

void path_quadratic_curve_to(long long path, float cpx, float cpy, float x, float y);

void path_arc(long long path,
              float x,
              float y,
              float radius,
              float start_angle,
              float end_angle,
              bool anti_clockwise);

void path_arc_to(long long path, float x1, float y1, float x2, float y2, float radius);

void path_ellipse(long long path,
                  float x,
                  float y,
                  float radius_x,
                  float radius_y,
                  float rotation,
                  float start_angle,
                  float end_angle,
                  bool anticlockwise);

void path_rect(long long path, float x, float y, float width, float height);

void destroy_path(long long path);

void pattern_set_transform(long long pattern, long long matrix);

void svg_draw_from_string(long long context, const int8_t *svg);

void svg_draw_from_path(long long context, const int8_t *path);

long long text_decoder_create(const char *decoding);

const char *text_decoder_get_encoding(long long decoder);

const char *text_decoder_decode(long long decoder, const uint8_t *data, uintptr_t len);

const struct U8Array *text_decoder_decode_to_bytes(long long decoder,
                                                   const uint8_t *data,
                                                   uintptr_t len);

const char *text_decoder_decode_u16(long long decoder, const uint16_t *data, uintptr_t len);

const char *text_decoder_decode_i16(long long decoder, const int16_t *data, uintptr_t len);

const char *text_decoder_decode_i32(long long decoder, const int32_t *data, uintptr_t len);

void destroy_text_decoder(long long decoder);

struct U8Array *text_decoder_decode_bytes(long long decoder, const uint8_t *data, uintptr_t len);

struct U8Array *text_decoder_decode_u16_bytes(long long decoder,
                                              const uint16_t *data,
                                              uintptr_t len);

struct U8Array *text_decoder_decode_i16_bytes(long long decoder,
                                              const int16_t *data,
                                              uintptr_t len);

struct U8Array *text_decoder_decode_i32_bytes(long long decoder,
                                              const int32_t *data,
                                              uintptr_t len);

struct U8Array *text_decoder_decode_u32_bytes(long long decoder,
                                              const uint32_t *data,
                                              uintptr_t len);

long long text_encoder_create(const char *encoding);

const char *text_encoder_get_encoding(long long encoder);

struct U8Array *text_encoder_encode(long long encoder, const char *text);

void destroy_text_encoder(long long encoder);

float text_metrics_get_width(long long metrics);

float text_metrics_get_actual_bounding_box_left(long long metrics);

float text_metrics_get_actual_bounding_box_right(long long metrics);

float text_metrics_get_actual_bounding_box_ascent(long long metrics);

float text_metrics_get_actual_bounding_box_descent(long long metrics);

float text_metrics_get_font_bounding_box_ascent(long long metrics);

float text_metrics_get_font_bounding_box_descent(long long metrics);

float text_metrics_get_em_height_ascent(long long metrics);

float text_metrics_get_em_height_descent(long long metrics);

float text_metrics_get_hanging_baseline(long long metrics);

float text_metrics_get_alphabetic_baseline(long long metrics);

float text_metrics_get_ideographic_baseline(long long metrics);

void flip_y_in_place(uint8_t *data, uintptr_t length, uintptr_t bytes_per_row, uintptr_t height);

void flip_y_in_place_3d(uint8_t *data,
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

void flip_y_in_place_3d_i16(int16_t *data,
                            uintptr_t length,
                            uintptr_t bytes_per_row,
                            uintptr_t height,
                            uintptr_t depth);

void flip_y_in_place_3d_u32(uint32_t *data,
                            uintptr_t length,
                            uintptr_t bytes_per_row,
                            uintptr_t height,
                            uintptr_t depth);

void flip_y_in_place_3d_i32(int32_t *data,
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

void flip_y_in_place_i8(int8_t *data, uintptr_t length, uintptr_t bytes_per_row, uintptr_t height);

void flip_y_in_place_u16(uint16_t *data,
                         uintptr_t length,
                         uintptr_t bytes_per_row,
                         uintptr_t height);

void flip_y_in_place_i16(int16_t *data,
                         uintptr_t length,
                         uintptr_t bytes_per_row,
                         uintptr_t height);

void flip_y_in_place_u32(uint32_t *data,
                         uintptr_t length,
                         uintptr_t bytes_per_row,
                         uintptr_t height);

void flip_y_in_place_i32(int32_t *data,
                         uintptr_t length,
                         uintptr_t bytes_per_row,
                         uintptr_t height);

void flip_y_in_place_f32(float *data, uintptr_t length, uintptr_t bytes_per_row, uintptr_t height);

void flip_y_in_place_f64(double *data, uintptr_t length, uintptr_t bytes_per_row, uintptr_t height);
