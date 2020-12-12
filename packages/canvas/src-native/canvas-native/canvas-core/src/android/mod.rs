use android_logger::Config;
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jbyteArray, jfloat, jint, jlong, JNI_FALSE, JNI_TRUE, jobject, jstring};
use log::{debug, info};
use log::Level;
use skia_safe::{AlphaType, Color, ColorType, EncodedImageFormat, ImageInfo, ISize, PixelGeometry, Surface};
use skia_safe::gpu::gl::Interface;

use crate::common::context::{Context, Device, State};
use crate::common::context::paths::path::Path;
use crate::common::context::text_styles::text_direction::TextDirection;
use crate::common::to_data_url;

pub mod context;
pub mod utils;
pub mod gradient;
pub mod image_asset;
pub mod image_data;
pub mod matrix;
pub mod paint;
pub mod path;
pub mod pattern;
pub mod text_decoder;
pub mod text_encoder;
pub mod gl;
pub mod text_metrics;

const GR_GL_RGB565: u32 = 0x8D62;
const GR_GL_RGBA8: u32 = 0x8058;

#[no_mangle]
pub extern "system" fn JNI_OnLoad() -> jint {
    {
        android_logger::init_once(Config::default().with_min_level(Level::Debug));
        info!("Canvas Native library loaded");
    }

    jni::sys::JNI_VERSION_1_6
}


#[no_mangle]
pub extern "C" fn Java_com_github_triniwiz_canvas_TNSCanvas_nativeInitContext(
    _: JNIEnv,
    _: JClass,
    width: jfloat,
    height: jfloat,
    density: jfloat,
    buffer_id: jint,
    samples: jint,
    alpha: jboolean,
    font_color: jint,
    ppi: jfloat,
    direction: jint,
) -> jlong {
    let density = 1.0;
    let mut device = Device {
        width,
        height,
        density,
        non_gpu: false,
        samples: samples as usize,
        alpha: alpha == JNI_TRUE,
        ppi,
    };
    let interface = Interface::new_native();
    let mut ctx = skia_safe::gpu::Context::new_gl(interface).unwrap();
    let mut frame_buffer = skia_safe::gpu::gl::FramebufferInfo::from_fboid(buffer_id as u32);
    if alpha == JNI_TRUE {
        frame_buffer.format = GR_GL_RGBA8;
    } else {
        frame_buffer.format = GR_GL_RGB565;
    }

    let target = skia_safe::gpu::BackendRenderTarget::new_gl(
        (width as i32, height as i32),
        Some(samples as usize),
        8,
        frame_buffer,
    );
    let surface_props =
        skia_safe::SurfaceProps::new(skia_safe::SurfacePropsFlags::default(), PixelGeometry::Unknown);
    let mut color_type = ColorType::RGBA8888;
    if alpha == JNI_FALSE {
        color_type = ColorType::RGB565;
    }
    let mut surface_holder = Surface::from_backend_render_target(
        &mut ctx,
        &target,
        skia_safe::gpu::SurfaceOrigin::BottomLeft,
        color_type,
        None,
        Some(&surface_props),
    );

    Box::into_raw(Box::new(Context {
        surface: surface_holder.unwrap(),
        path: Path::default(),
        state: State::from_device(device, TextDirection::from(direction)),
        state_stack: vec![],
        font_color: Color::new(font_color as u32),
        device,
    })) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_github_triniwiz_canvas_TNSCanvas_nativeInitContextWithCustomSurface(
    _: JNIEnv,
    _: JClass,
    width: jfloat,
    height: jfloat,
    density: jfloat,
    alpha: jboolean,
    font_color: jint,
    ppi: jfloat,
    direction: jint,
) -> jlong {
    let density = 1.0;
    let mut device = Device {
        width,
        height,
        density,
        non_gpu: true,
        samples: 0,
        alpha: alpha == JNI_TRUE,
        ppi,
    };
    Box::into_raw(Box::new(Context {
        surface: Surface::new_raster_n32_premul(
            ISize::new(width as i32, height as i32)
        ).unwrap(),
        path: Path::default(),
        state: State::from_device(device, TextDirection::from(direction)),
        state_stack: vec![],
        font_color: Color::new(font_color as u32),
        device,
    })) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_github_triniwiz_canvas_TNSCanvas_nativeDestroyContext(_: JNIEnv,
                                                                                            _: JClass, context: jlong) {
    if context == 0 {
        return;
    }
    unsafe {
        let context: *mut Context = context as _;
        let _ = Box::from_raw(context);
    }
}


#[no_mangle]
pub extern "C" fn Java_com_github_triniwiz_canvas_TNSCanvas_nativeResizeSurface(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    width: jfloat,
    height: jfloat,
    density: jfloat,
    buffer_id: jint,
    samples: jint,
    alpha: jboolean,
    ppi: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let interface = skia_safe::gpu::gl::Interface::new_native();
        let ctx = skia_safe::gpu::Context::new_gl(interface);
        if ctx.is_none() {
            return;
        }
        let mut ctx = ctx.unwrap();
        ctx.reset(None);
        let mut device = Device {
            width,
            height,
            density,
            non_gpu: false,
            samples: samples as usize,
            alpha: alpha == JNI_TRUE,
            ppi,
        };
        let mut frame_buffer = skia_safe::gpu::gl::FramebufferInfo::from_fboid(buffer_id as u32);

        if alpha == JNI_TRUE {
            frame_buffer.format = GR_GL_RGBA8;
        } else {
            frame_buffer.format = GR_GL_RGB565;
        }

        let target = skia_safe::gpu::BackendRenderTarget::new_gl(
            (width as i32, height as i32),
            Some(samples as usize),
            8,
            frame_buffer,
        );
        let surface_props =
            skia_safe::SurfaceProps::new(skia_safe::SurfacePropsFlags::default(), PixelGeometry::Unknown);
        let mut color_type = ColorType::RGBA8888;

        if alpha == JNI_FALSE {
            color_type = ColorType::RGB565;
        }
        if let Some(surface) = Surface::from_backend_render_target(
            &mut ctx,
            &target,
            skia_safe::gpu::SurfaceOrigin::BottomLeft,
            color_type,
            None,
            Some(&surface_props),
        ) {
            context.surface = surface;
            context.device = device;
            context.path = Path::default();
            context.reset_state();
        }
    }
}


#[no_mangle]
pub extern "C" fn Java_com_github_triniwiz_canvas_TNSCanvas_nativeDataURL(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    format: JString,
    quality: jfloat,
) -> jstring {
    unsafe {
        if context == 0 {
            return env.new_string("").unwrap().into_inner();
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if let Ok(format) = env.get_string(format) {
            let format = format.to_string_lossy();
            return env.new_string(
                to_data_url(
                    context,
                    format.as_ref(),
                    (quality * 100 as f32) as i32,
                )
            ).unwrap().into_inner();
        }
        return env.new_string("").unwrap().into_inner();
    }
}

#[no_mangle]
pub extern "C" fn Java_com_github_triniwiz_canvas_TNSCanvas_nativeSnapshotCanvas(env: JNIEnv,
                                                                                 _: JClass, context: jlong) -> jbyteArray {
    if context == 0 {
        return env.new_byte_array(0).unwrap();
    }
    unsafe {
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.surface.canvas().flush();
        let ss = context.surface.image_snapshot();
        let data = ss.encode_to_data(EncodedImageFormat::PNG).unwrap();
        let mut bytes = data.to_vec();
        env.byte_array_from_slice(bytes.as_slice()).unwrap()
    }
}


#[no_mangle]
pub extern "C" fn Java_com_github_triniwiz_canvas_TNSCanvas_nativeFlush(_: JNIEnv,
                                                                        _: JClass, context: jlong) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.flush()
    }
}

#[no_mangle]
pub extern "C" fn Java_com_github_triniwiz_canvas_TNSCanvas_nativeCustomWithBitmapFlush(env: JNIEnv, _: JNIEnv,
                                                                                        _: JClass, context: jlong, bitmap: JObject) {
    unsafe {
        if context == 0 {
            return;
        }


        let native_interface = env.get_native_interface();
        let bitmap = bitmap.into_inner();
        let bitmap_info: *mut crate::android::utils::bitmap::AndroidBitmapInfo = Box::into_raw(Box::new(crate::android::utils::bitmap::AndroidBitmapInfo::default()));

        unsafe {
            if crate::android::utils::bitmap::AndroidBitmap_getInfo(native_interface as _, bitmap, bitmap_info)
                < crate::android::utils::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
            {
                debug!("nativeCustomWithBitmapFlush: Get Bitmap Info Failed");
                return;
            }
        }
        let bitmap_info = *unsafe { Box::from_raw(bitmap_info) };
        let mut pixels = std::ptr::null_mut() as *mut std::os::raw::c_void;
        let mut pixels_ptr: *mut *mut std::os::raw::c_void = &mut pixels;
        unsafe {
            if crate::android::utils::bitmap::AndroidBitmap_lockPixels(native_interface as _, bitmap, pixels_ptr)
                < crate::android::utils::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
            {
                debug!("nativeCustomWithBitmapFlush: Get Bitmap Lock Failed");
                return;
            }
        }
        let length = (bitmap_info.height * bitmap_info.stride as u32) as usize;
        let slice: &mut [u8] =
            unsafe { std::slice::from_raw_parts_mut(pixels as *mut u8, length as usize) };


        let context: *mut Context = context as _;
        let context = &mut *context;


        let info = ImageInfo::new(
            ISize::new(bitmap_info.width as i32, bitmap_info.height as i32),
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );
        let surface = &mut context.surface;

        let mut dst_surface = Surface::new_raster_direct(&info, slice, None, None).unwrap();
        let dst_canvas = dst_surface.canvas();
        surface.draw(dst_canvas, ISize::new(0, 0), None);
        surface.flush_and_submit();
        dst_surface.flush_and_submit();

        unsafe {
            if crate::android::utils::bitmap::AndroidBitmap_unlockPixels(native_interface as _, bitmap) < crate::android::utils::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
            {
                debug!("nativeCustomWithBitmapFlush: Unlock Bitmap Failed");
            }
        }
    }
}
