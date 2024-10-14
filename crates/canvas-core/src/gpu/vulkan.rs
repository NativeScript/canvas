use ash::prelude::VkResult;
use ash::vk::{Handle, ImageView, SwapchainCreateInfoKHR, SwapchainKHR};
use ash::{vk, Entry, Instance};
use skia_safe::wrapper::NativeTransmutableWrapper;
use skia_safe::Surface;
use std::ffi::CString;
use std::fmt::{Debug, Formatter};
use std::os::raw::{c_char, c_void};
use std::ptr::NonNull;
use std::vec;

pub struct AshGraphics {
    pub current_index: Option<u32>,
    pub swap_chain_images: Option<Vec<vk::Image>>,
    pub swap_chain_image_view: Option<Vec<ImageView>>,
    pub swap_chain_loader: ash::khr::swapchain::Device,
    pub surface: Option<vk::SurfaceKHR>,
    pub swap_chain: Option<SwapchainKHR>,
    pub physical_device: vk::PhysicalDevice,
    pub device: ash::Device,
    pub queue_and_index: (vk::Queue, usize),
    pub command_buffers: Vec<vk::CommandBuffer>,
    pub entry: Entry,
    pub instance: Instance,
    pub image_available_semaphore: vk::Semaphore,
    pub render_finished_semaphore: vk::Semaphore,
    pub render_finished_fence: vk::Fence,
    pub render_pass: Vec<vk::RenderPass>,
    pub frame_buffers: Vec<vk::Framebuffer>,
    pub surface_size: vk::Extent2D,
    pub alpha: bool,
}

impl Drop for AshGraphics {
    fn drop(&mut self) {
        use ash::khr::surface::Instance;
        unsafe {
            self.device.device_wait_idle().unwrap();

            if let Some(surface) = self.surface {
                let instance = Instance::new(&self.entry, &self.instance);
                instance.destroy_surface(surface, None);
            }

            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

impl AshGraphics {
    pub fn vulkan_version() -> Option<(usize, usize, usize)> {
        let entry = unsafe { Entry::load() }.unwrap();

        let detected_version = unsafe { entry.try_enumerate_instance_version().unwrap_or(None) };

        detected_version.map(|ver| {
            (
                vk::api_version_major(ver).try_into().unwrap(),
                vk::api_version_minor(ver).try_into().unwrap(),
                vk::api_version_patch(ver).try_into().unwrap(),
            )
        })
    }

    pub unsafe fn new(app_name: &str) -> Result<AshGraphics, String> {
        let entry = Entry::load().or(Err("Failed to load Vulkan entry"))?;

        let minimum_version = vk::make_api_version(0, 1, 3, 0);

        let instance: Instance = {
            let api_version = Self::vulkan_version()
                .map(|(major, minor, patch)| {
                    vk::make_api_version(
                        0,
                        major.try_into().unwrap(),
                        minor.try_into().unwrap(),
                        patch.try_into().unwrap(),
                    )
                })
                .unwrap_or(minimum_version);

            let app_name = CString::new(app_name).unwrap();


            let extension_names_raw = [
                ash::khr::surface::NAME.as_ptr(),
                ash::khr::android_surface::NAME.as_ptr(),
            ];

            let app_info = vk::ApplicationInfo::default()
                .application_name(&app_name)
                .application_version(0)
                .engine_name(&app_name)
                .engine_version(0)
                .api_version(api_version);

            // let layers_names_raw: Vec<*const c_char> = layer_names
            //     .iter()
            //     .map(|raw_name| raw_name.as_ptr())
            //     .collect();

            let create_flags = vk::InstanceCreateFlags::default();

            let create_info = vk::InstanceCreateInfo::default()
                .application_info(&app_info)
                .enabled_layer_names(&[])
                .enabled_extension_names(&extension_names_raw)
                .flags(create_flags);

            entry.create_instance(&create_info, None)
        }
            .or(Err("Failed to create a Vulkan instance."))?;


        let (physical_device, queue_family_index) = {
            let physical_devices = instance
                .enumerate_physical_devices()
                .expect("Failed to enumerate Vulkan physical devices.");

            physical_devices
                .iter()
                .map(|physical_device| {
                    instance
                        .get_physical_device_queue_family_properties(*physical_device)
                        .iter()
                        .enumerate()
                        .find_map(|(index, info)| {
                            let supports_graphic =
                                info.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                            if supports_graphic {
                                Some((*physical_device, index))
                            } else {
                                None
                            }
                        })
                })
                .find_map(|v| v)
        }
            .ok_or("Failed to find a Vulkan physical device.")?;

        let device: ash::Device = {
            let features = vk::PhysicalDeviceFeatures::default();

            let priorities = [1.0];

            let queue_info = [vk::DeviceQueueCreateInfo::default()
                .queue_family_index(queue_family_index as _)
                .queue_priorities(&priorities)];

            let device_extension_names_raw = [
                ash::khr::swapchain::NAME.as_ptr()
            ];

            let device_create_info = vk::DeviceCreateInfo::default()
                .queue_create_infos(&queue_info)
                .enabled_extension_names(&device_extension_names_raw)
                .enabled_features(&features);

            instance.create_device(physical_device, &device_create_info, None)
        }
            .or(Err("Failed to create Device."))?;

        log::info!("Created Vulkan device. {:?}", device.handle());

        let queue_index: usize = 0;
        let queue: vk::Queue = device.get_device_queue(queue_family_index as _, queue_index as _);

        let swap_chain_loader = ash::khr::swapchain::Device::new(&instance, &device);

        let semaphore_info = vk::SemaphoreCreateInfo::default();
        let render_finished_fence_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);
        let image_available_semaphore = unsafe { device.create_semaphore(&semaphore_info, None).unwrap() };
        let render_finished_semaphore = unsafe { device.create_semaphore(&semaphore_info, None).unwrap() };
        let render_finished_fence = unsafe { device.create_fence(&render_finished_fence_info, None).unwrap() };


        let pool_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(queue_family_index as _)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);  // Allow buffers to be reset

        let command_pool = unsafe {
            device.create_command_pool(&pool_info, None)
                .or(Err("Failed to create CommandPool."))?
        };

        let alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)  // Primary command buffers
            .command_buffer_count(3);

        let command_buffers = unsafe {
            device.allocate_command_buffers(&alloc_info)
                .or(Err("Failed to allocate CommandBuffers."))?
        };

        Ok(AshGraphics {
            current_index: None,
            alpha: false,
            swap_chain_image_view: None,
            swap_chain_loader,
            queue_and_index: (queue, queue_index),
            device,
            physical_device,
            instance,
            entry,
            image_available_semaphore,
            render_finished_semaphore,
            render_finished_fence,
            swap_chain: None,
            surface: None,
            command_buffers,
            render_pass: vec![],
            frame_buffers: vec![],
            surface_size: vk::Extent2D { width: 0, height: 0 },
            swap_chain_images: None,
        })
    }
}

pub struct VulkanContext {
    ash: AshGraphics,
}

impl Debug for VulkanContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // todo
        f.debug_struct("VulkanContext")
            .finish()
    }
}

impl VulkanContext {
    pub fn set_alpha(&mut self, alpha: bool) {
        self.ash.alpha = alpha;
    }

    pub fn alpha(&self) -> bool {
        self.ash.alpha
    }
    pub fn version() -> Option<(usize, usize, usize)> {
        AshGraphics::vulkan_version()
    }

    pub fn new(app_name: &str) -> Result<VulkanContext, String> {
        unsafe {
            AshGraphics::new(app_name).map(|graphics| {
                Self { ash: graphics }
            })
        }
    }

    pub fn queue(&self) -> u64 {
        self.ash.queue_and_index.0.as_raw()
    }

    pub fn index(&self) -> usize {
        self.ash.queue_and_index.1
    }

    pub fn instance_handle(&self) -> u64 {
        self.ash.instance.handle().as_raw()
    }

    pub fn physical_device(&self) -> u64 {
        self.ash.physical_device.as_raw()
    }

    pub fn device_handle(&self) -> u64 {
        self.ash.device.handle().as_raw()
    }

    pub fn current_image(&mut self) -> Option<vk::Image> {
        if self.ash.current_index.is_none() {
            unsafe {
                if let Ok((image_index, _)) = self.ash.swap_chain_loader.acquire_next_image(
                    self.ash.swap_chain?, u64::MAX, self.ash.image_available_semaphore,
                    vk::Fence::null(),
                ) {
                    self.ash.current_index = Some(image_index);
                }
            }
        }

        match self.ash.swap_chain_images.as_ref() {
            None => None,
            Some(images) => {
                if let Some(current_index) = self.ash.current_index {
                    return images.get(current_index as usize).map(|image| *image);
                }
                None
            }
        }
    }

    pub fn current_image_raw(&mut self) -> Option<u64> {
        match self.current_image() {
            None => None,
            Some(image) => {
                Some(image.as_raw())
            }
        }
    }

    pub fn next_image(&mut self) -> Option<vk::Image> {
        unsafe {
            if let Ok((image_index, _)) = self.ash.swap_chain_loader.acquire_next_image(
                self.ash.swap_chain?, u64::MAX, self.ash.image_available_semaphore,
                vk::Fence::null(),
            ) {
                self.ash.current_index = Some(image_index);
            }
        }

        match self.ash.swap_chain_images.as_ref() {
            None => None,
            Some(images) => {
                if let Some(current_index) = self.ash.current_index {
                    return images.get(current_index as usize).map(|image| *image);
                }
                None
            }
        }
    }

    pub fn next_image_raw(&mut self) -> Option<u64> {
        match self.next_image() {
            None => None,
            Some(image) => {
                Some(image.as_raw())
            }
        }
    }

    unsafe fn create_swap_chain(&mut self, surface: vk::SurfaceKHR, width: u32, height: u32) {
        let surface_loader = ash::khr::surface::Instance::new(&self.ash.entry, &self.ash.instance);
        let formats = unsafe {
            surface_loader.get_physical_device_surface_formats(self.ash.physical_device, surface).unwrap()
        };
        let capabilities = unsafe {
            surface_loader.get_physical_device_surface_capabilities(self.ash.physical_device, surface).unwrap()
        };

        let present_modes = unsafe {
            surface_loader.get_physical_device_surface_present_modes(self.ash.physical_device, surface).unwrap()
        };

        let raw_flags = vk::SwapchainCreateFlagsKHR::empty();
        let composite_alpha = if self.alpha() {
            if capabilities.supported_composite_alpha.contains(vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED) {
                vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED
            } else if (capabilities.supported_composite_alpha.contains(vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED)) {
                vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED
            } else {
                vk::CompositeAlphaFlagsKHR::OPAQUE
            }
        } else {
            vk::CompositeAlphaFlagsKHR::OPAQUE
        };

        let old_swap_chain = self.ash.swap_chain.unwrap_or_else(|| SwapchainKHR::null());

        let info = SwapchainCreateInfoKHR::default()
            .flags(raw_flags)
            .surface(surface)
            .min_image_count(capabilities.min_image_count)
            .image_format(formats[0].format)
            .image_color_space(formats[0].color_space)
            .image_extent(vk::Extent2D { width, height })
            .image_array_layers(1)
            .image_usage(capabilities.supported_usage_flags)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
            .composite_alpha(composite_alpha)
            .present_mode(present_modes[0])
            .clipped(true)
            .old_swapchain(old_swap_chain);


        let color_attachment = vk::AttachmentDescription {
            format: vk::Format::R8G8B8A8_UNORM,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            ..Default::default()
        };

        let color_attachment_ref = vec![vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }];

        let subpass = vk::SubpassDescription {
            pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            p_color_attachments: color_attachment_ref.as_ptr(),
            color_attachment_count: color_attachment_ref.len() as u32,
            ..Default::default()
        };

        let render_pass_info = vk::RenderPassCreateInfo {
            attachment_count: 1,
            p_attachments: &color_attachment,
            subpass_count: 1,
            p_subpasses: &subpass,
            dependency_count: 0,
            p_dependencies: std::ptr::null(),
            ..Default::default()
        };


        let render_pass = self.ash.device.create_render_pass(&render_pass_info, None).unwrap();

        self.ash.render_pass = vec![render_pass];

        self.ash.swap_chain = self.ash.swap_chain_loader.create_swapchain(&info, None)
            .map(|swap_chain| {
                self.ash.swap_chain_image_view = self.ash.swap_chain_loader.get_swapchain_images(swap_chain).map(|images| {
                    let views = images.iter()
                        .map(|image| {
                            let view_info = vk::ImageViewCreateInfo {
                                view_type: vk::ImageViewType::TYPE_2D,
                                format: info.image_format,
                                components: vk::ComponentMapping {
                                    r: vk::ComponentSwizzle::IDENTITY,
                                    g: vk::ComponentSwizzle::IDENTITY,
                                    b: vk::ComponentSwizzle::IDENTITY,
                                    a: vk::ComponentSwizzle::IDENTITY,
                                },
                                subresource_range: vk::ImageSubresourceRange {
                                    aspect_mask: vk::ImageAspectFlags::COLOR,
                                    base_mip_level: 0,
                                    level_count: 1,
                                    base_array_layer: 0,
                                    layer_count: 1,
                                },
                                image: *image,
                                ..Default::default()
                            };

                            let image_view = unsafe {
                                self.ash.device
                                    .create_image_view(&view_info, None).unwrap()
                            };

                            let attachments = [image_view];

                            let framebuffer_info = vk::FramebufferCreateInfo::default()
                                .render_pass(render_pass)
                                .attachments(&attachments)
                                .width(width)
                                .height(height)
                                .layers(1);

                            let frame_buffer = unsafe {
                                self.ash.device.create_framebuffer(&framebuffer_info, None).unwrap()
                            };

                            self.ash.frame_buffers.push(frame_buffer);

                            image_view
                        })
                        .collect::<Vec<ImageView>>();
                    self.ash.swap_chain_images = Some(images);
                    views
                }).ok();
                // self.ash.current_index = Some(0);
                swap_chain
            })
            .ok();

        self.ash.surface_size = vk::Extent2D {
            width,
            height,
        };
    }

    pub fn set_view(&mut self, view: *mut c_void, width: u32, height: u32) {
        self.ash.surface = match NonNull::new(view) {
            None => None,
            Some(view) => {
                let display_handle = raw_window_handle::AndroidDisplayHandle::new();
                let display_handle = raw_window_handle::RawDisplayHandle::Android(display_handle);
                let window_handle = raw_window_handle::AndroidNdkWindowHandle::new(view);
                let window_handle = raw_window_handle::RawWindowHandle::AndroidNdk(window_handle);

                unsafe {
                    match ash_window::create_surface(
                        &self.ash.entry,
                        &self.ash.instance,
                        display_handle,
                        window_handle,
                        None,
                    ) {
                        Ok(surface) => {
                            self.create_swap_chain(surface, width, height);
                            Some(surface)
                        }
                        Err(_) => {
                            None
                        }
                    }
                }
            }
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let surface = self.ash.surface.as_ref().map(|surface| *surface);
        if let Some(surface) = surface {
            unsafe { self.create_swap_chain(surface, width, height); }
            self.ash.surface = Some(surface)
        }
    }

    pub fn record_command_buffer(&mut self) {
        let _ = self.current_image();
        if let (Some(image_index), Some(swapchain)) = (self.ash.current_index, self.ash.swap_chain.as_ref()) {
            unsafe {
                let command_buffer_begin_info = vk::CommandBufferBeginInfo::default();
                let _ = self.ash.device.begin_command_buffer(self.ash.command_buffers[image_index as usize], &command_buffer_begin_info);

                let alpha = if self.alpha() {
                    [0.0, 0.0, 0.0, 0f32]
                }else {
                    [0.0, 0.0, 0.0, 1.]
                };

                let clear  = [vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: alpha,
                    },
                }];
                let render_pass_info = vk::RenderPassBeginInfo::default()
                    .render_pass(self.ash.render_pass[0])
                    .framebuffer(self.ash.frame_buffers[image_index as usize])
                    .render_area(vk::Rect2D {
                        offset: vk::Offset2D { x: 0, y: 0 },
                        extent: self.ash.surface_size,
                    })
                    .clear_values(&clear);

                unsafe {
                    self.ash.device.cmd_begin_render_pass(self.ash.command_buffers[image_index as usize], &render_pass_info, vk::SubpassContents::INLINE);
                }
            }
        }
    }

    pub fn present(&mut self) {
        let _ = self.current_image();
        unsafe {
            if let (Some(image_index), Some(swapchain)) = (self.ash.current_index, self.ash.swap_chain.as_ref()) {

                unsafe {
                    self.ash.device.cmd_end_render_pass(self.ash.command_buffers[image_index as usize]);
                    let _ = self.ash.device.end_command_buffer(self.ash.command_buffers[image_index as usize]);
                    // .expect("End Command Buffer");
                }

                let command_buffers = [self.ash.command_buffers[image_index as usize]];
                let image_available_semaphore = [self.ash.image_available_semaphore];
                let render_finished_semaphore = [self.ash.render_finished_semaphore];
                let submit_info = vk::SubmitInfo::default()
                    .wait_semaphores(&image_available_semaphore)
                    .wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
                    .command_buffers(&command_buffers)
                    .signal_semaphores(&render_finished_semaphore);

                unsafe {
                    let _ = self.ash.device
                        .queue_submit(self.ash.queue_and_index.0, &[submit_info], self.ash.render_finished_fence);

                    // .expect("Queue Submit Failed.");
                }

                let swapchains = [*swapchain];
                let image_indices = [image_index];

                let present_info = vk::PresentInfoKHR::default()
                    .wait_semaphores(&render_finished_semaphore)
                    .swapchains(&swapchains)
                    .image_indices(&image_indices);

                unsafe {
                    self.ash.swap_chain_loader
                        .queue_present(self.ash.queue_and_index.0, &present_info)
                        .expect("Queue Present Failed.");
                }


                let _ = self.ash.device
                    .wait_for_fences(&[self.ash.render_finished_fence], true, u64::MAX);
                let _ = self.ash.device.reset_fences(&[self.ash.render_finished_fence]);

                let _ = self.next_image();
            }
        }
    }

    pub unsafe fn get_instance_proc_addr(&self, instance: *mut c_void, name: *const c_char) -> Option<unsafe extern "system" fn()> {
        let vk_instance = vk::Instance::from_raw(instance as _);
        self.ash.entry.get_instance_proc_addr(vk_instance, name)
    }

    pub unsafe fn get_device_proc_addr(&self, device: *mut c_void, name: *const c_char) -> Option<unsafe extern "system" fn()> {
        let vk_device = vk::Device::from_raw(device as _);
        self.ash.instance.get_device_proc_addr(vk_device, name)
    }
}
