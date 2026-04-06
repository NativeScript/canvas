use crate::context::paths::path::Path;
use crate::context::text_styles::text_direction::TextDirection;
use crate::context::{Context, State, SurfaceData, SurfaceEngine, SurfaceState};
use canvas_core::gpu::metal::MetalContext;
use foreign_types_shared::ForeignTypeRef;
use skia_safe::gpu::mtl::TextureInfo;
use skia_safe::{gpu, Color, ColorType};
use std::os::raw::c_void;
use canvas_core::context_attributes::ColorSpace;

#[cfg(feature = "metal")]
impl Context {
    pub fn draw_on_texture(&mut self, texture: &canvas_core::gpu::metal::MetalTexture) -> bool {
        let width = texture.width();
        let height = texture.height();
        if let Some(context) = self.direct_context.as_mut() {
            let info = unsafe { TextureInfo::new(texture.texture() as gpu::mtl::Handle) };
            let bt = unsafe {
                gpu::backend_textures::make_mtl(
                    (width as i32, height as i32),
                    gpu::Mipmapped::No,
                    &info,
                    "",
                )
            };

            // let snapshot = self.surface.image_snapshot();
            if let Some(mut surface) = gpu::surfaces::wrap_backend_texture(
                context,
                &bt,
                gpu::SurfaceOrigin::TopLeft,
                Some(0),
                ColorType::RGBA8888,
                None,
                None,
            ) {
                surface.canvas().draw_color(Color::RED, None);
                
                // surface
                //     .canvas()
                //     .draw_image(snapshot, skia_safe::Point::new(0.0, 0.0), None);
                 context.flush_surface(&mut surface);
                 context.flush_submit_and_sync_cpu();

                return true;
            }
        }
        false
    }
  
    pub fn new_metal(
        view: *mut c_void,
        density: f32,
        samples: usize,
        alpha: bool,
        font_color: i32,
        ppi: f32,
        direction: TextDirection,
        color_space: ColorSpace
    ) -> Self {
        let mtl_context = MetalContext::new(view);
        let backend = unsafe {
            gpu::mtl::BackendContext::new(
                mtl_context.device() as gpu::mtl::Handle,
                mtl_context.queue() as gpu::mtl::Handle,
            )
        };

        let (width, height) = mtl_context.drawable_size();

        let mut context = gpu::direct_contexts::make_metal(&backend, None).unwrap();

        let drawable = mtl_context.drawable().unwrap();
        let info = unsafe { TextureInfo::new(drawable.texture().as_ptr() as gpu::mtl::Handle) };
        let bt = unsafe {
            gpu::backend_textures::make_mtl(
                (width as i32, height as i32),
                gpu::Mipmapped::No,
                &info,
                "",
            )
        };
        let surface = gpu::surfaces::wrap_backend_texture(
            &mut context,
            &bt,
            gpu::SurfaceOrigin::TopLeft,
            Some(samples),
            ColorType::BGRA8888,
            <ColorSpace as Into<Option<skia_safe::ColorSpace>>>::into(color_space),
            None,
        )
        .unwrap();

        let mut state = State::default();
        state.direction = TextDirection::from(direction as u32);

        let bounds = skia_safe::Rect::from_wh(width as f32, height as f32);
        Context {
            surface_data: SurfaceData {
                bounds,
                scale: density,
                ppi,
                engine: SurfaceEngine::Metal,
                state: Default::default(),
                is_opaque: !alpha,
                color_space
            },
            surface,
            surface_state: Default::default(),
            #[cfg(feature = "vulkan")]
            vulkan_context: None,
            #[cfg(feature = "vulkan")]
            vulkan_texture: None,
            cpu_context: None,
            metal_context: Some(mtl_context),
            metal_texture_info: Some(info),
            #[cfg(feature = "gl")]
            gl_context: None,
            direct_context: Some(context),
            path: Path::default(),
            state,
            state_stack: vec![],
            font_color: Color::new(font_color as u32),
        }
    }

    pub fn new_metal_device_queue(
        view: *mut c_void,
        device: *mut c_void,
        queue: *mut c_void,
        density: f32,
        samples: usize,
        alpha: bool,
        font_color: i32,
        ppi: f32,
        direction: TextDirection,
        color_space: ColorSpace
    ) -> Self {
        let mut mtl_context = unsafe { MetalContext::new_device_queue(view, device, queue) };
        let backend = unsafe {
            gpu::mtl::BackendContext::new(device as gpu::mtl::Handle, queue as gpu::mtl::Handle)
        };
        let (width, height) = mtl_context.drawable_size();

        let mut context = gpu::direct_contexts::make_metal(&backend, None).unwrap();
        let drawable = mtl_context.current_drawable().unwrap();
        let info = unsafe { TextureInfo::new(drawable.texture().as_ptr() as gpu::mtl::Handle) };
        let bt = unsafe {
            gpu::backend_textures::make_mtl(
                (width as i32, height as i32),
                gpu::Mipmapped::No,
                &info,
                "",
            )
        };
        let surface = gpu::surfaces::wrap_backend_texture(
            &mut context,
            &bt,
            gpu::SurfaceOrigin::TopLeft,
            Some(samples),
            ColorType::BGRA8888,
            <ColorSpace as Into<Option<skia_safe::ColorSpace>>>::into(color_space),
            None,
        )
        .unwrap();

        let mut state = State::default();
        state.direction = TextDirection::from(direction as u32);

        let bounds = skia_safe::Rect::from_wh(surface.width() as f32, surface.height() as f32);
        Context {
            surface_data: SurfaceData {
                bounds,
                scale: density,
                ppi,
                engine: SurfaceEngine::Metal,
                state: Default::default(),
                is_opaque: !alpha,
                color_space
            },
            surface,
            surface_state: Default::default(),
            #[cfg(feature = "vulkan")]
            vulkan_context: None,
            #[cfg(feature = "vulkan")]
            vulkan_texture: None,
            cpu_context: None,
            metal_context: Some(mtl_context),
            metal_texture_info: Some(info),
            #[cfg(feature = "gl")]
            gl_context: None,
            direct_context: Some(context),
            path: Path::default(),
            state,
            state_stack: vec![],
            font_color: Color::new(font_color as u32),
        }
    }

    pub fn new_metal_offscreen(
        width: f32,
        height: f32,
        density: f32,
        samples: usize,
        alpha: bool,
        font_color: i32,
        ppi: f32,
        direction: TextDirection,
        color_space: ColorSpace
    ) -> Self {
        let mtl_context = MetalContext::new_offscreen(width, height);
        let backend = unsafe {
            gpu::mtl::BackendContext::new(
                mtl_context.device() as gpu::mtl::Handle,
                mtl_context.queue() as gpu::mtl::Handle,
            )
        };

        let (width, height) = mtl_context.drawable_size();

        let mut context = gpu::direct_contexts::make_metal(&backend, None).unwrap();

        let drawable = mtl_context.drawable().unwrap();
        let info = unsafe { TextureInfo::new(drawable.texture().as_ptr() as gpu::mtl::Handle) };
        let bt = unsafe {
            gpu::backend_textures::make_mtl(
                (width as i32, height as i32),
                gpu::Mipmapped::No,
                &info,
                "",
            )
        };
        let surface = gpu::surfaces::wrap_backend_texture(
            &mut context,
            &bt,
            gpu::SurfaceOrigin::TopLeft,
            Some(samples),
            ColorType::BGRA8888,
            <ColorSpace as Into<Option<skia_safe::ColorSpace>>>::into(color_space),
            None,
        )
        .unwrap();

        let mut state = State::default();
        state.direction = TextDirection::from(direction as u32);

        let bounds = skia_safe::Rect::from_wh(width as f32, height as f32);
        Context {
            surface_data: SurfaceData {
                bounds,
                scale: density,
                ppi,
                engine: SurfaceEngine::Metal,
                state: Default::default(),
                is_opaque: !alpha,
                color_space
            },
            surface,
            surface_state: Default::default(),
            #[cfg(feature = "vulkan")]
            vulkan_context: None,
            #[cfg(feature = "vulkan")]
            vulkan_texture: None,
            cpu_context: None,
            metal_context: Some(mtl_context),
            metal_texture_info: Some(info),
            #[cfg(feature = "gl")]
            gl_context: None,
            direct_context: Some(context),
            path: Path::default(),
            state,
            state_stack: vec![],
            font_color: Color::new(font_color as u32),
        }
    }

    pub fn resize_metal(context: &mut Context, width: f32, height: f32) {
        let _ = MetalContext::new_release_pool();

        // flush any pending draws before resizing
        context.flush_and_render_to_surface();

        let bounds = skia_safe::Rect::from_wh(width, height);
        context.surface_data.bounds = bounds;
        let mut samples = 1;
        let mut info: Option<TextureInfo> = None;
        if let Some(context) = context.metal_context.as_mut() {
            samples = context.sample_count();
            if context.is_offscreen() {
                context.set_drawable_size(width as f64, height as f64)
            }

            if let Some(drawable) = context.next_drawable() {
                info = unsafe {
                    Some(TextureInfo::new(
                        drawable.texture().as_ptr() as gpu::mtl::Handle
                    ))
                };
            }
        }
        let color_space: Option<skia_safe::ColorSpace> = context.surface_data.color_space.into();
        let mut surface = None;
        if let (Some(context), Some(info)) = (context.direct_context.as_mut(), info.as_ref()) {
            let bt = unsafe {
                gpu::backend_textures::make_mtl(
                    (width as i32, height as i32),
                    gpu::Mipmapped::No,
                    info,
                    "",
                )
            };

            surface = gpu::surfaces::wrap_backend_texture(
                context,
                &bt,
                gpu::SurfaceOrigin::TopLeft,
                Some(samples),
                ColorType::BGRA8888,
                color_space,
                None,
            );

            // surface = unsafe {
            //     gpu::surfaces::wrap_mtk_view(
            //         context,
            //         view as gpu::mtl::Handle,
            //         gpu::SurfaceOrigin::TopLeft,
            //         Some(sample_count),
            //         ColorType::BGRA8888,
            //         None,
            //         None,
            //     )
            // };
        }

        if let Some(surface) = surface {
            context.surface_data.state = Default::default();
            context.surface_state = SurfaceState::None;
            context.surface = surface;
            context.metal_texture_info = info;
            context.path = Path::default();
            context.reset_state();
        }
    }

    pub fn present(context: &mut Context) {
        let _ = MetalContext::new_release_pool();
        let mut info: Option<TextureInfo> = None;
        let mut width = 0;
        let mut height = 0;
        if let Some(context) = context.metal_context.as_mut() {
            context.present_drawable();
        }

        if let Some(context) = context.metal_context.as_mut() {
            if let Some(drawable) = context.next_drawable() {
                let texture = drawable.texture();
                width = texture.width();
                height = texture.height();
                info = unsafe { Some(TextureInfo::new(texture.as_ptr() as gpu::mtl::Handle)) };
            }
        }

        if let Some(info) = info.as_ref() {
            let bt = unsafe {
                gpu::backend_textures::make_mtl(
                    (width as i32, height as i32),
                    gpu::Mipmapped::No,
                    info,
                    "",
                )
            };
            context
                .surface
                .replace_backend_texture(&bt, gpu::SurfaceOrigin::TopLeft);
        }

        context.metal_texture_info = info;
    }
}
