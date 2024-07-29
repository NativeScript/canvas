use std::ffi::c_uint;
use std::sync::{Arc, Mutex};

use base64::Engine;
use parking_lot::MutexGuard;
use skia_safe::{Color, EncodedImageFormat, Image, PictureRecorder, Point, Surface};

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
use crate::context::matrix::Matrix;

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

#[cfg(target_os = "android")]
pub mod surface_vulkan;

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
    pub(crate) matrix: Matrix,
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
            matrix: Matrix(skia_safe::M44::new_identity()),
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

pub struct SurfaceData {
    pub(crate) bounds: skia_safe::Rect,
    pub(crate) scale: f32,
    pub(crate) ppi: f32,
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
}

pub struct Recorder {
    pub current: PictureRecorder,
    pub is_dirty: bool,
    pub layers: Vec<skia_safe::Picture>,
    pub cache: Option<Image>,
    pub matrix: skia_safe::Matrix,
    pub clip: Option<skia_safe::Path>,
    pub bounds: skia_safe::Rect,
}

impl Recorder {
    pub fn new(bounds: skia_safe::Rect) -> Self {
        let mut current = PictureRecorder::new();
        current.begin_recording(bounds, None);
        Self {
            current,
            bounds,
            is_dirty: false,
            layers: vec![],
            cache: None,
            matrix: skia_safe::Matrix::new_identity(),
            clip: None,
        }
    }

    pub fn set_bounds(&mut self, bounds: skia_safe::Rect) {
        *self = Recorder::new(bounds);
    }

    pub fn update_bounds(&mut self, bounds: skia_safe::Rect) {
        self.bounds = bounds;
    }

    pub fn append<F>(&mut self, f: F)
    where
        F: FnOnce(&skia_safe::Canvas),
    {
        if let Some(canvas) = self.current.recording_canvas() {
            f(canvas);
            self.is_dirty = true;
        }
    }

    pub fn set_matrix(&mut self, matrix: skia_safe::Matrix) {
        self.matrix = matrix;
        if let Some(canvas) = self.current.recording_canvas() {
            canvas.set_matrix(&skia_safe::M44::from(&matrix));
        }
    }

    pub fn flush(&mut self) {
        if self.is_dirty {
            if let Some(layer) = self.current.finish_recording_as_picture(Some(&self.bounds)) {
                self.layers.push(layer);
            }
            self.current.begin_recording(self.bounds, None);
            self.is_dirty = false;
            self.cache = None;
        }
    }

    pub fn get_image(&mut self) -> Option<Image> {
        self.flush();
        if self.cache.is_none() {
            let size = self.bounds.size().to_floor();
            if let Some(picture) = self.layers.last(){
                self.cache = skia_safe::images::deferred_from_picture(
                    picture,
                    size,
                    None,
                    None,
                    skia_safe::images::BitDepth::U8,
                    Some(skia_safe::ColorSpace::new_srgb()),
                    None,
                );
            }
        }
        self.cache.clone()
    }
    pub fn as_data(&mut self) -> Vec<u8> {
        if self.bounds.is_empty() {
            return vec![];
        }
        let read_data = |image: &Image, bounds: skia_safe::Rect| {
            let width = bounds.width();
            let height = bounds.height();
            let mut info = skia_safe::ImageInfo::new(
                skia_safe::ISize::new(width as i32, height as i32),
                skia_safe::ColorType::RGBA8888,
                skia_safe::AlphaType::Unpremul,
                None,
            );
            let row_bytes = info.width() * 4;
            let mut pixels = vec![255u8; (row_bytes * info.height()) as usize];
            let _ = image.read_pixels(
                &mut info,
                pixels.as_mut_slice(),
                row_bytes as usize,
                skia_safe::IPoint::new(0, 0),
                skia_safe::image::CachingHint::Allow,
            );
            pixels
        };

        if let Some(ref cache) = self.get_image() {
            read_data(cache, self.bounds)
        } else {
            vec![]
        }
    }

    pub fn as_data_url(&mut self, format: &str, quality: c_uint) -> String {
        if self.bounds.is_empty() {
            return "data:,".to_string();
        }

        self.flush();

        if let Some(image) = self.cache.as_ref() {
            let mut quality = quality;
            if quality > 100 || quality < 0 {
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
                None,
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

        "data:,".to_string()
    }
}

impl Default for Recorder {
    fn default() -> Self {
        Self {
            current: PictureRecorder::new(),
            is_dirty: false,
            layers: vec![],
            cache: None,
            matrix: skia_safe::Matrix::new_identity(),
            clip: None,
            bounds: skia_safe::Rect::default(),
        }
    }
}

#[cfg(any(not(target_os = "android")))]
pub struct Context {
    pub(crate) surface_data: SurfaceData,
    pub(crate) surface: Surface,
    #[cfg(any(feature = "gl", feature = "vulkan", feature = "metal"))]
    pub(crate) direct_context: Option<skia_safe::gpu::DirectContext>,
    pub(crate) recorder: Arc<parking_lot::Mutex<Recorder>>,
    pub(crate) path: Path,
    pub(crate) state: State,
    pub(crate) state_stack: Vec<State>,
    pub(crate) font_color: Color,
}

#[cfg(target_os = "android")]
pub struct Context {
    pub(crate) surface_data: SurfaceData,
    pub(crate) surface: Surface,
    #[cfg(feature = "vulkan")]
    pub(crate) ash_graphics: Option<surface_vulkan::AshGraphics>,
    #[cfg(feature = "vulkan")]
    pub(crate) vk_surface: Option<ash::vk::SurfaceKHR>,
    #[cfg(any(feature = "gl", feature = "vulkan", feature = "metal"))]
    pub(crate) direct_context: Option<skia_safe::gpu::DirectContext>,
    pub(crate) recorder: Arc<parking_lot::Mutex<Recorder>>,
    pub(crate) path: Path,
    pub(crate) state: State,
    pub(crate) state_stack: Vec<State>,
    pub(crate) font_color: Color,
}

pub type ContextWrapper = Arc<Mutex<Context>>;

impl Context {

    pub fn get_image(&self) -> Option<Image> {
        let recorder = Arc::clone(&self.recorder);
        let mut recorder = recorder.lock();
        recorder.get_image()
    }
    pub fn get_surface_data(&self) -> &SurfaceData {
        &self.surface_data
    }
    pub fn with_recorder<F>(&self, f: F)
    where
        F: FnOnce(MutexGuard<Recorder>),
    {
        let recorder = Arc::clone(&self.recorder);
        let recorder = recorder.lock();
        f(recorder);
    }
    pub fn with_matrix<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Matrix) -> &Matrix,
    {
        f(&mut self.state.matrix);
        self.with_recorder(|mut recorder| recorder.set_matrix(self.state.matrix.to_m33()));
    }

    pub fn width(&self) -> f32 {
        self.surface_data.bounds.width()
    }
    pub fn height(&self) -> f32 {
        self.surface_data.bounds.height()
    }

    pub fn with_canvas<F>(&self, f: F)
    where
        F: FnOnce(&skia_safe::Canvas),
    {
        let recorder = Arc::clone(&self.recorder);
        let mut recorder = recorder.lock();
        if let Some(canvas) = recorder.current.recording_canvas() {
            f(canvas)
        }
    }

    pub fn reset_state(&mut self) {
        let direction = self.state.direction;
        self.state = State::default();
        self.state_stack.clear();
        self.state.direction = direction;
    }

    pub fn clear_canvas(&mut self) {
        self.surface.canvas().clear(Color::TRANSPARENT);
        if let Some(mut context) = self.surface.direct_context() {
            context.flush_submit_and_sync_cpu();
        }
    }

    pub fn flush(&self) {
        let mut lock = self.recorder.lock();
        lock.flush();
    }
}
