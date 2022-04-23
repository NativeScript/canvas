use skia_safe::canvas::SrcRectConstraint;
use skia_safe::{Image, Rect};

use crate::context::Context;

impl Context {
    pub fn draw_image_src_xywh_dst_xywh(
        &mut self,
        image: &Image,
        src_x: f32,
        src_y: f32,
        src_width: f32,
        src_height: f32,
        dst_x: f32,
        dst_y: f32,
        dst_width: f32,
        dst_height: f32,
    ) {
        self.draw_image(
            image,
            Rect::from_xywh(src_x, src_y, src_width, src_height),
            Rect::from_xywh(dst_x, dst_y, dst_width, dst_height),
        )
    }

    pub fn draw_image(
        &mut self,
        image: &Image,
        src_rect: impl Into<Rect>,
        dst_rect: impl Into<Rect>,
    ) {
        let src_rect = src_rect.into();
        let dst_rect = dst_rect.into();
        self.state
            .paint
            .image_smoothing_quality_set(self.state.image_filter_quality());
        let paint = self.state.paint.image_paint();
        self.surface.canvas().draw_image_rect_with_sampling_options(
            image,
            Some((&src_rect, SrcRectConstraint::Strict)),
            dst_rect,
            self.state.image_smoothing_quality,
            &paint,
        );
    }

    pub fn draw_image_with_rect(&mut self, image: &Image, dst_rect: impl Into<Rect>) {
        let dst_rect = dst_rect.into();
        self.state
            .paint
            .image_smoothing_quality_set(self.state.image_filter_quality());
        let paint = self.state.paint.image_paint().clone();
        self.surface.canvas().draw_image_rect_with_sampling_options(
            image,
            None,
            dst_rect,
            self.state.image_smoothing_quality,
            &paint,
        );
    }

    pub(crate) fn draw_image_with_points(&mut self, image: &Image, x: f32, y: f32) {
        self.state
            .paint
            .image_smoothing_quality_set(self.state.image_filter_quality());
        let paint = self.state.paint.image_paint().clone();
        self.surface.canvas().draw_image_with_sampling_options(
            image,
            (x, y),
            self.state.image_smoothing_quality,
            Some(&paint),
        );
    }
}
