use image::{DynamicImage, GenericImageView, ImageError};
use rgb::FromSlice;
use roxmltree::{Document, Node};
use skia_safe::SamplingOptions;
use std::str::FromStr;

use crate::common::context::compositing::composite_operation_type::CompositeOperationType;
use crate::common::context::Context;
use crate::common::context::fill_and_stroke_styles::paint::PaintStyle;
use crate::common::context::fill_and_stroke_styles::pattern::Repetition;
use crate::common::context::image_smoothing::ImageSmoothingQuality;
use crate::common::svg::attribute_names::{Attribute, NodeExt};
use crate::common::svg::bounding_box::BoundingBox;
use crate::common::svg::elements::element_names::ElementName;
use crate::common::svg::elements::prelude::style_from;
use crate::common::svg::elements::reference_element::ReferenceElement;
use crate::common::svg::elements::renderer::{handle_render_children, handle_style_data, Renderer};
use crate::common::svg::elements::svg::create_context;
use crate::common::svg::enums::preserve_aspect_ratio::view_box_to_transform;
use crate::common::svg::prelude::ColorConversation;
use crate::common::svg::units::length::{convert_length, Length, LengthUnit};
use crate::common::svg::units::Units;
use crate::common::svg::view_box::ViewBox;
use crate::common::to_data_url;

#[derive(Clone, Debug)]
pub struct Mask {
    id: String,
    x: Length,
    y: Length,
    width: Length,
    height: Length,
    mask_units: Units,
    mask_content_units: Units,
    node_to_render: Option<String>,
}

impl Mask {
    pub fn new(
        id: String,
        x: Length,
        y: Length,
        width: Length,
        height: Length,
        mask_units: Units,
        mask_content_units: Units,
    ) -> Self {
        Self {
            id,
            x,
            y,
            width,
            height,
            mask_units,
            mask_content_units,
            node_to_render: None,
        }
    }

    pub fn zero() -> Self {
        Self {
            id: "".to_string(),
            x: Length::new(-10.0, LengthUnit::Percent),
            y: Length::new(-10.0, LengthUnit::Percent),
            width: Length::new(120.0, LengthUnit::Percent),
            height: Length::new(120.0, LengthUnit::Percent),
            mask_units: Units::ObjectBoundingBox,
            mask_content_units: Units::UserSpaceOnUse,
            node_to_render: None,
        }
    }

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

    pub fn mask_content_units(&self) -> Units {
        self.mask_content_units
    }

    pub fn mask_units(&self) -> Units {
        self.mask_units
    }

    pub fn create_mask(
        &self,
        context: &mut Context,
        view_box: ViewBox,
        bounding_box: BoundingBox,
        root_element: Node,
    ) -> Option<(skia_safe::Image, skia_safe::Rect)> {
        if let Some(id) = self.node_to_render.as_ref() {
            if let (Some(node), _) = root_element.get_node(id) {
                let density = context.device.density;
                let alpha = context.device.alpha;
                let font_color = context.font_color.to_int() as i32;
                let ppi = context.device.ppi;
                let direction = context.direction();
                let mut view_box = view_box;
                match self.mask_units {
                    Units::ObjectBoundingBox => {
                        view_box = bounding_box.to_view_box();
                    }
                    _ => {}
                }

                let mut x = convert_length(
                    self.x,
                    Attribute::X,
                    self.mask_units,
                    context.device,
                    &view_box,
                );
                let mut y = convert_length(
                    self.y,
                    Attribute::Y,
                    self.mask_units,
                    context.device,
                    &view_box,
                );

                let mut width = convert_length(
                    self.width,
                    Attribute::Width,
                    self.mask_units,
                    context.device,
                    &view_box,
                );
                let mut height = convert_length(
                    self.height,
                    Attribute::Height,
                    self.mask_units,
                    context.device,
                    &view_box,
                );

                match self.mask_units {
                    Units::ObjectBoundingBox => {
                        x = x * view_box.width() + view_box.x();
                        y = y * view_box.width() + view_box.y();
                        width = width * view_box.width();
                        height = height * view_box.height();
                    }
                    _ => {}
                }

                let mut mask_context =
                    create_context(width, height, density, alpha, font_color, ppi, direction);

                if self.mask_content_units == Units::ObjectBoundingBox {
                    mask_context.scale(view_box.width(), view_box.height());
                }

                let style = style_from(node);
                let parsed =
                    handle_style_data(&style, &mut mask_context, root_element, bounding_box);

                if !parsed.is_visible() {
                    return None;
                }

                mask_context.translate(x, y);

                if node.tag_name().name() == ElementName::Mask.to_str() {
                    let mut children = node.children();
                    handle_render_children(&mut mask_context, &mut children, root_element);
                }

                let image_data = mask_context.get_image_data(x, y, width, height);
                let data = image_data.data_mut().as_rgba_mut();

                let coeff_r = 0.2125 / 255.0;
                let coeff_g = 0.7154 / 255.0;
                let coeff_b = 0.0721 / 255.0;

                for y in 0..height as i32 {
                    for x in 0..width as i32 {
                        let idx = (y * width as i32 + x) as usize;
                        let ref mut pixel = data[idx];

                        let r = pixel.r as f32;
                        let g = pixel.g as f32;
                        let b = pixel.b as f32;

                        let luma = r * coeff_r + g * coeff_g + b * coeff_b;

                        pixel.r = 0;
                        pixel.g = 0;
                        pixel.b = 0;
                        pixel.a = (luma * 255.0).clamp(0.0, 255.0) as u8;
                    }
                }
                mask_context.clear_rect(x, y, width, height);
                mask_context.put_image_data(&image_data, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

                let ss = mask_context.surface.image_snapshot();
                let rect = skia_safe::Rect::from_xywh(x, y, width, height);

                return Some((ss, rect));
            }
        }
        None
    }
}

impl ReferenceElement for Mask {
    fn element_type() -> ElementName {
        ElementName::Mask
    }

    fn from_node(node: (Option<Node>, Option<Node>)) -> Option<Self> {
        if node.0.is_none() {
            return None;
        }

        let mut x: Option<Length> = None;
        let mut y: Option<Length> = None;
        let mut width: Option<Length> = None;
        let mut height: Option<Length> = None;
        let mut mask_content_units: Option<Units> = None;
        let mut mask_units: Option<Units> = None;
        let mut node_to_render: Option<String> = None;
        let mut id = String::new();

        let mut handle_mask = |node: &Node| {
            if x.is_none() {
                x = node.get_length_opt(Attribute::X);
            }

            if y.is_none() {
                y = node.get_length_opt(Attribute::Y);
            }
            if width.is_none() {
                width = node.get_length_opt(Attribute::Width);
            }

            if height.is_none() {
                height = node.get_length_opt(Attribute::Height);
            }

            if mask_content_units.is_none() {
                mask_content_units = Units::from_str(
                    node.attribute(Attribute::MaskContentUnits.to_str())
                        .unwrap_or(""),
                );
            }

            if mask_units.is_none() {
                mask_units =
                    Units::from_str(node.attribute(Attribute::MaskUnits.to_str()).unwrap_or(""));
            }
        };
        match node {
            (Some(node), Some(ref_node)) => {
                id = node.get_id().unwrap_or_default().to_owned();
                handle_mask(&node);
                handle_mask(&ref_node);
                if let Some(id) = node.attribute(Attribute::Id.to_str()) {
                    node_to_render = Some(id.to_owned());
                }
            }
            (Some(node), None) => {
                id = node.get_id().unwrap_or_default().to_owned();
                handle_mask(&node);
                if let Some(id) = node.attribute(Attribute::Id.to_str()) {
                    node_to_render = Some(id.to_owned());
                }
            }
            (_, _) => return None,
        };

        Some(Self {
            id,
            x: x.unwrap_or(Length::new(-10.0, LengthUnit::Percent)),
            y: y.unwrap_or(Length::new(-10.0, LengthUnit::Percent)),
            width: width.unwrap_or(Length::new(120.0, LengthUnit::Percent)),
            height: height.unwrap_or(Length::new(120.0, LengthUnit::Percent)),
            mask_content_units: mask_content_units.unwrap_or(Units::UserSpaceOnUse),
            mask_units: mask_units.unwrap_or(Units::ObjectBoundingBox),
            node_to_render,
        })
    }
}
