use roxmltree::{Document, Node};
use skia_safe::Data;

use crate::common::{
    context::{Context, Device},
    svg::{
        units::{length::convert_length, Units},
        view_box::ViewBox,
    },
};
use crate::common::context::drawing_paths::fill_rule::FillRule;
use crate::common::context::fill_and_stroke_styles::paint::PaintStyle;
use crate::common::context::fill_and_stroke_styles::pattern::Repetition;
use crate::common::context::paths::path::Path;
use crate::common::svg::attribute_names::{Attribute, NodeExt};
use crate::common::svg::bounding_box::BoundingBox;
use crate::common::svg::elements::parser::{Parser, StyleMap};
use crate::common::svg::elements::renderer::{handle_style_data, Renderer, set_filters};
use crate::common::svg::enums::preserve_aspect_ratio::{
    aligned_pos, AlignMeetOrSlice, AspectRatioMeetOrSlice,
};
use crate::common::svg::prelude::*;
use crate::common::svg::units::length::Length;

#[derive(Clone)]
pub struct Image {
    x: Length,
    y: Length,
    width: Length,
    height: Length,
    data: Option<skia_safe::Image>,
    style: StyleMap,
    bounding_box: BoundingBox,
    preserve_aspect_ratio: AlignMeetOrSlice,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            x: Length::zero(),
            y: Length::zero(),
            width: Length::zero(),
            height: Length::zero(),
            data: None,
            style: StyleMap::default(),
            bounding_box: Default::default(),
            preserve_aspect_ratio: Default::default(),
        }
    }
}

impl Image {
    pub fn x(&self) -> Length {
        self.x
    }

    pub fn y(&self) -> Length {
        self.y
    }

    pub fn width(&self) -> Length {
        self.width
    }

    pub fn height(&self) -> Length {
        self.height
    }

    pub fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }

    pub fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.bounding_box
    }

    pub fn update_bounding_box(&mut self, width: f32, height: f32, device: Device) {
        let view_box = ViewBox::new_wh(width, height);
        self.bounding_box.set_all(
            convert_length(
                self.x,
                Attribute::X,
                Units::UserSpaceOnUse,
                device,
                &view_box,
            ),
            convert_length(
                self.y,
                Attribute::Y,
                Units::UserSpaceOnUse,
                device,
                &view_box,
            ),
            convert_length(
                self.width,
                Attribute::Width,
                Units::UserSpaceOnUse,
                device,
                &view_box,
            ),
            convert_length(
                self.height,
                Attribute::Height,
                Units::UserSpaceOnUse,
                device,
                &view_box,
            ),
        )
    }

    pub fn style(&self) -> &StyleMap {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut StyleMap {
        &mut self.style
    }
}

impl Renderer for Image {
    fn render(&mut self, context: &mut Context, node: Node, root_element: Node) {
        if let Some(image) = self.data.as_ref() {
            context.save();
            let bounding_box = self.bounding_box;
            let parsed = handle_style_data(self.style(), context, root_element, bounding_box);
            if !parsed.is_visible() {
                return;
            }

            let vb = ViewBox::new_with_context(context);
            let view_box = ViewBox::new(
                convert_length(
                    self.x,
                    Attribute::X,
                    Units::UserSpaceOnUse,
                    context.device,
                    &vb,
                ),
                convert_length(
                    self.y,
                    Attribute::Y,
                    Units::UserSpaceOnUse,
                    context.device,
                    &vb,
                ),
                convert_length(
                    self.width,
                    Attribute::Width,
                    Units::UserSpaceOnUse,
                    context.device,
                    &vb,
                ),
                convert_length(
                    self.height,
                    Attribute::Height,
                    Units::UserSpaceOnUse,
                    context.device,
                    &vb,
                ),
            );


            let w = image.width();
            let h = image.height();

            if w <= 0 && h <= 0 {
                return;
            }

            // let mut surface = skia_safe::Surface::new_raster_n32_premul(skia_safe::ISize::new(w,h)).unwrap();
            //  surface.canvas().draw_image(image, skia_safe::Point::default(), None);

            let image_rect = |width: f32, height: f32| {
                let new_size = skia_safe::Size::new(width, height)
                    .fit_view_box(view_box, self.preserve_aspect_ratio);
                let (x, y) = aligned_pos(
                    self.preserve_aspect_ratio.align(),
                    view_box.x(),
                    view_box.y(),
                    view_box.width() - new_size.width,
                    view_box.height() - new_size.height,
                );

                new_size.to_rect(x, y)
            };

            let r = image_rect(w as f32, h as f32);
            let rect = skia_safe::Rect::from_xywh(r.x(), r.y(), r.width(), r.height());

            let affine = [
                rect.width() as f32 / w as f32,
                0.0,
                0.0,
                rect.height() as f32 / h as f32,
                r.x(),
                r.y(),
            ];
            let ts = skia_safe::Matrix::from_affine(&affine);

            let mut pattern = context.create_pattern(image.clone(), Repetition::NoRepeat);
            pattern.matrix_mut().pre_concat(&ts);

            if self.preserve_aspect_ratio.meet_or_slice() == AspectRatioMeetOrSlice::Slice {
                let r = view_box;
                let mut clip = Path::default();
                clip.rect(r.x(), r.y(), r.width(), r.height());
                context.clip(Some(&mut clip), None);
            }
            // TODO reuse shade if set
            //let current_shader = context.state.paint.fill_paint_mut().shader();

            context.set_fill_style(PaintStyle::Pattern(pattern));
            if let Some(filter) = parsed.filter() {
                set_filters(context, filter);
            }
            context.fill_rect(&rect);
            context.restore();
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}

impl Parser for Image {
    fn parse_from(node: Node) -> Self {
        let style = Image::style_from(node);
        let mut image = Image::default();
        image.x = Length::length_from_style(&style, Attribute::X, Length::zero());
        image.y = Length::length_from_style(&style, Attribute::Y, Length::zero());
        image.width = Length::length_from_style(&style, Attribute::Width, Length::zero());
        image.height = Length::length_from_style(&style, Attribute::Height, Length::zero());
        image.preserve_aspect_ratio = node.get_preserve_aspect_ratio();
        image.style.extend(style);
        if let Some(href) = node.get_href() {
            if href.starts_with("data:") {
                let url: Vec<_> = href.split(",").collect();
                if href.len() > 1 {
                    let data = decode_base64(url[1]);
                    let data = skia_safe::Data::new_copy(&data);
                    image.data = skia_safe::Image::from_encoded(data);
                }
            } else if href.starts_with("http") {
                if let Ok(response) = reqwest::blocking::get(href) {
                    if let Ok(bytes) = response.bytes() {
                        unsafe {
                            let data = skia_safe::Data::new_bytes(&bytes);
                            image.data = skia_safe::Image::from_encoded(data)
                        }
                    }
                }
            }
        }
        image
    }
}

type StaticCharVec = &'static [char];

const HTML_SPACE_CHARACTERS: StaticCharVec =
    &['\u{0020}', '\u{0009}', '\u{000a}', '\u{000c}', '\u{000d}'];

fn decode_base64(value: &str) -> Vec<u8> {
    fn is_html_space(c: char) -> bool {
        HTML_SPACE_CHARACTERS.iter().any(|&m| m == c)
    }
    let without_spaces = value
        .chars()
        .filter(|&c| !is_html_space(c))
        .collect::<String>();
    let mut input = &*without_spaces;

    if input.len() % 4 == 0 {
        if input.ends_with("==") {
            input = &input[..input.len() - 2]
        } else if input.ends_with("=") {
            input = &input[..input.len() - 1]
        }
    }

    if input.len() % 4 == 1 {
        return Vec::new();
    }

    if input
        .chars()
        .any(|c| c != '+' && c != '/' && !c.is_alphanumeric())
    {
        return Vec::new();
    }
    match base64::decode_config(&input, base64::STANDARD.decode_allow_trailing_bits(true)) {
        Ok(bytes) => bytes,
        Err(e) => {
            return Vec::new();
        }
    }
}
