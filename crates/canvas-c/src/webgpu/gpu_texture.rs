use std::os::raw::c_char;
use std::sync::Arc;

use crate::webgpu::error::{handle_error, handle_error_fatal};
use crate::webgpu::prelude::ptr_into_label;

use super::{
    enums::{
        CanvasGPUTextureFormat, CanvasOptionalTextureViewDimension,
        CanvasOptionalGPUTextureFormat, CanvasTextureDimension,
    },
    gpu::CanvasWebGPUInstance,
    gpu_texture_view::CanvasGPUTextureView,
    structs::CanvasImageSubresourceRange,
};

#[derive(Clone)]
pub struct CanvasGPUTexture {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) texture: wgpu_core::id::TextureId,
    pub(crate) surface_id: Option<wgpu_core::id::SurfaceId>,
    pub(crate) owned: bool,
    pub(crate) depth_or_array_layers: u32,
    pub(crate) dimension: CanvasTextureDimension,
    pub(crate) format: CanvasGPUTextureFormat,
    pub(crate) mipLevelCount: u32,
    pub(crate) sampleCount: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) usage: u32,
    pub(crate) error_sink: super::gpu_device::ErrorSink,
    pub(crate) has_surface_presented: Arc<std::sync::atomic::AtomicBool>,
}

impl Drop for CanvasGPUTexture {
    fn drop(&mut self) {
        if std::thread::panicking() {
            return;
        }
        match self.surface_id {
            Some(surface_id) => {
                if !self.has_surface_presented.load(std::sync::atomic::Ordering::SeqCst) {
                    let global = self.instance.global();
                    match gfx_select!(self.id => global.surface_texture_discard(surface_id)) {
                        Ok(_) => (),
                        Err(cause) => handle_error_fatal(global, cause, "canvas_native_webgpu_texture_release"),
                    }
                }
            }
            None => {
                let context = self.instance.global();
                gfx_select!(self.id => context.texture_drop(self.texture, false));
            }
        }
    }
}

#[repr(C)]
pub struct CanvasCreateTextureViewDescriptor {
    label: *const c_char,
    format: CanvasOptionalGPUTextureFormat,
    dimension: CanvasOptionalTextureViewDimension,
    range: *const CanvasImageSubresourceRange,
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_texture_reference(texture: *const CanvasGPUTexture) {
    if texture.is_null() { return; }
    Arc::increment_strong_count(texture);
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_texture_release(texture: *const CanvasGPUTexture) {
    if texture.is_null() { return; }
    Arc::decrement_strong_count(texture);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_texture_create_texture_view(
    texture: *const CanvasGPUTexture,
    descriptor: *const CanvasCreateTextureViewDescriptor,
) -> *const CanvasGPUTextureView {
    if texture.is_null() {
        return std::ptr::null_mut();
    }
    let texture = unsafe { &*texture };
    let texture_id = texture.texture;
    let global = texture.instance.global();

    let desc = if descriptor.is_null() {
        wgpu_core::resource::TextureViewDescriptor::default()
    } else {
        let descriptor = &*descriptor;

        let label = ptr_into_label(descriptor.label);

        let range = *descriptor.range;

        wgpu_core::resource::TextureViewDescriptor {
            label,
            format: descriptor.format.into(),
            dimension: descriptor.dimension.into(),
            range: range.into(),
        }
    };

    let (texture_view, error) =
        gfx_select!(texture_id => global.texture_create_view(texture_id, &desc, None));

    let error_sink = texture.error_sink.as_ref();

    if let Some(cause) = error {
        handle_error(
            global,
            error_sink,
            cause,
            "",
            None,
            "canvas_native_webgpu_texture_create_texture_view",
        );
    }

    Arc::into_raw(Arc::new(CanvasGPUTextureView {
        instance: texture.instance.clone(),
        texture_view,
    }))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_get_depth_or_array_layers(
    texture: *const CanvasGPUTexture,
) -> u32 {
    if texture.is_null() {
        return 0;
    }
    let texture = unsafe { &*texture };
    texture.depth_or_array_layers
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_get_width(texture: *const CanvasGPUTexture) -> u32 {
    if texture.is_null() {
        return 0;
    }
    let texture = unsafe { &*texture };
    texture.width
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_get_height(texture: *const CanvasGPUTexture) -> u32 {
    if texture.is_null() {
        return 0;
    }
    let texture = unsafe { &*texture };
    texture.height
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_get_dimension(
    texture: *const CanvasGPUTexture,
) -> CanvasTextureDimension {
    if texture.is_null() {
        return CanvasTextureDimension::D2;
    }
    let texture = unsafe { &*texture };
    texture.dimension
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_get_format(
    texture: *const CanvasGPUTexture,
) -> CanvasGPUTextureFormat {
    if texture.is_null() {
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        return CanvasGPUTextureFormat::Bgra8Unorm;

        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        return CanvasGPUTextureFormat::Rgba8Unorm;
    }
    let texture = unsafe { &*texture };
    texture.format
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_get_usage(texture: *const CanvasGPUTexture) -> u32 {
    if texture.is_null() {
        return 0;
    }
    let texture = unsafe { &*texture };
    texture.usage
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_get_label(
    _texture: *const CanvasGPUTexture,
) -> *mut c_char {
    // todo
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_get_mip_level_count(
    texture: *const CanvasGPUTexture,
) -> u32 {
    if texture.is_null() {
        return 0;
    }
    let texture = unsafe { &*texture };
    texture.mipLevelCount
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_get_sample_count(
    texture: *const CanvasGPUTexture,
) -> u32 {
    if texture.is_null() {
        return 0;
    }
    let texture = unsafe { &*texture };
    texture.sampleCount
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_destroy(
    texture: *const CanvasGPUTexture,
) {
    if texture.is_null() {
        return;
    }
    let texture = unsafe { &*texture };
    let texture_id = texture.texture;
    let global = texture.instance.global();

    let _ = gfx_select!(texture_id => global.texture_destroy(texture_id));
}
