use skia_safe::{Image, Point, Rect};
use skia_safe::canvas::SrcRectConstraint;

use crate::common::context::Context;

impl Context {
    pub fn draw_image(
        &mut self,
        image: &Image,
        src_rect: impl Into<Rect>,
        dst_rect: impl Into<Rect>,
    ) {
        let mut src_rect = src_rect.into();
        let mut dst_rect = dst_rect.into();
        let paint = self.state.paint.image_paint().clone();
        self.surface.canvas().draw_image_rect(
            image,
            Some((&src_rect, SrcRectConstraint::Strict)),
            dst_rect,
            &paint,
        );
    }
}

