use skia_safe::ImageFilter;
use std::ops::Deref;

use crate::common::context::Context;
use crate::common::svg::{HEIGHT_ATTR, STD_DEVIATION_ATTR, WIDTH_ATTR, X_ATTR, Y_ATTR};

pub fn handle_filters<'a>(context: &mut Context, filter: &roxmltree::Node, root: &'a roxmltree::Document<'a>) -> String {
    let mut x = "0";
    let mut y = "0";
    let mut width = "100%";
    let mut height = "100%";

    if let Some(val) = filter.attribute(X_ATTR) {
        x = val;
    }

    if let Some(val) = filter.attribute(Y_ATTR) {
        y = val;
    }


    if let Some(val) = filter.attribute(WIDTH_ATTR) {
        width = val;
    }

    if let Some(val) = filter.attribute(HEIGHT_ATTR) {
        height = val;
    }
    let mut image_filter = String::new();
    for node in filter.children().into_iter() {
        match node.tag_name().name() {
            FE_GAUSSIAN_BLUR_NS => {
                if let Some(val) = node.attribute(STD_DEVIATION_ATTR) {
                    let blur = format!("blur({}px);", val);
                    image_filter.push_str(&blur);
                }
            }
            _ => {}
        }
    }

    image_filter
}