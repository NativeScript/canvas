use phf::phf_map;
use roxmltree::Node;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use crate::common::svg::elements::element_names::ElementName;
use crate::common::svg::elements::parser::StyleMap;
use crate::common::svg::elements::prelude::*;
use crate::common::svg::enums::preserve_aspect_ratio::{
    AlignMeetOrSlice, AspectRatioAlign, AspectRatioMeetOrSlice,
};
use crate::common::svg::units::length::Length;
use crate::common::svg::view_box::ViewBox;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Attribute {
    Id,
    Cx,
    Cy,
    Fr,
    Fx,
    Fy,
    R,
    X,
    Y,
    Width,
    Height,
    X1,
    Y1,
    X2,
    Y2,
    PatternUnits,
    PatternContentUnits,
    PatternTransform,
    PrimitiveUnits,
    FilterUnits,
    GradientUnits,
    ClipPathUnits,
    MaskContentUnits,
    MaskUnits,
    Rx,
    Ry,
    ViewBox,
    Style,
    Points,
    D,
    Href,
    StdDeviation,
    GradientTransform,
    SpreadMethod,
    Dx,
    Dy,
    Offset,
    PreserveAspectRatio,
    MarkerHeight,
    MarkerWidth,
    MarkerUnits,
    Orient,
    RefX,
    RefY,

    In,
    AlignmentBaseline,
    BaselineShift,
    ClipPath,
    ClipRule,
    Color,
    ColorInterpolation,
    ColorInterpolationFilters,
    ColorRendering,
    Cursor,
    Direction,
    Display,
    DominantBaseline,
    Fill,
    FillOpacity,
    FillRule,
    Filter,
    FloodColor,
    FloodOpacity,
    FontFamily,
    FontSize,
    FontSizeAdjust,
    FontStretch,
    FontStyle,
    FontVariant,
    FontWeight,
    ImageRendering,
    LetterSpacing,
    LightingColor,
    MarkerEnd,
    MarkerMid,
    MarkerStart,
    Mask,
    Opacity,
    Overflow,
    PointerEvents,
    ShapeRendering,
    SolidColor,
    SolidOpacity,
    StopColor,
    StopOpacity,
    Stroke,
    StrokeDasharray,
    StrokeDashoffset,
    StrokeLinecap,
    StrokeLinejoin,
    StrokeMiterlimit,
    StrokeOpacity,
    StrokeWidth,
    TextAnchor,
    TextDecoration,
    TextRendering,
    Transform,
    UnicodeBidi,
    VectorEffect,
    Visibility,
    WordSpacing,
    WritingMode,
}

static ATTRIBUTE_NAMES_MAP: phf::Map<&'static str, Attribute> = phf_map! {
"id" => Attribute::Id,
"cx" => Attribute::Cx,
"cy" => Attribute::Cy,
"fr" => Attribute::Fr,
"fx" => Attribute::Fx,
"fy" => Attribute::Fy,
"r" => Attribute::R,
"x" => Attribute::X,
"y" => Attribute::Y,
"width" => Attribute::Width,
"height" => Attribute::Height,
"x1" => Attribute::X1,
"y1" => Attribute::Y1,
"x2" => Attribute::X2,
"y2" => Attribute::Y2,
"patternUnits" => Attribute::PatternUnits,
"patternContentUnits" => Attribute::PatternContentUnits,
"patternTransform" => Attribute::PatternTransform,
"primitiveUnit" => Attribute::PrimitiveUnits,
"filterUnits" => Attribute::FilterUnits,
"gradientUnits" => Attribute::GradientUnits,
"clipPathUnits" => Attribute::ClipPathUnits,
"maskUnits" => Attribute::MaskUnits,
"maskContentUnits" => Attribute::MaskContentUnits,
"rx" => Attribute::Rx,
"ry" => Attribute::Ry,
"viewBox" => Attribute::ViewBox,
"style" => Attribute::Style,
"points" => Attribute::Points,
"d" => Attribute::D,
"href" => Attribute::Href,
"in" => Attribute::In,
"stdDeviation" => Attribute::StdDeviation,
"gradientTransform" => Attribute::GradientTransform,
"spreadMethod" => Attribute::SpreadMethod,
"dx" => Attribute::Dx,
"dy" => Attribute::Dy,
"offset" => Attribute::Offset,
"preserveAspectRatio" => Attribute::PreserveAspectRatio,
"markerHeight" => Attribute::MarkerHeight,
"markerWidth" => Attribute::MarkerWidth,
"markerUnits" => Attribute::MarkerUnits,
"orient" => Attribute::Orient,
"refX" => Attribute::RefX,
"refY" => Attribute::RefY,

"alignment-baseline" => Attribute::AlignmentBaseline,
"baseline-shift" => Attribute::BaselineShift,
"clip-path" => Attribute::ClipPath,
"clip-rule" => Attribute::ClipRule,
"color" => Attribute::Color,
"color-interpolation" => Attribute::ColorInterpolation,
"color-interpolation-filters" => Attribute::ColorInterpolationFilters,
"color-rendering" => Attribute::ColorRendering,
"cursor" => Attribute::Cursor,
"direction" => Attribute::Direction,
"display" => Attribute::Display,
"dominant-baseline" => Attribute::DominantBaseline,
"fill" => Attribute::Fill,
"fill-opacity" => Attribute::FillOpacity,
"fill-rule" => Attribute::FillRule,
"filter" => Attribute::Filter,
"flood-color" => Attribute::FloodColor,
"flood-opacity" => Attribute::FloodOpacity,
"font-family" => Attribute::FontFamily,
"font-size" => Attribute::FontSize,
"font-size-adjust" => Attribute::FontSizeAdjust,
"font-stretch" => Attribute::FontStretch,
"font-style"  => Attribute::FontStyle,
"font-variant" => Attribute::FontVariant,
"font-weight" =>Attribute::FontWeight,
"image-rendering" => Attribute::ImageRendering,
"letter-spacing" => Attribute::LetterSpacing,
"lighting-color" => Attribute::LightingColor,
"marker-end" => Attribute::MarkerEnd,
"marker-mid" => Attribute::MarkerMid,
"marker-start" => Attribute::MarkerStart,
"mask" => Attribute::Mask,
"opacity" => Attribute::Opacity,
"overflow" => Attribute::Overflow,
"pointer-events" => Attribute::PointerEvents,
"shape-rendering" => Attribute::ShapeRendering,
"solid-color" => Attribute::SolidColor,
"solid-opacity" => Attribute::SolidOpacity,
"stop-color" => Attribute::StopColor,
"stop-opacity" =>Attribute::StopOpacity,
"stroke" => Attribute::Stroke,
"stroke-dasharray" => Attribute::StrokeDasharray,
"stroke-dashoffset" => Attribute::StrokeDashoffset,
"stroke-linecap" => Attribute::StrokeLinecap,
"stroke-linejoin" => Attribute::StrokeLinejoin,
"stroke-miterlimit" => Attribute::StrokeMiterlimit,
"stroke-opacity" => Attribute::StrokeOpacity,
"stroke-width" => Attribute::StrokeWidth,
"text-anchor" => Attribute::TextAnchor,
"text-decoration" => Attribute::TextDecoration,
"text-rendering" => Attribute::TextRendering,
"transform" => Attribute::Transform,
"unicode-bidi" => Attribute::UnicodeBidi,
"vector-effect" => Attribute::VectorEffect,
"visibility" => Attribute::Visibility,
"word-spacing" => Attribute::WordSpacing,
"writing-mode" => Attribute::WritingMode
};

impl Attribute {
    pub fn from_str(text: &str) -> Option<Attribute> {
        ATTRIBUTE_NAMES_MAP.get(text).cloned()
    }

    pub fn to_str(&self) -> &'static str {
        ATTRIBUTE_NAMES_MAP
            .entries()
            .find(|kv| kv.1 == self)
            .unwrap()
            .0
    }
}

pub trait NodeExt {
    fn get_id(&self) -> Option<&str>;
    fn get_length(&self, attr: Attribute, def: Length) -> Length;
    fn get_length_opt(&self, attr: Attribute) -> Option<Length>;
    fn get_view_box(&self) -> Option<ViewBox>;
    fn get_node(&self, id: &str) -> (Option<Node>, Option<Node>);
    fn get_node_with_style(
        &self,
        id: &str,
        style: Option<&mut StyleMap>,
        overwrite: bool,
    ) -> (Option<Node>, Option<Node>);
    fn get_href(&self) -> Option<&str>;
    fn get_element_from_id(&self, id: &str) -> Option<ElementName>;
    fn get_preserve_aspect_ratio(&self) -> AlignMeetOrSlice;
}

impl<'a> NodeExt for Node<'a, '_> {
    fn get_id(&self) -> Option<&str> {
        self.attribute(Attribute::Id.to_str())
    }

    fn get_length(&self, attr: Attribute, def: Length) -> Length {
        match self.attribute(attr.to_str().trim_end()) {
            None => def,
            Some(val) => Length::from_str(val).unwrap_or(def),
        }
    }
    fn get_length_opt(&self, attr: Attribute) -> Option<Length> {
        match self.attribute(attr.to_str().trim_end()) {
            None => None,
            Some(val) => match Length::from_str(val) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
        }
    }

    fn get_view_box(&self) -> Option<ViewBox> {
        match self.attribute(Attribute::ViewBox.to_str().trim_end()) {
            None => None,
            Some(val) => match ViewBox::from_str(val) {
                Ok(val) => Some(val),
                _ => None,
            },
        }
    }

    fn get_node(&self, id: &str) -> (Option<Node<'a, '_>>, Option<Node<'a, '_>>) {
        self.get_node_with_style(id, None, true)
    }

    fn get_node_with_style(
        &self,
        id: &str,
        mut style: Option<&mut StyleMap>,
        overwrite: bool,
    ) -> (Option<Node>, Option<Node>) {
        let root = self.document();
        let find_node = |id: &str| -> Option<Node> {
            let id = id
                .replace("url('#", "")
                .replace("')", "")
                .replace("url(#", "")
                .replace(")", "")
                .replace("#", "");
            root.root_element()
                .descendants()
                .find(|v| v.get_id() == Some(id.as_str()))
        };
        let first_doc = find_node(id);
        let mut ref_doc: Option<Node> = None;
        let mut element_style = StyleMap::new();
        let mut ref_style = StyleMap::new();
        let handle_style = style.is_some();
        if let Some(document) = first_doc.as_ref() {
            if let Some(href) = document.get_href() {
                let mut next = find_node(href);
                while next.is_some() {
                    match next {
                        None => {
                            next = None;
                            if handle_style {
                                element_style.update_with_style(&ref_style, overwrite);

                                if let Some(style) = style.as_mut() {
                                    style.update_with_style(&element_style, overwrite)
                                }
                            }
                        }
                        Some(val) => {
                            if handle_style {
                                let sm = crate::common::svg::elements::parser::style_from(val);
                                element_style = sm;
                                ref_style.update_with_style(&element_style, overwrite);
                            }
                            match val.get_href() {
                                None => {
                                    ref_doc = Some(val);
                                    next = None;
                                }
                                Some(href) => {
                                    next = find_node(href);
                                }
                            }
                        }
                    }
                }
            } else {
                let sm = crate::common::svg::elements::parser::style_from(*document);
                if let Some(style) = style.as_mut() {
                    style.update_with_style(&sm, overwrite)
                }
            }
        }
        return (first_doc, ref_doc);
    }

    fn get_href(&self) -> Option<&str> {
        let mut href: Option<&str>;
        match (
            self.attribute(Attribute::Href.to_str()),
            self.attribute(("http://www.w3.org/1999/xlink", Attribute::Href.to_str())),
        ) {
            (Some(href_), Some(_)) => href = Some(href_),
            (Some(href_), _) => href = Some(href_),
            (_, Some(xlink_href)) => href = Some(xlink_href),
            (_, _) => href = None,
        }
        href
    }

    fn get_element_from_id(&self, id: &str) -> Option<ElementName> {
        match self.get_node(id) {
            (Some(node), _) => ElementName::from_str(node.tag_name().name()),
            _ => None,
        }
    }

    fn get_preserve_aspect_ratio(&self) -> AlignMeetOrSlice {
        let mut result = AlignMeetOrSlice::default();
        match self.attribute(Attribute::PreserveAspectRatio.to_str()) {
            None => result,
            Some(val) => {
                let ratio: Vec<_> = val.split(" ").filter(|v| !v.is_empty()).collect();
                if ratio.len() == 1 {
                    if let Some(align) = ratio.get(0) {
                        result.align_set(AspectRatioAlign::from(*align))
                    }
                } else if ratio.len() >= 2 {
                    if let Some(align) = ratio.get(0) {
                        result.align_set(AspectRatioAlign::from(*align))
                    }

                    if let Some(meet_or_slice) = ratio.get(0) {
                        result.meet_or_slice_set(AspectRatioMeetOrSlice::from(*meet_or_slice))
                    }
                }
                result
            }
        }
    }
}
