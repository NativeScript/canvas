#![allow(non_snake_case)]

extern crate android_logger;
extern crate core;
#[macro_use]
extern crate log;

use std::os::raw::c_void;

use ::jni::signature::JavaType;
use ::jni::sys::jint;
use ::jni::JavaVM;
use android_logger::Config;
use itertools::izip;
use jni::objects::JClass;
use jni::sys::jlong;
use jni::{JNIEnv, NativeMethod};
use log::LevelFilter;
use once_cell::sync::OnceCell;

use crate::jni_compat::org_nativescript_canvas_NSCCanvas::{
    nativeContext2DPathTest, nativeContext2DPathTestNormal, nativeContext2DRender,
    nativeContext2DTest, nativeContext2DTestNormal, nativeCreate2DContext,
    nativeCreate2DContextNormal, nativeCustomWithBitmapFlush, nativeGLPointerRefCount,
    nativeGLPointerRefCountNormal, nativeGetGLPointer, nativeGetGLPointerNormal, nativeInitGL,
    nativeInitGLNoSurface, nativeMakeGLCurrent, nativeMakeGLCurrentNormal, nativeReleaseGL,
    nativeReleaseGLNormal, nativeReleaseGLPointer, nativeReleaseGLPointerNormal,
    nativeUpdate2DSurface, nativeUpdate2DSurfaceNoSurface, nativeUpdate2DSurfaceNoSurfaceNormal,
    nativeUpdateGLNoSurface, nativeUpdateGLNoSurfaceNormal, nativeUpdateGLSurface,
    nativeWebGLC2DRender, nativeWriteCurrentGLContextToBitmap,
};
use crate::jni_compat::org_nativescript_canvas_NSCCanvasRenderingContext2D::{
    nativeCreatePattern, nativeDrawAtlasWithBitmap, nativeDrawImageDxDyDwDhWithAsset,
    nativeDrawImageDxDyDwDhWithBitmap, nativeDrawImageDxDyWithAsset, nativeDrawImageDxDyWithBitmap,
    nativeDrawImageWithAsset, nativeDrawImageWithBitmap,
};
use crate::jni_compat::org_nativescript_canvas_NSCImageAsset::{
    nativeCreateImageAsset, nativeDestroyImageAsset, nativeGetDimensions, nativeGetError,
    nativeLoadFromBitmap, nativeLoadFromPath,
};
use crate::jni_compat::org_nativescript_canvas_NSCWebGLRenderingContext::{nativeTexImage2D, nativeTexSubImage2D};

use crate::utils::gl::st::{SurfaceTexture, SURFACE_TEXTURE};
use crate::utils::gl::texture_render::nativeDrawFrame;
use crate::utils::{
    nativeInitContextWithCustomSurface, nativeInitContextWithCustomSurfaceNormal,
    nativeResizeCustomSurface, nativeResizeCustomSurfaceNormal,
};

mod jni_compat;
pub mod utils;

pub static JVM: OnceCell<JavaVM> = OnceCell::new();

pub static API_LEVEL: OnceCell<i32> = OnceCell::new();

pub(crate) const BUILD_VERSION_CLASS: &str = "android/os/Build$VERSION";
const NSC_CANVAS_CLASS: &str = "org/nativescript/canvas/NSCCanvas";
const NSC_CANVAS_RENDERING_CONTEXT2D_CLASS: &str =
    "org/nativescript/canvas/NSCCanvasRenderingContext2D";
const ANDROID_O: i32 = 26;
#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: *const c_void) -> jint {
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Trace));

    if let Ok(mut env) = vm.get_env() {
        API_LEVEL.get_or_init(|| {
            let clazz = env.find_class(BUILD_VERSION_CLASS).unwrap();

            let sdk_int_id = env.get_static_field_id(&clazz, "SDK_INT", "I").unwrap();

            let sdk_int = env.get_static_field_unchecked(
                clazz,
                sdk_int_id,
                JavaType::Primitive(jni::signature::Primitive::Int),
            );

            let ret = sdk_int.unwrap().i().unwrap();

            canvas_c::API_LEVEL.get_or_init(|| ret);

            let canvas_class = env.find_class(NSC_CANVAS_CLASS).unwrap();

            let canvas_method_names = [
                "nativeInitGL",
                "nativeInitGLNoSurface",
                "nativeCreate2DContext",
                "nativeUpdateGLSurface",
                "nativeUpdate2DSurface",
                "nativeUpdate2DSurfaceNoSurface",
                "nativeUpdateGLNoSurface",
                "nativeReleaseGL",
                "nativeMakeGLCurrent",
                "nativeGLPointerRefCount",
                "nativeGetGLPointer",
                "nativeReleaseGLPointer",
                "nativeWriteCurrentGLContextToBitmap",
                "nativeInitContextWithCustomSurface",
                "nativeResizeCustomSurface",
                "nativeCustomWithBitmapFlush",
                "nativeContext2DTest",
                "nativeContext2DPathTest",
                "nativeContext2DRender",
                "nativeWebGLC2DRender",
            ];

            let canvas_signatures = if ret >= ANDROID_O {
                [
                    "(Landroid/view/Surface;ZZZZIZZZZZIZ)J",
                    "(IIZZZZIZZZZZIZ)J",
                    "(JIIZFIIFI)J",
                    "(Landroid/view/Surface;J)V",
                    "(Landroid/view/Surface;J)V",
                    "(IIJ)V",
                    "(IIJ)V",
                    "(J)V",
                    "(J)Z",
                    "(J)J",
                    "(J)J",
                    "(J)V",
                    "(JLandroid/graphics/Bitmap;)V",
                    "(FFFZIFI)J",
                    "(JFFFZI)V",
                    "(JLandroid/graphics/Bitmap;)V",
                    "(J)V",
                    "(J)V",
                    "(J)V",
                    "(JJII)V",
                ]
            } else {
                [
                    "!(Landroid/view/Surface;ZZZZIZZZZZIZ)J",
                    "!(IIZZZZIZZZZZIZ)J",
                    "!(JIIZFIIFI)J",
                    "!(Landroid/view/Surface;J)V",
                    "!(Landroid/view/Surface;J)V",
                    "!(IIJ)V",
                    "!(IIJ)V",
                    "!(J)V",
                    "!(J)Z",
                    "!(J)J",
                    "!(J)J",
                    "!(J)V",
                    "!(JLandroid/graphics/Bitmap;)V",
                    "!(FFFZIFI)J",
                    "!(JFFFZI)V",
                    "!(JLandroid/graphics/Bitmap;)V",
                    "!(J)V",
                    "!(J)V",
                    "!(J)V",
                    "!(JJII)V",
                ]
            };

            let canvas_methods = if ret >= ANDROID_O {
                [
                    nativeInitGL as *mut c_void,
                    nativeInitGLNoSurface as *mut c_void,
                    nativeCreate2DContext as *mut c_void,
                    nativeUpdateGLSurface as *mut c_void,
                    nativeUpdate2DSurface as *mut c_void,
                    nativeUpdate2DSurfaceNoSurface as *mut c_void,
                    nativeUpdateGLNoSurface as *mut c_void,
                    nativeReleaseGL as *mut c_void,
                    nativeMakeGLCurrent as *mut c_void,
                    nativeGLPointerRefCount as *mut c_void,
                    nativeGetGLPointer as *mut c_void,
                    nativeReleaseGLPointer as *mut c_void,
                    nativeWriteCurrentGLContextToBitmap as *mut c_void,
                    nativeInitContextWithCustomSurface as *mut c_void,
                    nativeResizeCustomSurface as *mut c_void,
                    nativeCustomWithBitmapFlush as *mut c_void,
                    nativeContext2DTest as *mut c_void,
                    nativeContext2DPathTest as *mut c_void,
                    nativeContext2DRender as *mut c_void,
                    nativeWebGLC2DRender as *mut c_void,
                ]
            } else {
                [
                    nativeInitGL as *mut c_void,
                    nativeInitGLNoSurface as *mut c_void,
                    nativeCreate2DContextNormal as *mut c_void,
                    nativeUpdateGLSurface as *mut c_void,
                    nativeUpdate2DSurface as *mut c_void,
                    nativeUpdate2DSurfaceNoSurfaceNormal as *mut c_void,
                    nativeUpdateGLNoSurfaceNormal as *mut c_void,
                    nativeReleaseGLNormal as *mut c_void,
                    nativeMakeGLCurrentNormal as *mut c_void,
                    nativeGLPointerRefCountNormal as *mut c_void,
                    nativeGetGLPointerNormal as *mut c_void,
                    nativeReleaseGLPointerNormal as *mut c_void,
                    nativeWriteCurrentGLContextToBitmap as *mut c_void,
                    nativeInitContextWithCustomSurfaceNormal as *mut c_void,
                    nativeResizeCustomSurfaceNormal as *mut c_void,
                    nativeCustomWithBitmapFlush as *mut c_void,
                    nativeContext2DTestNormal as *mut c_void,
                    nativeContext2DPathTestNormal as *mut c_void,
                    nativeContext2DRender as *mut c_void,
                    nativeWebGLC2DRender as *mut c_void,
                ]
            };

            let canvas_native_methods: Vec<NativeMethod> =
                izip!(canvas_method_names, canvas_signatures, canvas_methods)
                    .map(|(name, signature, method)| NativeMethod {
                        name: name.into(),
                        sig: signature.into(),
                        fn_ptr: method,
                    })
                    .collect();

            let _ = env.register_native_methods(&canvas_class, canvas_native_methods.as_slice());

            let text_render_class = env
                .find_class("org/nativescript/canvas/TextureRender")
                .unwrap();

            let nativeDrawFrameMethod = if ret >= ANDROID_O {
                "(Landroid/graphics/SurfaceTexture;ZIIIIIII[FIIIIIIII)V"
            } else {
                "!(Landroid/graphics/SurfaceTexture;ZIIIIIII[FIIIIIIII)V"
            };

            let _ = env.register_native_methods(
                &text_render_class,
                &[NativeMethod {
                    name: "nativeDrawFrame".into(),
                    sig: nativeDrawFrameMethod.into(),
                    fn_ptr: nativeDrawFrame as *mut c_void,
                }],
            );

            let canvas_rendering_context_2d_class = env
                .find_class(NSC_CANVAS_RENDERING_CONTEXT2D_CLASS)
                .unwrap();

            let canvas_rendering_context_2d_method_names = [
                "nativeCreatePattern",
                "nativeDrawImageDxDyWithBitmap",
                "nativeDrawImageDxDyDwDhWithBitmap",
                "nativeDrawImageWithBitmap",
                "nativeDrawAtlasWithBitmap",
                "nativeDrawImageDxDyWithAsset",
                "nativeDrawImageDxDyDwDhWithAsset",
                "nativeDrawImageWithAsset",
            ];

            let canvas_rendering_context_2d_signatures = if ret >= ANDROID_O {
                [
                    "(JLandroid/graphics/Bitmap;Ljava/lang/String;)J",
                    "(JLandroid/graphics/Bitmap;FFFF)Z",
                    "(JLandroid/graphics/Bitmap;FFFFFF)Z",
                    "(JLandroid/graphics/Bitmap;FFFFFFFFFF)Z",
                    "(JLandroid/graphics/Bitmap;[F[F[II)V",
                    "(JJFF)Z",
                    "(JJFFFF)Z",
                    "(JJFFFFFFFF)Z",
                ]
            } else {
                [
                    "!(JLandroid/graphics/Bitmap;Ljava/lang/String;)J",
                    "!(JLandroid/graphics/Bitmap;FFFF)Z",
                    "!(JLandroid/graphics/Bitmap;FFFFFF)Z",
                    "!(JLandroid/graphics/Bitmap;FFFFFFFFFF)Z",
                    "!(JLandroid/graphics/Bitmap;[F[F[II)V",
                    "!(JJFF)Z",
                    "!(JJFFFF)Z",
                    "!(JJFFFFFFFF)Z",
                ]
            };

            let canvas_rendering_context_2d_methods = [
                nativeCreatePattern as *mut c_void,
                nativeDrawImageDxDyWithBitmap as *mut c_void,
                nativeDrawImageDxDyDwDhWithBitmap as *mut c_void,
                nativeDrawImageWithBitmap as *mut c_void,
                nativeDrawAtlasWithBitmap as *mut c_void,
                nativeDrawImageDxDyWithAsset as *mut c_void,
                nativeDrawImageDxDyDwDhWithAsset as *mut c_void,
                nativeDrawImageWithAsset as *mut c_void,
            ];

            let canvas_rendering_context_2d_native_methods: Vec<NativeMethod> = izip!(
                canvas_rendering_context_2d_method_names,
                canvas_rendering_context_2d_signatures,
                canvas_rendering_context_2d_methods
            )
            .map(|(name, signature, method)| NativeMethod {
                name: name.into(),
                sig: signature.into(),
                fn_ptr: method,
            })
            .collect();

            let _ = env.register_native_methods(
                &canvas_rendering_context_2d_class,
                canvas_rendering_context_2d_native_methods.as_slice(),
            );

            let image_asset_class = env
                .find_class("org/nativescript/canvas/NSCImageAsset")
                .unwrap();

            let image_asset_method_names = [
                "nativeCreateImageAsset",
                "nativeDestroyImageAsset",
                "nativeLoadFromBitmap",
                "nativeGetDimensions",
                "nativeLoadFromPath",
                "nativeGetError",
            ];

            let image_asset_signatures = if ret >= ANDROID_O {
                [
                    "()J",
                    "(J)V",
                    "(JLandroid/graphics/Bitmap;)Z",
                    "(J[I)V",
                    "(JLjava/lang/String;)Z",
                    "(J)Ljava/lang/String;",
                ]
            } else {
                [
                    "!()J",
                    "!(J)V",
                    "!(JLandroid/graphics/Bitmap;)Z",
                    "!(J[I)V",
                    "!(JLjava/lang/String;)Z",
                    "!(J)Ljava/lang/String;",
                ]
            };

            let image_asset_methods = [
                nativeCreateImageAsset as *mut c_void,
                nativeDestroyImageAsset as *mut c_void,
                nativeLoadFromBitmap as *mut c_void,
                nativeGetDimensions as *mut c_void,
                nativeLoadFromPath as *mut c_void,
                nativeGetError as *mut c_void,
            ];

            let image_asset_native_methods: Vec<NativeMethod> = izip!(
                image_asset_method_names,
                image_asset_signatures,
                image_asset_methods
            )
            .map(|(name, signature, method)| NativeMethod {
                name: name.into(),
                sig: signature.into(),
                fn_ptr: method,
            })
            .collect();

            let _ = env
                .register_native_methods(&image_asset_class, image_asset_native_methods.as_slice());



            let webgl_rendering_class = env
                .find_class("org/nativescript/canvas/NSCWebGLRenderingContext")
                .unwrap();

            let webgl_rendering_method_names = [
                "nativeTexImage2D",
                "nativeTexSubImage2D"
            ];

            let webgl_rendering_signatures = if ret >= ANDROID_O {
                [
                    "(JIIIIILandroid/graphics/Bitmap;Z)V",
                    "(JIIIIIILandroid/graphics/Bitmap;Z)V",
                ]
            } else {
                [
                    "!(JIIIIILandroid/graphics/Bitmap;Z)V",
                    "!(JIIIIIILandroid/graphics/Bitmap;Z)v",
                ]
            };

            let webgl_rendering_methods = [
                nativeTexImage2D as *mut c_void,
                nativeTexSubImage2D as *mut c_void,
            ];

            let webgl_rendering_native_methods: Vec<NativeMethod> = izip!(
                webgl_rendering_method_names,
                webgl_rendering_signatures,
                webgl_rendering_methods
            )
                .map(|(name, signature, method)| NativeMethod {
                    name: name.into(),
                    sig: signature.into(),
                    fn_ptr: method,
                })
                .collect();

            let _ = env
                .register_native_methods(&webgl_rendering_class, webgl_rendering_native_methods.as_slice());

            ret
        });
    }

    SURFACE_TEXTURE.get_or_init(|| SurfaceTexture::new());

    JVM.get_or_init(|| vm);

    log::info!(target: "JS","Canvas library loaded");

    jni::sys::JNI_VERSION_1_6
}
