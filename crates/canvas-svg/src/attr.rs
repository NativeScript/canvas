//! `setAttribute`/`getAttribute` over the `skia_safe::svg` tree. Setting delegates to Skia's
//! parser; reading needs per-type dispatch since Skia has no string-valued getter.

use skia_safe::svg::{Length, LengthUnit, TypedNode};
use skia_safe::Point;

pub fn format_length(len: &Length) -> String {
    let unit = match len.unit {
        LengthUnit::Percentage => "%",
        LengthUnit::PX => "px",
        LengthUnit::EMS => "em",
        LengthUnit::EXS => "ex",
        LengthUnit::CM => "cm",
        LengthUnit::MM => "mm",
        LengthUnit::IN => "in",
        LengthUnit::PT => "pt",
        LengthUnit::PC => "pc",
        _ => "",
    };
    format!("{}{}", len.value, unit)
}

fn format_lengths(lengths: &[Length]) -> String {
    lengths.iter().map(format_length).collect::<Vec<_>>().join(" ")
}

fn format_numbers(nums: &[f32]) -> String {
    nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ")
}

/// Identity is `None` so an untransformed element reads as having no transform, which additive
/// animation relies on to know nothing is underneath.
fn format_matrix(matrix: &skia_safe::Matrix) -> Option<String> {
    if matrix.is_identity() {
        return None;
    }
    Some(format!(
        "matrix({},{},{},{},{},{})",
        matrix.scale_x(),
        matrix.skew_y(),
        matrix.skew_x(),
        matrix.scale_y(),
        matrix.translate_x(),
        matrix.translate_y()
    ))
}

fn format_points(points: &[Point]) -> String {
    points.iter().map(|p| format!("{},{}", p.x, p.y)).collect::<Vec<_>>().join(" ")
}

// Every TypedNode variant derives from TransformableNode, so the shared getters exist on all.
macro_rules! for_each_node {
    ($node:expr, |$n:ident| $body:expr) => {
        match $node {
            TypedNode::Circle($n) => $body,
            TypedNode::ClipPath($n) => $body,
            TypedNode::Defs($n) => $body,
            TypedNode::Ellipse($n) => $body,
            TypedNode::FeBlend($n) => $body,
            TypedNode::FeColorMatrix($n) => $body,
            TypedNode::FeComponentTransfer($n) => $body,
            TypedNode::FeComposite($n) => $body,
            TypedNode::FeDiffuseLighting($n) => $body,
            TypedNode::FeDisplacementMap($n) => $body,
            TypedNode::FeDistantLight($n) => $body,
            TypedNode::FeFlood($n) => $body,
            TypedNode::FeFuncA($n) => $body,
            TypedNode::FeFuncR($n) => $body,
            TypedNode::FeFuncG($n) => $body,
            TypedNode::FeFuncB($n) => $body,
            TypedNode::FeGaussianBlur($n) => $body,
            TypedNode::FeImage($n) => $body,
            TypedNode::FeMerge($n) => $body,
            TypedNode::FeMergeNode($n) => $body,
            TypedNode::FeMorphology($n) => $body,
            TypedNode::FeOffset($n) => $body,
            TypedNode::FePointLight($n) => $body,
            TypedNode::FeSpecularLighting($n) => $body,
            TypedNode::FeSpotLight($n) => $body,
            TypedNode::FeTurbulence($n) => $body,
            TypedNode::Filter($n) => $body,
            TypedNode::G($n) => $body,
            TypedNode::Image($n) => $body,
            TypedNode::Line($n) => $body,
            TypedNode::LinearGradient($n) => $body,
            TypedNode::Mask($n) => $body,
            TypedNode::Path($n) => $body,
            TypedNode::Pattern($n) => $body,
            TypedNode::Polygon($n) => $body,
            TypedNode::Polyline($n) => $body,
            TypedNode::RadialGradient($n) => $body,
            TypedNode::Rect($n) => $body,
            TypedNode::Stop($n) => $body,
            TypedNode::Svg($n) => $body,
            TypedNode::Text($n) => $body,
            TypedNode::TextLiteral($n) => $body,
            TypedNode::TextPath($n) => $body,
            TypedNode::TSpan($n) => $body,
            TypedNode::Use($n) => $body,
        }
    };
}

fn get_inherited_attribute(node: &TypedNode, name: &str) -> Option<String> {
    Some(match name {
        "opacity" => for_each_node!(node, |n| n.opacity()).unwrap_or(1.0).to_string(),
        "fill-opacity" => for_each_node!(node, |n| n.fill_opacity()).unwrap_or(1.0).to_string(),
        "stroke-width" => for_each_node!(node, |n| n.stroke_width())
            .map(|l| format_length(&l))
            .unwrap_or_default(),
        "stroke-opacity" => for_each_node!(node, |n| n.stroke_opacity()).unwrap_or(1.0).to_string(),
        "stroke-miterlimit" => for_each_node!(node, |n| n.stroke_miter_limit()).unwrap_or(4.0).to_string(),
        "transform" => return format_matrix(for_each_node!(node, |n| n.transform())),
        "fill" => for_each_node!(node, |n| n.fill())
            .and_then(|p| p.color())
            .map(canvas_2d::utils::color::to_parsed_color)
            .unwrap_or_else(|| "none".to_string()),
        _ => return None,
    })
}

fn get_specific_attribute(node: &TypedNode, name: &str) -> Option<String> {
    macro_rules! l {
        ($v:expr) => {
            format_length(&$v)
        };
    }
    Some(match node {
        TypedNode::Rect(n) => match name {
            "x" => l!(n.x()),
            "y" => l!(n.y()),
            "width" => l!(n.width()),
            "height" => l!(n.height()),
            _ => return None,
        },
        TypedNode::Circle(n) => match name {
            "cx" => l!(n.cx()),
            "cy" => l!(n.cy()),
            "r" => l!(n.r()),
            _ => return None,
        },
        TypedNode::Ellipse(n) => match name {
            "cx" => l!(n.cx()),
            "cy" => l!(n.cy()),
            _ => return None,
        },
        TypedNode::Line(n) => match name {
            "x1" => l!(n.x1()),
            "y1" => l!(n.y1()),
            "x2" => l!(n.x2()),
            "y2" => l!(n.y2()),
            _ => return None,
        },
        TypedNode::TextLiteral(n) => match name {
            "text" => n.text().to_string(),
            _ => return None,
        },
        TypedNode::Path(_) => return None, // no SkPath -> "d" string serializer available
        TypedNode::Use(n) => match name {
            "x" => l!(n.x()),
            "y" => l!(n.y()),
            "href" | "xlink:href" => format!("#{}", n.href().data()),
            _ => return None,
        },
        TypedNode::Image(n) => match name {
            "x" => l!(n.x()),
            "y" => l!(n.y()),
            "width" => l!(n.width()),
            "height" => l!(n.height()),
            "href" | "xlink:href" => format!("#{}", n.href().data()),
            _ => return None,
        },
        TypedNode::Svg(n) => match name {
            "x" => l!(n.x()),
            "y" => l!(n.y()),
            "width" => l!(n.width()),
            "height" => l!(n.height()),
            "viewBox" => {
                let b = n.view_box()?;
                format!("{} {} {} {}", b.left, b.top, b.width(), b.height())
            }
            _ => return None,
        },
        TypedNode::Stop(n) => match name {
            "offset" => l!(n.offset()),
            _ => return None,
        },
        TypedNode::LinearGradient(n) => match name {
            "x1" => l!(n.x1()),
            "y1" => l!(n.y1()),
            "x2" => l!(n.x2()),
            "y2" => l!(n.y2()),
            "href" | "xlink:href" => format!("#{}", n.as_base().href().data()),
            _ => return None,
        },
        TypedNode::RadialGradient(n) => match name {
            "cx" => l!(n.cx()),
            "cy" => l!(n.cy()),
            "r" => l!(n.r()),
            "fx" => n.fx().map(|v| format_length(&v)).unwrap_or_default(),
            "fy" => n.fy().map(|v| format_length(&v)).unwrap_or_default(),
            "href" | "xlink:href" => format!("#{}", n.as_base().href().data()),
            _ => return None,
        },
        TypedNode::Pattern(n) => match name {
            "x" => n.x().map(|v| format_length(&v)).unwrap_or_default(),
            "y" => n.y().map(|v| format_length(&v)).unwrap_or_default(),
            "width" => n.width().map(|v| format_length(&v)).unwrap_or_default(),
            "height" => n.height().map(|v| format_length(&v)).unwrap_or_default(),
            "href" | "xlink:href" => format!("#{}", n.href().data()),
            _ => return None,
        },
        TypedNode::Mask(n) => match name {
            "x" => l!(n.x()),
            "y" => l!(n.y()),
            "width" => l!(n.width()),
            "height" => l!(n.height()),
            _ => return None,
        },
        TypedNode::TextPath(n) => match name {
            "href" | "xlink:href" => format!("#{}", n.href().data()),
            "startOffset" => l!(n.start_offset()),
            _ => return None,
        },
        TypedNode::Polygon(n) | TypedNode::Polyline(n) => match name {
            "points" => format_points(n.points()),
            _ => return None,
        },
        TypedNode::Text(n) => match name {
            "x" => format_lengths(n.x()),
            "y" => format_lengths(n.y()),
            "dx" => format_lengths(n.dx()),
            "dy" => format_lengths(n.dy()),
            "rotate" => format_numbers(n.rotate()),
            _ => return None,
        },
        TypedNode::TSpan(n) => match name {
            "x" => format_lengths(n.x()),
            "y" => format_lengths(n.y()),
            "dx" => format_lengths(n.dx()),
            "dy" => format_lengths(n.dy()),
            "rotate" => format_numbers(n.rotate()),
            _ => return None,
        },
        _ => return None,
    })
}

pub fn set_attribute(node: &mut TypedNode, name: &str, value: &str) -> bool {
    if name == "id" {
        // Tracked by `SvgDocument`.
        return true;
    }
    // Skia only knows the namespaced spelling.
    let name = if name == "href" { "xlink:href" } else { name };

    // Typed setters skip side effects such as `<polygon>` rebuilding the path `onDraw` reads.
    for_each_node!(node, |n| n.set_string_attribute(name, value))
}

pub fn get_attribute(node: &TypedNode, name: &str) -> Option<String> {
    get_inherited_attribute(node, name).or_else(|| get_specific_attribute(node, name))
}
