use std::f32::consts::PI;

use roxmltree::{Children, Document, Node};
use skia_safe::{Color, Image, SamplingOptions, Vector};

use crate::common::context::compositing::composite_operation_type::CompositeOperationType;
use crate::common::context::Context;
use crate::common::context::drawing_paths::fill_rule::FillRule;
use crate::common::context::drawing_text::typography::parse_font;
use crate::common::context::fill_and_stroke_styles::paint::PaintStyle;
use crate::common::context::fill_and_stroke_styles::pattern::Repetition;
use crate::common::context::image_smoothing::ImageSmoothingQuality;
use crate::common::context::line_styles::line_cap::LineCap;
use crate::common::context::line_styles::line_join::LineJoin;
use crate::common::context::matrix::Matrix;
use crate::common::context::paths::path::Path;
use crate::common::svg::attribute_names::{Attribute, NodeExt};
use crate::common::svg::bounding_box::BoundingBox;
use crate::common::svg::elements::clip_path::{ClipPath, shapes::ClipShape};
use crate::common::svg::elements::element_names::ElementName;
use crate::common::svg::elements::filter::filter::Filter;
use crate::common::svg::elements::filter::filter_element::FilterElement;
use crate::common::svg::elements::filter::filter_in::FilterIn;
use crate::common::svg::elements::gradients::linear::LinearGradient;
use crate::common::svg::elements::gradients::radial::RadialGradient;
use crate::common::svg::elements::mask::Mask;
use crate::common::svg::elements::parser::{parse_transform, points_from_str, StyleMap};
use crate::common::svg::elements::pattern::Pattern;
use crate::common::svg::elements::prelude::Parser;
use crate::common::svg::elements::reference_element::ReferenceElement;
use crate::common::svg::elements::svg::create_context;
use crate::common::svg::enums::display::Display;
use crate::common::svg::enums::preserve_aspect_ratio::view_box_to_transform;
use crate::common::svg::enums::visibility::Visibility;
use crate::common::svg::prelude::*;
use crate::common::svg::units::length::convert_length;
use crate::common::svg::units::Units;
use crate::common::svg::view_box::ViewBox;
use crate::common::utils::color::parse_color;

use super::svg::Svg;

pub trait Renderer {
    fn render(&mut self, context: &mut Context, node: Node, root_element: Node);
    fn bounding_box(&self) -> BoundingBox;
}

pub struct RenderedStyle {
    clip: Option<ClipPath>,
    clip_rule: FillRule,
    fill_rule: FillRule,
    display: Display,
    fill: bool,
    stroke: bool,
    mask: Option<Mask>,
    filter: Option<Filter>,
    visibility: Visibility,
}

impl RenderedStyle {
    pub fn clip(&self) -> Option<&ClipPath> {
        self.clip.as_ref()
    }

    pub fn clip_rule(&self) -> FillRule {
        self.clip_rule
    }

    pub fn fill_rule(&self) -> FillRule {
        self.fill_rule
    }

    pub fn display(&self) -> Display {
        self.display
    }

    pub fn fill(&self) -> bool {
        self.fill
    }

    pub fn stroke(&self) -> bool {
        self.stroke
    }

    pub fn mask(&self) -> Option<&Mask> {
        self.mask.as_ref()
    }

    pub fn filter(&self) -> Option<&Filter> {
        self.filter.as_ref()
    }

    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    pub fn is_visible(&self) -> bool {
        if self.display == Display::None
            || self.visibility == Visibility::Collapse
            || self.visibility == Visibility::Hidden
        {
            return false;
        }
        true
    }
}

impl Default for RenderedStyle {
    fn default() -> Self {
        Self {
            clip: None,
            clip_rule: Default::default(),
            fill_rule: Default::default(),
            display: Default::default(),
            fill: true,
            stroke: false,
            mask: None,
            filter: None,
            visibility: Default::default(),
        }
    }
}

pub fn handle_style_data(
    style: &StyleMap,
    context: &mut Context,
    root_element: Node,
    bounding_box: BoundingBox,
) -> RenderedStyle {
    let mut rendered = RenderedStyle::default();
    let view_box = ViewBox::new_with_context(context);
    let device = context.device;
    for (key, val) in style.iter() {
        match key {
            Attribute::ClipPath => {
                rendered.clip = handle_clip(root_element, &val);
                let mut clip_path = Path::default();
                if let Some(clips) = rendered.clip.as_ref() {
                    for clip in clips.clips().iter() {
                        match clip {
                            ClipShape::None => {}
                            ClipShape::Circle { cx, cy, r } => {
                                let mut w = 1.0;
                                let mut h = 1.0;
                                let mut r_ = 1.0;
                                if clips.units() == Units::ObjectBoundingBox {
                                    let cx = convert_length(
                                        *cx,
                                        Attribute::Cx,
                                        Units::UserSpaceOnUse,
                                        device,
                                        &view_box,
                                    );
                                    let cy = convert_length(
                                        *cy,
                                        Attribute::Cy,
                                        Units::UserSpaceOnUse,
                                        device,
                                        &view_box,
                                    );

                                    let cr = convert_length(
                                        *r,
                                        Attribute::R,
                                        Units::UserSpaceOnUse,
                                        device,
                                        &view_box,
                                    );
                                    let bounds = skia_safe::Rect::from_xywh(
                                        cx - cr,
                                        cy - cr,
                                        cx + cr,
                                        cy + cr,
                                    );
                                    w = bounds.width();
                                    h = bounds.height();
                                    r_ = (bounds.width() + bounds.height()) / 2.0;
                                }
                                clip_path.arc(
                                    convert_length(
                                        *cx,
                                        Attribute::Cx,
                                        clips.units(),
                                        device,
                                        &view_box,
                                    ) * w,
                                    convert_length(
                                        *cy,
                                        Attribute::Cy,
                                        clips.units(),
                                        device,
                                        &view_box,
                                    ) * h,
                                    convert_length(
                                        *r,
                                        Attribute::R,
                                        clips.units(),
                                        device,
                                        &view_box,
                                    ) * r_,
                                    0f32,
                                    2f32 * PI,
                                    false,
                                );
                            }
                            ClipShape::Ellipse { cx, cy, rx, ry } => {
                                clip_path.ellipse(
                                    convert_length(
                                        *cx,
                                        Attribute::Cx,
                                        clips.units(),
                                        device,
                                        &view_box,
                                    ),
                                    convert_length(
                                        *cy,
                                        Attribute::Cy,
                                        clips.units(),
                                        device,
                                        &view_box,
                                    ),
                                    convert_length(
                                        *rx,
                                        Attribute::Rx,
                                        clips.units(),
                                        device,
                                        &view_box,
                                    ),
                                    convert_length(
                                        *ry,
                                        Attribute::Ry,
                                        clips.units(),
                                        device,
                                        &view_box,
                                    ),
                                    0f32,
                                    0f32,
                                    2f32 * PI,
                                    false,
                                );
                            }
                            ClipShape::Line { x1, y1, x2, y2 } => {
                                let mut line = Path::new();
                                line.move_to(
                                    convert_length(
                                        *x1,
                                        Attribute::X1,
                                        Units::UserSpaceOnUse,
                                        device,
                                        &view_box,
                                    ),
                                    convert_length(
                                        *y1,
                                        Attribute::Y1,
                                        Units::UserSpaceOnUse,
                                        device,
                                        &view_box,
                                    ),
                                );
                                line.line_to(
                                    convert_length(
                                        *x2,
                                        Attribute::X2,
                                        Units::UserSpaceOnUse,
                                        device,
                                        &view_box,
                                    ),
                                    convert_length(
                                        *y2,
                                        Attribute::Y2,
                                        Units::UserSpaceOnUse,
                                        device,
                                        &view_box,
                                    ),
                                );
                                clip_path.add_path(&line, None);
                            }
                            ClipShape::Polygon { points } => {
                                let polygon_points = points_from_str(points);
                                let mut poly = Path::new();
                                for (index, value) in polygon_points.iter().enumerate() {
                                    if index == 0 {
                                        &mut poly.move_to(
                                            *value.first().unwrap(),
                                            *value.last().unwrap(),
                                        );
                                    } else {
                                        &mut poly.line_to(
                                            *value.first().unwrap(),
                                            *value.last().unwrap(),
                                        );
                                    }
                                }
                                if !poly.path.is_empty() {
                                    clip_path.add_path(&poly, None);
                                }
                            }
                            ClipShape::Polyline { points } => {
                                let polyline_points = points_from_str(points);
                                let mut poly = Path::new();
                                for (index, value) in polyline_points.iter().enumerate() {
                                    if index == 0 {
                                        &mut poly.move_to(
                                            *value.first().unwrap(),
                                            *value.last().unwrap(),
                                        );
                                    } else {
                                        &mut poly.line_to(
                                            *value.first().unwrap(),
                                            *value.last().unwrap(),
                                        );
                                    }
                                }
                                if !poly.path.is_empty() {
                                    poly.close_path();
                                    clip_path.add_path(&poly, None);
                                }
                            }
                            ClipShape::Rect {
                                x,
                                y,
                                width,
                                height,
                            } => {
                                clip_path.rect(
                                    convert_length(
                                        *x,
                                        Attribute::X,
                                        clips.units(),
                                        context.device,
                                        &view_box,
                                    ),
                                    convert_length(
                                        *y,
                                        Attribute::Y,
                                        clips.units(),
                                        context.device,
                                        &view_box,
                                    ),
                                    convert_length(
                                        *width,
                                        Attribute::Width,
                                        clips.units(),
                                        context.device,
                                        &view_box,
                                    ),
                                    convert_length(
                                        *height,
                                        Attribute::Height,
                                        clips.units(),
                                        context.device,
                                        &view_box,
                                    ),
                                );
                            }
                            ClipShape::Path { d } => {
                                let path = Path::from_str(d);
                                if !path.path.is_empty() {
                                    clip_path.add_path(&path, None);
                                }
                            }
                            ClipShape::Text {
                                x,
                                y,
                                dx,
                                dy,
                                text,
                                font_size,
                                font_stretch,
                                font_family,
                                font_weight,
                                font_style,
                                font_variant,
                            } => {
                                let mut path = Path::new();
                                let units = clips.units();
                                let x = convert_length(*x, Attribute::X, units, device, &view_box)
                                    + convert_length(*dx, Attribute::Dx, units, device, &view_box);
                                let y = convert_length(*y, Attribute::Y, units, device, &view_box)
                                    + convert_length(*dy, Attribute::Dy, units, device, &view_box);
                                let mut matrix = skia_safe::Matrix::default();
                                matrix.post_translate(Vector::new(x, y));
                                path.path.transform(&matrix);
                                let font_str = format!(
                                    "{} {} {} {} {}",
                                    font_style, font_variant, font_weight, font_size, font_family
                                );
                                if let Ok(font) = parse_font(&font_str, device) {
                                    let font = font.1;
                                    let glyphs = font.str_to_glyphs_vec(text);
                                    let mut points =
                                        vec![skia_safe::Point::default(); glyphs.capacity()];
                                    font.get_pos(&glyphs, points.as_mut_slice(), None);
                                    for (glyph, point) in
                                    glyphs.into_iter().zip(&points).into_iter()
                                    {
                                        if let Some(text_path) = font.get_path(glyph) {
                                            let mut matrix = skia_safe::Matrix::default();
                                            matrix.post_translate(Vector::new(point.x, point.y));
                                            path.path.add_path_matrix(&text_path, &matrix, None);
                                        }
                                    }
                                }

                                if !path.path.is_empty() {
                                    clip_path.add_path(&path, None);
                                }
                            }
                        }
                    }
                }
                if !clip_path.path.is_empty() {
                    let mut clip_fill = FillRule::default();
                    if let Some(fill) = style.get(&Attribute::ClipRule) {
                        clip_fill = FillRule::from(fill);
                    }
                    context.clip(Some(&mut clip_path), Some(clip_fill));
                }
            }
            Attribute::ClipRule => {
                rendered.clip_rule = FillRule::from(val.as_str());
            }
            Attribute::FillRule => {
                rendered.fill_rule = FillRule::from(val.as_str());
            }
            Attribute::Display => {
                rendered.display = Display::from(val.as_str());
            }
            Attribute::Fill => {
                rendered.fill =
                    handle_color(context, Attribute::Fill, &val, root_element, bounding_box);
            }
            Attribute::Filter => rendered.filter = handle_filter(root_element, val.as_str()),
            Attribute::MarkerEnd => {}
            Attribute::MarkerMid => {}
            Attribute::MarkerStart => {}
            Attribute::Mask => {
                rendered.mask = get_handle_mask(val, root_element);
            }
            Attribute::Stroke => {
                rendered.stroke =
                    handle_color(context, Attribute::Stroke, &val, root_element, bounding_box);
            }
            Attribute::Transform => {
                if let Some(transform) = parse_transform(&val, device.density) {
                    context.transform_with_matrix(&transform);
                }
            }
            Attribute::Visibility => {
                rendered.visibility = Visibility::from(val.as_str());
            }
            _ => {
                handle_style(context, *key, &val);
            }
        }
    }
    rendered
}

fn parse_stroke_dash_array(value: &str) -> Option<Vec<f32>> {
    let mut has_error = false;
    if value.is_empty() {
        return None;
    }
    let array: Vec<_> = value
        .split(",")
        .into_iter()
        .filter(|v| !v.is_empty())
        .map(|v| {
            if let Ok(value) = v.parse::<f32>() {
                value
            } else {
                has_error = true;
                0.0
            }
        })
        .collect();
    if !has_error {
        return Some(array);
    }
    None
}

pub fn handle_filter(root: Node, val: &str) -> Option<Filter> {
    let handle_children = |children: Children, filter: &mut Filter| {
        for child in children.into_iter() {
            match ElementName::from_str(child.tag_name().name()) {
                None => {}
                Some(name) => match name {
                    ElementName::FeGaussianBlur => {
                        filter.filters_mut().push(FilterElement::FeGaussianBlur {
                            std_deviation: child
                                .attribute(Attribute::StdDeviation.to_str())
                                .unwrap_or("0.0")
                                .parse::<f32>()
                                .unwrap_or(0.0),
                            in_val: FilterIn::from_str(
                                child.attribute(Attribute::In.to_str()).unwrap_or(""),
                            )
                                .unwrap_or(FilterIn::SourceGraphic),
                        })
                    }
                    _ => {}
                },
            }
        }
    };
    match root.get_node(val) {
        (Some(ele), None) => {
            let mut filter = Filter::from_node((Some(ele), None));
            handle_children(ele.children(), &mut filter);
            Some(filter)
        }
        (Some(ele), Some(ref_ele)) => {
            let mut filter = Filter::from_node((Some(ele), Some(ref_ele)));
            handle_children(ref_ele.children(), &mut filter);
            Some(filter)
        }
        (_, _) => None,
    }
}

pub fn handle_clip<'a>(root: Node, value: &str) -> Option<ClipPath> {
    let parse_clip = |node: Node| {
        let mut clip = ClipPath::default();
        clip.set_units(
            match node
                .attribute(Attribute::ClipPathUnits.to_str())
                .unwrap_or("objectBoundingBox")
            {
                "userSpaceOnUse" => crate::common::svg::units::Units::UserSpaceOnUse,
                _ => crate::common::svg::units::Units::ObjectBoundingBox,
            },
        );
        let mut clips: Vec<ClipShape> = Vec::new();
        for node in node.children().into_iter() {
            if node.is_element() {
                clips.push(ClipShape::parse_from(node))
            }
        }

        return if clips.len() > 0 {
            clip.set_clips(clips.as_slice());
            Some(clip)
        } else {
            None
        };
    };
    match root.get_node(value) {
        (Some(node), _) => parse_clip(node),
        _ => None,
    }
}

pub fn get_handle_mask(value: &str, root: Node) -> Option<Mask> {
    if value.contains("url") {
        match root.get_element_from_id(value) {
            Some(name) => match name {
                ElementName::Mask => return Mask::from_id(value, root),
                _ => {}
            },
            _ => {}
        }
    }
    None
}

pub fn set_mask<'a>(
    mask: &Mask,
    context: &mut Context,
    view_box: ViewBox,
    bounding_box: BoundingBox,
    root_element: Node,
    style: &StyleMap,
) -> Option<(Context<'a>, skia_safe::Image, skia_safe::Rect)> {
    if let Some(mask) = mask.create_mask(context, view_box, bounding_box, root_element) {
        let density = context.device.density;
        let alpha = context.device.alpha;
        let font_color = context.font_color.to_int() as i32;
        let ppi = context.device.ppi;
        let direction = context.direction();
        let mut ctx = create_context(
            mask.1.width(),
            mask.1.height(),
            density,
            alpha,
            font_color,
            ppi,
            direction,
        );
        let _ = handle_style_data(style, &mut ctx, root_element, bounding_box);
        return Some((ctx, mask.0, mask.1));
    }
    None
}

pub fn render_mask(context: &mut Context, mut mask: (Context, skia_safe::Image, skia_safe::Rect)) {
    handle_mask(&mut mask.0, Some(&(mask.1, mask.2)));
    let ss = mask.0.surface.image_snapshot();
    context.draw_image_with_rect(&ss, mask.2);
}

pub fn handle_mask(context: &mut Context, mask: Option<&(skia_safe::Image, skia_safe::Rect)>) {
    if let Some(mask) = mask {
        context.set_fill_style(PaintStyle::Pattern(
            context.create_pattern(mask.0.clone(), Repetition::NoRepeat),
        ));
        context.set_global_composite_operation(CompositeOperationType::DestinationIn);
        context.fill_rect(&mask.1);
    }
}

pub fn handle_color(
    context: &mut Context,
    attr: Attribute,
    value: &str,
    root: Node,
    bounding_box: BoundingBox,
) -> bool {
    if value.eq("none") {
        return false;
    }
    if value.eq("currentColor") {
        return true;
    }
    let mut parse_value = |is_stroke: bool| {
        if !value.contains("url") {
            return if let Some(color) = parse_color(value) {
                if is_stroke {
                    context.set_stroke_style(PaintStyle::Color(color));
                } else {
                    context.set_fill_style(PaintStyle::Color(color));
                }
                true
            } else {
                return if is_stroke {
                    false
                } else {
                    context.set_fill_style(PaintStyle::Color(Color::BLACK));
                    true
                };
            };
        }
        let device = context.device;
        match root.get_element_from_id(value) {
            Some(name) => match name {
                ElementName::LinearGradient => {
                    if let Some(gradient) = LinearGradient::from_id(value, root) {
                        let mut matrix = skia_safe::Matrix::default();
                        let mut view_box = ViewBox::new_with_context(context);
                        if gradient.gradient_units() == Units::ObjectBoundingBox {
                            view_box = ViewBox::new(
                                bounding_box.x(),
                                bounding_box.y(),
                                bounding_box.width(),
                                bounding_box.height(),
                            );
                            let affine = [
                                view_box.width(),
                                0.0,
                                0.0,
                                view_box.height(),
                                view_box.x(),
                                view_box.y(),
                            ];
                            matrix = skia_safe::Matrix::from_affine(&affine);
                        }

                        if !gradient.gradient_transform().is_empty() {
                            if let Some(transform) =
                            parse_transform(gradient.gradient_transform(), device.density)
                            {
                                matrix.pre_concat(&transform);
                            }
                        }

                        let mut lg = context.create_linear_gradient_with_matrix(
                            convert_length(
                                gradient.x1(),
                                Attribute::X1,
                                gradient.gradient_units(),
                                device,
                                &view_box,
                            ),
                            convert_length(
                                gradient.y1(),
                                Attribute::Y1,
                                gradient.gradient_units(),
                                device,
                                &view_box,
                            ),
                            convert_length(
                                gradient.x2(),
                                Attribute::X2,
                                gradient.gradient_units(),
                                device,
                                &view_box,
                            ),
                            convert_length(
                                gradient.y2(),
                                Attribute::Y2,
                                gradient.gradient_units(),
                                device,
                                &view_box,
                            ),
                            Matrix::from(&matrix),
                        );
                        lg.set_tile_mode(gradient.spread_method().into());
                        for stop in gradient.stop_colors().into_iter() {
                            lg.add_color_stop(stop.0, stop.1)
                        }
                        let style = PaintStyle::Gradient(lg);
                        if is_stroke {
                            context.set_stroke_style(style)
                        } else {
                            context.set_fill_style(style)
                        }
                        return true;
                    }
                    false
                }
                ElementName::RadialGradient => {
                    if let Some(gradient) = RadialGradient::from_id(value, root) {
                        let mut matrix = skia_safe::Matrix::default();
                        let mut view_box = ViewBox::new_with_context(context);
                        if gradient.gradient_units() == Units::ObjectBoundingBox {
                            view_box = ViewBox::new(
                                bounding_box.x(),
                                bounding_box.y(),
                                bounding_box.width(),
                                bounding_box.height(),
                            );
                            let affine = [
                                view_box.width(),
                                0.0,
                                0.0,
                                view_box.height(),
                                view_box.x(),
                                view_box.y(),
                            ];
                            matrix = skia_safe::Matrix::from_affine(&affine);
                        }

                        if !gradient.gradient_transform().is_empty() {
                            if let Some(transform) =
                            parse_transform(gradient.gradient_transform(), device.density)
                            {
                                matrix.pre_concat(&transform);
                            }
                        }

                        let fx = convert_length(
                            gradient.fx(),
                            Attribute::Fx,
                            gradient.gradient_units(),
                            device,
                            &view_box,
                        );
                        let fy = convert_length(
                            gradient.fy(),
                            Attribute::Fy,
                            gradient.gradient_units(),
                            device,
                            &view_box,
                        );

                        let cx = convert_length(
                            gradient.cx(),
                            Attribute::Cx,
                            gradient.gradient_units(),
                            device,
                            &view_box,
                        );
                        let cy = convert_length(
                            gradient.cy(),
                            Attribute::Cy,
                            gradient.gradient_units(),
                            device,
                            &view_box,
                        );
                        let r = convert_length(
                            gradient.r(),
                            Attribute::R,
                            gradient.gradient_units(),
                            device,
                            &view_box,
                        );

                        let (fx, fy) = prepare_focal(cx, cy, r, fx, fy);
                        let mut rg = context.create_radial_gradient_with_matrix(
                            fx,
                            fy,
                            0.0,
                            cx,
                            cy,
                            r,
                            Matrix::from(&matrix),
                        );
                        rg.set_tile_mode(gradient.spread_method().into());
                        for stop in gradient.stop_colors().into_iter() {
                            rg.add_color_stop(stop.0, stop.1)
                        }
                        let style = PaintStyle::Gradient(rg);
                        if is_stroke {
                            context.set_stroke_style(style)
                        } else {
                            context.set_fill_style(style)
                        }
                        return true;
                    }
                    false
                }
                ElementName::Pattern => {
                    if let Some(pattern) = Pattern::from_id(value, root) {
                        let view_box = ViewBox::new_with_context(context);
                        if let Some(pattern) =
                        pattern.create_pattern(context, view_box, bounding_box, root)
                        {
                            return if is_stroke {
                                context.set_stroke_style(PaintStyle::Pattern(pattern));
                                true
                            } else {
                                context.set_fill_style(PaintStyle::Pattern(pattern));
                                true
                            };
                        }
                    }
                    false
                }
                _ => false,
            },
            _ => false,
        }
    };
    match attr {
        Attribute::Stroke => parse_value(true),
        Attribute::Fill => parse_value(false),
        _ => false,
    }
}

pub fn set_filters(context: &mut Context, filters: &Filter) {
    let filter = filters
        .filters()
        .iter()
        .fold(None, |chain, next_filter| match next_filter {
            FilterElement::FeGaussianBlur {
                std_deviation,
                in_val,
            } => match *in_val {
                FilterIn::SourceGraphic => skia_safe::image_filters::blur(
                    (*std_deviation, *std_deviation),
                    None,
                    chain,
                    None,
                ),
                FilterIn::SourceAlpha => chain,
                FilterIn::BackgroundImage => chain,
                FilterIn::BackgroundAlpha => chain,
                FilterIn::FillPaint => chain,
                FilterIn::StrokePaint => chain,
            },
        });
    context
        .state
        .paint
        .fill_paint_mut()
        .set_image_filter(filter.clone());
    context
        .state
        .paint
        .stroke_paint_mut()
        .set_image_filter(filter.clone());
    context
        .state
        .paint
        .image_paint_mut()
        .set_image_filter(filter);
}

pub fn handle_style(context: &mut Context, attr: Attribute, value: &str) {
    match attr {
        Attribute::GradientUnits => {}
        Attribute::ClipPathUnits => {}
        Attribute::MaskUnits => {}
        Attribute::AlignmentBaseline => {}
        Attribute::BaselineShift => {}
        Attribute::ClipPath => {}
        Attribute::ClipRule => {}
        Attribute::Color => {}
        Attribute::ColorInterpolation => {}
        Attribute::ColorInterpolationFilters => {}
        Attribute::ColorRendering => {}
        Attribute::Cursor => {}
        Attribute::Direction => {}
        Attribute::Display => {}
        Attribute::DominantBaseline => {}
        Attribute::Fill => {}
        Attribute::FillOpacity => {
            context
                .state
                .paint
                .fill_paint_mut()
                .set_alpha_f(value.parse::<f32>().unwrap_or(1.0));
        }
        Attribute::FillRule => {}
        Attribute::Filter => {}
        Attribute::FloodColor => {}
        Attribute::FloodOpacity => {}
        Attribute::FontFamily => {}
        Attribute::FontSize => {}
        Attribute::FontSizeAdjust => {}
        Attribute::FontStretch => {}
        Attribute::FontStyle => {}
        Attribute::FontVariant => {}
        Attribute::FontWeight => {}
        Attribute::ImageRendering => {}
        Attribute::LetterSpacing => {}
        Attribute::LightingColor => {}
        Attribute::MarkerEnd => {}
        Attribute::MarkerMid => {}
        Attribute::MarkerStart => {}
        Attribute::Mask => {}
        Attribute::Opacity => {
            let opacity = (value.parse::<f32>().unwrap_or(1.0) * 255.0) as u32;
            if opacity < 255 {
                context.surface.canvas().save_layer_alpha(None, opacity);
            }
        }
        Attribute::Overflow => {}
        Attribute::PointerEvents => {}
        Attribute::ShapeRendering => {}
        Attribute::SolidColor => {}
        Attribute::SolidOpacity => {}
        Attribute::StopColor => {}
        Attribute::StopOpacity => {}
        Attribute::Stroke => {}
        Attribute::StrokeDasharray => {
            if let Some(dasharray) = parse_stroke_dash_array(value) {
                context.set_line_dash(dasharray.as_slice());
            }
        }
        Attribute::StrokeDashoffset => {
            context.set_line_dash_offset(value.parse::<f32>().unwrap_or(0.0));
        }
        Attribute::StrokeLinecap => {
            context.set_line_cap(LineCap::from(value));
        }
        Attribute::StrokeLinejoin => {
            context.set_line_join(LineJoin::from(value));
        }
        Attribute::StrokeMiterlimit => {
            context.set_miter_limit(value.parse::<f32>().unwrap_or(10.0));
        }
        Attribute::StrokeOpacity => {
            context
                .state
                .paint
                .stroke_paint_mut()
                .set_alpha_f(value.parse::<f32>().unwrap_or(1.0));
        }
        Attribute::StrokeWidth => {
            context.set_line_width(value.parse::<f32>().unwrap_or(1.0));
        }
        Attribute::TextAnchor => {}
        Attribute::TextDecoration => {}
        Attribute::TextRendering => {}
        Attribute::Transform => {}
        Attribute::UnicodeBidi => {}
        Attribute::VectorEffect => {}
        Attribute::Visibility => {}
        Attribute::WordSpacing => {}
        Attribute::WritingMode => {}
        _ => {}
    }
}

pub fn handle_render_child(
    context: &mut Context,
    child: Node,
    root_element: Node,
    bounding_box: Option<&mut BoundingBox>,
    extra_style: Option<&StyleMap>,
) {
    use crate::common::svg::elements::g::G;
    use crate::common::svg::elements::image::Image;
    use crate::common::svg::elements::prelude::*;
    use crate::common::svg::elements::shapes::{
        circle::Circle, ellipse::Ellipse, line::Line, path::Path, polyshape::PolyShape, rect::Rect,
    };
    use crate::common::svg::elements::text::Text;
    use crate::common::svg::elements::use_element::Use;
    let mut default = BoundingBox::default();
    let bb = bounding_box.unwrap_or(&mut default);
    let mut update_bounding_box = |box_: &BoundingBox| bb.add_bounding_box(box_);
    let handle_extra_style = |style: &mut StyleMap| {
        if let Some(extra) = extra_style {
            style.update_with_style(extra, false);
        }
    };

    let device = context.device;
    let width = context.surface.width() as f32;
    let height = context.surface.height() as f32;
    let node = child;
    let tag = node.tag_name().name();
    if node.is_element() {
        match ElementName::from_str(tag) {
            None => {}
            Some(elem) => match elem {
                ElementName::Svg => {
                    if let Some(mut svg) = Svg::new_node(
                        node,
                        device,
                        context.surface.width() as f32,
                        context.surface.height() as f32,
                        device.density,
                        device.alpha,
                        context.font_color.to_int() as i32,
                        device.ppi,
                        context.direction(),
                    ) {
                        let mut children = node.children();
                        handle_render_children(svg.context_mut(), &mut children, root_element)
                    }
                }
                ElementName::Rect => {
                    let mut rect = Rect::parse_from(node);
                    rect.update_bounding_box(width, height, device);
                    update_bounding_box(rect.bounding_box());
                    handle_extra_style(rect.style_mut());
                    rect.render(context, node, root_element)
                }
                ElementName::Circle => {
                    let mut circle = Circle::parse_from(node);
                    circle.update_bounding_box(width, height, device);
                    update_bounding_box(circle.bounding_box());
                    handle_extra_style(circle.style_mut());
                    circle.render(context, node, root_element)
                }
                ElementName::Ellipse => {
                    let mut ellipse = Ellipse::parse_from(node);
                    ellipse.update_bounding_box(width, height, device);
                    update_bounding_box(ellipse.bounding_box());
                    handle_extra_style(ellipse.style_mut());
                    ellipse.render(context, node, root_element)
                }
                ElementName::Line => {
                    let mut line = Line::parse_from(node);
                    line.update_bounding_box(width, height, device);
                    update_bounding_box(line.bounding_box());
                    handle_extra_style(line.style_mut());
                    line.render(context, node, root_element)
                }
                ElementName::Polygon => {
                    let mut polygon = PolyShape::parse_from(node);
                    polygon.update_bounding_box(width, height, device);
                    update_bounding_box(polygon.bounding_box());
                    handle_extra_style(polygon.style_mut());
                    polygon.render(context, node, root_element)
                }
                ElementName::Polyline => {
                    let mut polygon = PolyShape::parse_from(node);
                    polygon.update_bounding_box(width, height, device);
                    update_bounding_box(polygon.bounding_box());
                    handle_extra_style(polygon.style_mut());
                    polygon.render(context, node, root_element)
                }
                ElementName::Path => {
                    let mut path = Path::parse_from(node);
                    path.update_bounding_box(width, height, device);
                    update_bounding_box(path.bounding_box());
                    handle_extra_style(path.style_mut());
                    path.render(context, node, root_element);
                }
                ElementName::Text => {
                    let mut text = Text::parse_from(node);
                    text.update_bounding_box(
                        width,
                        height,
                        device,
                        context.state.font.get_font(device).size(),
                        context.measure_text(text.text()).width,
                    );
                    update_bounding_box(text.bounding_box());
                    handle_extra_style(text.style_mut());
                    text.render(context, node, root_element)
                }
                ElementName::G => {
                    let mut g = G::parse_from(node);
                    handle_extra_style(g.style_mut());
                    g.render(context, node, root_element)
                }
                ElementName::Defs => {}
                ElementName::Use => {
                    let mut use_element = Use::parse_from(node);
                    use_element.render(context, node, root_element)
                }
                ElementName::Image => {
                    let mut image = Image::parse_from(node);
                    image.update_bounding_box(width, height, device);
                    handle_extra_style(image.style_mut());
                    image.render(context, node, root_element)
                }
                _ => {}
            },
        }
    }
}

pub fn handle_render_children(context: &mut Context, children: &mut Children, root_element: Node) {
    handle_render_children_with_bounding_box(context, children, root_element, None, None)
}

pub fn handle_render_children_with_bounding_box(
    context: &mut Context,
    children: &mut Children,
    root_element: Node,
    bounding_box: Option<&mut BoundingBox>,
    extra_style: Option<&StyleMap>,
) {
    let mut default = BoundingBox::default();
    let bb = bounding_box.unwrap_or(&mut default);
    for node in children.into_iter() {
        handle_render_child(context, node, root_element, Some(bb), extra_style)
    }
}

fn prepare_focal(cx: f32, cy: f32, r: f32, fx: f32, fy: f32) -> (f32, f32) {
    let max_r = r - r * 0.001;

    let mut line = Line::new(cx, cy, fx, fy);

    if line.length() > max_r {
        line.set_length(max_r);
    }

    (line.x2, line.y2)
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Line {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl Line {
    /// Creates a new line.
    #[inline]
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Line {
        Line { x1, y1, x2, y2 }
    }

    /// Calculates the line length.
    #[inline]
    pub fn length(&self) -> f32 {
        let x = self.x2 - self.x1;
        let y = self.y2 - self.y1;
        (x * x + y * y).sqrt()
    }

    /// Sets the line length.
    pub fn set_length(&mut self, len: f32) {
        let x = self.x2 - self.x1;
        let y = self.y2 - self.y1;
        let len2 = (x * x + y * y).sqrt();
        let line = Line {
            x1: self.x1,
            y1: self.y1,
            x2: self.x1 + x / len2,
            y2: self.y1 + y / len2,
        };

        self.x2 = self.x1 + (line.x2 - line.x1) * len;
        self.y2 = self.y1 + (line.y2 - line.y1) * len;
    }
}
