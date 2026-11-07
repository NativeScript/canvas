//! `GPUDevice.importExternalTexture`. The decoder already outputs RGB, so the frame is one
//! RGBA plane with identity conversion. Where the backend lacks `EXTERNAL_TEXTURE` (Vulkan) the
//! plane binds as a `texture_2d<f32>`, which shader modules use in place of `texture_external`
//! (`lower_external_textures`).

use std::borrow::Cow;
use std::os::raw::{c_char, c_void};
use std::sync::Arc;

use crate::webgpu::prelude::{label_to_ptr, ptr_into_label};

use super::gpu_device::CanvasGPUDevice;

pub struct CanvasGPUExternalTexture {
    pub(crate) label: Option<Cow<'static, str>>,
    /// None when emulated.
    pub(crate) external_texture: Option<Arc<wgpu_core::resource::ExternalTexture>>,
    pub(crate) plane: Arc<wgpu_core::resource::TextureView>,
}

impl CanvasGPUExternalTexture {
    pub(crate) fn binding_resource(&self) -> wgpu_core::binding_model::BindingResource<'static> {
        match &self.external_texture {
            Some(texture) => {
                wgpu_core::binding_model::BindingResource::ExternalTexture(Arc::clone(texture))
            }
            None => wgpu_core::binding_model::BindingResource::TextureView(Arc::clone(&self.plane)),
        }
    }
}

/// Without `EXTERNAL_TEXTURE`: `texture_external` becomes `texture_2d<f32>` and its 2-argument
/// `textureLoad`s get level 0 (sampling and `textureDimensions` read the same on both). None
/// when there is nothing to change.
pub(crate) fn lower_external_textures(source: &str) -> Option<String> {
    const EXTERNAL: &str = "texture_external";
    if !source.contains(EXTERNAL) {
        return None;
    }
    let is_ident = |c: char| c.is_ascii_alphanumeric() || c == '_';
    // Names declared `name: texture_external` (globals and parameters).
    let mut names = Vec::new();
    let mut search = 0;
    while let Some(found) = source[search..].find(EXTERNAL) {
        let at = search + found;
        search = at + EXTERNAL.len();
        let before = source[..at].trim_end();
        if let Some(before) = before.strip_suffix(':') {
            let before = before.trim_end();
            let start = before
                .char_indices()
                .rev()
                .find(|(_, c)| !is_ident(*c))
                .map_or(0, |(i, c)| i + c.len_utf8());
            if start < before.len() {
                names.push(&before[start..]);
            }
        }
    }
    let mut out = source.replace(EXTERNAL, "texture_2d<f32>");
    const LOAD: &str = "textureLoad(";
    let mut search = 0;
    while let Some(found) = out[search..].find(LOAD) {
        let open = search + found + LOAD.len();
        search = open;
        let first = out[open..]
            .trim_start()
            .split(|c: char| !is_ident(c))
            .next()
            .unwrap_or("");
        if !names.contains(&first) {
            continue;
        }
        let (mut depth, mut commas, mut close) = (1, 0, None);
        for (i, c) in out[open..].char_indices() {
            match c {
                '(' | '[' => depth += 1,
                ')' | ']' => {
                    depth -= 1;
                    if depth == 0 {
                        close = Some(open + i);
                        break;
                    }
                }
                ',' if depth == 1 => commas += 1,
                _ => {}
            }
        }
        if let (Some(close), 1) = (close, commas) {
            out.insert_str(close, ", 0");
        }
    }
    Some(out)
}

/// Identity affine transform in wgpu's column-major 3x2 layout.
const IDENTITY_TRANSFORM: [f32; 6] = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];

#[rustfmt::skip]
const IDENTITY_4X4: [f32; 16] = [
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0,
];

#[rustfmt::skip]
const IDENTITY_3X3: [f32; 9] = [
    1.0, 0.0, 0.0,
    0.0, 1.0, 0.0,
    0.0, 0.0, 1.0,
];

/// `native_texture` is a borrowed `MTLTexture*`. Keep the platform frame alive while the result
/// can be sampled: the decoder recycles its buffer once the frame is released.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_import_external_texture(
    device: *const CanvasGPUDevice,
    label: *const c_char,
    native_texture: *mut c_void,
    width: u32,
    height: u32,
) -> *const CanvasGPUExternalTexture {
    if device.is_null() || native_texture.is_null() || width == 0 || height == 0 {
        return std::ptr::null();
    }

    let device = &*device;

    let emulated = !device
        .device
        .features()
        .contains(wgt::Features::EXTERNAL_TEXTURE);

    #[cfg(target_os = "windows")]
    let plane = super::gpu_shared_frame::import_external_plane(device, native_texture, width, height);
    #[cfg(target_os = "linux")]
    let plane = super::linux_video_frame::import_external_plane(device, native_texture, width, height);
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    let plane = super::gpu_native_texture::import_platform_texture(&device.device, native_texture, width, height);
    let Some(plane) = plane else {
        return std::ptr::null();
    };

    let plane_view = plane.create_view(&wgpu_core::resource::TextureViewDescriptor {
        label: Some(Cow::Borrowed("externalTexture:Plane")),
        ..Default::default()
    });

    let label = ptr_into_label(label);

    let descriptor = wgpu_core::resource::ExternalTextureDescriptor {
        label: label.clone(),
        width,
        height,
        format: wgt::ExternalTextureFormat::Rgba,
        yuv_conversion_matrix: IDENTITY_4X4,
        gamut_conversion_matrix: IDENTITY_3X3,
        src_transfer_function: Default::default(),
        dst_transfer_function: Default::default(),
        sample_transform: IDENTITY_TRANSFORM,
        load_transform: IDENTITY_TRANSFORM,
    };

    let external_texture =
        (!emulated).then(|| device.device.create_external_texture(&descriptor, &[Arc::clone(&plane_view)]));

    Arc::into_raw(Arc::new(CanvasGPUExternalTexture {
        label,
        external_texture,
        plane: plane_view,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_external_texture_get_label(
    external_texture: *const CanvasGPUExternalTexture,
) -> *mut c_char {
    if external_texture.is_null() {
        return std::ptr::null_mut();
    }

    let external_texture = &*external_texture;
    label_to_ptr(external_texture.label.clone())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_external_texture_reference(
    external_texture: *const CanvasGPUExternalTexture,
) {
    if external_texture.is_null() {
        return;
    }

    Arc::increment_strong_count(external_texture);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_external_texture_release(
    external_texture: *const CanvasGPUExternalTexture,
) {
    if external_texture.is_null() {
        return;
    }

    Arc::decrement_strong_count(external_texture);
}
