use std::cmp::PartialEq;
use std::ffi::c_uint;

use base64::Engine;
use regex::bytes::Replacer;
use skia_safe::image::CachingHint;
use skia_safe::BlendMode;
use skia_safe::{AlphaType, Color, ColorType, EncodedImageFormat, IPoint, ISize, Image, ImageInfo, Point, Surface};

use compositing::composite_operation_type::CompositeOperationType;
use fill_and_stroke_styles::paint::Paint;
use filter_quality::FilterQuality;
use image_smoothing::ImageSmoothingQuality;
use line_styles::line_cap::LineCap;
use line_styles::line_join::LineJoin;
use paths::path::Path;
use text_styles::{
    text_align::TextAlign, text_baseline::TextBaseLine, text_direction::TextDirection,
};

use crate::context::drawing_text::typography::Font;

use bitflags::bitflags;

pub mod drawing_images;
pub mod drawing_text;
pub mod fill_and_stroke_styles;
pub mod paths;
pub mod pixel_manipulation;
pub mod text_styles;

pub mod compositing;
pub mod drawing_paths;
pub mod drawing_rectangles;
pub mod filters;
pub mod gradients_and_patterns;

pub mod image_smoothing;
pub mod line_styles;
pub mod shadows;
pub mod state;

pub mod filter_quality;
pub mod matrix;
pub mod non_standard;
pub mod surface;
pub mod surface_gl;
pub mod text_decoder;
pub mod text_encoder;
pub mod transformations;

// #[cfg(feature = "vulkan")]
pub mod surface_vulkan;

#[cfg(feature = "metal")]
pub mod surface_metal;


#[derive(Clone)]
pub struct State {
    pub(crate) direction: TextDirection,
    pub(crate) paint: Paint,
    pub(crate) font: String,
    pub(crate) font_style: Font,
    pub(crate) text_align: TextAlign,
    pub(crate) text_baseline: TextBaseLine,
    pub(crate) shadow_color: Color,
    pub(crate) shadow_offset: Point,
    pub(crate) shadow_blur: f32,
    pub(crate) image_smoothing_enabled: bool,
    pub(crate) image_smoothing_quality: ImageSmoothingQuality,
    pub(crate) line_width: f32,
    pub(crate) line_cap: LineCap,
    pub(crate) line_join: LineJoin,
    pub(crate) miter_limit: f32,
    pub(crate) line_dash_list: Vec<f32>,
    pub(crate) line_dash_offset: f32,
    pub(crate) filter: String,
    pub(crate) global_alpha: f32,
    pub(crate) global_composite_operation: CompositeOperationType,
    pub(crate) word_spacing_value: String,
    pub(crate) word_spacing: f32,
    pub(crate) letter_spacing_value: String,
    pub(crate) letter_spacing: f32,
    pub(crate) matrix: skia_safe::Matrix,
    pub(crate) clip: Option<Path>,
}

impl Default for State {
    fn default() -> Self {
        let mut paint = Paint::default();
        paint
            .stroke_paint_mut()
            .set_stroke_width(1.0)
            .set_stroke_miter(10.0);
        Self {
            direction: TextDirection::LTR,
            paint,
            font: "10px sans-serif".to_owned(),
            font_style: Font::default(),
            text_align: TextAlign::default(),
            text_baseline: TextBaseLine::default(),
            shadow_color: Color::TRANSPARENT,
            shadow_offset: (0.0, 0.0).into(),
            shadow_blur: 0.0,
            image_smoothing_enabled: false,
            image_smoothing_quality: ImageSmoothingQuality::default(),
            line_width: 1.,
            line_cap: LineCap::default(),
            line_join: LineJoin::default(),
            miter_limit: 10.0,
            line_dash_list: Default::default(),
            line_dash_offset: 0.0,
            filter: "none".into(),
            global_alpha: 1.0,
            global_composite_operation: CompositeOperationType::default(),
            word_spacing_value: "0px".to_string(),
            word_spacing: 0.,
            letter_spacing_value: "0px".to_string(),
            letter_spacing: 0.,
            matrix: skia_safe::Matrix::new_identity(),
            clip: None,
        }
    }
}

impl State {
    pub(crate) fn image_filter_quality(&self) -> FilterQuality {
        if self.image_smoothing_enabled {
            self.image_smoothing_quality.into()
        } else {
            FilterQuality::None
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SurfaceEngine {
    CPU,
    GL,
    Vulkan,
    Metal,
}


bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SurfaceState: u8 {
        const None = 0x00;
        const Pending = 0x01;
        const Invalidating = 0x02;
    }
}
impl Default for SurfaceState {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SurfaceData {
    pub(crate) bounds: skia_safe::Rect,
    pub(crate) scale: f32,
    pub(crate) ppi: f32,
    pub(crate) engine: SurfaceEngine,
    pub(crate) state: SurfaceState,
    pub(crate) is_opaque: bool,
}

impl SurfaceData {
    pub fn width(&self) -> f32 {
        self.bounds.width()
    }

    pub fn height(&self) -> f32 {
        self.bounds.height()
    }

    pub fn ppi(&self) -> f32 {
        self.ppi
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn is_opaque(&self) -> bool {
        self.is_opaque
    }
}

pub struct Context {
    pub(crate) surface_data: SurfaceData,
    pub(crate) surface: Surface,
    pub(crate) surface_state: SurfaceState,
    #[cfg(feature = "vulkan")]
    pub vulkan_context: Option<canvas_core::gpu::vulkan::VulkanContext>,
    #[cfg(feature = "vulkan")]
    pub vulkan_texture: Option<skia_safe::gpu::BackendTexture>,
    #[cfg(feature = "metal")]
    pub metal_context: Option<canvas_core::gpu::metal::MetalContext>,
    #[cfg(feature = "metal")]
    pub metal_texture_info: Option<skia_safe::gpu::mtl::TextureInfo>,
    #[cfg(feature = "gl")]
    pub gl_context: Option<canvas_core::gpu::gl::GLContext>,
    #[cfg(any(feature = "gl", feature = "vulkan", feature = "metal"))]
    pub(crate) direct_context: Option<skia_safe::gpu::DirectContext>,
    pub(crate) path: Path,
    pub(crate) state: State,
    pub(crate) state_stack: Vec<State>,
    pub(crate) font_color: Color,
}


impl Context {
    pub fn get_pixels(&mut self, buffer: &mut [u8], origin: impl Into<IPoint>, size: impl Into<ISize>) {
        let origin = origin.into();
        let size = size.into();
        let info = ImageInfo::new(size, ColorType::RGBA8888, AlphaType::Unpremul, None);

        if let Some(img) = self.get_image() {
            img.read_pixels(&info, buffer, info.min_row_bytes(), origin, CachingHint::Allow);
        }
    }


    pub fn submit(&mut self) {
        #[cfg(any(feature = "gl", feature = "vulkan", feature = "metal"))]
        match self.direct_context.as_mut() {
            Some(ctx) => {
                ctx.submit(None);
            }
            _ => {}
        }
    }

    pub fn flush_and_render_to_surface(&mut self) {
        self.flush();
    }

    pub fn flush_surface(&mut self) {
        #[cfg(any(feature = "gl", feature = "vulkan", feature = "metal"))]
        match self.direct_context.as_mut() {
            Some(ctx) => {
                ctx.flush_and_submit();
            }
            _ => {}
        }
    }

    pub fn flush_submit_and_sync_cpu(&mut self) {
        #[cfg(any(feature = "gl", feature = "vulkan", feature = "metal"))]
        match self.direct_context.as_mut() {
            Some(ctx) => {
                ctx.flush_submit_and_sync_cpu();
            }
            _ => {}
        }
    }

    pub fn surface_engine(&self) -> SurfaceEngine {
        self.surface_data.engine
    }

    pub fn surface_data(&self) -> &SurfaceData {
        &self.surface_data
    }

    pub fn surface_state(&self) -> SurfaceState {
        self.surface_state
    }

    pub fn with_matrix<F>(&mut self, f: F)
    where
        F: FnOnce(&mut skia_safe::Matrix) -> &skia_safe::Matrix,
    {
        f(&mut self.state.matrix);
    }

    pub fn dimensions(&self) -> (f32, f32) {
        (self.surface_data.bounds.width(), self.surface_data.bounds.height())
    }

    pub fn width(&self) -> f32 {
        self.surface_data.bounds.width()
    }
    pub fn height(&self) -> f32 {
        self.surface_data.bounds.height()
    }
    pub fn density(&self) -> f32 {
        self.surface_data.scale
    }

    pub fn with_canvas<F>(&mut self, f: F)
    where
        F: FnOnce(&skia_safe::Canvas),
    {
        f(self.surface.canvas());
    }

    pub fn with_canvas_dirty<F>(&mut self, f: F)
    where
        F: FnOnce(&skia_safe::Canvas),
    {
        f(self.surface.canvas());
        self.surface_state = self.surface_state | SurfaceState::Pending;
    }


    pub fn set_bounds(&mut self, bounds: skia_safe::Rect) {}

    pub fn update_bounds(&mut self, bounds: skia_safe::Rect) {
        self.surface_data.bounds = bounds;
    }

    pub fn append<F>(&mut self, f: F)
    where
        F: FnOnce(&skia_safe::Canvas),
    {
        f(self.surface.canvas());
        self.surface_state = self.surface_state | SurfaceState::Pending;
    }


    pub fn flush(&mut self) {
        let state = self.surface_state & SurfaceState::Pending;

        if state == SurfaceState::Pending {
            self.surface_state = self.surface_state | SurfaceState::Invalidating;
            self.flush_surface();
            self.surface_state = self.surface_state | SurfaceState::None;
        }
    }

    pub fn flush_and_sync_cpu(&mut self) {
        let state = self.surface_state & SurfaceState::Pending;

        if state == SurfaceState::Pending {
            self.surface_state = self.surface_state | SurfaceState::Invalidating;
            self.flush_submit_and_sync_cpu();
            self.surface_state = self.surface_state | SurfaceState::None;
        }
    }

    pub fn get_image(&mut self) -> Option<Image> {
        #[cfg(feature = "gl")]{
            if let Some(ref context) = self.gl_context {
                context.make_current();
            }
        }

        self.flush();

        let snapshot = self.surface.image_snapshot();
        if self.surface_data.engine == SurfaceEngine::GL {
            snapshot.make_raster_image(
                self.direct_context.as_mut(),
                Some(CachingHint::Allow),
            )
        } else {
            Some(snapshot)
        }
    }
    pub fn as_data(&mut self) -> Vec<u8> {
        let bounds = self.surface_data.bounds;
        if bounds.is_empty() {
            return vec![];
        }
        let read_data = |image: &Image, bounds: skia_safe::Rect| {
            let width = bounds.width();
            let height = bounds.height();
            let mut info = ImageInfo::new(
                ISize::new(width as i32, height as i32),
                ColorType::RGBA8888,
                AlphaType::Unpremul,
                None,
            );
            let row_bytes = info.width() * 4;
            let mut pixels = vec![255u8; (row_bytes * info.height()) as usize];
            let _ = image.read_pixels(
                &mut info,
                pixels.as_mut_slice(),
                row_bytes as usize,
                IPoint::new(0, 0),
                CachingHint::Allow,
            );
            pixels
        };

        if let Some(ref cache) = self.get_image() {
            read_data(cache, bounds)
        } else {
            vec![]
        }
    }

    pub fn as_data_url(&mut self, format: &str, quality: c_uint) -> String {
        if self.surface_data.bounds.is_empty() {
            return "data:,".to_string();
        }

        //  self.flush();

        let image = self.surface.image_snapshot();

        let mut quality = quality;
        if quality > 100 {
            quality = 92;
        }
        let data_txt = "data:";
        let base_64_txt = ";base64,";
        let mut encoded_prefix =
            String::with_capacity(data_txt.len() + format.len() + base_64_txt.len());
        encoded_prefix.push_str("data:");
        encoded_prefix.push_str(format);
        encoded_prefix.push_str(";base64,");
        let data = image.encode(
            self.direct_context.as_mut(),
            match format {
                "image/jpg" | "image/jpeg" => EncodedImageFormat::JPEG,
                "image/webp" => EncodedImageFormat::WEBP,
                "image/gif" => EncodedImageFormat::GIF,
                "image/heif" | "image/heic" | "image/heif-sequence" | "image/heic-sequence" => {
                    EncodedImageFormat::HEIF
                }
                _ => EncodedImageFormat::PNG,
            },
            quality,
        );
        return match data {
            Some(data) => {
                let encoded_data =
                    base64::engine::general_purpose::STANDARD.encode(data.as_bytes());
                if encoded_data.is_empty() {
                    return "data:,".to_string();
                }
                let mut encoded =
                    String::with_capacity(encoded_prefix.len() + encoded_data.len());
                encoded.push_str(&encoded_prefix);
                encoded.push_str(&encoded_data);
                encoded
            }
            _ => "data:,".to_string(),
        };
    }

    pub fn render_to_canvas<F>(&mut self, paint: &skia_safe::Paint, f: F)
    where
        F: Fn(&skia_safe::Canvas, &skia_safe::Paint),
    {
        let blend = self.state.global_composite_operation.get_blend_mode();
        match blend {
            BlendMode::SrcIn | BlendMode::SrcOut |
            BlendMode::DstIn | BlendMode::DstOut |
            BlendMode::DstATop | BlendMode::Src => {
                let mut layer_paint = paint.clone();
                layer_paint.set_anti_alias(true);
                layer_paint.set_blend_mode(BlendMode::SrcOver);
                let mut layer_recorder = skia_safe::PictureRecorder::new();
                layer_recorder.begin_recording(self.surface_data.bounds, None);
                let current_matrix = self.surface.canvas().local_to_device();
                if let Some(layer) = layer_recorder.recording_canvas() {
                    layer.set_matrix(&current_matrix);
                    f(layer, &layer_paint);
                }

                if let Some(pict) = layer_recorder.finish_recording_as_picture(Some(&self.surface_data.bounds)) {
                    let canvas = self.surface.canvas();
                    canvas.save();
                    let mut blend_paint = skia_safe::Paint::default();
                    blend_paint.set_anti_alias(true);
                    blend_paint.set_blend_mode(blend);
                    canvas.draw_picture(&pict, Some(&skia_safe::Matrix::new_identity()), Some(&blend_paint));
                    canvas.restore();
                    self.surface_state = self.surface_state | SurfaceState::Pending;
                }
            }
            _ => {
                self.with_canvas_dirty(|canvas| {
                    f(canvas, paint)
                });
            }
        };
    }

    pub fn render_text_to_canvas<F>(&mut self, paint: &skia_safe::Paint, f: F)
    where
        F: Fn(&skia_safe::Canvas, &skia_safe::Paint, &Font),
    {
        let blend = self.state.global_composite_operation.get_blend_mode();
        match blend {
            BlendMode::SrcIn | BlendMode::SrcOut |
            BlendMode::DstIn | BlendMode::DstOut |
            BlendMode::DstATop | BlendMode::Src => {
                let mut layer_paint = paint.clone();
                layer_paint.set_anti_alias(true);
                layer_paint.set_blend_mode(BlendMode::SrcOver);
                let mut layer_recorder = skia_safe::PictureRecorder::new();
                layer_recorder.begin_recording(self.surface_data.bounds, None);
                let current_matrix = skia_safe::M44::from(&self.state.matrix);
                if let Some(layer) = layer_recorder.recording_canvas() {
                    layer.set_matrix(&current_matrix);
                    f(layer, &layer_paint, &self.state.font_style);
                }

                if let Some(pict) = layer_recorder.finish_recording_as_picture(Some(&self.surface_data.bounds)) {
                    let canvas = self.surface.canvas();
                    canvas.save();
                    let mut blend_paint = skia_safe::Paint::default();
                    blend_paint.set_anti_alias(true);
                    blend_paint.set_blend_mode(blend);
                    canvas.draw_picture(&pict, Some(&skia_safe::Matrix::new_identity()), Some(&blend_paint));
                    canvas.restore();
                    self.surface_state = self.surface_state | SurfaceState::Pending;
                }
            }
            _ => {
                f(self.surface.canvas(), paint, &self.state.font_style);
                self.surface_state = self.surface_state | SurfaceState::Pending;
            }
        };
    }

    pub fn reset_state(&mut self) {
        let direction = self.state.direction;
        self.state = State::default();
        self.state_stack.clear();
        self.state.direction = direction;
    }

    pub fn clear_canvas(&mut self) {
        self.with_canvas_dirty(|canvas| {
            canvas.clear(Color::TRANSPARENT);
        });
    }


    pub fn draw_on_surface(&mut self, surface: &mut Surface) {
        let src_surface = &mut self.surface;
        src_surface.draw(
            surface.canvas(),
            Point::new(0., 0.),
            FilterQuality::High,
            None,
        )
    }
}
