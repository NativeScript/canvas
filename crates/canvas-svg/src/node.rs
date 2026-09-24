use skia_safe::svg::{
    Circle, ClipPath, Container, Defs, Ellipse, Filter, G, Image, LinearGradient, Line, Mask,
    Path, Poly, RadialGradient, Rect, Stop, Svg, Text, TextLiteral, TextPath, TSpan, TypedNode,
    Use,
};

/// `<symbol>` has no Skia node type, so it becomes a `<g>`. Filter primitives aren't covered yet.
pub fn create_element(tag: &str) -> Option<TypedNode> {
    Some(match tag.to_ascii_lowercase().as_str() {
        "svg" => TypedNode::Svg(Svg::default()),
        "g" => TypedNode::G(G::default()),
        "symbol" => TypedNode::G(G::default()),
        "rect" => TypedNode::Rect(Rect::default()),
        "circle" => TypedNode::Circle(Circle::default()),
        "ellipse" => TypedNode::Ellipse(Ellipse::default()),
        "line" => TypedNode::Line(Line::default()),
        "polygon" => TypedNode::Polygon(Poly::polygon()),
        "polyline" => TypedNode::Polyline(Poly::polyline()),
        "path" => TypedNode::Path(Path::default()),
        "use" => TypedNode::Use(Use::default()),
        "image" => TypedNode::Image(Image::default()),
        "text" => TypedNode::Text(Text::default()),
        "tspan" => TypedNode::TSpan(TSpan::default()),
        "textpath" => TypedNode::TextPath(TextPath::default()),
        "defs" => TypedNode::Defs(Defs::default()),
        "clippath" => TypedNode::ClipPath(ClipPath::default()),
        "mask" => TypedNode::Mask(Mask::default()),
        // "pattern": needs rust-skia to re-export pattern::Pattern (private in the fork).
        "lineargradient" => TypedNode::LinearGradient(LinearGradient::default()),
        "radialgradient" => TypedNode::RadialGradient(RadialGradient::default()),
        "stop" => TypedNode::Stop(Stop::default()),
        "filter" => TypedNode::Filter(Filter::default()),
        _ => return None,
    })
}

/// SVG character data, e.g. text inside `<text>`/`<tspan>`.
pub fn create_text_node(text: &str) -> TypedNode {
    let mut node = TextLiteral::default();
    node.set_text(text);
    TypedNode::TextLiteral(node)
}

/// Inverse of `create_element`.
pub fn tag_name(node: &TypedNode) -> &'static str {
    match node {
        TypedNode::Circle(_) => "circle",
        TypedNode::ClipPath(_) => "clipPath",
        TypedNode::Defs(_) => "defs",
        TypedNode::Ellipse(_) => "ellipse",
        TypedNode::FeBlend(_) => "feBlend",
        TypedNode::FeColorMatrix(_) => "feColorMatrix",
        TypedNode::FeComponentTransfer(_) => "feComponentTransfer",
        TypedNode::FeComposite(_) => "feComposite",
        TypedNode::FeDiffuseLighting(_) => "feDiffuseLighting",
        TypedNode::FeDisplacementMap(_) => "feDisplacementMap",
        TypedNode::FeDistantLight(_) => "feDistantLight",
        TypedNode::FeFlood(_) => "feFlood",
        TypedNode::FeFuncA(_) => "feFuncA",
        TypedNode::FeFuncR(_) => "feFuncR",
        TypedNode::FeFuncG(_) => "feFuncG",
        TypedNode::FeFuncB(_) => "feFuncB",
        TypedNode::FeGaussianBlur(_) => "feGaussianBlur",
        TypedNode::FeImage(_) => "feImage",
        TypedNode::FeMerge(_) => "feMerge",
        TypedNode::FeMergeNode(_) => "feMergeNode",
        TypedNode::FeMorphology(_) => "feMorphology",
        TypedNode::FeOffset(_) => "feOffset",
        TypedNode::FePointLight(_) => "fePointLight",
        TypedNode::FeSpecularLighting(_) => "feSpecularLighting",
        TypedNode::FeSpotLight(_) => "feSpotLight",
        TypedNode::FeTurbulence(_) => "feTurbulence",
        TypedNode::Filter(_) => "filter",
        TypedNode::G(_) => "g",
        TypedNode::Image(_) => "image",
        TypedNode::Line(_) => "line",
        TypedNode::LinearGradient(_) => "linearGradient",
        TypedNode::Mask(_) => "mask",
        TypedNode::Path(_) => "path",
        TypedNode::Pattern(_) => "pattern",
        TypedNode::Polygon(_) => "polygon",
        TypedNode::Polyline(_) => "polyline",
        TypedNode::RadialGradient(_) => "radialGradient",
        TypedNode::Rect(_) => "rect",
        TypedNode::Stop(_) => "stop",
        TypedNode::Svg(_) => "svg",
        TypedNode::Text(_) => "text",
        TypedNode::TextLiteral(_) => "#text",
        TypedNode::TextPath(_) => "textPath",
        TypedNode::TSpan(_) => "tspan",
        TypedNode::Use(_) => "use",
    }
}

/// Filter primitives excluded for now.
pub(crate) fn as_container_mut(node: &mut TypedNode) -> Option<&mut Container> {
    Some(match node {
        TypedNode::ClipPath(n) => n.as_base_mut(),
        TypedNode::Defs(n) => n.as_base_mut(),
        TypedNode::Filter(n) => n.as_base_mut(),
        TypedNode::G(n) => n.as_base_mut(),
        TypedNode::Image(n) => n.as_base_mut(),
        TypedNode::LinearGradient(n) => n.as_base_mut().as_base_mut(),
        TypedNode::RadialGradient(n) => n.as_base_mut().as_base_mut(),
        TypedNode::Mask(n) => n.as_base_mut(),
        TypedNode::Pattern(n) => n.as_base_mut(),
        TypedNode::Stop(n) => n.as_base_mut(),
        TypedNode::Svg(n) => n.as_base_mut(),
        TypedNode::Text(n) => n.as_base_mut().as_base_mut(),
        TypedNode::TSpan(n) => n.as_base_mut().as_base_mut(),
        TypedNode::TextPath(n) => n.as_base_mut().as_base_mut(),
        _ => return None,
    })
}

/// Appends in place via Skia's `SkSVGContainer::appendChild`.
pub fn append_child(parent: &mut TypedNode, child: TypedNode) -> Result<(), &'static str> {
    match as_container_mut(parent) {
        Some(container) => {
            container.append_child(child.into_node());
            Ok(())
        }
        None => Err("this element cannot contain children"),
    }
}

/// Skia keeps these elements' children in a private list, not `SkSVGContainer`'s. The fork types
/// them as containers, which is only safe for the virtual `appendChild` and the transform.
fn is_text_container(node: &TypedNode) -> bool {
    matches!(
        node,
        TypedNode::Text(_) | TypedNode::TSpan(_) | TypedNode::TextPath(_)
    )
}

/// A text node's own text; `None` for anything else.
pub fn text(node: &TypedNode) -> Option<String> {
    match node {
        TypedNode::TextLiteral(literal) => Some(literal.text().to_owned()),
        _ => None,
    }
}

/// Rewrites a text node's text. False for anything that is not a text node.
pub fn set_text(node: &mut TypedNode, value: &str) -> bool {
    match node {
        TypedNode::TextLiteral(literal) => {
            literal.set_text(value);
            true
        }
        _ => false,
    }
}

/// Mirrors the native child order because `SkSVGContainer` has no removal API.
/// `children` holds cloned ref-counted handles, not ownership.
pub struct SvgElementHandle {
    pub node: TypedNode,
    pub children: Vec<TypedNode>,
}

impl SvgElementHandle {
    pub fn new(node: TypedNode) -> Self {
        Self {
            node,
            children: Vec::new(),
        }
    }

    /// Clones the ref-counted node; `child` keeps its own independent lifetime.
    pub fn append_child(&mut self, child: &SvgElementHandle) -> Result<(), &'static str> {
        append_child(&mut self.node, child.node.clone())?;
        self.children.push(child.node.clone());
        Ok(())
    }

    /// The mirror only holds cloned handles, so the caller wraps the result fresh.
    pub fn remove_child(&mut self, index: usize) -> Result<TypedNode, &'static str> {
        if index >= self.children.len() {
            return Err("index out of range");
        }
        if is_text_container(&self.node) {
            // No removal exists for text children; a hidden fragment renders as if gone.
            let mut child = self.children.remove(index);
            crate::attr::set_attribute(&mut child, "display", "none");
            return Ok(child);
        }
        match as_container_mut(&mut self.node) {
            Some(container) => {
                container.remove_child(index);
                Ok(self.children.remove(index))
            }
            None => Err("this element cannot contain children"),
        }
    }
}

pub(crate) fn transform_of(node: &TypedNode) -> Option<skia_safe::Matrix> {
    let mut owned = node.clone();
    as_container_mut(&mut owned)
        .map(|container| *container.as_base().transform())
        .or_else(|| match &mut owned {
            TypedNode::Circle(n) => Some(*n.as_base().as_base().transform()),
            TypedNode::Ellipse(n) => Some(*n.as_base().as_base().transform()),
            TypedNode::Line(n) => Some(*n.as_base().as_base().transform()),
            TypedNode::Path(n) => Some(*n.as_base().as_base().transform()),
            TypedNode::Polygon(n) | TypedNode::Polyline(n) => Some(*n.as_base().as_base().transform()),
            TypedNode::Rect(n) => Some(*n.as_base().as_base().transform()),
            _ => None,
        })
}

pub(crate) fn set_transform_of(node: &mut TypedNode, matrix: &skia_safe::Matrix) {
    if let Some(container) = as_container_mut(node) {
        container.as_base_mut().set_transform(matrix);
        return;
    }
    match node {
        TypedNode::Circle(n) => n.as_base_mut().as_base_mut().set_transform(matrix),
        TypedNode::Ellipse(n) => n.as_base_mut().as_base_mut().set_transform(matrix),
        TypedNode::Line(n) => n.as_base_mut().as_base_mut().set_transform(matrix),
        TypedNode::Path(n) => n.as_base_mut().as_base_mut().set_transform(matrix),
        TypedNode::Polygon(n) | TypedNode::Polyline(n) => n.as_base_mut().as_base_mut().set_transform(matrix),
        TypedNode::Rect(n) => n.as_base_mut().as_base_mut().set_transform(matrix),
        _ => {}
    }
}
