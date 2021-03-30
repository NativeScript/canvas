use roxmltree::{Document, Node};
use skia_safe::{
    AlphaType, Color, ColorType, ImageFilter, ImageInfo, ISize, Paint, Point, Rect, Surface, Vector,
};
use skia_safe::paint::Style;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::cmp::max;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::f32::consts::PI;
use std::io::{Read, Seek, SeekFrom};
use std::iter::FromIterator;
use std::num::ParseFloatError;
use std::ops::{Index, IndexMut};
use std::rc::Rc;

use crate::common::context::{Context, Device, State};
use crate::common::context::drawing_paths::fill_rule::FillRule;
use crate::common::context::drawing_text::typography::parse_font;
use crate::common::context::fill_and_stroke_styles::gradient::Gradient;
use crate::common::context::fill_and_stroke_styles::paint::PaintStyle;
use crate::common::context::fill_and_stroke_styles::pattern::Pattern;
use crate::common::context::line_styles::line_cap::LineCap;
use crate::common::context::line_styles::line_join::LineJoin;
use crate::common::context::matrix::Matrix;
use crate::common::context::paths::path::Path;
use crate::common::context::text_styles::text_direction::TextDirection;
use crate::common::svg::clip_shape::{Clip, ClipShape};
use crate::common::svg::Visibility::Visible;
use crate::common::to_data_url;
use crate::common::utils::color::parse_color;

mod attribute_names;
mod bounding_box;
mod circle;
mod clip_shape;
mod elements;
mod ellipse;
mod enums;
mod error;
mod filters;
mod g;
mod gradient;
mod image;
mod line;
mod path;
mod pattern;
mod polygon;
mod prelude;
mod rect;
mod symbol;
mod text;
mod units;
mod use_;
mod view_box;

const SVG_NS: &str = "svg";
const XLINK_HREF_NS: &str = "http://www.w3.org/1999/xlink";
const RECT_NS: &str = "rect";
const CIRCLE_NS: &str = "circle";
const ELLIPSE_NS: &str = "ellipse";
const LINE_NS: &str = "line";
const POLYGON_NS: &str = "polygon";
const POLYLINE_NS: &str = "polyline";
const PATH_NS: &str = "path";
const TEXT_NS: &str = "text";
const G_NS: &str = "g";
const DEFS_NS: &str = "defs";
const USE_NS: &str = "use";
const IMAGE_NS: &str = "image";
const LINEAR_GRADIENT_NS: &str = "linearGradient";
const RADIAL_GRADIENT_NS: &str = "radialGradient";
const STOP_NS: &str = "stop";
const PATTERN_NS: &str = "pattern";
const SYMBOL_NS: &str = "symbol";
const FILTER_NS: &str = "filter";
const FE_GAUSSIAN_BLUR_NS: &str = "FeGaussianBlur";
const STD_DEVIATION_ATTR: &str = "StdDeviation";
const X_ATTR: &str = "x";
const Y_ATTR: &str = "y";
const X1_ATTR: &str = "x1";
const Y1_ATTR: &str = "y1";
const X2_ATTR: &str = "x2";
const Y2_ATTR: &str = "y2";
const CX_ATTR: &str = "cx";
const CY_ATTR: &str = "cy";
const FR_ATTR: &str = "fr";
const FX_ATTR: &str = "fx";
const FY_ATTR: &str = "fy";
const SPREAD_METHOD_ATTR: &str = "spreadMethod";
const R_ATTR: &str = "r";
const RX_ATTR: &str = "rx";
const RY_ATTR: &str = "ry";
const WIDTH_ATTR: &str = "width";
const HEIGHT_ATTR: &str = "height";
const FILL_ATTR: &str = "fill";
const FILL_RULE_ATTR: &str = "fill-rule";
const FILL_OPACITY_ATTR: &str = "fill-opacity";
const STROKE_ATTR: &str = "stroke";
const STROKE_DASH_ARRAY_ATTR: &str = "stroke-dasharray";
const STROKE_WIDTH_ATTR: &str = "stroke-width";
const STROKE_OPACITY_ATTR: &str = "stroke-opacity";
const STROKE_DASH_OFFSET_ATTR: &str = "stroke-dashoffset";
const STROKE_LINE_CAP_ATTR: &str = "stroke-linecap";
const STROKE_LINE_JOIN_ATTR: &str = "stroke-linejoin";
const STROKE_MITER_LIMIT_ATTR: &str = "stroke-miterlimit";
const STYLE_ATTR: &str = "style";
const POINTS_ATTR: &str = "points";
const D_ATTR: &str = "d";
const OFFSET_ATTR: &str = "offset";
const STOP_COLOR_ATTR: &str = "stop-color";
const STOP_OPACITY_ATTR: &str = "stop-opacity";
const GRADIENT_TRANSFORM_ATTR: &str = "gradientTransform";
const DISPLAY_ATTR: &str = "display";
const VISIBILITY_ATTR: &str = "visibility";
const GRADIENT_UNITS_ATTR: &str = "gradientUnits";
const CLIP_PATH_UNITS_ATTR: &str = "clipPathUnits";

const BEFORE_FILL_EVENT: &str = "before_fill";
const AFTER_FILL_EVENT: &str = "after_fill";
const BEFORE_STROKE_EVENT: &str = "before_stroke";
const AFTER_STROKE_EVENT: &str = "after_stroke";

const TRANSFORM_ATTR: &str = "transform";
const CLIP_PATH_ATTR: &str = "clip-path";

const HREF_ATTR: &str = "href";
const OPACITY_ATTR: &str = "opacity";

const VIEW_BOX_ATTR: &str = "viewBox";
const PATTERN_TRANSFORM_ATTR: &str = "patternTransform";

const FONT_FAMILY_ATTR: &str = "font-family";
const FONT_SIZE_ATTR: &str = "font-size";
const FONT_STRETCH_ATTR: &str = "font-stretch";
const FONT_STYLE_ATTR: &str = "font-style";
const FONT_VARIANT_ATTR: &str = "font-variant";
const FONT_WEIGHT_ATTR: &str = "font-weight";

#[derive(Debug, Copy, Clone)]
pub struct Shape {}

#[derive(Debug, Copy, Clone)]
pub struct ViewBox {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    is_valid: bool,
}

impl Default for ViewBox {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            is_valid: true,
        }
    }
}

impl ViewBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            is_valid: true,
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}

impl From<&str> for ViewBox {
    fn from(val: &str) -> Self {
        let val = val.replace(",", " ");
        let mut vb = ViewBox::from_iter(val.split(" "));
        if val.is_empty() {
            vb.is_valid = false;
        }
        vb
    }
}

impl<'a> FromIterator<&'a str> for ViewBox {
    fn from_iter<T: IntoIterator<Item=&'a str>>(iter: T) -> Self {
        let mut value = ViewBox::default();
        let mut position = 0;
        for i in iter {
            match i.parse::<f32>() {
                Ok(val) => match position {
                    0 => value.x = val,
                    1 => value.y = val,
                    2 => value.width = val,
                    3 => value.height = val,
                    _ => {}
                },
                Err(_) => {
                    value.is_valid = false;
                }
            };
            position += 1;
        }
        value
    }
}

#[derive(PartialEq)]
enum DrawType {
    FillStroke,
    Stroke,
    Fill,
    Text,
    Image,
}

pub struct FillPaint<'a> {
    user_set_color: bool,
    fill_color: &'a str,
    fill_opacity: f32,
    fill_rule: &'a str,
    fill_pattern: Option<Pattern>,
    fill_gradient: Option<Gradient>,
}

impl<'a> Default for FillPaint<'a> {
    fn default() -> Self {
        Self {
            user_set_color: false,
            fill_color: "black",
            fill_opacity: 1.0,
            fill_rule: "nonzero",
            fill_pattern: None,
            fill_gradient: None,
        }
    }
}

impl<'a> FillPaint<'a> {
    pub fn set_color(&mut self, color: &'a str) {
        self.user_set_color = true;
        self.fill_color = color;
    }
}

pub struct StrokePaint<'a> {
    user_set_color: bool,
    stroke_width: f32,
    stroke_color: &'a str,
    stroke_opacity: f32,
    stroke_dasharray: Option<Vec<f32>>,
    stroke_dashoffset: f32,
    stroke_linecap: &'a str,
    stroke_linejoin: &'a str,
    stroke_miterlimit: f32,
    stroke_pattern: Option<Pattern>,
    stroke_gradient: Option<Gradient>,
}

impl<'a> StrokePaint<'a> {
    pub fn set_color(&mut self, color: &'a str) {
        self.user_set_color = true;
        self.stroke_color = color;
    }
}

impl<'a> Default for StrokePaint<'a> {
    fn default() -> Self {
        Self {
            user_set_color: false,
            stroke_width: 1.0,
            stroke_color: "none",
            stroke_opacity: 1.0,
            stroke_dasharray: None,
            stroke_dashoffset: 0.0,
            stroke_linecap: "butt",
            stroke_linejoin: "round",
            stroke_miterlimit: 4.0,
            stroke_pattern: None,
            stroke_gradient: None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Display {
    None,
    Inline,
    Inherit,
}

impl Default for Display {
    fn default() -> Self {
        Self::Inline
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Visible
    }
}

pub struct ParsedStyle<'a> {
    parent: Option<Rc<RefCell<ParsedStyle<'a>>>>,
    fill_paint: FillPaint<'a>,
    stroke_paint: StrokePaint<'a>,
    transform: Option<HashMap<&'a str, String>>,
    clip_path: Clip<'a>,
    opacity: Option<f32>,
    stop_color: Option<&'a str>,
    stop_opacity: Option<f32>,
    filter: String,
    display: Display,
    visibility: Visibility,
}

impl<'a> Default for ParsedStyle<'a> {
    fn default() -> Self {
        Self {
            stroke_paint: StrokePaint::default(),
            fill_paint: FillPaint::default(),
            transform: None,
            clip_path: Clip::default(),
            opacity: None,
            stop_color: None,
            stop_opacity: None,
            parent: None,
            filter: String::new(),
            display: Display::default(),
            visibility: Visibility::default(),
        }
    }
}

pub(crate) fn draw_svg_from_path(context: &mut Context, path: &str) {
    let file = std::fs::File::open(path);
    match file {
        Ok(file) => {
            let mut reader = std::io::BufReader::new(file);
            let mut bytes = [0; 16];
            let result = reader.read(&mut bytes);
            match result {
                Ok(_) => {
                    let _ = reader.seek(SeekFrom::Start(0));
                    // TODO check bytes to verify it's an svg

                    let mut svg = Vec::new();
                    match reader.read_to_end(&mut svg) {
                        Ok(_) => {
                            let svg = std::string::String::from_utf8_lossy(&svg);
                            draw_svg(context, &svg);
                        }
                        Err(e) => {
                            println!("svg read to string error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("svg file read error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("svg file open error: {}", e);
        }
    }
}

fn handle_svg_document(context: &mut Context, svg: &roxmltree::Node, root: &roxmltree::Document) {
    let mut did_set_vb = false;
    let mut scale = context.device.density;
    let tag_name = svg.tag_name().name();
    if tag_name == DEFS_NS {
        return;
    }
    if svg.is_element() && !tag_name.is_empty() && tag_name == "svg" {
        let mut x = "0";
        let mut y = "0";
        let mut width = "auto";
        let mut height = "auto";
        if let Some(val) = svg.attribute(X_ATTR) {
            x = val;
        }
        if let Some(val) = svg.attribute(Y_ATTR) {
            y = val;
        }
        if let Some(w) = svg.attribute(WIDTH_ATTR) {
            width = w;
        }
        if let Some(h) = svg.attribute(HEIGHT_ATTR) {
            height = h;
        }

        let ctx_width = context.surface.width();
        let ctx_height = context.surface.height();
        let x = parse_length(x, ctx_width, scale);
        let y = parse_length(y, ctx_height, scale);
        let mut width = parse_length(width, ctx_width, scale);
        let mut height = parse_length(height, ctx_height, scale);
        let mut view_box = ViewBox::from(svg.attribute(VIEW_BOX_ATTR).unwrap_or(""));
        if width > 0.0 && height > 0.0 {
            //  let scale = 1.0;
            let mut ctx = create_context(
                width,
                height,
                scale,
                true,
                0,
                context.device.ppi,
                context.direction(),
            );
            //  ctx.device.density = 1.0;
            // https://svgwg.org/svg2-draft/coords.html#ComputingAViewportsTransform
            if view_box.is_valid() {
                let mut scale_x = width / view_box.width;
                let mut scale_y = height / view_box.height;
                let mut matrix = ctx.get_transform();
                matrix.pre_scale((scale_x, scale_y), None);
                let mut translate_x = x - (view_box.x * scale_x);
                let mut translate_y = y - (view_box.y * scale_y);
                matrix.post_translate(Vector::new(translate_x, translate_y));
                ctx.set_transform_matrix(&matrix);
            }
            // ctx.surface.canvas().scale((context.device.density, context.device.density));
            for node in svg.children().into_iter() {
                match node.tag_name().name() {
                    SVG_NS => handle_svg_document(context, &node, node.document()),
                    _ => handle_node(
                        &mut ctx,
                        node,
                        root,
                        Rc::new(RefCell::new(ParsedStyle::default())),
                        None,
                    ),
                }
            }
            let width = context.surface.width() as f32;
            let height = context.surface.height() as f32;
            let snapshot = ctx.surface.image_snapshot();
            let mut paint = skia_safe::Paint::default();
            paint.set_anti_alias(true);
            paint.set_alpha_f(context.global_alpha());
            let canvas = context.surface.canvas();
            let snapshot_width = ctx.surface.width() as f32;
            let snapshot_height = ctx.surface.height() as f32;
            let src = skia_safe::Rect::from_xywh(0f32, 0f32, snapshot_width, snapshot_height);
            //canvas.scale((context.device.density, context.device.density));
            // canvas.draw_image_rect(
            //     snapshot, Some((&src, skia_safe::canvas::SrcRectConstraint::Strict)),
            //     skia_safe::Rect::from_xywh(0.0, 0.0, width, height),
            //     &paint,
            // );

            canvas.draw_image_rect(
                snapshot,
                Some((&src, skia_safe::canvas::SrcRectConstraint::Strict)),
                src,
                &paint,
            );
        }
    }
}

pub(crate) fn draw_svg(context: &mut Context, svg: &str) {
    let mut svg = String::from(svg);
    if !svg.contains(r#"xmlns:xlink="http://www.w3.org/1999/xlink""#) {
        svg = svg.replace(
            "<svg ",
            "<svg xmlns:xlink=\"http://www.w3.org/1999/xlink\" ",
        );
    }
    match roxmltree::Document::parse(&svg) {
        Ok(ref root) => {
            for node in root.root().children().into_iter() {
                let tag_name = node.tag_name().name();
                if tag_name == DEFS_NS {
                    return;
                }
                if node.is_element() && !tag_name.is_empty() && tag_name == "svg" {
                    handle_svg_document(context, &node, root)
                }
            }
        }
        Err(e) => {
            println!("Document parser error: {}", e)
        }
    }
}

pub fn create_context(
    width: f32,
    height: f32,
    density: f32,
    alpha: bool,
    font_color: i32,
    ppi: f32,
    direction: TextDirection,
) -> Context {
    // let density = 1.0;
    let mut device = Device {
        width,
        height,
        density,
        non_gpu: true,
        samples: 0,
        alpha,
        ppi,
    };
    Context {
        surface: Surface::new_raster_n32_premul(ISize::new(width as i32, height as i32)).unwrap(),
        path: Path::default(),
        state: State::from_device(device, direction),
        state_stack: vec![],
        font_color: Color::new(font_color as u32),
        device,
    }
}

fn handle_node<'a>(
    context: &mut Context,
    node: Node<'a, '_>,
    root: &'a roxmltree::Document<'a>,
    style: Rc<RefCell<ParsedStyle<'a>>>,
    parent_attributes: Option<HashMap<&str, &str>>,
) {
    if node.is_element() {
        let mut child_style = parse_style(context, &node, root);
        child_style.parent = Some(Rc::clone(&style));
        let child_style = Rc::new(RefCell::new(child_style));
        match node.tag_name().name() {
            DEFS_NS => { /*to ignore*/ }
            RECT_NS => rect::handle_rect(context, node, child_style, parent_attributes, root),
            CIRCLE_NS => circle::handle_circle(context, node, child_style, parent_attributes, root),
            ELLIPSE_NS => {
                ellipse::handle_ellipse(context, node, child_style, parent_attributes, root)
            }
            LINE_NS => line::handle_line(context, node, child_style, parent_attributes, root),
            POLYGON_NS => {
                polygon::handle_polygon(context, node, false, child_style, parent_attributes, root)
            }
            POLYLINE_NS => {
                polygon::handle_polygon(context, node, true, child_style, parent_attributes, root)
            }
            PATH_NS => path::handle_path(context, node, child_style, parent_attributes, root),
            TEXT_NS => text::handle_text(context, node, child_style, parent_attributes, root),
            G_NS => g::handle_g(context, node, root, child_style, parent_attributes),
            USE_NS => use_::handle_use(context, node, root, child_style, None),
            IMAGE_NS => image::handle_image(context, node, root, child_style, parent_attributes),
            _ => {}
        }
    }
}

fn handle_transform(
    transform: &HashMap<&str, String>,
    matrix: &mut skia_safe::Matrix,
    parent: i32,
    scale: f32,
) {
    for key_value in transform.into_iter() {
        match *key_value.0 {
            "rotate" => {
                let value = key_value.1;
                let values: Vec<_>;
                if value.contains(",") {
                    values = key_value.1.split(",").collect();
                } else {
                    values = key_value.1.split(" ").collect();
                }
                let slice = values.as_slice();
                if slice.len() == 1 {
                    if let Ok(value) = slice[0].parse::<f32>() {
                        matrix.pre_rotate(value, None);
                    }
                } else if values.len() == 3 {
                    if let Some(points) = Some((slice[0], slice[1], slice[2])) {
                        if let (Ok(value), Ok(x), Ok(y)) = (
                            points.0.parse::<f32>(),
                            points.1.parse::<f32>(),
                            points.2.parse::<f32>(),
                        ) {
                            matrix.pre_rotate(value, Vector::new(x, y));
                        }
                    }
                }
            }
            "translateX" => {
                if key_value.1.contains("px") {
                    if let Ok(x) = key_value.1.replace("px", "").parse::<f32>() {
                        matrix.set_translate_x((x * scale));
                    }
                } else {
                    if let Ok(x) = key_value.1.parse::<f32>() {
                        matrix.set_translate_x((x));
                    }
                }
            }
            "translateY" => {
                if key_value.1.contains("px") {
                    if let Ok(y) = key_value.1.replace("px", "").parse::<f32>() {
                        matrix.set_translate_y(y * scale);
                    }
                } else {
                    if let Ok(y) = key_value.1.parse::<f32>() {
                        matrix.set_translate_y(y);
                    }
                }
            }
            "translate" => {
                let value = key_value.1;
                let values: Vec<_>;
                if value.contains(",") {
                    values = key_value.1.split(",").collect();
                } else {
                    values = key_value.1.split(" ").collect();
                }
                let slice = values.as_slice();
                if slice.len() == 1 {
                    if let Ok(point) = slice[0].parse::<f32>() {
                        matrix.pre_translate((point * scale, 0f32));
                    }
                } else if slice.len() == 2 {
                    if let Some(points) = Some((slice[0], slice[1])) {
                        match (points.0.parse::<f32>(), points.1.parse::<f32>()) {
                            (Ok(x), Ok(y)) => {
                                matrix.pre_translate((x * scale, y * scale));
                            }
                            (_, _) => {}
                        }
                    }
                }
            }
            "scale" => {
                let value = key_value.1;
                let values: Vec<_>;
                if value.contains(",") {
                    values = key_value.1.split(",").collect();
                } else {
                    values = key_value.1.split(" ").collect();
                }
                let slice = values.as_slice();
                if slice.len() == 1 {
                    if let Ok(value) = slice[0].parse::<f32>() {
                        matrix.pre_scale((value, value), None);
                    }
                } else if slice.len() == 2 {
                    if let Some(points) = Some((slice[0], slice[1])) {
                        match (points.0.parse::<f32>(), points.1.parse::<f32>()) {
                            (Ok(x), Ok(y)) => {
                                matrix.pre_scale((x, y), None);
                            }
                            (_, _) => {}
                        }
                    }
                }
            }
            "skewX" => {
                if let Ok(x) = key_value.1.parse::<f32>() {
                    matrix.set_skew_x(x);
                }
            }
            "skewY" => {
                if let Ok(y) = key_value.1.parse::<f32>() {
                    matrix.set_skew_y(y);
                }
            }
            "matrix" => {
                let value = key_value.1;
                let values: Vec<_>;
                if value.contains(",") {
                    values = key_value.1.split(",").collect();
                } else {
                    values = key_value.1.split(" ").collect();
                }
                let mut has_error = false;
                let values: Vec<_> = values
                    .into_iter()
                    .map(|value| {
                        return if let Ok(value) = value.parse::<f32>() {
                            value
                        } else {
                            has_error = true;
                            -1f32
                        };
                    })
                    .collect();

                if !has_error {
                    let affine = [
                        values.as_slice()[0],
                        values.as_slice()[1],
                        values.as_slice()[2],
                        values.as_slice()[3],
                        values.as_slice()[4],
                        values.as_slice()[5],
                    ];
                    let mut transform = skia_safe::Matrix::from_affine(&affine);
                    matrix.pre_concat(&transform);
                }
            }
            _ => {}
        }
    }
}

fn handle_drawing<'a>(
    context: &mut Context,
    mut style: Rc<RefCell<ParsedStyle>>,
    path: Option<Path>,
    draw_type: DrawType,
    callback: Option<Box<dyn Fn(&str, &mut Context)>>,
    root: &'a roxmltree::Document<'a>,
    node_view_box: &ViewBox,
) {
    let style = (*style).borrow();
    if style.visibility == Visibility::Hidden
        || style.visibility == Visibility::Collapse
        || style.display == Display::None
    {
        return;
    }
    if let Some(transform) = style.transform.as_ref() {
        let mut matrix = context.get_transform();
        handle_transform(
            transform,
            &mut matrix,
            context.surface.width(),
            context.device.density,
        );
        context.set_transform_matrix(&matrix);
    }
    let scale = context.device.density;
    let fill_opacity = style.fill_paint.fill_opacity;
    let stroke_opacity = style.stroke_paint.stroke_opacity;
    let file_rule = match style.fill_paint.fill_rule {
        "evenodd" => FillRule::EvenOdd,
        _ => FillRule::NonZero,
    };

    let mut fill_color = "";
    if let Some(mut parent) = style.parent.as_ref() {
        let parent = &(**parent);
        let parent = parent.borrow();
        if parent.fill_paint.user_set_color {
            fill_color = parent.fill_paint.fill_color;
        }
    }

    if style.fill_paint.user_set_color {
        fill_color = style.fill_paint.fill_color;
    }
    if fill_color.is_empty() {
        fill_color = "black";
    }

    let mut stroke_color = "none";
    if let Some(mut parent) = style.parent.as_ref() {
        let parent = &(**parent).borrow();
        if parent.stroke_paint.user_set_color {
            stroke_color = parent.stroke_paint.stroke_color;
        }
    }
    if style.stroke_paint.user_set_color {
        stroke_color = style.stroke_paint.stroke_color;
    }

    let mut clip_path = Clip::default();
    if let Some(mut parent) = style.parent.as_ref() {
        let parent = &(**parent).borrow();
        if !parent.clip_path.is_empty() {
            clip_path = parent.clip_path.clone()
        }
    }

    if !style.clip_path.is_empty() {
        clip_path = style.clip_path.clone()
    }

    if !clip_path.is_empty() {
        let mut path = Path::new();
        use crate::common::svg::length::convert_length;
        let device = context.device;
        let device = &device;
        let units = clip_path.units();
        let mut view_box = ViewBox::new(0.0, 0.0, 0.0, 0.0);
        let mut root_node: Option<Node> = None;
        let width = context.surface.width();
        let height = context.surface.height();
        for desc in root.descendants() {
            if desc.tag_name().name() == SVG_NS {
                root_node = Some(desc);
                break;
            }
        }

        if let Some(node) = root_node {
            if let Some(value) = node.attribute(VIEW_BOX_ATTR) {
                view_box = ViewBox::from(value);
                if !view_box.is_valid() {
                    view_box = ViewBox::new(0.0, 0.0, width as f32, height as f32);
                }
            } else {
                view_box = ViewBox::new(0.0, 0.0, width as f32, height as f32);
            }
        } else {
            view_box = ViewBox::new(0.0, 0.0, width as f32, height as f32);
        }

        for clip_path in clip_path.clips() {
            match clip_path {
                ClipShape::Circle { cx, cy, r } => {
                    let cx = convert_length(
                        *cx,
                        attribute_names::Attribute::Cx,
                        units,
                        device,
                        &view_box,
                    );
                    let cy = convert_length(
                        *cy,
                        attribute_names::Attribute::Cy,
                        units,
                        device,
                        &view_box,
                    );
                    let r =
                        convert_length(*r, attribute_names::Attribute::R, units, device, &view_box);
                    path.arc(cx, cy, r, 0f32, 2f32 * PI, false);
                }
                ClipShape::Ellipse { cx, cy, rx, ry } => {
                    let cx = convert_length(
                        *cx,
                        attribute_names::Attribute::Cx,
                        units,
                        device,
                        &view_box,
                    );
                    let cy = convert_length(
                        *cy,
                        attribute_names::Attribute::Cy,
                        units,
                        device,
                        &view_box,
                    );
                    let rx = convert_length(
                        *rx,
                        attribute_names::Attribute::Rx,
                        units,
                        device,
                        &view_box,
                    );
                    let ry = convert_length(
                        *ry,
                        attribute_names::Attribute::Ry,
                        units,
                        device,
                        &view_box,
                    );
                    path.ellipse(cx, cy, rx, ry, 0f32, 0f32, 2f32 * PI, false);
                }
                ClipShape::Line { x1, y1, x2, y2 } => {
                    let x1 = convert_length(
                        *x1,
                        attribute_names::Attribute::X1,
                        units,
                        device,
                        &view_box,
                    );
                    let y1 = convert_length(
                        *y1,
                        attribute_names::Attribute::Y1,
                        units,
                        device,
                        &view_box,
                    );
                    let x2 = convert_length(
                        *x2,
                        attribute_names::Attribute::X2,
                        units,
                        device,
                        &view_box,
                    );
                    let y2 = convert_length(
                        *y2,
                        attribute_names::Attribute::Y2,
                        units,
                        device,
                        &view_box,
                    );
                    path.move_to(x1, y1);
                    path.line_to(x2, y2);
                }
                ClipShape::Polygon { points } => {
                    let points: Vec<&str> = points.split(" ").collect();
                    let points: Vec<_> = points
                        .into_iter()
                        .map(|value| {
                            let points: Vec<_> = value.split(",").collect();
                            let points = points.into_iter().fold(Vec::new(), |mut acc, value| {
                                if let Ok(value) = value.parse::<f32>() {
                                    acc.push(value);
                                }
                                acc
                            });
                            points
                        })
                        .filter(|value| value.len() == 2)
                        .collect();
                    if !points.is_empty() {
                        for (index, value) in points.into_iter().enumerate() {
                            if index == 0 {
                                &mut path.move_to(*value.first().unwrap(), *value.last().unwrap());
                            } else {
                                &mut path.line_to(*value.first().unwrap(), *value.last().unwrap());
                            }
                        }
                        // do I need to close this??
                        path.close_path();
                    }
                }
                ClipShape::Polyline { points } => {
                    let points: Vec<&str> = points.split(" ").collect();
                    let points: Vec<_> = points
                        .into_iter()
                        .map(|value| {
                            let points: Vec<_> = value.split(",").collect();
                            let points = points.into_iter().fold(Vec::new(), |mut acc, value| {
                                if let Ok(value) = value.parse::<f32>() {
                                    acc.push(value);
                                }
                                acc
                            });
                            points
                        })
                        .filter(|value| value.len() == 2)
                        .collect();
                    if !points.is_empty() {
                        for (index, value) in points.into_iter().enumerate() {
                            if index == 0 {
                                &mut path.move_to(*value.first().unwrap(), *value.last().unwrap());
                            } else {
                                &mut path.line_to(*value.first().unwrap(), *value.last().unwrap());
                            }
                        }
                    }
                }
                ClipShape::Rect {
                    x,
                    y,
                    width,
                    height,
                } => {
                    let x =
                        convert_length(*x, attribute_names::Attribute::X, units, device, &view_box);
                    let y =
                        convert_length(*y, attribute_names::Attribute::Y, units, device, &view_box);
                    let width = convert_length(
                        *width,
                        attribute_names::Attribute::Width,
                        units,
                        device,
                        &view_box,
                    );
                    let height = convert_length(
                        *height,
                        attribute_names::Attribute::Height,
                        units,
                        device,
                        &view_box,
                    );
                    path.rect(x, y, width, height);
                }
                ClipShape::Path { d } => {
                    let mut child_path = Path::from_str(d);
                    path.add_path(&child_path, None);
                }
                ClipShape::Text {
                    x,
                    y,
                    text,
                    font_stretch,
                    font_weight,
                    font_variant,
                    font_style,
                    font_size,
                    font_family,
                } => {
                    let x =
                        convert_length(*x, attribute_names::Attribute::X, units, device, &view_box);
                    let y =
                        convert_length(*y, attribute_names::Attribute::Y, units, device, &view_box);
                    let mut matrix = skia_safe::Matrix::default();
                    matrix.post_translate(Vector::new(x, y));
                    path.path.transform(&matrix);
                    let device = &context.device;
                    let font_str = format!(
                        "{} {} {} {} {}",
                        font_style, font_variant, font_weight, font_size, font_family
                    );
                    if let Ok(mut font) = parse_font(&font_str, device) {
                        let mut font = font.1;
                        let glyphs = font.str_to_glyphs_vec(text);
                        let mut points = vec![Point::default(); glyphs.capacity()];
                        font.get_pos(&glyphs, points.as_mut_slice(), None);
                        for (glyph, point) in glyphs.into_iter().zip(&points).into_iter() {
                            if let Some(text_path) = font.get_path(glyph) {
                                let mut matrix = skia_safe::Matrix::default();
                                matrix.post_translate(Vector::new(point.x, point.y));
                                path.path.add_path_matrix(&text_path, &matrix, None);
                            }
                        }
                    }
                }
                _ => {}
            }

            if !path.path.is_empty() {
                context.clip(Some(&mut path), Some(file_rule));
            }
        }
    }

    context.set_line_width(style.stroke_paint.stroke_width * scale);

    if fill_color.ne("none") {
        if fill_color.contains("url") {
            if let Some(gradient) = &style.fill_paint.fill_gradient {
                context.set_fill_style(PaintStyle::Gradient(gradient.clone()))
            } else if let Some(pattern) = &style.fill_paint.fill_pattern {
                context.set_fill_style(PaintStyle::Pattern(pattern.clone()))
            }
        } else {
            let fill_color = parse_color(&fill_color).unwrap_or(Color::BLACK);
            context.set_fill_style(PaintStyle::new_color_rgba(
                fill_color.r(),
                fill_color.g(),
                fill_color.b(),
                fill_color.a(),
            ))
        }
    }

    if stroke_color.ne("none") {
        if stroke_color.contains("url") {
            if let Some(gradient) = &style.stroke_paint.stroke_gradient {
                context.set_stroke_style(PaintStyle::Gradient(gradient.clone()))
            } else if let Some(pattern) = &style.stroke_paint.stroke_pattern {
                context.set_stroke_style(PaintStyle::Pattern(pattern.clone()))
            }
        } else if let Some(stroke_color) = parse_color(stroke_color) {
            context.set_stroke_style(PaintStyle::new_color_rgba(
                stroke_color.r(),
                stroke_color.g(),
                stroke_color.b(),
                stroke_color.a(),
            ))
        }
    }

    if let Some(mut parent) = style.parent.as_ref() {
        let parent = &(**parent).borrow();
        if !parent.filter.is_empty() {
            context.set_filter(&parent.filter);
        }
    }

    if !style.filter.is_empty() {
        context.set_filter(&style.filter);
    }
    context.set_miter_limit(style.stroke_paint.stroke_miterlimit * scale);

    context.set_line_cap(match style.stroke_paint.stroke_linecap {
        "round" => LineCap::CapRound,
        "square" => LineCap::CapSquare,
        _ => LineCap::CapButt,
    });

    context.set_line_join(match style.stroke_paint.stroke_linejoin {
        "bevel" => LineJoin::JoinBevel,
        "miter" => LineJoin::JoinMiter,
        _ => LineJoin::JoinRound,
    });

    if let Some(dash_array) = &style.stroke_paint.stroke_dasharray {
        context.set_line_dash(dash_array)
    }

    context.set_line_dash_offset(style.stroke_paint.stroke_dashoffset);

    let mut path = path;

    let current_global_alpha = context.global_alpha();

    context
        .state
        .paint
        .stroke_paint_mut()
        .set_alpha_f(stroke_opacity);

    context
        .state
        .paint
        .fill_paint_mut()
        .set_alpha_f(fill_opacity);

    let mut opacity = 1.0;
    if let Some(mut parent) = style.parent.as_ref() {
        let parent = &(**parent).borrow();
        if let Some(value) = parent.opacity {
            context.state.paint.image_paint_mut().set_alpha_f(value);
            opacity = value;
        }
    }

    if let Some(value) = style.opacity {
        context.state.paint.image_paint_mut().set_alpha_f(value);
        opacity = value;
    }

    if opacity < 1.0 {
        context
            .surface
            .canvas()
            .save_layer_alpha(None, (opacity * 255.0) as u32);
    }

    if let Some(callback) = &callback {
        callback(BEFORE_FILL_EVENT, context);
    }

    if (draw_type == DrawType::Fill || draw_type == DrawType::FillStroke) && fill_color.ne("none") {
        if let Some(local_path) = path {
            let mut path_copy = local_path.clone();
            context.fill(Some(&mut path_copy), file_rule);
            path = Some(local_path);
        } else {
            context.fill(None, file_rule);
        }
    }

    if let Some(callback) = &callback {
        callback(AFTER_FILL_EVENT, context);
    }

    if let Some(callback) = &callback {
        callback(BEFORE_STROKE_EVENT, context);
    }

    if draw_type == DrawType::Stroke || draw_type == DrawType::FillStroke {
        if stroke_color.ne("none") {
            if let Some(mut local_path) = path {
                let mut path_copy = local_path.clone();
                context.stroke(Some(&mut path_copy));
                path = Some(local_path);
            } else {
                context.stroke(None);
            }
        }
    }
    if let Some(callback) = &callback {
        callback(AFTER_STROKE_EVENT, context);
    }
    if opacity < 1.0 {
        context.surface.canvas().restore();
    }
    context.set_global_alpha(current_global_alpha);
    context.reset_state();
}

fn convert_length_min(value: &str, parent: i32, scale: f32, min: f32) -> f32 {
    let mut val = 0f32;
    if value.eq("auto") {
        val = parent as f32;
    } else if value.contains("%") {
        val = value.replace("%", "").parse::<f32>().unwrap_or(0f32) / 100f32;
    } else if value.contains("px") {
        val = value.replace("px", "").parse::<f32>().unwrap_or(0f32);
    } else if value.contains("pt") {
        val = value.replace("pt", "").parse::<f32>().unwrap_or(0f32) * (1.0 / 72.0);
    } else if value.contains("mm") {
        val = (value.replace("mm", "").parse::<f32>().unwrap_or(0.0) * 3.7795275591);
    } else if value.contains(".") {
        if let Ok(value) = value.parse::<f32>() {
            val = value;
        }
    } else if let Ok(value) = value.parse::<f32>() {
        val = value;
    }
    if val == 0.0 {
        return min;
    }
    val
}

fn parse_length_min(value: &str, parent: i32, scale: f32, min: f32) -> f32 {
    let mut val = 0f32;
    if value.eq("auto") {
        val = parent as f32;
    } else if value.contains("%") {
        val = parent as f32 * value.replace("%", "").parse::<f32>().unwrap_or(0f32) / 100f32;
    } else if value.contains("px") {
        val = value.replace("px", "").parse::<f32>().unwrap_or(0f32) * scale;
    } else if value.contains("pt") {
        val = value.replace("pt", "").parse::<f32>().unwrap_or(0f32) * (96.0 / 72.0) * scale;
    } else if value.contains("mm") {
        val = (value.replace("mm", "").parse::<f32>().unwrap_or(0.0) * 3.7795275591) * scale;
    } else if value.contains(".") {
        if let Ok(value) = value.parse::<f32>() {
            val = value * scale;
        }
    } else if let Ok(value) = value.parse::<f32>() {
        val = value * scale
    }
    if val == 0.0 {
        return min * scale;
    }
    val
}

fn parse_length(value: &str, parent: i32, scale: f32) -> f32 {
    parse_length_min(value, parent, scale, 0.0)
}

fn get_real_node<'a>(
    value: &str,
    root: &'a roxmltree::Document<'a>,
    mut callback: impl FnMut(Node<'a, 'a>, bool),
) {
    if value.contains("url") || value.starts_with("#") {
        let id = value
            .replace("url('#", "")
            .replace("')", "")
            .replace("url(#", "")
            .replace(")", "")
            .replace("#", "");
        if let Some(node) = root.descendants().find(|v| v.attribute("id") == Some(&id)) {
            let mut href: Option<&str>;
            match (
                node.attribute(HREF_ATTR),
                node.attribute((XLINK_HREF_NS, HREF_ATTR)),
            ) {
                (Some(href_), Some(xlink_href)) => href = Some(href_),
                (Some(href_), _) => href = Some(href_),
                (_, Some(xlink_href)) => href = Some(xlink_href),
                (_, _) => href = None,
            }
            callback(node, href.is_none());
            if !href.is_none() {
                get_real_node(href.unwrap(), root, callback)
            }
        }
    }
}

fn parse_style<'a>(
    context: &mut Context,
    node: &roxmltree::Node<'a, '_>,
    root: &'a roxmltree::Document<'a>,
) -> ParsedStyle<'a> {
    let mut parsed_style = ParsedStyle::default();
    let scale = context.device.density;

    if let Some(value) = node.attribute(DISPLAY_ATTR) {
        parse_field(DISPLAY_ATTR, value, &mut parsed_style, root, scale);
    }

    if let Some(value) = node.attribute(VISIBILITY_ATTR) {
        parse_field(VISIBILITY_ATTR, value, &mut parsed_style, root, scale);
    }

    if let Some(value) = node.attribute(STROKE_ATTR) {
        if value.contains("url") {
            let mut value = value;
            let slice: Vec<_> = value.split(" ").collect();
            if slice.len() > 1 {
                if let Some(val) = slice.into_iter().find(|v| v.contains("url")) {
                    value = val;
                }
            }

            let id = value
                .replace("url('#", "")
                .replace("')", "")
                .replace("url(#", "")
                .replace(")", "");
            if let Some(node) = root.descendants().find(|v| v.attribute("id") == Some(&id)) {
                match node.tag_name().name() {
                    LINEAR_GRADIENT_NS => {
                        let gradient =
                            gradient::linear::handle_linear_gradient(context, &node, root);
                        parsed_style.stroke_paint.stroke_gradient = Some(gradient);
                    }
                    RADIAL_GRADIENT_NS => {
                        let gradient =
                            gradient::radial::handle_radial_gradient(context, &node, root);
                        parsed_style.stroke_paint.stroke_gradient = Some(gradient);
                    }
                    PATTERN_NS => {
                        let pattern = pattern::handle_pattern(context, &node, root, None);
                        parsed_style.stroke_paint.stroke_pattern = pattern
                    }
                    _ => {}
                }
            }
        }
        parse_field(STROKE_ATTR, value, &mut parsed_style, root, scale);
    }

    if let Some(value) = node.attribute(FILL_ATTR) {
        if value.contains("url") {
            let mut value = value;
            let slice: Vec<_> = value.split(" ").collect();
            if slice.len() > 1 {
                if let Some(val) = slice.into_iter().find(|v| v.contains("url")) {
                    value = val;
                }
            }

            let id = value
                .replace("url('#", "")
                .replace("')", "")
                .replace("url(#", "")
                .replace(")", "");
            if let Some(node) = root.descendants().find(|v| v.attribute("id") == Some(&id)) {
                match node.tag_name().name() {
                    LINEAR_GRADIENT_NS => {
                        let gradient =
                            gradient::linear::handle_linear_gradient(context, &node, root);
                        parsed_style.fill_paint.fill_gradient = Some(gradient);
                    }
                    RADIAL_GRADIENT_NS => {
                        let gradient =
                            gradient::radial::handle_radial_gradient(context, &node, root);
                        parsed_style.fill_paint.fill_gradient = Some(gradient);
                    }
                    PATTERN_NS => {
                        let pattern = pattern::handle_pattern(context, &node, root, None);
                        parsed_style.fill_paint.fill_pattern = pattern
                    }
                    _ => {}
                }
            }
        }
        parse_field(FILL_ATTR, value, &mut parsed_style, root, scale);
    }

    if let Some(value) = node.attribute(FILTER_NS) {
        let id = value
            .replace("url('#", "")
            .replace("')", "")
            .replace("url(#", "")
            .replace(")", "");
        if let Some(node) = root.descendants().find(|v| v.attribute("id") == Some(&id)) {
            match node.tag_name().name() {
                FILTER_NS => {
                    parsed_style.filter = filters::handle_filters(context, &node, root);
                }
                _ => {}
            }
        }
    }

    if let Some(value) = node.attribute(STROKE_LINE_JOIN_ATTR) {
        parse_field(STROKE_LINE_JOIN_ATTR, value, &mut parsed_style, root, scale);
    }

    if let Some(value) = node.attribute(STROKE_LINE_CAP_ATTR) {
        parse_field(STROKE_LINE_CAP_ATTR, value, &mut parsed_style, root, scale);
    }

    if let Some(value) = node.attribute(STROKE_DASH_ARRAY_ATTR) {
        parse_field(
            STROKE_DASH_ARRAY_ATTR,
            value,
            &mut parsed_style,
            root,
            scale,
        );
    }

    if let Some(value) = node.attribute(STROKE_DASH_OFFSET_ATTR) {
        parse_field(
            STROKE_DASH_OFFSET_ATTR,
            value,
            &mut parsed_style,
            root,
            scale,
        );
    }

    if let Some(value) = node.attribute(STROKE_MITER_LIMIT_ATTR) {
        parse_field(
            STROKE_MITER_LIMIT_ATTR,
            value,
            &mut parsed_style,
            root,
            scale,
        );
    }

    if let Some(value) = node.attribute(STROKE_WIDTH_ATTR) {
        parse_field(STROKE_WIDTH_ATTR, value, &mut parsed_style, root, scale);
    }

    if let Some(value) = node.attribute(FILL_OPACITY_ATTR) {
        parse_field(FILL_OPACITY_ATTR, value, &mut parsed_style, root, scale);
    }

    if let Some(value) = node.attribute(STROKE_OPACITY_ATTR) {
        parse_field(STROKE_OPACITY_ATTR, value, &mut parsed_style, root, scale);
    }

    if let Some(value) = node.attribute(TRANSFORM_ATTR) {
        parse_field(TRANSFORM_ATTR, value, &mut parsed_style, root, scale);
    }

    if let Some(value) = node.attribute(CLIP_PATH_ATTR) {
        parse_field(CLIP_PATH_ATTR, value, &mut parsed_style, root, scale)
    }

    if let Some(value) = node.attribute(OPACITY_ATTR) {
        parse_field(OPACITY_ATTR, value, &mut parsed_style, root, scale);
    }

    if let Some(value) = node.attribute(FILTER_NS) {
        parse_field(FILTER_NS, value, &mut parsed_style, root, scale)
    }

    if let Some(style) = parse_style_attribute(node.attribute(STYLE_ATTR)) {
        for key_val in style.into_iter() {
            let key = key_val.first();
            let value = key_val.last();
            if let (Some(key), Some(val)) = (key, value) {
                let key = *key;
                let val = *val;
                if !key.is_empty() {
                    match key {
                        FILL_ATTR => {
                            let value = val;
                            if value.contains("url") {
                                let mut value = value;
                                let slice: Vec<_> = value.split(" ").collect();
                                if slice.len() > 1 {
                                    if let Some(val) = slice.into_iter().find(|v| v.contains("url"))
                                    {
                                        value = val;
                                    }
                                }
                                let id = value
                                    .replace("url('#", "")
                                    .replace("')", "")
                                    .replace("url(#", "")
                                    .replace(")", "");
                                if let Some(node) =
                                root.descendants().find(|v| v.attribute("id") == Some(&id))
                                {
                                    match node.tag_name().name() {
                                        LINEAR_GRADIENT_NS => {
                                            let gradient = gradient::linear::handle_linear_gradient(
                                                context, &node, root,
                                            );
                                            parsed_style.fill_paint.fill_gradient = Some(gradient);
                                        }
                                        RADIAL_GRADIENT_NS => {
                                            let gradient = gradient::radial::handle_radial_gradient(
                                                context, &node, root,
                                            );
                                            parsed_style.fill_paint.fill_gradient = Some(gradient);
                                        }
                                        PATTERN_NS => {
                                            let pattern =
                                                pattern::handle_pattern(context, &node, root, None);
                                            parsed_style.fill_paint.fill_pattern = pattern
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            parse_field(FILL_ATTR, value, &mut parsed_style, root, scale);
                        }
                        STROKE_ATTR => {
                            let value = val;
                            if value.contains("url") {
                                let mut value = value;
                                let slice: Vec<_> = value.split(" ").collect();
                                if slice.len() > 1 {
                                    if let Some(val) = slice.into_iter().find(|v| v.contains("url"))
                                    {
                                        value = val;
                                    }
                                }
                                let id = value
                                    .replace("url('#", "")
                                    .replace("')", "")
                                    .replace("url(#", "")
                                    .replace(")", "");
                                if let Some(node) =
                                root.descendants().find(|v| v.attribute("id") == Some(&id))
                                {
                                    match node.tag_name().name() {
                                        LINEAR_GRADIENT_NS => {
                                            let gradient = gradient::linear::handle_linear_gradient(
                                                context, &node, root,
                                            );
                                            parsed_style.stroke_paint.stroke_gradient =
                                                Some(gradient);
                                        }
                                        RADIAL_GRADIENT_NS => {
                                            let gradient = gradient::radial::handle_radial_gradient(
                                                context, &node, root,
                                            );
                                            parsed_style.stroke_paint.stroke_gradient =
                                                Some(gradient);
                                        }
                                        PATTERN_NS => {
                                            let pattern =
                                                pattern::handle_pattern(context, &node, root, None);
                                            parsed_style.stroke_paint.stroke_pattern = pattern
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            parse_field(STROKE_ATTR, value, &mut parsed_style, root, scale);
                        }
                        FILTER_NS => {
                            let value = val;
                            let id = value
                                .replace("url('#", "")
                                .replace("')", "")
                                .replace("url(#", "")
                                .replace(")", "");
                            if let Some(node) =
                            root.descendants().find(|v| v.attribute("id") == Some(&id))
                            {
                                match node.tag_name().name() {
                                    FILTER_NS => {
                                        parsed_style.filter =
                                            filters::handle_filters(context, &node, root);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => parse_field(key, val, &mut parsed_style, root, scale),
                    }
                }
            }
        }
    }
    parsed_style
}

fn get_transform_map(value: &str) -> HashMap<&str, String> {
    let mut map = HashMap::new();
    let transform: Vec<_> = value
        .split(")")
        .into_iter()
        .map(|v| {
            let v = v.trim().replace("\n", "");
            v
        })
        .filter(|v| !v.is_empty())
        .collect();

    for transform in transform.into_iter() {
        if transform.contains("rotate") {
            let value = transform.replace("rotate(", "");
            map.insert("rotate", value);
        } else if transform.contains("translateX") {
            let value = transform.replace("translateX(", "");
            map.insert("translateX", value);
        } else if transform.contains("translateY") {
            let value = transform.replace("translateY(", "");
            map.insert("translateY", value);
        } else if transform.contains("translate") {
            let value = transform.replace("translate(", "");
            map.insert("translate", value);
        } else if transform.contains("skewX") {
            let value = transform.replace("skewX(", "");
            map.insert("skewX", value);
        } else if transform.contains("skewY") {
            let value = transform.replace("skewY(", "");
            map.insert("skewY", value);
        } else if transform.contains("scale") {
            let value = transform.replace("scale(", "");
            map.insert("scale", value);
        } else if transform.contains("matrix") {
            let value = transform.replace("matrix(", "");
            map.insert("matrix", value);
        }
    }
    map
}

fn handle_clip<'a>(
    name: &str,
    value: &'a str,
    style: &mut ParsedStyle<'a>,
    root: &'a roxmltree::Document<'a>,
    scale: f32,
) {
    get_real_node(value, root, |clip_path, is_last| {
        if is_last {
            let mut clip = Clip::default();
            clip.set_units(
                match clip_path
                    .attribute(CLIP_PATH_UNITS_ATTR)
                    .unwrap_or("objectBoundingBox")
                {
                    "userSpaceOnUse" => crate::common::svg::length::Units::UserSpaceOnUse,
                    _ => crate::common::svg::length::Units::ObjectBoundingBox,
                },
            );
            let mut clips: Vec<ClipShape> = Vec::new();
            for node in clip_path.children().into_iter() {
                if node.is_element() {
                    clips.push(ClipShape::from(&node))
                }
            }

            if clips.len() > 0 {
                clip.set_clips(clips.as_slice());
                style.clip_path = clip.clone();
            }
        }
    });
}

fn parse_field<'a>(
    name: &str,
    value: &'a str,
    style: &mut ParsedStyle<'a>,
    root: &'a roxmltree::Document<'a>,
    scale: f32,
) {
    match name {
        DISPLAY_ATTR => {
            style.display = match value {
                "none" => Display::None,
                "inherit" => Display::Inherit,
                _ => Display::Inline,
            }
        }
        VISIBILITY_ATTR => {
            style.visibility = match value {
                "hidden" => Visibility::Hidden,
                "collapse" => Visibility::Collapse,
                _ => Visibility::Visible,
            }
        }
        CLIP_PATH_ATTR => handle_clip(name, value, style, root, scale),
        TRANSFORM_ATTR => style.transform = Some(get_transform_map(value)),
        STROKE_LINE_JOIN_ATTR => style.stroke_paint.stroke_linejoin = value,
        STROKE_LINE_CAP_ATTR => style.stroke_paint.stroke_linecap = value,
        STROKE_DASH_ARRAY_ATTR => {
            style.stroke_paint.stroke_dasharray = parse_stroke_dash_array(value, scale)
        }
        STROKE_DASH_OFFSET_ATTR => {
            if let Ok(stroke_dash_offset) = value.parse::<f32>() {
                style.stroke_paint.stroke_dashoffset = stroke_dash_offset
            }
        }
        STROKE_MITER_LIMIT_ATTR => {
            if let Ok(stroke_miter_limit) = value.parse::<f32>() {
                style.stroke_paint.stroke_miterlimit = stroke_miter_limit
            }
        }
        STROKE_WIDTH_ATTR => style.stroke_paint.stroke_width = parse_length(value, 0, scale),
        STROKE_ATTR => {
            style.stroke_paint.set_color(value);
        }
        FILL_ATTR => {
            style.fill_paint.set_color(value);
        }
        FILL_OPACITY_ATTR => {
            if let Ok(opacity) = value.parse::<f32>() {
                style.fill_paint.fill_opacity = opacity
            }
        }
        FILL_RULE_ATTR => style.fill_paint.fill_rule = value,
        STROKE_OPACITY_ATTR => {
            if let Ok(opacity) = value.parse::<f32>() {
                style.stroke_paint.stroke_opacity = opacity
            }
        }
        OPACITY_ATTR => {
            if let Ok(opacity) = value.parse::<f32>() {
                style.opacity = Some(opacity)
            }
        }
        STOP_COLOR_ATTR => style.stop_color = Some(value),
        STOP_OPACITY_ATTR => {
            if let Ok(opacity) = value.parse::<f32>() {
                style.stop_opacity = Some(opacity)
            }
        }
        _ => {}
    }
}

fn parse_stroke_dash_array(value: &str, scale: f32) -> Option<Vec<f32>> {
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
                -1f32
            }
        })
        .collect();
    if !has_error {
        return Some(array);
    }
    None
}

fn parse_style_attribute(style: Option<&str>) -> Option<Vec<Vec<&str>>> {
    if let Some(style) = style {
        let value: Vec<_> = style
            .trim()
            .split(";")
            .into_iter()
            .map(|key_val| {
                let value: Vec<_> = key_val
                    .split(":")
                    .map(|v| v.trim())
                    .filter(|v| !v.is_empty())
                    .collect();
                value
            })
            .filter(|v| !v.is_empty())
            .collect();
        return Some(value);
    }
    None
}
