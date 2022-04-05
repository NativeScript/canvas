use std::fmt::format;

use cxx::{CxxString, let_cxx_string};
use parking_lot::MutexGuard;

use canvas_core::context::{Context, ContextWrapper};
use canvas_core::context::compositing::composite_operation_type::CompositeOperationType;
use canvas_core::context::drawing_paths::fill_rule::FillRule;
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

use crate::bridges::image_asset::ImageAsset;
use crate::bridges::image_data::ImageData;
use crate::bridges::matrix::Matrix;
use crate::bridges::path::Path;
use crate::gl_context::GLContext;

#[derive(Clone)]
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
}

#[derive(Clone, Copy)]
pub struct TextMetrics(canvas_core::context::drawing_text::text_metrics::TextMetrics);

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Matrix;
        type CanvasRenderingContext2D;
        type PaintStyle;
        type TextMetrics;
        type Path;
        type ImageData;
        type ImageAsset;

        fn canvas_native_context_create() -> Box<CanvasRenderingContext2D>;
        fn canvas_native_context_get_global_alpha(context: &CanvasRenderingContext2D) -> f32;
        fn canvas_native_context_set_global_alpha(
            context: &mut CanvasRenderingContext2D,
            alpha: f32,
        );

        fn canvas_native_context_get_image_smoothing_enabled(
            context: &CanvasRenderingContext2D,
        ) -> bool;

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

        fn canvas_native_context_get_line_dash(context: &CanvasRenderingContext2D) -> &[f32];

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

    }
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
    let ctx = GLContext::get_current();
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

pub fn canvas_native_context_get_line_dash(context: &CanvasRenderingContext2D) -> &[f32] {
    context.get_context().line_dash()
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
    let width = image.width() as f32;
    let height = image.height() as f32;
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
    let width = image.width() as f32;
    let height = image.height() as f32;
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
    FillRule::try_from(rule).map_or(false, |rule| context.is_point_in_path(None, x, y, rule))
}

pub fn canvas_native_context_is_point_in_path_with_path(
    context: &mut CanvasRenderingContext2D,
    path: &mut Path,
    x: f32,
    y: f32,
    rule: &str,
) -> bool {
    FillRule::try_from(rule).map_or(false, |rule| {
        context.is_point_in_path(Some(path.inner()), x, y, rule)
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
