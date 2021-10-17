use roxmltree::Node;
use skia_safe::{Color, ISize, Point, SamplingOptions, Surface, Vector};

use crate::common::context::{Context, Device, State};
use crate::common::context::paths::path::Path;
use crate::common::context::text_styles::text_direction::TextDirection;
use crate::common::svg::attribute_names::{Attribute, NodeExt};
use crate::common::svg::elements::element_names::ElementName;
use crate::common::svg::elements::renderer::handle_render_children;
use crate::common::svg::enums::preserve_aspect_ratio::{AlignMeetOrSlice, view_box_to_transform};
use crate::common::svg::units::length::{convert_length, Length, LengthUnit};
use crate::common::svg::units::Units;
use crate::common::svg::view_box::ViewBox;

pub struct Svg<'a> {
    x: Length,
    y: Length,
    width: Length,
    height: Length,
    view_box: Option<ViewBox>,
    context: Context<'a>,
    root_element: Node<'a, 'a>,
    preserve_aspect_ratio: AlignMeetOrSlice,
    needs_render: bool,
}

pub fn create_context<'a>(
    width: f32,
    height: f32,
    density: f32,
    alpha: bool,
    font_color: i32,
    ppi: f32,
    direction: TextDirection,
) -> Context<'a> {
    let device = Device {
        width,
        height,
        density,
        non_gpu: true,
        samples: 0,
        alpha,
        ppi,
    };

    let mut context = Context {
        surface: Surface::new_raster_n32_premul(ISize::new(
            (width * density) as i32,
            (height * density) as i32,
        ))
            .unwrap(),
        path: Path::default(),
        state: State::from_device(device, direction),
        state_stack: vec![],
        font_color: Color::new(font_color as u32),
        device,
    };

    context.set_miter_limit(4.0 * density);

    context
}

impl<'a> Svg<'a> {
    /*  pub fn new_str(
        text: &'a str,
        device: Device,
        max_width: f32,
        max_height: f32,
        density: f32,
        alpha: bool,
        font_color: i32,
        ppi: f32,
        direction: TextDirection,
    ) -> Option<Self> {
        let mut document = roxmltree::Document::parse(text);
        if document.is_err() {
            return None
        }
        let mut document = document.unwrap();
        let child = document.root_element();
        Svg::new_node(
            child, device, max_width, max_height, density, alpha, font_color, ppi,
            direction,
        )
        /* for child in children.into_iter() {
             if child.is_element() && child.tag_name().name() == ElementName::Svg.to_str() {
                 if let Some(mut svg) = Svg::new_node(
                     child, device, max_width, max_height, density, alpha, font_color, ppi,
                     direction,
                 ) {
                     svg.document = Some(document);
                     value = Some(svg);
                     break;
                 }
             }
         }
         */
       // return value;
    }*/

    pub fn new_node(
        node: roxmltree::Node<'a, '_>,
        device: Device,
        max_width: f32,
        max_height: f32,
        density: f32,
        alpha: bool,
        font_color: i32,
        ppi: f32,
        direction: TextDirection,
    ) -> Option<Self> {
        let mut value = None;
        if node.is_element() && node.tag_name().name() == ElementName::Svg.to_str() {
            let x_length = node.get_length(Attribute::X, Length::zero());
            let y_length = node.get_length(Attribute::Y, Length::zero());
            let mut width_length = node.get_length(Attribute::Height, Length::zero());
            let mut height_length = node.get_length(Attribute::Width, Length::zero());
            match (
                node.attribute(Attribute::Width.to_str()),
                node.attribute(Attribute::Height.to_str()),
            ) {
                (Some(_), Some(_)) => {}
                (Some(_), _) => {
                    width_length = Length::new(100.0, LengthUnit::Percent);
                }
                (_, Some(_)) => {
                    height_length = Length::new(100.0, LengthUnit::Percent);
                }
                (_, _) => {
                    width_length = Length::new(100.0, LengthUnit::Percent);
                    height_length = Length::new(100.0, LengthUnit::Percent);
                }
            }
            let view_box = node.get_view_box();

            if width_length.value() == 0.0 || height_length.value() == 0.0 {
                return None;
            }

            if let Some(view_box) = view_box.as_ref() {
                if view_box.width() == 0.0 || view_box.height() == 0.0 {
                    return None;
                }
            }

            let max_view_box = ViewBox::new_wh(max_width, max_width);

            let x = convert_length(
                x_length,
                Attribute::X,
                Units::UserSpaceOnUse,
                device,
                &max_view_box,
            );

            let y = convert_length(
                y_length,
                Attribute::Y,
                Units::UserSpaceOnUse,
                device,
                &max_view_box,
            );

            let width = convert_length(
                width_length,
                Attribute::Width,
                Units::UserSpaceOnUse,
                device,
                &max_view_box,
            );

            let height = convert_length(
                height_length,
                Attribute::Height,
                Units::UserSpaceOnUse,
                device,
                &max_view_box,
            );

            let mut context;
            let preserve_aspect_ratio = node.get_preserve_aspect_ratio();

            let mut vb = ViewBox::new(x, y, width, height);
            if let Some(view_box) = node.get_view_box() {
                vb.x_set(view_box.x());
                vb.y_set(view_box.y());
                vb.width_set(view_box.width());
                vb.height_set(view_box.height());
                let mat = view_box_to_transform(&vb, preserve_aspect_ratio, max_width, max_height);
                context = create_context(width, height, density, alpha, font_color, ppi, direction);

                // context.transform_with_matrix(&mat);
            } else {
                context = create_context(width, height, density, alpha, font_color, ppi, direction);
            }

            let mut svg = Svg {
                x: node.get_length(Attribute::X, Length::zero()),
                y: node.get_length(Attribute::Y, Length::zero()),
                width: width_length,
                height: height_length,
                view_box: node.get_view_box(),
                context,
                root_element: node,
                preserve_aspect_ratio,
                needs_render: true,
            };
            value = Some(svg);
        }
        value
    }

    pub fn x(&self) -> &Length {
        &self.x
    }

    pub fn y(&self) -> &Length {
        &self.y
    }

    pub fn width(&self) -> &Length {
        &self.width
    }

    pub fn height(&self) -> &Length {
        &self.height
    }

    pub fn view_box(&self) -> &Option<ViewBox> {
        &self.view_box
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn context_mut(&mut self) -> &'a mut Context {
        &mut self.context
    }

    pub fn render(&mut self) {
        let element = self.root_element;
        let mut children = element.children();
        let ctx = self.context_mut();
        handle_render_children(ctx, &mut children, element);
        self.needs_render = false;
    }

    pub fn render_to_context(&mut self, context: &mut Context) {
        if self.needs_render {
            self.render()
        }
        let snapshot = self.context.surface.image_snapshot();
        let mut paint = skia_safe::Paint::default();
        paint.set_anti_alias(true);
        let canvas = context.surface.canvas();
        canvas.draw_image_with_sampling_options(
            snapshot,
            (0.0, 0.0),
            SamplingOptions::from_filter_quality(skia_safe::FilterQuality::High, None),
            Some(&paint),
        );
    }

    // pub fn render_to_context(&mut self, context: &mut Context) {
    //     if self.needs_render {
    //         self.render()
    //     }
    //
    //     let snapshot = self.context.surface.image_snapshot();
    //     let mut paint = skia_safe::Paint::default();
    //     paint.set_anti_alias(true);
    //     let max_width = context.surface.width() as f32;
    //     let max_height = context.surface.height() as f32;
    //     let max_view_box = ViewBox::new_with_context(context);
    //     let canvas = context.surface.canvas();
    //     let device = context.device;
    //     let x = convert_length(
    //         self.x,
    //         Attribute::X,
    //         Units::UserSpaceOnUse,
    //         device,
    //         &max_view_box,
    //     );
    //
    //     let y = convert_length(
    //         self.y,
    //         Attribute::Y,
    //         Units::UserSpaceOnUse,
    //         device,
    //         &max_view_box,
    //     );
    //
    //     let width = convert_length(
    //         self.width,
    //         Attribute::Width,
    //         Units::UserSpaceOnUse,
    //         device,
    //         &max_view_box,
    //     );
    //
    //     let height = convert_length(
    //         self.height,
    //         Attribute::Height,
    //         Units::UserSpaceOnUse,
    //         device,
    //         &max_view_box,
    //     );
    //
    //     let mut vb = ViewBox::new(x, y, width, height);
    //     if let Some(view_box) = self.view_box() {
    //         vb.x_set(view_box.x());
    //         vb.y_set(view_box.y());
    //         vb.width_set(view_box.width());
    //         vb.height_set(view_box.height());
    //     }
    //
    //     let mut preserve_aspect_ratio = self.preserve_aspect_ratio;
    //
    //     let mat = view_box_to_transform(&vb, preserve_aspect_ratio, max_width, max_height);
    //
    //     let mut bounds = snapshot.bounds();
    //     let rect = skia_safe::Rect::from_xywh(
    //        mat.translate_x(), mat.translate_y(),
    //        bounds.width() as f32 * mat.scale_x(), bounds.height() as f32 * mat.scale_y(),
    //     );
    //     //canvas.draw_image(snapshot, Point::new(0.0, 0.0), Some(&paint));
    //     log::debug!(" rect {:?} size {:?} image {:?}", rect , snapshot.bounds().size(), &snapshot.bounds());
    //     canvas.draw_image_rect(snapshot, None, rect, &paint);
    // }
}
