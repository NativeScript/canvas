use crate::context::paths::path::Path;
use crate::context::text_styles::text_direction::TextDirection;
use crate::context::{ColorSpace, Context, State, SurfaceData, SurfaceEngine, SurfaceState};
use canvas_core::context_attributes::PowerPreference;
use skia_safe::gpu::gl::Interface;
use skia_safe::{gpu, surfaces, AlphaType, Color, ColorType, ISize, ImageInfo, PixelGeometry};
use std::ffi::c_void;
use std::ptr::NonNull;

/// Skia's GL entry points. On Windows GL is ANGLE, so they come from its `eglGetProcAddress`;
/// `new_native` would bind WGL (desktop GL), which is not the context that is current.
fn gl_interface() -> Option<Interface> {
    #[cfg(target_os = "windows")]
    {
        Interface::new_load_with(|name| canvas_core::gpu::gl::get_proc_address(name))
    }
    #[cfg(not(target_os = "windows"))]
    {
        Interface::new_native()
    }
}

/// 8 bits a channel even when opaque, as on the web: at 5-6 bits (RGB565) a translucent fill never
/// converges on its colour, so a trail fading out leaves a permanent tint. Opaque is RGB888x, whose
/// alpha reads back as 255 whatever was drawn.
fn color_type_for(alpha: bool) -> ColorType {
    if alpha {
        ColorType::RGBA8888
    } else {
        ColorType::RGB888x
    }
}

fn with_offscreen(
    ctx: &mut gpu::DirectContext,
    window: skia_safe::Surface,
    alpha: bool,
    color_space: Option<skia_safe::ColorSpace>,
) -> (skia_safe::Surface, Option<skia_safe::Surface>) {
    #[cfg(target_os = "android")]
    {
        let color_type = color_type_for(alpha);
        let alpha_type = if alpha { AlphaType::Premul } else { AlphaType::Opaque };
        let info = ImageInfo::new(
            ISize::new(window.width(), window.height()),
            color_type,
            alpha_type,
            color_space,
        );
        let props = window.props().clone();
        // Multisampled, so Skia draws antialiased paths on the GPU instead of rasterizing their
        // coverage on this thread and uploading it every frame; on a tiled GPU the samples mostly
        // stay in tile memory.
        let samples = ctx.max_surface_sample_count_for_color_type(color_type).min(4);
        if let Some(offscreen) = gpu::surfaces::render_target(
            ctx,
            gpu::Budgeted::Yes,
            &info,
            Some(samples),
            gpu::SurfaceOrigin::TopLeft,
            Some(&props),
            false,
            false,
        ) {
            return (offscreen, Some(window));
        }
    }
    #[cfg(not(target_os = "android"))]
    let _ = (ctx, alpha, color_space);
    (window, None)
}

/// A pbuffer keeps its contents across swaps; a window does not.
fn draws_to_window(gl: &canvas_core::gpu::gl::GLContext) -> bool {
    #[cfg(target_os = "android")]
    return !gl.is_pbuffer();
    #[cfg(not(target_os = "android"))]
    {
        let _ = gl;
        false
    }
}

const GR_GL_RGBA8: u32 = 0x8058;
/// An opaque config's buffer, which RGB888x wraps.
const GR_GL_RGB8: u32 = 0x8051;

#[cfg(feature = "gl")]
impl Context {
    pub fn new_gl(
        view: *mut c_void,
        width: f32,
        height: f32,
        density: f32,
        alpha: bool,
        font_color: i32,
        ppi: f32,
        direction: TextDirection,
        color_space: ColorSpace,
    ) -> Option<Self> {
        let mut attr = canvas_core::context_attributes::ContextAttributes::new(
            alpha,
            false,
            false,
            false,
            PowerPreference::Default,
            true,
            false,
            false,
            false,
            false,
            true,
            false,
            color_space
        );

        let bounds = skia_safe::Rect::from_wh(width, height);
        let mut engine = SurfaceEngine::GL;
        let mut zero_size = false;
        let mut width = width;
        if width <= 0. {
            zero_size = true;
            width = 1.
        }
        let mut height = height;

        if height <= 0. {
            zero_size = true;
            height = 1.
        }

        let gl_context = if zero_size {
            canvas_core::gpu::gl::GLContext::create_offscreen_context(&mut attr, width as i32, height as i32)
        } else if let Some(view) = NonNull::new(view) {
            #[cfg(target_os = "android")]{
                let handle = raw_window_handle::AndroidNdkWindowHandle::new(view);
                let handle = raw_window_handle::RawWindowHandle::AndroidNdk(handle);
                canvas_core::gpu::gl::GLContext::create_window_context(&mut attr, width as i32, height as i32, handle)
            }
            #[cfg(any(target_os = "ios", target_os = "macos", target_os = "visionos", target_os = "tvos"))]{
                canvas_core::gpu::gl::GLContext::create_window_context(&mut attr, view)
            }
            // No native window surfaces: the host presents the GL output itself.
            #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos", target_os = "visionos", target_os = "tvos")))]{
                let _ = view;
                canvas_core::gpu::gl::GLContext::create_offscreen_context(&mut attr, width as i32, height as i32)
            }
        } else {
            canvas_core::gpu::gl::GLContext::create_offscreen_context(&mut attr, width as i32, height as i32)
        }?;

        if !gl_context.make_current() {
            return None;
        }

        let mut buffer_id = [0i32];

        unsafe { gl_bindings::GetIntegerv(gl_bindings::FRAMEBUFFER_BINDING, buffer_id.as_mut_ptr()) }

        let interface = gl_interface()?;

        let mut ctx = gpu::direct_contexts::make_gl(interface, None)?;

        let mut frame_buffer = gpu::gl::FramebufferInfo::from_fboid(buffer_id[0] as u32);

        frame_buffer.format = if alpha { GR_GL_RGBA8 } else { GR_GL_RGB8 };

        let target = gpu::backend_render_targets::make_gl(
            (width as i32, height as i32),
            Some(0),
            0,
            frame_buffer,
        );
        let surface_props = skia_safe::SurfaceProps::new(
            skia_safe::SurfacePropsFlags::default(),
            PixelGeometry::Unknown,
        );
        let color_type = color_type_for(alpha);

        let surface = gpu::surfaces::wrap_backend_render_target(
            &mut ctx,
            &target,
            gpu::SurfaceOrigin::BottomLeft,
            color_type,
            <ColorSpace as Into<Option<skia_safe::ColorSpace>>>::into(color_space),
            Some(&surface_props),
        )?;

        let (surface, window_surface) = if draws_to_window(&gl_context) {
            with_offscreen(&mut ctx, surface, alpha, color_space.into())
        } else {
            (surface, None)
        };

        let direct_context = Some(ctx);


        let mut state = State::default();
        state.direction = direction;

        Some(Context {
            window_surface,
            direct_context,
            #[cfg(feature = "metal")]
            metal_context: None,
            #[cfg(feature = "metal")]
            metal_texture_info: None,
            gl_context: Some(gl_context),
            #[cfg(feature = "vulkan")]
            vulkan_context: None,
            #[cfg(feature = "vulkan")]
            vulkan_texture: None,
            cpu_context: None,
            #[cfg(all(feature = "d3d", target_os = "windows"))]
            d3d: None,
            surface_data: SurfaceData {
                bounds,
                scale: density,
                ppi,
                engine,
                state: Default::default(),
                is_opaque: !alpha,
                color_space
            },
            surface,
            path: Path::default(),
            state,
            state_stack: vec![],
            font_color: Color::new(font_color as u32),
            recording: None,
            surface_state: SurfaceState::None,
        })
    }

    /// Call after raw GL made behind Skia's back.
    pub fn reset_gpu_state(&mut self) {
        if let Some(ctx) = self.direct_context.as_mut() {
            ctx.reset(None);
        }
    }

    pub fn presents_through_window(&self) -> bool {
        self.window_surface.is_some()
    }

    /// The window's framebuffer, when 2D presents through one. The GL binding is no guide to it:
    /// Skia leaves its offscreen framebuffer bound.
    pub fn window_framebuffer(&mut self) -> Option<i32> {
        let window = self.window_surface.as_mut()?;
        gpu::surfaces::get_backend_render_target(
            window,
            skia_safe::surface::BackendHandleAccess::FlushRead,
        )?
        .gl_framebuffer_info()
        .map(|info| info.fboid as i32)
    }

    /// For a canvas made before its view had a surface. Starts blank; the caller restores it.
    pub fn use_offscreen_for_window(&mut self) {
        if self.window_surface.is_some() {
            return;
        }
        if !self.gl_context.as_ref().is_some_and(draws_to_window) {
            return;
        }
        let Some(ctx) = self.direct_context.as_mut() else {
            return;
        };
        let Some(placeholder) = surfaces::raster_n32_premul((1, 1)) else {
            return;
        };
        let window = std::mem::replace(&mut self.surface, placeholder);
        let (surface, window_surface) = with_offscreen(
            ctx,
            window,
            !self.surface_data.is_opaque,
            self.surface_data.color_space.into(),
        );
        self.surface = surface;
        self.window_surface = window_surface;
    }

    pub fn present_to_window(&mut self) {
        if self.window_surface.is_none() {
            return;
        }
        self.bind_surface();
        let Some(window) = self.window_surface.as_mut() else {
            return;
        };
        {
            // Dropped before the next draw, so the surface is not copied on write.
            let image = self.surface.image_snapshot();
            let mut paint = skia_safe::Paint::default();
            paint.set_blend_mode(skia_safe::BlendMode::Src);
            window.canvas().draw_image(&image, (0., 0.), Some(&paint));
        }
        if let Some(ctx) = self.direct_context.as_mut() {
            ctx.flush_and_submit_surface(window, None);
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
        let color_space: Option<skia_safe::ColorSpace> = context.surface_data.color_space.into();
        let bounds = skia_safe::Rect::from_wh(width, height);
        let mut direct_context = None;
        let mut window_surface = None;
        let mut engine = SurfaceEngine::GL;
        let surface = if bounds.is_empty() {
            let color_type = color_type_for(alpha);

            let alpha_type = if alpha {
                AlphaType::Unpremul
            } else {
                AlphaType::Opaque
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

            let info = ImageInfo::new(ISize::new(width as i32, height as i32), color_type, alpha_type, color_space);

            surfaces::raster(&info, None, None)
        } else {
            // Reuse the Skia context: the EGL context is unchanged, and a second Skia context on it
            // would free GL names the first may still hold images for.
            let mut ctx = match context.direct_context.take() {
                Some(mut ctx) => {
                    ctx.reset(None);
                    ctx
                }
                None => match gl_interface().and_then(|i| gpu::direct_contexts::make_gl(i, None)) {
                    Some(ctx) => ctx,
                    None => return,
                },
            };

            let mut frame_buffer = gpu::gl::FramebufferInfo::from_fboid(buffer_id as u32);

            frame_buffer.format = if alpha { GR_GL_RGBA8 } else { GR_GL_RGB8 };

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
            let color_type = color_type_for(alpha);

            let surface = gpu::surfaces::wrap_backend_render_target(
                &mut ctx,
                &target,
                gpu::SurfaceOrigin::BottomLeft,
                color_type,
                color_space.clone(),
                Some(&surface_props),
            );

            let on_window = context.gl_context.as_ref().is_some_and(draws_to_window);
            let surface = match surface {
                Some(surface) if on_window => {
                    let (surface, window) = with_offscreen(&mut ctx, surface, alpha, color_space);
                    window_surface = window;
                    Some(surface)
                }
                surface => surface,
            };

            direct_context = Some(ctx);
            surface
        };

        if let Some(surface) = surface {
            context.window_surface = window_surface;
            context.direct_context = direct_context;
            context.surface_state = SurfaceState::None;
            context.surface_data.engine = engine;
            context.surface_data.bounds = bounds;
            context.surface_data.scale = density;
            context.surface_data.ppi = ppi;
            context.surface_data.is_opaque = !alpha;
            context.path = Path::default();
            context.reset_state();
            context.surface = surface;
        } else if context.direct_context.is_none() {
            context.direct_context = direct_context;
        }
    }
}
