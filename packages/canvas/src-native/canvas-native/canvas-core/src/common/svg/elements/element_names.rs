use phf::phf_map;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ElementName {
    Svg,
    Rect,
    Circle,
    Ellipse,
    Line,
    Polygon,
    Polyline,
    Path,
    Text,
    G,
    Defs,
    Use,
    Image,
    LinearGradient,
    RadialGradient,
    Stop,
    Pattern,
    Symbol,
    Filter,
    Mask,
    FeGaussianBlur,
    ClipPath,
    Marker,
}

static ELEMENT_NAMES_MAP: phf::Map<&'static str, ElementName> = phf_map! {
"svg" => ElementName::Svg,
"rect" => ElementName::Rect ,
"circle" => ElementName::Circle ,
"ellipse" => ElementName::Ellipse ,
"line" => ElementName::Line ,
"polygon" => ElementName::Polygon ,
"polyline" => ElementName::Polyline ,
"path" => ElementName::Path ,
"text" => ElementName::Text ,
"g" => ElementName::G ,
"defs" => ElementName::Defs ,
"image" => ElementName::Image ,
"linearGradient" => ElementName::LinearGradient ,
"radialGradient" => ElementName::RadialGradient ,
"stop" => ElementName::Stop ,
"pattern" => ElementName::Pattern ,
"symbol" => ElementName::Symbol ,
"filter" => ElementName::Filter ,
"mask" => ElementName::Mask ,
"feGaussianBlur" => ElementName::FeGaussianBlur,
"use" => ElementName::Use,
"clipPath" => ElementName::ClipPath,
"marker" => ElementName::Marker
};

impl ElementName {
    pub fn from_str(text: &str) -> Option<Self> {
        ELEMENT_NAMES_MAP.get(text).cloned()
    }

    pub fn to_str(&self) -> &'static str {
        ELEMENT_NAMES_MAP
            .entries()
            .find(|kv| kv.1 == self)
            .unwrap()
            .0
    }
}
