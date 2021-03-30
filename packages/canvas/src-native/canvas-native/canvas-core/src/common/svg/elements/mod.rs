pub mod element_names;
pub mod g;
pub mod gradients;
pub mod image;
pub mod mask;
pub mod parser;
pub mod renderer;
pub mod shapes;
pub mod svg;
pub mod clip_path;
pub mod filter;
pub mod use_element;
pub mod text;
pub mod pattern;
pub mod reference_element;
pub mod symbol;
pub mod marker;

pub mod prelude {
    pub use crate::common::svg::elements::parser::*;
}