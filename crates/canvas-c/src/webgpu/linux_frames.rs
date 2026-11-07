//! Linux: a WebGPU canvas shown by GTK. `getCurrentTexture()` hands out Vulkan images whose
//! memory is exportable (`VK_KHR_external_memory_fd`); at present the host imports that memory
//! into GDK's GL context (`GL_EXT_memory_object_fd`) and shows it, so frames stay on the GPU.

use std::borrow::Cow;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use ash::vk;

use super::gpu_canvas_context::{CanvasGPUCanvasContext, SurfaceData};

/// A presented frame. `fd` is the image memory's OPAQUE_FD the first time an image is shown (the
/// receiver owns it), -1 after: images are identified by `image`.
pub struct LinuxGpuFrame {
    pub image: u64,
    pub fd: i32,
    pub allocation_size: u64,
    pub width: u32,
    pub height: u32,
    /// The bytes are B, G, R, A.
    pub bgra: bool,
    /// Set until the receiver stops showing the frame.
    pub busy: Arc<AtomicBool>,
    /// Images gone since the last frame.
    pub forget: Vec<u64>,
}

pub type LinuxFrameSink = Arc<dyn Fn(LinuxGpuFrame) + Send + Sync>;

struct Image {
    id: u64,
    texture: Arc<wgpu_core::resource::Texture>,
    width: u32,
    height: u32,
    format: wgt::TextureFormat,
    usage: wgt::TextureUsages,
    allocation_size: u64,
    /// Taken by the first present.
    fd: Option<i32>,
    busy: Arc<AtomicBool>,
}

pub struct LinuxPresenter {
    sink: LinuxFrameSink,
    images: parking_lot::Mutex<Vec<Image>>,
    current: parking_lot::Mutex<Option<u64>>,
    forget: parking_lot::Mutex<Vec<u64>>,
}

const MAX_IMAGES: usize = 3;
static NEXT_IMAGE: AtomicU64 = AtomicU64::new(1);

impl LinuxPresenter {
    pub fn new(sink: LinuxFrameSink) -> Arc<Self> {
        Arc::new(Self {
            sink,
            images: Default::default(),
            current: Default::default(),
            forget: Default::default(),
        })
    }

    /// A free image for the configured texture, made when needed.
    pub(crate) fn acquire(&self, data: &SurfaceData) -> Option<Arc<wgpu_core::resource::Texture>> {
        let t = data.texture_data;
        let usage = t.usage | wgt::TextureUsages::COPY_SRC;
        let fits = |image: &Image| {
            (image.width, image.height, image.format, image.usage)
                == (t.size.width, t.size.height, t.format, usage)
        };
        let mut images = self.images.lock();
        {
            let mut forget = self.forget.lock();
            images.retain(|image| {
                let keep = fits(image) || image.busy.load(Ordering::Acquire);
                if !keep {
                    forget.push(image.id);
                }
                keep
            });
        }
        let image = match images
            .iter()
            .position(|image| fits(image) && !image.busy.load(Ordering::Acquire))
        {
            Some(index) => &images[index],
            None if images.iter().filter(|image| fits(image)).count() < MAX_IMAGES => {
                let image = unsafe { create_image(data, usage) }?;
                images.push(image);
                images.last()?
            }
            // The receiver holds them all: draw into the oldest anyway rather than stall.
            None => images.iter().find(|image| fits(image))?,
        };
        *self.current.lock() = Some(image.id);
        Some(Arc::clone(&image.texture))
    }

    /// After the page's last submit for the frame: waits for the GPU, then hands the image over.
    pub(crate) fn present(&self, data: &SurfaceData) {
        let Some(current) = self.current.lock().take() else {
            return;
        };
        // GTK samples it in another context (and API): it must be complete.
        let _ = data.device.device.poll(wgt::PollType::Wait {
            submission_index: None,
            timeout: None,
        });
        let frame = {
            let mut images = self.images.lock();
            let Some(image) = images.iter_mut().find(|image| image.id == current) else {
                return;
            };
            image.busy.store(true, Ordering::Release);
            LinuxGpuFrame {
                image: image.id,
                fd: image.fd.take().unwrap_or(-1),
                allocation_size: image.allocation_size,
                width: image.width,
                height: image.height,
                bgra: matches!(
                    image.format,
                    wgt::TextureFormat::Bgra8Unorm | wgt::TextureFormat::Bgra8UnormSrgb
                ),
                busy: Arc::clone(&image.busy),
                forget: std::mem::take(&mut *self.forget.lock()),
            }
        };
        (self.sink)(frame);
    }
}

fn vk_format(format: wgt::TextureFormat) -> Option<vk::Format> {
    Some(match format {
        wgt::TextureFormat::Bgra8Unorm => vk::Format::B8G8R8A8_UNORM,
        wgt::TextureFormat::Bgra8UnormSrgb => vk::Format::B8G8R8A8_SRGB,
        wgt::TextureFormat::Rgba8Unorm => vk::Format::R8G8B8A8_UNORM,
        wgt::TextureFormat::Rgba8UnormSrgb => vk::Format::R8G8B8A8_SRGB,
        _ => return None,
    })
}

fn usages(usage: wgt::TextureUsages) -> (vk::ImageUsageFlags, wgt::TextureUses) {
    let mut vk_usage = vk::ImageUsageFlags::empty();
    let mut uses = wgt::TextureUses::empty();
    if usage.contains(wgt::TextureUsages::COPY_SRC) {
        vk_usage |= vk::ImageUsageFlags::TRANSFER_SRC;
        uses |= wgt::TextureUses::COPY_SRC;
    }
    if usage.contains(wgt::TextureUsages::COPY_DST) {
        vk_usage |= vk::ImageUsageFlags::TRANSFER_DST;
        uses |= wgt::TextureUses::COPY_DST;
    }
    if usage.contains(wgt::TextureUsages::TEXTURE_BINDING) {
        vk_usage |= vk::ImageUsageFlags::SAMPLED;
        uses |= wgt::TextureUses::RESOURCE;
    }
    if usage.contains(wgt::TextureUsages::STORAGE_BINDING) {
        vk_usage |= vk::ImageUsageFlags::STORAGE;
        uses |= wgt::TextureUses::STORAGE_READ_WRITE;
    }
    if usage.contains(wgt::TextureUsages::RENDER_ATTACHMENT) {
        vk_usage |= vk::ImageUsageFlags::COLOR_ATTACHMENT;
        uses |= wgt::TextureUses::COLOR_TARGET;
    }
    (vk_usage, uses)
}

/// A dedicated, exportable allocation behind a 2D image, wrapped as a wgpu texture.
unsafe fn create_image(data: &SurfaceData, usage: wgt::TextureUsages) -> Option<Image> {
    let t = data.texture_data;
    let (texture, fd, allocation_size) = unsafe {
        create_exportable_texture(
            &data.device.device,
            t.size,
            t.format,
            usage,
            data.previous_configuration.view_formats.clone(),
            wgt::TextureUses::UNINITIALIZED,
            "LinuxCanvasFrame",
        )
    }?;
    Some(Image {
        id: NEXT_IMAGE.fetch_add(1, Ordering::Relaxed),
        texture,
        width: t.size.width,
        height: t.size.height,
        format: t.format,
        usage,
        allocation_size,
        fd: Some(fd),
        busy: Arc::new(AtomicBool::new(false)),
    })
}

/// A 2D image on a dedicated allocation exported as an OPAQUE_FD (the caller owns the fd), as a
/// wgpu texture in the `initial` state. Wrapped images have no clear mode: they count as
/// initialized.
pub(crate) unsafe fn create_exportable_texture(
    device: &Arc<wgpu_core::device::Device>,
    size: wgt::Extent3d,
    format: wgt::TextureFormat,
    usage: wgt::TextureUsages,
    view_formats: Vec<wgt::TextureFormat>,
    initial: wgt::TextureUses,
    label: &'static str,
) -> Option<(Arc<wgpu_core::resource::Texture>, i32, u64)> {
    let vk_format = vk_format(format)?;
    let hal = unsafe { Arc::clone(device).as_hal::<wgpu_hal::api::Vulkan>() }?;
    if !hal
        .enabled_device_extensions()
        .contains(&ash::khr::external_memory_fd::NAME)
    {
        log::error!("canvas: sharing WebGPU images needs VK_KHR_external_memory_fd");
        return None;
    }
    let raw = hal.raw_device().clone();
    let instance = hal.shared_instance().raw_instance();
    let (vk_usage, uses) = usages(usage);
    let mut external = vk::ExternalMemoryImageCreateInfo::default()
        .handle_types(vk::ExternalMemoryHandleTypeFlags::OPAQUE_FD);
    let info = vk::ImageCreateInfo::default()
        .push_next(&mut external)
        .flags(if view_formats.is_empty() {
            vk::ImageCreateFlags::empty()
        } else {
            vk::ImageCreateFlags::MUTABLE_FORMAT
        })
        .image_type(vk::ImageType::TYPE_2D)
        .format(vk_format)
        .extent(vk::Extent3D {
            width: size.width,
            height: size.height,
            depth: 1,
        })
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk_usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);
    let image = unsafe { raw.create_image(&info, None) }.ok()?;
    let requirements = unsafe { raw.get_image_memory_requirements(image) };
    let properties =
        unsafe { instance.get_physical_device_memory_properties(hal.raw_physical_device()) };
    let Some(memory_type) = properties
        .memory_types_as_slice()
        .iter()
        .enumerate()
        .position(|(i, ty)| {
            requirements.memory_type_bits & (1 << i) != 0
                && ty
                    .property_flags
                    .contains(vk::MemoryPropertyFlags::DEVICE_LOCAL)
        })
    else {
        unsafe { raw.destroy_image(image, None) };
        return None;
    };
    let mut export = vk::ExportMemoryAllocateInfo::default()
        .handle_types(vk::ExternalMemoryHandleTypeFlags::OPAQUE_FD);
    let mut dedicated = vk::MemoryDedicatedAllocateInfo::default().image(image);
    let allocate = vk::MemoryAllocateInfo::default()
        .allocation_size(requirements.size)
        .memory_type_index(memory_type as u32)
        .push_next(&mut export)
        .push_next(&mut dedicated);
    let memory = match unsafe { raw.allocate_memory(&allocate, None) } {
        Ok(memory) => memory,
        Err(_) => {
            unsafe { raw.destroy_image(image, None) };
            return None;
        }
    };
    let release = {
        let raw = raw.clone();
        move || unsafe {
            raw.destroy_image(image, None);
            raw.free_memory(memory, None);
        }
    };
    if unsafe { raw.bind_image_memory(image, memory, 0) }.is_err() {
        release();
        return None;
    }
    let fd_fn = ash::khr::external_memory_fd::Device::new(instance, &raw);
    let fd = match unsafe {
        fd_fn.get_memory_fd(
            &vk::MemoryGetFdInfoKHR::default()
                .memory(memory)
                .handle_type(vk::ExternalMemoryHandleTypeFlags::OPAQUE_FD),
        )
    } {
        Ok(fd) => fd,
        Err(_) => {
            release();
            return None;
        }
    };
    let hal_desc = wgpu_hal::TextureDescriptor {
        label: Some(label),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgt::TextureDimension::D2,
        format,
        usage: uses,
        memory_flags: wgpu_hal::MemoryFlags::empty(),
        view_formats: view_formats.clone(),
    };
    let hal_texture = unsafe {
        hal.texture_from_raw(
            image,
            &hal_desc,
            Some(Box::new(release)),
            wgpu_hal::vulkan::TextureMemory::External,
        )
    };
    drop(hal);
    let desc = wgpu_core::resource::TextureDescriptor {
        label: Some(Cow::Borrowed(label)),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgt::TextureDimension::D2,
        format,
        usage,
        view_formats,
    };
    let (texture, error) =
        unsafe { device.create_texture_from_hal(Box::new(hal_texture), &desc, initial, true) };
    if let Some(error) = error {
        log::error!("canvas: could not wrap an exportable image: {error:?}");
        unsafe { libc::close(fd) };
        return None;
    }
    Some((texture, fd, requirements.size))
}

/// UI thread: shows the context's frames through `sink` (Linux hosts).
pub fn canvas_native_webgpu_context_set_linux_sink(
    context: *const CanvasGPUCanvasContext,
    sink: Option<LinuxFrameSink>,
) {
    if let Some(context) = unsafe { context.as_ref() } {
        *context.linux.lock() = sink.map(LinuxPresenter::new);
    }
}
