pub mod path;


use crate::c2d::path::JSPath2D;
use crate::dom_matrix::JSDOMMatrix;
use crate::image_asset::JSImageAsset;
use canvas_2d::context::text_styles::text_direction::TextDirection;
use canvas_2d::utils::color::to_parsed_color;
use canvas_c::enums::CanvasFillRule;
use canvas_c::{canvas_native_context_get_current_fill_style_type, canvas_native_context_get_fill_style, canvas_native_context_get_style_type, canvas_native_context_set_fill_style, canvas_native_paint_style_get_color_string, CanvasRenderingContext2D, PaintStyle, PaintStyleType};
use napi::bindgen_prelude::{ClassInstance, Either, Either3, FromNapiValue};
use napi::*;
use std::ffi::CString;
use std::sync::Arc;


#[napi(js_name = "CanvasRenderingContext2D")]
pub struct JSCanvasRenderingContext2D {
    context: *mut CanvasRenderingContext2D,
}

#[napi(js_name = "CanvasPattern")]
pub struct JSCanvasPattern {
    style: *mut PaintStyle,
}

#[napi(js_name = "CanvasGradient")]
pub struct JSCCanvasGradient {
    style: *mut PaintStyle,
}

#[napi]
impl JSCanvasRenderingContext2D {
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
        JSCanvasRenderingContext2D {
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
    pub fn with_cpu(
        width: f64,
        height: f64,
        density: f64,
        alpha: bool,
        font_color: i32,
        ppi: f64,
        direction: u32,
    ) -> Self {
        JSCanvasRenderingContext2D {
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
    pub fn begin_path(&self) {
        canvas_c::canvas_native_context_begin_path(self.context);
    }

    #[napi]
    pub fn render(&self) {
        canvas_c::canvas_native_context_flush(self.context);
        canvas_c::canvas_native_context_render(self.context);
    }

    #[napi]
    pub fn fill(&self, path: Option<ClassInstance<JSPath2D>>, rule: Option<JsString>) {
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
    pub fn global_alpha(&self) -> f64 {
        let context = unsafe { &*self.context };
        context.get_context().global_alpha() as f64
    }

    #[napi(setter)]
    pub fn set_global_alpha(&self, alpha: f64) {
        canvas_c::canvas_native_context_set_global_alpha(self.context, alpha as f32)
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
    pub fn image_smoothing_enabled(&self) -> bool {
        let context = unsafe { &*self.context };
        canvas_c::canvas_native_context_get_image_smoothing_enabled(self.context)
    }

    #[napi(setter)]
    pub fn set_image_smoothing_enabled(&self, enabled: bool) {
        canvas_c::canvas_native_context_set_image_smoothing_enabled(self.context, enabled);
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
            PaintStyleType::Gradient => JSCCanvasGradient { style }
                .into_instance(env)
                .map(|gradient| gradient)
                .map(|gradient| unsafe { JsUnknown::from_napi_value(env.raw(), gradient.value) })?,
            PaintStyleType::Pattern => JSCanvasPattern { style }
                .into_instance(env)
                .map(|pattern| pattern)
                .map(|pattern| unsafe { JsUnknown::from_napi_value(env.raw(), pattern.value) })?,
        }
    }

    #[napi(setter)]
    pub fn set_fill_style(
        &self,
        style: Either3<JsString, ClassInstance<JSCanvasPattern>, ClassInstance<JSCCanvasGradient>>,
    ) {
        match style {
            Either3::A(color) => {
                if let Some(color) = color.into_utf8().ok() {
                    if let Ok(color) = color.as_str() {
                        let context = unsafe { &mut *self.context };
                        context.get_context_mut().set_fill_style_with_color(color);
                    }
                }
            }
            Either3::B(pattern) => canvas_native_context_set_fill_style(self.context, pattern.style),
            Either3::C(gradient) => canvas_native_context_set_fill_style(self.context, gradient.style),
        }
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
            PaintStyleType::Gradient => JSCCanvasGradient { style }
                .into_instance(env)
                .map(|gradient| gradient)
                .map(|gradient| unsafe { JsUnknown::from_napi_value(env.raw(), gradient.value) })?,
            PaintStyleType::Pattern => JSCanvasPattern { style }
                .into_instance(env)
                .map(|pattern| pattern)
                .map(|pattern| unsafe { JsUnknown::from_napi_value(env.raw(), pattern.value) })?,
        }
    }

    #[napi(setter)]
    pub fn set_stroke_style(
        &self,
        style: Either3<JsString, ClassInstance<JSCanvasPattern>, ClassInstance<JSCCanvasGradient>>,
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
    pub fn font(&self) -> &str {
        let context = unsafe { &*self.context };
        context.get_context().font()
    }

    #[napi(setter)]
    pub fn set_font(&self, value: String) {
        let context = unsafe { &mut *self.context };
        context.get_context_mut().set_font(&value);
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


    #[napi]
    pub fn draw_image(&self, image: ClassInstance<JSImageAsset>, sx: Option<f64>, sy: Option<f64>, s_width: Option<f64>, s_height: Option<f64>,
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
    pub fn set_line_dash(&self, segments: Vec<f64>) {
        let segments = segments.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
        canvas_c::canvas_native_context_set_line_dash(self.context, segments.as_ptr(), segments.len() as _);
    }

    #[napi]
    pub fn stroke(&self, path: Option<ClassInstance<JSPath2D>>) {
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
    pub fn set_transform(&self, a: Either<f64, ClassInstance<JSDOMMatrix>>, b: Option<f64>, c: Option<f64>, d: Option<f64>, e: Option<f64>, f: Option<f64>) {
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
