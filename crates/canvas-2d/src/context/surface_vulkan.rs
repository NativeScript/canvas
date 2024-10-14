use crate::context::text_styles::text_direction::TextDirection;
use crate::context::{Context, State, SurfaceData, SurfaceEngine};
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
    ) -> Self {
        let mut vulkan_context = canvas_core::gpu::vulkan::VulkanContext::new("ns-app").unwrap();
        vulkan_context.set_alpha(alpha);
        let mut context = {
            let get_proc = |of| unsafe {
                let ret = match of {
                    gpu::vk::GetProcOf::Instance(instance, name) => {
                        if let Some(ret) = vulkan_context.get_instance_proc_addr(instance as _, name) {
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
                    (_, _) => {
                        std::ptr::null()
                    }
                }
            };

            let backend_context = unsafe {
                gpu::vk::BackendContext::new(
                    vulkan_context.instance_handle() as _,
                    vulkan_context.physical_device() as _,
                    vulkan_context.device_handle() as _,
                    (
                        vulkan_context.queue() as _,
                        vulkan_context.index(),
                    ),
                    &get_proc,
                )
            };

            gpu::direct_contexts::make_vulkan(&backend_context, None)
        }
            .unwrap();

        vulkan_context.set_view(view, width as u32, height as u32);

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

        let bt = unsafe { gpu::backend_textures::make_vk((width as i32, height as i32), &image_info, "") };


        let mut surface = gpu::surfaces::wrap_backend_texture(&mut context, &bt, gpu::SurfaceOrigin::TopLeft, None, ColorType::N32,
                                                              None,
                                                              None).unwrap();

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
            },
            vulkan_context: Some(vulkan_context),
            vulkan_texture: Some(bt),
            #[cfg(feature = "gl")]
            gl_context: None,
            #[cfg(feature = "metal")]
            metal_context: None,
            #[cfg(feature = "metal")]
            metal_texture_info: None,
            surface,
            path: Default::default(),
            state,
            state_stack: vec![],
            font_color: skia_safe::Color::new(font_color as u32),
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

                texture = Some(unsafe { gpu::backend_textures::make_vk((size.width() as i32, size.height() as i32), &image_info, "") });
            }
        }

        if let Some(texture) = texture {
            self.surface.replace_backend_texture(&texture, gpu::SurfaceOrigin::TopLeft);
            self.vulkan_texture = Some(texture);
        }
    }


    pub fn resize_vulkan(
        context: &mut Context,
        width: f32,
        height: f32,
        alpha: bool,
    ) {
        let mut image = None;
        let mut queue = None;
        if let Some(vulkan_context) = context.vulkan_context.as_mut() {
            vulkan_context.resize(width as u32, height as u32);
            image = vulkan_context.current_image_raw();
            queue = Some(vulkan_context.index() as u32);
        }

        if let Some(direct_context) = context.direct_context.as_mut() {
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

            let bt = unsafe { gpu::backend_textures::make_vk((width as i32, height as i32), &image_info, "") };


            let surface = gpu::surfaces::wrap_backend_texture(direct_context, &bt, gpu::SurfaceOrigin::TopLeft, None, ColorType::N32,
                                                              None,
                                                              None).unwrap();

            let bounds = skia_safe::Rect::from_wh(width, height);
            context.surface_data.bounds = bounds;
            context.surface = surface;
            context.vulkan_texture = Some(bt);
        }
    }
}
