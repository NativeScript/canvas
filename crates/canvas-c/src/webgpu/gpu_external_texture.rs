//! `GPUDevice.importExternalTexture`. The decoder already outputs RGB, so the frame is one
//! RGBA plane with identity conversion. Null where the backend lacks `EXTERNAL_TEXTURE`.

use std::borrow::Cow;
use std::os::raw::{c_char, c_void};
use std::sync::Arc;

use crate::webgpu::prelude::{label_to_ptr, ptr_into_label};

use super::gpu_device::CanvasGPUDevice;

pub struct CanvasGPUExternalTexture {
    pub(crate) label: Option<Cow<'static, str>>,
    pub(crate) external_texture: Arc<wgpu_core::resource::ExternalTexture>,
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

    if !device
        .device
        .features()
        .contains(wgt::Features::EXTERNAL_TEXTURE)
    {
        return std::ptr::null();
    }

    let Some(plane) = super::gpu_native_texture::import_platform_texture(
        &device.device,
        native_texture,
        width,
        height,
    ) else {
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

    let external_texture = device
        .device
        .create_external_texture(&descriptor, &[plane_view]);

    Arc::into_raw(Arc::new(CanvasGPUExternalTexture {
        label,
        external_texture,
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
