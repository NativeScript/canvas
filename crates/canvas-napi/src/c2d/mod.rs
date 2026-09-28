pub mod gradient;
pub mod image_data;
pub mod path;
pub mod pattern;
pub mod text_metrics;

use crate::c2d::gradient::CanvasGradient;
use crate::c2d::image_data::ImageData;
use crate::c2d::path::Path2D;
use crate::c2d::pattern::CanvasPattern;
use crate::c2d::text_metrics::TextMetrics;
use crate::dom_matrix::DOMMatrix;
use crate::image_asset::ImageAsset;
use canvas_2d::context::compositing::composite_operation_type::CompositeOperationType;
use canvas_2d::context::image_smoothing::ImageSmoothingQuality;
use canvas_2d::context::line_styles::line_cap::LineCap;
use canvas_2d::context::line_styles::line_join::LineJoin;
use canvas_2d::context::text_styles::text_align::TextAlign;
use canvas_2d::context::text_styles::text_baseline::TextBaseLine;
use canvas_2d::context::text_styles::text_direction::TextDirection;
use canvas_2d::utils::color::to_parsed_color;
use canvas_c::enums::{CanvasFillRule, CanvasRepetition};
use canvas_c::{
    canvas_native_context_get_current_fill_style_type, canvas_native_context_get_fill_style,
    canvas_native_context_get_style_type, canvas_native_context_set_fill_style,
    canvas_native_paint_style_get_color_string,
    CanvasRenderingContext2D as CCanvasRenderingContext2D, PaintStyle, PaintStyleType,
};
use napi::bindgen_prelude::{Either, Either3, Either5, ObjectFinalize, Unknown};
use napi::*;
use napi_derive::napi;
use crate::frame::FrameSlot;
use crate::gl::web_g_l_rendering_context;
use crate::gl2::web_g_l_2_rendering_context;
use crate::image_bitmap::ImageBitmap;
use std::cell::Cell;
use std::ffi::{c_void, CString};
use std::rc::Rc;

#[napi(custom_finalize)]
pub struct CanvasRenderingContext2D {
  pub(crate) context: *mut CCanvasRenderingContext2D,
  /// Dirty tracking: drawing marks the context, the host renders it at frame end.
  pub(crate) frame: Rc<FrameSlot>,
  continuous_render: Cell<bool>,
}

impl ObjectFinalize for CanvasRenderingContext2D {
  fn finalize(self, _: Env) -> Result<()> {
    canvas_c::canvas_native_context_release(self.context);
    Ok(())
  }
}

unsafe fn render_2d(context: *mut c_void) {
  canvas_c::canvas_native_context_render(context as *mut CCanvasRenderingContext2D);
  #[cfg(target_os = "windows")]
  if canvas_c::canvas_native_context_is_lost(context as *const CCanvasRenderingContext2D) {
    crate::frame::report_lost();
  }
}

impl CanvasRenderingContext2D {
  pub(crate) fn from_raw(context: *mut CCanvasRenderingContext2D) -> Self {
    Self {
      context,
      frame: FrameSlot::new(context as *mut c_void, render_2d),
      continuous_render: Cell::new(false),
    }
  }

  /// Marks pixels as changed (every drawing call does).
  #[inline]
  fn dirty(&self) {
    crate::frame::mark_dirty(&self.frame);
  }

  /// Renders pending drawing now, before something reads this canvas's pixels.
  #[inline]
  pub(crate) fn flush_pending(&self) {
    self.frame.flush_now();
  }
}

/// `drawPoints` takes `{x, y}` objects.
#[napi(object)]
pub struct CanvasPoint {
  pub x: f64,
  pub y: f64,
}

fn fill_rule(rule: i32) -> Option<CanvasFillRule> {
  match rule {
    0 => Some(CanvasFillRule::NonZero),
    1 => Some(CanvasFillRule::EvenOdd),
    _ => None,
  }
}

fn repetition(value: Option<&str>) -> CanvasRepetition {
  match value {
    Some("repeat-x") => CanvasRepetition::RepeatX,
    Some("repeat-y") => CanvasRepetition::RepeatY,
    Some("no-repeat") => CanvasRepetition::NoRepeat,
    _ => CanvasRepetition::Repeat,
  }
}

#[napi]
impl CanvasRenderingContext2D {
  #[napi]
  pub fn flush(&self) {
    canvas_c::canvas_native_context_flush(self.context);
  }

  #[napi]
  pub fn render(&self) {
    canvas_c::canvas_native_context_render(self.context);
  }

  #[napi]
  pub fn resize(&self, width: u32, height: u32) {
    let context = unsafe { &mut *self.context };
    canvas_c::resize(context, width as f32, height as f32);
  }

  #[napi(factory)]
  pub fn with_cpu(
    width: f64,
    height: f64,
    density: f64,
    alpha: bool,
    font_color: i32,
    ppi: f64,
    direction: u32,
  ) -> Self {
    CanvasRenderingContext2D::from_raw(
      canvas_c::canvas_native_context_create(
        width as f32,
        height as f32,
        density as f32,
        alpha,
        font_color,
        ppi as f32,
        direction,
        canvas_c::CanvasColorSpace::Srgb,
      ),
    )
  }

  /// `quality` is 0..1, as in `toDataURL`; out-of-range values use the encoder default.
  #[napi(js_name = "__toDataURL")]
  pub fn to_data_url(&self, format: Option<String>, quality: Option<f64>) -> String {
    self.flush_pending();
    let format = CString::new(format.unwrap_or_else(|| "image/png".into())).unwrap_or_default();
    let quality = quality.map_or(92, |q| (q * 100.) as u32);
    let ret = canvas_c::canvas_native_to_data_url(self.context, format.as_ptr(), quality);
    unsafe { CString::from_raw(ret as _).to_string_lossy().to_string() }
  }

  /// The canvas-c context pointer, as a decimal string (the host APIs take it back).
  #[napi(js_name = "__getPointer")]
  pub fn get_pointer(&self) -> String {
    (self.context as usize).to_string()
  }

  #[napi(js_name = "__makeDirty")]
  pub fn make_dirty(&self) {
    self.dirty();
  }

  #[napi(js_name = "__startRaf")]
  pub fn start_raf(&self) {
    self.frame.set_paused(false);
  }

  #[napi(js_name = "__stopRaf")]
  pub fn stop_raf(&self) {
    self.frame.set_paused(true);
  }

  #[napi(js_name = "__resize")]
  pub fn resize_surface(&self, width: f64, height: f64) {
    canvas_c::canvas_native_context_resize(self.context, width as f32, height as f32);
    self.dirty();
  }

  /// A pattern made by the platform's own code (Android/iOS `NSCCanvasRenderingContext2D`).
  #[napi(js_name = "__createPatternWithNative")]
  pub fn create_pattern_with_native(&self, pattern: i64) -> Option<CanvasPattern> {
    (pattern != 0).then(|| CanvasPattern {
      style: pattern as *mut PaintStyle,
    })
  }

  #[napi(getter)]
  pub fn continuous_render_mode(&self) -> bool {
    self.continuous_render.get()
  }

  #[napi(setter)]
  pub fn set_continuous_render_mode(&self, value: bool) {
    self.continuous_render.set(value);
  }

  #[napi(getter)]
  pub fn direction(&self) -> &str {
    let context = unsafe { &*self.context };
    match context.get_context().direction() {
      TextDirection::LTR => "ltr",
      TextDirection::RTL => "rtl",
    }
  }

  #[napi(setter)]
  pub fn set_direction(&self, direction: JsString) {
    if let Some(direction) = direction.into_utf8().ok() {
      if let Ok(direction) = direction.as_str() {
        let context = unsafe { &mut *self.context };
        match direction {
          "ltr" => context
            .get_context_mut()
            .set_direction(canvas_2d::context::text_styles::text_direction::TextDirection::LTR),
          "rtl" => context
            .get_context_mut()
            .set_direction(canvas_2d::context::text_styles::text_direction::TextDirection::RTL),
          _ => {}
        }
      }
    }
  }

  #[napi(getter)]
  pub fn fill_style(&self) -> Either3<String, CanvasGradient, CanvasPattern> {
    let style = canvas_c::canvas_native_context_get_fill_style(self.context);
    let style_ref = unsafe { &*style };
    match style_ref.style_type() {
      PaintStyleType::Color | PaintStyleType::Color4f => {
        let color =
          unsafe { CString::from_raw(canvas_native_paint_style_get_color_string(style) as _) };
        // The style is a copy made for this call; colours have no wrapper to own it.
        canvas_c::canvas_native_paint_style_release(style);
        Either3::A(color.to_string_lossy().into_owned())
      }
      PaintStyleType::Gradient => Either3::B(CanvasGradient { style }),
      PaintStyleType::Pattern => Either3::C(CanvasPattern { style }),
    }
  }

  #[napi(setter, return_if_invalid)]
  pub fn set_fill_style(
    &self,
    style: Either3<JsString, &CanvasPattern, &CanvasGradient>,
  ) -> Result<()> {
    match style {
      Either3::A(color) => {
        let context = unsafe { &mut *self.context };
        context
          .get_context_mut()
          .set_fill_style_with_color(color.into_utf8()?.as_str()?);
        Ok(())
      }
      Either3::B(pattern) => {
        canvas_native_context_set_fill_style(self.context, pattern.style);
        Ok(())
      }
      Either3::C(gradient) => {
        canvas_native_context_set_fill_style(self.context, gradient.style);
        Ok(())
      }
    }
  }

  #[napi(getter)]
  pub fn filter(&self) -> &str {
    let context = unsafe { &*self.context };
    context.get_context().get_filter()
  }

  #[napi(setter)]
  pub fn set_filter(&self, value: String) {
    let context = unsafe { &mut *self.context };
    context.get_context_mut().set_filter(&value);
  }

  #[napi(getter)]
  pub fn font(&self) -> &str {
    let context = unsafe { &*self.context };
    context.get_context().font()
  }

  #[napi(setter)]
  pub fn set_font(&self, value: String) {
    let context = unsafe { &mut *self.context };
    context.get_context_mut().set_font(&value);
  }

  #[napi(getter)]
  pub fn font_kerning(&self) -> &str {
    // todo fontKerning
    "auto"
  }

  #[napi(setter)]
  pub fn set_font_kerning(&self, value: String) {}

  #[napi(getter)]
  pub fn font_stretch(&self) -> &str {
    // todo fontStretch
    "normal"
  }

  #[napi(setter)]
  pub fn set_font_stretch(&self, value: String) {}

  #[napi(getter)]
  pub fn font_variant_caps(&self) -> &str {
    // todo fontVariantCaps
    "normal"
  }

  #[napi(setter)]
  pub fn set_font_variant_caps(&self, value: String) {}

  #[napi(getter)]
  pub fn global_alpha(&self) -> f64 {
    let context = unsafe { &*self.context };
    context.get_context().global_alpha() as f64
  }

  #[napi(setter)]
  pub fn set_global_alpha(&self, alpha: f64) {
    canvas_c::canvas_native_context_set_global_alpha(self.context, alpha as f32)
  }

  #[napi(getter)]
  pub fn global_composite_operation(&self) -> u32 {
    canvas_c::canvas_native_context_get_global_composition_int(self.context)
  }

  #[napi(setter)]
  pub fn set_global_composite_operation(&self, operation: u32) {
    canvas_c::canvas_native_context_set_global_composition_int(self.context, operation);
  }

  #[napi(getter)]
  pub fn image_smoothing_enabled(&self) -> bool {
    let context = unsafe { &*self.context };
    canvas_c::canvas_native_context_get_image_smoothing_enabled(self.context)
  }

  #[napi(setter)]
  pub fn set_image_smoothing_enabled(&self, enabled: bool) {
    canvas_c::canvas_native_context_set_image_smoothing_enabled(self.context, enabled);
  }

  #[napi(getter)]
  pub fn image_smoothing_quality(&self) -> &str {
    let context = unsafe { &*self.context };
    match context.get_context().get_image_smoothing_quality() {
      ImageSmoothingQuality::Low => "low",
      ImageSmoothingQuality::Medium => "medium",
      ImageSmoothingQuality::High => "high",
    }
  }

  #[napi(setter)]
  pub fn set_image_smoothing_quality(&self, quality: Either<u32, JsString>) {
    let context = unsafe { &mut *self.context };
    match quality {
      Either::A(quality) => {}
      Either::B(quality) => {
        if let Some(quality) = quality.into_utf8().ok() {
          if let Ok(quality) = quality.as_str() {
            let quality = match quality {
              "low" => Some(ImageSmoothingQuality::Low),
              "medium" => Some(ImageSmoothingQuality::Medium),
              "high" => Some(ImageSmoothingQuality::High),
              _ => None,
            };
            if let Some(quality) = quality {
              context
                .get_context_mut()
                .set_image_smoothing_quality(quality)
            }
          }
        }
      }
    }
  }

  #[napi(getter)]
  pub fn letter_spacing(&self) -> &str {
    let context = unsafe { &*self.context };
    context.get_context().get_letter_spacing()
  }

  #[napi(setter)]
  pub fn set_letter_spacing(&self, spacing: JsString) {
    let context = unsafe { &mut *self.context };
    if let Some(spacing) = spacing.into_utf8().ok() {
      if let Ok(spacing) = spacing.as_str() {
        context.get_context_mut().set_letter_spacing(spacing);
      }
    }
  }

  #[napi(getter)]
  pub fn line_cap(&self) -> &str {
    let context = unsafe { &*self.context };
    match context.get_context().line_cap() {
      LineCap::CapButt => "butt",
      LineCap::CapRound => "round",
      LineCap::CapSquare => "square",
    }
  }

  #[napi(setter)]
  pub fn set_line_cap(&self, cap: JsString) {
    let context = unsafe { &mut *self.context };
    if let Some(cap) = cap.into_utf8().ok() {
      if let Ok(cap) = cap.as_str() {
        let cap = match cap {
          "round" => Some(LineCap::CapRound),
          "butt" => Some(LineCap::CapButt),
          "square" => Some(LineCap::CapSquare),
          _ => None,
        };

        if let Some(cap) = cap {
          context.get_context_mut().set_line_cap(cap);
        }
      }
    }
  }

  #[napi(getter)]
  pub fn line_dash_offset(&self) -> f64 {
    canvas_c::canvas_native_context_get_line_dash_offset(self.context) as f64
  }

  #[napi(setter)]
  pub fn set_line_dash_offset(&self, offset: f64) {
    canvas_c::canvas_native_context_set_line_dash_offset(self.context, offset as f32);
  }

  #[napi(getter)]
  pub fn line_join(&self) -> &str {
    let context = unsafe { &*self.context };
    match context.get_context().line_join() {
      LineJoin::JoinRound => "round",
      LineJoin::JoinBevel => "bevel",
      LineJoin::JoinMiter => "miter",
    }
  }

  #[napi(setter)]
  pub fn set_line_join(&self, join: JsString) {
    let context = unsafe { &mut *self.context };
    if let Some(join) = join.into_utf8().ok() {
      if let Ok(join) = join.as_str() {
        let join = match join {
          "round" => Some(LineJoin::JoinRound),
          "bevel" => Some(LineJoin::JoinBevel),
          "miter" => Some(LineJoin::JoinMiter),
          _ => None,
        };

        if let Some(join) = join {
          context.get_context_mut().set_line_join(join);
        }
      }
    }
  }

  #[napi(getter)]
  pub fn line_width(&self) -> f64 {
    let context = unsafe { &*self.context };
    context.get_context().line_width() as f64
  }

  #[napi(setter)]
  pub fn set_line_width(&self, width: f64) {
    let context = unsafe { &mut *self.context };
    context.get_context_mut().set_line_width(width as f32);
  }

  #[napi(getter)]
  pub fn miter_limit(&self) -> f64 {
    canvas_c::canvas_native_context_get_miter_limit(self.context) as f64
  }

  #[napi(setter)]
  pub fn set_miter_limit(&self, limit: f64) {
    canvas_c::canvas_native_context_set_miter_limit(self.context, limit as f32);
  }

  #[napi(getter)]
  pub fn shadow_blur(&self) -> f64 {
    canvas_c::canvas_native_context_get_shadow_blur(self.context) as f64
  }

  #[napi(setter)]
  pub fn set_shadow_blur(&self, blur: f64) {
    let context = unsafe { &mut *self.context };
    canvas_c::canvas_native_context_set_shadow_blur(self.context, blur as f32);
  }

  #[napi(getter)]
  pub fn shadow_color(&self) -> String {
    let context = unsafe { &*self.context };
    to_parsed_color(context.get_context().shadow_color())
  }

  #[napi(setter)]
  pub fn set_shadow_color(&self, color: JsString) {
    let context = unsafe { &mut *self.context };
    if let Some(color) = color.into_utf8().ok() {
      if let Ok(color) = color.as_str() {
        let context = unsafe { &mut *self.context };
        context.get_context_mut().set_shadow_color_str(color);
      }
    }
  }

  #[napi(getter)]
  pub fn shadow_offset_x(&self) -> f64 {
    let context = unsafe { &*self.context };
    context.get_context().shadow_offset_x() as f64
  }

  #[napi(setter)]
  pub fn set_shadow_offset_x(&self, x: f64) {
    let context = unsafe { &mut *self.context };
    context.get_context_mut().set_shadow_offset_x(x as f32);
  }

  #[napi(getter)]
  pub fn shadow_offset_y(&self) -> f64 {
    let context = unsafe { &*self.context };
    context.get_context().shadow_offset_y() as f64
  }

  #[napi(setter)]
  pub fn set_shadow_offset_y(&self, y: f64) {
    let context = unsafe { &mut *self.context };
    context.get_context_mut().set_shadow_offset_y(y as f32);
  }

  #[napi(getter)]
  pub fn stroke_style(&self) -> Either3<String, CanvasGradient, CanvasPattern> {
    let style = canvas_c::canvas_native_context_get_stroke_style(self.context);
    let style_ref = unsafe { &*style };
    match style_ref.style_type() {
      PaintStyleType::Color | PaintStyleType::Color4f => {
        let color =
          unsafe { CString::from_raw(canvas_native_paint_style_get_color_string(style) as _) };
        // The style is a copy made for this call; colours have no wrapper to own it.
        canvas_c::canvas_native_paint_style_release(style);
        Either3::A(color.to_string_lossy().into_owned())
      }
      PaintStyleType::Gradient => Either3::B(CanvasGradient { style }),
      PaintStyleType::Pattern => Either3::C(CanvasPattern { style }),
    }
  }

  #[napi(setter)]
  pub fn set_stroke_style(
    &self,
    style: Either3<JsString, &CanvasPattern, &CanvasGradient>,
  ) {
    match style {
      Either3::A(color) => {
        if let Some(color) = color.into_utf8().ok() {
          if let Ok(color) = color.as_str() {
            let context = unsafe { &mut *self.context };
            context.get_context_mut().set_stroke_style_with_color(color);
          }
        }
      }
      Either3::B(pattern) => {
        canvas_c::canvas_native_context_set_stroke_style(self.context, pattern.style)
      }
      Either3::C(gradient) => {
        canvas_c::canvas_native_context_set_stroke_style(self.context, gradient.style)
      }
    }
  }

  #[napi(getter)]
  pub fn text_align(&self) -> &str {
    let context = unsafe { &*self.context };
    match context.get_context().text_align() {
      TextAlign::START => "start",
      TextAlign::LEFT => "left",
      TextAlign::CENTER => "center",
      TextAlign::RIGHT => "right",
      TextAlign::END => "end",
    }
  }

  #[napi(setter)]
  pub fn set_text_align(&self, align: JsString) {
    if let Some(align) = align.into_utf8().ok() {
      if let Ok(align) = align.as_str() {
        let context = unsafe { &mut *self.context };
        let align = match align {
          "start" => Some(TextAlign::START),
          "left" => Some(TextAlign::LEFT),
          "center" => Some(TextAlign::CENTER),
          "right" => Some(TextAlign::RIGHT),
          "end" => Some(TextAlign::END),
          _ => None,
        };

        if let Some(align) = align {
          context.get_context_mut().set_text_align(align);
        }
      }
    }
  }

  #[napi(getter)]
  pub fn text_baseline(&self) -> u32 {
    canvas_c::canvas_native_context_get_text_baseline(self.context) as u32
  }

  #[napi(setter)]
  pub fn set_text_baseline(&self, value: u32) {
    use canvas_c::TextBaseLine as Baseline;
    let baseline = match value {
      0 => Baseline::TOP,
      1 => Baseline::HANGING,
      2 => Baseline::MIDDLE,
      3 => Baseline::ALPHABETIC,
      4 => Baseline::IDEOGRAPHIC,
      5 => Baseline::BOTTOM,
      _ => return,
    };
    canvas_c::canvas_native_context_set_text_baseline(self.context, baseline);
  }

  #[napi(getter)]
  pub fn text_rendering(&self) -> &str {
    // todo textRendering
    "auto"
  }

  #[napi(setter)]
  pub fn set_text_rendering(&self, value: String) {}

  #[napi(getter)]
  pub fn word_spacing(&self) -> &str {
    let context = unsafe { &*self.context };
    context.get_context().get_word_spacing()
  }

  #[napi(setter)]
  pub fn set_word_spacing(&self, value: JsString) {
    if let Some(value) = value.into_utf8().ok() {
      if let Ok(value) = value.as_str() {
        let context = unsafe { &mut *self.context };
        context.get_context_mut().set_word_spacing(value);
      }
    }
  }

  #[napi]
  pub fn arc(
    &self,
    x: f64,
    y: f64,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
    anticlockwise: Option<bool>,
  ) {
    canvas_c::canvas_native_context_arc(
      self.context,
      x as f32,
      y as f32,
      radius as f32,
      start_angle as f32,
      end_angle as f32,
      anticlockwise.unwrap_or(false),
    )
  }

  #[napi]
  pub fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) {
    canvas_c::canvas_native_context_arc_to(
      self.context,
      x1 as f32,
      y1 as f32,
      x2 as f32,
      y2 as f32,
      radius as f32,
    )
  }

  #[napi]
  pub fn begin_path(&self) {
    canvas_c::canvas_native_context_begin_path(self.context);
  }

  #[napi]
  pub fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
    canvas_c::canvas_native_context_bezier_curve_to(
      self.context,
      cp1x as f32,
      cp1y as f32,
      cp2x as f32,
      cp2y as f32,
      x as f32,
      y as f32,
    )
  }

  #[napi]
  pub fn clear_rect(&self, x: f64, y: f64, width: f64, height: f64) {
    canvas_c::canvas_native_context_clear_rect(
      self.context,
      x as f32,
      y as f32,
      width as f32,
      height as f32,
    );
    self.dirty();
  }

  /// `clip()`, `clip(fillRule)`, `clip(path)`, `clip(path, fillRule)`; fill rules are 0/1.
  #[napi]
  pub fn clip(&self, path_or_rule: Option<Either<&Path2D, i32>>, rule: Option<i32>) {
    match path_or_rule {
      None => canvas_c::canvas_native_context_clip_rule(self.context, CanvasFillRule::NonZero),
      Some(Either::B(rule)) => {
        if let Some(rule) = fill_rule(rule) {
          canvas_c::canvas_native_context_clip_rule(self.context, rule)
        }
      }
      Some(Either::A(path)) => {
        if let Some(rule) = fill_rule(rule.unwrap_or(0)) {
          canvas_c::canvas_native_context_clip(self.context, path.path, rule)
        }
      }
    }
  }

  #[napi]
  pub fn close_path(&self) {
    canvas_c::canvas_native_context_close_path(self.context)
  }

  #[napi]
  pub fn create_conic_gradient(
    &self,
    start_angle: f64,
    x: f64,
    y: f64,
  ) -> CanvasGradient {
    let gradient = canvas_c::canvas_native_context_create_conic_gradient(
      self.context,
      start_angle as f32,
      x as f32,
      y as f32,
    );
    CanvasGradient { style: gradient }
  }

  #[napi]
  pub fn create_image_data(
    &self,
    width_or_image_data: Either<f64, &ImageData>,
    height: Option<f64>,
  ) -> Result<ImageData> {
    let (width, height) = match width_or_image_data {
      Either::A(width) => match height {
        Some(height) => (width as i32, height as i32),
        None => return Err(napi::Error::from_reason("Argument 1 is not an object.")),
      },
      Either::B(value) => (value.width_inner(), value.height_inner()),
    };
    ImageData::from_raw(canvas_c::canvas_native_context_create_image_data(width, height))
      .ok_or_else(|| napi::Error::from_reason("Failed to create ImageData"))
  }


  #[napi]
  pub fn create_linear_gradient(
    &self,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
  ) -> CanvasGradient {
    let gradient = canvas_c::canvas_native_context_create_linear_gradient(
      self.context,
      x0 as f32,
      y0 as f32,
      x1 as f32,
      y1 as f32,
    );
    CanvasGradient { style: gradient }
  }

  /// Sources are the native objects `packages/canvas` passes: an ImageAsset, an ImageBitmap or
  /// another canvas's 2D context.
  #[napi]
  pub fn create_pattern(
    &self,
    image: Either3<&ImageAsset, &ImageBitmap, &CanvasRenderingContext2D>,
    repetition_value: Option<String>,
  ) -> Option<CanvasPattern> {
    let repetition = repetition(repetition_value.as_deref());
    let style = match image {
      Either3::A(asset) => canvas_c::canvas_native_context_create_pattern_asset(
        self.context,
        asset.asset.as_ref(),
        repetition,
      ),
      Either3::B(bitmap) => canvas_c::canvas_native_context_create_pattern_asset(
        self.context,
        bitmap.asset.as_ref(),
        repetition,
      ),
      Either3::C(source) => {
        source.flush_pending();
        canvas_c::canvas_native_context_create_pattern_canvas2d(source.context, self.context, repetition)
      }
    };
    (!style.is_null()).then(|| CanvasPattern { style })
  }

  #[napi]
  pub fn create_radial_gradient(
    &self,
    x0: f64,
    y0: f64,
    r0: f64,
    x1: f64,
    y1: f64,
    r1: f64,
  ) -> CanvasGradient {
    let gradient = canvas_c::canvas_native_context_create_radial_gradient(
      self.context,
      x0 as f32,
      y0 as f32,
      r0 as f32,
      x1 as f32,
      y1 as f32,
      r1 as f32,
    );
    CanvasGradient { style: gradient }
  }

  #[napi]
  pub fn draw_focus_if_needed(
    &self,
    element_path: Either<&Path2D, Unknown>,
    element: Option<Unknown>,
  ) {
  }

  /// `drawImage(image, dx, dy)`, `(image, dx, dy, dw, dh)` and the 9-argument form. `image` is the
  /// native object `packages/canvas` passes: ImageAsset, ImageBitmap, or another canvas's 2D or
  /// WebGL context.
  #[napi]
  pub fn draw_image(
    &self,
    image: Either5<
      &ImageAsset,
      &ImageBitmap,
      &CanvasRenderingContext2D,
      &web_g_l_rendering_context,
      &web_g_l_2_rendering_context,
    >,
    a: f64,
    b: f64,
    c: Option<f64>,
    d: Option<f64>,
    e: Option<f64>,
    f: Option<f64>,
    g: Option<f64>,
    h: Option<f64>,
  ) {
    enum Source {
      Asset(*const canvas_c::ImageAsset),
      Context(*mut CCanvasRenderingContext2D),
      WebGL(*mut canvas_c::WebGLState),
    }
    let source = match image {
      Either5::A(asset) => Source::Asset(asset.asset.as_ref()),
      Either5::B(bitmap) => Source::Asset(bitmap.asset.as_ref()),
      Either5::C(context) => {
        context.flush_pending();
        Source::Context(context.context)
      }
      Either5::D(gl) => Source::WebGL(gl.state),
      Either5::E(gl) => Source::WebGL(gl.state),
    };
    let ctx = self.context;
    let (a, b) = (a as f32, b as f32);
    match (c, d, e, f, g, h) {
      (None, None, ..) => match source {
        Source::Asset(asset) => {
          canvas_c::canvas_native_context_draw_image_dx_dy_asset(ctx, asset as _, a, b)
        }
        Source::Context(src) => {
          canvas_c::canvas_native_context_draw_image_dx_dy_context(ctx, src, a, b)
        }
        Source::WebGL(gl) => canvas_c::canvas_native_context_draw_image_dx_dy_webgl(ctx, gl, a, b),
      },
      (Some(w), Some(h), None, ..) => {
        let (w, h) = (w as f32, h as f32);
        match source {
          Source::Asset(asset) => {
            canvas_c::canvas_native_context_draw_image_dx_dy_dw_dh_asset(ctx, asset as _, a, b, w, h)
          }
          Source::Context(src) => {
            canvas_c::canvas_native_context_draw_image_dx_dy_dw_dh_context(ctx, src, a, b, w, h)
          }
          Source::WebGL(gl) => {
            canvas_c::canvas_native_context_draw_image_dx_dy_dw_dh_webgl(ctx, gl, a, b, w, h)
          }
        }
      }
      (Some(sw), Some(sh), Some(dx), Some(dy), Some(dw), Some(dh)) => {
        let (sw, sh) = (sw as f32, sh as f32);
        let (dx, dy, dw, dh) = (dx as f32, dy as f32, dw as f32, dh as f32);
        match source {
          Source::Asset(asset) => canvas_c::canvas_native_context_draw_image_asset(
            ctx, asset as _, a, b, sw, sh, dx, dy, dw, dh,
          ),
          Source::Context(src) => canvas_c::canvas_native_context_draw_image_context(
            ctx, src, a, b, sw, sh, dx, dy, dw, dh,
          ),
          Source::WebGL(gl) => canvas_c::canvas_native_context_draw_image_webgl(
            ctx, gl, a, b, sw, sh, dx, dy, dw, dh,
          ),
        }
      }
      _ => return,
    }
    self.dirty();
  }

  #[napi]
  pub fn draw_paint(&self, color: String) {
    if let Ok(color) = CString::new(color) {
      canvas_c::canvas_native_context_draw_paint(self.context, color.as_ptr());
      self.dirty();
    }
  }

  #[napi]
  pub fn draw_point(&self, x: f64, y: f64) {
    canvas_c::canvas_native_context_draw_point(self.context, x as f32, y as f32);
    self.dirty();
  }

  /// `mode`: 0 points, 1 lines, 2 polygon.
  #[napi]
  pub fn draw_points(&self, mode: i32, points: Vec<CanvasPoint>) {
    if points.is_empty() || !(0..=2).contains(&mode) {
      return;
    }
    let flat: Vec<f32> = points.iter().flat_map(|p| [p.x as f32, p.y as f32]).collect();
    canvas_c::canvas_native_context_draw_points(self.context, mode, flat.as_ptr(), flat.len());
    self.dirty();
  }

  /// `xform` holds `[scos, ssin, tx, ty]` per sprite and `tex` `[x, y, w, h]`; `colors` are CSS
  /// colours.
  #[napi]
  pub fn draw_atlas(
    &self,
    image: &ImageAsset,
    xform: Vec<f64>,
    tex: Vec<f64>,
    colors: Option<Vec<String>>,
    blend_mode: u32,
  ) {
    let xform: Vec<f32> = xform.into_iter().map(|v| v as f32).collect();
    let tex: Vec<f32> = tex.into_iter().map(|v| v as f32).collect();
    let colors: Vec<CString> = colors
      .unwrap_or_default()
      .into_iter()
      .filter_map(|c| CString::new(c).ok())
      .collect();
    let color_ptrs: Vec<*const std::ffi::c_char> = colors.iter().map(|c| c.as_ptr()).collect();
    canvas_c::canvas_native_context_draw_atlas_asset(
      self.context,
      image.asset.as_ref(),
      xform.as_ptr(),
      xform.len(),
      tex.as_ptr(),
      tex.len(),
      if color_ptrs.is_empty() {
        std::ptr::null()
      } else {
        color_ptrs.as_ptr()
      },
      color_ptrs.len(),
      blend_mode,
    );
    self.dirty();
  }

  #[napi]
  pub fn fill_oval(&self, x: f64, y: f64, width: f64, height: f64) {
    canvas_c::canvas_native_context_fill_oval(
      self.context,
      x as f32,
      y as f32,
      width as f32,
      height as f32,
    );
    self.dirty();
  }

  // Hit regions were dropped from the spec; the V8 bindings keep them as no-ops.
  #[napi]
  pub fn add_hit_region(&self) {}

  #[napi]
  pub fn remove_hit_region(&self) {}

  #[napi]
  pub fn clear_hit_regions(&self) {}

  #[napi]
  pub fn scroll_path_into_view(&self) {}

  #[napi]
  pub fn ellipse(
    &self,
    x: f64,
    y: f64,
    radius_x: f64,
    radius_y: f64,
    rotation: f64,
    start_angle: f64,
    end_angle: f64,
    anticlockwise: Option<bool>,
  ) {
    canvas_c::canvas_native_context_ellipse(
      self.context,
      x as f32,
      y as f32,
      radius_x as f32,
      radius_y as f32,
      rotation as f32,
      start_angle as f32,
      end_angle as f32,
      anticlockwise.unwrap_or(false),
    )
  }

  /// `fill()`, `fill(fillRule)`, `fill(path)`, `fill(path, fillRule)`; fill rules are 0/1.
  #[napi]
  pub fn fill(&self, path_or_rule: Option<Either<&Path2D, i32>>, rule: Option<i32>) {
    match path_or_rule {
      None => canvas_c::canvas_native_context_fill(self.context, CanvasFillRule::NonZero),
      Some(Either::B(rule)) => match fill_rule(rule) {
        Some(rule) => canvas_c::canvas_native_context_fill(self.context, rule),
        None => return,
      },
      Some(Either::A(path)) => match fill_rule(rule.unwrap_or(0)) {
        Some(rule) => canvas_c::canvas_native_context_fill_with_path(self.context, path.path, rule),
        None => return,
      },
    }
    self.dirty();
  }

  #[napi]
  pub fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64) {
    canvas_c::canvas_native_context_fill_rect(
      self.context,
      x as f32,
      y as f32,
      width as f32,
      height as f32,
    );
    self.dirty();
  }

  #[napi]
  pub fn fill_text(&self, text: JsString, x: f64, y: f64, max_width: Option<f64>) {
    if let Some(text) = text.into_utf8().ok() {
      if let Ok(text) = text.as_str() {
        let context = unsafe { &mut *self.context };
        context.get_context_mut().fill_text(
          text,
          x as f32,
          y as f32,
          max_width.map(|width| width as f32),
        )
      }
    }
    self.dirty();
  }

  #[napi]
  pub fn get_image_data(&self, x: f64, y: f64, width: f64, height: f64) -> Result<ImageData> {
    ImageData::from_raw(canvas_c::canvas_native_context_get_image_data(
      self.context,
      x as f32,
      y as f32,
      width as f32,
      height as f32,
    ))
    .ok_or_else(|| napi::Error::from_reason("Failed to get ImageData"))
  }


  #[napi]
  pub fn get_line_dash(&self) -> Vec<f64> {
    let context = unsafe { &*self.context };
    context
      .get_context()
      .line_dash()
      .to_vec()
      .into_iter()
      .map(|v| v as f64)
      .collect::<Vec<f64>>()
  }

  #[napi]
  pub fn get_transform(&self) -> DOMMatrix {
    DOMMatrix {
      matrix: canvas_c::canvas_native_context_get_transform(self.context),
    }
  }

  #[napi]
  pub fn is_context_lost(&self) -> bool {
    // todo
    false
  }

  /// `(x, y[, fillRule])` or `(path, x, y[, fillRule])`.
  #[napi]
  pub fn is_point_in_path(
    &self,
    path_or_x: Either<&Path2D, f64>,
    a: f64,
    b: Option<f64>,
    rule: Option<i32>,
  ) -> bool {
    match path_or_x {
      Either::B(x) => {
        let rule = b.map_or(Some(CanvasFillRule::NonZero), |r| fill_rule(r as i32));
        rule.is_some_and(|rule| {
          canvas_c::canvas_native_context_is_point_in_path(self.context, x as f32, a as f32, rule)
        })
      }
      Either::A(path) => {
        let (Some(y), Some(rule)) = (b, fill_rule(rule.unwrap_or(0))) else {
          return false;
        };
        canvas_c::canvas_native_context_is_point_in_path_with_path(
          self.context,
          path.path,
          a as f32,
          y as f32,
          rule,
        )
      }
    }
  }

  /// `(x, y)` or `(path, x, y)`.
  #[napi]
  pub fn is_point_in_stroke(&self, path_or_x: Either<&Path2D, f64>, a: f64, b: Option<f64>) -> bool {
    match path_or_x {
      Either::B(x) => {
        canvas_c::canvas_native_context_is_point_in_stroke(self.context, x as f32, a as f32)
      }
      Either::A(path) => b.is_some_and(|y| {
        canvas_c::canvas_native_context_is_point_in_stroke_with_path(
          self.context,
          path.path,
          a as f32,
          y as f32,
        )
      }),
    }
  }

  #[napi]
  pub fn line_to(&self, x: f64, y: f64) {
    canvas_c::canvas_native_context_line_to(self.context, x as f32, y as f32)
  }

  #[napi]
  pub fn measure_text(&self, text: String) -> TextMetrics {
    let text = CString::new(text).unwrap_or_default();
    TextMetrics {
      metrics: canvas_c::canvas_native_context_measure_text(self.context, text.as_ptr()),
    }
  }

  #[napi]
  pub fn move_to(&self, x: f64, y: f64) {
    canvas_c::canvas_native_context_move_to(self.context, x as f32, y as f32)
  }

  #[napi]
  pub fn put_image_data(
    &self,
    image_data: &ImageData,
    dx: f64,
    dy: f64,
    dirty_x: Option<f64>,
    dirty_y: Option<f64>,
    dirty_width: Option<f64>,
    dirty_height: Option<f64>,
  ) {
    match (dirty_x, dirty_y, dirty_width, dirty_height) {
      (Some(x), Some(y), Some(width), Some(height)) => {
        canvas_c::canvas_native_context_put_image_data(
          self.context,
          image_data.as_ptr(),
          dx as f32,
          dy as f32,
          x as f32,
          y as f32,
          width as f32,
          height as f32,
        )
      }
      _ => canvas_c::canvas_native_context_put_image_data(
        self.context,
        image_data.as_ptr(),
        dx as f32,
        dy as f32,
        0.,
        0.,
        image_data.width_inner() as f32,
        image_data.height_inner() as f32,
      ),
    }
    self.dirty();
  }

  #[napi]
  pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) {
    canvas_c::canvas_native_context_quadratic_curve_to(
      self.context,
      cpx as f32,
      cpy as f32,
      x as f32,
      y as f32,
    )
  }

  #[napi]
  pub fn rect(&self, x: f64, y: f64, width: f64, height: f64) {
    canvas_c::canvas_native_context_rect(
      self.context,
      x as f32,
      y as f32,
      width as f32,
      height as f32,
    )
  }

  #[napi]
  pub fn reset(&self) {
    canvas_c::canvas_native_context_reset(self.context);
    self.dirty();
  }

  #[napi]
  pub fn reset_transform(&self) {
    canvas_c::canvas_native_context_reset_transform(self.context);
  }

  #[napi]
  pub fn restore(&self) {
    canvas_c::canvas_native_context_restore(self.context);
  }

  #[napi]
  pub fn rotate(&self, angle: f64) {
    canvas_c::canvas_native_context_rotate(self.context, angle as f32);
  }

  #[napi]
  pub fn round_rect(&self, x: f64, y: f64, width: f64, height: f64, radii: Either<f64, Vec<f64>>) {
    match radii {
      Either::A(radii) => {
        let radii = radii as f32;
        canvas_c::canvas_native_context_round_rect_tl_tr_br_bl(
          self.context,
          x as f32,
          y as f32,
          width as f32,
          height as f32,
          radii,
          radii,
          radii,
          radii,
        )
      }
      Either::B(radii) => {
        let radii = radii.into_iter().map(|v| v as f32).collect::<Vec<f32>>();

        canvas_c::canvas_native_context_round_rect(
          self.context,
          x as f32,
          y as f32,
          width as f32,
          height as f32,
          radii.as_ptr(),
          radii.len(),
        )
      }
    }
  }

  #[napi]
  pub fn save(&self) {
    canvas_c::canvas_native_context_save(self.context);
  }

  #[napi]
  pub fn scale(&self, x: f64, y: f64) {
    canvas_c::canvas_native_context_scale(self.context, x as f32, y as f32)
  }

  #[napi]
  pub fn set_line_dash(&self, segments: Vec<f64>) {
    let segments = segments.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
    canvas_c::canvas_native_context_set_line_dash(
      self.context,
      segments.as_ptr(),
      segments.len() as _,
    );
  }

  #[napi]
  pub fn stroke(&self, path: Option<&Path2D>) {
    match path {
      None => canvas_c::canvas_native_context_stroke(self.context),
      Some(path) => canvas_c::canvas_native_context_stroke_with_path(self.context, path.path),
    }
    self.dirty();
  }

  #[napi]
  pub fn stroke_rect(&self, x: f64, y: f64, width: f64, height: f64) {
    canvas_c::canvas_native_context_stroke_rect(
      self.context,
      x as f32,
      y as f32,
      width as f32,
      height as f32,
    );
    self.dirty();
  }

  #[napi]
  pub fn stroke_text(&self, text: JsString, x: f64, y: f64, max_width: Option<f64>) {
    if let Some(text) = text.into_utf8().ok() {
      if let Ok(text) = text.as_str() {
        let context = unsafe { &mut *self.context };
        context.get_context_mut().stroke_text(
          text,
          x as f32,
          y as f32,
          max_width.map(|width| width as f32),
        )
      }
    }
    self.dirty();
  }

  #[napi]
  pub fn stroke_oval(&self, x: f64, y: f64, width: f64, height: f64) {
    canvas_c::canvas_native_context_stroke_oval(
      self.context,
      x as f32,
      y as f32,
      width as f32,
      height as f32,
    );
    self.dirty();
  }
  #[napi]
  pub fn set_transform(
    &self,
    a: Either<f64, &DOMMatrix>,
    b: Option<f64>,
    c: Option<f64>,
    d: Option<f64>,
    e: Option<f64>,
    f: Option<f64>,
  ) {
    match a {
      Either::A(a) => match (b, c, d, e, f) {
        (Some(b), Some(c), Some(d), Some(e), Some(f)) => {
          canvas_c::canvas_native_context_set_transform(
            self.context,
            a as f32,
            b as f32,
            c as f32,
            d as f32,
            e as f32,
            f as f32,
          );
        }
        _ => {}
      },
      Either::B(b) => {
        canvas_c::canvas_native_context_set_transform_matrix(self.context, b.matrix);
      }
    }
  }

  #[napi]
  pub fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
    canvas_c::canvas_native_context_transform(
      self.context,
      a as f32,
      b as f32,
      c as f32,
      d as f32,
      e as f32,
      f as f32,
    );
  }

  #[napi]
  pub fn translate(&self, x: f64, y: f64) {
    canvas_c::canvas_native_context_translate(self.context, x as f32, y as f32);
  }
}

/// `CanvasModule.create2DContext(pointer)`: wraps (and takes ownership of) a canvas-c context.
#[napi(js_name = "create2DContext")]
pub fn create_2d_context(pointer: napi::bindgen_prelude::BigInt) -> Option<CanvasRenderingContext2D> {
  let (pointer, _) = pointer.get_i64();
  (pointer != 0).then(|| CanvasRenderingContext2D::from_raw(pointer as _))
}

/// `CanvasModule.create2DContextWithPointer(pointer)`: wraps the host view's context.
#[napi(js_name = "create2DContextWithPointer")]
pub fn create_2d_context_with_pointer(
  pointer: napi::bindgen_prelude::BigInt,
) -> Option<CanvasRenderingContext2D> {
  let (pointer, _) = pointer.get_i64();
  let context = canvas_c::canvas_native_context_create_with_pointer(pointer);
  if context.is_null() {
    return None;
  }
  canvas_c::canvas_native_context_reference(context);
  Some(CanvasRenderingContext2D::from_raw(context))
}
