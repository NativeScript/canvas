use std::{ffi::CString, os::raw::c_char};

use super::{
    enums::{CanvasGPUTextureFormat, CanvasTextureDimension},
    gpu::CanvasWebGPUInstance,
    gpu_texture_view::CanvasGPUTextureView,
};

#[derive(Clone)]
pub struct CanvasGPUTexture {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) texture: wgpu_core::id::TextureId,
    pub(crate) owned: bool,
    pub(crate) depth_or_array_layers: u32,
    pub(crate) dimension: CanvasTextureDimension,
    pub(crate) format: CanvasGPUTextureFormat,
    pub(crate) mipLevelCount: u32,
    pub(crate) sampleCount: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) usage: u32,
}

// #[no_mangle]
// pub extern "C" fn canvas_native_webgpu_texture_create_texture_view(
//     texture: *mut CanvasGPUTexture,
// )-> *mut CanvasGPUTextureView {
//     if texture.is_null() {
//         return;
//     }
//     let texture = unsafe { &*texture };
//     let texture_id = texture.texture;
//     let global = &texture.instance.0;

//     let desc = wgpu_core::resource::TextureViewDescriptor {

//     }

//     global.texture_create_view(texture_id, desc, id_in)

// }

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_get_depth_or_array_layers(
    texture: *mut CanvasGPUTexture,
) -> u32 {
    if texture.is_null() {
        return 0;
    }
    let texture = unsafe { &*texture };
    texture.depth_or_array_layers
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_get_width(texture: *mut CanvasGPUTexture) -> u32 {
    if texture.is_null() {
        return 0;
    }
    let texture = unsafe { &*texture };
    texture.width
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_get_height(texture: *mut CanvasGPUTexture) -> u32 {
    if texture.is_null() {
        return 0;
    }
    let texture = unsafe { &*texture };
    texture.height
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_get_dimension(
    texture: *mut CanvasGPUTexture,
) -> CanvasTextureDimension {
    if texture.is_null() {
        return CanvasTextureDimension::D2;
    }
    let texture = unsafe { &*texture };
    texture.dimension
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_get_format(
    texture: *mut CanvasGPUTexture,
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
pub extern "C" fn canvas_native_webgpu_texture_get_usage(texture: *mut CanvasGPUTexture) -> u32 {
    if texture.is_null() {
        return 0;
    }
    let texture = unsafe { &*texture };
    texture.usage
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_get_label(
    texture: *mut CanvasGPUTexture,
) -> *mut c_char {
    // todo
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_get_mip_level_count(
    texture: *mut CanvasGPUTexture,
) -> u32 {
    if texture.is_null() {
        return 0;
    }
    let texture = unsafe { &*texture };
    texture.mipLevelCount
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_get_sample_count(
    texture: *mut CanvasGPUTexture,
) -> u32 {
    if texture.is_null() {
        return 0;
    }
    let texture = unsafe { &*texture };
    texture.sampleCount
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_texture_destroy(
    texture: *mut CanvasGPUTexture,
) -> *mut c_char {
    if texture.is_null() {
        return std::ptr::null_mut();
    }
    let texture = unsafe { &*texture };
    let texture_id = texture.texture;
    let global = &texture.instance.0;

    match gfx_select!(texture_id => global.texture_destroy(texture_id)) {
        Ok(_) => std::ptr::null_mut(),
        Err(err) => {
            let err = err.to_string();
            CString::new(err).unwrap().into_raw()
        }
    }
}
