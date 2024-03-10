#![allow(dead_code)]

use skia_safe::{surfaces, AlphaType, Color, ColorType, ISize, ImageInfo, Rect};

use crate::context::paths::path::Path;
use crate::context::text_styles::text_direction::TextDirection;
use crate::context::{Context, Device, State};

const GR_GL_RGB565: u32 = 0x8D62;
const GR_GL_RGBA8: u32 = 0x8058;

impl Context {
    pub fn new(
        width: f32,
        height: f32,
        density: f32,
        alpha: bool,
        font_color: i32,
        ppi: f32,
        direction: TextDirection,
    ) -> Self {
        let device = Device::new_non_gpu(width, height, density, alpha, ppi);

        let mut state = State::from_device(device, direction);

        let color_type = if alpha {
            ColorType::RGBA8888
        } else {
            ColorType::RGB565
        };

        let alpha_type = if alpha {
            AlphaType::Unpremul
        } else {
            AlphaType::Premul
        };

        let info = if device.is_np {
            ImageInfo::new(ISize::new(1, 1), color_type, alpha_type, None)
        } else {
            ImageInfo::new(
                ISize::new(width as i32, height as i32),
                color_type,
                alpha_type,
                None,
            )
        };

        let surface = surfaces::raster(&info, None, None).unwrap();

        Context {
            surface,
            path: Path::default(),
            state,
            state_stack: vec![],
            font_color: Color::new(font_color as u32),
            device,
        }
    }

    pub fn resize(
        context: &mut Context,
        width: f32,
        height: f32,
        density: f32,
        alpha: bool,
        ppi: f32,
    ) {
        let device = Device::new_non_gpu(width, height, density, alpha, ppi);

        let color_type = if alpha {
            ColorType::RGBA8888
        } else {
            ColorType::RGB565
        };

        let alpha_type = if alpha {
            AlphaType::Unpremul
        } else {
            AlphaType::Premul
        };

        let info = if device.is_np {
            ImageInfo::new(ISize::new(1, 1), color_type, alpha_type, None)
        } else {
            ImageInfo::new(
                ISize::new(width as i32, height as i32),
                color_type,
                alpha_type,
                None,
            )
        };

        context.device = device;
        context.path = Path::default();
        context.reset_state();

        if let Some(surface) = surfaces::raster(&info, None, None) {
            context.surface = surface;
        }
    }

    pub fn flush_buffer(context: &mut Context, width: i32, height: i32, buffer: &mut [u8]) {
        if context.device.is_np {
            return;
        }
        let info = ImageInfo::new(
            ISize::new(width, height),
            ColorType::RGBA8888,
            AlphaType::Unknown,
            None,
        );
        let mut surface = surfaces::wrap_pixels(&info, buffer, None, None).unwrap();
        let canvas = surface.canvas();
        let mut paint = skia_safe::Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::PaintStyle::Fill);
        paint.set_blend_mode(skia_safe::BlendMode::Clear);
        canvas.draw_rect(
            Rect::from_xywh(0f32, 0f32, width as f32, height as f32),
            &paint,
        );
        context.draw_on_surface(&mut surface);
    }
}
