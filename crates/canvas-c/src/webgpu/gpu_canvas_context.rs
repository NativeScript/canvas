use std::os::raw::c_void;

use super::{
    enums::CanvasGPUTextureFormat, gpu::CanvasWebGPUInstance, gpu_device::CanvasGPUDevice,
    gpu_texture::CanvasGPUTexture,
};
use raw_window_handle::{
    AppKitDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};
pub struct CanvasGPUCanvasContext {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) surface: wgpu_core::id::SurfaceId,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) format: wgpu_types::TextureFormat,
    pub(crate) usage: u32,
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_context_create(
    instance: *mut CanvasWebGPUInstance,
    view: *mut c_void,
    width: u32,
    height: u32,
) -> *mut CanvasGPUCanvasContext {
    if instance.is_null() {
        return std::ptr::null_mut();
    }
    let instance = unsafe { &*instance };
    let global = &instance.0;

    match global.instance_create_surface_metal(view, None) {
        Ok(surface_id) => {
            let ctx = CanvasGPUCanvasContext {
                instance: instance.clone(),
                surface: surface_id,
                width,
                height,
                format: wgpu_types::TextureFormat::Bgra8Unorm,
                usage: wgpu_types::TextureUsages::RENDER_ATTACHMENT.bits(),
            };

            Box::into_raw(Box::new(ctx))
        }
        Err(_) => {
            // todo handle error
            std::ptr::null_mut()
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
    alphaMode: CanvasGPUSurfaceAlphaMode,
    usage: u32,
    presentMode: CanvasGPUPresentMode,
    view_formats: *const CanvasGPUTextureFormat,
    view_formats_size: usize,
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_context_configure(
    context: *mut CanvasGPUCanvasContext,
    device: *mut CanvasGPUDevice,
    config: *const CanvasGPUSurfaceConfiguration,
) {
    if context.is_null() || device.is_null() {
        return;
    }
    let context = unsafe { &*context };
    let surface_id = context.surface;
    let global = &context.instance.0;
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

    let config = wgpu_types::SurfaceConfiguration::<Vec<wgpu_types::TextureFormat>> {
        desired_maximum_frame_latency: 2,
        usage: wgpu_types::TextureUsages::from_bits_truncate(config.usage),
        format: context.format,
        width: context.height,
        height: context.width,
        present_mode: config.presentMode.into(),
        alpha_mode: config.alphaMode.into(),
        view_formats: view_formats,
    };

    // todo handle error
    if let Some(err) =
        gfx_select!(device_id => global.surface_configure(surface_id, device_id, &config))
    {}
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_context_unconfigure(
    context: *mut CanvasGPUCanvasContext,
    device: *mut CanvasGPUDevice,
    config: *const CanvasGPUSurfaceConfiguration,
) {
    if context.is_null() {
        return;
    }
    let context = &*context;
    let surface_id = context.surface;
    let global = &context.instance.0;
    let device = &*device;
    let device_id = device.device;
    let config = &*config;
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_context_get_current_texture(
    context: *mut CanvasGPUCanvasContext,
) -> *mut CanvasGPUTexture {
    if context.is_null() {
        return std::ptr::null_mut();
    }


    let context = unsafe { &*context };
    let surface_id = context.surface;
    let global = &context.instance.0;

    match gfx_select!(surface_id => global.surface_get_current_texture(surface_id, None)) {
        Ok(texture) => {
            match texture.status {
                wgpu_types::SurfaceStatus::Good | wgpu_types::SurfaceStatus::Suboptimal => {
                    Box::into_raw(Box::new(CanvasGPUTexture {
                        instance: context.instance.clone(),
                        texture: texture.texture_id.unwrap(),
                        owned: false,
                        depth_or_array_layers: 1,
                        dimension: super::enums::CanvasTextureDimension::D2,
                        format: context.format.into(),
                        mipLevelCount: 1,
                        sampleCount: 1,
                        width: context.width,
                        height: context.height,
                        usage: context.usage,
                    }))
                }
                _ => std::ptr::null_mut(),
            }
        },
        Err(error) => {
            // todo handle error
            std::ptr::null_mut()
        }
    }
}
