use ash::vk::{Extent2D, Handle, ImageView, SwapchainCreateInfoKHR, SwapchainKHR};
use ash::{vk, Entry, Instance};
use skia_safe::wrapper::NativeTransmutableWrapper;
use std::ffi::CString;
use std::fmt::{Debug, Formatter};
use std::os::raw::{c_char, c_void};
use std::ptr::NonNull;

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
    pub entry: Entry,
    pub instance: Instance,
    pub image_available_semaphore: vk::Semaphore,
    pub surface_size: vk::Extent2D,
    pub alpha: bool,
    pub surface_loader: ash::khr::surface::Instance,
}

impl Drop for AshGraphics {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();

            if let Some(surface) = self.surface {
                self.surface_loader.destroy_surface(surface, None);
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

        let minimum_version = vk::make_api_version(0, 1, 1, 0);

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

            let create_flags = vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR;

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

        let queue_index: usize = 0;
        let queue: vk::Queue = device.get_device_queue(queue_family_index as _, queue_index as _);

        let swap_chain_loader = ash::khr::swapchain::Device::new(&instance, &device);

        let semaphore_info = vk::SemaphoreCreateInfo::default();
        let image_available_semaphore = unsafe { device.create_semaphore(&semaphore_info, None).unwrap() };

        let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);

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
            swap_chain: None,
            surface: None,
            surface_size: Extent2D { width: 0, height: 0 },
            swap_chain_images: None,
            surface_loader
        })
    }
}

pub struct VulkanContext {
    ash: AshGraphics,
    view: *mut c_void,
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
                Self { ash: graphics, view: std::ptr::null_mut() }
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

    pub fn size(&self) -> Extent2D {
        self.ash.surface_size
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
        unsafe { self.ash.device.device_wait_idle().unwrap(); }

        self.ash.current_index = None;

        let formats = unsafe {
            self.ash.surface_loader.get_physical_device_surface_formats(self.ash.physical_device, surface).unwrap()
        };

        let capabilities = unsafe {
            self.ash.surface_loader.get_physical_device_surface_capabilities(self.ash.physical_device, surface).unwrap()
        };

        let present_modes = unsafe {
            self.ash.surface_loader.get_physical_device_surface_present_modes(self.ash.physical_device, surface).unwrap()
        };

        let raw_flags = vk::SwapchainCreateFlagsKHR::empty();
        let composite_alpha = if self.alpha() {
            if capabilities.supported_composite_alpha.contains(vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED) {
                vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED
            } else if (capabilities.supported_composite_alpha.contains(vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED)) {
                vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED
            } else if (capabilities.supported_composite_alpha.contains(vk::CompositeAlphaFlagsKHR::INHERIT)) {
                vk::CompositeAlphaFlagsKHR::INHERIT
            } else {
                vk::CompositeAlphaFlagsKHR::OPAQUE
            }
        } else {
            vk::CompositeAlphaFlagsKHR::OPAQUE
        };

        if let Some(swap_chain) = self.ash.swap_chain.take() {
            self.ash.swap_chain_loader.destroy_swapchain(swap_chain, None);
        }


        if let Some(views) = self.ash.swap_chain_image_view.as_mut() {
            views.iter().for_each(|image| {
                self.ash.device.destroy_image_view(*image, None);
            });
            views.clear();
        }
        //  let old_swap_chain = self.ash.swap_chain.unwrap_or_else(|| );

        let min_image_count = if capabilities.min_image_count + 1 <= capabilities.max_image_count {
            capabilities.min_image_count + 1
        }else {
            capabilities.max_image_count
        };

        let info = SwapchainCreateInfoKHR::default()
            .flags(raw_flags)
            .surface(surface)
            .min_image_count(min_image_count)
            .image_format(formats[0].format)
            .image_color_space(formats[0].color_space)
            .image_extent(Extent2D { width, height })
            .image_array_layers(1)
            .image_usage(capabilities.supported_usage_flags)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
            .composite_alpha(composite_alpha)
            .present_mode(present_modes[0])
            .clipped(true)
            .old_swapchain(SwapchainKHR::null());


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

                            unsafe {
                                self.ash.device
                                    .create_image_view(&view_info, None).unwrap()
                            }
                        })
                        .collect::<Vec<ImageView>>();
                    self.ash.swap_chain_images = Some(images);
                    views
                }).ok();
                swap_chain
            })
            .ok();

        self.ash.surface_size = Extent2D {
            width,
            height,
        };
    }

    pub fn set_view(&mut self, view: *mut c_void, width: u32, height: u32) {
        self.view = view;
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
        let mut surface = None;
        if let Some(value) = self.ash.surface.as_ref() {
          surface = Some(*value);
        }

        if let Some(surface) = surface {
            unsafe { self.create_swap_chain(surface, width, height); }
        }
    }

    pub fn present(&mut self) {
        let _ = self.current_image();
        unsafe {
            if let (Some(image_index), Some(swapchain)) = (self.ash.current_index, self.ash.swap_chain.as_ref()) {
                let swapchains = [*swapchain];
                let image_indices = [image_index];

                let present_info = vk::PresentInfoKHR::default()
                    .swapchains(&swapchains)
                    .image_indices(&image_indices);

                unsafe {
                    self.ash.swap_chain_loader
                        .queue_present(self.ash.queue_and_index.0, &present_info)
                        .expect("Queue Present Failed.");
                }

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
