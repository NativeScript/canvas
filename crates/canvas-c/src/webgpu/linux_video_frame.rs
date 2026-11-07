//! Linux: canvas-media frames (`CanvasLinuxVideoFrame`) as wgpu textures.
//!
//! A GL frame is blitted by its producer into an exportable image of ours (one per video,
//! reused). As with Windows' fences, the submit reading it waits for the producer's `ready`
//! semaphore and signals `release`, which the producer waits for before writing it again. Without
//! VK_KHR_external_semaphore_fd the producer finishes its blit and we wait for our reads instead.
//! Memory frames are written into a texture of ours: one upload.

use std::borrow::Cow;
use std::os::raw::c_void;
use std::sync::Arc;

use ash::vk;

use super::gpu_device::CanvasGPUDevice;
use super::gpu_queue::CanvasGPUQueue;
use crate::linux_video_frame::{CanvasLinuxFrameTarget, KIND_GL, KIND_MEMORY};

struct SharedImage {
    producer: u64,
    id: u64,
    width: u32,
    height: u32,
    texture: Arc<wgpu_core::resource::Texture>,
    allocation_size: u64,
    /// Handed over with the first copy.
    fds: Option<(i32, i32, i32)>,
    semaphores: Option<(vk::Semaphore, vk::Semaphore)>,
    raw: ash::Device,
    release_pending: bool,
    read: bool,
}

impl Drop for SharedImage {
    fn drop(&mut self) {
        unsafe {
            if let Some((memory, ready, release)) = self.fds.take() {
                for fd in [memory, ready, release] {
                    if fd >= 0 {
                        libc::close(fd);
                    }
                }
            }
            if let Some((ready, release)) = self.semaphores.take() {
                self.raw.destroy_semaphore(ready, None);
                self.raw.destroy_semaphore(release, None);
            }
        }
    }
}

#[derive(Default)]
pub struct LinuxFrameCache {
    images: Vec<SharedImage>,
    uploads: Vec<Arc<wgpu_core::resource::Texture>>,
    /// The device's UUID and whether it exports semaphores.
    device: Option<Option<([u8; 16], bool)>>,
}

const MAX_IMAGES: usize = 4;
static NEXT_IMAGE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

unsafe fn device_info(device: &Arc<wgpu_core::device::Device>) -> Option<([u8; 16], bool)> {
    let hal = unsafe { Arc::clone(device).as_hal::<wgpu_hal::api::Vulkan>() }?;
    let instance = hal.shared_instance().raw_instance();
    let mut id = vk::PhysicalDeviceIDProperties::default();
    let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut id);
    unsafe { instance.get_physical_device_properties2(hal.raw_physical_device(), &mut properties) };
    let semaphores = hal
        .enabled_device_extensions()
        .contains(&ash::khr::external_semaphore_fd::NAME);
    log::info!("canvas: WebGPU video frames from GL synchronize with semaphores: {semaphores}");
    Some((id.device_uuid, semaphores))
}

impl LinuxFrameCache {
    fn device(&mut self, device: &Arc<wgpu_core::device::Device>) -> Option<([u8; 16], bool)> {
        *self
            .device
            .get_or_insert_with(|| unsafe { device_info(device) })
    }
}

/// Two exportable binary semaphores and their OPAQUE_FDs.
unsafe fn semaphores(
    device: &Arc<wgpu_core::device::Device>,
) -> Option<((vk::Semaphore, vk::Semaphore), (i32, i32))> {
    let hal = unsafe { Arc::clone(device).as_hal::<wgpu_hal::api::Vulkan>() }?;
    let raw = hal.raw_device();
    let fd_fn =
        ash::khr::external_semaphore_fd::Device::new(hal.shared_instance().raw_instance(), raw);
    let mut made = Vec::with_capacity(2);
    for _ in 0..2 {
        let mut export = vk::ExportSemaphoreCreateInfo::default()
            .handle_types(vk::ExternalSemaphoreHandleTypeFlags::OPAQUE_FD);
        let info = vk::SemaphoreCreateInfo::default().push_next(&mut export);
        let Ok(semaphore) = (unsafe { raw.create_semaphore(&info, None) }) else {
            break;
        };
        let fd = unsafe {
            fd_fn.get_semaphore_fd(
                &vk::SemaphoreGetFdInfoKHR::default()
                    .semaphore(semaphore)
                    .handle_type(vk::ExternalSemaphoreHandleTypeFlags::OPAQUE_FD),
            )
        };
        match fd {
            Ok(fd) => made.push((semaphore, fd)),
            Err(_) => {
                unsafe { raw.destroy_semaphore(semaphore, None) };
                break;
            }
        }
    }
    if made.len() != 2 {
        for (semaphore, fd) in made {
            unsafe {
                raw.destroy_semaphore(semaphore, None);
                libc::close(fd);
            }
        }
        return None;
    }
    Some(((made[0].0, made[1].0), (made[0].1, made[1].1)))
}

unsafe fn create_shared(
    device: &Arc<wgpu_core::device::Device>,
    producer: u64,
    width: u32,
    height: u32,
    with_semaphores: bool,
) -> Option<SharedImage> {
    let size = wgt::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    // GL leaves it in SHADER_READ_ONLY (its semaphore signal says so): RESOURCE from the start.
    let (texture, fd, allocation_size) = unsafe {
        super::linux_frames::create_exportable_texture(
            device,
            size,
            wgt::TextureFormat::Rgba8Unorm,
            wgt::TextureUsages::TEXTURE_BINDING
                | wgt::TextureUsages::RENDER_ATTACHMENT
                | wgt::TextureUsages::COPY_SRC
                | wgt::TextureUsages::COPY_DST,
            vec![],
            wgt::TextureUses::RESOURCE,
            "videoBlit:SharedFrame",
        )
    }?;
    let (semaphores, (ready_fd, release_fd)) = match with_semaphores {
        true => match unsafe { self::semaphores(device) } {
            Some((semaphores, fds)) => (Some(semaphores), fds),
            None => (None, (-1, -1)),
        },
        false => (None, (-1, -1)),
    };
    let raw = unsafe { Arc::clone(device).as_hal::<wgpu_hal::api::Vulkan>() }?
        .raw_device()
        .clone();
    Some(SharedImage {
        producer,
        id: NEXT_IMAGE.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        width,
        height,
        texture,
        allocation_size,
        fds: Some((fd, ready_fd, release_fd)),
        semaphores,
        raw,
        release_pending: false,
        read: false,
    })
}

fn wait_idle(device: &Arc<wgpu_core::device::Device>) {
    let _ = device.poll(wgt::PollType::Wait {
        submission_index: None,
        timeout: None,
    });
}

unsafe fn shared(
    queue: &CanvasGPUQueue,
    frame: &mut crate::linux_video_frame::CanvasLinuxVideoFrame,
) -> Option<Arc<wgpu_core::resource::Texture>> {
    let copy_to = frame.copy_to?;
    let device = &queue.device_id;
    let mut cache = queue.linux_frames.lock();
    let (uuid, with_semaphores) = cache.device(device)?;
    if uuid != frame.device_uuid {
        return None;
    }
    let (width, height) = (frame.width, frame.height);
    let index = match cache.images.iter().position(|image| {
        (image.producer, image.width, image.height) == (frame.producer, width, height)
    }) {
        Some(index) => index,
        None => {
            let stale = cache
                .images
                .iter()
                .any(|image| image.producer == frame.producer)
                || cache.images.len() >= MAX_IMAGES;
            if stale {
                // Their semaphores may be in flight.
                wait_idle(device);
                cache
                    .images
                    .retain(|image| image.producer != frame.producer);
                if cache.images.len() >= MAX_IMAGES {
                    cache.images.remove(0);
                }
            }
            let image =
                unsafe { create_shared(device, frame.producer, width, height, with_semaphores) }?;
            cache.images.push(image);
            cache.images.len() - 1
        }
    };
    let image = &mut cache.images[index];
    if image.semaphores.is_none() && image.read {
        // No release semaphore: our last read must be over before GL writes.
        wait_idle(device);
    }
    let (memory_fd, ready_fd, release_fd) = image.fds.take().unwrap_or((-1, -1, -1));
    let target = CanvasLinuxFrameTarget {
        size: std::mem::size_of::<CanvasLinuxFrameTarget>() as u32,
        memory_fd,
        id: image.id,
        allocation_size: image.allocation_size,
        width,
        height,
        ready_fd,
        release_fd,
        wait_release: image.release_pending as u32,
        reserved: 0,
    };
    if !unsafe { copy_to(frame, &target) } {
        // Its fds may be gone: start over with a new image next time.
        cache.images.remove(index);
        return None;
    }
    let image = &mut cache.images[index];
    image.read = true;
    if let Some((ready, release)) = image.semaphores {
        let hal_queue = unsafe { Arc::clone(&queue.queue.id).as_hal::<wgpu_hal::api::Vulkan>() }?;
        hal_queue.add_wait_semaphore(ready, None, vk::PipelineStageFlags::ALL_COMMANDS);
        hal_queue.add_signal_semaphore(release, None);
        image.release_pending = true;
    }
    Some(Arc::clone(&image.texture))
}

unsafe fn upload(
    queue: &CanvasGPUQueue,
    frame: &crate::linux_video_frame::CanvasLinuxVideoFrame,
) -> Option<Arc<wgpu_core::resource::Texture>> {
    let (width, height) = (frame.width, frame.height);
    if frame.data.is_null() || frame.stride < width * 4 {
        return None;
    }
    let texture = {
        let mut cache = queue.linux_frames.lock();
        let fits = |texture: &Arc<wgpu_core::resource::Texture>| {
            let size = texture.descriptor().size;
            (size.width, size.height) == (width, height)
        };
        match cache.uploads.iter().find(|texture| fits(texture)) {
            Some(texture) => Arc::clone(texture),
            None => {
                let texture =
                    queue
                        .device_id
                        .create_texture(&wgpu_core::resource::TextureDescriptor {
                            label: Some(Cow::Borrowed("videoBlit:Upload")),
                            size: wgt::Extent3d {
                                width,
                                height,
                                depth_or_array_layers: 1,
                            },
                            mip_level_count: 1,
                            sample_count: 1,
                            dimension: wgt::TextureDimension::D2,
                            format: wgt::TextureFormat::Rgba8Unorm,
                            usage: wgt::TextureUsages::TEXTURE_BINDING
                                | wgt::TextureUsages::COPY_DST,
                            view_formats: vec![],
                        });
                cache.uploads.retain(|texture| fits(texture));
                cache.uploads.push(Arc::clone(&texture));
                texture
            }
        }
    };
    let length = frame.stride as usize * (height as usize - 1) + width as usize * 4;
    let data = unsafe { std::slice::from_raw_parts(frame.data, length) };
    queue.queue.id.write_texture(
        wgt::TexelCopyTextureInfo {
            texture: Arc::clone(&texture),
            mip_level: 0,
            origin: wgt::Origin3d::ZERO,
            aspect: wgt::TextureAspect::All,
        },
        data,
        &wgt::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(frame.stride),
            rows_per_image: Some(height),
        },
        &wgt::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    Some(texture)
}

/// The frame as a texture to sample in the queue's next submit, which must follow at once.
pub(crate) unsafe fn import_and_stage(
    queue: &CanvasGPUQueue,
    handle: *mut c_void,
    width: u32,
    height: u32,
) -> Option<Arc<wgpu_core::resource::Texture>> {
    let frame = unsafe { crate::linux_video_frame::frame(handle) }?;
    if (frame.width, frame.height) != (width, height) {
        return None;
    }
    let kind = frame.kind;
    let texture = match kind {
        KIND_GL => unsafe { shared(queue, frame) },
        KIND_MEMORY => unsafe { upload(queue, frame) },
        _ => None,
    };
    crate::linux_video_frame::log_path("WebGPU", kind, texture.is_some());
    texture
}

/// Drawn into a plane of our own at once: the external texture may be sampled after the producer
/// wrote the next frame.
pub(crate) unsafe fn import_external_plane(
    device: &CanvasGPUDevice,
    handle: *mut c_void,
    width: u32,
    height: u32,
) -> Option<Arc<wgpu_core::resource::Texture>> {
    let queue = &*device.queue;
    let size = wgt::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    let plane = device
        .device
        .create_texture(&wgpu_core::resource::TextureDescriptor {
            label: Some(Cow::Borrowed("externalTexture:Plane")),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgt::TextureDimension::D2,
            format: wgt::TextureFormat::Rgba8Unorm,
            usage: wgt::TextureUsages::TEXTURE_BINDING | wgt::TextureUsages::RENDER_ATTACHMENT,
            view_formats: vec![],
        });
    let source = unsafe { import_and_stage(queue, handle, width, height) }?;
    unsafe {
        super::gpu_native_texture::blit_texture(
            queue,
            &source,
            width,
            height,
            0,
            0,
            false,
            &plane,
            0,
            0,
            (0, 0),
            size,
        )
    }
    .then_some(plane)
}

/// 53 bits of the device UUID (as canvas-media folds GL's), for `supportsGPUFrames`; 0 if
/// unknown.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_get_uuid_key(
    device: *const CanvasGPUDevice,
) -> u64 {
    let Some(device) = (unsafe { device.as_ref() }) else {
        return 0;
    };
    let Some((uuid, _)) = device.queue.linux_frames.lock().device(&device.device) else {
        return 0;
    };
    let mut key = 0u64;
    for (i, byte) in uuid.iter().enumerate() {
        key ^= (*byte as u64) << ((i % 7) * 8);
    }
    key & ((1 << 53) - 1)
}
