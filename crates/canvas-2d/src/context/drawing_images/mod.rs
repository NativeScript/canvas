use skia_safe::{Image, Rect};
use skia_safe::canvas::SrcRectConstraint;

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
        image.with_skia_image_bytes(|image, bytes| {
            if let Some(image) = image {
                self.draw_image_src_xywh_dst_xywh(
                    &image, src_x, src_y, src_width, src_height, dst_x, dst_y, dst_width, dst_height,
                )
            } else {
                if let Some((dimensions, bytes)) = bytes {
                    let (width, height) = dimensions;
                    if let Some(image) = crate::utils::image::from_image_slice_no_copy(
                        bytes,
                        width as i32,
                        height as i32,
                    ) {
                        self.draw_image_src_xywh_dst_xywh(
                            &image, src_x, src_y, src_width, src_height, dst_x, dst_y, dst_width,
                            dst_height,
                        )
                    }
                }
            }
        });
    }

    pub fn draw_image_asset(
        &mut self,
        image: &ImageAsset,
        src_rect: impl Into<Rect>,
        dst_rect: impl Into<Rect>,
    ) {
        image.with_skia_image_bytes(|image, bytes| {
            if let Some(image) = image {
                self.draw_image(&image, src_rect, dst_rect)
            } else {
                if let Some((dimensions, bytes)) = bytes {
                    let (width, height) = dimensions;
                    if let Some(image) = crate::utils::image::from_image_slice_no_copy(
                        bytes,
                        width as i32,
                        height as i32,
                    ) {
                        self.draw_image(&image, src_rect, dst_rect)
                    }
                }
            }
        });
    }

    pub fn draw_image_asset_dx_dy(&mut self, image: &ImageAsset, x: f32, y: f32) {
        image.with_skia_image_bytes(|image, bytes| {
            if let Some(image) = image {
                self.draw_image_dx_dy(&image, x, y)
            } else {
                if let Some((dimensions, bytes)) = bytes {
                    let (width, height) = dimensions;
                    if let Some(image) = crate::utils::image::from_image_slice_no_copy(
                        bytes,
                        width as i32,
                        height as i32,
                    ) {
                        self.draw_image_dx_dy(&image, x, y)
                    }
                }
            }
        });
    }

    pub fn draw_image_asset_dx_dy_dw_dh(
        &mut self,
        image: &ImageAsset,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {
        image.with_skia_image_bytes(|image, bytes| {
            if let Some(image) = image {
                self.draw_image_dx_dy_dw_dh(&image, x, y, width, height)
            } else {
                if let Some((dimensions, bytes)) = bytes {
                    let (w, h) = dimensions;
                    if let Some(image) =
                        crate::utils::image::from_image_slice_no_copy(bytes, w as i32, h as i32)
                    {
                        self.draw_image_dx_dy_dw_dh(&image, x, y, width, height)
                    }
                }
            }
        });
    }

    pub fn draw_image_asset_with_rect(&mut self, image: &ImageAsset, dst_rect: impl Into<Rect>) {
        image.with_skia_image_bytes(|image, bytes| {
            if let Some(image) = image {
                self.draw_image_with_rect(&image, dst_rect)
            } else {
                if let Some((dimensions, bytes)) = bytes {
                    let (width, height) = dimensions;
                    if let Some(image) = crate::utils::image::from_image_slice_no_copy(
                        bytes,
                        width as i32,
                        height as i32,
                    ) {
                        self.draw_image_with_rect(&image, dst_rect)
                    }
                }
            }
        });
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

    pub fn draw_image_dx_dy(&mut self, image: &Image, x: f32, y: f32) {
        #[cfg(feature = "gl")]{
            if let Some(ref context) = self.gl_context {
                context.make_current();
            }
        }

        self.state
            .paint
            .image_smoothing_quality_set(self.state.image_filter_quality());

        let paint = self.state.paint.image_paint().clone();
        let quality = self.state.image_smoothing_quality;
        self.render_to_canvas(&paint, |canvas, paint| {
            canvas.draw_image_with_sampling_options(
                image,
                skia_safe::Point::new(x, y),
                quality,
                Some(paint),
            );
        });
    }

    pub fn draw_image_dx_dy_dw_dh(&mut self, image: &Image, dx: f32, dy: f32, dw: f32, dh: f32) {
        self.draw_image_with_rect(image, Rect::from_xywh(dx, dy, dw, dh))
    }

    fn draw_image(&mut self, image: &Image, src_rect: impl Into<Rect>, dst_rect: impl Into<Rect>) {
        #[cfg(feature = "gl")]{
            if let Some(ref context) = self.gl_context {
                context.make_current();
            }
        }

        let src_rect = src_rect.into();
        let dst_rect = dst_rect.into();

        let dimensions = image.dimensions();
        let (src, dst) = crate::utils::fit_bounds(
            dimensions.width as f32,
            dimensions.height as f32,
            src_rect,
            dst_rect,
        );

        self.state
            .paint
            .image_smoothing_quality_set(self.state.image_filter_quality());

        let paint = self.state.paint.image_paint().clone();
        let quality = self.state.image_smoothing_quality;
        self.render_to_canvas(&paint, |canvas, paint| {
            canvas.draw_image_rect_with_sampling_options(
                image,
                Some((&src, SrcRectConstraint::Strict)),
                dst,
                quality,
                paint,
            );
        });
    }

    fn draw_image_with_rect(&mut self, image: &Image, dst_rect: impl Into<Rect>) {

        #[cfg(feature = "gl")]{
            if let Some(ref context) = self.gl_context {
                context.make_current();
            }
        }

        let dimensions = image.dimensions();

        let src_rect = Rect::from_xywh(0., 0., dimensions.width as f32, dimensions.height as f32);
        let dst_rect = dst_rect.into();

        let (src, dst) = crate::utils::fit_bounds(
            dimensions.width as f32,
            dimensions.height as f32,
            src_rect,
            dst_rect,
        );

        self.state
            .paint
            .image_smoothing_quality_set(self.state.image_filter_quality());
        let paint = self.state.paint.image_paint().clone();

        let quality = self.state.image_smoothing_quality;
        self.render_to_canvas(&paint, |canvas, paint| {
            canvas.draw_image_rect_with_sampling_options(
                image,
                Some((&src, SrcRectConstraint::Strict)),
                dst,
                quality,
                paint,
            );
        });
    }

    pub(crate) fn draw_image_with_points(&mut self, image: &Image, x: f32, y: f32) {
        self.draw_image_with_rect(
            image,
            Rect::from_xywh(x, y, image.width() as f32, image.height() as f32),
        )
    }
}
