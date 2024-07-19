use std::os::raw::c_void;
use std::sync::Arc;

use parking_lot::lock_api::Mutex;
use raw_window_handle::{
    AppKitDisplayHandle, RawDisplayHandle, RawWindowHandle,
};

use crate::webgpu::error::handle_error_fatal;
use crate::webgpu::gpu_adapter::CanvasGPUAdapter;
use crate::webgpu::gpu_device::ErrorSink;
use crate::webgpu::structs::CanvasSurfaceCapabilities;

use super::{
    enums::CanvasGPUTextureFormat, gpu::CanvasWebGPUInstance, gpu_device::CanvasGPUDevice,
    gpu_texture::CanvasGPUTexture,
};

#[derive(Copy, Clone)]
struct TextureData {
    usage: wgpu_types::TextureUsages,
    dimension: wgpu_types::TextureDimension,
    size: wgpu_types::Extent3d,
    format: wgpu_types::TextureFormat,
    mip_level_count: u32,
    sample_count: u32,
}

struct SurfaceData {
    device_id: wgpu_core::id::DeviceId,
    error_sink: ErrorSink,
    texture_data: TextureData,
}

#[derive(Copy, Clone, Debug)]
struct ViewData {
    width: u32,
    height: u32,
}


pub struct CanvasGPUCanvasContext {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) surface: wgpu_core::id::SurfaceId,
    pub(crate) has_surface_presented: Arc<std::sync::atomic::AtomicBool>,
    pub(crate) data: parking_lot::Mutex<Option<SurfaceData>>,
    pub(crate) view_data: parking_lot::Mutex<ViewData>,
}

impl Drop for CanvasGPUCanvasContext {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let global = self.instance.global();
            global.surface_drop(self.surface);
        }
    }
}

#[cfg(any(target_os = "android"))]
#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_context_create(
    instance: *mut CanvasWebGPUInstance,
    window: *mut c_void,
    width: u32,
    height: u32,
) -> *const CanvasGPUCanvasContext {
    if instance.is_null() {
        return std::ptr::null_mut();
    }

    Arc::increment_strong_count(instance);
    let instance = Arc::from_raw(instance);
    let global = instance.global();

    let display_handle = RawDisplayHandle::Android(raw_window_handle::AndroidDisplayHandle::new());

    let handle =
        raw_window_handle::AndroidNdkWindowHandle::new(std::ptr::NonNull::new_unchecked(window));
    let window_handle = RawWindowHandle::AndroidNdk(handle);

    match global.instance_create_surface(display_handle, window_handle, None) {
        Ok(surface_id) => {
            let ctx = CanvasGPUCanvasContext {
                instance,
                surface: surface_id,
                has_surface_presented: Arc::default(),
                data: Default::default(),
                view_data: Mutex::new(ViewData { width, height }),
            };

            Arc::into_raw(Arc::new(ctx))
        }
        Err(cause) => {
            handle_error_fatal(global, cause, "canvas_native_webgpu_context_create");
            std::ptr::null_mut()
        }
    }
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_context_create(
    instance: *const CanvasWebGPUInstance,
    view: *mut c_void,
    width: u32,
    height: u32,
) -> *const CanvasGPUCanvasContext {
    if instance.is_null() {
        return std::ptr::null();
    }
    Arc::increment_strong_count(instance);
    let instance = Arc::from_raw(instance);
    let global = instance.global();

    match global.instance_create_surface_metal(view, None) {
        Ok(surface_id) => {
            let ctx = CanvasGPUCanvasContext {
                instance,
                surface: surface_id,
                data: Mutex::default(),
                has_surface_presented: Arc::default(),
                view_data: Mutex::new(
                    ViewData { width, height }
                ),
            };

            Arc::into_raw(Arc::new(ctx))
        }
        Err(cause) => {
            handle_error_fatal(global, cause, "canvas_native_webgpu_context_create");
            std::ptr::null()
        }
    }
}

#[cfg(any(target_os = "ios"))]
#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_context_create_uiview(
    instance: *const CanvasWebGPUInstance,
    view: *mut c_void,
    width: u32,
    height: u32,
) -> *const CanvasGPUCanvasContext {
    if instance.is_null() {
        return std::ptr::null();
    }
    Arc::increment_strong_count(instance);
    let instance = Arc::from_raw(instance);
    let global = instance.global();

    let display_handle = RawDisplayHandle::UiKit(raw_window_handle::UiKitDisplayHandle::new());

    let handle = raw_window_handle::UiKitWindowHandle::new(std::ptr::NonNull::new_unchecked(view));
    let window_handle = RawWindowHandle::UiKit(handle);

    match global.instance_create_surface(display_handle, window_handle, None) {
        Ok(surface_id) => {
            let ctx = CanvasGPUCanvasContext {
                instance,
                surface: surface_id,
                has_surface_presented: Arc::default(),
                data: Mutex::default(),
                view_data: Mutex::new(ViewData { width, height }),
            };

            Arc::into_raw(Arc::new(ctx))
        }
        Err(cause) => {
            handle_error_fatal(global, cause, "canvas_native_webgpu_context_create_uiview");
            std::ptr::null()
        }
    }
}

#[cfg(any(target_os = "macos"))]
#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_context_create_nsview(
    instance: *const CanvasWebGPUInstance,
    view: *mut c_void,
    width: u32,
    height: u32,
) -> *const CanvasGPUCanvasContext {
    if instance.is_null() {
        return std::ptr::null();
    }
    Arc::increment_strong_count(instance);
    let instance = Arc::from_raw(instance);
    let global = instance.global();

    let display_handle = RawDisplayHandle::AppKit(AppKitDisplayHandle::new());

    let handle = raw_window_handle::AppKitWindowHandle::new(std::ptr::NonNull::new_unchecked(view));
    let window_handle = RawWindowHandle::AppKit(handle);

    match global.instance_create_surface(display_handle, window_handle, None) {
        Ok(surface_id) => {
            let ctx = CanvasGPUCanvasContext {
                instance,
                surface: surface_id,
                has_surface_presented: Arc::default(),
                data: Mutex::default(),
                view_data: Mutex::new(ViewData { width, height }),
            };

            Arc::into_raw(Arc::new(ctx))
        }
        Err(cause) => {
            handle_error_fatal(global, cause, "canvas_native_webgpu_context_create");
            std::ptr::null()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CanvasGPUSurfaceAlphaMode {
    Auto = 0,
    Opaque = 1,
    PreMultiplied = 2,
    PostMultiplied = 3,
    Inherit = 4,
}

impl From<wgpu_types::CompositeAlphaMode> for CanvasGPUSurfaceAlphaMode {
    fn from(value: wgpu_types::CompositeAlphaMode) -> Self {
        match value {
            wgpu_types::CompositeAlphaMode::Auto => Self::Auto,
            wgpu_types::CompositeAlphaMode::Opaque => Self::Opaque,
            wgpu_types::CompositeAlphaMode::PreMultiplied => Self::PreMultiplied,
            wgpu_types::CompositeAlphaMode::PostMultiplied => Self::PostMultiplied,
            wgpu_types::CompositeAlphaMode::Inherit => Self::Inherit,
        }
    }
}

impl Into<wgpu_types::CompositeAlphaMode> for CanvasGPUSurfaceAlphaMode {
    fn into(self) -> wgpu_types::CompositeAlphaMode {
        match self {
            CanvasGPUSurfaceAlphaMode::Auto => wgpu_types::CompositeAlphaMode::Auto,
            CanvasGPUSurfaceAlphaMode::Opaque => wgpu_types::CompositeAlphaMode::Opaque,
            CanvasGPUSurfaceAlphaMode::PreMultiplied => {
                wgpu_types::CompositeAlphaMode::PreMultiplied
            }
            CanvasGPUSurfaceAlphaMode::PostMultiplied => {
                wgpu_types::CompositeAlphaMode::PostMultiplied
            }
            CanvasGPUSurfaceAlphaMode::Inherit => wgpu_types::CompositeAlphaMode::Inherit,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CanvasGPUPresentMode {
    AutoVsync = 0,
    AutoNoVsync = 1,
    Fifo = 2,
    FifoRelaxed = 3,
    Immediate = 4,
    Mailbox = 5,
}

impl Into<wgpu_types::PresentMode> for CanvasGPUPresentMode {
    fn into(self) -> wgpu_types::PresentMode {
        match self {
            CanvasGPUPresentMode::AutoVsync => wgpu_types::PresentMode::AutoVsync,
            CanvasGPUPresentMode::AutoNoVsync => wgpu_types::PresentMode::AutoNoVsync,
            CanvasGPUPresentMode::Fifo => wgpu_types::PresentMode::Fifo,
            CanvasGPUPresentMode::FifoRelaxed => wgpu_types::PresentMode::FifoRelaxed,
            CanvasGPUPresentMode::Immediate => wgpu_types::PresentMode::Immediate,
            CanvasGPUPresentMode::Mailbox => wgpu_types::PresentMode::Mailbox,
        }
    }
}

impl From<wgpu_types::PresentMode> for CanvasGPUPresentMode {
    fn from(value: wgpu_types::PresentMode) -> Self {
        match value {
            wgpu_types::PresentMode::AutoVsync => Self::AutoVsync,
            wgpu_types::PresentMode::AutoNoVsync => Self::AutoNoVsync,
            wgpu_types::PresentMode::Fifo => Self::Fifo,
            wgpu_types::PresentMode::FifoRelaxed => Self::FifoRelaxed,
            wgpu_types::PresentMode::Immediate => Self::Immediate,
            wgpu_types::PresentMode::Mailbox => Self::Mailbox,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CanvasGPUSurfaceConfiguration {
    pub alphaMode: CanvasGPUSurfaceAlphaMode,
    pub usage: u32,
    pub presentMode: CanvasGPUPresentMode,
    pub view_formats: *const CanvasGPUTextureFormat,
    pub view_formats_size: usize,
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_context_configure(
    context: *const CanvasGPUCanvasContext,
    device: *const CanvasGPUDevice,
    config: *const CanvasGPUSurfaceConfiguration,
) {
    if context.is_null() || device.is_null() {
        return;
    }
    Arc::increment_strong_count(context);
    let mut context = Arc::from_raw(context);
    let surface_id = context.surface;
    let global = context.instance.global();
    let device = unsafe { &*device };
    let device_id = device.device;

    let config = unsafe { &*config };

    let view_formats = if !config.view_formats.is_null() && config.view_formats_size > 0 {
        unsafe {
            std::slice::from_raw_parts(config.view_formats, config.view_formats_size)
                .to_vec()
                .into_iter()
                .map(|v| v.into())
                .collect::<Vec<wgpu_types::TextureFormat>>()
        }
    } else {
        vec![]
    };

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    let format = wgpu_types::TextureFormat::Bgra8Unorm;


    #[cfg(any(target_os = "android"))]
    let format = wgpu_types::TextureFormat::Rgba8Unorm;

    let usage = wgpu_types::TextureUsages::from_bits_truncate(config.usage);

    let view_data = context.view_data.lock();

    let config = wgpu_types::SurfaceConfiguration::<Vec<wgpu_types::TextureFormat>> {
        desired_maximum_frame_latency: 2,
        usage,
        format,
        width: view_data.height,
        height: view_data.width,
        present_mode: config.presentMode.into(),
        alpha_mode: config.alphaMode.into(),
        view_formats,
    };

    if let Some(cause) =
        gfx_select!(surface_id => global.surface_configure(surface_id, device_id, &config))
    {
        handle_error_fatal(global, cause, "canvas_native_webgpu_context_configure");
        let mut lock = context.data.lock();
        *lock = None;
    } else {
        let mut lock = context.data.lock();
        *lock = Some(SurfaceData {
            device_id,
            error_sink: device.error_sink.clone(),
            texture_data: TextureData {
                usage,
                dimension: wgpu_types::TextureDimension::D2,
                size: wgpu_types::Extent3d {
                    width: view_data.width,
                    height: view_data.height,
                    depth_or_array_layers: 1,
                },
                format,
                mip_level_count: 1,
                sample_count: 1,
            },
        });
        context
            .has_surface_presented
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_context_unconfigure(
    context: *const CanvasGPUCanvasContext,
    device: *mut CanvasGPUDevice,
    config: *const CanvasGPUSurfaceConfiguration,
) {
    if context.is_null() {
        return;
    }
    let context = &*context;
    let surface_id = context.surface;
    let global = context.instance.global();
    let device = &*device;
    let device_id = device.device;
    let config = &*config;
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_context_get_current_texture(
    context: *const CanvasGPUCanvasContext,
) -> *const CanvasGPUTexture {
    if context.is_null() {
        return std::ptr::null();
    }

    let context = unsafe { &*context };
    let global = context.instance.global();

    let data_guard = context.data.lock();

    let surface_data = match data_guard.as_ref() {
        Some(surface_data) => surface_data,
        None => {
            return std::ptr::null();
        }
    };

    let surface_id = context.surface;

    let result = gfx_select!(surface_id => global.surface_get_current_texture(surface_id, None));

    match result {
        Ok(texture) => match texture.status {
            wgpu_types::SurfaceStatus::Good | wgpu_types::SurfaceStatus::Suboptimal => {
                context
                    .has_surface_presented
                    .store(false, std::sync::atomic::Ordering::SeqCst);

                Arc::into_raw(Arc::new(CanvasGPUTexture {
                    instance: context.instance.clone(),
                    texture: texture.texture_id.unwrap(),
                    surface_id: Some(surface_id),
                    owned: false,
                    depth_or_array_layers: 1,
                    dimension: super::enums::CanvasTextureDimension::D2,
                    format: surface_data.texture_data.format.into(),
                    mipLevelCount: 1,
                    sampleCount: 1,
                    width: surface_data.texture_data.size.width,
                    height: surface_data.texture_data.size.height,
                    usage: surface_data.texture_data.usage.bits(),
                    error_sink: surface_data.error_sink.clone(),
                    has_surface_presented: context.has_surface_presented.clone(),
                }))
            }
            _ => std::ptr::null_mut(),
        },
        Err(cause) => {
            handle_error_fatal(
                global,
                cause,
                "canvas_native_webgpu_context_get_current_texture",
            );
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_context_present_surface(
    context: *const CanvasGPUCanvasContext,
    texture: *const CanvasGPUTexture,
) {
    if context.is_null() || texture.is_null() {
        return;
    }

    let context = unsafe { &*context };
    let global = context.instance.global();

    let surface_id = context.surface;
    let texture = unsafe { &*texture };

    if let Err(cause) = gfx_select!(device => global.surface_present(surface_id)) {
        handle_error_fatal(
            global,
            cause,
            "canvas_native_webgpu_context_present_surface",
        )
    } else {
        context
            .has_surface_presented
            .store(true, std::sync::atomic::Ordering::SeqCst);

        unsafe { Arc::decrement_strong_count(texture) };
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_context_get_capabilities(
    context: *const CanvasGPUCanvasContext,
    adapter: *const CanvasGPUAdapter,
) -> *mut CanvasSurfaceCapabilities {
    if context.is_null() || adapter.is_null() {
        return std::ptr::null_mut();
    }

    let adapter = unsafe { &*adapter };
    let adapter_id = adapter.adapter;
    let context = unsafe { &*context };
    let global = context.instance.global();

    let surface_id = context.surface;

    if let Ok(capabilities) =
        gfx_select!(surface_id => global.surface_get_capabilities(surface_id, adapter_id))
    {
        let cap: CanvasSurfaceCapabilities = capabilities.into();
        return Box::into_raw(Box::new(cap));
    }

    std::ptr::null_mut()
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_context_reference(
    context: *const CanvasGPUCanvasContext
) {
    if context.is_null() {
        return;
    }

    Arc::increment_strong_count(context);
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_context_release(
    context: *const CanvasGPUCanvasContext
) {
    if context.is_null() {
        return;
    }

    Arc::decrement_strong_count(context);
}
