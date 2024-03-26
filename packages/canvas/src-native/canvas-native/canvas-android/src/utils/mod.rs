use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jfloat, jint, jlong, jstring};
use jni::JNIEnv;

use canvas_2d::context::{Context, ContextWrapper};

pub mod gl;
pub mod image;

pub fn get_sdk_version() -> i32 {
    // todo
    -1
}

pub(crate) fn init_with_custom_surface(
    width: jfloat,
    height: jfloat,
    density: jfloat,
    alpha: jboolean,
    font_color: jint,
    ppi: jfloat,
    direction: jint,
) -> jlong {
    Box::into_raw(Box::new(canvas_c::CanvasRenderingContext2D::new(
        ContextWrapper::new(Context::new(
            width,
            height,
            density,
            alpha == jni::sys::JNI_TRUE,
            font_color,
            ppi,
            canvas_2d::context::text_styles::text_direction::TextDirection::from(direction as u32),
        )),
        canvas_core::gl::GLContext::default(),
        alpha == jni::sys::JNI_TRUE,
    ))) as jlong
}

#[no_mangle]
pub extern "system" fn nativeInitContextWithCustomSurface(
    width: jfloat,
    height: jfloat,
    density: jfloat,
    alpha: jboolean,
    font_color: jint,
    ppi: jfloat,
    direction: jint,
) -> jlong {
    init_with_custom_surface(width, height, density, alpha, font_color, ppi, direction)
}

#[no_mangle]
pub extern "system" fn nativeInitContextWithCustomSurfaceNormal(
    _env: JNIEnv,
    _: JClass,
    width: jfloat,
    height: jfloat,
    density: jfloat,
    alpha: jboolean,
    font_color: jint,
    ppi: jfloat,
    direction: jint,
) -> jlong {
    init_with_custom_surface(width, height, density, alpha, font_color, ppi, direction)
}

#[no_mangle]
pub extern "system" fn nativeResizeCustomSurface(
    context: jlong,
    width: jfloat,
    height: jfloat,
    _density: jfloat,
    _alpha: jboolean,
    _ppi: jfloat,
) {
    if context == 0 {
        return;
    }
    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };
    context.resize(width, height);
}

#[no_mangle]
pub extern "system" fn nativeResizeCustomSurfaceNormal(
    _env: JNIEnv,
    _: JClass,
    context: jlong,
    width: jfloat,
    height: jfloat,
    _density: jfloat,
    _alpha: jboolean,
    _ppi: jfloat,
) {
    if context == 0 {
        return;
    }
    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };
    context.resize(width, height);
}

/*
#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCCanvas_nativeCustomWithBitmapFlush(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    bitmap: JObject,
) {
    unsafe {
        if context == 0 {
            return;
        }
        image::bitmap_handler(
            &env,
            bitmap,
            Box::new(move |image_data| {
                if let Some((image_data, image_info)) = image_data {
                    let context: *mut ContextWrapper = context as _;
                    let context = &mut *context;
                    let mut context = context.get_context_mut();
                    Context::flush_buffer(
                        &mut context,
                        image_info.width() as i32,
                        image_info.height() as i32,
                        image_data,
                    )
                }
            }),
        )
    }
}
*/

#[no_mangle]
pub extern "system" fn nativeDestroyContext(context: jlong) {
    unsafe {
        if context == 0 {
            return;
        }

        let context = context as *mut canvas_c::CanvasRenderingContext2D;
        let _ = Box::from_raw(context);
    }
}

#[no_mangle]
pub extern "system" fn nativeDataURL(
    mut env: JNIEnv,
    _: JClass,
    context: jlong,
    format: JString,
    quality: jfloat,
) -> jstring {
    if context == 0 {
        return env.new_string("").unwrap().into_raw();
    }
    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };
    if let Ok(format) = env.get_string(&format) {
        let format = format.to_string_lossy();

        return env
            .new_string(canvas_2d::to_data_url_context(
                &mut context.get_context_mut(),
                format.as_ref(),
                (quality * 100f32) as u32,
            ))
            .unwrap()
            .into_raw();
    }
    return env.new_string("").unwrap().into_raw();
}

#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct ByteBufInner {
    needs_to_clean: bool,
}

#[repr(C)]
pub struct ByteBuf {
    pub data: *const u8,
    pub len: usize,
    inner: ByteBufInner,
}

impl ByteBuf {
    pub fn new(data: *const u8, length: usize) -> Self {
        Self {
            data,
            len: length,
            inner: Default::default(),
        }
    }

    pub fn as_slice<'a>(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.data, self.len) }
    }
}

impl From<Vec<u8>> for ByteBuf {
    fn from(vec: Vec<u8>) -> Self {
        let len = vec.len();
        let mut slice = vec.into_boxed_slice();
        let data = slice.as_mut_ptr() as *const u8;
        let _ = Box::into_raw(slice);
        Self {
            data,
            len,
            inner: ByteBufInner {
                needs_to_clean: true,
            },
        }
    }
}

impl Drop for ByteBuf {
    fn drop(&mut self) {
        if !self.inner.needs_to_clean {
            return;
        }
        if !self.data.is_null() && self.len != 0 {
            unsafe {
                let _ = Box::from_raw(std::slice::from_raw_parts_mut(
                    self.data as *mut u8,
                    self.len,
                ));
            }
        }
    }
}

unsafe impl Send for ByteBuf {}
