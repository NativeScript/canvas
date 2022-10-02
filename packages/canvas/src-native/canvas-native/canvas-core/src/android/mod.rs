use android_logger::Config;
use jni::{JNIEnv, objects::GlobalRef};
use jni::objects::{JClass, JMethodID, JObject, JStaticMethodID, JString, JValue};
use jni::sys::{jboolean, jbyteArray, jfloat, jint, jlong, JNI_FALSE, JNI_TRUE, jstring};
use log::Level;
use skia_safe::{
    AlphaType, Color, ColorType, EncodedImageFormat, ImageInfo, ISize, PixelGeometry, Rect, Surface,
};
use skia_safe::gpu::gl::Interface;

use std::ffi::c_void;

use jni::JavaVM;

use crate::common::context::{Context, Device, State};
use crate::common::context::paths::path::Path;
use crate::common::context::text_styles::text_direction::TextDirection;
use crate::common::to_data_url;

use once_cell::sync::OnceCell;
use parking_lot::RwLock;

use std::collections::HashMap;
use jni::signature::{Primitive, ReturnType};


pub mod context;
pub mod gl;
pub mod gradient;
pub mod image_asset;
pub mod image_bitmap;
pub mod image_data;
pub mod matrix;
pub mod paint;
pub mod path;
pub mod pattern;
pub mod svg;
pub mod text_decoder;
pub mod text_encoder;
pub mod text_metrics;
pub mod utils;

const GR_GL_RGB565: u32 = 0x8D62;
const GR_GL_RGBA8: u32 = 0x8058;

type FromSurfaceTexture<'a> = Option<libloading::Symbol<'a,
    unsafe fn(
        env: *mut JNIEnv,
        surfacetexture: jni::sys::jobject,
    ) -> *mut gl::surface_texture::ASurfaceTexture>>;

type UpdateTexImage<'a> = Option<libloading::Symbol<'a,
    unsafe fn(st: *mut gl::surface_texture::ASurfaceTexture) -> std::os::raw::c_int
>>;

type GetTransformMatrix<'a> = Option<libloading::Symbol<'a,
    unsafe fn(st: *mut gl::surface_texture::ASurfaceTexture, mtx: *mut f32)
>>;

type Release<'a> = Option<libloading::Symbol<'a,
    unsafe fn(
        st: *mut gl::surface_texture::ASurfaceTexture
    )
>>;


type FromSurfaceTextureRaw = Option<libloading::os::unix::Symbol<
    unsafe fn(
        env: *mut JNIEnv,
        surfacetexture: jni::sys::jobject,
    ) -> *mut gl::surface_texture::ASurfaceTexture>>;

type UpdateTexImageRaw = Option<libloading::os::unix::Symbol<
    unsafe fn(st: *mut gl::surface_texture::ASurfaceTexture) -> std::os::raw::c_int
>>;

type GetTransformMatrixRaw = Option<libloading::os::unix::Symbol<
    unsafe fn(st: *mut gl::surface_texture::ASurfaceTexture, mtx: *mut f32)
>>;

type ReleaseRaw = Option<libloading::os::unix::Symbol<
    unsafe fn(
        st: *mut gl::surface_texture::ASurfaceTexture
    )
>>;

type FromSurfaceTextureRef<'a> = Option<&'a libloading::os::unix::Symbol<
    unsafe fn(
        env: *mut JNIEnv,
        surfacetexture: jni::sys::jobject,
    ) -> *mut gl::surface_texture::ASurfaceTexture>>;

type UpdateTexImageRef<'a> = Option<&'a libloading::os::unix::Symbol<
    unsafe fn(st: *mut gl::surface_texture::ASurfaceTexture) -> std::os::raw::c_int
>>;

type GetTransformMatrixRef<'a> = Option<&'a libloading::os::unix::Symbol<
    unsafe fn(st: *mut gl::surface_texture::ASurfaceTexture, mtx: *mut f32)
>>;

type ReleaseRef<'a> = Option<&'a libloading::os::unix::Symbol<
    unsafe fn(
        st: *mut gl::surface_texture::ASurfaceTexture
    )
>>;

pub struct SurfaceTexture {
    lib: libloading::Library,
    from_surface_texture: FromSurfaceTextureRaw,
    update_tex_image: UpdateTexImageRaw,
    get_transform_matrix: GetTransformMatrixRaw,
    release: ReleaseRaw,
}

impl SurfaceTexture {
    pub fn new() -> Self {
        // should not fail to load the lib

        let lib = unsafe { libloading::Library::new("libandroid.so").unwrap() };

        let from_surface_texture: FromSurfaceTexture = unsafe {
            lib.get(b"ASurfaceTexture_fromSurfaceTexture\0")
                .ok()
        };

        let from_surface_texture = from_surface_texture.map(|v| unsafe { v.into_raw() });

        let update_tex_image: UpdateTexImage = unsafe {
            lib.get(b"ASurfaceTexture_updateTexImage\0")
                .ok()
        };

        let update_tex_image = update_tex_image.map(|v| unsafe { v.into_raw() });

        let get_transform_matrix: GetTransformMatrix = unsafe {
            lib.get(b"ASurfaceTexture_getTransformMatrix\0")
                .ok()
        };

        let get_transform_matrix = get_transform_matrix.map(|v| unsafe { v.into_raw() });

        let release: Release = unsafe {
            lib.get(b"ASurfaceTexture_release\0")
                .ok()
        };

        let release = release.map(|v| unsafe { v.into_raw() });

        Self {
            lib,
            from_surface_texture,
            update_tex_image,
            get_transform_matrix,
            release,
        }
    }

    pub fn from_surface_texture(&self) -> FromSurfaceTextureRef {
        self.from_surface_texture.as_ref()
    }
    pub fn update_tex_image(&self) -> UpdateTexImageRef {
        self.update_tex_image.as_ref()
    }
    pub fn get_transform_matrix(&self) -> GetTransformMatrixRef {
        self.get_transform_matrix.as_ref()
    }
    pub fn release(&self) -> ReleaseRef {
        self.release.as_ref()
    }
}

impl Drop for SurfaceTexture {
    fn drop(&mut self) {
        if let Some(from_surface_texture) = self.from_surface_texture.clone() {
            let _ = unsafe { libloading::Symbol::from_raw(from_surface_texture, &self.lib) };
        }
        if let Some(update_tex_image) = self.update_tex_image.clone() {
            let _ = unsafe { libloading::Symbol::from_raw(update_tex_image, &self.lib) };
        }
        if let Some(get_transform_matrix) = self.get_transform_matrix.clone() {
            let _ = unsafe { libloading::Symbol::from_raw(get_transform_matrix, &self.lib) };
        }
        if let Some(release) = self.release.clone() {
            let _ = unsafe { libloading::Symbol::from_raw(release, &self.lib) };
        }
    }
}


pub static JVM: OnceCell<JavaVM> = OnceCell::new();

pub static JVM_CLASS_CACHE: OnceCell<RwLock<HashMap<&'static str, GlobalRef>>> =
    OnceCell::new();

pub static JVM_METHOD_CACHE: OnceCell<RwLock<HashMap<&'static str, MethodCacheItem>>> =
    OnceCell::new();

pub static JVM_STATIC_METHOD_CACHE: OnceCell<RwLock<HashMap<&'static str, StaticMethodCacheItem>>> =
    OnceCell::new();

pub static SURFACE_TEXTURE: OnceCell<SurfaceTexture> = OnceCell::new();

#[derive(Clone)]
pub struct MethodCacheItem {
    clazz: GlobalRef,
    id: JMethodID,
}

impl MethodCacheItem {
    pub fn new(clazz: GlobalRef, id: JMethodID) -> Self {
        Self {
            clazz,
            id,
        }
    }

    pub fn clazz(&self) -> JClass {
        JClass::from(self.clazz.as_obj().into_inner())
    }
}

#[derive(Clone)]
pub struct StaticMethodCacheItem {
    clazz: GlobalRef,
    id: JStaticMethodID,
}

impl StaticMethodCacheItem {
    pub fn new(clazz: GlobalRef, id: JStaticMethodID) -> Self {
        Self {
            clazz,
            id,
        }
    }

    pub fn clazz(&self) -> JClass {
        JClass::from(self.clazz.as_obj().into_inner())
    }
}

pub(crate) const COLOR_STYLE_REF_CLASS: &str = "org/nativescript/canvas/TNSColorStyleRef";
pub(crate) const GC_CLASS: &str = "org/nativescript/canvas/GC";
pub(crate) const TEXTURE_RENDER_CLASS: &str = "org/nativescript/canvas/TextureRender";

pub(crate) const GC_STATIC_WATCH_OBJECT_METHOD: &str = "org_nativescript_canvas_GC_watchObject";
pub(crate) const TEXTURE_RENDER_STATIC_UPDATE_TEX_IMAGE_AND_GET_TRANSFORM_MATRIX_METHOD: &str = "org_nativescript_canvas_TextureRender_updateTexImageAndGetTransformMatrix";
pub(crate) static METHOD_CTOR: &str = "<init>";
pub(crate) static COLOR_STYLE_REF_SIG_OBJECT_CTOR: &str = "(JI)V";

pub(crate) const COLOR_STYLE_REF_CTOR: &str = "org_nativescript_canvas_TNSColorStyleRef_ctor";

pub fn find_class(name: &str) -> Option<JClass> {
    JVM_CLASS_CACHE.get().map_or(None, |c| {
        c.read()
            .get(name)
            .map(|c| JClass::from(c.as_obj().into_inner()))
    })
}

pub fn find_method_id(name: &str) -> Option<MethodCacheItem> {
    JVM_METHOD_CACHE.get().map_or(None, |c| {
        c.read()
            .get(name)
            .map(|c| c.clone())
    })
}

pub fn find_static_method_id(name: &str) -> Option<StaticMethodCacheItem> {
    JVM_STATIC_METHOD_CACHE.get().map_or(None, |c| {
        c.read()
            .get(name)
            .map(|c| c.clone())
    })
}

pub fn new_style_ref<'a>(env: &JNIEnv<'a>, style: jlong, style_type: i32) -> JObject<'a> {
    let method_id = find_method_id(COLOR_STYLE_REF_CTOR).unwrap();

    env.new_object_unchecked(
        method_id.clazz(), method_id.id, &[
            JValue::Long(style), JValue::Int(style_type)
        ],
    ).unwrap()


    // env.new_object(clazz, "(JI)V", &[
    //     JValue::Long(style), JValue::Int(style_type.into())
    // ]).unwrap()
}

pub(crate) const JAVA_VOID_TYPE: ReturnType = ReturnType::Primitive(Primitive::Void);

pub fn watch_item<'a>(env: &JNIEnv<'a>, id: jlong, buffer: JValue) {
    let method_id = find_static_method_id(GC_STATIC_WATCH_OBJECT_METHOD).unwrap();
    env.call_static_method_unchecked(
        method_id.clazz(), method_id.id, JAVA_VOID_TYPE, &[JValue::Long(id).to_jni(), buffer.to_jni()],
    ).unwrap();
}

#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: *const c_void) -> jint {
    {
        android_logger::init_once(Config::default().with_min_level(Level::Debug));
        log::info!("Canvas Native library loaded");
    }

    if let Ok(env) = vm.get_env() {
        SURFACE_TEXTURE.get_or_init(|| {
            SurfaceTexture::new()
        });

        let clazz = env.find_class(COLOR_STYLE_REF_CLASS).unwrap();

        let gc_clazz = env.find_class(GC_CLASS).unwrap();

        let tr_clazz = env.find_class(TEXTURE_RENDER_CLASS).unwrap();

        let watch_item_method = env.get_static_method_id(
            gc_clazz, "watchObject", "(JLjava/nio/ByteBuffer;)V",
        ).unwrap();

        let color_style_ctor_id = env.get_method_id(
            clazz, METHOD_CTOR, COLOR_STYLE_REF_SIG_OBJECT_CTOR,
        ).unwrap();

        let update_tex_image_and_get_transform_matrix_method = env.get_static_method_id(
            tr_clazz, "updateTexImageAndGetTransformMatrix", "(Landroid/graphics/SurfaceTexture;[F)V",
        ).unwrap();


        let clazz = env.new_global_ref(clazz).unwrap();

        let gc_clazz = env.new_global_ref(gc_clazz).unwrap();

        let tr_clazz = env.new_global_ref(tr_clazz).unwrap();

        let watch_item_method = StaticMethodCacheItem::new(gc_clazz.clone(), watch_item_method);

        let color_style_ctor_id = MethodCacheItem::new(clazz.clone(), color_style_ctor_id);

        let update_tex_image_and_get_transform_matrix_method = StaticMethodCacheItem::new(tr_clazz.clone(), update_tex_image_and_get_transform_matrix_method);

        JVM_CLASS_CACHE.get_or_init(|| {
            let mut map = HashMap::new();
            map.insert(COLOR_STYLE_REF_CLASS, clazz);
            map.insert(GC_CLASS, gc_clazz);
            map.insert(TEXTURE_RENDER_CLASS, tr_clazz);
            parking_lot::RwLock::new(map)
        });


        JVM_STATIC_METHOD_CACHE.get_or_init(|| {
            let mut map = HashMap::new();
            map.insert(GC_STATIC_WATCH_OBJECT_METHOD, watch_item_method);
            map.insert(TEXTURE_RENDER_STATIC_UPDATE_TEX_IMAGE_AND_GET_TRANSFORM_MATRIX_METHOD, update_tex_image_and_get_transform_matrix_method);
            parking_lot::RwLock::new(map)
        });


        JVM_METHOD_CACHE.get_or_init(|| {
            let mut map = HashMap::new();
            map.insert(COLOR_STYLE_REF_CTOR, color_style_ctor_id);
            parking_lot::RwLock::new(map)
        });
    }

    JVM.get_or_init(|| vm);
    jni::sys::JNI_VERSION_1_6
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeInitContext(
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
    let device = Device {
        width,
        height,
        density,
        non_gpu: false,
        samples: samples as usize,
        alpha: alpha == JNI_TRUE,
        ppi,
    };
    let interface = Interface::new_native();
    let mut ctx = skia_safe::gpu::DirectContext::new_gl(interface, None).unwrap();
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
    let surface_props = skia_safe::SurfaceProps::new(
        skia_safe::SurfacePropsFlags::default(),
        PixelGeometry::Unknown,
    );
    let mut color_type = ColorType::RGBA8888;
    if alpha == JNI_FALSE {
        color_type = ColorType::RGB565;
    }
    let surface_holder = Surface::from_backend_render_target(
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

pub(crate) fn init_with_custom_surface(
    width: jfloat,
    height: jfloat,
    density: jfloat,
    alpha: jboolean,
    font_color: jint,
    ppi: jfloat,
    direction: jint,
) -> jlong {
    // let density = 1.0;
    let device = Device {
        width,
        height,
        density,
        non_gpu: true,
        samples: 0,
        alpha: alpha == JNI_TRUE,
        ppi,
    };
    let info = ImageInfo::new(
        ISize::new(width as i32, height as i32),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );

    Box::into_raw(Box::new(Context {
        surface: Surface::new_raster(&info, None, None).unwrap(),
        path: Path::default(),
        state: State::from_device(device, TextDirection::from(direction)),
        state_stack: vec![],
        font_color: Color::new(font_color as u32),
        device,
    })) as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeInitContextWithCustomSurface(
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
    init_with_custom_surface(width, height, density, alpha, font_color, ppi, direction)
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeDestroyContext(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) {
    if context == 0 {
        return;
    }
    unsafe {
        let context: *mut Context = context as _;
        let _ = Box::from_raw(context);
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeResizeSurface(
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
        let interface = Interface::new_native();
        let ctx = skia_safe::gpu::DirectContext::new_gl(interface, None);
        if ctx.is_none() {
            return;
        }
        let mut ctx = ctx.unwrap();
        ctx.reset(None);
        let device = Device {
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
        let surface_props = skia_safe::SurfaceProps::new(
            skia_safe::SurfacePropsFlags::default(),
            PixelGeometry::Unknown,
        );
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
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeResizeCustomSurface(
    _: JNIEnv,
    _: JClass,
    context: jlong,
    width: jfloat,
    height: jfloat,
    density: jfloat,
    alpha: jboolean,
    ppi: jfloat,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let device = Device {
            width,
            height,
            density,
            non_gpu: true,
            samples: 0,
            alpha: alpha == JNI_TRUE,
            ppi,
        };

        let info = ImageInfo::new(
            ISize::new(width as i32, height as i32),
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );

        if let Some(surface) = Surface::new_raster(&info, None, None) {
            context.surface = surface;
            context.device = device;
            context.path = Path::default();
            context.reset_state();
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeDataURL(
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
            return env
                .new_string(to_data_url(
                    context,
                    format.as_ref(),
                    (quality * 100 as f32) as i32,
                ))
                .unwrap()
                .into_inner();
        }
        return env.new_string("").unwrap().into_inner();
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeSnapshotCanvas(
    env: JNIEnv,
    _: JClass,
    context: jlong,
) -> jbyteArray {
    if context == 0 {
        return env.new_byte_array(0).unwrap();
    }
    unsafe {
        let context: *mut Context = context as _;
        let context = &mut *context;

        let ss = context.surface.image_snapshot();
        match ss.to_raster_image(
            skia_safe::image::CachingHint::Allow
        ) {
            Some(image) => {
                let mut info = ImageInfo::new(
                    ISize::new(ss.width(), ss.height()),
                    ColorType::RGBA8888,
                    AlphaType::Unpremul,
                    None,
                );
                let row_bytes = info.width() * 4;
                let mut pixels = vec![255u8; (row_bytes * info.height()) as usize];
                let _read = image.read_pixels(
                    &mut info,
                    pixels.as_mut_slice(),
                    row_bytes as usize,
                    skia_safe::IPoint::new(0, 0),
                    skia_safe::image::CachingHint::Allow,
                );

                env.byte_array_from_slice(pixels.as_slice()).unwrap()
            }
            _ => {
                env.new_byte_array(0).unwrap()
            }
        }
    }
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeSnapshotCanvasEncoded(
    env: JNIEnv,
    _: JClass,
    context: jlong,
) -> jbyteArray {
    if context == 0 {
        return env.new_byte_array(0).unwrap();
    }
    unsafe {
        let context: *mut Context = context as _;
        let context = &mut *context;
        let ss = context.surface.image_snapshot();

        return match ss.encode_to_data(EncodedImageFormat::PNG) {
            None => env.byte_array_from_slice(&[]).unwrap(),
            Some(data) => {
                let data = data.to_vec();
                env.byte_array_from_slice(data.as_slice()).unwrap()
            }
        };
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeFlush(
    _: JNIEnv,
    _: JClass,
    context: jlong,
) {
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
pub extern "system" fn Java_org_nativescript_canvas_TNSCanvas_nativeCustomWithBitmapFlush(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    bitmap: JObject,
) {
    unsafe {
        if context == 0 {
            return;
        }
        utils::image::bitmap_handler(
            env,
            bitmap,
            Box::new(move |image_data, image_info| {
                let info = ImageInfo::new(
                    ISize::new(image_info.width as i32, image_info.height as i32),
                    ColorType::RGBA8888,
                    AlphaType::Premul,
                    None,
                );
                let context: *mut Context = context as _;
                let context = &mut *context;
                let mut surface =
                    Surface::new_raster_direct(&info, image_data, None, None).unwrap();
                let canvas = surface.canvas();
                let mut paint = skia_safe::Paint::default();
                paint.set_anti_alias(true);
                paint.set_style(skia_safe::PaintStyle::Fill);
                paint.set_blend_mode(skia_safe::BlendMode::Clear);
                canvas.draw_rect(
                    Rect::from_xywh(
                        0f32,
                        0f32,
                        image_info.width as f32,
                        image_info.height as f32,
                    ),
                    &paint,
                );
                context.draw_on_surface(&mut surface);
            }),
        )
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_GC_disposeByteBufMut(
    _env: JNIEnv,
    _: JClass,
    buf: jlong,
) {
    let buf: *mut crate::common::prelude::ByteBufMut = buf as _;
    if !buf.is_null() {
        let _ = unsafe { Box::from_raw(buf) };
    }
}
