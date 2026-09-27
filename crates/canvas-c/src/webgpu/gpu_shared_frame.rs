use std::borrow::Cow;
use std::collections::HashMap;
use std::os::raw::c_void;
use std::sync::Arc;

use windows::Win32::Foundation::HANDLE;
use windows::Win32::Graphics::Direct3D12::{ID3D12Device, ID3D12Fence, ID3D12Resource};

use super::gpu_device::CanvasGPUDevice;
use super::gpu_queue::CanvasGPUQueue;

/// A decoded video frame shared from another Direct3D device (canvas-media's frame server on
/// Windows). `nativeTexture` / `texturePointer` in `copyExternalImageToTexture` and
/// `importExternalTexture` is the address of one, valid for the duration of the call.
///
/// The producer signals `ready_fence` to `ready_value` once the frame is written. The consumer
/// waits for it on the GPU before reading, and signals `release_fence` to `release_value` after,
/// which is how the producer knows the texture may be written again.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CanvasD3DSharedFrame {
    /// `size_of::<CanvasD3DSharedFrame>()`, checked before anything else is read.
    pub size: u32,
    /// Set to 1 by the consumer once it has staged the release signal. A frame released
    /// unconsumed (the import failed) frees its texture at once instead of waiting on the fence.
    pub consumed: u32,
    /// The producer's adapter (`LowPart | HighPart << 32`); shared handles only open on the same one.
    pub adapter_luid: u64,
    /// Identify objects for caching: unique for the process's lifetime, unlike handle values.
    pub texture_id: u64,
    /// NT handle of a shareable `DXGI_FORMAT_B8G8R8A8_UNORM` 2D texture.
    pub texture: *mut c_void,
    pub ready_fence_id: u64,
    pub ready_fence: *mut c_void,
    pub ready_value: u64,
    pub release_fence_id: u64,
    pub release_fence: *mut c_void,
    pub release_value: u64,
}

/// Opened textures and fences per queue. Entries hold references on the shared objects, so the
/// cache is bounded; a producer recreates its textures when the video size changes.
#[derive(Default)]
pub struct SharedFrameCache {
    textures: HashMap<u64, Arc<wgpu_core::resource::Texture>>,
    fences: HashMap<u64, ID3D12Fence>,
}

// The fences are only used from the queue's thread; wgpu-core's objects are Send + Sync.
unsafe impl Send for SharedFrameCache {}

const MAX_CACHED: usize = 16;

/// A copy of the descriptor at `handle`, if it is one (the producer's memory stays its own).
unsafe fn frame(handle: *mut c_void) -> Option<CanvasD3DSharedFrame> {
    let frame = (handle as *const CanvasD3DSharedFrame).as_ref()?;
    (frame.size as usize == std::mem::size_of::<CanvasD3DSharedFrame>()).then_some(*frame)
}

unsafe fn raw_device(device: &Arc<wgpu_core::device::Device>) -> Option<ID3D12Device> {
    let hal = Arc::clone(device).as_hal::<wgpu_hal::api::Dx12>()?;
    Some(hal.raw_device().clone())
}

unsafe fn open_fence(raw: &ID3D12Device, cache: &mut SharedFrameCache, id: u64, handle: *mut c_void) -> Option<ID3D12Fence> {
    if let Some(fence) = cache.fences.get(&id) {
        return Some(fence.clone());
    }
    let mut fence: Option<ID3D12Fence> = None;
    raw.OpenSharedHandle(HANDLE(handle), &mut fence).ok()?;
    let fence = fence?;
    if cache.fences.len() >= MAX_CACHED {
        cache.fences.clear();
    }
    cache.fences.insert(id, fence.clone());
    Some(fence)
}

/// Opens the frame's texture on the queue's device and stages the fence wait / signal around the
/// queue's next submit. The texture must only be sampled (it stays in the COMMON state, which
/// promotes implicitly to shader reads), and the next submit must be the one that reads it.
pub(crate) unsafe fn import_and_stage(
    queue: &CanvasGPUQueue,
    handle: *mut c_void,
    width: u32,
    height: u32,
) -> Option<Arc<wgpu_core::resource::Texture>> {
    let frame = frame(handle)?;
    if frame.texture.is_null() || frame.ready_fence.is_null() || frame.release_fence.is_null() {
        return None;
    }
    let device = &queue.device_id;
    let raw = raw_device(device)?;
    let luid = raw.GetAdapterLuid();
    if (luid.LowPart as u64 | ((luid.HighPart as u32 as u64) << 32)) != frame.adapter_luid {
        return None;
    }

    let mut cache = queue.shared_frames.lock();
    let ready = open_fence(&raw, &mut cache, frame.ready_fence_id, frame.ready_fence)?;
    let release = open_fence(&raw, &mut cache, frame.release_fence_id, frame.release_fence)?;

    let texture = match cache.textures.get(&frame.texture_id) {
        Some(texture) => Arc::clone(texture),
        None => {
            let mut resource: Option<ID3D12Resource> = None;
            raw.OpenSharedHandle(HANDLE(frame.texture), &mut resource).ok()?;
            let size = wgt::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            };
            let hal_texture = wgpu_hal::dx12::Device::texture_from_raw(
                resource?,
                wgt::TextureFormat::Bgra8Unorm,
                wgt::TextureDimension::D2,
                size,
                1,
                1,
            );
            let descriptor = wgpu_core::resource::TextureDescriptor {
                label: Some(Cow::Borrowed("videoBlit:SharedFrame")),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgt::TextureDimension::D2,
                format: wgt::TextureFormat::Bgra8Unorm,
                usage: wgt::TextureUsages::TEXTURE_BINDING,
                view_formats: vec![],
            };
            // Shader reads only, from the COMMON state the resource is in between submits.
            let (texture, error) =
                device.create_texture_from_hal(Box::new(hal_texture), &descriptor, wgt::TextureUses::RESOURCE, true);
            if let Some(error) = error {
                log::error!("importing a shared video frame failed: {error:?}");
                return None;
            }
            if cache.textures.len() >= MAX_CACHED {
                cache.textures.clear();
            }
            cache.textures.insert(frame.texture_id, Arc::clone(&texture));
            texture
        }
    };
    drop(cache);

    let hal_queue = Arc::clone(&queue.queue.id).as_hal::<wgpu_hal::api::Dx12>()?;
    hal_queue.add_wait_fence(ready, frame.ready_value);
    hal_queue.add_signal_fence(release, frame.release_value);
    (*(handle as *mut CanvasD3DSharedFrame)).consumed = 1;
    Some(texture)
}

/// `importExternalTexture` on a shared frame: the frame is drawn into a texture of our own at once,
/// so the external texture's later uses never touch the producer's texture.
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
    let plane = device.device.create_texture(&wgpu_core::resource::TextureDescriptor {
        label: Some(Cow::Borrowed("externalTexture:Plane")),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgt::TextureDimension::D2,
        format: wgt::TextureFormat::Bgra8Unorm,
        usage: wgt::TextureUsages::TEXTURE_BINDING | wgt::TextureUsages::RENDER_ATTACHMENT,
        view_formats: vec![],
    });
    let source = import_and_stage(queue, handle, width, height)?;
    super::gpu_native_texture::blit_texture(queue, &source, width, height, 0, 0, false, &plane, 0, 0, (0, 0), size)
        .then_some(plane)
}

/// The LUID of the adapter the device runs on (`LowPart | HighPart << 32`), 0 if unknown: a video
/// shares its frames with a device only on the same adapter.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_get_adapter_luid(device: *const CanvasGPUDevice) -> u64 {
    let Some(device) = device.as_ref() else { return 0 };
    let Some(raw) = raw_device(&device.device) else { return 0 };
    let luid = raw.GetAdapterLuid();
    luid.LowPart as u64 | ((luid.HighPart as u32 as u64) << 32)
}
