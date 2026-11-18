//! Android: a GL 2D canvas copied into a WebGPU texture with no readback. Each canvas draws into
//! an AHardwareBuffer of ours that Vulkan imports. The submit sampling it waits for the canvas's
//! native fence (imported as a SYNC_FD semaphore) and signals one exported back to the canvas,
//! which waits for it before drawing into the buffer again.

use std::ffi::c_void;
use std::sync::Arc;
use std::time::{Duration, Instant};

use ash::vk;
use parking_lot::Mutex;

use super::gpu_queue::CanvasGPUQueue;
use super::gpu_command_encoder::CanvasImageCopyTexture;
use super::structs::{CanvasExtent3d, CanvasImageCopyCanvasRenderingContext2D};

const AHARDWAREBUFFER_FORMAT_R8G8B8A8_UNORM: u32 = 1;
const AHARDWAREBUFFER_USAGE_GPU_SAMPLED_IMAGE: u64 = 1 << 8;
const AHARDWAREBUFFER_USAGE_GPU_COLOR_OUTPUT: u64 = 1 << 9;

#[repr(C)]
struct AHardwareBufferDesc {
    width: u32,
    height: u32,
    layers: u32,
    format: u32,
    usage: u64,
    stride: u32,
    rfu0: u32,
    rfu1: u64,
}

struct Ahb {
    allocate: unsafe extern "C" fn(*const AHardwareBufferDesc, *mut *mut c_void) -> i32,
    release: unsafe extern "C" fn(*mut c_void),
}

/// API 26+, so looked up rather than linked.
fn ahb() -> Option<&'static Ahb> {
    static AHB: std::sync::OnceLock<Option<Ahb>> = std::sync::OnceLock::new();
    AHB.get_or_init(|| unsafe {
        let allocate = libc::dlsym(libc::RTLD_DEFAULT, c"AHardwareBuffer_allocate".as_ptr());
        let release = libc::dlsym(libc::RTLD_DEFAULT, c"AHardwareBuffer_release".as_ptr());
        (!allocate.is_null() && !release.is_null()).then(|| Ahb {
            allocate: std::mem::transmute(allocate),
            release: std::mem::transmute(release),
        })
    })
    .as_ref()
}

struct Entry {
    producer: u64,
    id: u64,
    width: u32,
    height: u32,
    buffer: *mut c_void,
    /// Signalled by each copy's submit, then exported to `release_fd`.
    release: vk::Semaphore,
    release_fd: i32,
    used: Instant,
}

struct Device {
    raw: ash::Device,
    fd: ash::khr::external_semaphore_fd::Device,
}

#[derive(Default)]
pub struct AndroidCanvasCache {
    entries: Vec<Entry>,
    /// `None` once the device is known to lack SYNC_FD semaphores.
    device: Option<Option<Device>>,
    /// Ready semaphores whose waits have completed, for reuse.
    retired: Arc<Mutex<Vec<vk::Semaphore>>>,
}

unsafe impl Send for AndroidCanvasCache {}

static NEXT_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

impl Drop for AndroidCanvasCache {
    fn drop(&mut self) {
        let Some(Some(device)) = self.device.take() else {
            return;
        };
        unsafe {
            let _ = device.raw.device_wait_idle();
            for entry in self.entries.drain(..) {
                destroy_entry(&device, entry);
            }
            for semaphore in self.retired.lock().drain(..) {
                device.raw.destroy_semaphore(semaphore, None);
            }
        }
    }
}

unsafe fn destroy_entry(device: &Device, entry: Entry) {
    device.raw.destroy_semaphore(entry.release, None);
    if entry.release_fd >= 0 {
        libc::close(entry.release_fd);
    }
    if let Some(ahb) = ahb() {
        (ahb.release)(entry.buffer);
    }
}

/// The device, when it can import and export SYNC_FD semaphores.
unsafe fn device_of(queue: &CanvasGPUQueue) -> Option<Device> {
    let hal = Arc::clone(&queue.device_id).as_hal::<wgpu_hal::api::Vulkan>()?;
    let extensions = hal.enabled_device_extensions();
    if !extensions.contains(&ash::khr::external_semaphore_fd::NAME)
        || !extensions.contains(&ash::android::external_memory_android_hardware_buffer::NAME)
    {
        return None;
    }
    let instance = hal.shared_instance().raw_instance();
    let mut properties = vk::ExternalSemaphoreProperties::default();
    instance.get_physical_device_external_semaphore_properties(
        hal.raw_physical_device(),
        &vk::PhysicalDeviceExternalSemaphoreInfo::default()
            .handle_type(vk::ExternalSemaphoreHandleTypeFlags::SYNC_FD),
        &mut properties,
    );
    let needed = vk::ExternalSemaphoreFeatureFlags::EXPORTABLE | vk::ExternalSemaphoreFeatureFlags::IMPORTABLE;
    if !properties.external_semaphore_features.contains(needed) {
        return None;
    }
    let raw = hal.raw_device().clone();
    let fd = ash::khr::external_semaphore_fd::Device::new(instance, &raw);
    Some(Device { raw, fd })
}

fn wait_idle(device: &Arc<wgpu_core::device::Device>) {
    let _ = device.poll(wgt::PollType::Wait {
        submission_index: None,
        timeout: None,
    });
}

unsafe fn semaphore(device: &Device, exportable: bool) -> Option<vk::Semaphore> {
    let mut export = vk::ExportSemaphoreCreateInfo::default().handle_types(vk::ExternalSemaphoreHandleTypeFlags::SYNC_FD);
    let mut info = vk::SemaphoreCreateInfo::default();
    if exportable {
        info = info.push_next(&mut export);
    }
    device.raw.create_semaphore(&info, None).ok()
}

/// The canvas's entry, made (with its buffer) the first time and again after a resize. Canvases
/// keep theirs until they go a second without a copy.
unsafe fn entry(
    queue: &CanvasGPUQueue,
    cache: &mut AndroidCanvasCache,
    producer: u64,
    width: u32,
    height: u32,
) -> Option<usize> {
    if let Some(index) = cache
        .entries
        .iter()
        .position(|e| (e.producer, e.width, e.height) == (producer, width, height))
    {
        return Some(index);
    }
    let ahb = ahb()?;
    let idle = |e: &Entry| e.producer == producer || e.used.elapsed() >= Duration::from_secs(1);
    if cache.entries.iter().any(idle) {
        // Their semaphores may be in flight.
        wait_idle(&queue.device_id);
        let device = cache.device.as_ref()?.as_ref()?;
        let (gone, kept): (Vec<_>, Vec<_>) = cache.entries.drain(..).partition(idle);
        cache.entries = kept;
        for entry in gone {
            destroy_entry(device, entry);
        }
    }
    let device = cache.device.as_ref()?.as_ref()?;
    let desc = AHardwareBufferDesc {
        width,
        height,
        layers: 1,
        format: AHARDWAREBUFFER_FORMAT_R8G8B8A8_UNORM,
        usage: AHARDWAREBUFFER_USAGE_GPU_SAMPLED_IMAGE | AHARDWAREBUFFER_USAGE_GPU_COLOR_OUTPUT,
        stride: 0,
        rfu0: 0,
        rfu1: 0,
    };
    let mut buffer = std::ptr::null_mut();
    if (ahb.allocate)(&desc, &mut buffer) != 0 || buffer.is_null() {
        return None;
    }
    let Some(release) = semaphore(device, true) else {
        (ahb.release)(buffer);
        return None;
    };
    cache.entries.push(Entry {
        producer,
        id: NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        width,
        height,
        buffer,
        release,
        release_fd: -1,
        used: Instant::now(),
    });
    Some(cache.entries.len() - 1)
}

/// The buffer as a texture wgpu takes to be in the shader-read layout already: `transfer`'s
/// acquire puts it there. A transition from UNDEFINED on wgpu's own queue family may discard
/// what GL drew.
unsafe fn import(
    queue: &CanvasGPUQueue,
    buffer: *mut c_void,
    width: u32,
    height: u32,
) -> Option<Arc<wgpu_core::resource::Texture>> {
    let size = wgt::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    let hal_texture = {
        let hal = Arc::clone(&queue.device_id).as_hal::<wgpu_hal::api::Vulkan>()?;
        let descriptor = wgpu_hal::TextureDescriptor {
            label: Some("canvasCopy:Buffer"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgt::TextureDimension::D2,
            format: wgt::TextureFormat::Rgba8Unorm,
            usage: wgt::TextureUses::RESOURCE,
            memory_flags: wgpu_hal::MemoryFlags::empty(),
            view_formats: vec![],
            texture_binding_view_dimension: None,
        };
        let raw = hal.raw_device();
        let instance = hal.shared_instance().raw_instance();
        let ahb_fns = ash::android::external_memory_android_hardware_buffer::Device::new(instance, raw);
        let mut format = vk::AndroidHardwareBufferFormatPropertiesANDROID::default();
        let mut properties = vk::AndroidHardwareBufferPropertiesANDROID::default().push_next(&mut format);
        ahb_fns
            .get_android_hardware_buffer_properties(buffer.cast(), &mut properties)
            .ok()?;
        let (allocation_size, type_bits) = (properties.allocation_size, properties.memory_type_bits);
        if format.format != vk::Format::R8G8B8A8_UNORM || type_bits == 0 {
            return None;
        }
        // The usages the buffer was allocated for: some drivers lay out an image by its usage,
        // and one created for less than the buffer's own may not read what GL wrote.
        let mut external = vk::ExternalMemoryImageCreateInfo::default()
            .handle_types(vk::ExternalMemoryHandleTypeFlags::ANDROID_HARDWARE_BUFFER_ANDROID);
        let image_info = vk::ImageCreateInfo::default()
            .push_next(&mut external)
            .image_type(vk::ImageType::TYPE_2D)
            .format(vk::Format::R8G8B8A8_UNORM)
            .extent(vk::Extent3D { width, height, depth: 1 })
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(
                vk::ImageUsageFlags::SAMPLED
                    | vk::ImageUsageFlags::COLOR_ATTACHMENT
                    | vk::ImageUsageFlags::TRANSFER_SRC
                    | vk::ImageUsageFlags::TRANSFER_DST,
            )
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .initial_layout(vk::ImageLayout::UNDEFINED);
        let image = raw.create_image(&image_info, None).ok()?;
        let mut dedicated = vk::MemoryDedicatedAllocateInfo::default().image(image);
        let mut import = vk::ImportAndroidHardwareBufferInfoANDROID::default().buffer(buffer.cast());
        let allocate = vk::MemoryAllocateInfo::default()
            .push_next(&mut dedicated)
            .push_next(&mut import)
            .allocation_size(allocation_size)
            .memory_type_index(type_bits.trailing_zeros());
        let memory = match raw.allocate_memory(&allocate, None) {
            Ok(memory) => memory,
            Err(_) => {
                raw.destroy_image(image, None);
                return None;
            }
        };
        if raw.bind_image_memory(image, memory, 0).is_err() {
            raw.free_memory(memory, None);
            raw.destroy_image(image, None);
            return None;
        }
        hal.texture_from_raw(image, &descriptor, None, wgpu_hal::vulkan::TextureMemory::Dedicated(memory))
    };
    let descriptor = wgpu_core::resource::TextureDescriptor {
        label: Some(std::borrow::Cow::Borrowed("canvasCopy:Buffer")),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgt::TextureDimension::D2,
        format: wgt::TextureFormat::Rgba8Unorm,
        usage: wgt::TextureUsages::TEXTURE_BINDING,
        view_formats: vec![],
        texture_binding_view_dimension: None,
    };
    let (texture, error) =
        queue
            .device_id
            .create_texture_from_hal(Box::new(hal_texture), &descriptor, wgt::TextureUses::RESOURCE, true);
    error.is_none().then_some(texture)
}

/// Submits the queue family ownership transfer of `texture` from the foreign owner (GL) to wgpu's
/// queue (`acquire`), or back. Pending semaphores go with this submit.
unsafe fn transfer(queue: &CanvasGPUQueue, texture: &Arc<wgpu_core::resource::Texture>, acquire: bool) {
    use wgpu_hal::{CommandEncoder as _, QueueFamily, QueueFamilyOwnershipTransfer, StateTransition, TextureBarrier};

    let Some(family) = Arc::clone(&queue.device_id)
        .as_hal::<wgpu_hal::api::Vulkan>()
        .map(|hal| hal.queue_family_index())
    else {
        return;
    };
    let encoder = queue.device_id.create_command_encoder(&wgt::CommandEncoderDescriptor {
        label: Some(std::borrow::Cow::Borrowed("canvasCopy:Transfer")),
    });
    if let Some(raw) = Arc::clone(texture).as_hal::<wgpu_hal::api::Vulkan>() {
        encoder.as_hal_mut::<wgpu_hal::api::Vulkan, _, _>(|encoder| {
            let Some(encoder) = encoder else {
                return;
            };
            let ours = QueueFamily::Explicit(family);
            let (src, dst, from) = if acquire {
                (QueueFamily::External, ours, wgt::TextureUses::UNINITIALIZED)
            } else {
                (ours, QueueFamily::External, wgt::TextureUses::RESOURCE)
            };
            encoder.transition_textures(std::iter::once(TextureBarrier {
                texture: &*raw,
                range: wgt::ImageSubresourceRange::default(),
                usage: StateTransition {
                    from,
                    to: wgt::TextureUses::RESOURCE,
                },
                queue_family_ownership_transfer: Some(QueueFamilyOwnershipTransfer { src, dst }),
            }));
        });
    }
    let command_buffer = encoder.finish(&wgt::CommandBufferDescriptor {
        label: Some(std::borrow::Cow::Borrowed("canvasCopy:Transfer")),
    });
    queue.queue.id.submit(&[command_buffer]);
}

/// `false` leaves the copy to the readback path.
pub(crate) unsafe fn copy_context_on_gpu(
    queue: &CanvasGPUQueue,
    copy: &CanvasImageCopyCanvasRenderingContext2D,
    destination: &CanvasImageCopyTexture,
    size: CanvasExtent3d,
) -> bool {
    let destination_texture = &(*destination.texture).texture;
    if !destination_texture
        .descriptor()
        .usage
        .contains(wgt::TextureUsages::RENDER_ATTACHMENT)
    {
        return false;
    }
    let size: wgt::Extent3d = size.into();
    let context = &mut *(copy.source as *mut crate::c2d::CanvasRenderingContext2D);
    // A threaded canvas's stand-in reports CPU; its real context answers below.
    if !context.is_threaded() && context.engine() != crate::c2d::Engine::GL {
        return false;
    }
    let (width, height) = context.get_context().dimensions();
    let (width, height) = (width as u32, height as u32);
    if width == 0
        || height == 0
        || size.width == 0
        || size.height == 0
        || copy.origin.x + size.width > width
        || copy.origin.y + size.height > height
    {
        return false;
    }

    // Queued work goes first, so the semaphores land on the blit's submit.
    super::gpu_device::timeline_of(&queue.error_sink).drain();

    let mut cache = queue.android_canvases.lock();
    if cache.device.is_none() {
        cache.device = Some(device_of(queue));
    }
    if !matches!(cache.device, Some(Some(_))) {
        return false;
    }
    let producer = context.get_context().vulkan_producer();
    let Some(index) = entry(queue, &mut cache, producer, width, height) else {
        return false;
    };
    let target = {
        let entry = &mut cache.entries[index];
        entry.used = Instant::now();
        canvas_2d::context::vulkan_interop::HardwareBufferTarget {
            id: entry.id,
            buffer: entry.buffer,
            width: width as i32,
            height: height as i32,
            release_fd: std::mem::replace(&mut entry.release_fd, -1),
        }
    };
    let buffer = target.buffer;
    let Some(ready_fd) = context
        .with_pixels_source(move |context| context.draw_into_hardware_buffer(&target))
        .flatten()
    else {
        return false;
    };

    let device = cache.device.as_ref().and_then(Option::as_ref);
    let source = device.and_then(|_| import(queue, buffer, width, height));
    let hal_queue = source
        .as_ref()
        .and_then(|_| Arc::clone(&queue.queue.id).as_hal::<wgpu_hal::api::Vulkan>());
    let (Some(device), Some(source), Some(hal_queue)) = (device, source, hal_queue) else {
        if ready_fd >= 0 {
            libc::close(ready_fd);
        }
        return false;
    };
    let mut ready = None;
    if ready_fd >= 0 {
        let semaphore = cache.retired.lock().pop().or_else(|| semaphore(device, false));
        let imported = semaphore.filter(|semaphore| {
            device
                .fd
                .import_semaphore_fd(
                    &vk::ImportSemaphoreFdInfoKHR::default()
                        .semaphore(*semaphore)
                        .flags(vk::SemaphoreImportFlags::TEMPORARY)
                        .handle_type(vk::ExternalSemaphoreHandleTypeFlags::SYNC_FD)
                        .fd(ready_fd),
                )
                .is_ok()
        });
        match imported {
            Some(semaphore) => {
                hal_queue.add_wait_semaphore(semaphore, None, vk::PipelineStageFlags::ALL_COMMANDS);
                ready = Some(semaphore);
            }
            None => {
                // Vulkan did not take the fd: wait for the drawing here.
                let mut poll = libc::pollfd {
                    fd: ready_fd,
                    events: libc::POLLIN,
                    revents: 0,
                };
                libc::poll(&mut poll, 1, -1);
                libc::close(ready_fd);
                if let Some(semaphore) = semaphore {
                    cache.retired.lock().push(semaphore);
                }
            }
        }
    }
    drop(hal_queue);
    transfer(queue, &source, true);

    // Skia's surface is premultiplied.
    let blitted = super::gpu_native_texture::blit_texture_with(
        queue,
        &source,
        width,
        height,
        copy.origin.x,
        copy.origin.y,
        copy.flip_y,
        !copy.premultiplied_alpha,
        destination_texture,
        destination.mip_level,
        destination.origin.z,
        (destination.origin.x, destination.origin.y),
        size,
    );

    let release = cache.entries[index].release;
    if let Some(hal_queue) = Arc::clone(&queue.queue.id).as_hal::<wgpu_hal::api::Vulkan>() {
        hal_queue.add_signal_semaphore(release, None);
    }
    transfer(queue, &source, false);
    if let Some(semaphore) = ready {
        let retired = Arc::clone(&cache.retired);
        queue
            .queue
            .id
            .on_submitted_work_done(Box::new(move || retired.lock().push(semaphore)));
    }
    let exported = device.fd.get_semaphore_fd(
        &vk::SemaphoreGetFdInfoKHR::default()
            .semaphore(release)
            .handle_type(vk::ExternalSemaphoreHandleTypeFlags::SYNC_FD),
    );
    match exported {
        Ok(fd) => cache.entries[index].release_fd = fd,
        // The canvas would not know when this copy is over.
        Err(_) => wait_idle(&queue.device_id),
    }
    blitted
}
