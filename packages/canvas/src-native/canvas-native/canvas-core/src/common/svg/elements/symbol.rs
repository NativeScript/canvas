use roxmltree::Node;

use crate::common::context::Context;
use crate::common::svg::attribute_names::{Attribute, NodeExt};
use crate::common::svg::bounding_box::BoundingBox;
use crate::common::svg::elements::parser::{Parser, style_from, StyleMap};
use crate::common::svg::elements::renderer::{handle_render_child, handle_style_data, Renderer};
use crate::common::svg::elements::svg::create_context;
use crate::common::svg::enums::preserve_aspect_ratio::view_box_to_transform;
use crate::common::svg::prelude::ColorConversation;
use crate::common::svg::units::length::{convert_length, Length, LengthUnit};
use crate::common::svg::units::Units;
use crate::common::svg::view_box::ViewBox;
use crate::common::to_data_url;

#[derive(Clone)]
pub struct Symbol {
    x: Length,
    y: Length,
    width: Length,
    height: Length,
    bounding_box: BoundingBox,
    id_ref: String,
    style: StyleMap,
}

impl Symbol {
    pub fn from_id(
        context: &mut Context,
        parent_view_box: ViewBox,
        node: Node,
        root_element: Node,
    ) {
        let style = style_from(node);
        let device = context.device;
        let x = node.get_length(Attribute::X, Length::zero());
        let y = node.get_length(Attribute::Y, Length::zero());
        let mut width = node.get_length(Attribute::Width, Length::zero());
        let mut height = node.get_length(Attribute::Height, Length::zero());

        match (
            node.attribute(Attribute::Width.to_str()),
            node.attribute(Attribute::Height.to_str()),
        ) {
            (Some(_), Some(_)) => {}
            (Some(_), _) => {
                width = Length::new(100.0, LengthUnit::Percent);
            }
            (_, Some(_)) => {
                height = Length::new(100.0, LengthUnit::Percent);
            }
            (_, _) => {
                width = Length::new(100.0, LengthUnit::Percent);
                height = Length::new(100.0, LengthUnit::Percent);
            }
        }

        let view_box = node.get_view_box();

        if width.value() == 0.0 || height.value() == 0.0 {
            return;
        }

        if let Some(view_box) = view_box.as_ref() {
            if view_box.width() == 0.0 || view_box.height() == 0.0 {
                return;
            }
        }

        let max_view_box = parent_view_box;

        let x = convert_length(
            x,
            Attribute::X,
            Units::UserSpaceOnUse,
            device,
            &max_view_box,
        );

        let y = convert_length(
            y,
            Attribute::Y,
            Units::UserSpaceOnUse,
            device,
            &max_view_box,
        );

        let mut width = convert_length(
            width,
            Attribute::Width,
            Units::UserSpaceOnUse,
            device,
            &max_view_box,
        );

        let mut height = convert_length(
            height,
            Attribute::Height,
            Units::UserSpaceOnUse,
            device,
            &max_view_box,
        );

        let mut translate_x = false;
        let mut translate_y = false;

        if width < 1.0 {
            width = width + 1.0;
            translate_x = true;
        }

        if height < 1.0 {
            height = height + 1.0;
            translate_y = true;
        }


        // use the default size ??
        let mut symbol_ctx = create_context(
            parent_view_box.width(), parent_view_box.height(), device.density, device.alpha, context.font_color.to_int() as i32, device.ppi, context.direction(),
        );

        let bounding_box = BoundingBox::new(x,y,width,height);
        let parsed = handle_style_data(&style, context, root_element, bounding_box);
        if !parsed.is_visible() {
            return;
        }


        log::debug!("symbol_ctx");
        let preserve_aspect_ratio = node.get_preserve_aspect_ratio();

        let mut vb = ViewBox::new(x, y, width, height);
        if let Some(view_box) = node.get_view_box() {
            vb.x_set(view_box.x());
            vb.y_set(view_box.y());
            vb.width_set(view_box.width());
            vb.height_set(view_box.height());
        }

        let mat = view_box_to_transform(&vb, preserve_aspect_ratio, parent_view_box.width(), parent_view_box.height());
        symbol_ctx.transform_with_matrix(&mat);

        if translate_x {
            symbol_ctx.translate(-1.0, 0.0);
        }

        if translate_y {
            symbol_ctx.translate(0.0, -1.0);
        }

        log::debug!("before");
        for child in node.children().into_iter() {
            if child.is_element() {
                log::debug!("rendering symbol {:?}", child.tag_name().name());
                handle_render_child(&mut symbol_ctx, child, root_element, None, None)
            }
        }

        let ss = symbol_ctx.surface.image_snapshot();

        log::debug!("image");

        context.draw_image_with_points(&ss, 0.0, 0.0)
    }
}

impl Renderer for Symbol {
    fn render(&mut self, context: &mut Context, node: Node, root_element: Node) {}

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}
