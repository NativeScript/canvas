use skia_safe::{
    AlphaType, Color, ColorType, EncodedImageFormat, ImageInfo, ISize, PixelGeometry, Rect, Surface,
};
use skia_safe::gpu::gl::Interface;
use skia_safe::image::CachingHint;

use crate::context::{Context, Device, State};
use crate::context::paths::path::Path;
use crate::context::text_styles::text_direction::TextDirection;

const GR_GL_RGB565: u32 = 0x8D62;
const GR_GL_RGBA8: u32 = 0x8058;

#[cfg(feature = "gl")]
impl Context {
    pub fn new_gl(
        width: f32,
        height: f32,
        density: f32,
        buffer_id: i32,
        samples: i32,
        alpha: bool,
        font_color: i32,
        ppi: f32,
        direction: TextDirection,
    ) -> Self {
        let device = Device {
            width,
            height,
            density,
            non_gpu: false,
            samples: samples as usize,
            alpha,
            ppi,
        };
        let interface = Interface::new_native();
        let mut ctx = skia_safe::gpu::DirectContext::new_gl(interface, None).unwrap();
        let mut frame_buffer = skia_safe::gpu::gl::FramebufferInfo::from_fboid(buffer_id as u32);
        if alpha {
            frame_buffer.format = GR_GL_RGBA8;
        } else {
            frame_buffer.format = GR_GL_RGB565;
        }

        let target = skia_safe::gpu::BackendRenderTarget::new_gl(
            (width as i32, height as i32),
            Some(samples as usize),
            8,
            frame_buffer,
        );
        let surface_props = skia_safe::SurfaceProps::new(
            skia_safe::SurfacePropsFlags::default(),
            PixelGeometry::Unknown,
        );
        let mut color_type = ColorType::RGBA8888;
        if !alpha {
            color_type = ColorType::RGB565;
        }
        let surface_holder = Surface::from_backend_render_target(
            &mut ctx,
            &target,
            skia_safe::gpu::SurfaceOrigin::BottomLeft,
            color_type,
            None,
            Some(&surface_props),
        );

        Context {
            surface: surface_holder.unwrap(),
            path: Path::default(),
            state: State::from_device(device, direction),
            state_stack: vec![],
            font_color: Color::new(font_color as u32),
            device,
        }
    }

    pub fn resize_gl(
        context: &mut Context,
        width: f32,
        height: f32,
        density: f32,
        buffer_id: i32,
        samples: i32,
        alpha: bool,
        ppi: f32,
    ) {
        let interface = Interface::new_native();
        let ctx = skia_safe::gpu::DirectContext::new_gl(interface, None);
        if ctx.is_none() {
            return;
        }
        let mut ctx = ctx.unwrap();
        ctx.reset(None);
        let device = Device {
            width,
            height,
            density,
            non_gpu: false,
            samples: samples as usize,
            alpha,
            ppi,
        };
        let mut frame_buffer = skia_safe::gpu::gl::FramebufferInfo::from_fboid(buffer_id as u32);

        if alpha {
            frame_buffer.format = GR_GL_RGBA8;
        } else {
            frame_buffer.format = GR_GL_RGB565;
        }

        let target = skia_safe::gpu::BackendRenderTarget::new_gl(
            (width as i32, height as i32),
            Some(samples as usize),
            8,
            frame_buffer,
        );
        let surface_props = skia_safe::SurfaceProps::new(
            skia_safe::SurfacePropsFlags::default(),
            PixelGeometry::Unknown,
        );
        let mut color_type = ColorType::RGBA8888;

        if !alpha {
            color_type = ColorType::RGB565;
        }

        if let Some(surface) = Surface::from_backend_render_target(
            &mut ctx,
            &target,
            skia_safe::gpu::SurfaceOrigin::BottomLeft,
            color_type,
            None,
            Some(&surface_props),
        ) {
            context.surface = surface;
            context.device = device;
            context.path = Path::default();
            context.reset_state();
        }
    }
}
