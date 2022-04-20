use std::ffi::{CStr, CString};
use std::fmt::format;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use cxx::{CxxString, ExternType, let_cxx_string, SharedPtr, type_id};
use parking_lot::{Mutex, MutexGuard, RawMutex};

use canvas_core::context::{Context, ContextWrapper};
use canvas_core::context::compositing::composite_operation_type::CompositeOperationType;
use canvas_core::context::drawing_paths::fill_rule::FillRule;
use canvas_core::context::fill_and_stroke_styles::paint::paint_style_set_color_with_string;
use canvas_core::context::fill_and_stroke_styles::pattern::Repetition;
use canvas_core::context::image_smoothing::ImageSmoothingQuality;
use canvas_core::context::line_styles::line_cap::LineCap;
use canvas_core::context::line_styles::line_join::LineJoin;
use canvas_core::context::text_styles::text_align::TextAlign;
use canvas_core::context::text_styles::text_direction::TextDirection;
use canvas_core::utils::color::{parse_color, to_parsed_color};
use canvas_core::utils::image::{
    from_image_slice, from_image_slice_encoded, to_image, to_image_encoded,
};

use crate::gl_context::GLContext;

/* Utils */

#[derive(Clone, Copy)]
#[repr(isize)]
pub enum LogPriority {
    UNKNOWN = 0,
    DEFAULT = 1,
    VERBOSE = 2,
    DEBUG = 3,
    INFO = 4,
    WARN = 5,
    ERROR = 6,
    FATAL = 7,
    SILENT = 8,
}

impl TryFrom<isize> for LogPriority {
    type Error = &'static str;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value < 0 || value > 8 {
            Err("Invalid LogPriority")
        } else {
            match value {
                0 => Ok(LogPriority::UNKNOWN),
                1 => Ok(LogPriority::DEFAULT),
                2 => Ok(LogPriority::VERBOSE),
                3 => Ok(LogPriority::DEBUG),
                4 => Ok(LogPriority::INFO),
                5 => Ok(LogPriority::WARN),
                6 => Ok(LogPriority::ERROR),
                7 => Ok(LogPriority::FATAL),
                8 => Ok(LogPriority::SILENT),
                _ => Err("Invalid LogPriority"),
            }
        }
    }
}

extern "C" {
    pub fn __android_log_write(prio: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}

pub fn _log(priority: isize, tag: &str, text: &str) {
    __log(priority.try_into().unwrap(), tag, text);
}

pub fn __log(priority: LogPriority, tag: &str, text: &str) {
    let tag = CString::new(tag).unwrap();
    let text = CString::new(text).unwrap();
    unsafe {
        __android_log_write(priority as c_int, tag.as_ptr(), text.as_ptr());
    }
}

pub fn console_log(text: &str) {
    __log(LogPriority::INFO, "JS", text);
}

pub fn to_rust_string(value: &[c_char]) -> String {
    let val = unsafe { CStr::from_ptr(value.as_ptr()) }
        .to_string_lossy()
        .to_string();
    if value.is_empty() {
        return String::new();
    }
    unsafe { CStr::from_ptr(value.as_ptr()).to_string_lossy().to_string() }
}

/* Utils */

/* TextEncoder */
#[derive(Clone)]
pub struct TextEncoder(canvas_core::context::text_encoder::TextEncoder);
/* TextEncoder */

/* TextDecoder */
#[derive(Clone)]
pub struct TextDecoder(canvas_core::context::text_decoder::TextDecoder);
/* TextDecoder */


/* Raf */
#[derive(Clone)]
pub struct Raf(crate::raf::Raf);
/* Raf */

/* CanvasRenderingContext2D */

pub struct CanvasRenderingContext2D {
    context: canvas_core::context::ContextWrapper,
    gl_context: GLContext,
}

impl CanvasRenderingContext2D {
    pub fn get_context(&self) -> MutexGuard<Context> {
        self.context.get_context()
    }
}

#[derive(Clone)]
pub struct PaintStyle(Option<canvas_core::context::fill_and_stroke_styles::paint::PaintStyle>);

impl PaintStyle {
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn style_type(&self) -> ffi::PaintStyleType {
        if let Some(style) = self.0.as_ref() {
            return match style {
                canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Color(_) => {
                    ffi::PaintStyleType::Color
                }
                canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
                    ffi::PaintStyleType::Gradient
                }
                canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
                    ffi::PaintStyleType::Pattern
                }
            };
        }
        return ffi::PaintStyleType::None;
    }
}

#[derive(Clone, Copy)]
pub struct TextMetrics(canvas_core::context::drawing_text::text_metrics::TextMetrics);

#[cxx::bridge]
mod ffi {
    pub(crate) enum PaintStyleType {
        None,
        Color,
        Gradient,
        Pattern,
    }

    pub(crate) enum InvalidateState {
        NONE,
        PENDING,
        INVALIDATING,
    }

    unsafe extern "C++" {
        include!("canvas-android-v8/src/OnImageAssetLoadCallbackHolder.h");
        fn OnImageAssetLoadCallbackHolderComplete(callback: isize, complete: bool);
    }

    unsafe extern "C++" {
        include!("canvas-android-v8/src/OnRafCallback.h");
        fn OnRafCallbackOnFrame(callback: isize, ts: i64);
    }

    extern "Rust" {
        type CanvasRenderingContext2D;
        type PaintStyle;
        type TextMetrics;
        type Path;
        type Matrix;
        type ImageData;
        type ImageAsset;
        type TextDecoder;
        type TextEncoder;
        type Raf;

        /* Utils */
        fn _log(priority: isize, tag: &str, text: &str);
        fn console_log(text: &str);
        fn to_rust_string(value: &[c_char]) -> String;
        /* Utils */

        /* Path2D */
        fn canvas_native_path_add_path(path: &mut Path, path_to_add: &Path);
        fn canvas_native_path_create() -> Box<Path>;
        fn canvas_native_path_create_with_path(path: &Path) -> Box<Path>;
        fn canvas_native_path_create_with_string(string: String) -> Box<Path>;
        fn canvas_native_path_close_path(path: &mut Path);
        fn canvas_native_path_move_to(path: &mut Path, x: f32, y: f32);
        fn canvas_native_path_line_to(path: &mut Path, x: f32, y: f32);
        fn canvas_native_path_bezier_curve_to(
            path: &mut Path,
            cp1x: f32,
            cp1y: f32,
            cp2x: f32,
            cp2y: f32,
            x: f32,
            y: f32,
        );

        fn canvas_native_path_quadratic_curve_to(
            path: &mut Path,
            cpx: f32,
            cpy: f32,
            x: f32,
            y: f32,
        );

        fn canvas_native_path_arc(
            path: &mut Path,
            x: f32,
            y: f32,
            radius: f32,
            start_angle: f32,
            end_angle: f32,
            anti_clockwise: bool,
        );

        fn canvas_native_path_arc_to(
            path: &mut Path,
            x1: f32,
            y1: f32,
            x2: f32,
            y2: f32,
            radius: f32,
        );
        fn canvas_native_path_ellipse(
            path: &mut Path,
            x: f32,
            y: f32,
            radius_x: f32,
            radius_y: f32,
            rotation: f32,
            start_angle: f32,
            end_angle: f32,
            anticlockwise: bool,
        );

        fn canvas_native_path_rect(path: &mut Path, x: f32, y: f32, width: f32, height: f32);

        fn canvas_native_path_to_string(path: &Path) -> String;
        /* Path2D */

        /* DOMMatrix */
        fn canvas_native_matrix_create() -> Box<Matrix>;

        fn canvas_native_matrix_update(matrix: &mut Matrix, slice: &[f32]);

        fn canvas_native_matrix_get_a(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_a(matrix: &mut Matrix, a: f32);

        fn canvas_native_matrix_get_b(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_b(matrix: &mut Matrix, b: f32);

        fn canvas_native_matrix_get_c(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_c(matrix: &mut Matrix, c: f32);

        fn canvas_native_matrix_get_d(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_d(matrix: &mut Matrix, d: f32);

        fn canvas_native_matrix_get_e(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_e(matrix: &mut Matrix, e: f32);

        fn canvas_native_matrix_get_f(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_f(matrix: &mut Matrix, f: f32);

        fn canvas_native_matrix_get_m11(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m11(matrix: &mut Matrix, m11: f32);

        fn canvas_native_matrix_get_m12(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m12(matrix: &mut Matrix, m12: f32);

        fn canvas_native_matrix_get_m13(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m13(matrix: &mut Matrix, m13: f32);

        fn canvas_native_matrix_get_m14(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m14(matrix: &mut Matrix, m14: f32);

        fn canvas_native_matrix_get_m21(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m21(matrix: &mut Matrix, m21: f32);

        fn canvas_native_matrix_get_m22(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m22(matrix: &mut Matrix, m22: f32);

        fn canvas_native_matrix_get_m23(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m23(matrix: &mut Matrix, m23: f32);

        fn canvas_native_matrix_get_m24(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m24(matrix: &mut Matrix, m24: f32);

        fn canvas_native_matrix_get_m31(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m31(matrix: &mut Matrix, m31: f32);

        fn canvas_native_matrix_get_m32(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m32(matrix: &mut Matrix, m32: f32);

        fn canvas_native_matrix_get_m33(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m33(matrix: &mut Matrix, m33: f32);

        fn canvas_native_matrix_get_m34(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m34(matrix: &mut Matrix, m34: f32);

        fn canvas_native_matrix_get_m41(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m41(matrix: &mut Matrix, m41: f32);

        fn canvas_native_matrix_get_m42(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m42(matrix: &mut Matrix, m42: f32);

        fn canvas_native_matrix_get_m43(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m43(matrix: &mut Matrix, m43: f32);

        fn canvas_native_matrix_get_m44(matrix: &Matrix) -> f32;

        fn canvas_native_matrix_set_m44(matrix: &mut Matrix, m44: f32);
        /* DOMMatrix */

        /* ImageData */
        fn canvas_native_image_data_create(width: i32, height: i32) -> Box<ImageData>;
        fn canvas_native_image_data_get_width(image_data: &ImageData) -> i32;
        fn canvas_native_image_data_get_height(image_data: &ImageData) -> i32;
        fn canvas_native_image_data_get_data(image_data: &mut ImageData) -> &mut [u8];
        fn canvas_native_image_data_get_shared_instance(
            image_data: &mut ImageData,
        ) -> Box<ImageData>;
        /* ImageData */

        /* ImageAsset */
        fn canvas_native_image_asset_create() -> Box<ImageAsset>;

        fn canvas_native_image_asset_load_from_path(asset: &mut ImageAsset, path: String) -> bool;

        fn canvas_native_image_asset_load_from_path_async(
            asset: &mut ImageAsset,
            path: String,
            callback: isize,
        );

        fn canvas_native_image_asset_load_from_url(asset: &mut ImageAsset, url: String) -> bool;

        fn canvas_native_image_asset_load_from_url_async(
            asset: &mut ImageAsset,
            url: String,
            callback: isize,
        );

        fn canvas_native_image_asset_load_from_raw(asset: &mut ImageAsset, array: &[u8]) -> bool;

        fn canvas_native_image_asset_load_from_raw_async(
            asset: &mut ImageAsset,
            array: &[u8],
            callback: isize,
        );

        fn canvas_native_image_asset_get_bytes(asset: &mut ImageAsset) -> Vec<u8>;

        fn canvas_native_image_asset_get_rgba_bytes(asset: &mut ImageAsset) -> Vec<u8>;

        fn canvas_native_image_asset_get_rgb_bytes(asset: &mut ImageAsset) -> Vec<u8>;

        fn canvas_native_image_asset_width(asset: &mut ImageAsset) -> u32;

        fn canvas_native_image_asset_height(asset: &mut ImageAsset) -> u32;

        fn canvas_native_image_asset_get_error(asset: &mut ImageAsset) -> String;

        fn canvas_native_image_asset_has_error(asset: &mut ImageAsset) -> bool;

        fn canvas_native_image_asset_scale(asset: &mut ImageAsset, x: u32, y: u32) -> bool;

        fn canvas_native_image_asset_flip_x(asset: &mut ImageAsset) -> bool;

        fn canvas_native_image_asset_flip_x_in_place(asset: &mut ImageAsset) -> bool;

        fn canvas_native_image_asset_flip_y(asset: &mut ImageAsset) -> bool;

        fn canvas_native_image_asset_flip_y_in_place_owned(buf: &mut [u8]);

        fn canvas_native_image_asset_flip_x_in_place_owned(buf: &mut [u8]);

        fn canvas_native_image_asset_flip_y_in_place(asset: &mut ImageAsset) -> bool;

        fn canvas_native_image_asset_save_path(
            asset: &mut ImageAsset,
            path: &str,
            format: u32,
        ) -> bool;

        fn canvas_native_image_asset_save_path_async(
            asset: &mut ImageAsset,
            path: &str,
            format: u32,
            callback: isize,
        );
        /* ImageAsset */

        /* TextMetrics */
        fn canvas_native_text_metrics_get_width(metrics: &TextMetrics) -> f32;

        fn canvas_native_text_metrics_get_actual_bounding_box_left(metrics: &TextMetrics) -> f32;

        fn canvas_native_text_metrics_get_actual_bounding_box_right(metrics: &TextMetrics) -> f32;

        fn canvas_native_text_metrics_get_actual_bounding_box_ascent(metrics: &TextMetrics) -> f32;

        fn canvas_native_text_metrics_get_actual_bounding_box_descent(metrics: &TextMetrics)
            -> f32;

        fn canvas_native_text_metrics_get_font_bounding_box_ascent(metrics: &TextMetrics) -> f32;

        fn canvas_native_text_metrics_get_font_bounding_box_descent(metrics: &TextMetrics) -> f32;

        fn canvas_native_text_metrics_get_em_height_ascent(metrics: &TextMetrics) -> f32;

        fn canvas_native_text_metrics_get_em_height_descent(metrics: &TextMetrics) -> f32;

        fn canvas_native_text_metrics_get_hanging_baseline(metrics: &TextMetrics) -> f32;

        fn canvas_native_text_metrics_get_alphabetic_baseline(metrics: &TextMetrics) -> f32;

        fn canvas_native_text_metrics_get_ideographic_baseline(metrics: &TextMetrics) -> f32;
        /* TextMetrics */

        /* CanvasGradient */
        fn canvas_native_gradient_add_color_stop(style: &mut PaintStyle, stop: f32, color: &str);

        /* CanvasGradient */

        /* CanvasPattern */
        fn canvas_native_pattern_set_transform(pattern: &mut PaintStyle, matrix: &Matrix);
        /* CanvasPattern */

        /* TextDecoder */
        fn canvas_native_text_decoder_create(decoding: &str) -> Box<TextDecoder>;
        fn canvas_native_text_decoder_get_encoding(decoder: &mut TextDecoder) -> String;
        fn canvas_native_text_decoder_decode(decoder: &mut TextDecoder, data: &[u8]) -> String;
        /* TextDecoder */

        /* TextEncoder */
        fn canvas_native_text_encoder_create(encoding: &str) -> Box<TextEncoder>;
        fn canvas_native_text_encoder_get_encoding(decoder: &mut TextEncoder) -> String;
        fn canvas_native_text_encoder_encode(encoder: &mut TextEncoder, text: &str) -> Vec<u8>;
        /* TextEncoder */

        /* Raf */
        fn canvas_native_raf_create(callback: isize)-> Box<Raf>;
        fn canvas_native_raf_start(raf: &mut Raf);
        fn canvas_native_raf_stop(raf: &mut Raf);
        fn canvas_native_raf_get_started(raf: &Raf) -> bool;
        /* Raf */

        /* GL */
        fn canvas_native_context_gl_make_current(context: &CanvasRenderingContext2D)-> bool;
        fn canvas_native_context_gl_swap_buffers(context: &CanvasRenderingContext2D)->bool;
        /* GL */

        /* CanvasRenderingContext2D */
        fn canvas_native_context_create_with_wrapper(
            context: isize,
        ) -> Box<CanvasRenderingContext2D>;

        fn canvas_native_context_create(
            width: f32,
            height: f32,
            density: f32,
            alpha: bool,
            font_color: i32,
            ppi: f32,
            direction: u32,
        ) -> Box<CanvasRenderingContext2D>;

        pub fn canvas_native_context_create_gl(
            width: f32,
            height: f32,
            density: f32,
            buffer_id: i32,
            samples: i32,
            alpha: bool,
            font_color: i32,
            ppi: f32,
            direction: u32,
        ) -> Box<CanvasRenderingContext2D>;

        fn canvas_native_context_flush(context: &CanvasRenderingContext2D);

        fn canvas_native_context_get_global_alpha(context: &CanvasRenderingContext2D) -> f32;
        fn canvas_native_context_set_global_alpha(
            context: &mut CanvasRenderingContext2D,
            alpha: f32,
        );

        fn canvas_native_context_get_image_smoothing_enabled(
            context: &CanvasRenderingContext2D,
        ) -> bool;

        fn canvas_native_context_set_image_smoothing_enabled(
            context: &mut CanvasRenderingContext2D,
            enabled: bool,
        );

        fn canvas_native_context_get_image_smoothing_quality(
            context: &CanvasRenderingContext2D,
        ) -> String;

        fn canvas_native_context_set_image_smoothing_quality(
            context: &mut CanvasRenderingContext2D,
            quality: &str,
        );

        fn canvas_native_context_get_line_join(context: &CanvasRenderingContext2D) -> String;

        fn canvas_native_context_set_line_join(context: &mut CanvasRenderingContext2D, join: &str);

        fn canvas_native_context_get_line_cap(context: &CanvasRenderingContext2D) -> String;

        fn canvas_native_context_set_line_cap(context: &mut CanvasRenderingContext2D, cap: &str);

        fn canvas_native_context_get_miter_limit(context: &CanvasRenderingContext2D) -> f32;

        fn canvas_native_context_set_miter_limit(
            context: &mut CanvasRenderingContext2D,
            limit: f32,
        );

        fn canvas_native_context_get_shadow_color(context: &CanvasRenderingContext2D) -> String;

        fn canvas_native_context_set_shadow_color(
            context: &mut CanvasRenderingContext2D,
            color: &str,
        );

        fn canvas_native_context_get_shadow_blur(context: &CanvasRenderingContext2D) -> f32;

        fn canvas_native_context_set_shadow_blur(context: &mut CanvasRenderingContext2D, blur: f32);

        fn canvas_native_context_get_shadow_offset_x(context: &CanvasRenderingContext2D) -> f32;

        fn canvas_native_context_set_shadow_offset_x(
            context: &mut CanvasRenderingContext2D,
            x: f32,
        );

        fn canvas_native_context_get_shadow_offset_y(context: &CanvasRenderingContext2D) -> f32;

        fn canvas_native_context_set_shadow_offset_y(
            context: &mut CanvasRenderingContext2D,
            y: f32,
        );

        fn canvas_native_context_get_text_align(context: &CanvasRenderingContext2D) -> String;

        fn canvas_native_context_set_text_align(
            context: &mut CanvasRenderingContext2D,
            alignment: &str,
        );

        fn canvas_native_context_get_global_composition(
            context: &CanvasRenderingContext2D,
        ) -> String;

        fn canvas_native_context_set_global_composition(
            context: &CanvasRenderingContext2D,
            composition: &str,
        );

        fn canvas_native_paint_style_set_fill_color_with_string(
            context: &mut CanvasRenderingContext2D,
            color: &str,
        );

        fn canvas_native_paint_style_set_stroke_color_with_string(
            context: &mut CanvasRenderingContext2D,
            color: &str,
        );

        fn canvas_native_paint_style_get_color_string(color: &mut PaintStyle) -> String;

        fn canvas_native_context_get_style_type(style: &PaintStyle) -> PaintStyleType;

        fn canvas_native_context_get_fill_style(
            context: &mut CanvasRenderingContext2D,
        ) -> Box<PaintStyle>;

        fn canvas_native_context_set_fill_style(
            context: &mut CanvasRenderingContext2D,
            style: &PaintStyle,
        );

        fn canvas_native_context_get_stroke_style(
            context: &CanvasRenderingContext2D,
        ) -> Box<PaintStyle>;

        fn canvas_native_context_set_stroke_style(
            context: &mut CanvasRenderingContext2D,
            style: &PaintStyle,
        );

        fn canvas_native_context_get_line_width(context: &CanvasRenderingContext2D) -> f32;

        fn canvas_native_context_set_line_width(context: &mut CanvasRenderingContext2D, width: f32);

        fn canvas_native_context_get_line_dash_offset(context: &CanvasRenderingContext2D) -> f32;

        fn canvas_native_context_set_line_dash_offset(
            context: &mut CanvasRenderingContext2D,
            offset: f32,
        );

        fn canvas_native_context_get_line_dash(context: &CanvasRenderingContext2D) -> Vec<f32>;

        fn canvas_native_context_set_line_dash(
            context: &mut CanvasRenderingContext2D,
            dash: &[f32],
        );

        fn canvas_native_context_arc(
            context: &mut CanvasRenderingContext2D,
            x: f32,
            y: f32,
            radius: f32,
            start_angle: f32,
            end_angle: f32,
            anticlockwise: bool,
        );

        fn canvas_native_context_arc_to(
            context: &mut CanvasRenderingContext2D,
            x1: f32,
            y1: f32,
            x2: f32,
            y2: f32,
            radius: f32,
        );

        fn canvas_native_context_begin_path(context: &mut CanvasRenderingContext2D);

        fn canvas_native_context_bezier_curve_to(
            context: &mut CanvasRenderingContext2D,
            cp1x: f32,
            cp1y: f32,
            cp2x: f32,
            cp2y: f32,
            x: f32,
            y: f32,
        );

        fn canvas_native_context_clear_rect(
            context: &mut CanvasRenderingContext2D,
            x: f32,
            y: f32,
            width: f32,
            height: f32,
        );

        fn canvas_native_context_clip(
            context: &mut CanvasRenderingContext2D,
            path: &mut Path,
            rule: &str,
        );

        fn canvas_native_context_clip_rule(context: &mut CanvasRenderingContext2D, rule: &str);

        fn canvas_native_context_close_path(context: &mut CanvasRenderingContext2D);

        fn canvas_native_context_create_image_data(width: i32, height: i32) -> Box<ImageData>;

        fn canvas_native_context_create_linear_gradient(
            context: &mut CanvasRenderingContext2D,
            x0: f32,
            y0: f32,
            x1: f32,
            y1: f32,
        ) -> Box<PaintStyle>;

        fn canvas_native_context_create_pattern(
            context: &mut CanvasRenderingContext2D,
            data: &[u8],
            width: i32,
            height: i32,
            repetition: &str,
        ) -> Box<PaintStyle>;

        fn canvas_native_context_create_pattern_asset(
            context: &mut CanvasRenderingContext2D,
            asset: &ImageAsset,
            repetition: &str,
        ) -> Box<PaintStyle>;

        fn canvas_native_context_create_pattern_encoded(
            context: &mut CanvasRenderingContext2D,
            data: &[u8],
            repetition: &str,
        ) -> Box<PaintStyle>;

        fn canvas_native_context_create_radial_gradient(
            context: &mut CanvasRenderingContext2D,
            x0: f32,
            y0: f32,
            r0: f32,
            x1: f32,
            y1: f32,
            r1: f32,
        ) -> Box<PaintStyle>;

        fn canvas_native_context_draw_image_dx_dy(
            context: &mut CanvasRenderingContext2D,
            data: &[u8],
            width: f32,
            height: f32,
            dx: f32,
            dy: f32,
        );

        fn canvas_native_context_draw_image_dx_dy_dw_dh(
            context: &mut CanvasRenderingContext2D,
            data: &[u8],
            width: f32,
            height: f32,
            dx: f32,
            dy: f32,
            d_width: f32,
            d_height: f32,
        );

        fn canvas_native_context_draw_image(
            context: &mut CanvasRenderingContext2D,
            data: &[u8],
            width: f32,
            height: f32,
            sx: f32,
            sy: f32,
            s_width: f32,
            s_height: f32,
            dx: f32,
            dy: f32,
            d_width: f32,
            d_height: f32,
        );

        fn canvas_native_context_draw_image_encoded_dx_dy(
            context: &mut CanvasRenderingContext2D,
            data: &[u8],
            dx: f32,
            dy: f32,
        );

        fn canvas_native_context_draw_image_encoded_dx_dy_dw_dh(
            context: &mut CanvasRenderingContext2D,
            data: &[u8],
            dx: f32,
            dy: f32,
            d_width: f32,
            d_height: f32,
        );

        fn canvas_native_context_draw_image_encoded(
            context: &mut CanvasRenderingContext2D,
            data: &[u8],
            sx: f32,
            sy: f32,
            s_width: f32,
            s_height: f32,
            dx: f32,
            dy: f32,
            d_width: f32,
            d_height: f32,
        );

        fn canvas_native_context_draw_image_dx_dy_asset(
            context: &mut CanvasRenderingContext2D,
            asset: &mut ImageAsset,
            dx: f32,
            dy: f32,
        );

        fn canvas_native_context_draw_image_dx_dy_dw_dh_asset(
            context: &mut CanvasRenderingContext2D,
            asset: &mut ImageAsset,
            dx: f32,
            dy: f32,
            d_width: f32,
            d_height: f32,
        );

        fn canvas_native_context_draw_image_asset(
            context: &mut CanvasRenderingContext2D,
            asset: &mut ImageAsset,
            sx: f32,
            sy: f32,
            s_width: f32,
            s_height: f32,
            dx: f32,
            dy: f32,
            d_width: f32,
            d_height: f32,
        );

        fn canvas_native_context_ellipse(
            context: &mut CanvasRenderingContext2D,
            x: f32,
            y: f32,
            radius_x: f32,
            radius_y: f32,
            rotation: f32,
            start_angle: f32,
            end_angle: f32,
            anticlockwise: bool,
        );

        fn canvas_native_context_fill(context: &mut CanvasRenderingContext2D, rule: &str);

        fn canvas_native_context_fill_with_path(
            context: &mut CanvasRenderingContext2D,
            path: &mut Path,
            rule: &str,
        );

        fn canvas_native_context_fill_rect(
            context: &mut CanvasRenderingContext2D,
            x: f32,
            y: f32,
            width: f32,
            height: f32,
        );

        fn canvas_native_context_fill_text(
            context: &mut CanvasRenderingContext2D,
            text: &str,
            x: f32,
            y: f32,
            width: f32,
        );

        fn canvas_native_context_get_image_data(
            context: &mut CanvasRenderingContext2D,
            sx: f32,
            sy: f32,
            sw: f32,
            sh: f32,
        ) -> Box<ImageData>;

        fn canvas_native_context_get_transform(
            context: &mut CanvasRenderingContext2D,
        ) -> Box<Matrix>;

        fn canvas_native_context_is_point_in_path(
            context: &mut CanvasRenderingContext2D,
            x: f32,
            y: f32,
            rule: &str,
        ) -> bool;

        fn canvas_native_context_is_point_in_path_with_path(
            context: &mut CanvasRenderingContext2D,
            path: &mut Path,
            x: f32,
            y: f32,
            rule: &str,
        ) -> bool;

        fn canvas_native_context_is_point_in_stroke(
            context: &mut CanvasRenderingContext2D,
            x: f32,
            y: f32,
        ) -> bool;

        fn canvas_native_context_is_point_in_stroke_with_path(
            context: &mut CanvasRenderingContext2D,
            path: &mut Path,
            x: f32,
            y: f32,
        ) -> bool;

        fn canvas_native_context_line_to(context: &mut CanvasRenderingContext2D, x: f32, y: f32);

        fn canvas_native_context_measure_text(
            context: &mut CanvasRenderingContext2D,
            text: &str,
        ) -> Box<TextMetrics>;

        fn canvas_native_context_move_to(context: &mut CanvasRenderingContext2D, x: f32, y: f32);

        fn canvas_native_context_put_image_data(
            context: &mut CanvasRenderingContext2D,
            image_data: &mut ImageData,
            dx: f32,
            dy: f32,
            dirty_x: f32,
            dirty_y: f32,
            dirty_width: f32,
            dirty_height: f32,
        );

        fn canvas_native_context_quadratic_curve_to(
            context: &mut CanvasRenderingContext2D,
            cpx: f32,
            cpy: f32,
            x: f32,
            y: f32,
        );

        fn canvas_native_context_rect(
            context: &mut CanvasRenderingContext2D,
            x: f32,
            y: f32,
            width: f32,
            height: f32,
        );

        fn canvas_native_context_reset_transform(context: &mut CanvasRenderingContext2D);

        fn canvas_native_context_restore(context: &mut CanvasRenderingContext2D);

        fn canvas_native_context_rotate(context: &mut CanvasRenderingContext2D, angle: f32);

        fn canvas_native_context_save(context: &mut CanvasRenderingContext2D);

        fn canvas_native_context_scale(context: &mut CanvasRenderingContext2D, x: f32, y: f32);

        fn canvas_native_context_set_transform(
            context: &mut CanvasRenderingContext2D,
            a: f32,
            b: f32,
            c: f32,
            d: f32,
            e: f32,
            f: f32,
        );

        fn canvas_native_context_set_transform_matrix(
            context: &mut CanvasRenderingContext2D,
            matrix: &mut Matrix,
        );
        fn canvas_native_context_stroke(context: &mut CanvasRenderingContext2D);

        fn canvas_native_context_stroke_with_path(
            context: &mut CanvasRenderingContext2D,
            path: &mut Path,
        );

        fn canvas_native_context_stroke_rect(
            context: &mut CanvasRenderingContext2D,
            x: f32,
            y: f32,
            width: f32,
            height: f32,
        );

        fn canvas_native_context_stroke_text(
            context: &mut CanvasRenderingContext2D,
            text: &str,
            x: f32,
            y: f32,
            width: f32,
        );

        fn canvas_native_context_transform(
            context: &mut CanvasRenderingContext2D,
            a: f32,
            b: f32,
            c: f32,
            d: f32,
            e: f32,
            f: f32,
        );

        fn canvas_native_context_translate(context: &mut CanvasRenderingContext2D, x: f32, y: f32);

        /* CanvasRenderingContext2D */
    }
}

fn canvas_native_context_create_with_wrapper(context: isize) -> Box<CanvasRenderingContext2D> {
    console_log(format!("context ptr {:?}", context).as_str());
    let mut wrapper = unsafe { context as *mut ContextWrapper };
    let mut wrapper = unsafe { &mut *wrapper };
    console_log("wrapper unboxed");
    let ctx = GLContext::get_current();
    console_log("current gl context");
    let clone = wrapper.clone();
    console_log("after clone");
    let b = Box::new(CanvasRenderingContext2D {
        context: clone,
        gl_context: ctx,
    });

    console_log("context wrapped");
    b
}

pub fn canvas_native_context_create(
    width: f32,
    height: f32,
    density: f32,
    alpha: bool,
    font_color: i32,
    ppi: f32,
    direction: u32,
) -> Box<CanvasRenderingContext2D> {
    Box::new(CanvasRenderingContext2D {
        context: ContextWrapper::new(Context::new(
            width,
            height,
            density,
            alpha,
            font_color,
            ppi,
            TextDirection::from(direction),
        )),
        gl_context: GLContext::default(),
    })
}

pub fn canvas_native_context_create_gl(
    width: f32,
    height: f32,
    density: f32,
    buffer_id: i32,
    samples: i32,
    alpha: bool,
    font_color: i32,
    ppi: f32,
    direction: u32,
) -> Box<CanvasRenderingContext2D> {
    let ctx = GLContext::get_current();
    Box::new(CanvasRenderingContext2D {
        context: ContextWrapper::new(Context::new_gl(
            width,
            height,
            density,
            buffer_id,
            samples,
            alpha,
            font_color,
            ppi,
            TextDirection::from(direction),
        )),
        gl_context: ctx,
    })
}

pub fn canvas_native_context_get_global_alpha(context: &CanvasRenderingContext2D) -> f32 {
    context.context.get_context().global_alpha()
}

pub fn canvas_native_context_set_global_alpha(context: &mut CanvasRenderingContext2D, alpha: f32) {
    context.context.get_context().set_global_alpha(alpha);
}

pub fn canvas_native_context_get_image_smoothing_enabled(
    context: &CanvasRenderingContext2D,
) -> bool {
    context.context.get_context().get_image_smoothing_enabled()
}

pub fn canvas_native_context_set_image_smoothing_enabled(
    context: &mut CanvasRenderingContext2D,
    enabled: bool,
) {
    context
        .context
        .get_context()
        .set_image_smoothing_enabled(enabled)
}

pub fn canvas_native_context_get_image_smoothing_quality(
    context: &CanvasRenderingContext2D,
) -> String {
    let ret = match context.context.get_context().get_image_smoothing_quality() {
        ImageSmoothingQuality::Low => "low",
        ImageSmoothingQuality::Medium => "medium",
        ImageSmoothingQuality::High => "high",
    };

    ret.to_string()
}

pub fn canvas_native_context_set_image_smoothing_quality(
    context: &mut CanvasRenderingContext2D,
    quality: &str,
) {
    match quality {
        "low" => context
            .context
            .get_context()
            .set_image_smoothing_quality(ImageSmoothingQuality::Low),
        "medium" => context
            .context
            .get_context()
            .set_image_smoothing_quality(ImageSmoothingQuality::Medium),
        "high" => context
            .context
            .get_context()
            .set_image_smoothing_quality(ImageSmoothingQuality::High),
        _ => {}
    }
}

pub fn canvas_native_context_get_line_join(context: &CanvasRenderingContext2D) -> String {
    let ret = match context.context.get_context().line_join() {
        LineJoin::JoinBevel => "bevel",
        LineJoin::JoinMiter => "miter",
        LineJoin::JoinRound => "round",
    };

    ret.to_string()
}

pub fn canvas_native_context_set_line_join(context: &mut CanvasRenderingContext2D, join: &str) {
    match join {
        "bevel" => context
            .context
            .get_context()
            .set_line_join(LineJoin::JoinBevel),
        "miter" => context
            .context
            .get_context()
            .set_line_join(LineJoin::JoinMiter),
        "round" => context
            .context
            .get_context()
            .set_line_join(LineJoin::JoinRound),
        _ => {}
    }
}

pub fn canvas_native_context_get_line_cap(context: &CanvasRenderingContext2D) -> String {
    let ret = match context.context.get_context().line_cap() {
        LineCap::CapRound => "round",
        LineCap::CapButt => "butt",
        LineCap::CapSquare => "square",
    };

    ret.to_string()
}

pub fn canvas_native_context_set_line_cap(context: &mut CanvasRenderingContext2D, cap: &str) {
    match cap {
        "round" => context
            .context
            .get_context()
            .set_line_cap(LineCap::CapRound),
        "butt" => context.context.get_context().set_line_cap(LineCap::CapButt),
        "square" => context
            .context
            .get_context()
            .set_line_cap(LineCap::CapSquare),
        _ => {}
    }
}

pub fn canvas_native_context_get_miter_limit(context: &CanvasRenderingContext2D) -> f32 {
    context.context.get_context().miter_limit()
}

pub fn canvas_native_context_set_miter_limit(context: &mut CanvasRenderingContext2D, limit: f32) {
    context.context.get_context().set_miter_limit(limit);
}

pub fn canvas_native_context_get_shadow_color(context: &CanvasRenderingContext2D) -> String {
    let color = context.context.get_context().shadow_color();
    to_parsed_color(color)
}

pub fn canvas_native_context_set_shadow_color(context: &mut CanvasRenderingContext2D, color: &str) {
    if let Some(color) = parse_color(color) {
        context.context.get_context().set_shadow_color(color);
    }
}

pub fn canvas_native_context_get_shadow_blur(context: &CanvasRenderingContext2D) -> f32 {
    context.context.get_context().shadow_blur()
}

pub fn canvas_native_context_set_shadow_blur(context: &mut CanvasRenderingContext2D, blur: f32) {
    context.context.get_context().set_shadow_blur(blur)
}

pub fn canvas_native_context_get_shadow_offset_x(context: &CanvasRenderingContext2D) -> f32 {
    context.context.get_context().shadow_offset_x()
}

pub fn canvas_native_context_set_shadow_offset_x(context: &mut CanvasRenderingContext2D, x: f32) {
    context.context.get_context().set_shadow_offset_x(x)
}

pub fn canvas_native_context_get_shadow_offset_y(context: &CanvasRenderingContext2D) -> f32 {
    context.context.get_context().shadow_offset_y()
}

pub fn canvas_native_context_set_shadow_offset_y(context: &mut CanvasRenderingContext2D, y: f32) {
    context.context.get_context().set_shadow_offset_y(y)
}

pub fn canvas_native_context_get_text_align(context: &CanvasRenderingContext2D) -> String {
    match context.context.get_context().text_align() {
        TextAlign::START => "start",
        TextAlign::LEFT => "left",
        TextAlign::CENTER => "center",
        TextAlign::RIGHT => "right",
        TextAlign::END => "end",
    }
    .to_string()
}

pub fn canvas_native_context_set_text_align(
    context: &mut CanvasRenderingContext2D,
    alignment: &str,
) {
    match alignment {
        "start" => context
            .context
            .get_context()
            .set_text_align(TextAlign::START),
        "left" => context
            .context
            .get_context()
            .set_text_align(TextAlign::LEFT),
        "center" => context
            .context
            .get_context()
            .set_text_align(TextAlign::CENTER),
        "right" => context
            .context
            .get_context()
            .set_text_align(TextAlign::RIGHT),
        "end" => context.context.get_context().set_text_align(TextAlign::END),
        _ => {}
    }
}

pub fn canvas_native_context_get_global_composition(context: &CanvasRenderingContext2D) -> String {
    context
        .context
        .get_context()
        .global_composite_operation()
        .to_str()
        .to_string()
}

pub fn canvas_native_context_set_global_composition(
    context: &CanvasRenderingContext2D,
    composition: &str,
) {
    if let Some(composition) = CompositeOperationType::from_str(composition) {
        context
            .context
            .get_context()
            .set_global_composite_operation(composition)
    }
}

pub fn canvas_native_paint_style_set_fill_color_with_string(
    context: &mut CanvasRenderingContext2D,
    color: &str,
) {
    paint_style_set_color_with_string(&mut context.context, true, color);
}

pub fn canvas_native_paint_style_set_stroke_color_with_string(
    context: &mut CanvasRenderingContext2D,
    color: &str,
) {
    paint_style_set_color_with_string(&mut context.context, false, color);
}

pub fn canvas_native_paint_style_get_color_string(color: &mut PaintStyle) -> String {
    if let Some(color) = color.0.as_ref() {
        return match color {
            canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Color(color) => {
                to_parsed_color(*color)
            }
            _ => String::new(),
        };
    }
    String::new()
}

pub fn canvas_native_context_get_style_type(style: &PaintStyle) -> ffi::PaintStyleType {
    style.style_type()
}

pub fn canvas_native_context_get_fill_style(context: &CanvasRenderingContext2D) -> Box<PaintStyle> {
    Box::new(PaintStyle(Some(context.get_context().fill_style().clone())))
}

pub fn canvas_native_context_set_fill_style(
    context: &mut CanvasRenderingContext2D,
    style: &PaintStyle,
) {
    if !style.is_empty() {
        context
            .get_context()
            .set_fill_style(style.0.as_ref().unwrap().clone())
    }
}

pub fn canvas_native_context_get_stroke_style(
    context: &CanvasRenderingContext2D,
) -> Box<PaintStyle> {
    Box::new(PaintStyle(Some(
        context.get_context().stroke_style().clone(),
    )))
}

pub fn canvas_native_context_set_stroke_style(
    context: &mut CanvasRenderingContext2D,
    style: &PaintStyle,
) {
    if !style.is_empty() {
        context
            .get_context()
            .set_stroke_style(style.0.as_ref().unwrap().clone())
    }
}

pub fn canvas_native_context_get_line_width(context: &CanvasRenderingContext2D) -> f32 {
    context.get_context().line_width()
}

pub fn canvas_native_context_set_line_width(context: &mut CanvasRenderingContext2D, width: f32) {
    context.get_context().set_line_width(width);
}

pub fn canvas_native_context_get_line_dash_offset(context: &CanvasRenderingContext2D) -> f32 {
    context.get_context().line_dash_offset()
}

pub fn canvas_native_context_set_line_dash_offset(
    context: &mut CanvasRenderingContext2D,
    offset: f32,
) {
    context.get_context().set_line_dash_offset(offset)
}

pub fn canvas_native_context_get_line_dash(context: &CanvasRenderingContext2D) -> Vec<f32> {
    context.get_context().line_dash().to_vec()
}

pub fn canvas_native_context_set_line_dash(context: &mut CanvasRenderingContext2D, dash: &[f32]) {
    context.get_context().set_line_dash(dash)
}

pub fn canvas_native_context_arc(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    anticlockwise: bool,
) {
    context
        .get_context()
        .arc(x, y, radius, start_angle, end_angle, anticlockwise)
}

pub fn canvas_native_context_arc_to(
    context: &mut CanvasRenderingContext2D,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    radius: f32,
) {
    context.get_context().arc_to(x1, y1, x2, y2, radius)
}

pub fn canvas_native_context_begin_path(context: &mut CanvasRenderingContext2D) {
    context.get_context().begin_path()
}

pub fn canvas_native_context_bezier_curve_to(
    context: &mut CanvasRenderingContext2D,
    cp1x: f32,
    cp1y: f32,
    cp2x: f32,
    cp2y: f32,
    x: f32,
    y: f32,
) {
    context
        .get_context()
        .bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y)
}

pub fn canvas_native_context_clear_rect(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    context.get_context().clear_rect(x, y, width, height)
}

pub fn canvas_native_context_clip(
    context: &mut CanvasRenderingContext2D,
    path: &mut Path,
    rule: &str,
) {
    if let Ok(rule) = FillRule::try_from(rule) {
        context
            .get_context()
            .clip(Some(path.inner_mut()), Some(rule))
    }
}

pub fn canvas_native_context_clip_rule(context: &mut CanvasRenderingContext2D, rule: &str) {
    if let Ok(rule) = FillRule::try_from(rule) {
        context.get_context().clip(None, Some(rule))
    }
}

pub fn canvas_native_context_close_path(context: &mut CanvasRenderingContext2D) {
    context.get_context().close_path()
}

pub fn canvas_native_context_create_image_data(width: i32, height: i32) -> Box<ImageData> {
    Box::new(ImageData::new(Context::create_image_data(width, height)))
}

pub fn canvas_native_context_create_linear_gradient(
    context: &mut CanvasRenderingContext2D,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
) -> Box<PaintStyle> {
    Box::new(PaintStyle(Some(
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(
            context.get_context().create_linear_gradient(x0, y0, x1, y1),
        ),
    )))
}

pub fn canvas_native_context_create_pattern(
    context: &mut CanvasRenderingContext2D,
    data: &[u8],
    width: i32,
    height: i32,
    repetition: &str,
) -> Box<PaintStyle> {
    Box::new(PaintStyle(Repetition::try_from(repetition).map_or(
        None,
        |repetition| {
            from_image_slice(data, width, height).map(|image| {
                canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                    context.get_context().create_pattern(image, repetition),
                )
            })
        },
    )))
}

pub fn canvas_native_context_create_pattern_asset(
    context: &mut CanvasRenderingContext2D,
    asset: &ImageAsset,
    repetition: &str,
) -> Box<PaintStyle> {
    let bytes = asset.lock().rgba_internal_bytes();
    Box::new(PaintStyle(Repetition::try_from(repetition).map_or(
        None,
        |repetition| {
            from_image_slice(
                bytes.as_slice(),
                asset.width() as i32,
                asset.height() as i32,
            )
            .map(|image| {
                canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                    context.get_context().create_pattern(image, repetition),
                )
            })
        },
    )))
}

pub fn canvas_native_context_create_pattern_encoded(
    context: &mut CanvasRenderingContext2D,
    data: &[u8],
    repetition: &str,
) -> Box<PaintStyle> {
    Box::new(PaintStyle(Repetition::try_from(repetition).map_or(
        None,
        |repetition| {
            from_image_slice_encoded(data).map(|image| {
                canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                    context.get_context().create_pattern(image, repetition),
                )
            })
        },
    )))
}

pub fn canvas_native_context_create_radial_gradient(
    context: &mut CanvasRenderingContext2D,
    x0: f32,
    y0: f32,
    r0: f32,
    x1: f32,
    y1: f32,
    r1: f32,
) -> Box<PaintStyle> {
    Box::new(PaintStyle(Some(
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(
            context
                .get_context()
                .create_radial_gradient(x0, y0, r0, x1, y1, r1),
        ),
    )))
}

pub fn canvas_native_context_draw_image_dx_dy(
    context: &mut CanvasRenderingContext2D,
    data: &[u8],
    width: f32,
    height: f32,
    dx: f32,
    dy: f32,
) {
    canvas_native_context_draw_image(
        context, data, width, height, 0.0, 0.0, width, height, dx, dy, width, height,
    );
}

pub fn canvas_native_context_draw_image_dx_dy_dw_dh(
    context: &mut CanvasRenderingContext2D,
    data: &[u8],
    width: f32,
    height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    canvas_native_context_draw_image(
        context, data, width, height, 0.0, 0.0, width, height, dx, dy, d_width, d_height,
    )
}

pub fn canvas_native_context_draw_image(
    context: &mut CanvasRenderingContext2D,
    data: &[u8],
    width: f32,
    height: f32,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    if let Some(image) = from_image_slice(data, width as i32, height as i32) {
        context.get_context().draw_image_src_xywh_dst_xywh(
            &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
        )
    }
}

pub fn canvas_native_context_draw_image_encoded_dx_dy(
    context: &mut CanvasRenderingContext2D,
    data: &[u8],
    dx: f32,
    dy: f32,
) {
    if let Some(image) = from_image_slice_encoded(data) {
        let width = image.width() as f32;
        let height = image.height() as f32;
        context
            .get_context()
            .draw_image_src_xywh_dst_xywh(&image, 0.0, 0.0, width, height, dx, dy, width, height)
    }
}

pub fn canvas_native_context_draw_image_encoded_dx_dy_dw_dh(
    context: &mut CanvasRenderingContext2D,
    data: &[u8],
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    if let Some(image) = from_image_slice_encoded(data) {
        let width = image.width() as f32;
        let height = image.height() as f32;
        context.get_context().draw_image_src_xywh_dst_xywh(
            &image, 0.0, 0.0, width, height, dx, dy, d_width, d_height,
        )
    }
}

pub fn canvas_native_context_draw_image_encoded(
    context: &mut CanvasRenderingContext2D,
    data: &[u8],
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    if let Some(image) = from_image_slice_encoded(data) {
        context.get_context().draw_image_src_xywh_dst_xywh(
            &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
        )
    }
}

pub fn canvas_native_context_draw_image_dx_dy_asset(
    context: &mut CanvasRenderingContext2D,
    asset: &mut ImageAsset,
    dx: f32,
    dy: f32,
) {
    let width = asset.width() as f32;
    let height = asset.height() as f32;
    canvas_native_context_draw_image_asset(
        context, asset, 0.0, 0.0, width, height, dx, dy, width, height,
    );
}

pub fn canvas_native_context_draw_image_dx_dy_dw_dh_asset(
    context: &mut CanvasRenderingContext2D,
    asset: &mut ImageAsset,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    let width = asset.width() as f32;
    let height = asset.height() as f32;
    canvas_native_context_draw_image_asset(
        context, asset, 0.0, 0.0, width, height, dx, dy, d_width, d_height,
    );
}

pub fn canvas_native_context_draw_image_asset(
    context: &mut CanvasRenderingContext2D,
    asset: &mut ImageAsset,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    let bytes = asset.lock().rgba_internal_bytes();
    if let Some(image) = from_image_slice(
        bytes.as_slice(),
        asset.width() as i32,
        asset.height() as i32,
    ) {
        context.get_context().draw_image_src_xywh_dst_xywh(
            &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
        )
    }
}

pub fn canvas_native_context_ellipse(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    radius_x: f32,
    radius_y: f32,
    rotation: f32,
    start_angle: f32,
    end_angle: f32,
    anticlockwise: bool,
) {
    context.get_context().ellipse(
        x,
        y,
        radius_x,
        radius_y,
        rotation,
        start_angle,
        end_angle,
        anticlockwise,
    )
}

pub fn canvas_native_context_fill(context: &mut CanvasRenderingContext2D, rule: &str) {
    if let Ok(rule) = FillRule::try_from(rule) {
        context.get_context().fill(None, rule)
    }
}

pub fn canvas_native_context_fill_with_path(
    context: &mut CanvasRenderingContext2D,
    path: &mut Path,
    rule: &str,
) {
    if let Ok(rule) = FillRule::try_from(rule) {
        context.get_context().fill(Some(path.inner_mut()), rule)
    }
}

pub fn canvas_native_context_fill_rect(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    context.get_context().fill_rect_xywh(x, y, width, height);
}

pub fn canvas_native_context_fill_text(
    context: &mut CanvasRenderingContext2D,
    text: &str,
    x: f32,
    y: f32,
    width: f32,
) {
    context.get_context().fill_text(text, x, y, width)
}

pub fn canvas_native_context_get_image_data(
    context: &mut CanvasRenderingContext2D,
    sx: f32,
    sy: f32,
    sw: f32,
    sh: f32,
) -> Box<ImageData> {
    Box::new(ImageData(
        context.get_context().get_image_data(sx, sy, sw, sh),
    ))
}

pub fn canvas_native_context_get_transform(context: &mut CanvasRenderingContext2D) -> Box<Matrix> {
    Box::new(Matrix(canvas_core::context::matrix::Matrix::from(
        context.get_context().get_transform(),
    )))
}

pub fn canvas_native_context_is_point_in_path(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    rule: &str,
) -> bool {
    FillRule::try_from(rule).map_or(false, |rule| {
        context.get_context().is_point_in_path(None, x, y, rule)
    })
}

pub fn canvas_native_context_is_point_in_path_with_path(
    context: &mut CanvasRenderingContext2D,
    path: &mut Path,
    x: f32,
    y: f32,
    rule: &str,
) -> bool {
    FillRule::try_from(rule).map_or(false, |rule| {
        context
            .get_context()
            .is_point_in_path(Some(path.inner()), x, y, rule)
    })
}

pub fn canvas_native_context_is_point_in_stroke(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
) -> bool {
    context.get_context().is_point_in_stroke(None, x, y)
}

pub fn canvas_native_context_is_point_in_stroke_with_path(
    context: &mut CanvasRenderingContext2D,
    path: &mut Path,
    x: f32,
    y: f32,
) -> bool {
    context
        .get_context()
        .is_point_in_stroke(Some(path.inner()), x, y)
}

pub fn canvas_native_context_line_to(context: &mut CanvasRenderingContext2D, x: f32, y: f32) {
    context.get_context().line_to(x, y)
}

pub fn canvas_native_context_measure_text(
    context: &mut CanvasRenderingContext2D,
    text: &str,
) -> Box<TextMetrics> {
    Box::new(TextMetrics(
        context.get_context().measure_text(text.as_ref()),
    ))
}

pub fn canvas_native_context_move_to(context: &mut CanvasRenderingContext2D, x: f32, y: f32) {
    context.get_context().move_to(x, y)
}

pub fn canvas_native_context_put_image_data(
    context: &mut CanvasRenderingContext2D,
    image_data: &mut ImageData,
    dx: f32,
    dy: f32,
    dirty_x: f32,
    dirty_y: f32,
    dirty_width: f32,
    dirty_height: f32,
) {
    context.get_context().put_image_data(
        image_data.inner(),
        dx,
        dy,
        dirty_x,
        dirty_y,
        dirty_width,
        dirty_height,
    )
}

pub fn canvas_native_context_quadratic_curve_to(
    context: &mut CanvasRenderingContext2D,
    cpx: f32,
    cpy: f32,
    x: f32,
    y: f32,
) {
    context.get_context().quadratic_curve_to(cpx, cpy, x, y)
}

pub fn canvas_native_context_rect(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    context.get_context().rect(x, y, width, height)
}

pub fn canvas_native_context_reset_transform(context: &mut CanvasRenderingContext2D) {
    context.get_context().reset_transform()
}

pub fn canvas_native_context_restore(context: &mut CanvasRenderingContext2D) {
    context.get_context().restore()
}

pub fn canvas_native_context_rotate(context: &mut CanvasRenderingContext2D, angle: f32) {
    context.get_context().rotate(angle)
}

pub fn canvas_native_context_save(context: &mut CanvasRenderingContext2D) {
    context.get_context().save()
}

pub fn canvas_native_context_scale(context: &mut CanvasRenderingContext2D, x: f32, y: f32) {
    context.get_context().scale(x, y)
}

pub fn canvas_native_context_set_transform(
    context: &mut CanvasRenderingContext2D,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
) {
    context.get_context().set_transform(a, b, c, d, e, f)
}

pub fn canvas_native_context_set_transform_matrix(
    context: &mut CanvasRenderingContext2D,
    matrix: &mut Matrix,
) {
    let matrix = matrix.inner_mut().to_m33();
    context.get_context().set_transform_matrix(&matrix)
}

pub fn canvas_native_context_stroke(context: &mut CanvasRenderingContext2D) {
    context.get_context().stroke(None)
}

pub fn canvas_native_context_stroke_with_path(
    context: &mut CanvasRenderingContext2D,
    path: &mut Path,
) {
    context.get_context().stroke(Some(path.inner_mut()))
}

pub fn canvas_native_context_stroke_rect(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    context.get_context().stroke_rect_xywh(x, y, width, height);
}

pub fn canvas_native_context_stroke_text(
    context: &mut CanvasRenderingContext2D,
    text: &str,
    x: f32,
    y: f32,
    width: f32,
) {
    context.get_context().stroke_text(text, x, y, width)
}

pub fn canvas_native_context_transform(
    context: &mut CanvasRenderingContext2D,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
) {
    context.get_context().transform(a, b, c, d, e, f)
}

pub fn canvas_native_context_translate(context: &mut CanvasRenderingContext2D, x: f32, y: f32) {
    context.get_context().translate(x, y)
}

pub fn canvas_native_context_flush(context: &CanvasRenderingContext2D) {
    context.get_context().flush();
}

/* CanvasRenderingContext2D */

/* Path2D */
#[derive(Clone)]
pub struct Path(canvas_core::context::paths::path::Path);

impl Default for Path {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Path {
    pub(crate) fn inner(&self) -> &canvas_core::context::paths::path::Path {
        &self.0
    }

    pub(crate) fn inner_mut(&mut self) -> &mut canvas_core::context::paths::path::Path {
        &mut self.0
    }
}

pub fn canvas_native_path_add_path(path: &mut Path, path_to_add: &Path) {
    path.0.add_path(&path_to_add.0, None);
}

pub fn canvas_native_path_create() -> Box<Path> {
    Box::new(Path::default())
}

pub fn canvas_native_path_create_with_path(path: &Path) -> Box<Path> {
    Box::new(path.clone())
}

pub fn canvas_native_path_create_with_string(string: String) -> Box<Path> {
    Box::new(Path(canvas_core::context::paths::path::Path::from_str(
        string.as_str(),
    )))
}

pub fn canvas_native_path_close_path(path: &mut Path) {
    path.0.close_path()
}

pub fn canvas_native_path_move_to(path: &mut Path, x: f32, y: f32) {
    path.0.move_to(x, y)
}

pub fn canvas_native_path_line_to(path: &mut Path, x: f32, y: f32) {
    path.0.line_to(x, y)
}

pub fn canvas_native_path_bezier_curve_to(
    path: &mut Path,
    cp1x: f32,
    cp1y: f32,
    cp2x: f32,
    cp2y: f32,
    x: f32,
    y: f32,
) {
    path.0.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y)
}

pub fn canvas_native_path_quadratic_curve_to(path: &mut Path, cpx: f32, cpy: f32, x: f32, y: f32) {
    path.0.quadratic_curve_to(cpx, cpy, x, y)
}

pub fn canvas_native_path_arc(
    path: &mut Path,
    x: f32,
    y: f32,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    anti_clockwise: bool,
) {
    path.0
        .arc(x, y, radius, start_angle, end_angle, anti_clockwise)
}

pub fn canvas_native_path_arc_to(path: &mut Path, x1: f32, y1: f32, x2: f32, y2: f32, radius: f32) {
    path.0.arc_to(x1, y1, x2, y2, radius)
}

pub fn canvas_native_path_ellipse(
    path: &mut Path,
    x: f32,
    y: f32,
    radius_x: f32,
    radius_y: f32,
    rotation: f32,
    start_angle: f32,
    end_angle: f32,
    anticlockwise: bool,
) {
    path.0.ellipse(
        x,
        y,
        radius_x,
        radius_y,
        rotation,
        start_angle,
        end_angle,
        anticlockwise,
    )
}

pub fn canvas_native_path_rect(path: &mut Path, x: f32, y: f32, width: f32, height: f32) {
    path.0.rect(x, y, width, height)
}

pub fn canvas_native_path_to_string(path: &Path) -> String {
    path.0.path().to_svg()
}

/* Path 2D */

/* DOMMatrix */
#[derive(Clone)]
pub struct Matrix(canvas_core::context::matrix::Matrix);

impl Matrix {
    pub fn inner(&self) -> &canvas_core::context::matrix::Matrix {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut canvas_core::context::matrix::Matrix {
        &mut self.0
    }
}

pub fn canvas_native_matrix_create() -> Box<Matrix> {
    Box::new(Matrix(canvas_core::context::matrix::Matrix::new()))
}

pub fn canvas_native_matrix_update(matrix: &mut Matrix, slice: &[f32]) {
    let mut affine = [0f32; 6];
    affine.copy_from_slice(slice);
    matrix.inner_mut().set_affine(&affine);
}

pub fn canvas_native_matrix_get_a(matrix: &Matrix) -> f32 {
    matrix.inner().a()
}

pub fn canvas_native_matrix_set_a(matrix: &mut Matrix, a: f32) {
    matrix.inner_mut().set_a(a)
}

pub fn canvas_native_matrix_get_b(matrix: &Matrix) -> f32 {
    matrix.inner().b()
}

pub fn canvas_native_matrix_set_b(matrix: &mut Matrix, b: f32) {
    matrix.inner_mut().set_b(b)
}

pub fn canvas_native_matrix_get_c(matrix: &Matrix) -> f32 {
    matrix.inner().c()
}

pub fn canvas_native_matrix_set_c(matrix: &mut Matrix, c: f32) {
    matrix.inner_mut().set_c(c)
}

pub fn canvas_native_matrix_get_d(matrix: &Matrix) -> f32 {
    matrix.inner().d()
}

pub fn canvas_native_matrix_set_d(matrix: &mut Matrix, d: f32) {
    matrix.inner_mut().set_d(d)
}

pub fn canvas_native_matrix_get_e(matrix: &Matrix) -> f32 {
    matrix.inner().e()
}

pub fn canvas_native_matrix_set_e(matrix: &mut Matrix, e: f32) {
    matrix.inner_mut().set_e(e)
}

pub fn canvas_native_matrix_get_f(matrix: &Matrix) -> f32 {
    matrix.inner().f()
}

pub fn canvas_native_matrix_set_f(matrix: &mut Matrix, f: f32) {
    matrix.inner_mut().set_f(f)
}

pub fn canvas_native_matrix_get_m11(matrix: &Matrix) -> f32 {
    matrix.inner().m11()
}

pub fn canvas_native_matrix_set_m11(matrix: &mut Matrix, m11: f32) {
    matrix.inner_mut().set_m11(m11)
}

pub fn canvas_native_matrix_get_m12(matrix: &Matrix) -> f32 {
    matrix.inner().m12()
}

pub fn canvas_native_matrix_set_m12(matrix: &mut Matrix, m12: f32) {
    matrix.inner_mut().set_m12(m12)
}

pub fn canvas_native_matrix_get_m13(matrix: &Matrix) -> f32 {
    matrix.inner().m13()
}

pub fn canvas_native_matrix_set_m13(matrix: &mut Matrix, m13: f32) {
    matrix.inner_mut().set_m13(m13)
}

pub fn canvas_native_matrix_get_m14(matrix: &Matrix) -> f32 {
    matrix.inner().m14()
}

pub fn canvas_native_matrix_set_m14(matrix: &mut Matrix, m14: f32) {
    matrix.inner_mut().set_m14(m14)
}

pub fn canvas_native_matrix_get_m21(matrix: &Matrix) -> f32 {
    matrix.inner().m21()
}

pub fn canvas_native_matrix_set_m21(matrix: &mut Matrix, m21: f32) {
    matrix.inner_mut().set_m21(m21)
}

pub fn canvas_native_matrix_get_m22(matrix: &Matrix) -> f32 {
    matrix.inner().m22()
}

pub fn canvas_native_matrix_set_m22(matrix: &mut Matrix, m22: f32) {
    matrix.inner_mut().set_m22(m22)
}

pub fn canvas_native_matrix_get_m23(matrix: &Matrix) -> f32 {
    matrix.inner().m23()
}

pub fn canvas_native_matrix_set_m23(matrix: &mut Matrix, m23: f32) {
    matrix.inner_mut().set_m23(m23)
}

pub fn canvas_native_matrix_get_m24(matrix: &Matrix) -> f32 {
    matrix.inner().m24()
}

pub fn canvas_native_matrix_set_m24(matrix: &mut Matrix, m24: f32) {
    matrix.inner_mut().set_m24(m24)
}

pub fn canvas_native_matrix_get_m31(matrix: &Matrix) -> f32 {
    matrix.inner().m31()
}

pub fn canvas_native_matrix_set_m31(matrix: &mut Matrix, m31: f32) {
    matrix.inner_mut().set_m31(m31)
}

pub fn canvas_native_matrix_get_m32(matrix: &Matrix) -> f32 {
    matrix.inner().m32()
}

pub fn canvas_native_matrix_set_m32(matrix: &mut Matrix, m32: f32) {
    matrix.inner_mut().set_m32(m32)
}

pub fn canvas_native_matrix_get_m33(matrix: &Matrix) -> f32 {
    matrix.inner().m33()
}

pub fn canvas_native_matrix_set_m33(matrix: &mut Matrix, m33: f32) {
    matrix.inner_mut().set_m33(m33)
}

pub fn canvas_native_matrix_get_m34(matrix: &Matrix) -> f32 {
    matrix.inner().m34()
}

pub fn canvas_native_matrix_set_m34(matrix: &mut Matrix, m34: f32) {
    matrix.inner_mut().set_m34(m34)
}

pub fn canvas_native_matrix_get_m41(matrix: &Matrix) -> f32 {
    matrix.inner().m41()
}

pub fn canvas_native_matrix_set_m41(matrix: &mut Matrix, m41: f32) {
    matrix.inner_mut().set_m41(m41)
}

pub fn canvas_native_matrix_get_m42(matrix: &Matrix) -> f32 {
    matrix.inner().m42()
}

pub fn canvas_native_matrix_set_m42(matrix: &mut Matrix, m42: f32) {
    matrix.inner_mut().set_m42(m42)
}

pub fn canvas_native_matrix_get_m43(matrix: &Matrix) -> f32 {
    matrix.inner().m43()
}

pub fn canvas_native_matrix_set_m43(matrix: &mut Matrix, m43: f32) {
    matrix.inner_mut().set_m43(m43)
}

pub fn canvas_native_matrix_get_m44(matrix: &Matrix) -> f32 {
    matrix.inner().m44()
}

pub fn canvas_native_matrix_set_m44(matrix: &mut Matrix, m44: f32) {
    matrix.inner_mut().set_m44(m44)
}

/* DOMMatrix */

/* ImageData */
#[derive(Clone)]
pub struct ImageData(canvas_core::context::pixel_manipulation::image_data::ImageData);

impl ImageData {
    pub(crate) fn new(
        data: canvas_core::context::pixel_manipulation::image_data::ImageData,
    ) -> Self {
        ImageData(data)
    }

    pub fn inner(&self) -> &canvas_core::context::pixel_manipulation::image_data::ImageData {
        &self.0
    }
}

pub fn canvas_native_image_data_create(width: i32, height: i32) -> Box<ImageData> {
    let data = canvas_core::context::pixel_manipulation::ImageData::new(width, height);
    Box::new(ImageData::new(data))
}

pub fn canvas_native_image_data_get_width(image_data: &ImageData) -> i32 {
    image_data.0.width()
}

pub fn canvas_native_image_data_get_height(image_data: &ImageData) -> i32 {
    image_data.0.height()
}

pub fn canvas_native_image_data_get_data(image_data: &mut ImageData) -> &mut [u8] {
    image_data.0.data_mut()
}

pub fn canvas_native_image_data_get_shared_instance(image_data: &mut ImageData) -> Box<ImageData> {
    Box::new(image_data.clone())
}

/* ImageData */

/* ImageAsset */

pub struct ImageAsset(Arc<Mutex<canvas_core::context::image_asset::ImageAsset>>);

impl Default for ImageAsset {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Default::default())))
    }
}

impl Clone for ImageAsset {
    fn clone(&self) -> Self {
        Self {
            0: Arc::clone(&self.0),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.0 = Arc::clone(&source.0)
    }
}

impl ImageAsset {
    pub fn lock(&self) -> MutexGuard<canvas_core::context::image_asset::ImageAsset> {
        self.0.lock()
    }

    pub fn width(&self) -> c_uint {
        self.0.lock().width()
    }

    pub fn height(&self) -> c_uint {
        self.0.lock().height()
    }
}

pub fn canvas_native_image_asset_create() -> Box<ImageAsset> {
    Box::new(ImageAsset::default())
}

pub fn canvas_native_image_asset_load_from_path(asset: &mut ImageAsset, path: String) -> bool {
    asset.lock().load_from_path(&path)
}

pub fn canvas_native_image_asset_load_from_path_async(
    asset: &mut ImageAsset,
    path: String,
    callback: isize,
) {
    let mut asset = asset.clone();
    std::thread::spawn(move || {
        let done = asset.lock().load_from_path(path.as_ref());
        ffi::OnImageAssetLoadCallbackHolderComplete(callback, done);
    });
}

pub fn canvas_native_image_asset_load_from_raw(asset: &mut ImageAsset, array: &[u8]) -> bool {
    asset.lock().load_from_bytes(array)
}

pub fn canvas_native_image_asset_load_from_raw_async(
    asset: &mut ImageAsset,
    array: &[u8],
    callback: isize,
) {
    let mut asset = asset.clone();
    let array = array.to_vec();
    std::thread::spawn(move || {
        let done = asset.lock().load_from_bytes(array.as_slice());
        ffi::OnImageAssetLoadCallbackHolderComplete(callback, done);
    });
}

pub fn canvas_native_image_asset_load_from_url_str(asset: &mut ImageAsset, url: &str) -> bool {
    use std::fs::File;
    use std::io::prelude::*;
    if let Ok(res) = ureq::get(url).call() {
        let mut reader = res.into_reader();
        let mut data = Vec::new();
        return match reader.read_to_end(&mut data) {
            Ok(_) => asset.lock().load_from_bytes(data.as_slice()),
            Err(_) => false,
        };
    }
    false
}

pub fn canvas_native_image_asset_load_from_url(asset: &mut ImageAsset, url: String) -> bool {
    canvas_native_image_asset_load_from_url_str(asset, url.as_str())
}

pub fn canvas_native_image_asset_load_from_url_async(
    asset: &mut ImageAsset,
    url: String,
    callback: isize,
) {
    let mut asset = asset.clone();
    let callback = callback.clone();
    std::thread::spawn(move || {
        let done = canvas_native_image_asset_load_from_url_str(&mut asset, url.as_str());
        ffi::OnImageAssetLoadCallbackHolderComplete(callback, done);
    });
}

pub fn canvas_native_image_asset_get_bytes(asset: &mut ImageAsset) -> Vec<u8> {
    asset.lock().bytes_internal()
}

pub fn canvas_native_image_asset_get_rgba_bytes(asset: &mut ImageAsset) -> Vec<u8> {
    asset.lock().rgba_internal_bytes()
}

pub fn canvas_native_image_asset_get_rgb_bytes(asset: &mut ImageAsset) -> Vec<u8> {
    asset.lock().rgb_internal_bytes()
}

pub fn canvas_native_image_asset_width(asset: &mut ImageAsset) -> u32 {
    asset.lock().width()
}

pub fn canvas_native_image_asset_height(asset: &mut ImageAsset) -> u32 {
    asset.lock().height()
}

pub fn canvas_native_image_asset_get_error(asset: &mut ImageAsset) -> String {
    asset.lock().error().to_string()
}

pub fn canvas_native_image_asset_has_error(asset: &mut ImageAsset) -> bool {
    if asset.lock().error().is_empty() {
        return false;
    }
    true
}

pub fn canvas_native_image_asset_scale(asset: &mut ImageAsset, x: u32, y: u32) -> bool {
    asset.lock().scale(x, y)
}

pub fn canvas_native_image_asset_flip_x(asset: &mut ImageAsset) -> bool {
    asset.lock().flip_x()
}

pub fn canvas_native_image_asset_flip_x_in_place(asset: &mut ImageAsset) -> bool {
    asset.lock().flip_x_in_place()
}

pub fn canvas_native_image_asset_flip_y(asset: &mut ImageAsset) -> bool {
    asset.lock().flip_y()
}

pub fn canvas_native_image_asset_flip_y_in_place_owned(buf: &mut [u8]) {
    if let Ok(mut image) = image::load_from_memory(buf) {
        image::imageops::flip_vertical_in_place(&mut image);
    }
}

pub fn canvas_native_image_asset_flip_x_in_place_owned(buf: &mut [u8]) {
    if let Ok(mut image) = image::load_from_memory(buf) {
        image::imageops::flip_horizontal_in_place(&mut image);
    }
}

pub fn canvas_native_image_asset_flip_y_in_place(asset: &mut ImageAsset) -> bool {
    asset.lock().flip_y_in_place()
}

pub fn canvas_native_image_asset_save_path(
    asset: &mut ImageAsset,
    path: &str,
    format: u32,
) -> bool {
    asset.lock().save_path(
        path,
        canvas_core::context::image_asset::OutputFormat::from(format),
    )
}

pub fn canvas_native_image_asset_save_path_async(
    asset: &mut ImageAsset,
    path: &str,
    format: u32,
    callback: isize,
) {
    let mut asset = asset.clone();
    let path = path.to_string();
    std::thread::spawn(move || {
        let done = asset.lock().save_path(
            path.as_ref(),
            canvas_core::context::image_asset::OutputFormat::from(format),
        );

        ffi::OnImageAssetLoadCallbackHolderComplete(callback, done);
    });
}

/* ImageAsset */

/* TextMetrics */

pub fn canvas_native_text_metrics_get_width(metrics: &TextMetrics) -> f32 {
    metrics.0.width()
}

pub fn canvas_native_text_metrics_get_actual_bounding_box_left(metrics: &TextMetrics) -> f32 {
    metrics.0.actual_bounding_box_left()
}

pub fn canvas_native_text_metrics_get_actual_bounding_box_right(metrics: &TextMetrics) -> f32 {
    metrics.0.actual_bounding_box_right()
}

pub fn canvas_native_text_metrics_get_actual_bounding_box_ascent(metrics: &TextMetrics) -> f32 {
    metrics.0.actual_bounding_box_ascent()
}

pub fn canvas_native_text_metrics_get_actual_bounding_box_descent(metrics: &TextMetrics) -> f32 {
    metrics.0.actual_bounding_box_descent()
}

pub fn canvas_native_text_metrics_get_font_bounding_box_ascent(metrics: &TextMetrics) -> f32 {
    metrics.0.font_bounding_box_ascent()
}

pub fn canvas_native_text_metrics_get_font_bounding_box_descent(metrics: &TextMetrics) -> f32 {
    metrics.0.font_bounding_box_descent()
}

pub fn canvas_native_text_metrics_get_em_height_ascent(metrics: &TextMetrics) -> f32 {
    metrics.0.em_height_ascent()
}

pub fn canvas_native_text_metrics_get_em_height_descent(metrics: &TextMetrics) -> f32 {
    metrics.0.em_height_descent()
}

pub fn canvas_native_text_metrics_get_hanging_baseline(metrics: &TextMetrics) -> f32 {
    metrics.0.hanging_baseline()
}

pub fn canvas_native_text_metrics_get_alphabetic_baseline(metrics: &TextMetrics) -> f32 {
    metrics.0.alphabetic_baseline()
}

pub fn canvas_native_text_metrics_get_ideographic_baseline(metrics: &TextMetrics) -> f32 {
    metrics.0.ideographic_baseline()
}

/* TextMetrics */

/* CanvasGradient */
pub fn canvas_native_gradient_add_color_stop(style: &mut PaintStyle, stop: f32, color: &str) {
    if let Some(style) = style.0.as_mut() {
        match style {
            canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(gradient) => {
                gradient.add_color_stop_str(stop, color)
            }
            _ => {}
        }
    }
}

/* CanvasGradient */

/* CanvasPattern */
fn canvas_native_pattern_set_transform(pattern: &mut PaintStyle, matrix: &Matrix) {
    if let Some(pattern) = pattern.0.as_mut() {
        match pattern {
            canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(pattern) => {
                pattern.set_pattern_transform(matrix.inner())
            }
            _ => {}
        }
    }
}

/* CanvasPattern */

/* TextDecoder */
pub fn canvas_native_text_decoder_create(decoding: &str) -> Box<TextDecoder> {
    Box::new(TextDecoder(
        canvas_core::context::text_decoder::TextDecoder::new(Some(decoding)),
    ))
}
pub fn canvas_native_text_decoder_decode(decoder: &mut TextDecoder, data: &[u8]) -> String {
    decoder.0.decode_to_string(data)
}

pub fn canvas_native_text_decoder_get_encoding(decoder: &mut TextDecoder) -> String {
    decoder.0.encoding().to_string()
}
/* TextDecoder */

/* TextEncoder */
pub fn canvas_native_text_encoder_create(encoding: &str) -> Box<TextEncoder> {
    Box::new(TextEncoder(
        canvas_core::context::text_encoder::TextEncoder::new(Some(encoding)),
    ))
}
pub fn canvas_native_text_encoder_encode(encoder: &mut TextEncoder, text: &str) -> Vec<u8> {
    encoder.0.encode(text)
}

pub fn canvas_native_text_encoder_get_encoding(encoder: &mut TextEncoder) -> String {
    encoder.0.encoding().to_string()
}

/* TextEncoder */


pub fn canvas_native_raf_create(callback: isize)-> Box<Raf> {
    Box::new(
        Raf(crate::raf::Raf::new(Some(Box::new(move |ts|{
            ffi::OnRafCallbackOnFrame(callback, ts);
        }))))
    )
}

pub fn canvas_native_raf_start(raf: &mut Raf) {
    raf.0.start();
}
pub fn canvas_native_raf_stop(raf: &mut Raf) {
    raf.0.stop()
}
pub fn canvas_native_raf_get_started(raf: &Raf) -> bool {
    raf.0.started()
}

pub fn canvas_native_context_gl_make_current(context: &CanvasRenderingContext2D) -> bool {
    context.gl_context.make_current()
}
pub fn canvas_native_context_gl_swap_buffers(context: &CanvasRenderingContext2D) -> bool {
    context.gl_context.swap_buffers()
}