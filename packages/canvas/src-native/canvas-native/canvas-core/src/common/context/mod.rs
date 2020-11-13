use std::os::raw::c_float;

use skia_safe::{
    BlendMode, BlurStyle, ClipOp, Color, EncodedImageFormat, Image, ImageInfo, MaskFilter, Matrix,
    PathOp, Point, Rect, Surface,
};
use skia_safe::canvas::SrcRectConstraint;
use skia_safe::paint::Style;

use crate::common::context::compositing::composite_operation_type::CompositeOperationType;
use crate::common::context::drawing_paths::fill_rule::FillRule;
use crate::common::context::drawing_text::typography::Font;
use crate::common::context::fill_and_stroke_styles::paint::Paint;
use crate::common::context::image_smoothing::ImageSmoothingQuality;
use crate::common::context::line_styles::line_cap::LineCap;
use crate::common::context::line_styles::line_join::LineJoin;
use crate::common::context::paths::path::Path;
use crate::common::context::text_styles::{
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

pub mod image_asset;
pub mod matrix;
pub mod text_decoder;
pub mod text_encoder;
pub mod transformations;

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
pub(crate) struct State {
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
    pub fn from_device(device: Device, direction: TextDirection) -> Self {
        let mut font = Font::new("10px sans-serif", device.density);
        Self {
            direction,
            paint: Paint::default(),
            font,
            text_align: TextAlign::default(),
            text_baseline: TextBaseLine::default(),
            shadow_color: Color::TRANSPARENT,
            shadow_offset: (0.0, 0.0).into(),
            shadow_blur: 0.0,
            image_smoothing_enabled: false,
            image_smoothing_quality: ImageSmoothingQuality::default(),
            line_width: 1.0 * device.density,
            line_cap: LineCap::default(),
            line_join: LineJoin::default(),
            miter_limit: 10.0 * device.density,
            line_dash_list: vec![],
            line_dash_offset: 0.0,
            filter: "none".to_string(),
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

impl Context {
    pub fn reset_state(&mut self) {
        let direction = self.state.direction;
        self.state = State::from_device(self.device, direction);
    }

    pub fn clear_canvas(&mut self) {
        self.surface.canvas().clear(Color::TRANSPARENT).flush();
    }

    pub fn flush(&mut self) {
        self.surface.canvas().flush();
    }
}
