#![allow(non_snake_case)]

use cxx::{type_id, ExternType};
use std::borrow::Cow;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::io::Read;
use std::os::raw::{c_char, c_int, c_longlong, c_uint, c_ulong, c_void};
use std::os::unix::io::FromRawFd;
use std::os::unix::prelude::IntoRawFd;
use std::sync::Arc;

use parking_lot::lock_api::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use parking_lot::{Mutex, MutexGuard, RawMutex, RawRwLock};

use canvas_2d::context::compositing::composite_operation_type::CompositeOperationType;
use canvas_2d::context::drawing_paths::fill_rule::FillRule;
use canvas_2d::context::fill_and_stroke_styles::paint::paint_style_set_color_with_string;
use canvas_2d::context::fill_and_stroke_styles::pattern::Repetition;
use canvas_2d::context::image_asset::OutputFormat;
use canvas_2d::context::image_smoothing::ImageSmoothingQuality;
use canvas_2d::context::line_styles::line_cap::LineCap;
use canvas_2d::context::line_styles::line_join::LineJoin;
use canvas_2d::context::text_styles::text_align::TextAlign;
use canvas_2d::context::text_styles::text_direction::TextDirection;
use canvas_2d::context::{Context, ContextWrapper};
use canvas_2d::gl::GLContext;
use canvas_2d::image_asset::OutputFormat;
use canvas_2d::utils::color::{parse_color, to_parsed_color};
use canvas_2d::utils::image::{
    from_bitmap_slice, from_image_slice, from_image_slice_encoded, from_image_slice_no_copy,
    to_image_encoded_from_data,
};
use canvas_core::gl::GLContext;
use canvas_core::image_asset::OutputFormat;

use crate::canvas2d;
use crate::ffi::{ImageBitmapPremultiplyAlpha, WebGLExtensionType, WebGLResultType};
use crate::webgl2::BitmapBytes;

/* Utils */

pub fn console_log(text: &str) {}

pub fn str_to_buf(value: &str) -> Vec<u8> {
    value.as_bytes().to_vec()
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

/* CanvasRenderingContext2D */

pub struct CanvasRenderingContext2D {
    context: ContextWrapper,
    pub(crate) gl_context: GLContext,
    alpha: bool,
}

unsafe impl ExternType for CanvasRenderingContext2D {
    type Id = type_id!("CanvasRenderingContext2D");
    type Kind = cxx::kind::Trivial;
}

fn to_data_url(context: &mut CanvasRenderingContext2D, format: &str, quality: i32) -> String {
    canvas_core::to_data_url(&mut context.context, format, quality)
}

impl CanvasRenderingContext2D {
    pub fn get_context(&self) -> RwLockReadGuard<'_, RawRwLock, Context> {
        self.context.get_context()
    }

    pub fn get_context_mut(&self) -> RwLockWriteGuard<'_, RawRwLock, Context> {
        self.context.get_context_mut()
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.make_current();
        self.context.resize_gl(width, height);
    }

    pub fn make_current(&self) {
        self.gl_context.make_current();
    }

    pub fn remove_if_current(&self) {
        self.gl_context.remove_if_current();
    }
}

#[derive(Clone)]
pub struct PaintStyle(Option<canvas2d::fill_and_stroke_styles::paint::PaintStyle>);

impl PaintStyle {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    #[inline]
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
pub(crate) mod ffi {
    pub(crate) enum PaintStyleType {
        None,
        Color,
        Gradient,
        Pattern,
    }

    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    pub enum ImageBitmapPremultiplyAlpha {
        Default,
        Premultiply,
        None,
    }

    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    pub enum ImageBitmapColorSpaceConversion {
        Default,
        None,
    }

    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    pub enum ImageBitmapResizeQuality {
        Low,
        Medium,
        High,
        Pixelated,
    }

    extern "C++" {
        include!("canvas-cxx/src/lib.rs.h");
        type WebGLState = crate::webgl::WebGLState;
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

        /* Utils */
        fn console_log(text: &str);
        fn str_to_buf(value: &str) -> Vec<u8>;
        /* Utils */

        /* ImageBitmap */
        fn canvas_native_image_bitmap_create_from_asset(
            asset: &mut ImageAsset,
            flip_y: bool,
            premultiply_alpha: ImageBitmapPremultiplyAlpha,
            color_space_conversion: ImageBitmapColorSpaceConversion,
            resize_quality: ImageBitmapResizeQuality,
            resize_width: f32,
            resize_height: f32,
        ) -> Box<ImageAsset>;

        fn canvas_native_image_bitmap_create_from_asset_src_rect(
            asset: &mut ImageAsset,
            sx: f32,
            sy: f32,
            s_width: f32,
            s_height: f32,
            flip_y: bool,
            premultiply_alpha: ImageBitmapPremultiplyAlpha,
            color_space_conversion: ImageBitmapColorSpaceConversion,
            resize_quality: ImageBitmapResizeQuality,
            resize_width: f32,
            resize_height: f32,
        ) -> Box<ImageAsset>;

        fn canvas_native_image_bitmap_create_from_encoded_bytes(
            bytes: &[u8],
            flip_y: bool,
            premultiply_alpha: ImageBitmapPremultiplyAlpha,
            color_space_conversion: ImageBitmapColorSpaceConversion,
            resize_quality: ImageBitmapResizeQuality,
            resize_width: f32,
            resize_height: f32,
        ) -> Box<ImageAsset>;

        fn canvas_native_image_bitmap_create_from_encoded_bytes_with_output(
            bytes: &[u8],
            flip_y: bool,
            premultiply_alpha: ImageBitmapPremultiplyAlpha,
            color_space_conversion: ImageBitmapColorSpaceConversion,
            resize_quality: ImageBitmapResizeQuality,
            resize_width: f32,
            resize_height: f32,
            output: &mut ImageAsset,
        ) -> bool;

        fn canvas_native_image_bitmap_create_from_encoded_bytes_src_rect(
            bytes: &[u8],
            sx: f32,
            sy: f32,
            s_width: f32,
            s_height: f32,
            flip_y: bool,
            premultiply_alpha: ImageBitmapPremultiplyAlpha,
            color_space_conversion: ImageBitmapColorSpaceConversion,
            resize_quality: ImageBitmapResizeQuality,
            resize_width: f32,
            resize_height: f32,
        ) -> Box<ImageAsset>;

        fn canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(
            bytes: &[u8],
            sx: f32,
            sy: f32,
            s_width: f32,
            s_height: f32,
            flip_y: bool,
            premultiply_alpha: ImageBitmapPremultiplyAlpha,
            color_space_conversion: ImageBitmapColorSpaceConversion,
            resize_quality: ImageBitmapResizeQuality,
            resize_width: f32,
            resize_height: f32,
            output: &mut ImageAsset,
        ) -> bool;

        /* ImageBitmap */

        /* Path2D */
        fn canvas_native_path_add_path(path: &mut Path, path_to_add: &Path);
        fn canvas_native_path_create() -> Box<Path>;
        fn canvas_native_path_create_with_path(path: &Path) -> Box<Path>;
        fn canvas_native_path_create_with_string(string: String) -> Box<Path>;
        fn canvas_native_path_create_with_str(string: &str) -> Box<Path>;
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

        fn canvas_native_image_asset_shared_clone(asset: &ImageAsset) -> Box<ImageAsset>;

        fn canvas_native_image_asset_get_size(asset: &ImageAsset) -> usize;

        fn canvas_native_image_asset_load_from_path(asset: &mut ImageAsset, path: &str) -> bool;

        fn canvas_native_image_asset_load_from_url(asset: &mut ImageAsset, url: &str) -> bool;

        fn canvas_native_image_asset_load_from_raw(asset: &mut ImageAsset, array: &[u8]) -> bool;

        // fn canvas_native_image_asset_get_bytes(asset: &mut ImageAsset) -> &[u8];

        fn canvas_native_image_asset_addr(asset: &mut ImageAsset) -> i64;

        fn canvas_native_image_asset_width(asset: &mut ImageAsset) -> u32;

        fn canvas_native_image_asset_height(asset: &mut ImageAsset) -> u32;

        fn canvas_native_image_asset_get_error(asset: &mut ImageAsset) -> String;

        fn canvas_native_image_asset_has_error(asset: &mut ImageAsset) -> bool;

        fn canvas_native_image_asset_scale(asset: &mut ImageAsset, x: u32, y: u32) -> bool;

        fn canvas_native_image_asset_save_path(
            asset: &mut ImageAsset,
            path: &str,
            format: u32,
        ) -> bool;

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

        /* GL */
        fn canvas_native_context_gl_make_current(context: &CanvasRenderingContext2D) -> bool;
        fn canvas_native_context_gl_swap_buffers(context: &CanvasRenderingContext2D) -> bool;
        /* GL */

        /* CanvasRenderingContext2D */
        fn canvas_native_context_create_with_wrapper(context: i64)
            -> Box<CanvasRenderingContext2D>;

        fn canvas_native_context_resize(
            context: &mut CanvasRenderingContext2D,
            width: f32,
            height: f32,
        );

        fn canvas_native_context_create_with_current(
            width: f32,
            height: f32,
            scale: f32,
            font_color: i32,
            ppi: f32,
            direction: u32,
            alpha: bool,
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

        fn canvas_native_context_create_gl(
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

        #[cfg(feature = "webgl")]
        fn canvas_native_context_create_gl_no_window(
            width: f32,
            height: f32,
            density: f32,
            font_color: i32,
            ppi: f32,
            direction: u32,
            alpha: bool,
        ) -> Box<CanvasRenderingContext2D>;

        fn canvas_native_context_flush(context: &CanvasRenderingContext2D);

        fn canvas_native_to_data_url(
            context: &mut CanvasRenderingContext2D,
            format: &str,
            quality: i32,
        ) -> String;

        fn canvas_native_context_get_font(context: &CanvasRenderingContext2D) -> String;

        fn canvas_native_context_set_font(context: &mut CanvasRenderingContext2D, font: &str);

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
        ) -> &str;

        fn canvas_native_context_set_image_smoothing_quality(
            context: &mut CanvasRenderingContext2D,
            quality: &str,
        );

        fn canvas_native_context_get_line_join(context: &CanvasRenderingContext2D) -> &str;

        fn canvas_native_context_set_line_join(context: &mut CanvasRenderingContext2D, join: &str);

        fn canvas_native_context_get_line_cap(context: &CanvasRenderingContext2D) -> &str;

        fn canvas_native_context_set_line_cap(context: &mut CanvasRenderingContext2D, cap: &str);

        fn canvas_native_context_get_miter_limit(context: &CanvasRenderingContext2D) -> f32;

        fn canvas_native_context_set_miter_limit(
            context: &mut CanvasRenderingContext2D,
            limit: f32,
        );

        fn canvas_native_context_get_shadow_color(context: &CanvasRenderingContext2D) -> String;

        fn canvas_native_context_get_shadow_color_buf(
            context: &CanvasRenderingContext2D,
        ) -> Vec<u8>;

        fn canvas_native_context_get_shadow_color_rgba(
            context: &CanvasRenderingContext2D,
            r: &mut u8,
            g: &mut u8,
            b: &mut u8,
            a: &mut u8,
        );

        fn canvas_native_context_set_shadow_color(
            context: &mut CanvasRenderingContext2D,
            color: &str,
        );

        fn canvas_native_context_set_shadow_color_rgba(
            context: &mut CanvasRenderingContext2D,
            r: u8,
            g: u8,
            b: u8,
            a: u8,
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

        fn canvas_native_context_get_text_align(context: &CanvasRenderingContext2D) -> &str;

        fn canvas_native_context_set_text_align(
            context: &mut CanvasRenderingContext2D,
            alignment: &str,
        );

        fn canvas_native_context_get_global_composition(context: &CanvasRenderingContext2D)
            -> &str;

        fn canvas_native_context_set_global_composition(
            context: &CanvasRenderingContext2D,
            composition: &str,
        );

        fn canvas_native_paint_style_set_fill_color_with_string(
            context: &mut CanvasRenderingContext2D,
            color: &str,
        );

        fn canvas_native_parse_css_color_rgba(
            value: &str,
            r: &mut u8,
            g: &mut u8,
            b: &mut u8,
            a: &mut u8,
        ) -> bool;

        fn canvas_native_paint_style_set_stroke_color_with_rgba(
            context: &mut CanvasRenderingContext2D,
            r: u8,
            g: u8,
            b: u8,
            a: u8,
        );

        fn canvas_native_paint_style_set_fill_color_with_rgba(
            context: &mut CanvasRenderingContext2D,
            r: u8,
            g: u8,
            b: u8,
            a: u8,
        );

        fn canvas_native_paint_style_set_stroke_color_with_string(
            context: &mut CanvasRenderingContext2D,
            color: &str,
        );

        fn canvas_native_paint_style_get_color_string(color: &mut PaintStyle) -> String;

        fn canvas_native_paint_style_get_current_stroke_color_string(
            context: &mut CanvasRenderingContext2D,
        ) -> String;

        fn canvas_native_paint_style_get_current_stroke_color_buf(
            context: &mut CanvasRenderingContext2D,
        ) -> Vec<u8>;

        fn canvas_native_paint_style_get_current_stroke_color_r_g_b_a(
            context: &mut CanvasRenderingContext2D,
            r: &mut u8,
            g: &mut u8,
            b: &mut u8,
            a: &mut u8,
        );

        fn canvas_native_paint_style_get_current_fill_color_string(
            context: &mut CanvasRenderingContext2D,
        ) -> String;

        fn canvas_native_paint_style_get_current_fill_color_buf(
            context: &mut CanvasRenderingContext2D,
        ) -> Vec<u8>;

        fn canvas_native_paint_style_get_current_fill_color_r_g_b_a(
            context: &mut CanvasRenderingContext2D,
            r: &mut u8,
            g: &mut u8,
            b: &mut u8,
            a: &mut u8,
        );

        fn canvas_native_context_get_style_type(style: &PaintStyle) -> PaintStyleType;

        fn canvas_native_context_get_current_fill_style_type(
            context: &mut CanvasRenderingContext2D,
        ) -> PaintStyleType;

        fn canvas_native_context_get_current_stroke_style_type(
            context: &mut CanvasRenderingContext2D,
        ) -> PaintStyleType;

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

        fn canvas_native_context_create_pattern_bytes(
            context: &mut CanvasRenderingContext2D,
            bytes: i64,
            repetition: &str,
        ) -> Box<PaintStyle>;

        fn canvas_native_context_create_pattern_asset(
            context: &mut CanvasRenderingContext2D,
            asset: &mut ImageAsset,
            repetition: &str,
        ) -> Box<PaintStyle>;

        pub fn canvas_native_context_create_pattern_canvas2d(
            source: &mut CanvasRenderingContext2D,
            context: &mut CanvasRenderingContext2D,
            repetition: &str,
        ) -> Box<PaintStyle>;

        #[cfg(feature = "webgl")]
        pub fn canvas_native_context_create_pattern_webgl(
            source: &mut WebGLState,
            context: &mut CanvasRenderingContext2D,
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

impl Into<i32> for ffi::ImageBitmapPremultiplyAlpha {
    fn into(self) -> i32 {
        if self == ffi::ImageBitmapPremultiplyAlpha::Premultiply {
            return 1;
        } else if self == ffi::ImageBitmapPremultiplyAlpha::None {
            return 2;
        }
        return 0;
    }
}

impl Into<i32> for ffi::ImageBitmapColorSpaceConversion {
    fn into(self) -> i32 {
        if self == ffi::ImageBitmapColorSpaceConversion::None {
            return 1;
        }
        0
    }
}

impl Into<i32> for ffi::ImageBitmapResizeQuality {
    fn into(self) -> i32 {
        if self == ffi::ImageBitmapResizeQuality::Medium {
            return 1;
        } else if self == ffi::ImageBitmapResizeQuality::High {
            return 2;
        } else if self == ffi::ImageBitmapResizeQuality::Pixelated {
            return 3;
        }
        0
    }
}

fn canvas_native_context_create_with_wrapper(context: i64) -> Box<CanvasRenderingContext2D> {
    let mut wrapper: *mut ContextWrapper = unsafe { context as _ };
    let mut wrapper = unsafe { &mut *wrapper };
    let mut alpha = false;
    {
        let lock = wrapper.get_context();
        alpha = lock.device().alpha;
    }
    let clone = ContextWrapper::from_inner(Arc::clone(wrapper.get_inner()));
    let ctx = GLContext::get_current();
    Box::new(CanvasRenderingContext2D {
        context: clone,
        gl_context: ctx,
        alpha,
    })
}

fn canvas_native_context_resize(context: &mut CanvasRenderingContext2D, width: f32, height: f32) {
    context.resize(width, height);
}

fn canvas_native_context_create_with_current(
    width: f32,
    height: f32,
    scale: f32,
    font_color: i32,
    ppi: f32,
    direction: u32,
    alpha: bool,
) -> Box<CanvasRenderingContext2D> {
    unsafe {
        gl_bindings::glViewport(0, 0, width as i32, height as i32);
    }
    let mut frame_buffers = [0];
    unsafe {
        gl_bindings::glGetIntegerv(
            gl_bindings::GL_FRAMEBUFFER_BINDING,
            frame_buffers.as_mut_ptr(),
        )
    };
    canvas_native_context_create_gl(
        width,
        height,
        scale,
        frame_buffers[0],
        0,
        alpha,
        font_color,
        ppi,
        direction,
    )
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
        alpha,
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
    let ret = Box::new(CanvasRenderingContext2D {
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
        alpha,
    });
    ret.gl_context.swap_buffers();
    ret
}

#[cfg(feature = "webgl")]
pub fn canvas_native_context_create_gl_no_window(
    width: f32,
    height: f32,
    density: f32,
    font_color: i32,
    ppi: f32,
    direction: u32,
    alpha: bool,
) -> Box<CanvasRenderingContext2D> {
    crate::webgl::canvas_native_webgl_create_no_window_internal(
        width as i32,
        height as i32,
        "canvas",
        alpha,
        false,
        false,
        false,
        "default",
        true,
        false,
        false,
        false,
        false,
        true,
    );

    let mut buffer_id = [0i32];

    unsafe {
        gl_bindings::glGetIntegerv(gl_bindings::GL_FRAMEBUFFER_BINDING, buffer_id.as_mut_ptr())
    }

    let gl_context = GLContext::get_current();

    let context = ContextWrapper::new(Context::new_gl(
        width,
        height,
        density,
        buffer_id[0],
        0,
        alpha,
        font_color,
        ppi,
        TextDirection::from(direction),
    ));

    Box::new(CanvasRenderingContext2D {
        context,
        gl_context,
        alpha,
    })
}

fn canvas_native_context_get_font(context: &CanvasRenderingContext2D) -> String {
    context.get_context().font().to_string()
}

fn canvas_native_context_set_font(context: &mut CanvasRenderingContext2D, font: &str) {
    context.get_context_mut().set_font(font);
}

pub fn canvas_native_context_get_global_alpha(context: &CanvasRenderingContext2D) -> f32 {
    context.get_context().global_alpha()
}

pub fn canvas_native_context_set_global_alpha(context: &mut CanvasRenderingContext2D, alpha: f32) {
    context.get_context_mut().set_global_alpha(alpha);
}

pub fn canvas_native_context_get_image_smoothing_enabled(
    context: &CanvasRenderingContext2D,
) -> bool {
    context.get_context().get_image_smoothing_enabled()
}

pub fn canvas_native_context_set_image_smoothing_enabled(
    context: &mut CanvasRenderingContext2D,
    enabled: bool,
) {
    context
        .get_context_mut()
        .set_image_smoothing_enabled(enabled)
}

pub fn canvas_native_context_get_image_smoothing_quality(
    context: &CanvasRenderingContext2D,
) -> &str {
    match context.get_context().get_image_smoothing_quality() {
        ImageSmoothingQuality::Low => "low",
        ImageSmoothingQuality::Medium => "medium",
        ImageSmoothingQuality::High => "high",
    }
}

pub fn canvas_native_context_set_image_smoothing_quality(
    context: &mut CanvasRenderingContext2D,
    quality: &str,
) {
    match quality {
        "low" => context
            .get_context_mut()
            .set_image_smoothing_quality(ImageSmoothingQuality::Low),
        "medium" => context
            .get_context_mut()
            .set_image_smoothing_quality(ImageSmoothingQuality::Medium),
        "high" => context
            .get_context_mut()
            .set_image_smoothing_quality(ImageSmoothingQuality::High),
        _ => {}
    }
}

pub fn canvas_native_context_get_line_join(context: &CanvasRenderingContext2D) -> &str {
    match context.get_context().line_join() {
        LineJoin::JoinBevel => "bevel",
        LineJoin::JoinMiter => "miter",
        LineJoin::JoinRound => "round",
    }
}

pub fn canvas_native_context_set_line_join(context: &mut CanvasRenderingContext2D, join: &str) {
    match join {
        "bevel" => context.get_context_mut().set_line_join(LineJoin::JoinBevel),
        "miter" => context.get_context_mut().set_line_join(LineJoin::JoinMiter),
        "round" => context.get_context_mut().set_line_join(LineJoin::JoinRound),
        _ => {}
    }
}

pub fn canvas_native_context_get_line_cap(context: &CanvasRenderingContext2D) -> &str {
    match context.get_context().line_cap() {
        LineCap::CapRound => "round",
        LineCap::CapButt => "butt",
        LineCap::CapSquare => "square",
    }
}

pub fn canvas_native_context_set_line_cap(context: &mut CanvasRenderingContext2D, cap: &str) {
    match cap {
        "round" => context.get_context_mut().set_line_cap(LineCap::CapRound),
        "butt" => context.get_context_mut().set_line_cap(LineCap::CapButt),
        "square" => context.get_context_mut().set_line_cap(LineCap::CapSquare),
        _ => {}
    }
}

pub fn canvas_native_context_get_miter_limit(context: &CanvasRenderingContext2D) -> f32 {
    context.get_context().miter_limit()
}

pub fn canvas_native_context_set_miter_limit(context: &mut CanvasRenderingContext2D, limit: f32) {
    context.get_context_mut().set_miter_limit(limit);
}

pub fn canvas_native_context_get_shadow_color(context: &CanvasRenderingContext2D) -> String {
    let value = context.get_context().shadow_color();
    to_parsed_color(value)
}

pub fn canvas_native_context_get_shadow_color_buf(context: &CanvasRenderingContext2D) -> Vec<u8> {
    let value = context.get_context().shadow_color();
    to_parsed_color(value).into_bytes()
}

pub fn canvas_native_context_get_shadow_color_rgba(
    context: &CanvasRenderingContext2D,
    r: &mut u8,
    g: &mut u8,
    b: &mut u8,
    a: &mut u8,
) {
    context.get_context().shadow_color_rgba(r, g, b, a);
}

pub fn canvas_native_context_set_shadow_color(context: &mut CanvasRenderingContext2D, color: &str) {
    let mut lock = context.get_context_mut();
    if let Some(color) = parse_color(color) {
        lock.set_shadow_color(color);
    }
}

fn canvas_native_context_set_shadow_color_rgba(
    context: &mut CanvasRenderingContext2D,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) {
    let mut lock = context.get_context_mut();
    lock.set_shadow_color_rgba(r, g, b, a);
}

pub fn canvas_native_context_get_shadow_blur(context: &CanvasRenderingContext2D) -> f32 {
    context.get_context().shadow_blur()
}

pub fn canvas_native_context_set_shadow_blur(context: &mut CanvasRenderingContext2D, blur: f32) {
    context.get_context_mut().set_shadow_blur(blur)
}

pub fn canvas_native_context_get_shadow_offset_x(context: &CanvasRenderingContext2D) -> f32 {
    context.get_context().shadow_offset_x()
}

pub fn canvas_native_context_set_shadow_offset_x(context: &mut CanvasRenderingContext2D, x: f32) {
    context.get_context_mut().set_shadow_offset_x(x)
}

pub fn canvas_native_context_get_shadow_offset_y(context: &CanvasRenderingContext2D) -> f32 {
    context.get_context().shadow_offset_y()
}

pub fn canvas_native_context_set_shadow_offset_y(context: &mut CanvasRenderingContext2D, y: f32) {
    context.get_context_mut().set_shadow_offset_y(y)
}

pub fn canvas_native_context_get_text_align(context: &CanvasRenderingContext2D) -> &str {
    match context.get_context().text_align() {
        TextAlign::START => "start",
        TextAlign::LEFT => "left",
        TextAlign::CENTER => "center",
        TextAlign::RIGHT => "right",
        TextAlign::END => "end",
    }
}

pub fn canvas_native_context_set_text_align(
    context: &mut CanvasRenderingContext2D,
    alignment: &str,
) {
    match alignment {
        "start" => context.get_context_mut().set_text_align(TextAlign::START),
        "left" => context.get_context_mut().set_text_align(TextAlign::LEFT),
        "center" => context.get_context_mut().set_text_align(TextAlign::CENTER),
        "right" => context.get_context_mut().set_text_align(TextAlign::RIGHT),
        "end" => context.get_context_mut().set_text_align(TextAlign::END),
        _ => {}
    }
}

pub fn canvas_native_context_get_global_composition(context: &CanvasRenderingContext2D) -> &str {
    context.get_context().global_composite_operation().to_str()
}

pub fn canvas_native_context_set_global_composition(
    context: &CanvasRenderingContext2D,
    composition: &str,
) {
    if let Some(composition) = CompositeOperationType::from_str(composition) {
        context
            .get_context_mut()
            .set_global_composite_operation(composition)
    }
}

#[inline]
pub fn canvas_native_paint_style_set_fill_color_with_string(
    context: &mut CanvasRenderingContext2D,
    color: &str,
) {
    paint_style_set_color_with_string(&mut context.context, true, color);
}

fn canvas_native_paint_style_set_fill_color_with_utf16(
    context: &mut CanvasRenderingContext2D,
    color: &[u16],
) {
    if let Ok(color) = String::from_utf16(color) {
        paint_style_set_color_with_string(&mut context.context, true, color.as_str());
    }
}

#[inline]
pub fn canvas_native_paint_style_set_fill_color_with_c_str(
    context: &mut CanvasRenderingContext2D,
    color: *const c_char,
) {
    if color.is_null() {
        return;
    }
    let color = unsafe { CStr::from_ptr(color) };
    let color = color.to_string_lossy();
    paint_style_set_color_with_string(&mut context.context, true, color.as_ref());
}

#[inline]
pub fn canvas_native_paint_style_set_stroke_color_with_c_str(
    context: &mut CanvasRenderingContext2D,
    color: *const c_char,
) {
    if color.is_null() {
        return;
    }
    let color = unsafe { CStr::from_ptr(color) };
    let color = color.to_string_lossy();
    paint_style_set_color_with_string(&mut context.context, false, color.as_ref());
}

#[inline]
pub fn canvas_native_paint_style_set_stroke_color_with_string(
    context: &mut CanvasRenderingContext2D,
    color: &str,
) {
    paint_style_set_color_with_string(&mut context.context, false, color);
}

fn canvas_native_paint_style_set_stroke_color_with_utf16(
    context: &mut CanvasRenderingContext2D,
    color: &[u16],
) {
    if let Ok(color) = String::from_utf16(color) {
        paint_style_set_color_with_string(&mut context.context, false, color.as_str());
    }
}

#[inline]
pub fn canvas_native_paint_style_set_stroke_color_with_rgba(
    context: &mut CanvasRenderingContext2D,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) {
    canvas_core::context::fill_and_stroke_styles::paint::paint_style_set_color_with_rgba(
        &mut context.context,
        false,
        r,
        g,
        b,
        a,
    );
}

#[inline]
pub fn canvas_native_paint_style_set_fill_color_with_rgba(
    context: &mut CanvasRenderingContext2D,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) {
    canvas_core::context::fill_and_stroke_styles::paint::paint_style_set_color_with_rgba(
        &mut context.context,
        true,
        r,
        g,
        b,
        a,
    );
}

#[inline(always)]
pub fn canvas_native_parse_css_color_rgba(
    value: &str,
    r: &mut u8,
    g: &mut u8,
    b: &mut u8,
    a: &mut u8,
) -> bool {
    canvas_core::utils::color::parse_color_rgba(value, r, g, b, a)
}

#[inline]
pub fn canvas_native_paint_style_get_color_string(color: &mut PaintStyle) -> String {
    if let Some(color) = color.0.as_ref() {
        return match color {
            canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Color(color) => {
                to_parsed_color(*color)
            }
            _ => String::new(),
        };
    }
    return String::new();
}

fn canvas_native_paint_style_get_current_stroke_color_string(
    context: &mut CanvasRenderingContext2D,
) -> String {
    let lock = context.get_context();
    match lock.stroke_style() {
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            to_parsed_color(*stroke)
        }
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            String::with_capacity(0)
        }
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
            String::with_capacity(0)
        }
    }
}

fn canvas_native_paint_style_get_current_stroke_color_buf(
    context: &mut CanvasRenderingContext2D,
) -> Vec<u8> {
    let lock = context.get_context();
    match lock.stroke_style() {
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            to_parsed_color(*stroke).into_bytes()
        }
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            Vec::with_capacity(0)
        }
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
            Vec::with_capacity(0)
        }
    }
}

fn canvas_native_paint_style_get_current_stroke_color_r_g_b_a(
    context: &mut CanvasRenderingContext2D,
    r: &mut u8,
    g: &mut u8,
    b: &mut u8,
    a: &mut u8,
) {
    let lock = context.get_context();
    match lock.stroke_style() {
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            *r = stroke.r();
            *g = stroke.g();
            *b = stroke.b();
            *a = stroke.a();
        }
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {}
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {}
    }
}

fn canvas_native_paint_style_get_current_fill_color_r_g_b_a(
    context: &mut CanvasRenderingContext2D,
    r: &mut u8,
    g: &mut u8,
    b: &mut u8,
    a: &mut u8,
) {
    let lock = context.get_context();
    match lock.fill_style() {
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            *r = stroke.r();
            *g = stroke.g();
            *b = stroke.b();
            *a = stroke.a();
        }
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {}
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {}
    }
}

fn canvas_native_paint_style_get_current_fill_color_string(
    context: &mut CanvasRenderingContext2D,
) -> String {
    let lock = context.get_context();
    match lock.fill_style() {
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            to_parsed_color(*stroke)
        }
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            String::with_capacity(0)
        }
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
            String::with_capacity(0)
        }
    }
}

fn canvas_native_paint_style_get_current_fill_color_buf(
    context: &mut CanvasRenderingContext2D,
) -> Vec<u8> {
    let lock = context.get_context();
    match lock.fill_style() {
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            to_parsed_color(*stroke).into_bytes()
        }
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            Vec::with_capacity(0)
        }
        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
            Vec::with_capacity(0)
        }
    }
}

pub fn canvas_native_context_get_style_type(style: &PaintStyle) -> ffi::PaintStyleType {
    style.style_type()
}

pub fn canvas_native_context_get_current_fill_style_type(
    context: &mut CanvasRenderingContext2D,
) -> ffi::PaintStyleType {
    let lock = context.get_context();
    return match lock.fill_style() {
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

pub fn canvas_native_context_get_current_stroke_style_type(
    context: &mut CanvasRenderingContext2D,
) -> ffi::PaintStyleType {
    let lock = context.get_context();
    return match lock.fill_style() {
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

#[inline]
pub fn canvas_native_context_get_fill_style(context: &CanvasRenderingContext2D) -> Box<PaintStyle> {
    Box::new(PaintStyle(Some(context.get_context().fill_style().clone())))
}

#[inline]
pub fn canvas_native_context_set_fill_style(
    context: &mut CanvasRenderingContext2D,
    style: &PaintStyle,
) {
    if !style.is_empty() {
        context
            .get_context_mut()
            .set_fill_style(style.0.clone().unwrap())
    }
}

#[inline]
pub fn canvas_native_context_get_stroke_style(
    context: &CanvasRenderingContext2D,
) -> Box<PaintStyle> {
    Box::new(PaintStyle(Some(
        context.get_context().stroke_style().clone(),
    )))
}

#[inline]
pub fn canvas_native_context_set_stroke_style(
    context: &mut CanvasRenderingContext2D,
    style: &PaintStyle,
) {
    if !style.is_empty() {
        context
            .get_context_mut()
            .set_stroke_style(style.0.clone().unwrap())
    }
}

pub fn canvas_native_context_get_line_width(context: &CanvasRenderingContext2D) -> f32 {
    context.get_context().line_width()
}

pub fn canvas_native_context_set_line_width(context: &mut CanvasRenderingContext2D, width: f32) {
    context.get_context_mut().set_line_width(width);
}

pub fn canvas_native_context_get_line_dash_offset(context: &CanvasRenderingContext2D) -> f32 {
    context.get_context().line_dash_offset()
}

pub fn canvas_native_context_set_line_dash_offset(
    context: &mut CanvasRenderingContext2D,
    offset: f32,
) {
    context.get_context_mut().set_line_dash_offset(offset)
}

pub fn canvas_native_context_get_line_dash(context: &CanvasRenderingContext2D) -> Vec<f32> {
    context.get_context().line_dash().to_vec()
}

pub fn canvas_native_context_set_line_dash(context: &mut CanvasRenderingContext2D, dash: &[f32]) {
    context.get_context_mut().set_line_dash(dash)
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
        .get_context_mut()
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
    context.get_context_mut().arc_to(x1, y1, x2, y2, radius)
}

#[inline(always)]
pub fn canvas_native_context_begin_path(context: &mut CanvasRenderingContext2D) {
    context.get_context_mut().begin_path()
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
        .get_context_mut()
        .bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y)
}

pub fn canvas_native_context_clear_rect(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    context.make_current();
    context.get_context_mut().clear_rect(x, y, width, height)
}

pub fn canvas_native_context_clip(
    context: &mut CanvasRenderingContext2D,
    path: &mut Path,
    rule: &str,
) {
    if let Ok(rule) = FillRule::try_from(rule) {
        context.make_current();
        context
            .get_context_mut()
            .clip(Some(path.inner_mut()), Some(rule))
    }
}

pub fn canvas_native_context_clip_rule(context: &mut CanvasRenderingContext2D, rule: &str) {
    if let Ok(rule) = FillRule::try_from(rule) {
        context.make_current();
        context.get_context_mut().clip(None, Some(rule))
    }
}

#[inline(always)]
pub fn canvas_native_context_close_path(context: &mut CanvasRenderingContext2D) {
    context.get_context_mut().close_path()
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
    asset: &mut ImageAsset,
    repetition: &str,
) -> Box<PaintStyle> {
    let has_alpha = asset.get_channels() == 4;
    if let Some(bytes) = asset.get_bytes() {
        return if has_alpha {
            Box::new(PaintStyle(Repetition::try_from(repetition).map_or(
                None,
                |repetition| {
                    from_image_slice(bytes, asset.width() as i32, asset.height() as i32).map(|image| {
                        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                            context.get_context().create_pattern(image, repetition),
                        )
                    })
                },
            )))
        } else {
            Box::new(PaintStyle(Repetition::try_from(repetition.as_ref()).map_or(
                None,
                |repetition| {
                    from_bitmap_slice(bytes, asset.width() as i32, asset.height() as i32).map(|image| {
                        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                            context.get_context().create_pattern(image, repetition),
                        )
                    })
                },
            )))
        };
    }
    Box::new(PaintStyle(None))
}

fn canvas_native_context_create_pattern_bytes(
    context: &mut CanvasRenderingContext2D,
    bm: i64,
    repetition: &str,
) -> Box<PaintStyle> {
    Box::new(PaintStyle(Repetition::try_from(repetition).map_or(
        None,
        |repetition| {
            if bm == 0 {
                return None;
            }
            let bm = unsafe { bm as *mut BitmapBytes };
            let mut bm = unsafe { *Box::from_raw(bm) };
            let mut width = 0;
            let mut height = 0;
            {
                if let Some(info) = bm.0.info() {
                    width = info.width();
                    height = info.height();
                }
            }
            if let Some(bytes) = bm.0.data_mut() {
                return from_image_slice(bytes, width as i32, height as i32).map(|image| {
                    canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                        context.get_context().create_pattern(image, repetition),
                    )
                });
            }
            None
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

pub fn canvas_native_context_create_pattern_canvas2d(
    source: &mut CanvasRenderingContext2D,
    context: &mut CanvasRenderingContext2D,
    repetition: &str,
) -> Box<PaintStyle> {
    Box::new(PaintStyle(Repetition::try_from(repetition).map_or(
        None,
        |repetition| {
            context.remove_if_current();
            let mut width = 0i32;
            let mut height = 0i32;
            let mut source_non_gpu = false;
            let mut non_gpu = false;
            {
                let ctx = source.get_context();
                let device = ctx.device();
                width = device.width as i32;
                height = device.height as i32;
                source_non_gpu = device.non_gpu;
            }

            let mut source_ctx = source.get_context_mut();
            let mut buf;
            if !source_non_gpu {
                source.make_current();
                // source.get_context_mut().flush();
                /*   buf = vec![0u8; (width as i32 * height as i32 * 4) as usize];

                unsafe {
                    gl_bindings::glPixelStorei(gl_bindings::GL_UNPACK_ALIGNMENT, 1);
                    //   gl_bindings::glFinish();
                    gl_bindings::glReadPixels(
                        0,
                        0,
                        width as i32,
                        height as i32,
                        gl_bindings::GL_RGBA,
                        gl_bindings::GL_UNSIGNED_BYTE,
                        buf.as_mut_ptr() as *mut c_void,
                    );
                }
                */

                // todo use gpu image created from snapshot ... need single or shared context or transfer to a texture
                return if let Some(data) = source_ctx.read_pixels_to_encoded_data() {
                    to_image_encoded_from_data(data).map(|image| {
                        canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                            context.get_context().create_pattern(image, repetition),
                        )
                    })
                } else {
                    None
                };
            } else {
                buf = source_ctx.read_pixels();
            }

            source.remove_if_current();

            {
                let ctx = context.get_context();
                let device = ctx.device();
                non_gpu = device.non_gpu;
            }

            if !non_gpu {
                context.make_current();
            }

            from_image_slice(buf.as_slice(), width as i32, height as i32).map(|image| {
                canvas_core::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                    context.get_context().create_pattern(image, repetition),
                )
            })
        },
    )))
}

#[cfg(feature = "webgl")]
pub fn canvas_native_context_create_pattern_webgl(
    source: &mut crate::webgl::WebGLState,
    context: &mut CanvasRenderingContext2D,
    repetition: &str,
) -> Box<PaintStyle> {
    Box::new(PaintStyle(Repetition::try_from(repetition).map_or(
        None,
        |repetition| {
            let state = source.get_inner();
            state.make_current();
            let mut width = state.get_drawing_buffer_width();
            let mut height = state.get_drawing_buffer_height();

            let mut buf = vec![0u8; (width * height * 4) as usize];

            unsafe {
                gl_bindings::glFinish();
                gl_bindings::glReadPixels(
                    0,
                    0,
                    width,
                    height,
                    gl_bindings::GL_RGBA,
                    gl_bindings::GL_UNSIGNED_BYTE,
                    buf.as_mut_ptr() as *mut c_void,
                );
            }

            context.make_current();
            from_image_slice(buf.as_slice(), width, height).map(|image| {
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
        context.make_current();
        context.get_context_mut().draw_image_src_xywh_dst_xywh(
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
        context.make_current();
        let width = image.width() as f32;
        let height = image.height() as f32;
        context
            .get_context_mut()
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
        context.make_current();
        let width = image.width() as f32;
        let height = image.height() as f32;
        context.get_context_mut().draw_image_src_xywh_dst_xywh(
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
        context.make_current();
        context.get_context_mut().draw_image_src_xywh_dst_xywh(
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
    let width = asset.width();
    let height = asset.height();
    let has_alpha = asset.get_channels() == 4;
    if let Some(bytes) = asset.get_bytes() {
        if has_alpha {
            if let Some(image) = from_image_slice_no_copy(bytes, width as c_int, height as c_int) {
                context.make_current();
                context.get_context_mut().draw_image_src_xywh_dst_xywh(
                    &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
                )
            }
        } else {
            if let Some(image) = from_bitmap_slice(bytes, width as c_int, height as c_int) {
                context.make_current();
                context.get_context_mut().draw_image_src_xywh_dst_xywh(
                    &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
                )
            }
        }
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
    context.get_context_mut().ellipse(
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
        context.make_current();
        context.get_context_mut().fill_rule(None, rule)
    }
}

pub fn canvas_native_context_fill_with_path(
    context: &mut CanvasRenderingContext2D,
    path: &mut Path,
    rule: &str,
) {
    if let Ok(rule) = FillRule::try_from(rule) {
        context.make_current();
        context
            .get_context_mut()
            .fill_rule(Some(path.inner_mut()), rule)
    }
}

#[inline]
pub fn canvas_native_context_fill_rect(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    context.make_current();
    context
        .get_context_mut()
        .fill_rect_xywh(x, y, width, height);
}

#[inline]
pub fn canvas_native_context_fill_text(
    context: &mut CanvasRenderingContext2D,
    text: &str,
    x: f32,
    y: f32,
    width: f32,
) {
    context.make_current();
    context.get_context_mut().fill_text(text, x, y, width)
}

pub fn canvas_native_context_get_image_data(
    context: &mut CanvasRenderingContext2D,
    sx: f32,
    sy: f32,
    sw: f32,
    sh: f32,
) -> Box<ImageData> {
    context.make_current();
    Box::new(ImageData(
        context.get_context_mut().get_image_data(sx, sy, sw, sh),
    ))
}

pub fn canvas_native_context_get_transform(context: &mut CanvasRenderingContext2D) -> Box<Matrix> {
    context.make_current();
    Box::new(Matrix(canvas_core::context::matrix::Matrix::from(
        context.get_context_mut().get_transform(),
    )))
}

pub fn canvas_native_context_is_point_in_path(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    rule: &str,
) -> bool {
    FillRule::try_from(rule).map_or(false, |rule| {
        context.make_current();
        context.get_context_mut().is_point_in_path(None, x, y, rule)
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
        context.make_current();
        context
            .get_context_mut()
            .is_point_in_path(Some(path.inner()), x, y, rule)
    })
}

pub fn canvas_native_context_is_point_in_stroke(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
) -> bool {
    context.make_current();
    context.get_context_mut().is_point_in_stroke(None, x, y)
}

pub fn canvas_native_context_is_point_in_stroke_with_path(
    context: &mut CanvasRenderingContext2D,
    path: &mut Path,
    x: f32,
    y: f32,
) -> bool {
    context.make_current();
    context
        .get_context_mut()
        .is_point_in_stroke(Some(path.inner()), x, y)
}

#[inline(always)]
pub fn canvas_native_context_line_to(context: &mut CanvasRenderingContext2D, x: f32, y: f32) {
    context.get_context_mut().line_to(x, y)
}

pub fn canvas_native_context_measure_text(
    context: &mut CanvasRenderingContext2D,
    text: &str,
) -> Box<TextMetrics> {
    Box::new(TextMetrics(context.get_context().measure_text(text)))
}

#[inline(always)]
pub fn canvas_native_context_move_to(context: &mut CanvasRenderingContext2D, x: f32, y: f32) {
    context.get_context_mut().move_to(x, y)
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
    context.make_current();
    context.get_context_mut().put_image_data(
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
    context.get_context_mut().quadratic_curve_to(cpx, cpy, x, y)
}

pub fn canvas_native_context_rect(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    context.get_context_mut().rect(x, y, width, height)
}

pub fn canvas_native_context_reset_transform(context: &mut CanvasRenderingContext2D) {
    context.make_current();
    context.get_context_mut().reset_transform()
}

#[inline(always)]
pub fn canvas_native_context_restore(context: &mut CanvasRenderingContext2D) {
    context.get_context_mut().restore()
}

pub fn canvas_native_context_rotate(context: &mut CanvasRenderingContext2D, angle: f32) {
    context.make_current();
    context.get_context_mut().rotate(angle)
}

#[inline(always)]
pub fn canvas_native_context_save(context: &mut CanvasRenderingContext2D) {
    context.get_context_mut().save()
}

pub fn canvas_native_context_scale(context: &mut CanvasRenderingContext2D, x: f32, y: f32) {
    context.make_current();
    context.get_context_mut().scale(x, y)
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
    context.make_current();
    context.get_context_mut().set_transform(a, b, c, d, e, f)
}

pub fn canvas_native_context_set_transform_matrix(
    context: &mut CanvasRenderingContext2D,
    matrix: &mut Matrix,
) {
    context.make_current();
    let matrix = matrix.inner_mut().to_m33();
    context.get_context_mut().set_transform_matrix(&matrix)
}

pub fn canvas_native_context_stroke(context: &mut CanvasRenderingContext2D) {
    context.make_current();
    context.get_context_mut().stroke(None)
}

pub fn canvas_native_context_stroke_with_path(
    context: &mut CanvasRenderingContext2D,
    path: &mut Path,
) {
    context.make_current();
    context.get_context_mut().stroke(Some(path.inner_mut()))
}

pub fn canvas_native_context_stroke_rect(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    context.make_current();
    context
        .get_context_mut()
        .stroke_rect_xywh(x, y, width, height);
}

pub fn canvas_native_context_stroke_text(
    context: &mut CanvasRenderingContext2D,
    text: &str,
    x: f32,
    y: f32,
    width: f32,
) {
    context.make_current();
    context.get_context_mut().stroke_text(text, x, y, width)
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
    context.make_current();
    context.get_context_mut().transform(a, b, c, d, e, f)
}

pub fn canvas_native_context_translate(context: &mut CanvasRenderingContext2D, x: f32, y: f32) {
    context.make_current();
    context.get_context_mut().translate(x, y)
}

pub fn canvas_native_context_flush(context: &CanvasRenderingContext2D) {
    context.make_current();
    context.get_context_mut().flush();
}

pub fn canvas_native_to_data_url(
    context: &mut CanvasRenderingContext2D,
    format: &str,
    quality: i32,
) -> String {
    context.make_current();
    to_data_url(context, format, quality)
}

/* CanvasRenderingContext2D */

/* ImageBitmap */

fn canvas_native_image_bitmap_create_from_asset(
    asset: &mut ImageAsset,
    flip_y: bool,
    premultiply_alpha: ffi::ImageBitmapPremultiplyAlpha,
    color_space_conversion: ffi::ImageBitmapColorSpaceConversion,
    resize_quality: ffi::ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
) -> Box<ImageAsset> {
    Box::new(ImageAsset(
        canvas_core::image_bitmap::create_from_image_asset_src_rect(
            &mut asset.0,
            None,
            flip_y,
            premultiply_alpha.into(),
            color_space_conversion.into(),
            resize_quality.into(),
            resize_width,
            resize_height,
        ),
    ))
}

fn canvas_native_image_bitmap_create_from_asset_src_rect(
    asset: &mut ImageAsset,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    flip_y: bool,
    premultiply_alpha: ffi::ImageBitmapPremultiplyAlpha,
    color_space_conversion: ffi::ImageBitmapColorSpaceConversion,
    resize_quality: ffi::ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
) -> Box<ImageAsset> {
    Box::new(ImageAsset(
        canvas_core::image_bitmap::create_from_image_asset_src_rect(
            &mut asset.0,
            Some((sx, sy, s_width, s_height).into()),
            flip_y,
            premultiply_alpha.into(),
            color_space_conversion.into(),
            resize_quality.into(),
            resize_width,
            resize_height,
        ),
    ))
}

fn canvas_native_image_bitmap_create_from_encoded_bytes(
    bytes: &[u8],
    flip_y: bool,
    premultiply_alpha: ffi::ImageBitmapPremultiplyAlpha,
    color_space_conversion: ffi::ImageBitmapColorSpaceConversion,
    resize_quality: ffi::ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
) -> Box<ImageAsset> {
    Box::new(ImageAsset(
        canvas_core::image_bitmap::create_image_asset_encoded(
            bytes,
            None,
            flip_y,
            premultiply_alpha.into(),
            color_space_conversion.into(),
            resize_quality.into(),
            resize_width,
            resize_height,
        ),
    ))
}

fn canvas_native_image_bitmap_create_from_encoded_bytes_with_output(
    bytes: &[u8],
    flip_y: bool,
    premultiply_alpha: ffi::ImageBitmapPremultiplyAlpha,
    color_space_conversion: ffi::ImageBitmapColorSpaceConversion,
    resize_quality: ffi::ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
    output: &mut ImageAsset,
) -> bool {
    canvas_core::image_bitmap::create_image_asset_with_output(
        bytes,
        None,
        flip_y,
        premultiply_alpha.into(),
        color_space_conversion.into(),
        resize_quality.into(),
        resize_width,
        resize_height,
        &mut output.0,
    );

    if output.width() > 0 && output.height() > 0 {
        return true;
    }
    false
}

fn canvas_native_image_bitmap_create_from_encoded_bytes_src_rect(
    bytes: &[u8],
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    flip_y: bool,
    premultiply_alpha: ffi::ImageBitmapPremultiplyAlpha,
    color_space_conversion: ffi::ImageBitmapColorSpaceConversion,
    resize_quality: ffi::ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
) -> Box<ImageAsset> {
    Box::new(ImageAsset(
        canvas_core::image_bitmap::create_image_asset_encoded(
            bytes,
            Some((sx, sy, s_width, s_height).into()),
            flip_y,
            premultiply_alpha.into(),
            color_space_conversion.into(),
            resize_quality.into(),
            resize_width,
            resize_height,
        ),
    ))
}

fn canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(
    bytes: &[u8],
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    flip_y: bool,
    premultiply_alpha: ffi::ImageBitmapPremultiplyAlpha,
    color_space_conversion: ffi::ImageBitmapColorSpaceConversion,
    resize_quality: ffi::ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
    output: &mut ImageAsset,
) -> bool {
    canvas_core::image_bitmap::create_image_asset_with_output(
        bytes,
        Some((sx, sy, s_width, s_height).into()),
        flip_y,
        premultiply_alpha.into(),
        color_space_conversion.into(),
        resize_quality.into(),
        resize_width,
        resize_height,
        &mut output.0,
    );

    if output.width() > 0 && output.height() > 0 {
        return true;
    }
    false
}

/* ImageBitmap */

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

#[inline(always)]
pub fn canvas_native_path_create() -> Box<Path> {
    Box::new(Path::default())
}

#[inline(always)]
pub fn canvas_native_path_create_with_path(path: &Path) -> Box<Path> {
    Box::new(path.clone())
}

#[inline(always)]
pub fn canvas_native_path_create_with_string(string: String) -> Box<Path> {
    Box::new(Path(canvas_core::context::paths::path::Path::from_str(
        string.as_str(),
    )))
}

#[inline(always)]
pub fn canvas_native_path_create_with_str(string: &str) -> Box<Path> {
    let path = Path(canvas_core::context::paths::path::Path::from_str(string));
    Box::new(path)
}

#[inline(always)]
pub fn canvas_native_path_close_path(path: &mut Path) {
    path.0.close_path()
}

#[inline(always)]
pub fn canvas_native_path_move_to(path: &mut Path, x: f32, y: f32) {
    path.0.move_to(x, y)
}

#[inline(always)]
pub fn canvas_native_path_line_to(path: &mut Path, x: f32, y: f32) {
    path.0.line_to(x, y)
}

#[inline(always)]
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

#[inline(always)]
pub fn canvas_native_path_quadratic_curve_to(path: &mut Path, cpx: f32, cpy: f32, x: f32, y: f32) {
    path.0.quadratic_curve_to(cpx, cpy, x, y)
}

#[inline(always)]
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

#[inline(always)]
pub fn canvas_native_path_arc_to(path: &mut Path, x1: f32, y1: f32, x2: f32, y2: f32, radius: f32) {
    path.0.arc_to(x1, y1, x2, y2, radius)
}

#[inline(always)]
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

#[inline(always)]
pub fn canvas_native_path_rect(path: &mut Path, x: f32, y: f32, width: f32, height: f32) {
    path.0.rect(x, y, width, height)
}

#[inline(always)]
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

#[derive(Clone)]
pub struct ImageAsset(pub(crate) canvas_core::image_asset::ImageAsset);

unsafe impl ExternType for ImageAsset {
    type Id = type_id!("ImageAsset");
    type Kind = cxx::kind::Trivial;
}

impl Default for ImageAsset {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl ImageAsset {
    pub fn width(&self) -> c_uint {
        self.0.width()
    }

    pub fn height(&self) -> c_uint {
        self.0.height()
    }

    pub fn save_path(&mut self, path: &str, format: OutputFormat) -> bool {
        self.0.save_path(path, format)
    }

    pub fn size(&self) -> usize {
        self.0.size()
    }

    pub fn load_from_reader<R>(&mut self, reader: &mut R) -> bool
    where
        R: Read + std::io::Seek,
    {
        self.0.load_from_reader(reader)
    }

    pub fn load_from_path(&mut self, path: &str) -> bool {
        self.0.load_from_path(path)
    }

    pub fn load_from_bytes(&mut self, buf: &[u8]) -> bool {
        self.0.load_from_bytes(buf)
    }

    pub fn scale(&mut self, x: c_uint, y: c_uint) -> bool {
        self.0.scale(x, y)
    }

    pub fn get_bytes(&self) -> Option<&[u8]> {
        self.0.get_bytes()
    }

    pub fn get_channels(&self) -> i32 {
        //self.0.get_channels()
        // always rgba
        4
    }

    pub fn error(&self) -> Cow<'_, str> {
        self.0.error()
    }

    pub fn set_error(&mut self, error: &str) {
        self.0.set_error(error);
    }
}

pub fn canvas_native_image_asset_create() -> Box<ImageAsset> {
    Box::new(ImageAsset::default())
}

pub fn canvas_native_image_asset_shared_clone(asset: &ImageAsset) -> Box<ImageAsset> {
    Box::new(asset.clone())
}

fn canvas_native_image_asset_get_size(asset: &ImageAsset) -> usize {
    asset.size()
}

pub fn canvas_native_image_asset_load_from_path(asset: &mut ImageAsset, path: &str) -> bool {
    asset.load_from_path(path)
}

pub fn canvas_native_image_asset_load_from_raw(asset: &mut ImageAsset, array: &[u8]) -> bool {
    asset.load_from_bytes(array)
}

pub fn canvas_native_image_asset_load_from_url(asset: &mut ImageAsset, url: &str) -> bool {
    canvas_native_image_asset_load_from_url_internal(&mut asset.0, url)
}

pub(crate) fn canvas_native_image_asset_load_from_url_internal(
    asset: &mut canvas_core::context::image_asset::ImageAsset,
    url: &str,
) -> bool {
    use std::io::prelude::*;

    let resp = ureq::AgentBuilder::new()
        .redirects(10)
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .get(url);

    if let Ok(res) = resp.call() {
        if res.status() != 200 {
            return false;
        }
        assert!(res.has("Content-Length"));
        let mut len: usize = 0;
        if let Ok(length) = res.header("Content-Length").unwrap().parse::<usize>() {
            len = length;
        } else {
            return false;
        }

        let mut bytes: Vec<u8> = Vec::with_capacity(len);
        if let Ok(_) = res.into_reader().read_to_end(&mut bytes) {
            assert_eq!(bytes.len(), len);
        } else {
            return false;
        }

        return asset.load_from_bytes(bytes.as_slice());
    }
    false
}

// pub fn canvas_native_image_asset_get_bytes(asset: &mut ImageAsset) -> Option<&[u8]> {
//     asset.get_bytes()
// }

pub fn canvas_native_image_asset_addr(asset: &mut ImageAsset) -> i64 {
    asset as *mut ImageAsset as i64
}

pub fn canvas_native_image_asset_width(asset: &mut ImageAsset) -> u32 {
    asset.width()
}

pub fn canvas_native_image_asset_height(asset: &mut ImageAsset) -> u32 {
    asset.height()
}

pub fn canvas_native_image_asset_get_error(asset: &mut ImageAsset) -> String {
    asset.error().to_string()
}

pub fn canvas_native_image_asset_has_error(asset: &mut ImageAsset) -> bool {
    if asset.error().is_empty() {
        return false;
    }
    true
}

pub fn canvas_native_image_asset_scale(asset: &mut ImageAsset, x: u32, y: u32) -> bool {
    asset.scale(x, y)
}

pub fn canvas_native_image_asset_save_path(
    asset: &mut ImageAsset,
    path: &str,
    format: u32,
) -> bool {
    asset.save_path(
        path,
        canvas_core::context::image_asset::OutputFormat::from(format),
    )
}

#[derive(Debug, Clone)]
pub struct LooperData {
    looper: *mut looper::ALooper,
}

unsafe impl Send for LooperData {}

#[derive(Debug, Clone)]
struct ThreadCallbackData {
    id: i64,
    done: bool,
}

impl ThreadCallbackData {
    pub fn new(id: i64, done: bool) -> Self {
        Self { id, done }
    }
}

impl Default for ThreadCallbackData {
    fn default() -> Self {
        Self { id: 0, done: false }
    }
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
    decoder.0.encoding().to_string().to_lowercase()
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
    encoder.0.encoding().to_string().to_lowercase()
}

/* TextEncoder */

/* GL */
pub fn canvas_native_context_gl_make_current(context: &CanvasRenderingContext2D) -> bool {
    context.gl_context.make_current()
}

pub fn canvas_native_context_gl_swap_buffers(context: &CanvasRenderingContext2D) -> bool {
    context.gl_context.swap_buffers()
}
/* GL */
