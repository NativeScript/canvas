use skia_safe::canvas::SrcRectConstraint;
use skia_safe::{Image, Rect};

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
        self.state
            .paint
            .image_smoothing_quality_set(self.state.image_filter_quality());
        let mut paint = self.state.paint.image_paint().clone();
        self.surface.canvas().draw_image_rect_with_sampling_options(
            image,
            Some((&src_rect, SrcRectConstraint::Strict)),
            dst_rect,
            skia_safe::SamplingOptions::from_filter_quality(
                self.state.image_smoothing_quality.into(),
                None,
            ),
            &paint,
        );
    }

    pub fn draw_image_with_rect(&mut self, image: &Image, dst_rect: impl Into<Rect>) {
        let mut dst_rect = dst_rect.into();
        self.state
            .paint
            .image_smoothing_quality_set(self.state.image_filter_quality());
        let paint = self.state.paint.image_paint().clone();
        self.surface.canvas().draw_image_rect_with_sampling_options(
            image,
            None,
            dst_rect,
            skia_safe::SamplingOptions::from_filter_quality(
                self.state.image_smoothing_quality.into(),
                None,
            ),
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
            skia_safe::SamplingOptions::from_filter_quality(
                self.state.image_smoothing_quality.into(),
                None,
            ),
            Some(&paint),
        );
    }
}
