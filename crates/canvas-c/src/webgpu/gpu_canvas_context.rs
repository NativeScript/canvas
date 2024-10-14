use std::os::raw::c_void;
use std::sync::Arc;

use parking_lot::lock_api::Mutex;
use raw_window_handle::{AppKitDisplayHandle, RawDisplayHandle, RawWindowHandle};
use wgt::SurfaceStatus;

use crate::webgpu::enums::{CanvasOptionalGPUTextureFormat, SurfaceGetCurrentTextureStatus};
use crate::webgpu::error::handle_error_fatal;
use crate::webgpu::gpu_adapter::CanvasGPUAdapter;
use crate::webgpu::gpu_device::ErrorSink;
use crate::webgpu::structs::{CanvasExtent3d, CanvasSurfaceCapabilities};

use super::{
    enums::CanvasGPUTextureFormat, gpu::CanvasWebGPUInstance, gpu_device::CanvasGPUDevice,
    gpu_texture::CanvasGPUTexture,
};

//use wgpu_core::gfx_select;

#[derive(Copy, Clone)]
struct TextureData {
    usage: wgt::TextureUsages,
    dimension: wgt::TextureDimension,
    size: wgt::Extent3d,
    format: wgt::TextureFormat,
    mip_level_count: u32,
    sample_count: u32,
}

pub struct SurfaceData {
    device_id: wgpu_core::id::DeviceId,
    error_sink: ErrorSink,
    texture_data: TextureData,
    previous_configuration: wgt::SurfaceConfiguration<Vec<wgt::TextureFormat>>,
}

#[derive(Copy, Clone, Debug)]
pub struct ViewData {
    pub width: u32,
    pub height: u32,
}

pub struct CanvasGPUCanvasContext {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) surface: parking_lot::Mutex<wgpu_core::id::SurfaceId>,
    pub(crate) has_surface_presented: Arc<std::sync::atomic::AtomicBool>,
    pub(crate) data: parking_lot::Mutex<Option<SurfaceData>>,
    pub(crate) view_data: parking_lot::Mutex<ViewData>,
}

impl Drop for CanvasGPUCanvasContext {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let global = self.instance.global();
            let surface = self.surface.lock();
            global.surface_drop(*surface);
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
                surface: Mutex::new(surface_id),
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


#[cfg(any(target_os = "android"))]
#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_context_resize(
    context: *mut CanvasGPUCanvasContext,
    window: *mut c_void,
    width: u32,
    height: u32,
) {
    if context.is_null() {
        return;
    }

    let context = &*context;

    let mut surface_data_lock = context.data.lock();

    let global = context.instance.global();

    let display_handle = RawDisplayHandle::Android(raw_window_handle::AndroidDisplayHandle::new());

    let handle =
        raw_window_handle::AndroidNdkWindowHandle::new(std::ptr::NonNull::new_unchecked(window));
    let window_handle = RawWindowHandle::AndroidNdk(handle);

    let mut surface = context.surface.lock();

    global.surface_drop(*surface);

    match global.instance_create_surface(display_handle, window_handle, None) {
        Ok(surface_id) => {
            *surface = surface_id;
            context.has_surface_presented.store(false, std::sync::atomic::Ordering::SeqCst);
            let mut view_data = context.view_data.lock();
            view_data.width = width;
            view_data.height = height;

            if let Some(surface_data) = surface_data_lock.as_mut() {
                surface_data.texture_data.size.width = width;
                surface_data.texture_data.size.height = height;


                let mut new_config = surface_data.previous_configuration.clone();
                new_config.width = width;
                new_config.height = height;

                if let Some(cause) = global.surface_configure(surface_id, surface_data.device_id, &new_config)
                {
                    handle_error_fatal(global, cause, "canvas_native_webgpu_context_resize");
                } else {
                    surface_data.previous_configuration = new_config;
                }
            }
        }
        Err(cause) => {
            handle_error_fatal(global, cause, "canvas_native_webgpu_context_resize");
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
                surface: Mutex::new(surface_id),
                data: Mutex::default(),
                has_surface_presented: Arc::default(),
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
                surface: Mutex::new(surface_id),
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

#[cfg(any(target_os = "ios"))]
#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_context_resize_uiview(
    context: *const crate::webgpu::gpu_canvas_context::CanvasGPUCanvasContext,
    view: *mut c_void,
    width: u32,
    height: u32,
) {
    if context.is_null() {
        return;
    }
    let context = &*context;

    let mut surface_data_lock = context.data.lock();

    let global = context.instance.global();

    let display_handle = RawDisplayHandle::UiKit(raw_window_handle::UiKitDisplayHandle::new());

    let handle = raw_window_handle::UiKitWindowHandle::new(std::ptr::NonNull::new_unchecked(view));

    let window_handle = RawWindowHandle::UiKit(handle);

    let mut surface = context.surface.lock();

    global.surface_drop(*surface);

    match global.instance_create_surface(display_handle, window_handle, None) {
        Ok(surface_id) => {
            *surface = surface_id;
            context.has_surface_presented.store(false, std::sync::atomic::Ordering::SeqCst);
            let mut view_data = context.view_data.lock();
            view_data.width = width;
            view_data.height = height;

            if let Some(surface_data) = surface_data_lock.as_mut() {
                surface_data.texture_data.size.width = width;
                surface_data.texture_data.size.height = height;

                let mut new_config = surface_data.previous_configuration.clone();
                new_config.width = width;
                new_config.height = height;

                if let Some(cause) = global.surface_configure(surface_id, surface_data.device_id, &new_config)
                {
                    handle_error_fatal(global, cause, "canvas_native_webgpu_context_resize_uiview");
                } else {
                    surface_data.previous_configuration = new_config;
                }
            }
        }
        Err(cause) => {
            handle_error_fatal(global, cause, "canvas_native_webgpu_context_resize_uiview");
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
    use raw_window_handle::AppKitDisplayHandle;
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
                surface: Mutex::new(surface_id),
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



#[cfg(any(target_os = "macos"))]
#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_context_resize_nsview(
    context: *const CanvasGPUCanvasContext,
    view: *mut c_void,
    width: u32,
    height: u32,
) {
    if context.is_null() {
        return;
    }
    let context = &*context;

    let mut surface_data_lock = context.data.lock();

    let global = context.instance.global();

    let display_handle = RawDisplayHandle::AppKit(AppKitDisplayHandle::new());

    let handle = raw_window_handle::AppKitWindowHandle::new(std::ptr::NonNull::new_unchecked(view));
    let window_handle = RawWindowHandle::AppKit(handle);

    let mut surface = context.surface.lock();

    global.surface_drop(*surface);

    match global.instance_create_surface(display_handle, window_handle, None) {
        Ok(surface_id) => {
            *surface = surface_id;
            context.has_surface_presented.store(false, std::sync::atomic::Ordering::SeqCst);
            let mut view_data = context.view_data.lock();
            view_data.width = width;
            view_data.height = height;

            if let Some(surface_data) = surface_data_lock.as_mut() {
                surface_data.texture_data.size.width = width;
                surface_data.texture_data.size.height = height;


                let mut new_config = surface_data.previous_configuration.clone();
                new_config.width = width;
                new_config.height = height;

                if let Some(cause) = global.surface_configure(surface_id, surface_data.device_id, &new_config)
                {
                    handle_error_fatal(global, cause, "canvas_native_webgpu_context_resize_nsview");
                } else {
                    surface_data.previous_configuration = new_config;
                }
            }
        }
        Err(cause) => {
            handle_error_fatal(global, cause, "canvas_native_webgpu_context_resize_nsview");
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

impl From<wgt::CompositeAlphaMode> for CanvasGPUSurfaceAlphaMode {
    fn from(value: wgt::CompositeAlphaMode) -> Self {
        match value {
            wgt::CompositeAlphaMode::Auto => Self::Auto,
            wgt::CompositeAlphaMode::Opaque => Self::Opaque,
            wgt::CompositeAlphaMode::PreMultiplied => Self::PreMultiplied,
            wgt::CompositeAlphaMode::PostMultiplied => Self::PostMultiplied,
            wgt::CompositeAlphaMode::Inherit => Self::Inherit,
        }
    }
}

impl Into<wgt::CompositeAlphaMode> for CanvasGPUSurfaceAlphaMode {
    fn into(self) -> wgt::CompositeAlphaMode {
        match self {
            CanvasGPUSurfaceAlphaMode::Auto => wgt::CompositeAlphaMode::Auto,
            CanvasGPUSurfaceAlphaMode::Opaque => wgt::CompositeAlphaMode::Opaque,
            CanvasGPUSurfaceAlphaMode::PreMultiplied => {
                wgt::CompositeAlphaMode::PreMultiplied
            }
            CanvasGPUSurfaceAlphaMode::PostMultiplied => {
                wgt::CompositeAlphaMode::PostMultiplied
            }
            CanvasGPUSurfaceAlphaMode::Inherit => wgt::CompositeAlphaMode::Inherit,
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

impl Into<wgt::PresentMode> for CanvasGPUPresentMode {
    fn into(self) -> wgt::PresentMode {
        match self {
            CanvasGPUPresentMode::AutoVsync => wgt::PresentMode::AutoVsync,
            CanvasGPUPresentMode::AutoNoVsync => wgt::PresentMode::AutoNoVsync,
            CanvasGPUPresentMode::Fifo => wgt::PresentMode::Fifo,
            CanvasGPUPresentMode::FifoRelaxed => wgt::PresentMode::FifoRelaxed,
            CanvasGPUPresentMode::Immediate => wgt::PresentMode::Immediate,
            CanvasGPUPresentMode::Mailbox => wgt::PresentMode::Mailbox,
        }
    }
}

impl From<wgt::PresentMode> for CanvasGPUPresentMode {
    fn from(value: wgt::PresentMode) -> Self {
        match value {
            wgt::PresentMode::AutoVsync => Self::AutoVsync,
            wgt::PresentMode::AutoNoVsync => Self::AutoNoVsync,
            wgt::PresentMode::Fifo => Self::Fifo,
            wgt::PresentMode::FifoRelaxed => Self::FifoRelaxed,
            wgt::PresentMode::Immediate => Self::Immediate,
            wgt::PresentMode::Mailbox => Self::Mailbox,
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
    pub size: *const CanvasExtent3d,
    pub format: CanvasOptionalGPUTextureFormat,
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
    let context = Arc::from_raw(context);
    let surface_id = context.surface.lock();
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
                .collect::<Vec<wgt::TextureFormat>>()
        }
    } else {
        vec![]
    };

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    let mut format = wgt::TextureFormat::Bgra8Unorm;

    #[cfg(any(target_os = "android"))]
    let mut format = wgt::TextureFormat::Rgba8Unorm;

    match config.format {
        CanvasOptionalGPUTextureFormat::None => {}
        CanvasOptionalGPUTextureFormat::Some(value) => {
            format = value.into();
        }
    }

    let usage = wgt::TextureUsages::from_bits_truncate(config.usage);

    let view_data = if !config.size.is_null() {
        let size = &*config.size;
        (size.width, size.height)
    } else {
        let view_data = context.view_data.lock();
        (view_data.width, view_data.height)
    };

    let config = wgt::SurfaceConfiguration::<Vec<wgt::TextureFormat>> {
        desired_maximum_frame_latency: 2,
        usage,
        format,
        width: view_data.0,
        height: view_data.1,
        present_mode: config.presentMode.into(),
        alpha_mode: config.alphaMode.into(),
        view_formats,
    };

    if let Some(cause) = global.surface_configure(*surface_id, device_id, &config)
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
                dimension: wgt::TextureDimension::D2,
                size: wgt::Extent3d {
                    width: view_data.0,
                    height: view_data.1,
                    depth_or_array_layers: 1,
                },
                format,
                mip_level_count: 1,
                sample_count: 1,
            },
            previous_configuration: config,
        });
        context
            .has_surface_presented
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

#[no_mangle]
#[allow(unused)]
pub unsafe extern "C" fn canvas_native_webgpu_context_unconfigure(
    context: *const CanvasGPUCanvasContext
) {
    if context.is_null() {
        return;
    }
    let context = &*context;

    let mut lock = context.data.lock();
    *lock = None;
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

    let surface_id = context.surface.lock();

    let result = global.surface_get_current_texture(*surface_id, None);

    match result {
        Ok(texture) => {
            let mut suboptimal = false;
            let status = match texture.status {
                SurfaceStatus::Good => SurfaceGetCurrentTextureStatus::Success,
                SurfaceStatus::Suboptimal => {
                    suboptimal = true;
                    SurfaceGetCurrentTextureStatus::Success
                }
                SurfaceStatus::Timeout => SurfaceGetCurrentTextureStatus::Timeout,
                SurfaceStatus::Outdated => SurfaceGetCurrentTextureStatus::Outdated,
                SurfaceStatus::Lost => SurfaceGetCurrentTextureStatus::Lost,
            };

            context
                .has_surface_presented
                .store(false, std::sync::atomic::Ordering::SeqCst);

            let data_guard = context.data.lock();

            let surface_data = match data_guard.as_ref() {
                Some(surface_data) => surface_data,
                None => {
                    return std::ptr::null();
                }
            };

            Arc::into_raw(Arc::new(CanvasGPUTexture {
                label: None,
                instance: context.instance.clone(),
                texture: texture.texture_id.unwrap(),
                surface_id: Some(*surface_id),
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
                suboptimal,
                status,
                has_surface_presented: context.has_surface_presented.clone(),
            }))
        }
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

    let surface_id = context.surface.lock();
    let texture = unsafe { &*texture };

    if let Err(cause) = global.surface_present(*surface_id) {
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

    let surface_id = context.surface.lock();

    match global.surface_get_capabilities(*surface_id, adapter_id) {
        Ok(capabilities) => {
            let cap: CanvasSurfaceCapabilities = capabilities.into();
            Box::into_raw(Box::new(cap))
        }
        Err(cause) => {
            handle_error_fatal(global, cause, "canvas_native_webgpu_context_get_capabilities");
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_context_reference(
    context: *const CanvasGPUCanvasContext,
) {
    if context.is_null() {
        return;
    }

    Arc::increment_strong_count(context);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_context_release(
    context: *const CanvasGPUCanvasContext,
) {
    if context.is_null() {
        return;
    }

    Arc::decrement_strong_count(context);
}
