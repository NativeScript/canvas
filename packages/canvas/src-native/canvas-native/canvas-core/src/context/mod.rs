use std::os::raw::c_float;
use std::sync::Arc;

use parking_lot::{Mutex, RawMutex, RawRwLock};
use parking_lot::lock_api::{MutexGuard, RwLockReadGuard, RwLockWriteGuard};
use skia_safe::{Color, Point, Surface};

use compositing::composite_operation_type::CompositeOperationType;
use drawing_text::typography::Font;
use fill_and_stroke_styles::paint::Paint;
use filter_quality::FilterQuality;
use image_smoothing::ImageSmoothingQuality;
use line_styles::line_cap::LineCap;
use line_styles::line_join::LineJoin;
use paths::path::Path;
use text_styles::{
    text_align::TextAlign, text_baseline::TextBaseLine, text_direction::TextDirection,
};

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
pub mod image_asset;
pub mod matrix;
pub mod text_decoder;
pub mod text_encoder;
pub mod transformations;
pub mod surface;
pub mod surface_gl;

#[derive(Copy, Clone, Debug)]
pub struct Device {
    pub width: c_float,
    pub height: c_float,
    pub density: c_float,
    pub non_gpu: bool,
    pub samples: usize,
    pub alpha: bool,
    pub ppi: c_float,
}

impl Device {
    pub fn new_non_gpu(width: c_float, height: c_float, density: c_float, ppi: c_float) -> Self {
        Self {
            width,
            height,
            density,
            non_gpu: true,
            samples: 0,
            alpha: false,
            ppi,
        }
    }
}

#[derive(Clone)]
pub struct State {
    pub(crate) direction: TextDirection,
    pub(crate) paint: Paint,
    pub(crate) font: Font,
    pub(crate) text_align: TextAlign,
    pub(crate) text_baseline: TextBaseLine,
    pub(crate) shadow_color: Color,
    pub(crate) shadow_offset: Point,
    pub(crate) shadow_blur: f32,
    pub(crate) image_smoothing_enabled: bool,
    pub(crate) image_smoothing_quality: ImageSmoothingQuality,
    pub(crate) line_width: c_float,
    pub(crate) line_cap: LineCap,
    pub(crate) line_join: LineJoin,
    pub(crate) miter_limit: f32,
    pub(crate) line_dash_list: Vec<f32>,
    pub(crate) line_dash_offset: f32,
    pub(crate) filter: String,
    pub(crate) global_alpha: f32,
    pub(crate) global_composite_operation: CompositeOperationType,
}

impl State {
    pub(crate) fn image_filter_quality(&self) -> FilterQuality {
        if self.image_smoothing_enabled {
            self.image_smoothing_quality.into()
        } else {
            FilterQuality::None
        }
    }
    pub fn from_device(device: Device, direction: TextDirection) -> Self {
        let font = Font::new("10px sans-serif", device);
        let mut paint = Paint::default();
        paint
            .stroke_paint_mut()
            .set_stroke_width(1.)
            .set_stroke_miter(10.);
        Self {
            direction,
            paint,
            font,
            text_align: TextAlign::default(),
            text_baseline: TextBaseLine::default(),
            shadow_color: Color::TRANSPARENT,
            shadow_offset: (0.0, 0.0).into(),
            shadow_blur: 0.0,
            image_smoothing_enabled: false,
            image_smoothing_quality: ImageSmoothingQuality::default(),
            line_width: 1.0,
            line_cap: LineCap::default(),
            line_join: LineJoin::default(),
            miter_limit: 10.0,
            line_dash_list: Default::default(),
            line_dash_offset: 0.0,
            filter: "none".into(),
            global_alpha: 1.0,
            global_composite_operation: CompositeOperationType::default(),
        }
    }
}

#[derive(Clone)]
pub struct Context {
    pub(crate) surface: Surface,
    pub(crate) path: Path,
    pub(crate) state: State,
    pub(crate) state_stack: Vec<State>,
    pub(crate) device: Device,
    pub(crate) font_color: Color,
}

impl Drop for Context {
    fn drop(&mut self) {
        log::debug!(target: "JS","Drop Context");
    }
}

pub struct ContextWrapper {
    inner: Arc<parking_lot::Mutex<Context>>,
}

unsafe impl Send for ContextWrapper {}

unsafe impl Sync for ContextWrapper {}

impl ContextWrapper {
    pub fn new(context: Context) -> ContextWrapper {
        Self {
            inner: Arc::new(parking_lot::Mutex::new(context)),
        }
    }

    pub fn get_context(&self) -> MutexGuard<'_, RawMutex, Context> {
        self.inner.lock()
    }

    pub fn into_box(self) -> Box<ContextWrapper> {
        Box::new(self)
    }

    pub fn into_raw(self) -> *mut ContextWrapper {
        Box::into_raw(self.into_box())
    }

    pub fn get_inner(&self) -> &Arc<Mutex<Context>> {
        log::debug!(target: "JS","get_inner");
        &self.inner
    }

    pub fn from_inner(inner: Arc<Mutex<Context>>) -> ContextWrapper {
        Self { inner }
    }
}

impl Drop for ContextWrapper{
    fn drop(&mut self) {
        log::debug!(target: "JS","Drop ContextWrapper");
    }
}

impl Clone for ContextWrapper {
    fn clone(&self) -> Self {
        log::debug!(target: "JS","Cloning ContextWrapper");
        Self {
            inner: Arc::clone(&self.inner),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.inner = Arc::clone(&source.inner)
    }
}

impl Context {
    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn reset_state(&mut self) {
        let direction = self.state.direction;
        self.state = State::from_device(self.device, direction);
    }

    pub fn clear_canvas(&mut self) {
        self.surface.canvas().clear(Color::TRANSPARENT);
        self.surface.flush();
    }

    pub fn flush(&mut self) {
        self.surface.flush();
    }

    pub fn read_pixels(&mut self) -> Vec<u8> {
        self.flush();
        let info = self.surface.image_info();
        let size = info.min_row_bytes() * info.height() as usize;
        let mut buf = vec![0_u8; size];

        self.surface.read_pixels(
            &info,
            buf.as_mut_slice(),
            info.min_row_bytes(),
            skia_safe::IPoint::new(0, 0),
        );
        buf
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
