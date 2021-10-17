use roxmltree::Node;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use crate::common::svg::attribute_names::{Attribute, NodeExt};
use crate::common::svg::elements::element_names::ElementName;
use crate::common::svg::elements::parser::StyleMap;

pub trait ReferenceElement: Sized {
    fn element_type() -> ElementName;
    fn from_node(node: (Option<Node>, Option<Node>)) -> Option<Self>;
    fn from_id_with_style(val: &str, node: Node, style: Option<&mut StyleMap>, overwrite: bool) -> Option<Self> {
        let nodes = match style {
            None => node.get_node(val),
            Some(style) => node.get_node_with_style(val, Some(style), overwrite)
        };
        match nodes.0.as_ref() {
            Some(node) => {
                if let Some(name) = ElementName::from_str(node.tag_name().name()) {
                    if name == Self::element_type() {
                        return match nodes {
                            (Some(node), Some(node_ref)) => {
                                Self::from_node((Some(node), Some(node_ref)))
                            }
                            (Some(node), None) => Self::from_node((Some(node), None)),
                            _ => None,
                        };
                    }
                }
                None
            }
            _ => None,
        }
    }
    fn from_id(val: &str, node: Node) -> Option<Self> {
        Self::from_id_with_style(val, node, None, true)
    }
}
