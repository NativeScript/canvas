use crate::webgpu::enums::{CanvasOptionalGPUTextureFormat, SurfaceGetCurrentTextureStatus};
use crate::webgpu::error::handle_error_fatal;
use crate::webgpu::gpu_adapter::CanvasGPUAdapter;
use crate::webgpu::gpu_device::ErrorSink;
use crate::webgpu::structs::{CanvasExtent3d, CanvasSurfaceCapabilities};
use base64::Engine;
use parking_lot::lock_api::Mutex;
use raw_window_handle::{AppKitDisplayHandle, RawDisplayHandle, RawWindowHandle};
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::sync::Arc;
use wgt::{SurfaceStatus, TextureFormat};

use super::{
    enums::CanvasGPUTextureFormat, gpu::CanvasWebGPUInstance, gpu_device::CanvasGPUDevice,
    gpu_texture::CanvasGPUTexture,
};

//use wgpu_core::gfx_select;

#[derive(Copy, Clone, Debug)]
struct TextureData {
    usage: wgt::TextureUsages,
    dimension: wgt::TextureDimension,
    size: wgt::Extent3d,
    format: wgt::TextureFormat,
    mip_level_count: u32,
    sample_count: u32,
}

#[derive(Debug)]
pub struct SurfaceData {
    device: Arc<CanvasGPUDevice>,
    error_sink: ErrorSink,
    texture_data: TextureData,
    previous_configuration: wgt::SurfaceConfiguration<Vec<wgt::TextureFormat>>,
}

#[derive(Copy, Clone, Debug)]
pub struct ViewData {
    pub width: u32,
    pub height: u32,
}

#[derive(Copy, Clone, Debug)]
pub struct ReadBackTexture {
    texture: wgpu_core::id::TextureId,
    data: TextureData,
}

pub struct CanvasGPUCanvasContext {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) surface: parking_lot::Mutex<wgpu_core::id::SurfaceId>,
    pub(crate) read_back_texture: parking_lot::Mutex<Option<ReadBackTexture>>,
    pub(crate) has_surface_presented: Arc<std::sync::atomic::AtomicBool>,
    pub(crate) data: parking_lot::Mutex<Option<SurfaceData>>,
    pub(crate) view_data: parking_lot::Mutex<ViewData>,
    pub(crate) current_texture: parking_lot::Mutex<Option<Arc<CanvasGPUTexture>>>,
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

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_to_data_url(
    context: *const CanvasGPUCanvasContext,
    format: *const c_char,
    quality: u32,
) -> *mut c_char {
    if context.is_null() {
        return std::ptr::null_mut();
    }
    let context = &*context;
    let data = context.data.lock();
    match &*data {
        Some(data) => {
            if format.is_null() {
                return std::ptr::null_mut();
            }
            let format = CStr::from_ptr(format as *const c_char).to_string_lossy();
            let device = data.device.as_ref();
            if let Some(data) = to_data_url(context, device, format.as_ref(), quality) {
                return CString::new(data).unwrap().into_raw();
            }

            std::ptr::null_mut()
        }
        None => std::ptr::null_mut(),
    }
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_to_data_url_with_texture(
    context: *const CanvasGPUCanvasContext,
    texture: *const CanvasGPUTexture,
    format: *const c_char,
    quality: u32,
) -> *mut c_char {
    if context.is_null() || texture.is_null() {
        return std::ptr::null_mut();
    }
    let context = &*context;

    let data = context.data.lock();
    match &*data {
        Some(data) => {
            let texture = &*texture;
            if format.is_null() {
                return std::ptr::null_mut();
            }
            let format = CStr::from_ptr(format as *const c_char).to_string_lossy();
            let device = data.device.as_ref();
            if let Some(data) = to_data_url_with_texture(context, device, texture.texture, texture.width, texture.height, format.as_ref(), quality) {
                return CString::new(data).unwrap().into_raw();
            }
            std::ptr::null_mut()
        }
        None => std::ptr::null_mut()
    }
}


fn to_data_url(
    context: &CanvasGPUCanvasContext,
    device: &CanvasGPUDevice,
    format: &str,
    quality: u32,
) -> Option<String> {
    let context = &*context;
    let read_back_texture_lock = context.read_back_texture.lock();
    match read_back_texture_lock.as_ref() {
        None => None,
        Some(texture) => {
            to_data_url_with_texture(context, device, texture.texture, texture.data.size.width, texture.data.size.height, format, quality)
        }
    }
}


fn round_up_to_256(value: u32) -> u32 {
    (value + 255) & !255
}

fn round_up_to_256_u64(value: u64) -> u64 {
    (value + 255) & !255
}


fn to_data_url_with_texture(
    context: &CanvasGPUCanvasContext,
    device: &CanvasGPUDevice,
    texture: wgpu_core::id::TextureId,
    width: u32,
    height: u32,
    format: &str,
    quality: u32,
) -> Option<String> {
    let context = &*context;
    let global = context.instance.global();
    let queue = &device.queue;
    let mut invalid = false;
    let mut is_bgra = false;
    {
        let lock = context.data.lock();
        if let Some(data) = lock.as_ref() {
            match data.texture_data.format {
                TextureFormat::Rgba8Unorm | TextureFormat::Rgba8UnormSrgb => {
                    is_bgra = false;
                }
                TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb => {
                    is_bgra = true
                }
                _ => {
                    invalid = true;
                }
            }
        }
    }
    if invalid {
        return None;
    }


    unsafe {
        let output_buffer_size = round_up_to_256_u64((width as u64) * 4) * (height as u64);
        let label = Cow::Borrowed("ToDataURL:Buffer");
        let (output_buffer, error) = global.device_create_buffer(device.device, &wgt::BufferDescriptor {
            label: wgpu_core::Label::from(label),
            size: output_buffer_size,
            usage: wgt::BufferUsages::COPY_DST | wgt::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        }, None);

        if let Some(_) = error {
            return None;
        }

        let texture_extent = wgt::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture_copy = wgt::ImageCopyTexture {
            texture,
            mip_level: 0,
            origin: wgt::Origin3d::ZERO,
            aspect: wgt::TextureAspect::All,
        };

        let buffer_copy = wgt::ImageCopyBuffer {
            buffer: output_buffer,
            layout: wgt::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(round_up_to_256(4 * width)),
                rows_per_image: Some(height),
            },
        };

        let label = Cow::Borrowed("ToDataURL:Encoder");

        let (encoder, error) = global.device_create_command_encoder(device.device, &wgt::CommandEncoderDescriptor {
            label: wgpu_core::Label::from(label),
        }, None);

        if let Some(_) = error {
            return None;
        }

        if let Err(_) = global.command_encoder_copy_texture_to_buffer(encoder, &texture_copy, &buffer_copy, &texture_extent) {
            return None;
        }

        let desc = wgt::CommandBufferDescriptor { label: None };

        let (id, error) = global.command_encoder_finish(encoder, &desc);

        if let Some(_) = error {
            return None;
        }

        global.queue_submit(queue.queue.id, &[id]).ok()?;

        let op = wgpu_core::resource::BufferMapOperation {
            host: wgpu_core::device::HostMap::Read,
            callback: None,
        };


        if let Err(_) = global.buffer_map_async(output_buffer, 0, None, op) {
            return None;
        }

        if let Err(_) = global.device_poll(device.device, wgt::Maintain::Wait) {
            return None;
        }

        match global.buffer_get_mapped_range(output_buffer, 0, None) {
            Ok((ptr, size)) => {
                let bytes = std::slice::from_raw_parts(ptr.as_ptr(), size as usize);

                if cfg!(feature = "2d") {
                    return Some(canvas_2d::bytes_to_data_n32_url(width as i32, height as i32, bytes, round_up_to_256_u64((4 * width) as u64) as usize, format, quality));
                }

                if cfg!(not(feature = "2d"))
                {
                    let mut buffer = None;

                    if is_bgra {
                        let mut bytes = bytes.to_vec();
                        canvas_core::image_asset::ImageAsset::bgra_to_rgba_in_place(&mut bytes);
                        buffer = image::ImageBuffer::from_raw(width, height, bytes)
                    } else {
                        buffer = image::ImageBuffer::from_raw(width, height, bytes.to_vec());
                    }

                    return match buffer {
                        Some(image) => {
                            let buffer = image::DynamicImage::ImageRgb8(
                                image,
                            );


                            let mut output = std::io::Cursor::new(Vec::new());

                            let fmt = match format {
                                "image/jpg" | "image/jpeg" => {
                                    buffer.write_to(&mut output, image::ImageFormat::Jpeg);
                                    "image/jpg"
                                }
                                "image/webp" => {
                                    buffer.write_to(&mut output, image::ImageFormat::WebP);
                                    "image/webp"
                                }
                                "image/gif" => {
                                    buffer.write_to(&mut output, image::ImageFormat::Gif);
                                    "image/gif"
                                }
                                "image/heif" | "image/heif-sequence" | "image/heic" | "image/heic-sequence" => {
                                    buffer.write_to(&mut output, image::ImageFormat::Avif);
                                    "image/heif"
                                }
                                "image/png" => {
                                    buffer.write_to(&mut output, image::ImageFormat::Png);
                                    "image/png"
                                }
                                _ => {
                                    ""
                                }
                            };


                            let mut data: Option<String> = None;
                            let encoded = base64::engine::general_purpose::STANDARD.encode(output.get_ref());
                            data = Some(format!("data:{};base64,{}", fmt, encoded));


                            if let Err(_) = global.buffer_unmap(output_buffer) {
                                return None;
                            }

                            data
                        }
                        None => None,
                    };
                }

                None
            }
            Err(_) => {
                None
            }
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
                read_back_texture: Mutex::default(),
                current_texture: Mutex::default(),
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
            drop(surface);
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

                let mut read_back_texture_lock = context.read_back_texture.lock();
                if let Some(texture) = read_back_texture_lock.take() {
                    global.texture_drop(texture.texture);
                }

                let ((read_back, error), texture_data) = {
                    #[cfg(any(target_os = "ios", target_os = "macos"))]
                    let mut format = wgt::TextureFormat::Bgra8Unorm;

                    #[cfg(any(target_os = "android"))]
                    let mut format = wgt::TextureFormat::Rgba8Unorm;


                    let desc = wgt::TextureDescriptor {
                        label: Some(Cow::Borrowed("ContextReadBack")),
                        size: wgt::Extent3d {
                            width,
                            height,
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgt::TextureDimension::D2,
                        format,
                        usage: wgt::TextureUsages::COPY_SRC | wgt::TextureUsages::COPY_DST,
                        view_formats: vec![],
                    };
                    let texture_data = TextureData {
                        usage: desc.usage,
                        dimension: desc.dimension,
                        size: desc.size,
                        format: desc.format,
                        mip_level_count: desc.mip_level_count,
                        sample_count: desc.sample_count,
                    };
                    (global.device_create_texture(surface_data.device.device, &desc, None), texture_data)
                };

                if let Some(cause) = error {
                    handle_error_fatal(global, cause, "canvas_native_webgpu_context_resize: create readback texture");
                } else {
                    *read_back_texture_lock = Some(ReadBackTexture {
                        texture: read_back,
                        data: texture_data,
                    });
                }

                if let Some(cause) = global.surface_configure(surface_id, surface_data.device.device, &new_config)
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
                read_back_texture: Mutex::default(),
                current_texture: Mutex::default(),
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
                read_back_texture: Default::default(),
                has_surface_presented: Arc::default(),
                data: Mutex::default(),
                view_data: Mutex::new(ViewData { width, height }),
                current_texture: Mutex::default(),
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


    let mut surface = context.surface.lock();

    let global = context.instance.global();

    global.surface_drop(*surface);

    let mut surface_data_lock = context.data.lock();

    let display_handle = RawDisplayHandle::UiKit(raw_window_handle::UiKitDisplayHandle::new());

    let handle = raw_window_handle::UiKitWindowHandle::new(std::ptr::NonNull::new_unchecked(view));

    let window_handle = RawWindowHandle::UiKit(handle);

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


                let mut read_back_texture_lock = context.read_back_texture.lock();
                if let Some(texture) = read_back_texture_lock.take() {
                    global.texture_drop(texture.texture);
                }

                let ((read_back, error), texture_data) = {
                    #[cfg(any(target_os = "ios", target_os = "macos"))]
                    let mut format = wgt::TextureFormat::Bgra8Unorm;

                    #[cfg(any(target_os = "android"))]
                    let mut format = wgt::TextureFormat::Rgba8Unorm;


                    let desc = wgt::TextureDescriptor {
                        label: Some(Cow::Borrowed("ContextReadBack")),
                        size: wgt::Extent3d {
                            width,
                            height,
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgt::TextureDimension::D2,
                        format,
                        usage: wgt::TextureUsages::COPY_SRC | wgt::TextureUsages::COPY_DST,
                        view_formats: vec![],
                    };

                    let texture_data = TextureData {
                        usage: desc.usage,
                        dimension: desc.dimension,
                        size: desc.size,
                        format: desc.format,
                        mip_level_count: desc.mip_level_count,
                        sample_count: desc.sample_count,
                    };

                    (global.device_create_texture(surface_data.device.device, &desc, None), texture_data)
                };

                if let Some(cause) = error {
                    handle_error_fatal(global, cause, "canvas_native_webgpu_context_resize_uiview: create readback texture");
                } else {
                    *read_back_texture_lock = Some(ReadBackTexture {
                        texture: read_back,
                        data: texture_data,
                    });
                }

                if let Some(cause) = global.surface_configure(surface_id, surface_data.device.device, &new_config)
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
                read_back_texture: Mutex::default(),
                current_texture: Mutex::default(),
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

                let mut read_back_texture_lock = context.read_back_texture.lock();
                if let Some(texture) = read_back_texture_lock.take() {
                    global.texture_drop(texture.texture);
                }

                let ((read_back, error), texture_data) = {
                    #[cfg(any(target_os = "ios", target_os = "macos"))]
                    let mut format = wgt::TextureFormat::Bgra8Unorm;

                    #[cfg(any(target_os = "android"))]
                    let mut format = wgt::TextureFormat::Rgba8Unorm;


                    let desc = wgt::TextureDescriptor {
                        label: Some(Cow::Borrowed("ContextReadBack")),
                        size: wgt::Extent3d {
                            width,
                            height,
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgt::TextureDimension::D2,
                        format,
                        usage: wgt::TextureUsages::COPY_SRC | wgt::TextureUsages::COPY_DST,
                        view_formats: vec![],
                    };
                    let texture_data = TextureData {
                        usage: desc.usage,
                        dimension: desc.dimension,
                        size: desc.size,
                        format: desc.format,
                        mip_level_count: desc.mip_level_count,
                        sample_count: desc.sample_count,
                    };

                    (global.device_create_texture(surface_data.device.device, &desc, None), texture_data)
                };

                if let Some(cause) = error {
                    handle_error_fatal(global, cause, "canvas_native_webgpu_context_resize_nsview: create readback texture");
                } else {
                    *read_back_texture_lock = Some(ReadBackTexture {
                        texture: read_back,
                        data: texture_data,
                    });
                }

                if let Some(cause) = global.surface_configure(surface_id, surface_data.device.device, &new_config)
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


#[cfg(any(target_os = "macos", target_os = "ios"))]
#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_context_resize_layer(
    context: *const CanvasGPUCanvasContext,
    layer: *mut c_void,
    width: u32,
    height: u32,
) {
    if context.is_null() {
        return;
    }
    let context = &*context;

    let mut surface_data_lock = context.data.lock();

    let global = context.instance.global();


    let mut surface = context.surface.lock();

    global.surface_drop(*surface);

    match global.instance_create_surface_metal(layer, None) {
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

                let mut read_back_texture_lock = context.read_back_texture.lock();
                if let Some(texture) = read_back_texture_lock.take() {
                    global.texture_drop(texture.texture);
                }

                let ((read_back, error), texture_data) = {
                    #[cfg(any(target_os = "ios", target_os = "macos"))]
                    let mut format = wgt::TextureFormat::Bgra8Unorm;

                    #[cfg(any(target_os = "android"))]
                    let mut format = wgt::TextureFormat::Rgba8Unorm;


                    let desc = wgt::TextureDescriptor {
                        label: Some(Cow::Borrowed("ContextReadBack")),
                        size: wgt::Extent3d {
                            width,
                            height,
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgt::TextureDimension::D2,
                        format,
                        usage: wgt::TextureUsages::COPY_SRC | wgt::TextureUsages::COPY_DST,
                        view_formats: vec![],
                    };
                    let texture_data = TextureData {
                        usage: desc.usage,
                        dimension: desc.dimension,
                        size: desc.size,
                        format: desc.format,
                        mip_level_count: desc.mip_level_count,
                        sample_count: desc.sample_count,
                    };

                    (global.device_create_texture(surface_data.device.device, &desc, None), texture_data)
                };

                if let Some(cause) = error {
                    handle_error_fatal(global, cause, "canvas_native_webgpu_context_resize_nsview: create readback texture");
                } else {
                    *read_back_texture_lock = Some(ReadBackTexture {
                        texture: read_back,
                        data: texture_data,
                    });
                }

                if let Some(cause) = global.surface_configure(surface_id, surface_data.device.device, &new_config)
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
    let device_ref = unsafe { &*device };
    let device_id = device_ref.device;

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

    let (width, height) = if !config.size.is_null() {
        let size = &*config.size;
        (size.width, size.height)
    } else {
        let view_data = context.view_data.lock();
        (view_data.width, view_data.height)
    };

    let config = wgt::SurfaceConfiguration::<Vec<TextureFormat>> {
        desired_maximum_frame_latency: 2,
        usage,
        format,
        width,
        height,
        present_mode: config.presentMode.into(),
        alpha_mode: config.alphaMode.into(),
        view_formats,
    };

    let mut read_back_texture_lock = context.read_back_texture.lock();
    if let Some(texture) = read_back_texture_lock.take() {
        global.texture_drop(texture.texture);
    }

    let ((read_back, error), texture_data) = {
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        let mut format = TextureFormat::Bgra8Unorm;

        #[cfg(any(target_os = "android"))]
        let mut format = wgt::TextureFormat::Rgba8Unorm;

        let desc = wgt::TextureDescriptor {
            label: Some(Cow::Borrowed("ContextReadBack")),
            size: wgt::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgt::TextureDimension::D2,
            format,
            usage: wgt::TextureUsages::COPY_SRC | wgt::TextureUsages::COPY_DST,
            view_formats: vec![],
        };

        let texture_data = TextureData {
            usage: desc.usage,
            dimension: desc.dimension,
            size: desc.size,
            format: desc.format,
            mip_level_count: desc.mip_level_count,
            sample_count: desc.sample_count,
        };

        (global.device_create_texture(device_id, &desc, None), texture_data)
    };

    if let Some(cause) = error {
        handle_error_fatal(global, cause, "canvas_native_webgpu_context_resize: create readback texture");
    } else {
        *read_back_texture_lock = Some(ReadBackTexture {
            texture: read_back,
            data: texture_data,
        });
    }

    if let Some(cause) = global.surface_configure(*surface_id, device_id, &config)
    {
        handle_error_fatal(global, cause, "canvas_native_webgpu_context_configure");
        let mut lock = context.data.lock();
        *lock = None;
    } else {
        let mut lock = context.data.lock();

        Arc::increment_strong_count(device);
        let device = Arc::from_raw(device);
        let error_sink = device.error_sink.clone();
        *lock = Some(SurfaceData {
            device,
            error_sink,
            texture_data: TextureData {
                usage,
                dimension: wgt::TextureDimension::D2,
                size: wgt::Extent3d {
                    width,
                    height,
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
pub extern "C" fn canvas_native_webgpu_context_has_current_texture(
    context: *const CanvasGPUCanvasContext,
) -> bool {
    if context.is_null() {
        return false;
    }

    let context = unsafe { &*context };
    {
        let current_texture = context.current_texture.lock();
        return current_texture.is_some();
    }

    false
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_context_get_current_texture(
    context: *const CanvasGPUCanvasContext,
) -> *const CanvasGPUTexture {
    if context.is_null() {
        return std::ptr::null();
    }

    let context = unsafe { &*context };
    {
        let current_texture = context.current_texture.lock();
        if let Some(current_texture) = current_texture.as_ref() {
            return Arc::into_raw(Arc::clone(current_texture));
        }
    }
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

            let texture = Arc::new(CanvasGPUTexture {
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
            });

            let ret = Arc::into_raw(
                Arc::clone(&texture)
            );

            *context.current_texture.lock() = Some(texture);

            ret
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


    {
        let lock = context.read_back_texture.lock();
        let surface_lock = context.data.lock();
        if let (Some(dst_texture), Some(data)) = (lock.as_ref(), surface_lock.as_ref()) {
            unsafe {
                let texture_extent = wgt::Extent3d {
                    width: dst_texture.data.size.width,
                    height: dst_texture.data.size.height,
                    depth_or_array_layers: 1,
                };

                let texture_src_copy = wgpu_core::command::ImageCopyTexture {
                    texture: texture.texture,
                    mip_level: 0,
                    origin: wgt::Origin3d::ZERO,
                    aspect: wgt::TextureAspect::All,
                };

                let texture_dst_copy = wgpu_core::command::ImageCopyTexture {
                    texture: dst_texture.texture,
                    mip_level: 0,
                    origin: wgt::Origin3d::ZERO,
                    aspect: wgt::TextureAspect::All,
                };

                let device = data.device.device;

                let label = Cow::Borrowed("PresentSurface:Encoder");

                let (encoder, error) = global.device_create_command_encoder(device, &wgt::CommandEncoderDescriptor {
                    label: wgpu_core::Label::from(label),
                }, None);

                if error.is_none() {
                    match global.command_encoder_copy_texture_to_texture(encoder, &texture_src_copy, &texture_dst_copy, &texture_extent) {
                        Ok(_) => {
                            let desc = wgt::CommandBufferDescriptor { label: None };

                            let (id, error) = global.command_encoder_finish(encoder, &desc);

                            if error.is_none() {
                                global.queue_submit(data.device.queue.queue.id, &[id]).ok();
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
        };
    }


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

        {
            let mut current_texture = context.current_texture.lock();
            if let Some(current_texture_ref) = current_texture.as_ref() {
                if current_texture_ref.texture == texture.texture {
                    *current_texture = None;
                }
            }
        }

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
