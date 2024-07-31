#![allow(dead_code)]

use std::sync::Arc;

use skia_safe::{surfaces, AlphaType, Color, ColorType, ISize, ImageInfo, PictureRecorder, Rect};

use crate::context::paths::path::Path;
use crate::context::text_styles::text_direction::TextDirection;
use crate::context::{Context, Recorder, State, SurfaceData};

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
        let mut state = State::default();
        state.direction = direction;

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

        let info = ImageInfo::new(
            ISize::new(width as i32, height as i32),
            color_type,
            alpha_type,
            None,
        );

        let surface = surfaces::raster(&info, None, None).unwrap();
        let bounds = Rect::from_wh(width, height);
        let recorder = Recorder::new(bounds);

        Context {
            direct_context: None,
            #[cfg(feature = "vulkan")]
            ash_graphics: None,
            #[cfg(feature = "vulkan")]
            vk_surface: None,
            surface_data: SurfaceData {
                bounds,
                ppi,
                scale: density,
            },
            surface,
            recorder: Arc::new(parking_lot::Mutex::new(recorder)),
            path: Path::default(),
            state,
            state_stack: vec![],
            font_color: Color::new(font_color as u32),
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

        let bounds = Rect::from_wh(width, height);
        let info = if bounds.is_empty() {
            ImageInfo::new(ISize::new(1, 1), color_type, alpha_type, None)
        } else {
            ImageInfo::new(
                ISize::new(width as i32, height as i32),
                color_type,
                alpha_type,
                None,
            )
        };

        context.path = Path::default();
        context.reset_state();

        if let Some(surface) = surfaces::raster(&info, None, None) {
            context.surface = surface;
            context.surface_data.bounds = bounds;
            let mut lock = context.recorder.lock();
            lock.current = PictureRecorder::new();
            lock.cache = None;
            lock.is_dirty = false;
            lock.layers.clear();
            context.surface_data.scale = density;
            context.surface_data.ppi = ppi;
        }
    }

    pub fn flush_buffer(context: &mut Context, width: i32, height: i32, buffer: &mut [u8]) {
        if context.surface_data.bounds.is_empty() {
            return;
        }
        let info = ImageInfo::new(
            ISize::new(width, height),
            ColorType::RGBA8888,
            AlphaType::Unknown,
            None,
        );
        let mut surface = surfaces::wrap_pixels(&info, buffer, None, None).unwrap();
        let mut paint = skia_safe::Paint::default();
        {
            let canvas = surface.canvas();
            paint.set_anti_alias(true);
            paint.set_style(skia_safe::PaintStyle::Fill);
            paint.set_blend_mode(skia_safe::BlendMode::Clear);
            canvas.draw_rect(
                Rect::from_xywh(0f32, 0f32, width as f32, height as f32),
                &paint,
            );
        }

        let mut lock = context.recorder.lock();
        if let Some(image) = lock.get_image() {
            surface
                .canvas()
                .draw_image(image, skia_safe::Point::default(), Some(&paint));
        }
    }
}
