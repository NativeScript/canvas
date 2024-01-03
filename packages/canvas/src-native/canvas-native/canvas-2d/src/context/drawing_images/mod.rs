use skia_safe::canvas::SrcRectConstraint;
use skia_safe::{Image, Rect};

use canvas_core::image_asset::ImageAsset;

use crate::context::Context;

impl Context {
    pub fn draw_image_asset_src_xywh_dst_xywh(
        &mut self,
        image: &ImageAsset,
        src_x: f32,
        src_y: f32,
        src_width: f32,
        src_height: f32,
        dst_x: f32,
        dst_y: f32,
        dst_width: f32,
        dst_height: f32,
    ) {
        let image = image.skia_image();

        if let Some(image) = image {
            self.draw_image_src_xywh_dst_xywh(
                &image, src_x, src_y, src_width, src_height, dst_x, dst_y, dst_width, dst_height,
            )
        }
    }

    pub fn draw_image_asset(
        &mut self,
        image: &ImageAsset,
        src_rect: impl Into<Rect>,
        dst_rect: impl Into<Rect>,
    ) {
        let image = image.skia_image();

        if let Some(image) = image {
            self.draw_image(&image, src_rect, dst_rect)
        }
    }

    pub fn draw_image_asset_dx_dy(&mut self, image: &ImageAsset, x: f32, y: f32) {
        let image = image.skia_image();

        if let Some(image) = image {
            let width = image.width() as f32;
            let height = image.height() as f32;

            self.draw_image_src_xywh_dst_xywh(&image, 0., 0., width, height, x, y, width, height)
        }
    }

    pub fn draw_image_asset_dx_dy_dw_dh(
        &mut self,
        image: &ImageAsset,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {
        let image = image.skia_image();

        if let Some(image) = image {
            let w = image.width() as f32;
            let h = image.height() as f32;

            self.draw_image_src_xywh_dst_xywh(&image, 0., 0., w, h, x, y, width, height)
        }
    }

    pub fn draw_image_asset_with_rect(&mut self, image: &ImageAsset, dst_rect: impl Into<Rect>) {
        let image = image.skia_image();

        if let Some(image) = image {
            self.draw_image_with_rect(&image, dst_rect)
        }
    }

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
            paint,
        );
    }

    pub fn draw_image_with_rect(&mut self, image: &Image, dst_rect: impl Into<Rect>) {
        let dst_rect = dst_rect.into();
        self.state
            .paint
            .image_smoothing_quality_set(self.state.image_filter_quality());
        let paint = self.state.paint.image_paint();
        self.surface.canvas().draw_image_rect_with_sampling_options(
            image,
            None,
            dst_rect,
            self.state.image_smoothing_quality,
            paint,
        );
    }

    pub(crate) fn draw_image_with_points(&mut self, image: &Image, x: f32, y: f32) {
        self.state
            .paint
            .image_smoothing_quality_set(self.state.image_filter_quality());
        let paint = self.state.paint.image_paint();
        self.surface.canvas().draw_image_with_sampling_options(
            image,
            (x, y),
            self.state.image_smoothing_quality,
            Some(&paint),
        );
    }
}
