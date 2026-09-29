use std::os::raw::{c_float, c_int};

use skia_safe::{AlphaType, ColorType, IPoint, ISize, IVector, ImageInfo};

pub use image_data::*;

use crate::context::Context;

pub mod image_data;

impl Context {
    pub fn create_image_data(width: c_int, height: c_int) -> ImageData {
        ImageData::new(width, height)
    }

    pub fn get_image_data(
        &mut self,
        sx: c_float,
        sy: c_float,
        sw: c_float,
        sh: c_float,
    ) -> ImageData {
        #[cfg(feature = "gl")]{
            if let Some(ref context) = self.gl_context {
                context.make_current();
            }
        }

        let info = ImageInfo::new(
            ISize::new(sw as i32, sh as i32),
            ColorType::RGBA8888,
            AlphaType::Unpremul,
            None,
        );
        let row_bytes = info.width() * 4;
        let mut slice = bytes::BytesMut::zeroed((row_bytes * info.height()) as usize);

        self.flush();

        let _ = self.surface.read_pixels(
            &info,
            slice.as_mut(),
            row_bytes as usize,
            IPoint::new(sx as i32, sy as i32)
        );


        ImageData::new_with_buffer(info.width(), info.height(), slice)
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

        #[cfg(feature = "gl")]{
            if let Some(ref context) = self.gl_context {
                context.make_current();
            }
        }

        let (mut x, mut y, mut w, mut h) = (sx, sy, sw, sh);
        if w < 0. {
            x += w;
            w = -w;
        }
        if h < 0. {
            y += h;
            h = -h;
        }
        if x < 0. {
            w += x;
            x = 0.;
        }
        if y < 0. {
            h += y;
            y = 0.;
        }
        w = w.min(data.width() as f32 - x);
        h = h.min(data.height() as f32 - y);
        let (x, y, w, h) = (x.floor() as i32, y.floor() as i32, w.floor() as i32, h.floor() as i32);
        if w <= 0 || h <= 0 {
            return;
        }

        let info = ImageInfo::new(
            ISize::new(w, h),
            ColorType::RGBA8888,
            AlphaType::Unpremul,
            None,
        );
        let row_bytes = data.width() as usize * 4;
        let start = y as usize * row_bytes + x as usize * 4;
        let pixels = &data.data()[start..];
        let (dx, dy) = (dx + x as f32, dy + y as f32);

        let origin = IVector::new(dx as i32, dy as i32);
        if self.record_pixels(&info, pixels, row_bytes, origin) {
            self.surface_state = self.surface_state | crate::context::SurfaceState::Pending;
            return;
        }
        self.with_canvas_dirty(|canvas| {
            let _ = canvas.write_pixels(&info, pixels, row_bytes, origin);
        });
    }
}
