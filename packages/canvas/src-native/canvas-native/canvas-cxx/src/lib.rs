#![allow(non_snake_case)]

use std::borrow::Cow;
use std::cell::{Ref, RefCell, RefMut};
use std::ffi::CStr;
use std::io::{Read, Write};
use std::os::raw::c_ulong;
use std::os::raw::c_void;
use std::os::raw::{c_char, c_int, c_uint};
use std::rc::Rc;
use std::sync::Arc;

use cxx::{type_id, ExternType};
use parking_lot::lock_api::{RwLockReadGuard, RwLockWriteGuard};
use parking_lot::{RawRwLock, RwLock};

use canvas_2d::context::compositing::composite_operation_type::CompositeOperationType;
use canvas_2d::context::drawing_paths::fill_rule::FillRule;
use canvas_2d::context::fill_and_stroke_styles::paint::paint_style_set_color_with_string;
pub use canvas_2d::context::fill_and_stroke_styles::pattern::Repetition;
use canvas_2d::context::image_smoothing::ImageSmoothingQuality;
use canvas_2d::context::line_styles::line_cap::LineCap;
use canvas_2d::context::line_styles::line_join::LineJoin;
use canvas_2d::context::non_standard::PointMode;
use canvas_2d::context::text_styles::text_align::TextAlign;
use canvas_2d::context::text_styles::text_direction::TextDirection;
use canvas_2d::context::{Context, ContextWrapper};
use canvas_2d::utils::color::{parse_color, to_parsed_color};
use canvas_2d::utils::image::{
    from_bitmap_slice, from_image_encoded_data, from_image_slice, from_image_slice_encoded,
    from_image_slice_no_copy, to_image_encoded_from_data,
};
use canvas_core::image_asset::OutputFormat;
use canvas_webgl::prelude::WebGLVersion;
#[cfg(target_os = "android")]
use once_cell::sync::OnceCell;

/* Utils */

pub fn console_log(_text: &str) {}

pub fn str_to_buf(value: &str) -> Vec<u8> {
    value.as_bytes().to_vec()
}

#[cfg(target_os = "android")]
pub static API_LEVEL: OnceCell<i32> = OnceCell::new();

#[cfg(target_os = "android")]
pub mod choregrapher;

#[cfg(any(target_os = "android", target_os = "ios"))]
mod raf;

/* Raf */
#[cfg(any(target_os = "android", target_os = "ios"))]
#[derive(Clone)]
pub struct Raf(raf::Raf);
/* Raf */


#[derive(Clone)]
pub struct ImageFilter(canvas_2d::context::filters::ImageFilter);

/* Helpers */

#[derive(Clone, Default)]
pub struct FileHelper {
    data: Option<Vec<u8>>,
    error: Option<String>,
}

fn canvas_native_helper_read_file(path: &str) -> Box<FileHelper> {
    let mut ret = FileHelper::default();
    match std::fs::File::open(path) {
        Ok(mut file) => {
            let mut buf = Vec::new();
            let _ = file.read_to_end(&mut buf);
            ret.data = Some(buf);
        }
        Err(error) => ret.error = Some(error.to_string()),
    }
    Box::new(ret)
}

fn canvas_native_helper_read_file_has_error(file: &FileHelper) -> bool {
    return file.error.is_some();
}

fn canvas_native_helper_read_file_get_data(file: Box<FileHelper>) -> Vec<u8> {
    match file.data {
        None => Vec::new(),
        Some(data) => data,
    }
}

fn canvas_native_helper_read_file_get_error(file: &FileHelper) -> String {
    match file.error.as_ref() {
        None => String::new(),
        Some(data) => data.to_string(),
    }
}

/* Helpers */

/* Utils */

/* TextEncoder */
#[derive(Clone)]
pub struct TextEncoder(canvas_2d::context::text_encoder::TextEncoder);
/* TextEncoder */

/* TextDecoder */
#[derive(Clone)]
pub struct TextDecoder(canvas_2d::context::text_decoder::TextDecoder);
/* TextDecoder */

/* CanvasRenderingContext2D */

#[allow(dead_code)]
pub struct CanvasRenderingContext2D {
    context: ContextWrapper,
    pub(crate) gl_context: canvas_core::gl::GLContext,
    alpha: bool,
}

fn to_data_url(context: &mut CanvasRenderingContext2D, format: &str, quality: i32) -> String {
    canvas_2d::to_data_url(&mut context.context, format, quality)
}

impl CanvasRenderingContext2D {
    pub fn new(
        context: ContextWrapper,
        gl_context: canvas_core::gl::GLContext,
        alpha: bool,
    ) -> Self {
        Self {
            context,
            gl_context,
            alpha,
        }
    }

    pub fn render(&self) {
        self.gl_context.make_current();
        self.get_context_mut().flush();
        self.gl_context.swap_buffers();
    }

    pub fn get_context(&self) -> Ref<Context> {
        self.context.get_context()
    }

    pub fn get_context_mut(&self) -> RefMut<Context> {
        self.context.get_context_mut()
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.make_current();
        self.context.resize_gl(width, height);
    }

    pub fn make_current(&self) -> bool {
        self.gl_context.make_current()
    }

    pub fn swap_buffers(&self) -> bool {
        self.gl_context.swap_buffers()
    }

    pub fn remove_if_current(&self) {
        self.gl_context.remove_if_current();
    }
}

#[derive(Clone)]
pub struct PaintStyle(
    pub(crate) Option<canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle>,
);

impl PaintStyle {
    // useful when creating styles using platform images e.g Bitmap/UIImage ... test??
    pub fn from_ptr(ptr: i64) -> Self {
        if ptr == 0 {
            return Self::new(None);
        }

        unsafe { *Box::from_raw(ptr as *mut PaintStyle) }
    }

    pub fn new_with_color(color: &str) -> Self {
        Self(canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::new_color_str(color))
    }

    pub fn new(
        style: Option<canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle>,
    ) -> Self {
        Self(style)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    #[inline]
    pub fn style_type(&self) -> ffi::PaintStyleType {
        if let Some(style) = self.0.as_ref() {
            return match style {
                canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(_) => {
                    ffi::PaintStyleType::Color
                }
                canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
                    ffi::PaintStyleType::Gradient
                }
                canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
                    ffi::PaintStyleType::Pattern
                }
            };
        }
        return ffi::PaintStyleType::None;
    }
}

#[derive(Clone, Copy)]
pub struct TextMetrics(canvas_2d::context::drawing_text::text_metrics::TextMetrics);

unsafe impl ExternType for CanvasRenderingContext2D {
    type Id = type_id!("org::nativescript::canvas::CanvasRenderingContext2D");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for ImageAsset {
    type Id = type_id!("org::nativescript::canvas::ImageAsset");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for PaintStyle {
    type Id = type_id!("org::nativescript::canvas::PaintStyle");
    type Kind = cxx::kind::Trivial;
}

/* Raf */
#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn canvas_native_raf_create(callback: isize) -> Box<Raf> {
    Box::new(Raf(raf::Raf::new(Some(Box::new(move |ts| {
        ffi::OnRafCallbackOnFrame(callback, ts);
    })))))
}

#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn canvas_native_raf_start(raf: &mut Raf) {
    raf.0.start();
}

#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn canvas_native_raf_stop(raf: &mut Raf) {
    raf.0.stop()
}

#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn canvas_native_raf_get_started(raf: &Raf) -> bool {
    raf.0.started()
}
/* Raf */

#[cxx::bridge(namespace = "org::nativescript::canvas")]
pub mod ffi {

    extern "Rust" {
        type FileHelper;

        fn canvas_native_helper_read_file(path: &str) -> Box<FileHelper>;

        fn canvas_native_helper_read_file_has_error(file: &FileHelper) -> bool;

        fn canvas_native_helper_read_file_get_data(file: Box<FileHelper>) -> Vec<u8>;

        fn canvas_native_helper_read_file_get_error(file: &FileHelper) -> String;
    }

    extern "Rust" {
        type Raf;
        /* Raf */
        fn canvas_native_raf_create(callback: isize) -> Box<Raf>;
        fn canvas_native_raf_start(raf: &mut Raf);
        fn canvas_native_raf_stop(raf: &mut Raf);
        fn canvas_native_raf_get_started(raf: &Raf) -> bool;
        /* Raf */
    }

    #[allow(non_camel_case_types)]
    #[repr(u32)]
    enum GLConstants {
        UNPACK_FLIP_Y_WEBGL = 0x9240,

        UNPACK_PREMULTIPLY_ALPHA_WEBGL = 0x9241,

        UNPACK_COLORSPACE_CONVERSION_WEBGL = 0x9243,
    }

    pub enum InvalidateState {
        NONE,
        PENDING,
        INVALIDATING,
    }

    pub enum PaintStyleType {
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

    #[namespace = ""]
    unsafe extern "C++" {
        include!("OnRafCallback.h");
        pub(crate) fn OnRafCallbackOnFrame(callback: isize, ts: i64);
    }

    extern "Rust" {
        pub type CanvasRenderingContext2D;
        type PaintStyle;
        type TextMetrics;
        type Path;
        type Matrix;
        type ImageData;
        pub type ImageAsset;
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

        fn canvas_native_path_round_rect(
            path: &mut Path,
            x: f32,
            y: f32,
            width: f32,
            height: f32,
            radii: &[f32],
        );

        fn canvas_native_path_round_rect_tl_tr_br_bl(
            path: &mut Path,
            x: f32,
            y: f32,
            width: f32,
            height: f32,
            top_left: f32,
            top_right: f32,
            bottom_right: f32,
            bottom_left: f32,
        );

        fn canvas_native_path_to_string(path: &Path) -> String;

        /* Path2D */

        /* DOMMatrix */
        fn canvas_native_matrix_create() -> Box<Matrix>;

        fn canvas_native_matrix_update(matrix: &mut Matrix, slice: &[f32]);

        fn canvas_native_matrix_update_3d(matrix: &mut Matrix, slice: &[f32; 16]);

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

        //fn canvas_native_image_asset_get_size(asset: &ImageAsset) -> usize;

        fn canvas_native_image_asset_load_from_fd(asset: &mut ImageAsset, fd: i32) -> bool;

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

        /* PaintStyle */
        fn canvas_native_paint_style_from_bytes(
            context: &CanvasRenderingContext2D,
            repetition: i32,
            width: i32,
            height: i32,
            bytes: &[u8],
        ) -> Box<PaintStyle>;

        fn canvas_native_pattern_from_ptr(ptr: i64) -> Box<PaintStyle>;

        fn canvas_native_paint_style_empty() -> Box<PaintStyle>;
        /* PaintStyle */

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
        fn canvas_native_context_create_with_wrapper(
            context: i64,
            gl_context: i64,
        ) -> Box<CanvasRenderingContext2D>;

        fn canvas_native_context_resize(
            context: &mut CanvasRenderingContext2D,
            width: f32,
            height: f32,
        );

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
            gl_context: i64,
            samples: i32,
            alpha: bool,
            font_color: i32,
            ppi: f32,
            direction: u32,
        ) -> Box<CanvasRenderingContext2D>;

        #[cfg(feature = "webgl")]
        fn canvas_native_context_create_with_pointer(pointer: i64)
            -> Box<CanvasRenderingContext2D>;

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

        fn canvas_native_context_render(context: &CanvasRenderingContext2D);

        fn canvas_native_to_data_url(
            context: &mut CanvasRenderingContext2D,
            format: &str,
            quality: i32,
        ) -> String;

        fn canvas_native_context_get_filter(context: &CanvasRenderingContext2D) -> String;

        fn canvas_native_context_set_filter(context: &mut CanvasRenderingContext2D, font: &str);

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

        fn canvas_native_context_create_image_data_with_data(
            width: i32,
            height: i32,
            data: &[u8],
        ) -> Box<ImageData>;

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
            asset: &mut ImageAsset,
            repetition: &str,
        ) -> Box<PaintStyle>;

        pub fn canvas_native_context_create_pattern_canvas2d(
            source: &mut CanvasRenderingContext2D,
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

        fn canvas_native_context_draw_paint(context: &mut CanvasRenderingContext2D, color: &str);

        fn canvas_native_context_draw_point(context: &mut CanvasRenderingContext2D, x: f32, y: f32);

        fn canvas_native_context_draw_points(
            context: &mut CanvasRenderingContext2D,
            mode: i32,
            points: &[f32],
        );

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

        fn canvas_native_context_draw_image_dx_dy_context(
            context: &mut CanvasRenderingContext2D,
            source: &mut CanvasRenderingContext2D,
            dx: f32,
            dy: f32,
        );

        fn canvas_native_context_draw_image_dx_dy_dw_dh_context(
            context: &mut CanvasRenderingContext2D,
            source: &mut CanvasRenderingContext2D,
            dx: f32,
            dy: f32,
            d_width: f32,
            d_height: f32,
        );

        fn canvas_native_context_draw_image_context(
            context: &mut CanvasRenderingContext2D,
            source: &mut CanvasRenderingContext2D,
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

        fn canvas_native_context_round_rect(
            context: &mut CanvasRenderingContext2D,
            x: f32,
            y: f32,
            width: f32,
            height: f32,
            radii: &[f32],
        );

        fn canvas_native_context_round_rect_tl_tr_br_bl(
            context: &mut CanvasRenderingContext2D,
            x: f32,
            y: f32,
            width: f32,
            height: f32,
            top_left: f32,
            top_right: f32,
            bottom_right: f32,
            bottom_left: f32,
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

    /* GL1 */

    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    pub enum WebGLExtensionType {
        EXT_blend_minmax,
        EXT_color_buffer_half_float,
        EXT_disjoint_timer_query,
        EXT_sRGB,
        EXT_shader_texture_lod,
        EXT_texture_filter_anisotropic,
        OES_element_index_uint,
        OES_standard_derivatives,
        OES_texture_float,
        OES_texture_float_linear,
        OES_texture_half_float,
        OES_texture_half_float_linear,
        OES_vertex_array_object,
        WEBGL_color_buffer_float,
        WEBGL_compressed_texture_atc,
        WEBGL_compressed_texture_etc1,
        WEBGL_compressed_texture_s3tc,
        WEBGL_compressed_texture_s3tc_srgb,
        WEBGL_compressed_texture_etc,
        WEBGL_compressed_texture_pvrtc,
        WEBGL_lose_context,
        ANGLE_instanced_arrays,
        WEBGL_depth_texture,
        WEBGL_draw_buffers,
        None,
    }

    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    pub enum WebGLResultType {
        Boolean,
        I32Array,
        U32Array,
        F32Array,
        BooleanArray,
        U32,
        I32,
        F32,
        String,
        None,
    }

    extern "Rust" {
        type WebGLState;
        type WebGLActiveInfo;
        type WebGLResult;
        type ContextAttributes;
        type WebGLExtension;
        type WebGLFramebufferAttachmentParameter;
        type WebGLShaderPrecisionFormat;
        type EXT_blend_minmax;
        type EXT_color_buffer_half_float;
        type EXT_disjoint_timer_query;
        type EXT_sRGB;
        type EXT_shader_texture_lod;
        type EXT_texture_filter_anisotropic;
        type OES_element_index_uint;
        type OES_standard_derivatives;
        type OES_texture_float;
        type OES_texture_float_linear;
        type OES_texture_half_float;
        type OES_texture_half_float_linear;
        type OES_vertex_array_object;
        type WEBGL_color_buffer_float;
        type WEBGL_compressed_texture_atc;
        type WEBGL_compressed_texture_etc1;
        type WEBGL_compressed_texture_s3tc;
        type WEBGL_compressed_texture_s3tc_srgb;
        type WEBGL_compressed_texture_etc;
        type WEBGL_compressed_texture_pvrtc;
        type WEBGL_lose_context;
        type ANGLE_instanced_arrays;
        type WEBGL_depth_texture;
        type WEBGL_draw_buffers;

        pub fn canvas_native_context_create_pattern_webgl(
            source: &mut WebGLState,
            context: &mut CanvasRenderingContext2D,
            repetition: &str,
        ) -> Box<PaintStyle>;

        /* GL */

        fn canvas_native_webgl_make_current(state: &mut WebGLState) -> bool;
        fn canvas_native_webgl_swap_buffers(state: &mut WebGLState) -> bool;

        fn canvas_native_webgl_to_data_url(
            state: &mut WebGLState,
            format: &str,
            quality: i32,
        ) -> String;

        fn canvas_native_webgl_resized(state: &mut WebGLState);
        /* GL */

        /* WebGLActiveInfo */

        fn canvas_native_webgl_active_info_get_name(info: &WebGLActiveInfo) -> &str;
        fn canvas_native_webgl_active_info_get_size(info: &WebGLActiveInfo) -> i32;
        fn canvas_native_webgl_active_info_get_type(info: &WebGLActiveInfo) -> u32;
        fn canvas_native_webgl_active_info_get_is_empty(info: &WebGLActiveInfo) -> bool;

        /* WebGLActiveInfo */

        /* WebGLShaderPrecisionFormat */
        fn canvas_native_webgl_shader_precision_format_get_range_min(
            shader: &WebGLShaderPrecisionFormat,
        ) -> i32;
        fn canvas_native_webgl_shader_precision_format_get_range_max(
            shader: &WebGLShaderPrecisionFormat,
        ) -> i32;
        fn canvas_native_webgl_shader_precision_format_get_precision(
            shader: &WebGLShaderPrecisionFormat,
        ) -> i32;
        /* WebGLShaderPrecisionFormat */

        /* ContextAttributes */
        fn canvas_native_webgl_context_attribute_get_get_alpha(attr: &ContextAttributes) -> bool;
        fn canvas_native_webgl_context_attribute_get_get_antialias(
            attr: &ContextAttributes,
        ) -> bool;
        fn canvas_native_webgl_context_attribute_get_get_depth(attr: &ContextAttributes) -> bool;
        fn canvas_native_webgl_context_attribute_get_get_fail_if_major_performance_caveat(
            attr: &ContextAttributes,
        ) -> bool;
        fn canvas_native_webgl_context_attribute_get_get_power_preference(
            attr: &ContextAttributes,
        ) -> String;
        fn canvas_native_webgl_context_attribute_get_get_premultiplied_alpha(
            attr: &ContextAttributes,
        ) -> bool;
        fn canvas_native_webgl_context_attribute_get_get_preserve_drawing_buffer(
            attr: &ContextAttributes,
        ) -> bool;
        fn canvas_native_webgl_context_attribute_get_get_stencil(attr: &ContextAttributes) -> bool;
        fn canvas_native_webgl_context_attribute_get_get_desynchronized(
            attr: &ContextAttributes,
        ) -> bool;
        fn canvas_native_webgl_context_attribute_get_get_xr_compatible(
            attr: &ContextAttributes,
        ) -> bool;

        /* ContextAttributes */

        /* WebGLExtension */
        fn canvas_native_webgl_context_extension_is_none(extension: &WebGLExtension) -> bool;
        fn canvas_native_webgl_context_extension_get_type(
            extension: &WebGLExtension,
        ) -> WebGLExtensionType;
        fn canvas_native_webgl_context_extension_to_ext_disjoint_timer_query(
            extension: Box<WebGLExtension>,
        ) -> Box<EXT_disjoint_timer_query>;
        pub fn canvas_native_webgl_context_extension_to_angle_instanced_arrays(
            extension: Box<WebGLExtension>,
        ) -> Box<ANGLE_instanced_arrays>;

        pub fn canvas_native_webgl_context_extension_to_lose_context(
            extension: Box<WebGLExtension>,
        ) -> Box<WEBGL_lose_context>;

        pub fn canvas_native_webgl_context_extension_to_draw_buffers(
            extension: Box<WebGLExtension>,
        ) -> Box<WEBGL_draw_buffers>;

        pub fn canvas_native_webgl_context_extension_to_oes_vertex_array_object(
            extension: Box<WebGLExtension>,
        ) -> Box<OES_vertex_array_object>;

        /* WebGLExtension */

        /* WEBGL_lose_context */

        fn canvas_native_webgl_lose_context_lose_context(context: &WEBGL_lose_context);

        fn canvas_native_webgl_lose_context_restore_context(context: &WEBGL_lose_context);

        /* WEBGL_lose_context */

        /* WEBGL_draw_buffers */

        fn canvas_native_webgl_draw_buffers_draw_buffers_webgl(
            buffers: &[u32],
            context: &WEBGL_draw_buffers,
        );

        /* WEBGL_draw_buffers */

        /* OES_vertex_array_object */

        fn canvas_native_webgl_oes_vertex_array_object_create_vertex_array_oes(
            object: &OES_vertex_array_object,
        ) -> u32;

        fn canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(
            array_object: u32,
            object: &OES_vertex_array_object,
        );

        fn canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(
            array_object: u32,
            object: &OES_vertex_array_object,
        ) -> bool;

        fn canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(
            array_object: u32,
            object: &OES_vertex_array_object,
        );

        /* OES_vertex_array_object */

        /* WebGLFramebufferAttachmentParameter */
        fn canvas_native_webgl_framebuffer_attachment_parameter_get_is_texture(
            param: &WebGLFramebufferAttachmentParameter,
        ) -> bool;
        fn canvas_native_webgl_framebuffer_attachment_parameter_get_is_renderbuffer(
            param: &WebGLFramebufferAttachmentParameter,
        ) -> bool;
        fn canvas_native_webgl_framebuffer_attachment_parameter_get_value(
            param: &WebGLFramebufferAttachmentParameter,
        ) -> i32;
        /* WebGLFramebufferAttachmentParameter */

        /* WebGLResult */

        fn canvas_native_webgl_result_get_type(result: &WebGLResult) -> WebGLResultType;
        fn canvas_native_webgl_result_get_bool(result: &WebGLResult) -> bool;
        fn canvas_native_webgl_result_get_i32_array(result: &WebGLResult) -> Vec<i32>;
        fn canvas_native_webgl_result_get_u32_array(result: &WebGLResult) -> Vec<u32>;
        fn canvas_native_webgl_result_get_f32_array(result: &WebGLResult) -> Vec<f32>;
        fn canvas_native_webgl_result_get_bool_array(result: &WebGLResult) -> Vec<u8>;
        fn canvas_native_webgl_result_get_u32(result: &WebGLResult) -> u32;
        fn canvas_native_webgl_result_get_i32(result: &WebGLResult) -> i32;
        fn canvas_native_webgl_result_get_f32(result: &WebGLResult) -> f32;
        fn canvas_native_webgl_result_get_string(result: &WebGLResult) -> String;
        fn canvas_native_webgl_result_get_is_none(result: &WebGLResult) -> bool;

        /* WebGLResult */

        /* WebGLState */
        fn canvas_native_webgl_state_get_unpack_colorspace_conversion_webgl(
            state: &mut WebGLState,
        ) -> i32;
        fn canvas_native_webgl_state_get_flip_y(state: &mut WebGLState) -> bool;
        fn canvas_native_webgl_state_get_premultiplied_alpha(state: &mut WebGLState) -> bool;
        fn canvas_native_webgl_state_get_drawing_buffer_width(state: &mut WebGLState) -> i32;
        fn canvas_native_webgl_state_get_drawing_buffer_height(state: &mut WebGLState) -> i32;
        /* WebGLState */

        /* EXT_disjoint_timer_query */
        fn canvas_native_webgl_ext_disjoint_timer_query_create_query_ext(
            query: &EXT_disjoint_timer_query,
        ) -> u32;

        fn canvas_native_webgl_ext_disjoint_timer_query_delete_query_ext(
            value: u32,
            query: &EXT_disjoint_timer_query,
        );

        fn canvas_native_webgl_ext_disjoint_timer_query_is_query_ext(
            value: u32,
            query: &EXT_disjoint_timer_query,
        ) -> bool;

        fn canvas_native_webgl_ext_disjoint_timer_query_begin_query_ext(
            target: u32,
            value: u32,
            query: &EXT_disjoint_timer_query,
        );

        fn canvas_native_webgl_ext_disjoint_timer_query_end_query_ext(
            target: u32,
            query: &EXT_disjoint_timer_query,
        );

        fn canvas_native_webgl_ext_disjoint_timer_query_query_counter_ext(
            value: u32,
            target: u32,
            query: &EXT_disjoint_timer_query,
        );

        fn canvas_native_webgl_ext_disjoint_timer_query_get_query_ext(
            target: u32,
            pname: u32,
            query: &EXT_disjoint_timer_query,
        ) -> i32;

        fn canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(
            target: u32,
            pname: u32,
            query: &EXT_disjoint_timer_query,
        ) -> Box<WebGLResult>;

        /* EXT_disjoint_timer_query */

        /* ANGLE_instanced_arrays */
        fn canvas_native_webgl_angle_instanced_arrays_draw_arrays_instanced_angle(
            mode: u32,
            first: i32,
            count: i32,
            primcount: i32,
            arrays: &ANGLE_instanced_arrays,
        );
        fn canvas_native_webgl_angle_instanced_arrays_draw_elements_instanced_angle(
            mode: u32,
            count: i32,
            type_: u32,
            offset: i32,
            primcount: i32,
            arrays: &ANGLE_instanced_arrays,
        );
        fn canvas_native_webgl_angle_instanced_arrays_vertex_attrib_divisor_angle(
            index: u32,
            divisor: u32,
            arrays: &ANGLE_instanced_arrays,
        );
        /* ANGLE_instanced_arrays */

        /* WebGLRenderingContext */

        fn canvas_native_webgl_create(
            gl_context: i64,
            version: &str,
            alpha: bool,
            antialias: bool,
            depth: bool,
            fail_if_major_performance_caveat: bool,
            power_preference: &str,
            premultiplied_alpha: bool,
            preserve_drawing_buffer: bool,
            stencil: bool,
            desynchronized: bool,
            xr_compatible: bool,
        ) -> Box<WebGLState>;

        fn canvas_native_webgl_create_no_window(
            width: i32,
            height: i32,
            version: &str,
            alpha: bool,
            antialias: bool,
            depth: bool,
            fail_if_major_performance_caveat: bool,
            power_preference: &str,
            premultiplied_alpha: bool,
            preserve_drawing_buffer: bool,
            stencil: bool,
            desynchronized: bool,
            xr_compatible: bool,
            is_canvas: bool,
        ) -> Box<WebGLState>;

        fn canvas_native_webgl_active_texture(texture: u32, state: &mut WebGLState);

        fn canvas_native_webgl_attach_shader(program: u32, shader: u32, state: &mut WebGLState);

        fn canvas_native_webgl_bind_attrib_location(
            program: u32,
            index: u32,
            name: &str,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_bind_buffer(target: u32, buffer: u32, state: &mut WebGLState);

        fn canvas_native_webgl_bind_frame_buffer(
            target: u32,
            framebuffer: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_bind_render_buffer(
            target: u32,
            renderbuffer: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_bind_texture(target: u32, texture: u32, state: &mut WebGLState);

        fn canvas_native_webgl_blend_color(
            red: f32,
            green: f32,
            blue: f32,
            alpha: f32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_blend_equation_separate(
            mode_rgb: u32,
            mode_alpha: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_blend_equation(mode: u32, state: &mut WebGLState);

        fn canvas_native_webgl_blend_func_separate(
            src_rgb: u32,
            dst_rgb: u32,
            src_alpha: u32,
            dst_alpha: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_blend_func(sfactor: u32, dfactor: u32, state: &mut WebGLState);

        fn canvas_native_webgl_buffer_data(
            target: u32,
            src_data: &[u8],
            usage: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_buffer_data_u16(
            target: u32,
            src_data: &[u16],
            usage: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_buffer_data_f32(
            target: u32,
            src_data: &[f32],
            usage: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_buffer_data_none(
            target: u32,
            size: isize,
            usage: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_buffer_sub_data(
            target: u32,
            offset: isize,
            src_data: &[u8],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_buffer_sub_data_none(
            target: u32,
            offset: isize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_check_frame_buffer_status(
            target: u32,
            state: &mut WebGLState,
        ) -> u32;

        fn canvas_native_webgl_clear(mask: u32, state: &mut WebGLState);

        fn canvas_native_webgl_clear_color(
            red: f32,
            green: f32,
            blue: f32,
            alpha: f32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_clear_depth(depth: f32, state: &mut WebGLState);

        fn canvas_native_webgl_clear_stencil(stencil: i32, state: &mut WebGLState);

        fn canvas_native_webgl_color_mask(
            red: bool,
            green: bool,
            blue: bool,
            alpha: bool,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_commit(state: &mut WebGLState);

        fn canvas_native_webgl_compile_shader(shader: u32, state: &mut WebGLState);

        fn canvas_native_webgl_compressed_tex_image2d(
            target: u32,
            level: i32,
            internalformat: u32,
            width: i32,
            height: i32,
            border: i32,
            pixels: &[u8],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_compressed_tex_image2d_none(
            target: u32,
            level: i32,
            internalformat: u32,
            width: i32,
            height: i32,
            border: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_compressed_tex_sub_image2d(
            target: u32,
            level: i32,
            xoffset: i32,
            yoffset: i32,
            width: i32,
            height: i32,
            format: u32,
            pixels: &[u8],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_copy_tex_image2d(
            target: u32,
            level: i32,
            internalformat: u32,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            border: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_copy_tex_sub_image2d(
            target: u32,
            level: i32,
            xoffset: i32,
            yoffset: i32,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_create_buffer(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl_create_framebuffer(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl_create_program(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl_create_renderbuffer(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl_create_shader(shader_type: u32, state: &mut WebGLState) -> u32;

        fn canvas_native_webgl_create_texture(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl_cull_face(mode: u32, state: &mut WebGLState);

        fn canvas_native_webgl_delete_buffer(buffer: u32, state: &mut WebGLState);

        fn canvas_native_webgl_delete_framebuffer(frame_buffer: u32, state: &mut WebGLState);

        fn canvas_native_webgl_delete_program(program: u32, state: &mut WebGLState);

        fn canvas_native_webgl_delete_renderbuffer(render_buffer: u32, state: &mut WebGLState);

        fn canvas_native_webgl_delete_shader(shader: u32, state: &mut WebGLState);

        fn canvas_native_webgl_delete_texture(texture: u32, state: &mut WebGLState);

        fn canvas_native_webgl_depth_func(func: u32, state: &mut WebGLState);

        fn canvas_native_webgl_depth_mask(flag: bool, state: &mut WebGLState);

        fn canvas_native_webgl_depth_range(z_near: f32, z_far: f32, state: &mut WebGLState);

        fn canvas_native_webgl_detach_shader(program: u32, shader: u32, state: &mut WebGLState);

        fn canvas_native_webgl_disable(cap: u32, state: &mut WebGLState);

        fn canvas_native_webgl_disable_vertex_attrib_array(index: u32, state: &mut WebGLState);

        fn canvas_native_webgl_draw_arrays(
            mode: u32,
            first: i32,
            count: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_draw_elements(
            mode: u32,
            count: i32,
            element_type: u32,
            offset: isize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_enable(cap: u32, state: &mut WebGLState);

        fn canvas_native_webgl_enable_vertex_attrib_array(index: u32, state: &mut WebGLState);

        fn canvas_native_webgl_finish(state: &mut WebGLState);

        fn canvas_native_webgl_flush(state: &mut WebGLState);

        fn canvas_native_webgl_framebuffer_renderbuffer(
            target: u32,
            attachment: u32,
            renderbuffertarget: u32,
            renderbuffer: u32,
            state: &mut WebGLState,
        );

        pub fn canvas_native_webgl_framebuffer_texture2d(
            target: u32,
            attachment: u32,
            textarget: u32,
            texture: u32,
            level: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_front_face(mode: u32, state: &mut WebGLState);

        fn canvas_native_webgl_generate_mipmap(target: u32, state: &mut WebGLState);

        fn canvas_native_webgl_get_active_attrib(
            program: u32,
            index: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLActiveInfo>;

        fn canvas_native_webgl_get_active_uniform(
            program: u32,
            index: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLActiveInfo>;

        fn canvas_native_webgl_get_attached_shaders(
            program: u32,
            state: &mut WebGLState,
        ) -> Vec<u32>;

        fn canvas_native_webgl_get_attrib_location(
            program: u32,
            name: &str,
            state: &mut WebGLState,
        ) -> i32;

        fn canvas_native_webgl_get_buffer_parameter(
            target: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> i32;

        fn canvas_native_webgl_get_context_attributes(
            state: &mut WebGLState,
        ) -> Box<ContextAttributes>;

        fn canvas_native_webgl_get_error(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl_get_extension(
            name: &str,
            state: &mut WebGLState,
        ) -> Box<WebGLExtension>;

        fn canvas_native_webgl_get_framebuffer_attachment_parameter(
            target: u32,
            attachment: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLFramebufferAttachmentParameter>;

        fn canvas_native_webgl_get_parameter(
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl_get_program_info_log(program: u32, state: &mut WebGLState)
            -> String;

        fn canvas_native_webgl_get_program_parameter(
            program: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl_get_renderbuffer_parameter(
            target: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> i32;

        fn canvas_native_webgl_get_shader_info_log(shader: u32, state: &mut WebGLState) -> String;

        fn canvas_native_webgl_get_shader_parameter(
            shader: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl_get_shader_precision_format(
            shader_type: u32,
            precision_type: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLShaderPrecisionFormat>;

        fn canvas_native_webgl_get_shader_source(shader: u32, state: &mut WebGLState) -> String;

        fn canvas_native_webgl_get_supported_extensions(state: &mut WebGLState) -> Vec<String>;

        fn canvas_native_webgl_get_supported_extensions_to_string(state: &mut WebGLState)
            -> String;

        fn canvas_native_webgl_get_tex_parameter(
            target: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> i32;

        fn canvas_native_webgl_get_uniform_location(
            program: u32,
            name: &str,
            state: &mut WebGLState,
        ) -> i32;

        fn canvas_native_webgl_get_uniform(
            program: u32,
            location: i32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl_get_vertex_attrib_offset(
            index: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> usize;

        fn canvas_native_webgl_get_vertex_attrib(
            index: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl_get_is_context_lost(state: &mut WebGLState) -> bool;

        fn canvas_native_webgl_hint(target: u32, mode: u32, state: &mut WebGLState);

        fn canvas_native_webgl_is_buffer(buffer: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl_is_enabled(cap: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl_is_framebuffer(framebuffer: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl_is_program(program: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl_is_renderbuffer(renderbuffer: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl_is_shader(shader: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl_is_texture(texture: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl_line_width(width: f32, state: &mut WebGLState);

        fn canvas_native_webgl_link_program(program: u32, state: &mut WebGLState);

        fn canvas_native_webgl_pixel_storei(pname: u32, param: i32, state: &mut WebGLState);

        fn canvas_native_webgl_polygon_offset(factor: f32, units: f32, state: &mut WebGLState);

        fn canvas_native_webgl_read_pixels_u8(
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            format: u32,
            pixel_type: u32,
            pixels: &mut [u8],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_read_pixels_u16(
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            format: u32,
            pixel_type: u32,
            pixels: &mut [u16],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_read_pixels_f32(
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            format: u32,
            pixel_type: u32,
            pixels: &mut [f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_renderbuffer_storage(
            target: u32,
            internal_format: u32,
            width: i32,
            height: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_sample_coverage(value: f32, invert: bool, state: &mut WebGLState);

        fn canvas_native_webgl_scissor(
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_shader_source(shader: u32, source: &str, state: &mut WebGLState);

        fn canvas_native_webgl_stencil_func(
            func: u32,
            reference: i32,
            mask: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_stencil_func_separate(
            face: u32,
            func: u32,
            reference: i32,
            mask: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_stencil_mask(mask: u32, state: &mut WebGLState);

        fn canvas_native_webgl_stencil_mask_separate(face: u32, mask: u32, state: &mut WebGLState);

        fn canvas_native_webgl_stencil_op(
            fail: u32,
            zfail: u32,
            zpass: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_stencil_op_separate(
            face: u32,
            fail: u32,
            zfail: u32,
            zpass: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_image2d_image_none(
            target: i32,
            level: i32,
            internalformat: i32,
            format: i32,
            image_type: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_image2d_canvas2d(
            target: i32,
            level: i32,
            internalformat: i32,
            format: i32,
            image_type: i32,
            canvas: &mut CanvasRenderingContext2D,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_image2d_webgl(
            target: i32,
            level: i32,
            internalformat: i32,
            format: i32,
            image_type: i32,
            webgl: &mut WebGLState,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_image2d(
            target: i32,
            level: i32,
            internalformat: i32,
            width: i32,
            height: i32,
            border: i32,
            format: i32,
            image_type: i32,
            buf: &mut [u8],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_image2d_none(
            target: i32,
            level: i32,
            internalformat: i32,
            width: i32,
            height: i32,
            border: i32,
            format: i32,
            image_type: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_image2d_image_asset(
            target: i32,
            level: i32,
            internalformat: i32,
            format: i32,
            image_type: i32,
            image_asset: &mut ImageAsset,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_parameterf(
            target: u32,
            pname: u32,
            param: f32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_parameteri(
            target: u32,
            pname: u32,
            param: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_sub_image2d_asset(
            target: u32,
            level: i32,
            xoffset: i32,
            yoffset: i32,
            format: u32,
            image_type: i32,
            asset: &mut ImageAsset,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_tex_sub_image2d(
            target: u32,
            level: i32,
            xoffset: i32,
            yoffset: i32,
            width: i32,
            height: i32,
            format: u32,
            image_type: i32,
            buf: &[u8],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_uniform1f(location: i32, v0: f32, state: &mut WebGLState);

        fn canvas_native_webgl_uniform1fv(location: i32, value: &[f32], state: &mut WebGLState);

        fn canvas_native_webgl_uniform1i(location: i32, v0: i32, state: &mut WebGLState);

        fn canvas_native_webgl_uniform1iv(location: i32, value: &[i32], state: &mut WebGLState);

        fn canvas_native_webgl_uniform2f(location: i32, v0: f32, v1: f32, state: &mut WebGLState);

        fn canvas_native_webgl_uniform2fv(location: i32, value: &[f32], state: &mut WebGLState);

        fn canvas_native_webgl_uniform2i(location: i32, v0: i32, v1: i32, state: &mut WebGLState);

        fn canvas_native_webgl_uniform2iv(location: i32, value: &[i32], state: &mut WebGLState);

        fn canvas_native_webgl_uniform3f(
            location: i32,
            v0: f32,
            v1: f32,
            v2: f32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_uniform3fv(location: i32, value: &[f32], state: &mut WebGLState);

        fn canvas_native_webgl_uniform3i(
            location: i32,
            v0: i32,
            v1: i32,
            v2: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_uniform3iv(location: i32, value: &[i32], state: &mut WebGLState);

        fn canvas_native_webgl_uniform4f(
            location: i32,
            v0: f32,
            v1: f32,
            v2: f32,
            v3: f32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_uniform4fv(location: i32, value: &[f32], state: &mut WebGLState);

        fn canvas_native_webgl_uniform4i(
            location: i32,
            v0: i32,
            v1: i32,
            v2: i32,
            v3: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_uniform4iv(location: i32, value: &[i32], state: &mut WebGLState);

        fn canvas_native_webgl_uniform_matrix2fv(
            location: i32,
            transpose: bool,
            value: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_uniform_matrix3fv(
            location: i32,
            transpose: bool,
            value: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_uniform_matrix4fv(
            location: i32,
            transpose: bool,
            value: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_use_program(program: u32, state: &mut WebGLState);

        fn canvas_native_webgl_validate_program(program: u32, state: &mut WebGLState);

        fn canvas_native_webgl_vertex_attrib1f(index: u32, v0: f32, state: &mut WebGLState);

        fn canvas_native_webgl_vertex_attrib1fv(index: u32, value: &[f32], state: &mut WebGLState);

        fn canvas_native_webgl_vertex_attrib2f(
            index: u32,
            v0: f32,
            v1: f32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_vertex_attrib2fv(index: u32, value: &[f32], state: &mut WebGLState);

        fn canvas_native_webgl_vertex_attrib3f(
            index: u32,
            v0: f32,
            v1: f32,
            v2: f32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_vertex_attrib3fv(index: u32, value: &[f32], state: &mut WebGLState);

        fn canvas_native_webgl_vertex_attrib4f(
            index: u32,
            v0: f32,
            v1: f32,
            v2: f32,
            v3: f32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_vertex_attrib4fv(index: u32, value: &[f32], state: &mut WebGLState);

        fn canvas_native_webgl_vertex_attrib_pointer(
            index: u32,
            size: i32,
            d_type: u32,
            normalized: bool,
            stride: i32,
            offset: isize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl_viewport(
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            state: &mut WebGLState,
        );

        /* WebGLRenderingContext */
    }

    /* GL1 */

    /* GL2 */

    extern "Rust" {
        type WebGLSync;
        type WebGLIndexedParameter;

        /* WebGL2RenderingContext */

        /* WebGLIndexedParameter */
        pub fn canvas_native_webgl2_indexed_parameter_get_value(
            param: &WebGLIndexedParameter,
        ) -> isize;

        pub fn canvas_native_webgl2_indexed_parameter_get_buffer_value(
            param: &WebGLIndexedParameter,
        ) -> isize;

        pub fn canvas_native_webgl2_indexed_parameter_get_is_buffer(
            param: &WebGLIndexedParameter,
        ) -> bool;

        /* WebGLIndexedParameter */

        fn canvas_native_webgl2_begin_query(target: u32, id: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_begin_transform_feedback(
            primitive_mode: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_bind_buffer_base(
            target: u32,
            index: u32,
            buffer: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_bind_buffer_range(
            target: u32,
            index: u32,
            buffer: u32,
            offset: isize,
            size: isize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_bind_sampler(unit: u32, sampler: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_bind_transform_feedback(
            target: u32,
            transform_feedback: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_bind_vertex_array(vertex_array: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_blit_framebuffer(
            src_x0: i32,
            src_y0: i32,
            src_x1: i32,
            src_y1: i32,
            dst_x0: i32,
            dst_y0: i32,
            dst_x1: i32,
            dst_y1: i32,
            mask: u32,
            filter: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_clear_bufferfi(
            buffer: u32,
            drawbuffer: i32,
            depth: f32,
            stencil: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_clear_bufferfv(
            buffer: u32,
            drawbuffer: i32,
            values: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_clear_bufferiv(
            buffer: u32,
            drawbuffer: i32,
            values: &[i32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_clear_bufferuiv(
            buffer: u32,
            drawbuffer: i32,
            values: &[u32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_client_wait_sync(
            sync: &WebGLSync,
            flags: u32,
            timeout: isize,
            state: &mut WebGLState,
        ) -> u32;

        fn canvas_native_webgl2_compressed_tex_sub_image3d_none(
            target: u32,
            level: i32,
            xoffset: i32,
            yoffset: i32,
            zoffset: i32,
            width: i32,
            height: i32,
            depth: i32,
            format: u32,
            image_size: i32,
            offset: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_compressed_tex_sub_image3d(
            target: u32,
            level: i32,
            xoffset: i32,
            yoffset: i32,
            zoffset: i32,
            width: i32,
            height: i32,
            depth: i32,
            format: u32,
            src: &[u8],
            src_offset: usize,
            src_length_override: usize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_copy_buffer_sub_data(
            read_target: u32,
            write_target: u32,
            read_offset: isize,
            write_offset: isize,
            size: isize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_copy_tex_sub_image3d(
            target: u32,
            level: i32,
            xoffset: i32,
            yoffset: i32,
            zoffset: i32,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_create_query(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl2_create_sampler(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl2_create_transform_feedback(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl2_create_vertex_array(state: &mut WebGLState) -> u32;

        fn canvas_native_webgl2_delete_query_with_query(id: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_delete_sampler_with_sampler(sampler: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_delete_sync_with_sync(sync: &WebGLSync, state: &mut WebGLState);

        fn canvas_native_webgl2_delete_transform_feedback(
            transform_feedback: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_delete_vertex_array_with_vertex_array(
            vertex_array: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_draw_arrays_instanced(
            mode: u32,
            first: i32,
            count: i32,
            instance_count: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_draw_buffers(buffers: &[u32], state: &mut WebGLState);

        fn canvas_native_webgl2_draw_elements_instanced(
            mode: u32,
            count: i32,
            type_: u32,
            offset: isize,
            instance_count: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_draw_range_elements(
            mode: u32,
            start: u32,
            end: u32,
            count: i32,
            type_: u32,
            offset: isize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_end_query(target: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_end_transform_feedback(state: &mut WebGLState);

        fn canvas_native_webgl2_fence_sync(
            condition: u32,
            flags: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLSync>;

        fn canvas_native_webgl2_framebuffer_texture_layer(
            target: u32,
            attachment: u32,
            texture: u32,
            level: i32,
            layer: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_get_active_uniform_block_name(
            program: u32,
            uniform_block_index: u32,
            state: &mut WebGLState,
        ) -> String;

        fn canvas_native_webgl2_get_active_uniform_block_parameter(
            program: u32,
            uniform_block_index: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl2_get_active_uniforms(
            program: u32,
            uniform_indices: &[u32],
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl2_get_buffer_sub_data(
            target: u32,
            src_byte_offset: isize,
            dst_data: &mut [u8],
            dst_offset: usize,
            length: usize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_get_frag_data_location(
            program: u32,
            name: &str,
            state: &mut WebGLState,
        ) -> i32;

        fn canvas_native_webgl2_get_indexed_parameter(
            target: u32,
            index: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLIndexedParameter>;

        fn canvas_native_webgl2_get_internalformat_parameter(
            target: u32,
            internalformat: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl2_get_parameter(
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl2_get_query_parameter(
            query: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl2_get_query(
            target: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl2_get_sampler_parameter(
            sampler: u32,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl2_get_sync_parameter(
            sync: &WebGLSync,
            pname: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLResult>;

        fn canvas_native_webgl2_get_transform_feedback_varying(
            program: u32,
            index: u32,
            state: &mut WebGLState,
        ) -> Box<WebGLActiveInfo>;

        fn canvas_native_webgl2_get_uniform_block_index(
            program: u32,
            uniform_block_name: &str,
            state: &mut WebGLState,
        ) -> u32;

        fn canvas_native_webgl2_get_uniform_indices(
            program: u32,
            uniform_names: &[&str],
            state: &mut WebGLState,
        ) -> Vec<u32>;

        fn canvas_native_webgl2_invalidate_framebuffer(
            target: u32,
            attachments: &[u32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_invalidate_sub_framebuffer(
            target: u32,
            attachments: &[u32],
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_is_query(query: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl2_is_sampler(sampler: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl2_is_sync(sync: &WebGLSync, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl2_is_transform_feedback(
            transform_feedback: u32,
            state: &mut WebGLState,
        ) -> bool;

        fn canvas_native_webgl2_is_vertex_array(vertex_array: u32, state: &mut WebGLState) -> bool;

        fn canvas_native_webgl2_pause_transform_feedback(state: &mut WebGLState);

        fn canvas_native_webgl2_read_buffer(src: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_renderbuffer_storage_multisample(
            target: u32,
            samples: i32,
            internal_format: u32,
            width: i32,
            height: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_resume_transform_feedback(state: &mut WebGLState);

        fn canvas_native_webgl2_sampler_parameterf(
            sampler: u32,
            pname: u32,
            param: f32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_sampler_parameteri(
            sampler: u32,
            pname: u32,
            param: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_tex_image3d_none(
            target: u32,
            level: i32,
            internalformat: i32,
            width: i32,
            height: i32,
            depth: i32,
            border: i32,
            format: u32,
            type_: u32,
            offset: usize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_tex_image3d_asset(
            target: u32,
            level: i32,
            internalformat: i32,
            width: i32,
            height: i32,
            depth: i32,
            border: i32,
            format: u32,
            type_: u32,
            asset: &ImageAsset,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_tex_image3d(
            target: u32,
            level: i32,
            internalformat: i32,
            width: i32,
            height: i32,
            depth: i32,
            border: i32,
            format: u32,
            type_: u32,
            buf: &[u8],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_tex_image3d_offset(
            target: u32,
            level: i32,
            internalformat: i32,
            width: i32,
            height: i32,
            depth: i32,
            border: i32,
            format: u32,
            type_: u32,
            buf: &[u8],
            offset: usize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_tex_storage2d(
            target: u32,
            levels: i32,
            internalformat: u32,
            width: i32,
            height: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_tex_storage3d(
            target: u32,
            levels: i32,
            internalformat: u32,
            width: i32,
            height: i32,
            depth: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_tex_sub_image3d_none(
            target: u32,
            level: i32,
            xoffset: i32,
            yoffset: i32,
            zoffset: i32,
            width: i32,
            height: i32,
            depth: i32,
            format: u32,
            type_: u32,
            offset: usize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_tex_sub_image3d(
            target: u32,
            level: i32,
            xoffset: i32,
            yoffset: i32,
            zoffset: i32,
            width: i32,
            height: i32,
            depth: i32,
            format: u32,
            type_: u32,
            buf: &[u8],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_tex_sub_image3d_asset(
            target: u32,
            level: i32,
            xoffset: i32,
            yoffset: i32,
            zoffset: i32,
            width: i32,
            height: i32,
            depth: i32,
            format: u32,
            type_: u32,
            asset: &ImageAsset,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_tex_sub_image3d_offset(
            target: u32,
            level: i32,
            xoffset: i32,
            yoffset: i32,
            zoffset: i32,
            width: i32,
            height: i32,
            depth: i32,
            format: u32,
            type_: u32,
            buf: &[u8],
            offset: usize,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_transform_feedback_varyings(
            program: u32,
            varyings: &[&str],
            buffer_mode: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform1ui(location: i32, v0: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_uniform1uiv(location: i32, data: &[u32], state: &mut WebGLState);

        fn canvas_native_webgl2_uniform2ui(location: i32, v0: u32, v1: u32, state: &mut WebGLState);

        fn canvas_native_webgl2_uniform2uiv(location: i32, data: &[u32], state: &mut WebGLState);

        fn canvas_native_webgl2_uniform3ui(
            location: i32,
            v0: u32,
            v1: u32,
            v2: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform3uiv(location: i32, data: &[u32], state: &mut WebGLState);

        fn canvas_native_webgl2_uniform4ui(
            location: i32,
            v0: u32,
            v1: u32,
            v2: u32,
            v3: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform4uiv(location: i32, data: &[u32], state: &mut WebGLState);

        fn canvas_native_webgl2_uniform_block_binding(
            program: u32,
            uniform_block_index: u32,
            uniform_block_binding: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform_matrix2x3fv(
            location: i32,
            transpose: bool,
            data: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform_matrix2x4fv(
            location: i32,
            transpose: bool,
            data: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform_matrix3x2fv(
            location: i32,
            transpose: bool,
            data: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform_matrix3x4fv(
            location: i32,
            transpose: bool,
            data: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform_matrix4x2fv(
            location: i32,
            transpose: bool,
            data: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_uniform_matrix4x3fv(
            location: i32,
            transpose: bool,
            data: &[f32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_vertex_attrib_divisor(
            index: u32,
            divisor: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_vertex_attrib_i4i(
            index: u32,
            x: i32,
            y: i32,
            z: i32,
            w: i32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_vertex_attrib_i4iv(
            index: u32,
            value: &[i32],
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_vertex_attrib_i4ui(
            index: u32,
            x: u32,
            y: u32,
            z: u32,
            w: u32,
            state: &mut WebGLState,
        );

        fn canvas_native_webgl2_vertex_attrib_i4uiv(
            index: u32,
            value: &[u32],
            state: &mut WebGLState,
        );

        /* WebGL2RenderingContext */
    }

    /* GL2 */
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

fn canvas_native_context_create_with_wrapper(
    context: i64,
    gl_context: i64,
) -> Box<CanvasRenderingContext2D> {
    let wrapper = context as *mut ContextWrapper;
    let wrapper = unsafe { &mut *wrapper };

    let gl_context = gl_context as *const RefCell<canvas_core::gl::GLContextInner>;
    let gl_context = canvas_core::gl::GLContext::from_raw_inner(gl_context);
    let alpha;
    {
        let lock = wrapper.get_context();
        alpha = lock.device().alpha;
    }
    let clone = ContextWrapper::from_inner(Rc::clone(wrapper.get_inner()));
    Box::new(CanvasRenderingContext2D {
        context: clone,
        gl_context,
        alpha,
    })
}

fn canvas_native_context_resize(context: &mut CanvasRenderingContext2D, width: f32, height: f32) {
    context.resize(width, height);
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
        gl_context: canvas_core::gl::GLContext::default(),
        alpha,
    })
}

pub fn canvas_native_context_create_gl(
    width: f32,
    height: f32,
    density: f32,
    gl_context: i64,
    samples: i32,
    alpha: bool,
    font_color: i32,
    ppi: f32,
    direction: u32,
) -> Box<CanvasRenderingContext2D> {
    let gl_context = gl_context as *const RefCell<canvas_core::gl::GLContextInner>;
    let gl_context = canvas_core::gl::GLContext::from_raw_inner(gl_context);

    gl_context.make_current();
    let mut frame_buffers = [0];
    unsafe {
        gl_bindings::GetIntegerv(gl_bindings::FRAMEBUFFER_BINDING, frame_buffers.as_mut_ptr())
    };

    let ret = Box::new(CanvasRenderingContext2D {
        context: ContextWrapper::new(Context::new_gl(
            width,
            height,
            density,
            frame_buffers[0],
            samples,
            alpha,
            font_color,
            ppi,
            TextDirection::from(direction),
        )),
        gl_context,
        alpha,
    });

    ret
}

pub fn canvas_native_context_create_with_pointer(pointer: i64) -> Box<CanvasRenderingContext2D> {
    unsafe { Box::from_raw(pointer as *mut CanvasRenderingContext2D) }
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
    canvas_native_webgl_create_no_window_internal(
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

    let mut attr = canvas_core::context_attributes::ContextAttributes::new(
        alpha, false, false, false, "default", true, false, false, false, false, true,
    );

    let gl_context =
        canvas_core::gl::GLContext::new_with_no_window(width as i32, height as i32, &mut attr)
            .unwrap();

    gl_context.make_current();

    let mut buffer_id = [0i32];

    unsafe { gl_bindings::GetIntegerv(gl_bindings::FRAMEBUFFER_BINDING, buffer_id.as_mut_ptr()) }

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
    gl_context.remove_if_current();

    Box::new(CanvasRenderingContext2D {
        context,
        gl_context,
        alpha,
    })
}

fn canvas_native_context_get_filter(context: &CanvasRenderingContext2D) -> String {
    context.get_context().get_filter().to_string()
}

fn canvas_native_context_set_filter(context: &mut CanvasRenderingContext2D, filter: &str) {
    context.get_context_mut().set_filter(filter)
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

#[inline]
pub fn canvas_native_paint_style_set_stroke_color_with_rgba(
    context: &mut CanvasRenderingContext2D,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) {
    canvas_2d::context::fill_and_stroke_styles::paint::paint_style_set_color_with_rgba(
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
    canvas_2d::context::fill_and_stroke_styles::paint::paint_style_set_color_with_rgba(
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
    canvas_2d::utils::color::parse_color_rgba(value, r, g, b, a)
}

#[inline]
pub fn canvas_native_paint_style_get_color_string(color: &mut PaintStyle) -> String {
    if let Some(color) = color.0.as_ref() {
        return match color {
            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(color) => {
                to_parsed_color(color.clone())
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
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            to_parsed_color(*stroke)
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            String::with_capacity(0)
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
            String::with_capacity(0)
        }
    }
}

fn canvas_native_paint_style_get_current_stroke_color_buf(
    context: &mut CanvasRenderingContext2D,
) -> Vec<u8> {
    let lock = context.get_context();
    match lock.stroke_style() {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            to_parsed_color(*stroke).into_bytes()
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            Vec::with_capacity(0)
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
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
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            *r = stroke.r();
            *g = stroke.g();
            *b = stroke.b();
            *a = stroke.a();
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {}
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {}
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
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            *r = stroke.r();
            *g = stroke.g();
            *b = stroke.b();
            *a = stroke.a();
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {}
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {}
    }
}

fn canvas_native_paint_style_get_current_fill_color_string(
    context: &mut CanvasRenderingContext2D,
) -> String {
    let lock = context.get_context();
    match lock.fill_style() {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            to_parsed_color(*stroke)
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            String::with_capacity(0)
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
            String::with_capacity(0)
        }
    }
}

fn canvas_native_paint_style_get_current_fill_color_buf(
    context: &mut CanvasRenderingContext2D,
) -> Vec<u8> {
    let lock = context.get_context();
    match lock.fill_style() {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            to_parsed_color(*stroke).into_bytes()
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            Vec::with_capacity(0)
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
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
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(_) => {
            ffi::PaintStyleType::Color
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            ffi::PaintStyleType::Gradient
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
            ffi::PaintStyleType::Pattern
        }
    };
}

pub fn canvas_native_context_get_current_stroke_style_type(
    context: &mut CanvasRenderingContext2D,
) -> ffi::PaintStyleType {
    let lock = context.get_context();
    return match lock.fill_style() {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(_) => {
            ffi::PaintStyleType::Color
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            ffi::PaintStyleType::Gradient
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
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
    context.get_context_mut().clear_rect(x, y, width, height);
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
            .clip(Some(path.inner_mut()), Some(rule));
    }
}

pub fn canvas_native_context_clip_rule(context: &mut CanvasRenderingContext2D, rule: &str) {
    if let Ok(rule) = FillRule::try_from(rule) {
        context.make_current();
        context.get_context_mut().clip(None, Some(rule));
    }
}

#[inline(always)]
pub fn canvas_native_context_close_path(context: &mut CanvasRenderingContext2D) {
    context.get_context_mut().close_path()
}

pub fn canvas_native_context_create_image_data(width: i32, height: i32) -> Box<ImageData> {
    Box::new(ImageData::new(Context::create_image_data(width, height)))
}

pub fn canvas_native_context_create_image_data_with_data(
    width: i32,
    height: i32,
    data: &[u8],
) -> Box<ImageData> {
    Box::new(ImageData::new(
        canvas_2d::context::pixel_manipulation::image_data::ImageData::new_with_data(
            width,
            height,
            data.to_vec(),
        ),
    ))
}

pub fn canvas_native_context_create_linear_gradient(
    context: &mut CanvasRenderingContext2D,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
) -> Box<PaintStyle> {
    Box::new(PaintStyle(Some(
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(
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
                canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
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
                    from_image_slice(bytes, asset.width() as i32, asset.height() as i32).map(
                        |image| {
                            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                                context.get_context().create_pattern(image, repetition),
                            )
                        },
                    )
                },
            )))
        } else {
            Box::new(PaintStyle(
                Repetition::try_from(repetition.as_ref()).map_or(None, |repetition| {
                    from_bitmap_slice(bytes, asset.width() as i32, asset.height() as i32).map(
                        |image| {
                            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                                context.get_context().create_pattern(image, repetition),
                            )
                        },
                    )
                }),
            ))
        };
    }
    Box::new(PaintStyle(None))
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
                canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
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
            let width;
            let height;
            let source_non_gpu;
            let non_gpu;
            {
                let ctx = source.get_context();
                let device = ctx.device();
                width = device.width as i32;
                height = device.height as i32;
                source_non_gpu = device.non_gpu;
            }

            let mut source_ctx = source.get_context_mut();
            let buf;
            if !source_non_gpu {
                source.make_current();

                // todo use gpu image created from snapshot ... need single or shared context or transfer to a texture
                //let data = source_ctx.snapshot_to_raster_data();

                //let mut data = source_ctx.snapshot_to_raster_data();
                let image = source_ctx.snapshot_to_raster_image();
                context.make_current();

                return image.map(|image| {
                    canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                        context.get_context().create_pattern(image, repetition),
                    )
                });

                // return from_image_slice(data.as_slice(), width, height).map(|image| {
                //     canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                //         context.get_context().create_pattern(image, repetition),
                //     )
                // });
            } else {
                source.make_current();
                buf = source_ctx.read_pixels();
            }

            {
                let ctx = context.get_context();
                let device = ctx.device();
                non_gpu = device.non_gpu;
            }

            if !non_gpu {
                context.make_current();
            }

            let ret = from_image_slice(buf.as_slice(), width as i32, height as i32).map(|image| {
                canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                    context.get_context().create_pattern(image, repetition),
                )
            });
            ret
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
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(
            context
                .get_context()
                .create_radial_gradient(x0, y0, r0, x1, y1, r1),
        ),
    )))
}

fn canvas_native_context_draw_paint(context: &mut CanvasRenderingContext2D, color: &str) {
    context.make_current();
    context.get_context_mut().draw_paint(color);
}

fn canvas_native_context_draw_point(context: &mut CanvasRenderingContext2D, x: f32, y: f32) {
    context.make_current();
    context.get_context_mut().draw_point(x, y);
}

fn canvas_native_context_draw_points(
    context: &mut CanvasRenderingContext2D,
    mode: i32,
    points: &[f32],
) {
    context.make_current();
    context
        .get_context_mut()
        .draw_points(mode.try_into().unwrap(), points);
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
        );
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
            .draw_image_src_xywh_dst_xywh(&image, 0.0, 0.0, width, height, dx, dy, width, height);
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
        );
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
        );
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
                );
            }
        } else {
            if let Some(image) = from_bitmap_slice(bytes, width as c_int, height as c_int) {
                context.make_current();
                context.get_context_mut().draw_image_src_xywh_dst_xywh(
                    &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
                );
            }
        }
    }
}

pub fn canvas_native_context_draw_image_dx_dy_context(
    context: &mut CanvasRenderingContext2D,
    source: &mut CanvasRenderingContext2D,
    dx: f32,
    dy: f32,
) {
    let device;
    {
        let source_ctx = source.get_context();
        device = *source_ctx.device();
    }

    let width = device.width;
    let height = device.height;

    canvas_native_context_draw_image_context(
        context, source, 0.0, 0.0, width, height, dx, dy, width, height,
    );
}

pub fn canvas_native_context_draw_image_dx_dy_dw_dh_context(
    context: &mut CanvasRenderingContext2D,
    source: &mut CanvasRenderingContext2D,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    let device;
    {
        let source_ctx = source.get_context();
        device = *source_ctx.device();
    }

    let width = device.width;
    let height = device.height;
    canvas_native_context_draw_image_context(
        context, source, 0.0, 0.0, width, height, dx, dy, d_width, d_height,
    );
}

pub fn canvas_native_context_draw_image_context(
    context: &mut CanvasRenderingContext2D,
    source: &mut CanvasRenderingContext2D,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    source.gl_context.make_current();
    let source_ctx = source.get_context_mut();
    let width = source_ctx.device().width;
    let height = source_ctx.device().height;
    let bytes = source.get_context_mut().snapshot_to_raster_data();

    if let Some(image) = from_image_slice_no_copy(bytes.as_slice(), width as c_int, height as c_int)
    {
        context.make_current();
        context.get_context_mut().draw_image_src_xywh_dst_xywh(
            &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
        );
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
        context.get_context_mut().fill_rule(None, rule);
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
            .fill_rule(Some(path.inner_mut()), rule);
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
    context.get_context_mut().fill_text(text, x, y, width);
}

pub fn canvas_native_context_get_image_data(
    context: &mut CanvasRenderingContext2D,
    sx: f32,
    sy: f32,
    sw: f32,
    sh: f32,
) -> Box<ImageData> {
    context.make_current();
    let ret = Box::new(ImageData(
        context.get_context_mut().get_image_data(sx, sy, sw, sh),
    ));

    ret
}

pub fn canvas_native_context_get_transform(context: &mut CanvasRenderingContext2D) -> Box<Matrix> {
    context.make_current();
    let ret = Box::new(Matrix(canvas_2d::context::matrix::Matrix::from(
        context.get_context_mut().get_transform(),
    )));

    ret
}

pub fn canvas_native_context_is_point_in_path(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    rule: &str,
) -> bool {
    FillRule::try_from(rule).map_or(false, |rule| {
        context.make_current();
        let ret = context.get_context_mut().is_point_in_path(None, x, y, rule);

        ret
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
        let ret = context
            .get_context_mut()
            .is_point_in_path(Some(path.inner()), x, y, rule);

        ret
    })
}

pub fn canvas_native_context_is_point_in_stroke(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
) -> bool {
    context.make_current();
    let ret = context.get_context_mut().is_point_in_stroke(None, x, y);

    ret
}

pub fn canvas_native_context_is_point_in_stroke_with_path(
    context: &mut CanvasRenderingContext2D,
    path: &mut Path,
    x: f32,
    y: f32,
) -> bool {
    context.make_current();
    let ret = context
        .get_context_mut()
        .is_point_in_stroke(Some(path.inner()), x, y);

    ret
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
    );
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

fn canvas_native_context_round_rect(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    radii: &[f32],
) {
    if radii.len() == 8 {
        context
            .get_context_mut()
            .round_rect(x, y, width, height, radii)
    }
}

fn canvas_native_context_round_rect_tl_tr_br_bl(
    context: &mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    top_left: f32,
    top_right: f32,
    bottom_right: f32,
    bottom_left: f32,
) {
    let radii = [
        top_left,
        top_left,
        top_right,
        top_right,
        bottom_right,
        bottom_right,
        bottom_left,
        bottom_left,
    ];
    context
        .get_context_mut()
        .round_rect(x, y, width, height, radii.as_ref())
}

pub fn canvas_native_context_reset_transform(context: &mut CanvasRenderingContext2D) {
    context.make_current();
    context.get_context_mut().reset_transform();
}

#[inline(always)]
pub fn canvas_native_context_restore(context: &mut CanvasRenderingContext2D) {
    context.get_context_mut().restore()
}

pub fn canvas_native_context_rotate(context: &mut CanvasRenderingContext2D, angle: f32) {
    context.make_current();
    context.get_context_mut().rotate(angle);
}

#[inline(always)]
pub fn canvas_native_context_save(context: &mut CanvasRenderingContext2D) {
    context.get_context_mut().save()
}

pub fn canvas_native_context_scale(context: &mut CanvasRenderingContext2D, x: f32, y: f32) {
    context.make_current();
    context.get_context_mut().scale(x, y);
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
    context.get_context_mut().set_transform(a, b, c, d, e, f);
}

pub fn canvas_native_context_set_transform_matrix(
    context: &mut CanvasRenderingContext2D,
    matrix: &mut Matrix,
) {
    context.make_current();
    let matrix = matrix.inner_mut().to_m33();
    context.get_context_mut().set_transform_matrix(&matrix);
}

pub fn canvas_native_context_stroke(context: &mut CanvasRenderingContext2D) {
    context.make_current();
    context.get_context_mut().stroke(None);
}

pub fn canvas_native_context_stroke_with_path(
    context: &mut CanvasRenderingContext2D,
    path: &mut Path,
) {
    context.make_current();
    context.get_context_mut().stroke(Some(path.inner_mut()));
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
    context.get_context_mut().stroke_text(text, x, y, width);
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
    context.get_context_mut().transform(a, b, c, d, e, f);
}

pub fn canvas_native_context_translate(context: &mut CanvasRenderingContext2D, x: f32, y: f32) {
    context.make_current();
    context.get_context_mut().translate(x, y);
}

pub fn canvas_native_context_flush(context: &CanvasRenderingContext2D) {
    context.make_current();
    context.get_context_mut().flush();
}

pub fn canvas_native_context_render(context: &CanvasRenderingContext2D) {
    context.render();
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
        canvas_2d::image_bitmap::create_from_image_asset_src_rect(
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
        canvas_2d::image_bitmap::create_from_image_asset_src_rect(
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
        canvas_2d::image_bitmap::create_image_asset_encoded(
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
    canvas_2d::image_bitmap::create_image_asset_with_output(
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
        canvas_2d::image_bitmap::create_image_asset_encoded(
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
    canvas_2d::image_bitmap::create_image_asset_with_output(
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
pub struct Path(canvas_2d::context::paths::path::Path);

impl Default for Path {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Path {
    pub(crate) fn inner(&self) -> &canvas_2d::context::paths::path::Path {
        &self.0
    }

    pub(crate) fn inner_mut(&mut self) -> &mut canvas_2d::context::paths::path::Path {
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
    Box::new(Path(canvas_2d::context::paths::path::Path::from_str(
        string.as_str(),
    )))
}

#[inline(always)]
pub fn canvas_native_path_create_with_str(string: &str) -> Box<Path> {
    let path = Path(canvas_2d::context::paths::path::Path::from_str(string));
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
fn canvas_native_path_round_rect(
    path: &mut Path,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    radii: &[f32],
) {
    let size = radii.len();
    if size == 0 {
        return;
    }
    /*
    [all-corners]
    [top-left-and-bottom-right, top-right-and-bottom-left]
    [top-left, top-right-and-bottom-left, bottom-right]
    [top-left, top-right, bottom-right, bottom-left]
     */
    let mut top_left = 0.;
    let mut top_right = 0.;
    let mut bottom_right = 0.;
    let mut bottom_left = 0.;

    match size {
        1 => {
            top_left = radii[0];
            top_right = top_left;
            bottom_right = top_left;
            bottom_left = top_left;
        }
        2 => {
            top_left = radii[0];
            top_right = radii[1];
            bottom_right = top_left;
            bottom_left = top_right;
        }

        3 => {
            top_left = radii[0];
            top_right = radii[1];
            bottom_right = radii[2];
            bottom_left = top_right
        }
        4 => {
            top_left = radii[0];
            top_right = radii[1];
            bottom_right = radii[2];
            bottom_left = radii[3];
        }
        _ => {}
    }

    if size > 0 && size <= 4 {
        path.0.round_rect(
            x,
            y,
            width,
            height,
            &[
                top_left,
                top_left,
                top_right,
                top_right,
                bottom_right,
                bottom_right,
                bottom_left,
                bottom_left,
            ],
        )
    }
}

#[inline(always)]
fn canvas_native_path_round_rect_tl_tr_br_bl(
    path: &mut Path,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    top_left: f32,
    top_right: f32,
    bottom_right: f32,
    bottom_left: f32,
) {
    path.0.round_rect(
        x,
        y,
        width,
        height,
        &[
            top_left,
            top_left,
            top_right,
            top_right,
            bottom_right,
            bottom_right,
            bottom_left,
            bottom_left,
        ],
    )
}

#[inline(always)]
pub fn canvas_native_path_to_string(path: &Path) -> String {
    path.0.path().to_svg()
}

/* Path 2D */

/* DOMMatrix */
#[derive(Clone)]
pub struct Matrix(canvas_2d::context::matrix::Matrix);

impl Matrix {
    pub fn inner(&self) -> &canvas_2d::context::matrix::Matrix {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut canvas_2d::context::matrix::Matrix {
        &mut self.0
    }
}

pub fn canvas_native_matrix_create() -> Box<Matrix> {
    Box::new(Matrix(canvas_2d::context::matrix::Matrix::new()))
}

pub fn canvas_native_matrix_update(matrix: &mut Matrix, slice: &[f32]) {
    let mut affine = [0f32; 6];
    affine.copy_from_slice(slice);
    matrix.inner_mut().set_affine(&affine);
}

pub fn canvas_native_matrix_update_3d(matrix: &mut Matrix, slice: &[f32; 16]) {
    let matrix = matrix.inner_mut();
    matrix.set_m11(slice[0]);
    matrix.set_m12(slice[1]);
    matrix.set_m13(slice[2]);
    matrix.set_m14(slice[3]);

    matrix.set_m21(slice[4]);
    matrix.set_m22(slice[5]);
    matrix.set_m23(slice[6]);
    matrix.set_m24(slice[7]);

    matrix.set_m31(slice[8]);
    matrix.set_m32(slice[9]);
    matrix.set_m33(slice[10]);
    matrix.set_m34(slice[11]);

    matrix.set_m41(slice[12]);
    matrix.set_m42(slice[13]);
    matrix.set_m43(slice[14]);
    matrix.set_m44(slice[15]);
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
pub struct ImageData(canvas_2d::context::pixel_manipulation::image_data::ImageData);

impl ImageData {
    pub(crate) fn new(data: canvas_2d::context::pixel_manipulation::image_data::ImageData) -> Self {
        ImageData(data)
    }

    pub fn inner(&self) -> &canvas_2d::context::pixel_manipulation::image_data::ImageData {
        &self.0
    }
}

pub fn canvas_native_image_data_create(width: i32, height: i32) -> Box<ImageData> {
    let data = canvas_2d::context::pixel_manipulation::ImageData::new(width, height);
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

impl ImageAsset {
    pub fn new(asset: canvas_core::image_asset::ImageAsset) -> Self {
        Self(asset)
    }
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

    // pub fn size(&self) -> usize {
    //     self.0.size()
    // }

    #[cfg(any(unix))]
    pub fn load_from_fd(&mut self, fd: c_int) -> bool {
        self.0.load_from_fd(fd)
    }

    pub fn load_from_reader<R>(&mut self, reader: &mut R) -> bool
    where
        R: Read + std::io::Seek + std::io::BufRead,
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

    pub fn copy_bytes(&self) -> Option<Vec<u8>> {
        self.0.copy_bytes()
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

// fn canvas_native_image_asset_get_size(asset: &ImageAsset) -> usize {
//     asset.size()
// }

#[cfg(any(unix))]
pub fn canvas_native_image_asset_load_from_fd(asset: &mut ImageAsset, fd: c_int) -> bool {
    asset.load_from_fd(fd)
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
    asset: &mut canvas_core::image_asset::ImageAsset,
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
        let len: usize;
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
    asset.save_path(path, OutputFormat::from(format))
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ThreadCallbackData {
    id: i64,
    done: bool,
}

impl ThreadCallbackData {
    #[allow(dead_code)]
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

/* PaintStyle */
fn canvas_native_paint_style_from_bytes(
    context: &CanvasRenderingContext2D,
    repetition: i32,
    width: i32,
    height: i32,
    bytes: &[u8],
) -> Box<PaintStyle> {
    Box::new(PaintStyle(from_image_slice(bytes, width, height).map(
        |image| {
            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                context.get_context().create_pattern(
                    image,
                    canvas_2d::context::fill_and_stroke_styles::pattern::Repetition::from(
                        repetition,
                    ),
                ),
            )
        },
    )))
}

fn canvas_native_pattern_from_ptr(ptr: i64) -> Box<PaintStyle> {
    Box::new(PaintStyle::from_ptr(ptr))
}

fn canvas_native_paint_style_empty() -> Box<PaintStyle> {
    Box::new(PaintStyle(None))
}
/* PaintStyle */
/* CanvasGradient */
pub fn canvas_native_gradient_add_color_stop(style: &mut PaintStyle, stop: f32, color: &str) {
    if let Some(style) = style.0.as_mut() {
        match style {
            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(gradient) => {
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
            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(pattern) => {
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
        canvas_2d::context::text_decoder::TextDecoder::new(Some(decoding)),
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
        canvas_2d::context::text_encoder::TextEncoder::new(Some(encoding)),
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
    context.make_current()
}

pub fn canvas_native_context_gl_swap_buffers(context: &CanvasRenderingContext2D) -> bool {
    context.swap_buffers()
}
/* GL */

pub struct GLContext(pub(crate) canvas_core::gl::GLContext);

unsafe impl ExternType for WebGLActiveInfo {
    type Id = type_id!("::org::nativescript::canvas::WebGLActiveInfo");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for ContextAttributes {
    type Id = type_id!("::org::nativescript::canvas::ContextAttributes");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for WebGLResult {
    type Id = type_id!("::org::nativescript::canvas::WebGLResult");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for WebGLState {
    type Id = type_id!("::org::nativescript::canvas::WebGLState");
    type Kind = cxx::kind::Trivial;
}

impl Into<ffi::WebGLExtensionType> for canvas_webgl::prelude::WebGLExtensionType {
    fn into(self) -> ffi::WebGLExtensionType {
        match self {
            canvas_webgl::prelude::WebGLExtensionType::EXT_blend_minmax => {
                ffi::WebGLExtensionType::EXT_blend_minmax
            }
            canvas_webgl::prelude::WebGLExtensionType::EXT_color_buffer_half_float => {
                ffi::WebGLExtensionType::EXT_color_buffer_half_float
            }
            canvas_webgl::prelude::WebGLExtensionType::EXT_disjoint_timer_query => {
                ffi::WebGLExtensionType::EXT_disjoint_timer_query
            }
            canvas_webgl::prelude::WebGLExtensionType::EXT_sRGB => {
                ffi::WebGLExtensionType::EXT_sRGB
            }
            canvas_webgl::prelude::WebGLExtensionType::EXT_shader_texture_lod => {
                ffi::WebGLExtensionType::EXT_shader_texture_lod
            }
            canvas_webgl::prelude::WebGLExtensionType::EXT_texture_filter_anisotropic => {
                ffi::WebGLExtensionType::EXT_texture_filter_anisotropic
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_element_index_uint => {
                ffi::WebGLExtensionType::OES_element_index_uint
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_standard_derivatives => {
                ffi::WebGLExtensionType::OES_standard_derivatives
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_texture_float => {
                ffi::WebGLExtensionType::OES_texture_float
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_texture_float_linear => {
                ffi::WebGLExtensionType::OES_texture_float_linear
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_texture_half_float => {
                ffi::WebGLExtensionType::OES_texture_half_float
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_texture_half_float_linear => {
                ffi::WebGLExtensionType::OES_texture_half_float_linear
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_vertex_array_object => {
                ffi::WebGLExtensionType::OES_vertex_array_object
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_color_buffer_float => {
                ffi::WebGLExtensionType::WEBGL_color_buffer_float
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_atc => {
                ffi::WebGLExtensionType::WEBGL_compressed_texture_atc
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_etc1 => {
                ffi::WebGLExtensionType::WEBGL_compressed_texture_etc1
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_s3tc => {
                ffi::WebGLExtensionType::WEBGL_compressed_texture_s3tc
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_s3tc_srgb => {
                ffi::WebGLExtensionType::WEBGL_compressed_texture_s3tc_srgb
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_etc => {
                ffi::WebGLExtensionType::WEBGL_compressed_texture_etc
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_pvrtc => {
                ffi::WebGLExtensionType::WEBGL_compressed_texture_pvrtc
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_lose_context => {
                ffi::WebGLExtensionType::WEBGL_lose_context
            }
            canvas_webgl::prelude::WebGLExtensionType::ANGLE_instanced_arrays => {
                ffi::WebGLExtensionType::ANGLE_instanced_arrays
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_depth_texture => {
                ffi::WebGLExtensionType::WEBGL_depth_texture
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_draw_buffers => {
                ffi::WebGLExtensionType::WEBGL_draw_buffers
            }
            canvas_webgl::prelude::WebGLExtensionType::None => ffi::WebGLExtensionType::None,
        }
    }
}

pub fn canvas_native_context_create_pattern_webgl(
    source: &mut WebGLState,
    context: &mut CanvasRenderingContext2D,
    repetition: &str,
) -> Box<PaintStyle> {
    Box::new(PaintStyle(Repetition::try_from(repetition).map_or(
        None,
        |repetition| {
            let state = source.get_inner();
            state.make_current();
            let width = state.get_drawing_buffer_width();
            let height = state.get_drawing_buffer_height();

            let mut buf = vec![0u8; (width * height * 4) as usize];

            unsafe {
                gl_bindings::Finish();
                gl_bindings::ReadPixels(
                    0,
                    0,
                    width,
                    height,
                    gl_bindings::RGBA,
                    gl_bindings::UNSIGNED_BYTE,
                    buf.as_mut_ptr() as *mut c_void,
                );
            }

            context.make_current();
            let ret = from_image_slice(buf.as_slice(), width, height).map(|image| {
                canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                    context.get_context().create_pattern(image, repetition),
                )
            });

            ret
        },
    )))
}

/* GL */

pub fn canvas_native_webgl_make_current(state: &mut WebGLState) -> bool {
    state.get_inner().make_current()
}

pub fn canvas_native_webgl_swap_buffers(state: &mut WebGLState) -> bool {
    state.get_inner().swap_buffers()
}
/* GL */

/* WebGL */

fn canvas_native_webgl_resized(_state: &mut WebGLState) {
    //state.get_inner_mut().resized();
}

fn canvas_native_webgl_to_data_url(state: &mut WebGLState, format: &str, quality: i32) -> String {
    let info = state.get_inner();
    let width = info.drawing_buffer_width();
    let height = info.drawing_buffer_height();
    info.make_current();
    // gl_bindings::PixelStorei(gl_bindings::UNPACK_ALIGNMENT, 1);
    let mut buffer = vec![0u8; (width * height * 4) as usize];
    unsafe {
        gl_bindings::ReadPixels(
            0,
            0,
            width as i32,
            height as i32,
            gl_bindings::RGBA,
            gl_bindings::UNSIGNED_BYTE,
            buffer.as_mut_ptr() as *mut c_void,
        );
    }

    canvas_2d::bytes_to_data_url(width, height, buffer.as_slice(), format, quality)
}

#[derive(Debug)]
pub struct WebGLState(canvas_webgl::prelude::WebGLState);

impl WebGLState {
    pub fn new_with_context(
        context: canvas_core::gl::GLContext,
        version: WebGLVersion,
        alpha: bool,
        antialias: bool,
        depth: bool,
        fail_if_major_performance_caveat: bool,
        power_preference: &str,
        premultiplied_alpha: bool,
        preserve_drawing_buffer: bool,
        stencil: bool,
        desynchronized: bool,
        xr_compatible: bool,
        is_canvas: bool,
    ) -> Self {
        Self(
            canvas_webgl::prelude::WebGLState::new_with_context_attributes(
                context,
                version,
                alpha,
                antialias,
                depth,
                fail_if_major_performance_caveat,
                power_preference,
                premultiplied_alpha,
                preserve_drawing_buffer,
                stencil,
                desynchronized,
                xr_compatible,
                is_canvas,
            ),
        )
    }
    pub fn get_inner(&self) -> &canvas_webgl::prelude::WebGLState {
        &self.0
    }

    pub fn get_inner_mut(&mut self) -> &mut canvas_webgl::prelude::WebGLState {
        &mut self.0
    }
}

pub struct WebGLActiveInfo(pub(crate) canvas_webgl::prelude::WebGLActiveInfo);

impl WebGLActiveInfo {
    pub fn get_name(&self) -> &str {
        self.0.get_name()
    }

    pub fn get_size(&self) -> i32 {
        self.0.get_size()
    }

    pub fn get_type(&self) -> u32 {
        self.0.get_type()
    }

    pub fn get_is_empty(&self) -> bool {
        self.0.get_is_empty()
    }
}

pub struct ContextAttributes(canvas_webgl::prelude::ContextAttributes);

impl ContextAttributes {
    pub fn get_alpha(&self) -> bool {
        self.0.get_alpha()
    }
    pub fn get_antialias(&self) -> bool {
        self.0.get_antialias()
    }
    pub fn get_depth(&self) -> bool {
        self.0.get_depth()
    }
    pub fn get_fail_if_major_performance_caveat(&self) -> bool {
        self.0.get_fail_if_major_performance_caveat()
    }
    pub fn get_power_preference(&self) -> String {
        self.0.get_power_preference()
    }
    pub fn get_premultiplied_alpha(&self) -> bool {
        self.0.get_premultiplied_alpha()
    }
    pub fn get_preserve_drawing_buffer(&self) -> bool {
        self.0.get_preserve_drawing_buffer()
    }
    pub fn get_stencil(&self) -> bool {
        self.0.get_stencil()
    }
    pub fn get_desynchronized(&self) -> bool {
        self.0.get_desynchronized()
    }
    pub fn get_xr_compatible(&self) -> bool {
        self.0.get_xr_compatible()
    }
}

pub struct WebGLFramebufferAttachmentParameter(
    canvas_webgl::prelude::WebGLFramebufferAttachmentParameter,
);

impl WebGLFramebufferAttachmentParameter {
    pub fn get_is_texture(&self) -> bool {
        self.0.get_is_texture()
    }

    pub fn get_is_renderbuffer(&self) -> bool {
        self.0.get_is_renderbuffer()
    }

    pub fn get_value(&self) -> i32 {
        self.0.get_value()
    }
}

pub struct WebGLShaderPrecisionFormat(pub(crate) canvas_webgl::prelude::WebGLShaderPrecisionFormat);

impl WebGLShaderPrecisionFormat {
    pub fn get_precision(&self) -> i32 {
        self.0.get_precision()
    }

    pub fn get_range_min(&self) -> i32 {
        self.0.get_range_min()
    }

    pub fn get_range_max(&self) -> i32 {
        self.0.get_range_max()
    }
}

pub struct WebGLExtension(pub(crate) Option<Box<dyn canvas_webgl::prelude::WebGLExtension>>);

impl WebGLExtension {
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    pub fn extension_type(&self) -> ffi::WebGLExtensionType {
        if self.is_none() {
            return ffi::WebGLExtensionType::None;
        }
        self.0.as_ref().unwrap().extension_type().into()
    }

    pub fn into_ext_disjoint_timer_query(self) -> Box<EXT_disjoint_timer_query> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext =
            unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::EXT_disjoint_timer_query) };
        Box::new(EXT_disjoint_timer_query(*ext))
    }

    pub fn into_angle_instanced_arrays(self) -> Box<ANGLE_instanced_arrays> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext =
            unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::ANGLE_instanced_arrays) };
        Box::new(ANGLE_instanced_arrays(*ext))
    }

    pub fn into_lose_context(self) -> Box<WEBGL_lose_context> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext = unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::WEBGL_lose_context) };
        Box::new(WEBGL_lose_context(*ext))
    }

    pub fn into_draw_buffers(self) -> Box<WEBGL_draw_buffers> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext = unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::WEBGL_draw_buffers) };
        Box::new(WEBGL_draw_buffers(*ext))
    }

    pub fn into_oes_vertex_array_object(self) -> Box<OES_vertex_array_object> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext =
            unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::OES_vertex_array_object) };
        Box::new(OES_vertex_array_object(*ext))
    }
}

#[allow(non_camel_case_types)]
pub struct EXT_blend_minmax(canvas_webgl::prelude::EXT_blend_minmax);

#[allow(non_camel_case_types)]
pub struct EXT_color_buffer_half_float(canvas_webgl::prelude::EXT_color_buffer_half_float);

#[allow(non_camel_case_types)]
pub struct EXT_disjoint_timer_query(canvas_webgl::prelude::EXT_disjoint_timer_query);

#[allow(non_camel_case_types)]
pub struct EXT_sRGB(canvas_webgl::prelude::EXT_sRGB);

#[allow(non_camel_case_types)]
pub struct EXT_shader_texture_lod(canvas_webgl::prelude::EXT_shader_texture_lod);

#[allow(non_camel_case_types)]
pub struct EXT_texture_filter_anisotropic(canvas_webgl::prelude::EXT_texture_filter_anisotropic);

#[allow(non_camel_case_types)]
pub struct OES_element_index_uint(canvas_webgl::prelude::OES_element_index_uint);

#[allow(non_camel_case_types)]
pub struct OES_standard_derivatives(canvas_webgl::prelude::OES_standard_derivatives);

#[allow(non_camel_case_types)]
pub struct OES_texture_float(canvas_webgl::prelude::OES_texture_float);

#[allow(non_camel_case_types)]
pub struct OES_texture_float_linear(canvas_webgl::prelude::OES_texture_float_linear);

#[allow(non_camel_case_types)]
pub struct OES_texture_half_float(canvas_webgl::prelude::OES_texture_half_float);

#[allow(non_camel_case_types)]
pub struct OES_texture_half_float_linear(canvas_webgl::prelude::OES_texture_half_float_linear);

#[allow(non_camel_case_types)]
pub struct OES_vertex_array_object(canvas_webgl::prelude::OES_vertex_array_object);

#[allow(non_camel_case_types)]
pub struct WEBGL_color_buffer_float(canvas_webgl::prelude::WEBGL_color_buffer_float);

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_atc(canvas_webgl::prelude::WEBGL_compressed_texture_atc);

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_etc1(canvas_webgl::prelude::WEBGL_compressed_texture_etc1);

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_s3tc(canvas_webgl::prelude::WEBGL_compressed_texture_s3tc);

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_s3tc_srgb(
    canvas_webgl::prelude::WEBGL_compressed_texture_s3tc_srgb,
);

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_etc(canvas_webgl::prelude::WEBGL_compressed_texture_etc);

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_pvrtc(canvas_webgl::prelude::WEBGL_compressed_texture_pvrtc);

#[allow(non_camel_case_types)]
pub struct WEBGL_lose_context(canvas_webgl::prelude::WEBGL_lose_context);

#[allow(non_camel_case_types)]
pub struct ANGLE_instanced_arrays(canvas_webgl::prelude::ANGLE_instanced_arrays);

#[allow(non_camel_case_types)]
pub struct WEBGL_depth_texture(canvas_webgl::prelude::WEBGL_depth_texture);

#[allow(non_camel_case_types)]
pub struct WEBGL_draw_buffers(canvas_webgl::prelude::WEBGL_draw_buffers);

pub struct WebGLResult(pub(crate) canvas_webgl::prelude::WebGLResult);

/* WebGLActiveInfo */

pub fn canvas_native_webgl_active_info_get_name(info: &WebGLActiveInfo) -> &str {
    info.get_name()
}

pub fn canvas_native_webgl_active_info_get_size(info: &WebGLActiveInfo) -> i32 {
    info.get_size()
}

pub fn canvas_native_webgl_active_info_get_type(info: &WebGLActiveInfo) -> u32 {
    info.get_type()
}

pub fn canvas_native_webgl_active_info_get_is_empty(info: &WebGLActiveInfo) -> bool {
    info.get_is_empty()
}

/* WebGLActiveInfo */

/* WebGLShaderPrecisionFormat */

fn canvas_native_webgl_shader_precision_format_get_range_min(
    shader: &WebGLShaderPrecisionFormat,
) -> i32 {
    shader.get_range_min()
}

fn canvas_native_webgl_shader_precision_format_get_range_max(
    shader: &WebGLShaderPrecisionFormat,
) -> i32 {
    shader.get_range_max()
}

fn canvas_native_webgl_shader_precision_format_get_precision(
    shader: &WebGLShaderPrecisionFormat,
) -> i32 {
    shader.get_precision()
}

/* WebGLShaderPrecisionFormat */

/* ContextAttributes */
pub fn canvas_native_webgl_context_attribute_get_get_alpha(attr: &ContextAttributes) -> bool {
    attr.get_alpha()
}

pub fn canvas_native_webgl_context_attribute_get_get_antialias(attr: &ContextAttributes) -> bool {
    attr.get_antialias()
}

pub fn canvas_native_webgl_context_attribute_get_get_depth(attr: &ContextAttributes) -> bool {
    attr.get_depth()
}

pub fn canvas_native_webgl_context_attribute_get_get_fail_if_major_performance_caveat(
    attr: &ContextAttributes,
) -> bool {
    attr.get_fail_if_major_performance_caveat()
}

pub fn canvas_native_webgl_context_attribute_get_get_power_preference(
    attr: &ContextAttributes,
) -> String {
    attr.get_power_preference()
}

pub fn canvas_native_webgl_context_attribute_get_get_premultiplied_alpha(
    attr: &ContextAttributes,
) -> bool {
    attr.get_premultiplied_alpha()
}

pub fn canvas_native_webgl_context_attribute_get_get_preserve_drawing_buffer(
    attr: &ContextAttributes,
) -> bool {
    attr.get_preserve_drawing_buffer()
}

pub fn canvas_native_webgl_context_attribute_get_get_stencil(attr: &ContextAttributes) -> bool {
    attr.get_stencil()
}

pub fn canvas_native_webgl_context_attribute_get_get_desynchronized(
    attr: &ContextAttributes,
) -> bool {
    attr.get_desynchronized()
}

pub fn canvas_native_webgl_context_attribute_get_get_xr_compatible(
    attr: &ContextAttributes,
) -> bool {
    attr.get_xr_compatible()
}

/* ContextAttributes */

/* WebGLExtension */
pub fn canvas_native_webgl_context_extension_is_none(extension: &WebGLExtension) -> bool {
    extension.is_none()
}

pub fn canvas_native_webgl_context_extension_get_type(
    extension: &WebGLExtension,
) -> ffi::WebGLExtensionType {
    extension.extension_type().into()
}

pub fn canvas_native_webgl_context_extension_to_ext_disjoint_timer_query(
    extension: Box<WebGLExtension>,
) -> Box<EXT_disjoint_timer_query> {
    extension.into_ext_disjoint_timer_query()
}

pub fn canvas_native_webgl_context_extension_to_angle_instanced_arrays(
    extension: Box<WebGLExtension>,
) -> Box<ANGLE_instanced_arrays> {
    extension.into_angle_instanced_arrays()
}

pub fn canvas_native_webgl_context_extension_to_lose_context(
    extension: Box<WebGLExtension>,
) -> Box<WEBGL_lose_context> {
    extension.into_lose_context()
}

pub fn canvas_native_webgl_context_extension_to_draw_buffers(
    extension: Box<WebGLExtension>,
) -> Box<WEBGL_draw_buffers> {
    extension.into_draw_buffers()
}

pub fn canvas_native_webgl_context_extension_to_oes_vertex_array_object(
    extension: Box<WebGLExtension>,
) -> Box<OES_vertex_array_object> {
    extension.into_oes_vertex_array_object()
}

/* WebGLExtension */

/* WebGLResult */
fn canvas_native_webgl_result_get_type(result: &WebGLResult) -> ffi::WebGLResultType {
    match result.0 {
        canvas_webgl::prelude::WebGLResult::Boolean(_) => ffi::WebGLResultType::Boolean,
        canvas_webgl::prelude::WebGLResult::I32Array(_) => ffi::WebGLResultType::I32Array,
        canvas_webgl::prelude::WebGLResult::U32Array(_) => ffi::WebGLResultType::U32Array,
        canvas_webgl::prelude::WebGLResult::F32Array(_) => ffi::WebGLResultType::F32Array,
        canvas_webgl::prelude::WebGLResult::BooleanArray(_) => ffi::WebGLResultType::BooleanArray,
        canvas_webgl::prelude::WebGLResult::U32(_) => ffi::WebGLResultType::U32,
        canvas_webgl::prelude::WebGLResult::I32(_) => ffi::WebGLResultType::I32,
        canvas_webgl::prelude::WebGLResult::F32(_) => ffi::WebGLResultType::F32,
        canvas_webgl::prelude::WebGLResult::String(_) => ffi::WebGLResultType::String,
        canvas_webgl::prelude::WebGLResult::None => ffi::WebGLResultType::None,
    }
}

fn canvas_native_webgl_result_get_bool(result: &WebGLResult) -> bool {
    match result.0 {
        canvas_webgl::prelude::WebGLResult::Boolean(value) => value,
        _ => false,
    }
}

fn canvas_native_webgl_result_get_i32_array(result: &WebGLResult) -> Vec<i32> {
    match &result.0 {
        canvas_webgl::prelude::WebGLResult::I32Array(value) => value.clone(),
        _ => Vec::new(),
    }
}

fn canvas_native_webgl_result_get_u32_array(result: &WebGLResult) -> Vec<u32> {
    match &result.0 {
        canvas_webgl::prelude::WebGLResult::U32Array(value) => value.clone(),
        _ => Vec::new(),
    }
}

fn canvas_native_webgl_result_get_f32_array(result: &WebGLResult) -> Vec<f32> {
    match &result.0 {
        canvas_webgl::prelude::WebGLResult::F32Array(value) => value.clone(),
        _ => Vec::new(),
    }
}

fn canvas_native_webgl_result_get_bool_array(result: &WebGLResult) -> Vec<u8> {
    match &result.0 {
        canvas_webgl::prelude::WebGLResult::BooleanArray(value) => unsafe {
            std::slice::from_raw_parts(value.as_ptr() as *const u8, value.len()).to_vec()
        },
        _ => Vec::new(),
    }
}

fn canvas_native_webgl_result_get_u32(result: &WebGLResult) -> u32 {
    match result.0 {
        canvas_webgl::prelude::WebGLResult::U32(value) => value,
        _ => 0,
    }
}

fn canvas_native_webgl_result_get_i32(result: &WebGLResult) -> i32 {
    match result.0 {
        canvas_webgl::prelude::WebGLResult::I32(value) => value,
        _ => 0,
    }
}

fn canvas_native_webgl_result_get_f32(result: &WebGLResult) -> f32 {
    match result.0 {
        canvas_webgl::prelude::WebGLResult::F32(value) => value,
        _ => 0.,
    }
}

fn canvas_native_webgl_result_get_string(result: &WebGLResult) -> String {
    let ret;
    match result.0 {
        canvas_webgl::prelude::WebGLResult::String(ref result) => {
            let val = result.to_string_lossy();

            if val.contains("OpenGL ES 3.0") {
                return "WebGL 2.0 (OpenGL ES 3.0 NativeScript)".to_string();
            }

            ret = "WebGL 1.0 (OpenGL ES 2.0 NativeScript)".to_string();
        }
        _ => {
            ret = String::default();
        }
    };
    ret
}

fn canvas_native_webgl_result_get_is_none(result: &WebGLResult) -> bool {
    match result.0 {
        canvas_webgl::prelude::WebGLResult::None => true,
        _ => false,
    }
}

/* WebGLResult */

/* WebGLState */
pub fn canvas_native_webgl_state_get_unpack_colorspace_conversion_webgl(
    state: &mut WebGLState,
) -> i32 {
    state.get_inner().get_unpack_colorspace_conversion_webgl()
}

pub fn canvas_native_webgl_state_get_flip_y(state: &mut WebGLState) -> bool {
    state.get_inner().get_flip_y()
}

pub fn canvas_native_webgl_state_get_premultiplied_alpha(state: &mut WebGLState) -> bool {
    state.get_inner().get_premultiplied_alpha()
}

pub fn canvas_native_webgl_state_get_drawing_buffer_width(state: &mut WebGLState) -> i32 {
    state.get_inner().get_drawing_buffer_width()
}

pub fn canvas_native_webgl_state_get_drawing_buffer_height(state: &mut WebGLState) -> i32 {
    state.get_inner().get_drawing_buffer_height()
}

/* WebGLState */

/* EXT_disjoint_timer_query */
pub fn canvas_native_webgl_ext_disjoint_timer_query_create_query_ext(
    query: &EXT_disjoint_timer_query,
) -> u32 {
    query.0.create_query_ext()
}

pub fn canvas_native_webgl_ext_disjoint_timer_query_delete_query_ext(
    value: u32,
    query: &EXT_disjoint_timer_query,
) {
    query.0.delete_query_ext(value)
}

pub fn canvas_native_webgl_ext_disjoint_timer_query_is_query_ext(
    value: u32,
    query: &EXT_disjoint_timer_query,
) -> bool {
    query.0.is_query_ext(value)
}

pub fn canvas_native_webgl_ext_disjoint_timer_query_begin_query_ext(
    target: u32,
    value: u32,
    query: &EXT_disjoint_timer_query,
) {
    query.0.begin_query_ext(target, value)
}

pub fn canvas_native_webgl_ext_disjoint_timer_query_end_query_ext(
    target: u32,
    query: &EXT_disjoint_timer_query,
) {
    query.0.end_query_ext(target)
}

pub fn canvas_native_webgl_ext_disjoint_timer_query_query_counter_ext(
    value: u32,
    target: u32,
    query: &EXT_disjoint_timer_query,
) {
    query.0.query_counter_ext(value, target)
}

pub fn canvas_native_webgl_ext_disjoint_timer_query_get_query_ext(
    target: u32,
    pname: u32,
    query: &EXT_disjoint_timer_query,
) -> i32 {
    query.0.get_query_ext(target, pname)
}

pub fn canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(
    target: u32,
    pname: u32,
    query: &EXT_disjoint_timer_query,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(query.0.get_query_object_ext(target, pname)))
}

/* EXT_disjoint_timer_query */

/* ANGLE_instanced_arrays */
fn canvas_native_webgl_angle_instanced_arrays_draw_arrays_instanced_angle(
    mode: u32,
    first: i32,
    count: i32,
    primcount: i32,
    arrays: &ANGLE_instanced_arrays,
) {
    arrays
        .0
        .draw_arrays_instanced_angle(mode, first, count, primcount)
}

fn canvas_native_webgl_angle_instanced_arrays_draw_elements_instanced_angle(
    mode: u32,
    count: i32,
    type_: u32,
    offset: i32,
    primcount: i32,
    arrays: &ANGLE_instanced_arrays,
) {
    arrays
        .0
        .draw_elements_instanced_angle(mode, count, type_, offset, primcount)
}

fn canvas_native_webgl_angle_instanced_arrays_vertex_attrib_divisor_angle(
    index: u32,
    divisor: u32,
    arrays: &ANGLE_instanced_arrays,
) {
    arrays.0.vertex_attrib_divisor_angle(index, divisor)
}
/* ANGLE_instanced_arrays */

/* WEBGL_lose_context */
fn canvas_native_webgl_lose_context_lose_context(context: &WEBGL_lose_context) {
    context.0.lose_context()
}

fn canvas_native_webgl_lose_context_restore_context(context: &WEBGL_lose_context) {
    context.0.restore_context()
}
/* WEBGL_lose_context */

/* WEBGL_draw_buffers */

fn canvas_native_webgl_draw_buffers_draw_buffers_webgl(
    buffers: &[u32],
    context: &WEBGL_draw_buffers,
) {
    context.0.draw_buffers_webgl(buffers);
}

/* WEBGL_draw_buffers */

/* OES_vertex_array_object */

fn canvas_native_webgl_oes_vertex_array_object_create_vertex_array_oes(
    object: &OES_vertex_array_object,
) -> u32 {
    object.0.create_vertex_array_oes()
}

fn canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(
    array_object: u32,
    object: &OES_vertex_array_object,
) {
    object.0.delete_vertex_array_oes(array_object)
}

fn canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(
    array_object: u32,
    object: &OES_vertex_array_object,
) -> bool {
    object.0.is_vertex_array_oes(array_object)
}

fn canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(
    array_object: u32,
    object: &OES_vertex_array_object,
) {
    object.0.bind_vertex_array_oes(array_object)
}

/* OES_vertex_array_object */

pub fn canvas_native_webgl_create(
    gl_context: i64,
    version: &str,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: &str,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
) -> Box<WebGLState> {
    let version = if version.eq("v1") {
        WebGLVersion::V1
    } else if version.eq("v2") {
        WebGLVersion::V2
    } else {
        WebGLVersion::NONE
    };

    let gl_context = gl_context as *const RefCell<canvas_core::gl::GLContextInner>;
    let gl_context = canvas_core::gl::GLContext::from_raw_inner(gl_context);

    let inner = WebGLState::new_with_context(
        gl_context,
        version,
        alpha,
        antialias,
        depth,
        fail_if_major_performance_caveat,
        power_preference,
        premultiplied_alpha,
        preserve_drawing_buffer,
        stencil,
        desynchronized,
        xr_compatible,
        false,
    );
    Box::new(inner)
}

pub fn canvas_native_webgl_create_no_window(
    width: i32,
    height: i32,
    version: &str,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: &str,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
    is_canvas: bool,
) -> Box<WebGLState> {
    Box::new(canvas_native_webgl_create_no_window_internal(
        width,
        height,
        version,
        alpha,
        antialias,
        depth,
        fail_if_major_performance_caveat,
        power_preference,
        premultiplied_alpha,
        preserve_drawing_buffer,
        stencil,
        desynchronized,
        xr_compatible,
        is_canvas,
    ))
}

pub fn canvas_native_webgl_create_no_window_internal(
    width: i32,
    height: i32,
    version: &str,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: &str,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
    is_canvas: bool,
) -> WebGLState {
    let version = if version.eq("v1") || version.eq("canvas") {
        WebGLVersion::V1
    } else if version.eq("v2") {
        WebGLVersion::V2
    } else {
        WebGLVersion::NONE
    };

    let mut attrs = canvas_core::context_attributes::ContextAttributes::new(
        alpha,
        antialias,
        depth,
        fail_if_major_performance_caveat,
        power_preference,
        premultiplied_alpha,
        preserve_drawing_buffer,
        stencil,
        desynchronized,
        xr_compatible,
        is_canvas,
    );

    let ctx = canvas_core::gl::GLContext::new_with_no_window(width, height, &mut attrs)
        .unwrap_or_default();

    WebGLState::new_with_context(
        ctx,
        version,
        attrs.get_alpha(),
        attrs.get_antialias(),
        attrs.get_depth(),
        attrs.get_fail_if_major_performance_caveat(),
        attrs.get_power_preference().as_str(),
        attrs.get_premultiplied_alpha(),
        attrs.get_preserve_drawing_buffer(),
        attrs.get_stencil(),
        attrs.get_desynchronized(),
        attrs.get_xr_compatible(),
        attrs.get_is_canvas(),
    )
}

pub fn canvas_native_webgl_active_texture(texture: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_active_texture(texture, state.get_inner_mut())
}

pub fn canvas_native_webgl_attach_shader(program: u32, shader: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_attach_shader(program, shader, state.get_inner_mut())
}

pub fn canvas_native_webgl_bind_attrib_location(
    program: u32,
    index: u32,
    name: &str,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_bind_attrib_location(
        program,
        index,
        name,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_bind_buffer(target: u32, buffer: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_bind_buffer(target, buffer, state.get_inner_mut())
}

pub fn canvas_native_webgl_bind_frame_buffer(
    target: u32,
    framebuffer: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_bind_frame_buffer(
        target,
        framebuffer,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_bind_render_buffer(
    target: u32,
    renderbuffer: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_bind_render_buffer(
        target,
        renderbuffer,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_bind_texture(target: u32, texture: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_bind_texture(target, texture, state.get_inner_mut())
}

pub fn canvas_native_webgl_blend_color(
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_blend_color(
        red,
        green,
        blue,
        alpha,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_blend_equation_separate(
    mode_rgb: u32,
    mode_alpha: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_blend_equation_separate(
        mode_rgb,
        mode_alpha,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_blend_equation(mode: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_blend_equation(mode, state.get_inner_mut())
}

pub fn canvas_native_webgl_blend_func_separate(
    src_rgb: u32,
    dst_rgb: u32,
    src_alpha: u32,
    dst_alpha: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_blend_func_separate(
        src_rgb,
        dst_rgb,
        src_alpha,
        dst_alpha,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_blend_func(sfactor: u32, dfactor: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_blend_func(sfactor, dfactor, state.get_inner_mut())
}

pub fn canvas_native_webgl_buffer_data(
    target: u32,
    src_data: &[u8],
    usage: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_buffer_data(
        target,
        src_data,
        usage,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_buffer_data_u16(
    target: u32,
    src_data: &[u16],
    usage: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_buffer_data_u16(
        target,
        src_data,
        usage,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_buffer_data_f32(
    target: u32,
    src_data: &[f32],
    usage: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_buffer_data_f32(
        target,
        src_data,
        usage,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_buffer_data_none(
    target: u32,
    size: isize,
    usage: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_buffer_data_none(
        target,
        size,
        usage,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_buffer_sub_data(
    target: u32,
    offset: isize,
    src_data: &[u8],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_buffer_sub_data(
        target,
        offset,
        src_data,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_buffer_sub_data_none(
    target: u32,
    offset: isize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_buffer_sub_data_none(
        target,
        offset,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_check_frame_buffer_status(target: u32, state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl::canvas_native_webgl_check_frame_buffer_status(
        target,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_clear(mask: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_clear(mask, state.get_inner_mut())
}

pub fn canvas_native_webgl_clear_color(
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_clear_color(
        red,
        green,
        blue,
        alpha,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_clear_depth(depth: f32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_clear_depth(depth, state.get_inner_mut())
}

pub fn canvas_native_webgl_clear_stencil(stencil: i32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_clear_stencil(stencil, state.get_inner_mut())
}

pub fn canvas_native_webgl_color_mask(
    red: bool,
    green: bool,
    blue: bool,
    alpha: bool,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_color_mask(
        red,
        green,
        blue,
        alpha,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_commit(_: &mut WebGLState) {
    // noop
}

pub fn canvas_native_webgl_compile_shader(shader: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_compile_shader(shader, state.get_inner_mut())
}

pub fn canvas_native_webgl_compressed_tex_image2d(
    target: u32,
    level: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    border: i32,
    pixels: &[u8],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_compressed_tex_image2d(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        pixels,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_compressed_tex_image2d_none(
    target: u32,
    level: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    border: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_compressed_tex_image2d_none(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_compressed_tex_sub_image2d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    width: i32,
    height: i32,
    format: u32,
    pixels: &[u8],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_compressed_tex_sub_image2d(
        target,
        level,
        xoffset,
        yoffset,
        width,
        height,
        format,
        pixels,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_copy_tex_image2d(
    target: u32,
    level: i32,
    internalformat: u32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    border: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_copy_tex_image2d(
        target,
        level,
        internalformat,
        x,
        y,
        width,
        height,
        border,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_copy_tex_sub_image2d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_copy_tex_sub_image2d(
        target,
        level,
        xoffset,
        yoffset,
        x,
        y,
        width,
        height,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_create_buffer(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl::canvas_native_webgl_create_buffer(state.get_inner_mut())
}

pub fn canvas_native_webgl_create_framebuffer(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl::canvas_native_webgl_create_framebuffer(state.get_inner_mut())
}

pub fn canvas_native_webgl_create_program(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl::canvas_native_webgl_create_program(state.get_inner_mut())
}

pub fn canvas_native_webgl_create_renderbuffer(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl::canvas_native_webgl_create_renderbuffer(state.get_inner_mut())
}

pub fn canvas_native_webgl_create_shader(shader_type: u32, state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl::canvas_native_webgl_create_shader(shader_type, state.get_inner_mut())
}

pub fn canvas_native_webgl_create_texture(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl::canvas_native_webgl_create_texture(state.get_inner_mut())
}

pub fn canvas_native_webgl_cull_face(mode: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_cull_face(mode, state.get_inner_mut())
}

pub fn canvas_native_webgl_delete_buffer(buffer: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_delete_buffer(buffer, state.get_inner_mut())
}

pub fn canvas_native_webgl_delete_framebuffer(frame_buffer: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_delete_framebuffer(frame_buffer, state.get_inner_mut())
}

pub fn canvas_native_webgl_delete_program(program: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_delete_program(program, state.get_inner_mut())
}

pub fn canvas_native_webgl_delete_renderbuffer(render_buffer: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_delete_renderbuffer(
        render_buffer,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_delete_shader(shader: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_delete_shader(shader, state.get_inner_mut())
}

pub fn canvas_native_webgl_delete_texture(texture: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_delete_texture(texture, state.get_inner_mut())
}

pub fn canvas_native_webgl_depth_func(func: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_depth_func(func, state.get_inner_mut())
}

pub fn canvas_native_webgl_depth_mask(flag: bool, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_depth_mask(flag, state.get_inner_mut())
}

pub fn canvas_native_webgl_depth_range(z_near: f32, z_far: f32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_depth_range(z_near, z_far, state.get_inner_mut())
}

pub fn canvas_native_webgl_detach_shader(program: u32, shader: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_detach_shader(program, shader, state.get_inner_mut())
}

pub fn canvas_native_webgl_disable(cap: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_disable(cap, state.get_inner_mut())
}

pub fn canvas_native_webgl_disable_vertex_attrib_array(index: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_disable_vertex_attrib_array(
        index,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_draw_arrays(mode: u32, first: i32, count: i32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_draw_arrays(mode, first, count, state.get_inner_mut())
    // Flush Context
}

pub fn canvas_native_webgl_draw_elements(
    mode: u32,
    count: i32,
    element_type: u32,
    offset: isize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_draw_elements(
        mode,
        count,
        element_type,
        offset,
        state.get_inner_mut(),
    )
    // Flush Context
}

pub fn canvas_native_webgl_enable(cap: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_enable(cap, state.get_inner_mut())
}

pub fn canvas_native_webgl_enable_vertex_attrib_array(index: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_enable_vertex_attrib_array(
        index,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_finish(state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_finish(state.get_inner_mut())
}

pub fn canvas_native_webgl_flush(state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_flush(state.get_inner_mut())
}

pub fn canvas_native_webgl_framebuffer_renderbuffer(
    target: u32,
    attachment: u32,
    renderbuffertarget: u32,
    renderbuffer: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_framebuffer_renderbuffer(
        target,
        attachment,
        renderbuffertarget,
        renderbuffer,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_framebuffer_texture2d(
    target: u32,
    attachment: u32,
    textarget: u32,
    texture: u32,
    level: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_framebuffer_texture2d(
        target,
        attachment,
        textarget,
        texture,
        level,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_front_face(mode: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_front_face(mode, state.get_inner_mut())
}

pub fn canvas_native_webgl_generate_mipmap(target: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_generate_mipmap(target, state.get_inner_mut())
}

pub fn canvas_native_webgl_get_active_attrib(
    program: u32,
    index: u32,
    state: &mut WebGLState,
) -> Box<WebGLActiveInfo> {
    let info = canvas_webgl::webgl::canvas_native_webgl_get_active_attrib(
        program,
        index,
        state.get_inner_mut(),
    );

    Box::new(WebGLActiveInfo(info))
}

pub fn canvas_native_webgl_get_active_uniform(
    program: u32,
    index: u32,
    state: &mut WebGLState,
) -> Box<WebGLActiveInfo> {
    let info = canvas_webgl::webgl::canvas_native_webgl_get_active_uniform(
        program,
        index,
        state.get_inner_mut(),
    );

    Box::new(WebGLActiveInfo(info))
}

pub fn canvas_native_webgl_get_attached_shaders(program: u32, state: &mut WebGLState) -> Vec<u32> {
    canvas_webgl::webgl::canvas_native_webgl_get_attached_shaders(program, state.get_inner_mut())
}

pub fn canvas_native_webgl_get_attrib_location(
    program: u32,
    name: &str,
    state: &mut WebGLState,
) -> i32 {
    canvas_webgl::webgl::canvas_native_webgl_get_attrib_location(
        program,
        name,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_get_buffer_parameter(
    target: u32,
    pname: u32,
    state: &mut WebGLState,
) -> i32 {
    canvas_webgl::webgl::canvas_native_webgl_get_buffer_parameter(
        target,
        pname,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_get_context_attributes(
    state: &mut WebGLState,
) -> Box<ContextAttributes> {
    Box::new(ContextAttributes(
        canvas_webgl::webgl::canvas_native_webgl_get_context_attributes(state.get_inner()),
    ))
}

pub fn canvas_native_webgl_get_error(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl::canvas_native_webgl_get_error(state.get_inner_mut())
}

pub fn canvas_native_webgl_get_extension(
    name: &str,
    state: &mut WebGLState,
) -> Box<WebGLExtension> {
    Box::new(WebGLExtension(
        canvas_webgl::webgl::canvas_native_webgl_get_extension(name, state.get_inner_mut()),
    ))
}

pub fn canvas_native_webgl_get_framebuffer_attachment_parameter(
    target: u32,
    attachment: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLFramebufferAttachmentParameter> {
    Box::new(WebGLFramebufferAttachmentParameter(
        canvas_webgl::webgl::canvas_native_webgl_get_framebuffer_attachment_parameter(
            target,
            attachment,
            pname,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl_framebuffer_attachment_parameter_get_is_texture(
    param: &WebGLFramebufferAttachmentParameter,
) -> bool {
    param.get_is_texture()
}

pub fn canvas_native_webgl_framebuffer_attachment_parameter_get_is_renderbuffer(
    param: &WebGLFramebufferAttachmentParameter,
) -> bool {
    param.get_is_renderbuffer()
}

pub fn canvas_native_webgl_framebuffer_attachment_parameter_get_value(
    param: &WebGLFramebufferAttachmentParameter,
) -> i32 {
    param.get_value()
}

pub fn canvas_native_webgl_get_parameter(pname: u32, state: &mut WebGLState) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_parameter(pname, state.get_inner_mut()),
    ))
}

pub fn canvas_native_webgl_get_program_info_log(program: u32, state: &mut WebGLState) -> String {
    canvas_webgl::webgl::canvas_native_webgl_get_program_info_log(program, state.get_inner_mut())
}

pub fn canvas_native_webgl_get_program_parameter(
    program: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_program_parameter(
            program,
            pname,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl_get_renderbuffer_parameter(
    target: u32,
    pname: u32,
    state: &mut WebGLState,
) -> i32 {
    canvas_webgl::webgl::canvas_native_webgl_get_renderbuffer_parameter(
        target,
        pname,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_get_shader_info_log(shader: u32, state: &mut WebGLState) -> String {
    canvas_webgl::webgl::canvas_native_webgl_get_shader_info_log(shader, state.get_inner_mut())
}

pub fn canvas_native_webgl_get_shader_parameter(
    shader: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_shader_parameter(
            shader,
            pname,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl_get_shader_precision_format(
    shader_type: u32,
    precision_type: u32,
    state: &mut WebGLState,
) -> Box<WebGLShaderPrecisionFormat> {
    Box::new(WebGLShaderPrecisionFormat(
        canvas_webgl::webgl::canvas_native_webgl_get_shader_precision_format(
            shader_type,
            precision_type,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl_get_shader_source(shader: u32, state: &mut WebGLState) -> String {
    canvas_webgl::webgl::canvas_native_webgl_get_shader_source(shader, state.get_inner_mut())
}

pub fn canvas_native_webgl_get_supported_extensions(state: &mut WebGLState) -> Vec<String> {
    canvas_webgl::webgl::canvas_native_webgl_get_supported_extensions(state.get_inner_mut())
}

pub fn canvas_native_webgl_get_supported_extensions_to_string(state: &mut WebGLState) -> String {
    let ret =
        canvas_webgl::webgl::canvas_native_webgl_get_supported_extensions(state.get_inner_mut());
    ret.join(",").to_string()
}

pub fn canvas_native_webgl_get_tex_parameter(
    target: u32,
    pname: u32,
    state: &mut WebGLState,
) -> i32 {
    canvas_webgl::webgl::canvas_native_webgl_get_tex_parameter(target, pname, state.get_inner_mut())
}

pub fn canvas_native_webgl_get_uniform_location(
    program: u32,
    name: &str,
    state: &mut WebGLState,
) -> i32 {
    canvas_webgl::webgl::canvas_native_webgl_get_uniform_location(
        program,
        name,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_get_uniform(
    program: u32,
    location: i32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_uniform(
            program,
            location,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl_get_vertex_attrib_offset(
    index: u32,
    pname: u32,
    state: &mut WebGLState,
) -> usize {
    canvas_webgl::webgl::canvas_native_webgl_get_vertex_attrib_offset(
        index,
        pname,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_get_vertex_attrib(
    index: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_vertex_attrib(
            index,
            pname,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl_get_is_context_lost(_: &mut WebGLState) -> bool {
    // TODO improve
    false
}

pub fn canvas_native_webgl_hint(target: u32, mode: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_hint(target, mode, state.get_inner_mut())
}

pub fn canvas_native_webgl_is_buffer(buffer: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl::canvas_native_webgl_is_buffer(buffer, state.get_inner_mut())
}

pub fn canvas_native_webgl_is_enabled(cap: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl::canvas_native_webgl_is_enabled(cap, state.get_inner_mut())
}

pub fn canvas_native_webgl_is_framebuffer(framebuffer: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl::canvas_native_webgl_is_framebuffer(framebuffer, state.get_inner_mut())
}

pub fn canvas_native_webgl_is_program(program: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl::canvas_native_webgl_is_program(program, state.get_inner_mut())
}

pub fn canvas_native_webgl_is_renderbuffer(renderbuffer: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl::canvas_native_webgl_is_renderbuffer(renderbuffer, state.get_inner_mut())
}

pub fn canvas_native_webgl_is_shader(shader: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl::canvas_native_webgl_is_shader(shader, state.get_inner_mut())
}

pub fn canvas_native_webgl_is_texture(texture: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl::canvas_native_webgl_is_texture(texture, state.get_inner_mut())
}

pub fn canvas_native_webgl_line_width(width: f32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_line_width(width, state.get_inner_mut())
}

pub fn canvas_native_webgl_link_program(program: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_link_program(program, state.get_inner_mut())
}

pub fn canvas_native_webgl_pixel_storei(pname: u32, param: i32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_pixel_storei(pname, param, state.get_inner_mut())
}

pub fn canvas_native_webgl_polygon_offset(factor: f32, units: f32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_polygon_offset(factor, units, state.get_inner_mut())
}

pub fn canvas_native_webgl_read_pixels_u8(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    pixel_type: u32,
    pixels: &mut [u8],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_read_pixels_u8(
        x,
        y,
        width,
        height,
        format,
        pixel_type,
        pixels,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_read_pixels_u16(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    pixel_type: u32,
    pixels: &mut [u16],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_read_pixels_u16(
        x,
        y,
        width,
        height,
        format,
        pixel_type,
        pixels,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_read_pixels_f32(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    pixel_type: u32,
    pixels: &mut [f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_read_pixels_f32(
        x,
        y,
        width,
        height,
        format,
        pixel_type,
        pixels,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_renderbuffer_storage(
    target: u32,
    internal_format: u32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_renderbuffer_storage(
        target,
        internal_format,
        width,
        height,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_sample_coverage(value: f32, invert: bool, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_sample_coverage(value, invert, state.get_inner_mut())
}

pub fn canvas_native_webgl_scissor(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_scissor(x, y, width, height, state.get_inner_mut())
}

pub fn canvas_native_webgl_shader_source(shader: u32, source: &str, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_shader_source(shader, source, state.get_inner_mut())
}

pub fn canvas_native_webgl_stencil_func(
    func: u32,
    reference: i32,
    mask: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_stencil_func(
        func,
        reference,
        mask,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_stencil_func_separate(
    face: u32,
    func: u32,
    reference: i32,
    mask: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_stencil_func_separate(
        face,
        func,
        reference,
        mask,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_stencil_mask(mask: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_stencil_mask(mask, state.get_inner_mut())
}

pub fn canvas_native_webgl_stencil_mask_separate(face: u32, mask: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_stencil_mask_separate(
        face,
        mask,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_stencil_op(fail: u32, zfail: u32, zpass: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_stencil_op(fail, zfail, zpass, state.get_inner())
}

pub fn canvas_native_webgl_stencil_op_separate(
    face: u32,
    fail: u32,
    zfail: u32,
    zpass: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_stencil_op_separate(
        face,
        fail,
        zfail,
        zpass,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_tex_image2d_image_none(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d_image_none(
        target,
        level,
        internalformat,
        format,
        image_type,
        state.get_inner_mut(),
    )
}

fn canvas_native_webgl_tex_image2d_canvas2d(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    canvas: &mut CanvasRenderingContext2D,
    state: &mut WebGLState,
) {
    {
        let context = &mut state.0;
        context.remove_if_current();
    }

    let width;
    let height;
    let source_non_gpu;
    {
        let ctx = canvas.get_context();
        let device = ctx.device();
        width = device.width;
        height = device.height;
        source_non_gpu = device.non_gpu;
    }

    let mut source_ctx = canvas.get_context_mut();
    let mut buf;
    if !source_non_gpu {
        canvas.gl_context.make_current();
        buf = source_ctx.snapshot_to_raster_data();
        canvas.gl_context.remove_if_current();
    } else {
        buf = source_ctx.read_pixels();
    }

    canvas_webgl::webgl::canvas_native_webgl_tex_image2d(
        target,
        level,
        internalformat,
        width as i32,
        height as i32,
        0,
        format,
        image_type,
        buf.as_mut_slice(),
        state.get_inner_mut(),
    );
}

fn canvas_native_webgl_tex_image2d_webgl(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    webgl: &mut WebGLState,
    state: &mut WebGLState,
) {
    let mut pixels =
        canvas_webgl::webgl::canvas_native_webgl_read_webgl_pixels(&mut webgl.0, &mut state.0);
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d(
        target,
        level,
        internalformat,
        pixels.0,
        pixels.1,
        0,
        format,
        image_type,
        pixels.2.as_mut_slice(),
        state.get_inner_mut(),
    );
}

pub fn canvas_native_webgl_tex_image2d(
    target: i32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    border: i32,
    format: i32,
    image_type: i32,
    buf: &mut [u8],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        format,
        image_type,
        buf,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_tex_image2d_none(
    target: i32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    border: i32,
    format: i32,
    image_type: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d_none(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        format,
        image_type,
        state.get_inner_mut(),
    )
}

fn canvas_native_webgl_tex_image2d_image_asset(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    image_asset: &mut ImageAsset,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d_asset(
        target,
        level,
        internalformat,
        format,
        image_type,
        &image_asset.0,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_tex_parameterf(
    target: u32,
    pname: u32,
    param: f32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_tex_parameterf(target, pname, param, state.get_inner())
}

pub fn canvas_native_webgl_tex_parameteri(
    target: u32,
    pname: u32,
    param: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_tex_parameteri(target, pname, param, state.get_inner())
}

pub fn canvas_native_webgl_tex_sub_image2d_asset(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    format: u32,
    image_type: i32,
    asset: &mut ImageAsset,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_tex_sub_image2d_asset(
        target,
        level,
        xoffset,
        yoffset,
        format,
        image_type,
        &asset.0,
        state.get_inner(),
    )
}

pub fn canvas_native_webgl_tex_sub_image2d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    width: i32,
    height: i32,
    format: u32,
    image_type: i32,
    buf: &[u8],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_tex_sub_image2d(
        target,
        level,
        xoffset,
        yoffset,
        width,
        height,
        format,
        image_type,
        buf,
        state.get_inner(),
    )
}

pub fn canvas_native_webgl_uniform1f(location: i32, v0: f32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform1f(location, v0, state.get_inner())
}

pub fn canvas_native_webgl_uniform1fv(location: i32, value: &[f32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform1fv(location, value, state.get_inner())
}

pub fn canvas_native_webgl_uniform1i(location: i32, v0: i32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform1i(location, v0, state.get_inner())
}

pub fn canvas_native_webgl_uniform1iv(location: i32, value: &[i32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform1iv(location, value, state.get_inner())
}

pub fn canvas_native_webgl_uniform2f(location: i32, v0: f32, v1: f32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform2f(location, v0, v1, state.get_inner())
}

pub fn canvas_native_webgl_uniform2fv(location: i32, value: &[f32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform2fv(location, value, state.get_inner())
}

pub fn canvas_native_webgl_uniform2i(location: i32, v0: i32, v1: i32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform2i(location, v0, v1, state.get_inner())
}

pub fn canvas_native_webgl_uniform2iv(location: i32, value: &[i32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform2iv(location, value, state.get_inner())
}

pub fn canvas_native_webgl_uniform3f(
    location: i32,
    v0: f32,
    v1: f32,
    v2: f32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_uniform3f(location, v0, v1, v2, state.get_inner())
}

pub fn canvas_native_webgl_uniform3fv(location: i32, value: &[f32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform3fv(location, value, state.get_inner())
}

pub fn canvas_native_webgl_uniform3i(
    location: i32,
    v0: i32,
    v1: i32,
    v2: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_uniform3i(location, v0, v1, v2, state.get_inner())
}

pub fn canvas_native_webgl_uniform3iv(location: i32, value: &[i32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform3iv(location, value, state.get_inner())
}

pub fn canvas_native_webgl_uniform4f(
    location: i32,
    v0: f32,
    v1: f32,
    v2: f32,
    v3: f32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_uniform4f(location, v0, v1, v2, v3, state.get_inner())
}

pub fn canvas_native_webgl_uniform4fv(location: i32, value: &[f32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform4fv(location, value, state.get_inner_mut())
}

pub fn canvas_native_webgl_uniform4i(
    location: i32,
    v0: i32,
    v1: i32,
    v2: i32,
    v3: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_uniform4i(
        location,
        v0,
        v1,
        v2,
        v3,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_uniform4iv(location: i32, value: &[i32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_uniform4iv(location, value, state.get_inner_mut())
}

pub fn canvas_native_webgl_uniform_matrix2fv(
    location: i32,
    transpose: bool,
    value: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_uniform_matrix2fv(
        location,
        transpose,
        value,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_uniform_matrix3fv(
    location: i32,
    transpose: bool,
    value: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_uniform_matrix3fv(
        location,
        transpose,
        value,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_uniform_matrix4fv(
    location: i32,
    transpose: bool,
    value: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_uniform_matrix4fv(
        location,
        transpose,
        value,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_use_program(program: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_use_program(program, state.get_inner_mut())
}

pub fn canvas_native_webgl_validate_program(program: u32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_validate_program(program, state.get_inner_mut())
}

pub fn canvas_native_webgl_vertex_attrib1f(index: u32, v0: f32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib1f(index, v0, state.get_inner_mut())
}

pub fn canvas_native_webgl_vertex_attrib1fv(index: u32, value: &[f32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib1fv(index, value, state.get_inner_mut())
}

pub fn canvas_native_webgl_vertex_attrib2f(index: u32, v0: f32, v1: f32, state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib2f(index, v0, v1, state.get_inner_mut())
}

pub fn canvas_native_webgl_vertex_attrib2fv(index: u32, value: &[f32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib2fv(index, value, state.get_inner_mut())
}

pub fn canvas_native_webgl_vertex_attrib3f(
    index: u32,
    v0: f32,
    v1: f32,
    v2: f32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib3f(
        index,
        v0,
        v1,
        v2,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_vertex_attrib3fv(index: u32, value: &[f32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib3fv(index, value, state.get_inner_mut())
}

pub fn canvas_native_webgl_vertex_attrib4f(
    index: u32,
    v0: f32,
    v1: f32,
    v2: f32,
    v3: f32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib4f(
        index,
        v0,
        v1,
        v2,
        v3,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_vertex_attrib4fv(index: u32, value: &[f32], state: &mut WebGLState) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib4fv(index, value, state.get_inner_mut())
}

pub fn canvas_native_webgl_vertex_attrib_pointer(
    index: u32,
    size: i32,
    d_type: u32,
    normalized: bool,
    stride: i32,
    offset: isize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib_pointer(
        index,
        size,
        d_type,
        normalized,
        stride,
        offset,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl_viewport(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl::canvas_native_webgl_viewport(x, y, width, height, state.get_inner_mut())
}

/* WebGL */

/* WEBGL IMPL */

/* WEBGL2 IMPL */

/* WebGL2 */

pub struct WebGLSync(canvas_webgl::webgl2::GLSync);

pub struct WebGLIndexedParameter(canvas_webgl::prelude::WebGLIndexedParameter);

/* WebGLIndexedParameter */
pub fn canvas_native_webgl2_indexed_parameter_get_value(param: &WebGLIndexedParameter) -> isize {
    param.0.get_value() as isize
}

pub fn canvas_native_webgl2_indexed_parameter_get_buffer_value(
    param: &WebGLIndexedParameter,
) -> isize {
    param.0.get_buffer_value() as isize
}

pub fn canvas_native_webgl2_indexed_parameter_get_is_buffer(param: &WebGLIndexedParameter) -> bool {
    param.0.get_is_buffer()
}
/* WebGLIndexedParameter */

pub fn canvas_native_webgl2_begin_query(target: u32, id: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_begin_query(target, id, state.get_inner_mut())
}

pub fn canvas_native_webgl2_begin_transform_feedback(primitive_mode: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_begin_transform_feedback(
        primitive_mode,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_bind_buffer_base(
    target: u32,
    index: u32,
    buffer: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_bind_buffer_base(
        target,
        index,
        buffer,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_bind_buffer_range(
    target: u32,
    index: u32,
    buffer: u32,
    offset: isize,
    size: isize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_bind_buffer_range(
        target,
        index,
        buffer,
        offset,
        size,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_bind_sampler(unit: u32, sampler: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_bind_sampler(unit, sampler, state.get_inner_mut())
}

pub fn canvas_native_webgl2_bind_transform_feedback(
    target: u32,
    transform_feedback: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_bind_sampler(
        target,
        transform_feedback,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_bind_vertex_array(vertex_array: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_bind_vertex_array(
        vertex_array,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_blit_framebuffer(
    src_x0: i32,
    src_y0: i32,
    src_x1: i32,
    src_y1: i32,
    dst_x0: i32,
    dst_y0: i32,
    dst_x1: i32,
    dst_y1: i32,
    mask: u32,
    filter: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_blit_framebuffer(
        src_x0,
        src_y0,
        src_x1,
        src_y1,
        dst_x0,
        dst_y0,
        dst_x1,
        dst_y1,
        mask,
        filter,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_clear_bufferfi(
    buffer: u32,
    drawbuffer: i32,
    depth: f32,
    stencil: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_clear_bufferfi(
        buffer,
        drawbuffer,
        depth,
        stencil,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_clear_bufferfv(
    buffer: u32,
    drawbuffer: i32,
    values: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_clear_bufferfv(
        buffer,
        drawbuffer,
        values,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_clear_bufferiv(
    buffer: u32,
    drawbuffer: i32,
    values: &[i32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_clear_bufferiv(
        buffer,
        drawbuffer,
        values,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_clear_bufferuiv(
    buffer: u32,
    drawbuffer: i32,
    values: &[u32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_clear_bufferuiv(
        buffer,
        drawbuffer,
        values,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_client_wait_sync(
    sync: &WebGLSync,
    flags: u32,
    timeout: isize,
    state: &mut WebGLState,
) -> u32 {
    canvas_webgl::webgl2::canvas_native_webgl2_client_wait_sync(
        &sync.0,
        flags,
        timeout as c_ulong,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_compressed_tex_sub_image3d_none(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    image_size: i32,
    offset: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_compressed_tex_sub_image3d_none(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        image_size,
        offset,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_compressed_tex_sub_image3d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    src: &[u8],
    src_offset: usize,
    src_length_override: usize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_compressed_tex_sub_image3d(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        src,
        src_offset,
        src_length_override,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_copy_buffer_sub_data(
    read_target: u32,
    write_target: u32,
    read_offset: isize,
    write_offset: isize,
    size: isize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_copy_buffer_sub_data(
        read_target,
        write_target,
        read_offset,
        write_offset,
        size,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_copy_tex_sub_image3d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_copy_tex_sub_image3d(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        x,
        y,
        width,
        height,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_create_query(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl2::canvas_native_webgl2_create_query(state.get_inner_mut())
}

pub fn canvas_native_webgl2_create_sampler(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl2::canvas_native_webgl2_create_sampler(state.get_inner_mut())
}

pub fn canvas_native_webgl2_create_transform_feedback(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl2::canvas_native_webgl2_create_transform_feedback(state.get_inner_mut())
}

pub fn canvas_native_webgl2_create_vertex_array(state: &mut WebGLState) -> u32 {
    canvas_webgl::webgl2::canvas_native_webgl2_create_vertex_array(state.get_inner_mut())
}

pub fn canvas_native_webgl2_delete_query_with_query(id: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_delete_query_with_query(id, state.get_inner_mut())
}

pub fn canvas_native_webgl2_delete_sampler_with_sampler(sampler: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_delete_sampler_with_sampler(
        sampler,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_delete_sync_with_sync(sync: &WebGLSync, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_delete_sync_with_sync(&sync.0, state.get_inner_mut())
}

pub fn canvas_native_webgl2_delete_transform_feedback(
    transform_feedback: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_delete_transform_feedback(
        transform_feedback,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_delete_vertex_array_with_vertex_array(
    vertex_array: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_delete_vertex_array_with_vertex_array(
        vertex_array,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_draw_arrays_instanced(
    mode: u32,
    first: i32,
    count: i32,
    instance_count: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_draw_arrays_instanced(
        mode,
        first,
        count,
        instance_count,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_draw_buffers(buffers: &[u32], state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_draw_buffers(buffers, state.get_inner_mut())
}

pub fn canvas_native_webgl2_draw_elements_instanced(
    mode: u32,
    count: i32,
    type_: u32,
    offset: isize,
    instance_count: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_draw_elements_instanced(
        mode,
        count,
        type_,
        offset,
        instance_count,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_draw_range_elements(
    mode: u32,
    start: u32,
    end: u32,
    count: i32,
    type_: u32,
    offset: isize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_draw_range_elements(
        mode,
        start,
        end,
        count,
        type_,
        offset,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_end_query(target: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_end_query(target, state.get_inner_mut())
}

pub fn canvas_native_webgl2_end_transform_feedback(state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_end_transform_feedback(state.get_inner_mut())
}

pub fn canvas_native_webgl2_fence_sync(
    condition: u32,
    flags: u32,
    state: &mut WebGLState,
) -> Box<WebGLSync> {
    Box::new(WebGLSync(
        canvas_webgl::webgl2::canvas_native_webgl2_fence_sync(
            condition,
            flags,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_framebuffer_texture_layer(
    target: u32,
    attachment: u32,
    texture: u32,
    level: i32,
    layer: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_framebuffer_texture_layer(
        target,
        attachment,
        texture,
        level,
        layer,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_get_active_uniform_block_name(
    program: u32,
    uniform_block_index: u32,
    state: &mut WebGLState,
) -> String {
    canvas_webgl::webgl2::canvas_native_webgl2_get_active_uniform_block_name(
        program,
        uniform_block_index,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_get_active_uniform_block_parameter(
    program: u32,
    uniform_block_index: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_active_uniform_block_parameter(
            program,
            uniform_block_index,
            pname,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_get_active_uniforms(
    program: u32,
    uniform_indices: &[u32],
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_active_uniforms(
            program,
            uniform_indices,
            pname,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_get_buffer_sub_data(
    target: u32,
    src_byte_offset: isize,
    dst_data: &mut [u8],
    dst_offset: usize,
    length: usize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_get_buffer_sub_data(
        target,
        src_byte_offset,
        dst_data,
        dst_offset,
        length,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_get_frag_data_location(
    program: u32,
    name: &str,
    state: &mut WebGLState,
) -> i32 {
    canvas_webgl::webgl2::canvas_native_webgl2_get_frag_data_location(
        program,
        name,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_get_indexed_parameter(
    target: u32,
    index: u32,
    state: &mut WebGLState,
) -> Box<WebGLIndexedParameter> {
    Box::new(WebGLIndexedParameter(
        canvas_webgl::webgl2::canvas_native_webgl2_get_indexed_parameter(
            target,
            index,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_get_internalformat_parameter(
    target: u32,
    internalformat: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_internalformat_parameter(
            target,
            internalformat,
            pname,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_get_parameter(pname: u32, state: &mut WebGLState) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_parameter(pname, state.get_inner_mut()),
    ))
}

pub fn canvas_native_webgl2_get_query_parameter(
    query: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_query_parameter(
            query,
            pname,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_get_query(
    target: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_query(target, pname, state.get_inner_mut()),
    ))
}

pub fn canvas_native_webgl2_get_sampler_parameter(
    sampler: u32,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_sampler_parameter(
            sampler,
            pname,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_get_sync_parameter(
    sync: &WebGLSync,
    pname: u32,
    state: &mut WebGLState,
) -> Box<WebGLResult> {
    Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_sync_parameter(
            &sync.0,
            pname,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_get_transform_feedback_varying(
    program: u32,
    index: u32,
    state: &mut WebGLState,
) -> Box<WebGLActiveInfo> {
    Box::new(WebGLActiveInfo(
        canvas_webgl::webgl2::canvas_native_webgl2_get_transform_feedback_varying(
            program,
            index,
            state.get_inner_mut(),
        ),
    ))
}

pub fn canvas_native_webgl2_get_uniform_block_index(
    program: u32,
    uniform_block_name: &str,
    state: &mut WebGLState,
) -> u32 {
    canvas_webgl::webgl2::canvas_native_webgl2_get_uniform_block_index(
        program,
        uniform_block_name,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_get_uniform_indices(
    program: u32,
    uniform_names: &[&str],
    state: &mut WebGLState,
) -> Vec<u32> {
    let uniform_names: Vec<String> = uniform_names.iter().map(|i| i.to_string()).collect();
    canvas_webgl::webgl2::canvas_native_webgl2_get_uniform_indices(
        program,
        uniform_names.as_slice(),
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_invalidate_framebuffer(
    target: u32,
    attachments: &[u32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_invalidate_framebuffer(
        target,
        attachments,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_invalidate_sub_framebuffer(
    target: u32,
    attachments: &[u32],
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_invalidate_sub_framebuffer(
        target,
        attachments,
        x,
        y,
        width,
        height,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_is_query(query: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl2::canvas_native_webgl2_is_query(query, state.get_inner_mut())
}

pub fn canvas_native_webgl2_is_sampler(sampler: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl2::canvas_native_webgl2_is_sampler(sampler, state.get_inner_mut())
}

pub fn canvas_native_webgl2_is_sync(sync: &WebGLSync, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl2::canvas_native_webgl2_is_sync(&sync.0, state.get_inner_mut())
}

pub fn canvas_native_webgl2_is_transform_feedback(
    transform_feedback: u32,
    state: &mut WebGLState,
) -> bool {
    canvas_webgl::webgl2::canvas_native_webgl2_is_transform_feedback(
        transform_feedback,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_is_vertex_array(vertex_array: u32, state: &mut WebGLState) -> bool {
    canvas_webgl::webgl2::canvas_native_webgl2_is_vertex_array(vertex_array, state.get_inner_mut())
}

pub fn canvas_native_webgl2_pause_transform_feedback(state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_pause_transform_feedback(state.get_inner_mut())
}

pub fn canvas_native_webgl2_read_buffer(src: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_read_buffer(src, state.get_inner_mut())
}

pub fn canvas_native_webgl2_renderbuffer_storage_multisample(
    target: u32,
    samples: i32,
    internal_format: u32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_renderbuffer_storage_multisample(
        target,
        samples,
        internal_format,
        width,
        height,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_resume_transform_feedback(state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_resume_transform_feedback(state.get_inner_mut())
}

pub fn canvas_native_webgl2_sampler_parameterf(
    sampler: u32,
    pname: u32,
    param: f32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_sampler_parameterf(
        sampler,
        pname,
        param,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_sampler_parameteri(
    sampler: u32,
    pname: u32,
    param: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_sampler_parameteri(
        sampler,
        pname,
        param,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_tex_image3d_none(
    target: u32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: u32,
    offset: usize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_tex_image3d_none(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        offset,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_tex_image3d_asset(
    target: u32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: u32,
    asset: &ImageAsset,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_tex_image3d_asset(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        &asset.0,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_tex_image3d(
    target: u32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: u32,
    buf: &[u8],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_tex_image3d(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        buf,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_tex_image3d_offset(
    target: u32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: u32,
    buf: &[u8],
    offset: usize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_tex_image3d_offset(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        buf,
        offset,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_tex_storage2d(
    target: u32,
    levels: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_tex_storage2d(
        target,
        levels,
        internalformat,
        width,
        height,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_tex_storage3d(
    target: u32,
    levels: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    depth: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_tex_storage3d(
        target,
        levels,
        internalformat,
        width,
        height,
        depth,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_tex_sub_image3d_none(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    type_: u32,
    offset: usize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_tex_sub_image3d_none(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        type_,
        offset,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_tex_sub_image3d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    type_: u32,
    buf: &[u8],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_tex_sub_image3d(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        type_,
        buf,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_tex_sub_image3d_asset(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    type_: u32,
    asset: &ImageAsset,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_tex_sub_image3d_asset(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        type_,
        &asset.0,
        state.get_inner_mut(),
    );
}

pub fn canvas_native_webgl2_tex_sub_image3d_offset(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    type_: u32,
    buf: &[u8],
    offset: usize,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_tex_sub_image3d_offset(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        type_,
        buf,
        offset,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_transform_feedback_varyings(
    program: u32,
    varyings: &[&str],
    buffer_mode: u32,
    state: &mut WebGLState,
) {
    let varyings: Vec<String> = varyings.iter().map(|i| i.to_string()).collect();
    canvas_webgl::webgl2::canvas_native_webgl2_transform_feedback_varyings(
        program,
        varyings.as_slice(),
        buffer_mode,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform1ui(location: i32, v0: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform1ui(location, v0, state.get_inner_mut())
}

pub fn canvas_native_webgl2_uniform1uiv(location: i32, data: &[u32], state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform1uiv(location, data, state.get_inner_mut())
}

pub fn canvas_native_webgl2_uniform2ui(location: i32, v0: u32, v1: u32, state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform2ui(location, v0, v1, state.get_inner_mut())
}

pub fn canvas_native_webgl2_uniform2uiv(location: i32, data: &[u32], state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform2uiv(location, data, state.get_inner_mut())
}

pub fn canvas_native_webgl2_uniform3ui(
    location: i32,
    v0: u32,
    v1: u32,
    v2: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform3ui(
        location,
        v0,
        v1,
        v2,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform3uiv(location: i32, data: &[u32], state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform3uiv(location, data, state.get_inner_mut())
}

pub fn canvas_native_webgl2_uniform4ui(
    location: i32,
    v0: u32,
    v1: u32,
    v2: u32,
    v3: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform4ui(
        location,
        v0,
        v1,
        v2,
        v3,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform4uiv(location: i32, data: &[u32], state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform4uiv(location, data, state.get_inner_mut())
}

pub fn canvas_native_webgl2_uniform_block_binding(
    program: u32,
    uniform_block_index: u32,
    uniform_block_binding: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_block_binding(
        program,
        uniform_block_index,
        uniform_block_binding,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform_matrix2x3fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix2x3fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform_matrix2x4fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix2x4fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform_matrix3x2fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix3x2fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform_matrix3x4fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix3x4fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform_matrix4x2fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix4x2fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_uniform_matrix4x3fv(
    location: i32,
    transpose: bool,
    data: &[f32],
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix4x3fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_vertex_attrib_divisor(
    index: u32,
    divisor: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_divisor(
        index,
        divisor,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_vertex_attrib_i4i(
    index: u32,
    x: i32,
    y: i32,
    z: i32,
    w: i32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_i4i(
        index,
        x,
        y,
        z,
        w,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_vertex_attrib_i4iv(index: u32, value: &[i32], state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_i4iv(
        index,
        value,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_vertex_attrib_i4ui(
    index: u32,
    x: u32,
    y: u32,
    z: u32,
    w: u32,
    state: &mut WebGLState,
) {
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_i4ui(
        index,
        x,
        y,
        z,
        w,
        state.get_inner_mut(),
    )
}

pub fn canvas_native_webgl2_vertex_attrib_i4uiv(index: u32, value: &[u32], state: &mut WebGLState) {
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_i4uiv(
        index,
        value,
        state.get_inner_mut(),
    )
}

/* WebGL2 */

/* WEBGL2 IMPL */
