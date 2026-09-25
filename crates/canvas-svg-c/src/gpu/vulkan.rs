//! Vulkan rasterization. The swapchain hands out a new image each frame, so the Skia surface is
//! re-pointed after every present.

use canvas_core::context_attributes::ColorSpace;
use canvas_core::gpu::vulkan::{PresentStatus, VulkanContext};
use skia_safe::gpu::{self, DirectContext, SurfaceOrigin};
use skia_safe::{ColorType, Surface};
use std::ffi::CStr;

use super::Frame;

/// Field order is drop order: the surface and texture reference images `vk` owns, so it drops last.
pub struct VulkanSurface {
    surface: Surface,
    texture: gpu::BackendTexture,
    direct: DirectContext,
    vk: VulkanContext,
    width: i32,
    height: i32,
}

impl VulkanSurface {
    pub fn new(window: *mut std::ffi::c_void, width: i32, height: i32) -> Option<Self> {
        let mut vk = match VulkanContext::new("ns-svg", ColorSpace::Srgb) {
            Ok(vk) => vk,
            Err(cause) => {
                log::warn!("svg gpu: vulkan context: {cause}");
                return None;
            }
        };
        vk.set_alpha(true);

        let mut direct = {
            let get_proc = |of| unsafe {
                let resolved = match of {
                    gpu::vk::GetProcOf::Instance(instance, name) => {
                        vk.get_instance_proc_addr(instance as _, name).map(|f| (f, name))
                    }
                    gpu::vk::GetProcOf::Device(device, name) => {
                        vk.get_device_proc_addr(device as _, name).map(|f| (f, name))
                    }
                };
                match resolved {
                    Some((f, _)) => f as _,
                    None => {
                        let name = match of {
                            gpu::vk::GetProcOf::Instance(_, name)
                            | gpu::vk::GetProcOf::Device(_, name) => CStr::from_ptr(name),
                        };
                        log_resolve_failure(&name.to_string_lossy());
                        std::ptr::null()
                    }
                }
            };

            let backend = unsafe {
                gpu::vk::BackendContext::new(
                    vk.instance_handle() as _,
                    vk.physical_device() as _,
                    vk.device_handle() as _,
                    (vk.queue() as _, vk.index()),
                    &get_proc,
                )
            };

            gpu::direct_contexts::make_vulkan(&backend, None)
        }?;

        vk.set_view(window, width as u32, height as u32);

        let (surface, texture) = wrap_current_image(&mut vk, &mut direct, width, height)?;

        Some(Self {
            vk,
            direct,
            surface,
            texture,
            width,
            height,
        })
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        if width == self.width && height == self.height {
            return;
        }
        self.width = width;
        self.height = height;
        self.rebuild_swapchain();
    }

    pub fn render(&mut self, paint: &dyn Fn(&skia_safe::Canvas, i32, i32)) -> Frame {
        if self.direct.is_device_lost() {
            return Frame::Lost;
        }
        let (width, height) = (self.width, self.height);
        {
            let canvas = self.surface.canvas();
            canvas.clear(skia_safe::Color::TRANSPARENT);
            paint(canvas, width, height);
        }
        // No semaphore: relies on same-queue submission order, as `VulkanContext::present`
        // (shared with canvas-2d) waits on none.
        self.direct.flush_and_submit();
        match self.vk.present() {
            PresentStatus::Ok => {
                self.replace_backend_texture();
                Frame::Presented
            }
            // Rotation, fold or unseen resize: the device is fine, so rebuild the swapchain only.
            PresentStatus::OutOfDate => {
                if self.rebuild_swapchain() {
                    Frame::Skipped
                } else {
                    Frame::Lost
                }
            }
            PresentStatus::Lost => Frame::Lost,
        }
    }

    /// Skia must finish with the old images first; they are destroyed underneath it.
    fn rebuild_swapchain(&mut self) -> bool {
        self.direct.flush_submit_and_sync_cpu();
        if !self.vk.resize(self.width as u32, self.height as u32) {
            return false;
        }
        let Some((surface, texture)) =
            wrap_current_image(&mut self.vk, &mut self.direct, self.width, self.height)
        else {
            return false;
        };
        self.surface = surface;
        self.texture = texture;
        true
    }

    fn replace_backend_texture(&mut self) {
        let Some(texture) = make_backend_texture(&mut self.vk, self.width, self.height) else {
            return;
        };
        self.surface
            .replace_backend_texture(&texture, SurfaceOrigin::TopLeft);
        self.texture = texture;
    }
}

impl Drop for VulkanSurface {
    fn drop(&mut self) {
        // Skia must be done with the swapchain images before the context tears them down.
        self.direct.flush_submit_and_sync_cpu();
        self.direct.abandon();
    }
}

fn make_backend_texture(
    vk: &mut VulkanContext,
    width: i32,
    height: i32,
) -> Option<gpu::BackendTexture> {
    let image = vk.current_image_raw()?;
    let info = unsafe {
        gpu::vk::ImageInfo::new(
            image as gpu::vk::Image,
            gpu::vk::Alloc::default(),
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
    Some(unsafe { gpu::backend_textures::make_vk((width, height), &info, "") })
}

fn wrap_current_image(
    vk: &mut VulkanContext,
    direct: &mut DirectContext,
    width: i32,
    height: i32,
) -> Option<(Surface, gpu::BackendTexture)> {
    let texture = make_backend_texture(vk, width, height)?;
    let surface = gpu::surfaces::wrap_backend_texture(
        direct,
        &texture,
        SurfaceOrigin::TopLeft,
        None,
        ColorType::N32,
        None,
        None,
    )?;
    Some((surface, texture))
}

fn log_resolve_failure(name: &str) {
    log::error!("vulkan: resolve of {name} failed");
}
