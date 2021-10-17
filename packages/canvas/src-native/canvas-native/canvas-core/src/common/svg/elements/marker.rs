use roxmltree::Node;
use skia_safe::Point;
use std::num::ParseIntError;

use crate::common::context::Context;
use crate::common::context::paths::path::Path;
use crate::common::svg::attribute_names::{Attribute, NodeExt};
use crate::common::svg::bounding_box::BoundingBox;
use crate::common::svg::elements::element_names::ElementName;
use crate::common::svg::elements::prelude::style_from;
use crate::common::svg::elements::reference_element::ReferenceElement;
use crate::common::svg::elements::renderer::{handle_render_children, handle_style_data};
use crate::common::svg::elements::svg::create_context;
use crate::common::svg::enums::preserve_aspect_ratio::{AlignMeetOrSlice, view_box_to_transform};
use crate::common::svg::prelude::ColorConversation;
use crate::common::svg::units::length::{convert_length, Length, LengthUnit};
use crate::common::svg::units::Units;
use crate::common::svg::view_box::ViewBox;
use crate::common::to_data_url;
use crate::common::utils::geometry::to_degrees;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MarkerPosition {
    Start,
    Mid,
    End,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MarkerUnits {
    UserSpaceOnUse,
    StrokeWidth,
}

impl Default for MarkerUnits {
    fn default() -> Self {
        Self::StrokeWidth
    }
}

impl MarkerUnits {
    pub fn from_str(val: &str) -> Option<Self> {
        match val {
            "strokeWidth" => Some(Self::StrokeWidth),
            "userSpaceOnUse" => Some(Self::UserSpaceOnUse),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MarkerOrient {
    Auto,
    AutoStartReverse,
    Angle(i32),
}

impl Default for MarkerOrient {
    fn default() -> Self {
        Self::Angle(0)
    }
}

impl MarkerOrient {
    pub fn from_str(val: &str) -> Option<Self> {
        return match val.parse::<i32>() {
            Ok(val) => Some(Self::Angle(val)),
            Err(_) => match val {
                "auto" => Some(Self::Auto),
                "auto-start-reverse" => Some(Self::AutoStartReverse),
                _ => None,
            },
        };
    }
}

#[derive(Clone)]
pub struct Marker {
    id: String,
    ref_x: Length,
    ref_y: Length,
    marker_height: Length,
    marker_width: Length,
    marker_units: MarkerUnits,
    orient: MarkerOrient,
    preserve_aspect_ratio: AlignMeetOrSlice,
    node_to_render: String,
    view_box: Option<ViewBox>,
}

impl Marker {
    pub fn set_markers(markers: &mut (Vec<Point>, Vec<f32>), path: &Path) {
        let pts = &mut markers.0;
        if pts.is_empty() {
            let count = path.path.count_points();
            let mut points = vec![skia_safe::Point::default(); count];
            let angles = &mut markers.1;
            path.path.get_points(points.as_mut_slice());
            if !points.is_empty() {
                let last = points.len() - 1;
                let mut position = 0;
                for point in points.iter() {
                    if position == last {
                        break;
                    }
                    if let Some(next_point) = points.get(position + 1) {
                        angles.push((next_point.y - point.y).atan2(next_point.x - point.x));
                    }

                    position += 1;
                }
                let _ = std::mem::replace(pts, points);
            }
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn ref_x(&self) -> Length {
        self.ref_x
    }

    pub fn ref_y(&self) -> Length {
        self.ref_y
    }

    pub fn marker_width(&self) -> Length {
        self.marker_width
    }

    pub fn marker_height(&self) -> Length {
        self.marker_height
    }

    pub fn marker_units(&self) -> MarkerUnits {
        self.marker_units
    }

    pub fn marker_orient(&self) -> MarkerOrient {
        self.orient
    }

    pub fn preserve_aspect_ratio(&self) -> AlignMeetOrSlice {
        self.preserve_aspect_ratio
    }

    pub fn render_marker(
        &self,
        context: &mut Context,
        view_box: ViewBox,
        root_element: Node,
        stroke_width: f32,
        markers: Option<(&[skia_safe::Point], &[f32])>,
        position: MarkerPosition,
    ) {
        let id = &self.node_to_render;
        if let (Some(node), _) = root_element.get_node(id) {
            let density = context.device.density;
            let alpha = context.device.alpha;
            let font_color = context.font_color.to_int() as i32;
            let ppi = context.device.ppi;
            let direction = context.direction();

            let x = convert_length(
                self.ref_x,
                Attribute::RefX,
                Units::UserSpaceOnUse,
                context.device,
                &view_box,
            );
            let y = convert_length(
                self.ref_y,
                Attribute::Y,
                Units::UserSpaceOnUse,
                context.device,
                &view_box,
            );

            let width = convert_length(
                self.marker_width,
                Attribute::MarkerWidth,
                Units::UserSpaceOnUse,
                context.device,
                &view_box,
            );
            let height = convert_length(
                self.marker_height,
                Attribute::MarkerHeight,
                Units::UserSpaceOnUse,
                context.device,
                &view_box,
            );

            let mut marker_context =
                create_context(width, height, density, alpha, font_color, ppi, direction);

            if let Some(view_box) = self.view_box {
                let preserve_aspect_ratio = node.get_preserve_aspect_ratio();
                let mat = view_box_to_transform(&view_box, preserve_aspect_ratio, width, height);
                marker_context.transform_with_matrix(&mat);
            } else {
                marker_context.scale(stroke_width, stroke_width);
            }

            marker_context.translate(-x, -y);

            let style = style_from(node);
            let bounding_box = BoundingBox::default();
            let parsed = handle_style_data(&style, &mut marker_context, root_element, bounding_box);

            if !parsed.is_visible() {
                return;
            }

            if node.tag_name().name() == ElementName::Marker.to_str() {
                let mut children = node.children();
                handle_render_children(&mut marker_context, &mut children, root_element);
            }

            let image = marker_context.surface.image_snapshot();
            match self.orient {
                MarkerOrient::Auto => match markers {
                    None => {}
                    Some(markers) => match position {
                        MarkerPosition::Start => {
                            if let (Some(point), Some(value)) = (markers.0.get(0), markers.1.get(0))
                            {
                                context.save();
                                context.rotate(*value);
                                context.translate(point.x, point.y);
                                context.draw_image_with_points(&image, 0.0, 0.0);
                                context.restore();
                            }
                        }
                        MarkerPosition::Mid => {
                            let point_count = markers.0.len();
                            if point_count > 0 {
                                for i in 1..point_count - 1 {
                                    if let (Some(point), Some(value)) =
                                    (markers.0.get(i), markers.1.get(i))
                                    {
                                        context.save();
                                        context.rotate(*value);
                                        context.translate(point.x, point.y);
                                        context.draw_image_with_points(&image, 0.0, 0.0);
                                        context.restore();
                                    }
                                }
                            }
                        }
                        MarkerPosition::End => {
                            let point_count = markers.0.len();
                            let angle_count = markers.1.len();
                            if point_count > 0 && angle_count > 0 {
                                if let (Some(point), Some(value)) = (
                                    markers.0.get(point_count - 1),
                                    markers.1.get(angle_count - 1),
                                ) {
                                    context.save();
                                    context.rotate(*value);
                                    context.translate(point.x, point.y);
                                    context.draw_image_with_points(&image, 0.0, 0.0);
                                    context.restore();
                                }
                            }
                        }
                    },
                },
                MarkerOrient::AutoStartReverse => match markers {
                    None => {}
                    Some(markers) => {
                        if position == MarkerPosition::Start {
                            if let Some(value) = markers.1.get(0) {
                                context.rotate(*value - 180.0);
                            }
                        }

                        match position {
                            MarkerPosition::Start => {
                                if let Some(value) = markers.1.get(0) {
                                    context.rotate(*value - 180.0);
                                }
                            }
                            MarkerPosition::Mid => {
                                let point_count = markers.0.len();
                                if point_count > 0 {
                                    for i in 1..point_count - 1 {
                                        if let Some(value) = markers.1.get(i) {
                                            context.rotate(*value - 180.0);
                                        }
                                    }
                                }
                            }
                            MarkerPosition::End => {
                                let point_count = markers.0.len();
                                let angle_count = markers.1.len();
                                if point_count > 0 && angle_count > 0 {
                                    if let (Some(point), Some(value)) = (
                                        markers.0.get(point_count - 1),
                                        markers.1.get(angle_count - 1),
                                    ) {
                                        context.rotate(*value - 180.0);
                                    }
                                }
                            }
                        }
                    }
                },
                MarkerOrient::Angle(angle) => {
                    context.rotate(angle as f32);
                }
            }

            context.restore();
        }
    }
}

impl ReferenceElement for Marker {
    fn element_type() -> ElementName {
        ElementName::Marker
    }
    fn from_node(node: (Option<Node>, Option<Node>)) -> Option<Self> {
        if node.0.is_none() {
            return None;
        }

        let mut marker_height: Option<Length> = None;
        let mut marker_width: Option<Length> = None;
        let mut orient: Option<MarkerOrient> = None;
        let mut marker_units: Option<MarkerUnits> = None;
        let mut preserve_aspect_ratio: Option<AlignMeetOrSlice> = None;
        let mut view_box: Option<ViewBox> = None;
        let mut ref_x: Option<Length> = None;
        let mut ref_y: Option<Length> = None;
        let mut id = String::new();
        let mut handle_marker = |node: &Node| {
            if ref_x.is_none() {
                ref_x = node.get_length_opt(Attribute::RefX);
            }
            if ref_y.is_none() {
                ref_y = node.get_length_opt(Attribute::RefY);
            }

            if marker_height.is_none() {
                marker_height = node.get_length_opt(Attribute::MarkerHeight);
            }
            if marker_width.is_none() {
                marker_width = node.get_length_opt(Attribute::MarkerWidth);
            }

            if orient.is_none() {
                orient = MarkerOrient::from_str(
                    node.attribute(Attribute::Orient.to_str())
                        .unwrap_or_default(),
                );
            }

            if marker_units.is_none() {
                marker_units = MarkerUnits::from_str(
                    node.attribute(Attribute::MarkerUnits.to_str())
                        .unwrap_or_default(),
                );
            }
            if preserve_aspect_ratio.is_none() {
                preserve_aspect_ratio = Some(node.get_preserve_aspect_ratio());
            }

            if view_box.is_none() {
                view_box = node.get_view_box();
            }
        };

        match node {
            (Some(node), _) => {
                id = node.get_id().unwrap_or_default().to_owned();
                handle_marker(&node);
            }
            (_, _) => {}
        };

        let node_to_render = id.clone();
        Some(Marker {
            id,
            ref_x: ref_x.unwrap_or(Length::new(0.0, LengthUnit::None)),
            view_box,
            marker_height: marker_height.unwrap_or(Length::new(3.0, LengthUnit::None)),
            marker_width: marker_width.unwrap_or(Length::new(3.0, LengthUnit::None)),
            marker_units: marker_units.unwrap_or(MarkerUnits::default()),
            orient: orient.unwrap_or(MarkerOrient::default()),
            preserve_aspect_ratio: preserve_aspect_ratio.unwrap_or_default(),
            node_to_render,
            ref_y: ref_y.unwrap_or(Length::new(0.0, LengthUnit::None)),
        })
    }
}
