use crate::context::paths::path::Path;
use crate::context::text_styles::text_direction::TextDirection;
use crate::context::{Context, State, SurfaceData, SurfaceEngine};
use canvas_core::gpu::metal::MetalContext;
use foreign_types_shared::ForeignTypeRef;
use skia_safe::gpu::mtl::TextureInfo;
use skia_safe::{gpu, Color, ColorType};
use std::os::raw::c_void;

#[cfg(feature = "metal")]
impl Context {
    pub fn new_metal(
        view: *mut c_void,
        density: f32,
        samples: usize,
        alpha: bool,
        font_color: i32,
        ppi: f32,
        direction: TextDirection,
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
        // let surface_holder = unsafe {
        //     gpu::surfaces::wrap_mtk_view(
        //         &mut context,
        //         view as gpu::mtl::Handle,
        //         gpu::SurfaceOrigin::TopLeft,
        //         Some(samples),
        //         ColorType::BGRA8888,
        //         None,
        //         None,
        //     )
        // }.unwrap();


        let drawable = mtl_context.drawable().unwrap();
        let info = unsafe { TextureInfo::new(drawable.texture().as_ptr() as gpu::mtl::Handle) };
        let bt = unsafe { gpu::backend_textures::make_mtl((width as i32, height as i32), gpu::Mipmapped::No, &info, "") };
        let surface = gpu::surfaces::wrap_backend_texture(&mut context, &bt, gpu::SurfaceOrigin::TopLeft, Some(samples), ColorType::BGRA8888,
                                                          None,
                                                          None).unwrap();

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
            },
            surface,
            surface_state: Default::default(),
            #[cfg(feature = "vulkan")]
            vulkan_context: None,
            #[cfg(feature = "vulkan")]
            vulkan_texture: None,
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
    ) -> Self {
        let mut mtl_context = unsafe { MetalContext::new_device_queue(view, device, queue) };
        let backend = unsafe {
            gpu::mtl::BackendContext::new(
                device as gpu::mtl::Handle,
                queue as gpu::mtl::Handle,
            )
        };
        let (width, height) = mtl_context.drawable_size();

        let mut context = gpu::direct_contexts::make_metal(&backend, None).unwrap();
        // let surface = unsafe {
        //     gpu::surfaces::wrap_mtk_view(
        //         &mut context,
        //         view as gpu::mtl::Handle,
        //         gpu::SurfaceOrigin::TopLeft,
        //         Some(samples),
        //         ColorType::BGRA8888,
        //         None,
        //         None,
        //     )
        // }.unwrap();
        let drawable = mtl_context.current_drawable().unwrap();
        let info = unsafe { TextureInfo::new(drawable.texture().as_ptr() as gpu::mtl::Handle) };
        let bt = unsafe { gpu::backend_textures::make_mtl((width as i32, height as i32), gpu::Mipmapped::No, &info, "") };
        let surface = gpu::surfaces::wrap_backend_texture(&mut context, &bt, gpu::SurfaceOrigin::TopLeft, Some(samples), ColorType::BGRA8888,
                                                          None,
                                                          None).unwrap();


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
            },
            surface,
            surface_state: Default::default(),
            #[cfg(feature = "vulkan")]
            vulkan_context: None,
            #[cfg(feature = "vulkan")]
            vulkan_texture: None,
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
        let bounds = skia_safe::Rect::from_wh(width, height);
        context.surface_data.bounds = bounds;
        let mut samples = 1;
        let mut info: Option<TextureInfo> = None;
        if let Some(context) = context.metal_context.as_mut() {
            samples = context.sample_count();
            if let Some(drawable) = context.next_drawable() {
                info = unsafe { Some(TextureInfo::new(drawable.texture().as_ptr() as gpu::mtl::Handle)) };
            }
        }


        let mut surface = None;
        if let (Some(context), Some(info)) = (context.direct_context.as_mut(), info.as_ref()) {
            let bt = unsafe { gpu::backend_textures::make_mtl((width as i32, height as i32), gpu::Mipmapped::No, info, "") };

            surface = gpu::surfaces::wrap_backend_texture(context, &bt, gpu::SurfaceOrigin::TopLeft, Some(samples), ColorType::BGRA8888,
                                                          None,
                                                          None);

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
            context.surface = surface;
            context.metal_texture_info = info;
        }
    }

    pub fn present(context: &mut Context) {
        let _ = MetalContext::new_release_pool();
        let mut info: Option<TextureInfo> = None;
        let mut width = 0;
        let mut height = 0;
        if let Some(context) = context.metal_context.as_mut() {
            context.present_drawable();
            if let Some(drawable) = context.next_drawable() {
                let texture = drawable.texture();
                width = texture.width();
                height = texture.height();
                info = unsafe { Some(TextureInfo::new(texture.as_ptr() as gpu::mtl::Handle)) };
            }
        }


        if let Some(info) = info.as_ref() {
            let bt = unsafe { gpu::backend_textures::make_mtl((width as i32, height as i32), gpu::Mipmapped::No, info, "") };
            context.surface.replace_backend_texture(&bt, gpu::SurfaceOrigin::TopLeft);
        }

        context.metal_texture_info = info;
    }
}
