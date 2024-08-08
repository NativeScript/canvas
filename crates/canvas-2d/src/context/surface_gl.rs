use skia_safe::{AlphaType, Color, ColorType, gpu, ImageInfo, ISize, PixelGeometry, surfaces};
use skia_safe::gpu::gl::Interface;

use crate::context::{Context, State, SurfaceData, SurfaceEngine, SurfaceState};
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
        let bounds = skia_safe::Rect::from_wh(width, height);
        let mut direct_context = None;
        let mut engine = SurfaceEngine::GL;
        let surface = if bounds.is_empty() {
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

            let mut width = width;
            if width <= 0. {
                width = 1.
            }
            let mut height = height;

            if height <= 0. {
                height = 1.
            }

            engine = SurfaceEngine::CPU;

            let info = ImageInfo::new(ISize::new(width as i32, height as i32), color_type, alpha_type, None);

            surfaces::raster(&info, None, None).unwrap()
        } else {
            let interface = Interface::new_native();
            let mut ctx = gpu::direct_contexts::make_gl(interface.unwrap(), None).unwrap();

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
            let surface = gpu::surfaces::wrap_backend_render_target(
                &mut ctx,
                &target,
                gpu::SurfaceOrigin::BottomLeft,
                color_type,
                None,
                Some(&surface_props),
            )
                .unwrap();

            direct_context = Some(ctx);
            surface
        };

        let mut state = State::default();
        state.direction = direction;

        Context {
            direct_context,
            #[cfg(feature = "vulkan")]
            ash_graphics: None,
            #[cfg(feature = "vulkan")]
            vk_surface: None,
            surface_data: SurfaceData {
                bounds,
                scale: density,
                ppi,
                engine,
            },
            surface,
            path: Path::default(),
            state,
            state_stack: vec![],
            font_color: Color::new(font_color as u32),
            surface_state: SurfaceState::None,
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
        let bounds = skia_safe::Rect::from_wh(width, height);
        let mut direct_context = None;
        let mut engine = SurfaceEngine::GL;
        let surface = if bounds.is_empty() {
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

            let mut width = width;
            if width <= 0. {
                width = 1.
            }
            let mut height = height;

            if height <= 0. {
                height = 1.
            }

            engine = SurfaceEngine::CPU;

            let info = ImageInfo::new(ISize::new(width as i32, height as i32), color_type, alpha_type, None);

            surfaces::raster(&info, None, None)
        } else {
            let interface = Interface::new_native();
            let ctx = gpu::direct_contexts::make_gl(interface.unwrap(), None);
            if ctx.is_none() {
                return;
            }
            let mut ctx = ctx.unwrap();

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

            let surface = gpu::surfaces::wrap_backend_render_target(
                &mut ctx,
                &target,
                gpu::SurfaceOrigin::BottomLeft,
                color_type,
                None,
                Some(&surface_props),
            );

            direct_context = Some(ctx);
            surface
        };

        if let Some(surface) = surface {
            context.direct_context = direct_context;
            context.surface_state = SurfaceState::None;
            context.surface_data.engine = engine;
            context.surface_data.bounds = bounds;
            context.surface_data.scale = density;
            context.surface_data.ppi = ppi;
            context.path = Path::default();
            context.reset_state();
            context.surface = surface;
        }
    }
}
