use skia_safe::gpu::gl::Interface;
use skia_safe::{gpu, surfaces, AlphaType, Color, ColorType, ISize, ImageInfo, PixelGeometry};

use crate::context::paths::path::Path;
use crate::context::text_styles::text_direction::TextDirection;
use crate::context::{Context, Device, State};

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
        let device = Device::new(width, height, density, samples as usize, alpha, ppi);
        let surface = if device.is_np {
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

            let info = ImageInfo::new(ISize::new(1, 1), color_type, alpha_type, None);

            surfaces::raster(&info, None, None).unwrap()
        } else {
            let interface = Interface::new_native();
            let mut ctx = gpu::DirectContext::new_gl(interface.unwrap(), None).unwrap();

            ctx.reset(None);

            let mut frame_buffer = gpu::gl::FramebufferInfo::from_fboid(buffer_id as u32);
            if alpha {
                frame_buffer.format = GR_GL_RGBA8;
            } else {
                frame_buffer.format = GR_GL_RGB565;
            }

            let target = gpu::backend_render_targets::make_gl(
                (width as i32, height as i32),
                Some(samples as usize),
                0,
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
            gpu::surfaces::wrap_backend_render_target(
                &mut ctx,
                &target,
                gpu::SurfaceOrigin::BottomLeft,
                color_type,
                None,
                Some(&surface_props),
            )
            .unwrap()
        };

        let mut state = State::from_device(device, direction);

        Context {
            surface,
            path: Path::default(),
            state,
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
        let device = Device::new(width, height, density, samples as usize, alpha, ppi);

        let surface = if device.is_np {
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

            surfaces::raster(&info, None, None)
        } else {
            let interface = Interface::new_native();
            let ctx = gpu::DirectContext::new_gl(interface.unwrap(), None);
            if ctx.is_none() {
                return;
            }
            let mut ctx = ctx.unwrap();
            ctx.reset(None);

            let mut frame_buffer = gpu::gl::FramebufferInfo::from_fboid(buffer_id as u32);

            if alpha {
                frame_buffer.format = GR_GL_RGBA8;
            } else {
                frame_buffer.format = GR_GL_RGB565;
            }

            let target = gpu::backend_render_targets::make_gl(
                (width as i32, height as i32),
                Some(samples as usize),
                0,
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

            gpu::surfaces::wrap_backend_render_target(
                &mut ctx,
                &target,
                gpu::SurfaceOrigin::BottomLeft,
                color_type,
                None,
                Some(&surface_props),
            )
        };

        if let Some(surface) = surface {
            context.device = device;
            context.path = Path::default();
            context.reset_state();
            context.surface = surface;
        }
    }
}
