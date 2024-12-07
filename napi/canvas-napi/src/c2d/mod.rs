pub mod path;
mod pattern;
mod gradient;
mod image_data;
mod text_metrics;

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
use canvas_c::{canvas_native_context_get_current_fill_style_type, canvas_native_context_get_fill_style, canvas_native_context_get_style_type, canvas_native_context_set_fill_style, canvas_native_paint_style_get_color_string, CanvasRenderingContext2D as CCanvasRenderingContext2D, PaintStyle, PaintStyleType};
use napi::bindgen_prelude::{ClassInstance, Either, Either3, FromNapiValue, ObjectFinalize};
use napi::*;
use napi_derive::napi;
use std::ffi::CString;
use std::sync::Arc;

#[napi(custom_finalize)]
pub struct CanvasRenderingContext2D {
    pub(crate) context: *mut CCanvasRenderingContext2D,
}

impl ObjectFinalize for CanvasRenderingContext2D {
    fn finalize(self, _: Env) -> Result<()> {
        canvas_c::canvas_native_context_release(self.context);
        Ok(())
    }
}


#[napi]
impl CanvasRenderingContext2D {
    #[napi(factory)]
    pub fn with_view(
        view: i64,
        width: f64,
        height: f64,
        density: f64,
        alpha: bool,
        font_color: i32,
        ppi: f64,
        direction: u32,
    ) -> Self {
        CanvasRenderingContext2D {
            context: canvas_c::canvas_native_context_create_gl(
                view as _,
                width as f32,
                height as f32,
                density as f32,
                alpha,
                font_color,
                ppi as f32,
                direction,
            ),
        }
    }


    #[napi(factory)]
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub fn with_mtl_view_device_queue(
        view: i64,
        device: i64,
        queue: i64,
        alpha: bool,
        density: f64,
        samples: u32,
        font_color: i32,
        ppi: f64,
        direction: i32,
    ) -> Self {
        let ctx_2d = canvas_c::CanvasRenderingContext2D::new_metal(
            canvas_2d::context::Context::new_metal_device_queue(
                view as _,
                device as _,
                queue as _,
                density as f32,
                samples as _,
                alpha,
                font_color,
                ppi as f32,
                canvas_2d::context::text_styles::text_direction::TextDirection::from(direction as u32),
            ),
            alpha,
        );

        CanvasRenderingContext2D {
            context: Box::into_raw(
                Box::new(ctx_2d) as _,
            ),
        }
    }

    #[napi]
    pub fn present(&self) {
        let context = unsafe { &mut *self.context };
        canvas_2d::context::Context::present(context.get_context_mut());
    }

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
        CanvasRenderingContext2D {
            context: canvas_c::canvas_native_context_create(
                width as f32,
                height as f32,
                density as f32,
                alpha,
                font_color,
                ppi as f32,
                direction,
            ),
        }
    }

    #[napi(js_name = "toDataURL")]
    pub fn to_data_url(&self, format: String, encoderOptions: Option<f64>) -> String {
        let c_str = CString::new(format).unwrap();
        let quality = encoderOptions
            .map(|v| v as f32)
            .unwrap_or(0.92)
            .try_into()
            .unwrap_or(0.92);
        let quality: u32 = (quality * 100.) as u32;
        let ret = canvas_c::canvas_native_to_data_url(self.context, c_str.as_ptr(), quality);
        unsafe { CString::from_raw(ret as _).to_string_lossy().to_string() }
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
    pub fn fill_style(&self, env: Env) -> Result<JsUnknown> {
        let style = canvas_c::canvas_native_context_get_fill_style(self.context);
        let style_ref = unsafe { &*style };
        match style_ref.style_type() {
            PaintStyleType::Color => {
                let str =
                    unsafe { CString::from_raw(canvas_native_paint_style_get_color_string(style) as _) };
                let str = str.to_string_lossy();
                let str = str.to_string();
                env
                    .create_string_from_std(str)
                    .map(|str| str.into_unknown())
            }
            PaintStyleType::Gradient => CanvasGradient { style }
                .into_instance(env)
                .map(|gradient| gradient)
                .map(|gradient| unsafe { JsUnknown::from_napi_value(env.raw(), gradient.value) })?,
            PaintStyleType::Pattern => CanvasPattern { style }
                .into_instance(env)
                .map(|pattern| pattern)
                .map(|pattern| unsafe { JsUnknown::from_napi_value(env.raw(), pattern.value) })?,
        }
    }

    #[napi(setter, return_if_invalid)]
    pub fn set_fill_style(
        &self,
        style: Either3<JsString, ClassInstance<CanvasPattern>, ClassInstance<CanvasGradient>>,
    ) -> Result<()> {
        match style {
            Either3::A(color) => {
                let context = unsafe { &mut *self.context };
                context.get_context_mut().set_fill_style_with_color(color.into_utf8()?.as_str()?);
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
    pub fn global_composite_operation(&self) -> &str {
        let context = unsafe { &*self.context };
        context.get_context().global_composite_operation().to_str()
    }

    #[napi(setter)]
    pub fn set_global_composite_operation(&self, operation: JsString) {
        if let Some(operation) = operation.into_utf8().ok() {
            if let Ok(operation) = operation.as_str() {
                if let Some(operation) = CompositeOperationType::from_str(operation) {
                    let context = unsafe { &mut *self.context };
                    context.get_context_mut().set_global_composite_operation(operation);
                }
            }
        }
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
            ImageSmoothingQuality::High => "high"
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
                            _ => None
                        };
                        if let Some(quality) = quality {
                            context.get_context_mut().set_image_smoothing_quality(quality)
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
                    "round" => Some(LineCap::CapButt),
                    "butt" => Some(LineCap::CapRound),
                    "square" => Some(LineCap::CapSquare),
                    _ => None
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
                    _ => None
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
    pub fn stroke_style(&self, env: Env) -> Result<JsUnknown> {
        let style = canvas_c::canvas_native_context_get_stroke_style(self.context);
        let style_ref = unsafe { &*style };
        match style_ref.style_type() {
            PaintStyleType::Color => {
                let str =
                    unsafe { CString::from_raw(canvas_native_paint_style_get_color_string(style) as _) };
                let str = str.to_string_lossy();
                let str = str.to_string();
                env
                    .create_string_from_std(str)
                    .map(|str| str.into_unknown())
            }
            PaintStyleType::Gradient => CanvasGradient { style }
                .into_instance(env)
                .map(|gradient| gradient)
                .map(|gradient| unsafe { JsUnknown::from_napi_value(env.raw(), gradient.value) })?,
            PaintStyleType::Pattern => CanvasPattern { style }
                .into_instance(env)
                .map(|pattern| pattern)
                .map(|pattern| unsafe { JsUnknown::from_napi_value(env.raw(), pattern.value) })?,
        }
    }

    #[napi(setter)]
    pub fn set_stroke_style(
        &self,
        style: Either3<JsString, ClassInstance<CanvasPattern>, ClassInstance<CanvasGradient>>,
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
            TextAlign::END => "end"
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
                    _ => None
                };

                if let Some(align) = align {
                    context.get_context_mut().set_text_align(align);
                }
            }
        }
    }


    #[napi(getter)]
    pub fn text_baseline(&self) -> &str {
        let context = unsafe { &*self.context };
        match context.get_context().text_baseline() {
            TextBaseLine::TOP => "top",
            TextBaseLine::HANGING => "hanging",
            TextBaseLine::MIDDLE => "middle",
            TextBaseLine::ALPHABETIC => "alphabetic",
            TextBaseLine::IDEOGRAPHIC => "ideographic",
            TextBaseLine::BOTTOM => "bottom",
        }
    }

    #[napi(setter)]
    pub fn set_text_baseline(&self, value: JsString) {
        if let Some(value) = value.into_utf8().ok() {
            if let Ok(value) = value.as_str() {
                let context = unsafe { &mut *self.context };
                let value = match value {
                    "top" => Some(TextBaseLine::TOP),
                    "hanging" => Some(TextBaseLine::HANGING),
                    "middle" => Some(TextBaseLine::MIDDLE),
                    "alphabetic" => Some(TextBaseLine::ALPHABETIC),
                    "ideographic" => Some(TextBaseLine::IDEOGRAPHIC),
                    "bottom" => Some(TextBaseLine::BOTTOM),
                    _ => None
                };

                if let Some(value) = value {
                    context.get_context_mut().set_text_baseline(value);
                }
            }
        }
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
    pub fn arc(&self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64, anticlockwise: Option<bool>) {
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
            self.context, x1 as f32, y1 as f32, x2 as f32, y2 as f32, radius as f32,
        )
    }

    #[napi]
    pub fn begin_path(&self) {
        canvas_c::canvas_native_context_begin_path(self.context);
    }

    #[napi]
    pub fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        canvas_c::canvas_native_context_bezier_curve_to(
            self.context, cp1x as f32, cp1y as f32, cp2x as f32, cp2y as f32, x as f32, y as f32,
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
        )
    }

    #[napi]
    pub fn clip(&self, path: Option<ClassInstance<Path2D>>, rule: Option<JsString>) {
        match (path, rule) {
            (None, None) => {
                canvas_c::canvas_native_context_clip_rule(
                    self.context, CanvasFillRule::NonZero,
                )
            }
            (Some(path), None) => {
                canvas_c::canvas_native_context_clip(self.context, path.path, CanvasFillRule::NonZero)
            }
            (None, Some(rule)) => {
                if let Some(rule) = rule.into_utf8().ok() {
                    if let Ok(rule) = rule.as_str() {
                        match rule {
                            "nonzero" => {
                                canvas_c::canvas_native_context_clip_rule(self.context, CanvasFillRule::NonZero)
                            }
                            "evenodd" => {
                                canvas_c::canvas_native_context_clip_rule(self.context, CanvasFillRule::EvenOdd)
                            }
                            _ => {}
                        }
                    }
                }
            }
            (Some(path), Some(rule)) => {
                if let Some(rule) = rule.into_utf8().ok() {
                    if let Ok(rule) = rule.as_str() {
                        match rule {
                            "nonzero" => {
                                canvas_c::canvas_native_context_clip(self.context, path.path, CanvasFillRule::NonZero)
                            }
                            "evenodd" => {
                                canvas_c::canvas_native_context_clip(self.context, path.path, CanvasFillRule::EvenOdd)
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    #[napi]
    pub fn close_path(&self) {
        canvas_c::canvas_native_context_close_path(
            self.context)
    }

    #[napi]
    pub fn create_conic_gradient(&self, env: Env, start_angle: f64, x: f64, y: f64) -> Result<ClassInstance<CanvasGradient>> {
        let gradient = canvas_c::canvas_native_context_create_conic_gradient(self.context, start_angle as f32, x as f32, y as f32);
        CanvasGradient { style: gradient }
            .into_instance(env)
    }

    #[napi]
    pub fn create_image_data(&self, env: Env, width_or_image_data: Either<f64, ClassInstance<ImageData>>, height: Option<f64>) -> Result<ClassInstance<ImageData>> {
        match width_or_image_data {
            Either::A(width) => {
                if let Some(height) = height {
                    return ImageData {
                        data: canvas_c::canvas_native_context_create_image_data(width as i32, height as i32)
                    }.into_instance(env);
                }
                Err(napi::Error::from_reason("Argument 1 is not an object."))
            }
            Either::B(value) => {
                ImageData {
                    data: canvas_c::canvas_native_context_create_image_data(value.width_inner(), value.height_inner())
                }.into_instance(env)
            }
        }
    }


    #[napi]
    pub fn create_linear_gradient(&self, env: Env, x0: f64, y0: f64, x1: f64, y1: f64) -> Result<ClassInstance<CanvasGradient>> {
        let gradient = canvas_c::canvas_native_context_create_linear_gradient(self.context, x0 as f32, y0 as f32, x1 as f32, y1 as f32);
        CanvasGradient { style: gradient }
            .into_instance(env)
    }

    #[napi]
    pub fn create_pattern(&self, env: Env, image: ClassInstance<ImageAsset>, repetition: Option<JsString>) -> Result<ClassInstance<CanvasPattern>> {
        let repetition = match repetition {
            Some(repetition) => {
                if let Some(repetition) = repetition.into_utf8().ok() {
                    if let Ok(repetition) = repetition.as_str() {
                        match repetition {
                            "repeat" => CanvasRepetition::Repeat,
                            "repeat-x" => CanvasRepetition::RepeatX,
                            "repeat-y" => CanvasRepetition::RepeatY,
                            "no-repeat" => CanvasRepetition::NoRepeat,
                            _ => CanvasRepetition::Repeat
                        }
                    } else {
                        CanvasRepetition::Repeat
                    }
                } else {
                    CanvasRepetition::Repeat
                }
            }
            _ => CanvasRepetition::Repeat
        };

        let pattern = canvas_c::canvas_native_context_create_pattern_asset(self.context, image.asset.as_ref(), repetition);
        CanvasPattern { style: pattern }
            .into_instance(env)
    }


    #[napi]
    pub fn create_radial_gradient(&self, env: Env, x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) -> Result<ClassInstance<CanvasGradient>> {
        let gradient = canvas_c::canvas_native_context_create_radial_gradient(self.context, x0 as f32, y0 as f32, r0 as f32, x1 as f32, y1 as f32, r1 as f32);
        CanvasGradient { style: gradient }
            .into_instance(env)
    }

    #[napi]
    pub fn draw_focus_if_needed(&self, element_path: Either<JsObject, ClassInstance<Path2D>>, element: Option<JsObject>) {}


    #[napi]
    pub fn draw_image(&self, image: ClassInstance<ImageAsset>, sx: Option<f64>, sy: Option<f64>, s_width: Option<f64>, s_height: Option<f64>,
                      dx: Option<f64>, dy: Option<f64>, d_width: Option<f64>, d_height: Option<f64>,
    ) {
        let image = Arc::as_ptr(&image.asset);
        match (sx, sy, s_width, s_height, dx, dy, d_height, d_width) {
            (Some(dx), Some(dy), None, None, None, None, None, None) => {
                canvas_c::canvas_native_context_draw_image_dx_dy_asset(
                    self.context, image as _, dx as f32, dy as f32,
                );
            }
            (Some(dx), Some(dy), Some(d_width), Some(d_height), None, None, None, None) => {
                canvas_c::canvas_native_context_draw_image_dx_dy_dw_dh_asset(
                    self.context, image as _, dx as f32, dy as f32, d_width as f32, d_height as f32,
                );
            }
            (Some(sx), Some(sy), Some(s_width), Some(s_height), Some(dx), Some(dy), Some(d_width), Some(d_height)) => {
                canvas_c::canvas_native_context_draw_image_asset(
                    self.context, image as _, sx as f32, sy as f32, s_width as f32, s_height as f32,
                    dx as f32, dy as f32, d_width as f32, d_height as f32,
                );
            }
            _ => {}
        }
    }

    #[napi]
    pub fn ellipse(&self,
                   x: f64,
                   y: f64,
                   radius_x: f64,
                   radius_y: f64,
                   rotation: f64,
                   start_angle: f64,
                   end_angle: f64,
                   anticlockwise: Option<bool>) {
        canvas_c::canvas_native_context_ellipse(self.context, x as f32, y as f32, radius_x as f32, radius_y as f32, rotation as f32, start_angle as f32, end_angle as f32, anticlockwise.unwrap_or(false))
    }

    #[napi]
    pub fn fill(&self, path: Option<ClassInstance<Path2D>>, rule: Option<JsString>) {
        match (path, rule) {
            (None, None) => {
                canvas_c::canvas_native_context_fill(
                    self.context, CanvasFillRule::NonZero,
                )
            }
            (Some(path), None) => {
                canvas_c::canvas_native_context_fill_with_path(self.context, path.path, CanvasFillRule::NonZero)
            }
            (None, Some(rule)) => {
                if let Some(rule) = rule.into_utf8().ok() {
                    if let Ok(rule) = rule.as_str() {
                        match rule {
                            "nonzero" => {
                                canvas_c::canvas_native_context_fill(self.context, CanvasFillRule::NonZero)
                            }
                            "evenodd" => {
                                canvas_c::canvas_native_context_fill(self.context, CanvasFillRule::EvenOdd)
                            }
                            _ => {}
                        }
                    }
                }
            }
            (Some(path), Some(rule)) => {
                if let Some(rule) = rule.into_utf8().ok() {
                    if let Ok(rule) = rule.as_str() {
                        match rule {
                            "nonzero" => {
                                canvas_c::canvas_native_context_fill_with_path(self.context, path.path, CanvasFillRule::NonZero)
                            }
                            "evenodd" => {
                                canvas_c::canvas_native_context_fill_with_path(self.context, path.path, CanvasFillRule::EvenOdd)
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }


    #[napi]
    pub fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        canvas_c::canvas_native_context_fill_rect(
            self.context,
            x as f32,
            y as f32,
            width as f32,
            height as f32,
        )
    }

    #[napi]
    pub fn fill_text(&self, text: JsString, x: f64, y: f64, max_width: Option<f64>) {
        if let Some(text) = text.into_utf8().ok() {
            if let Ok(text) = text.as_str() {
                let context = unsafe { &mut *self.context };
                context.get_context_mut().fill_text(
                    text, x as f32, y as f32, max_width.map(|width| width as f32),
                )
            }
        }
    }

    #[napi]
    pub fn get_image_data(&self, env: Env, x: f64, y: f64, width: f64, height: f64) -> Result<ClassInstance<ImageData>> {
        ImageData {
            data: canvas_c::canvas_native_context_get_image_data(
                self.context,
                x as f32,
                y as f32,
                width as f32,
                height as f32,
            )
        }.into_instance(env)
    }

    #[napi]
    pub fn get_line_dash(&self) -> Vec<f64> {
        let context = unsafe { &*self.context };
        context.get_context().line_dash()
            .to_vec()
            .into_iter()
            .map(|v| v as f64)
            .collect::<Vec<f64>>()
    }

    #[napi]
    pub fn get_transform(&self, env: Env) -> Result<ClassInstance<DOMMatrix>> {
        DOMMatrix {
            matrix: canvas_c::canvas_native_context_get_transform(self.context),
        }.into_instance(env)
    }

    #[napi]
    pub fn is_context_lost(&self) -> bool {
        // todo
        false
    }

    #[napi]
    pub fn is_point_in_path(&self, x_or_path: Either<f64, ClassInstance<Path2D>>, y: Option<f64>, rule_or_y: Option<Either<f64, JsString>>, rule: Option<JsString>) -> bool {
        let mut x_inner = None;
        let mut y_inner = None;
        let mut rule_inner = CanvasFillRule::NonZero;
        match x_or_path {
            Either::A(x) => {
                x_inner = Some(x as f32);
                if let Some(y) = y {
                    y_inner = Some(y as f32);

                    return if let Some(rule_or_y) = rule_or_y {
                        match rule_or_y {
                            Either::A(y) => {
                                // return early
                                canvas_c::canvas_native_context_is_point_in_path(
                                    self.context, x as f32, y as f32, rule_inner,
                                )
                            }
                            Either::B(rule) => {
                                if let Some(rule) = rule.into_utf8().ok() {
                                    if let Ok(rule) = rule.as_str() {
                                        match rule {
                                            "evenodd" => {
                                                rule_inner = CanvasFillRule::EvenOdd;
                                            }
                                            _ => {}
                                        }
                                    }
                                }

                                canvas_c::canvas_native_context_is_point_in_path(
                                    self.context, x as f32, y as f32, rule_inner,
                                )
                            }
                        }
                    } else {
                        canvas_c::canvas_native_context_is_point_in_path(
                            self.context, x as f32, y as f32, rule_inner,
                        )
                    };
                }
                false
            }
            Either::B(path) => {
                match (y, rule_or_y, rule) {
                    (Some(x), Some(rule_or_y), Some(rule)) => {
                        match rule_or_y {
                            Either::A(y) => {
                                if let Some(rule) = rule.into_utf8().ok() {
                                    if let Ok(rule) = rule.as_str() {
                                        match rule {
                                            "evenodd" => {
                                                rule_inner = CanvasFillRule::EvenOdd;
                                            }
                                            _ => {}
                                        }
                                    }
                                }

                                canvas_c::canvas_native_context_is_point_in_path_with_path(
                                    self.context, path.path, x as f32, y as f32, rule_inner,
                                )
                            }
                            Either::B(_) => {
                                false
                            }
                        }
                    }
                    (Some(x), Some(rule_or_y), _) => {
                        match rule_or_y {
                            Either::A(y) => {
                                canvas_c::canvas_native_context_is_point_in_path(
                                    self.context, x as f32, y as f32, rule_inner,
                                )
                            }
                            Either::B(_) => {
                                false
                            }
                        }
                    }
                    _ => false
                }
            }
        }
    }


    #[napi]
    pub fn is_point_in_stroke(&self, x_or_path: Either<f64, ClassInstance<Path2D>>, x_or_y: Option<f64>, y: Option<f64>) -> bool {
        match x_or_path {
            Either::A(x) => {
                if let Some(y) = x_or_y {
                    return canvas_c::canvas_native_context_is_point_in_stroke(
                        self.context, x as f32, y as f32,
                    );
                }
                false
            }
            Either::B(path) => {
                match (x_or_y, y) {
                    (Some(x), Some(y)) => {
                        canvas_c::canvas_native_context_is_point_in_stroke_with_path(
                            self.context, path.path, x as f32, y as f32,
                        )
                    }
                    _ => false
                }
            }
        }
    }


    #[napi]
    pub fn line_to(&self, x: f64, y: f64) {
        canvas_c::canvas_native_context_line_to(self.context, x as f32, y as f32)
    }

    #[napi]
    pub fn measure_text(&self, env: Env, text: String) -> Result<ClassInstance<TextMetrics>> {
        let text = CString::new(text).unwrap();
        TextMetrics {
            metrics: canvas_c::canvas_native_context_measure_text(self.context, text.as_ptr())
        }.into_instance(env)
    }


    #[napi]
    pub fn move_to(&self, x: f64, y: f64) {
        canvas_c::canvas_native_context_move_to(self.context, x as f32, y as f32)
    }

    #[napi]
    pub fn put_image_data(&self, image_data: ClassInstance<ImageData>, dx: f64, dy: f64, dirty_x: Option<f64>, dirty_y: Option<f64>, dirty_width: Option<f64>, dirty_height: Option<f64>) {
        match (dirty_x, dirty_y, dirty_width, dirty_height) {
            (Some(x), Some(y), Some(width), Some(height)) => {
                canvas_c::canvas_native_context_put_image_data(
                    self.context, image_data.data, dx as f32, dy as f32, x as f32, y as f32, width as f32, height as f32,
                )
            }
            _ => {
                canvas_c::canvas_native_context_put_image_data(
                    self.context, image_data.data, dx as f32, dy as f32, 0., 0., image_data.width_inner() as f32, image_data.height_inner() as f32,
                )
            }
        }
    }

    #[napi]
    pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) {
        canvas_c::canvas_native_context_quadratic_curve_to(
            self.context, cpx as f32, cpy as f32, x as f32, y as f32)
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
        canvas_c::canvas_native_context_scale(
            self.context,
            x as f32,
            y as f32,
        )
    }

    #[napi]
    pub fn set_line_dash(&self, segments: Vec<f64>) {
        let segments = segments.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
        canvas_c::canvas_native_context_set_line_dash(self.context, segments.as_ptr(), segments.len() as _);
    }

    #[napi]
    pub fn stroke(&self, path: Option<ClassInstance<Path2D>>) {
        match path {
            None => {
                canvas_c::canvas_native_context_stroke(
                    self.context
                )
            }
            Some(path) => {
                canvas_c::canvas_native_context_stroke_with_path(self.context, path.path)
            }
        }
    }


    #[napi]
    pub fn stroke_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        canvas_c::canvas_native_context_stroke_rect(
            self.context,
            x as f32,
            y as f32,
            width as f32,
            height as f32,
        )
    }

    #[napi]
    pub fn stroke_text(&self, text: JsString, x: f64, y: f64, max_width: Option<f64>) {
        if let Some(text) = text.into_utf8().ok() {
            if let Ok(text) = text.as_str() {
                let context = unsafe { &mut *self.context };
                context.get_context_mut().stroke_text(
                    text, x as f32, y as f32, max_width.map(|width| width as f32),
                )
            }
        }
    }


    #[napi]
    pub fn stroke_oval(&self, x: f64, y: f64, width: f64, height: f64) {
        canvas_c::canvas_native_context_stroke_oval(
            self.context,
            x as f32,
            y as f32,
            width as f32,
            height as f32,
        )
    }
    #[napi]
    pub fn set_transform(&self, a: Either<f64, ClassInstance<DOMMatrix>>, b: Option<f64>, c: Option<f64>, d: Option<f64>, e: Option<f64>, f: Option<f64>) {
        match a {
            Either::A(a) => {
                match (b, c, d, e, f) {
                    (Some(b), Some(c), Some(d), Some(e), Some(f)) => {
                        canvas_c::canvas_native_context_set_transform(self.context, a as f32, b as f32, c as f32, d as f32, e as f32, f as f32);
                    }
                    _ => {}
                }
            }
            Either::B(b) => {
                canvas_c::canvas_native_context_set_transform_matrix(self.context, b.matrix);
            }
        }
    }

    #[napi]
    pub fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        canvas_c::canvas_native_context_transform(self.context, a as f32, b as f32, c as f32, d as f32, e as f32, f as f32);
    }


    #[napi]
    pub fn translate(&self, x: f64, y: f64) {
        canvas_c::canvas_native_context_translate(self.context, x as f32, y as f32);
    }
}
