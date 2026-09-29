use crate::context::paths::path::Path;
use crate::context::text_styles::text_direction::TextDirection;
use crate::context::{ColorSpace, Context, State, SurfaceData, SurfaceEngine, SurfaceState};
use skia_safe::wrapper::PointerWrapper;
use skia_safe::{gpu, ColorType};
use std::ffi::CStr;
use std::os::raw::c_void;

#[cfg(feature = "vulkan")]
impl Context {
    pub fn new_vulkan(
        width: f32,
        height: f32,
        view: *mut c_void,
        density: f32,
        alpha: bool,
        font_color: i32,
        ppi: f32,
        direction: u8,
        color_space: ColorSpace,
    ) -> Self {
        let mut vulkan_context = canvas_core::gpu::vulkan::VulkanContext::new("ns-app", color_space).unwrap();
        vulkan_context.set_alpha(alpha);
        let mut context = {
            let get_proc = |of| unsafe {
                let ret = match of {
                    gpu::vk::GetProcOf::Instance(instance, name) => {
                        if let Some(ret) =
                            vulkan_context.get_instance_proc_addr(instance as _, name)
                        {
                            (Some(ret), None)
                        } else {
                            let name = unsafe { CStr::from_ptr(name) };
                            let name = name.to_string_lossy();
                            (None, Some(name.to_string()))
                        }
                    }
                    gpu::vk::GetProcOf::Device(device, name) => {
                        if let Some(ret) = vulkan_context.get_device_proc_addr(device as _, name) {
                            (Some(ret), None)
                        } else {
                            let name = unsafe { CStr::from_ptr(name) };
                            let name = name.to_string_lossy();
                            (None, Some(name.to_string()))
                        }
                    }
                };
                match ret {
                    (Some(f), None) => f as _,
                    (None, Some(name)) => {
                        #[cfg(target_os = "android")]
                        log::info!("resolve of {} failed", name);

                        #[cfg(not(target_os = "android"))]
                        println!("resolve of {} failed", name);
                        std::ptr::null()
                    }
                    (_, _) => std::ptr::null(),
                }
            };

            let backend_context = unsafe {
                gpu::vk::BackendContext::new(
                    vulkan_context.instance_handle() as _,
                    vulkan_context.physical_device() as _,
                    vulkan_context.device_handle() as _,
                    (vulkan_context.queue() as _, vulkan_context.index()),
                    &get_proc,
                )
            };

            gpu::direct_contexts::make_vulkan(&backend_context, None)
        }
        .unwrap();


        vulkan_context.set_view(view, width as u32, height as u32);

        let color_space = vulkan_context.color_space();

        let image = vulkan_context.current_image_raw();

        let alloc = gpu::vk::Alloc::default();
        let image_info = unsafe {
            gpu::vk::ImageInfo::new(
                image.unwrap() as gpu::vk::Image,
                alloc,
                gpu::vk::ImageTiling::OPTIMAL,
                gpu::vk::ImageLayout::UNDEFINED,
                gpu::vk::Format::R8G8B8A8_UNORM,
                1,
                None,
                None,
                None,
                None,
            )
        };

        let bt = unsafe {
            gpu::backend_textures::make_vk((width as i32, height as i32), &image_info, "")
        };

        let mut surface = gpu::surfaces::wrap_backend_texture(
            &mut context,
            &bt,
            gpu::SurfaceOrigin::TopLeft,
            None,
            ColorType::N32,
            <ColorSpace as Into<Option<skia_safe::ColorSpace>>>::into(color_space),
            None,
        )
        .unwrap();

        let mut state = State::default();
        state.direction = TextDirection::from(direction as u32);

        let bounds = skia_safe::Rect::from_wh(width, height);
        Context {
            direct_context: Some(context),
            surface_data: SurfaceData {
                bounds,
                scale: density,
                ppi,
                engine: SurfaceEngine::Vulkan,
                state: Default::default(),
                is_opaque: !alpha,
                color_space,
            },
            vulkan_context: Some(vulkan_context),
            vulkan_texture: Some(bt),
            #[cfg(feature = "gl")]
            gl_context: None,
            #[cfg(feature = "metal")]
            metal_context: None,
            #[cfg(feature = "metal")]
            metal_texture_info: None,
            cpu_context: None,
            #[cfg(all(feature = "d3d", target_os = "windows"))]
            d3d: None,
            surface,
            path: Default::default(),
            state,
            state_stack: vec![],
            font_color: skia_safe::Color::new(font_color as u32),
            recording: None,
            #[cfg(feature = "gl")]
            window_surface: None,
            surface_state: crate::context::SurfaceState::None,
        }
    }

    pub fn replace_backend_texture(&mut self) {
        let size = self.surface_data.bounds;
        let mut texture = None;
        if let Some(context) = self.vulkan_context.as_mut() {
            let image = context.current_image_raw();
            if let Some(image) = image {
                let alloc = gpu::vk::Alloc::default();
                let image_info = unsafe {
                    gpu::vk::ImageInfo::new(
                        image as gpu::vk::Image,
                        alloc,
                        gpu::vk::ImageTiling::OPTIMAL,
                        gpu::vk::ImageLayout::UNDEFINED,
                        gpu::vk::Format::R8G8B8A8_UNORM,
                        1,
                        None,
                        None,
                        None,
                        None,
                    )
                };

                texture = Some(unsafe {
                    gpu::backend_textures::make_vk(
                        (size.width() as i32, size.height() as i32),
                        &image_info,
                        "",
                    )
                });
            }
        }

        if let Some(texture) = texture {
            self.surface
                .replace_backend_texture(&texture, gpu::SurfaceOrigin::TopLeft);
            self.vulkan_texture = Some(texture);
        }
    }

    fn offscreen_vulkan_surface(&mut self, width: i32, height: i32, alpha: bool) -> Option<skia_safe::Surface> {
        let info = skia_safe::ImageInfo::new(
            skia_safe::ISize::new(width.max(1), height.max(1)),
            ColorType::N32,
            if alpha { skia_safe::AlphaType::Premul } else { skia_safe::AlphaType::Opaque },
            <ColorSpace as Into<Option<skia_safe::ColorSpace>>>::into(self.surface_data.color_space),
        );
        gpu::surfaces::render_target(
            self.direct_context.as_mut()?,
            gpu::Budgeted::Yes,
            &info,
            None,
            gpu::SurfaceOrigin::TopLeft,
            None,
            false,
            None,
        )
    }

    pub fn detach_vulkan_view(&mut self) {
        if self.vulkan_context.is_none() {
            return;
        }
        let bounds = self.surface_data.bounds;
        let alpha = !self.surface_data.is_opaque;
        let snapshot = self.surface.image_snapshot();
        let Some(mut surface) =
            self.offscreen_vulkan_surface(bounds.width() as i32, bounds.height() as i32, alpha)
        else {
            return;
        };
        let matrix = self.surface.canvas().local_to_device();
        let canvas = surface.canvas();
        canvas.draw_image(&snapshot, (0., 0.), None);
        for _ in 0..self.state_stack.len() {
            canvas.save();
        }
        canvas.set_matrix(&matrix);
        self.surface = surface;
        self.vulkan_texture = None;
        // The snapshot draw still reads the swapchain image.
        self.flush_submit_and_sync_cpu();
        if let Some(vulkan_context) = self.vulkan_context.as_mut() {
            vulkan_context.clear_view();
        }
    }

    pub fn resize_vulkan(context: &mut Context, width: f32, height: f32, alpha: bool) {
        // flush any pending draws before resizing
        context.flush_and_render_to_surface();

        let mut image = None;
        let mut queue = None;
        if let Some(vulkan_context) = context.vulkan_context.as_mut() {
            vulkan_context.resize(width as u32, height as u32);
            image = vulkan_context.current_image_raw();
            queue = Some(vulkan_context.index() as u32);
        }

        let color_space = context.surface_data.color_space;

        let Some(image) = image else {
            let Some(surface) = context.offscreen_vulkan_surface(width as i32, height as i32, alpha)
            else {
                return;
            };
            context.surface_data.state = Default::default();
            context.surface_data.is_opaque = !alpha;
            context.surface_data.bounds = skia_safe::Rect::from_wh(width, height);
            context.surface_state = SurfaceState::None;
            context.surface = surface;
            context.vulkan_texture = None;
            context.path = Path::default();
            context.reset_state();
            return;
        };

        if let Some(direct_context) = context.direct_context.as_mut() {
            let alloc = gpu::vk::Alloc::default();
            let image_info = unsafe {
                gpu::vk::ImageInfo::new(
                    image as gpu::vk::Image,
                    alloc,
                    gpu::vk::ImageTiling::OPTIMAL,
                    gpu::vk::ImageLayout::UNDEFINED,
                    gpu::vk::Format::R8G8B8A8_UNORM,
                    1,
                    None,
                    None,
                    None,
                    None,
                )
            };

            let bt = unsafe {
                gpu::backend_textures::make_vk((width as i32, height as i32), &image_info, "")
            };

            let surface = gpu::surfaces::wrap_backend_texture(
                direct_context,
                &bt,
                gpu::SurfaceOrigin::TopLeft,
                None,
                ColorType::N32,
                <ColorSpace as Into<Option<skia_safe::ColorSpace>>>::into(color_space) ,
                None,
            )
            .unwrap();

            let bounds = skia_safe::Rect::from_wh(width, height);
            context.surface_data.state = Default::default();
            context.surface_data.is_opaque = !alpha;
            context.surface_data.bounds = bounds;
            context.surface_state = SurfaceState::None;
            context.surface = surface;
            context.vulkan_texture = Some(bt);
            context.path = Path::default();
            context.reset_state();
        }
    }
}
