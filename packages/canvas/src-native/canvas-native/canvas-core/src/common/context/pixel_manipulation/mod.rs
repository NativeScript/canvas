use std::os::raw::{c_float, c_int};

use skia_safe::{AlphaType, Bitmap, ColorType, Image, ImageInfo, IPoint, ISize, IVector, Point, Rect};

use crate::common::context::Context;
use crate::common::context::pixel_manipulation::image_data::ImageData;

pub mod image_data;

impl Context {
    pub fn create_image_data(width: c_int, height: c_int) -> ImageData {
        ImageData::new(width, height)
    }

    pub fn get_image_data(&mut self, sx: c_float, sy: c_float, sw: c_float, sh: c_float) -> ImageData {
        let info = ImageInfo::new_n32_premul(ISize::new(sw as i32, sh as i32), None);
        let row_bytes = info.width() * 4;
        let mut slice = vec![255u8; (row_bytes * info.height()) as usize];
        let _ = self.surface.canvas().read_pixels(
            &info,
            slice.as_mut_slice(),
            row_bytes as usize,
            IPoint::new(sx as i32, sy as i32),
        );
        let mut image_data = ImageData::new(info.width(), info.height());
        image_data.set_data(slice);
        image_data
    }

    pub fn put_image_data(
        &mut self,
        data: &ImageData,
        dx: c_float,
        dy: c_float,
        sx: c_float,
        sy: c_float,
        sw: c_float,
        sh: c_float,
    ) {
        let mut dx = dx;
        let mut dy = dy;
        let mut sx = sx;
        let mut sy = sy;
        let mut sw = sw;
        let mut sh = sh;
        let srect: Rect = Rect::from_xywh(sx, sy, sw, sh);
        let info: ImageInfo;
        let row_bytes: usize;
        if srect.is_empty() {
            info = ImageInfo::new(
                ISize::new(data.width(), data.height()),
                ColorType::RGBA8888,
                AlphaType::Unpremul,
                None,
            );
            row_bytes = (data.width() * 4) as usize;
        } else {
            if sw < 0.0 {
                sx += sw;
                sw = -sw;
            }

            if sy < 0.0 {
                sy += sh;
                sh = -sh;
            }

            if sx + sw > data.width() as f32 {
                sw = data.width() as f32 - sx;
            }
            if sy + sh > data.height() as f32 {
                sh = data.height() as f32 - sy;
            }

            dx += sx;
            dy += sy;

            info = ImageInfo::new(
                ISize::new(sw as i32, sh as i32),
                ColorType::RGBA8888,
                AlphaType::Unpremul,
                None,
            );

            row_bytes = (sw * 4.0) as usize;
        }
        let _ = self.surface
            .canvas()
            .write_pixels(&info, &data.data(), row_bytes, IVector::new(dx as i32, dy as i32));
    }
}