#![allow(non_snake_case)]

extern crate android_logger;
extern crate core;
extern crate log;

use std::os::raw::c_void;
use std::sync::OnceLock;

use ::jni::signature::JavaType;
use ::jni::sys::jint;
use ::jni::JavaVM;
use android_logger::Config;
use itertools::izip;
use jni::NativeMethod;
use jni::sys::{jfloat, jlong};
use log::LevelFilter;

// #[cfg(feature = "vulkan")]
use crate::jni_compat::org_nativescript_canvas_NSCCanvas::{ nativeCreate2dContextVulkan};

use crate::jni_compat::org_nativescript_canvas_NSCCanvas::{nativeContext2DPathTest, nativeContext2DPathTestNormal, nativeContext2DRender, nativeContext2DTest, nativeContext2DTestNormal, nativeCreate2DContext, nativeCustomWithBitmapFlush, nativeInitWebGL, nativeInitWebGLNoSurface, nativeInitWebGPU, nativeMakeWebGLCurrent, nativeMakeWebGLCurrentNormal, nativeReleaseWebGL, nativeReleaseWebGLNormal, nativeResizeWebGPU, nativeUpdate2DSurface, nativeUpdate2DSurfaceNoSurface, nativeUpdate2DSurfaceNoSurfaceNormal, nativeUpdateGLNoSurface, nativeUpdateWebGLNoSurfaceNormal, nativeUpdateWebGLSurface, nativeWebGLC2DRender, nativeWriteCurrentWebGLContextToBitmap};
use crate::jni_compat::org_nativescript_canvas_NSCCanvasRenderingContext2D::{nativeCreatePattern, nativeDrawAtlasWithBitmap, nativeDrawImageDxDyDwDhWithAsset, nativeDrawImageDxDyDwDhWithBitmap, nativeDrawImageDxDyWithAsset, nativeDrawImageDxDyWithBitmap, nativeDrawImageWithAsset, nativeDrawImageWithBitmap, nativeScale};
use crate::jni_compat::org_nativescript_canvas_NSCImageAsset::{nativeCreateImageAsset, nativeDestroyImageAsset, nativeGetDimensions, nativeGetError, nativeLoadFromBitmap, nativeLoadFromBuffer, nativeLoadFromBytes, nativeLoadFromPath, nativeLoadFromUrl};
use crate::jni_compat::org_nativescript_canvas_NSCImageBitmap::{nativeLoadBitmapFromBuffer, nativeLoadBitmapFromBufferOptions, nativeLoadBitmapFromBufferRectOptions, nativeLoadBitmapFromBytes, nativeLoadBitmapFromBytesOptions, nativeLoadBitmapFromBytesRectOptions};
use crate::jni_compat::org_nativescript_canvas_NSCWebGLRenderingContext::{
    nativeTexImage2D, nativeTexSubImage2D,
};
use crate::utils::{
    nativeInitContextWithCustomSurface, nativeInitContextWithCustomSurfaceNormal,
    nativeResizeCustomSurface, nativeResizeCustomSurfaceNormal,
};
use crate::utils::gl::st::{SURFACE_TEXTURE, SurfaceTexture};
use crate::utils::gl::texture_render::nativeDrawFrame;

mod jni_compat;
pub mod utils;

pub static JVM: OnceLock<JavaVM> = OnceLock::new();

pub static API_LEVEL: OnceLock<i32> = OnceLock::new();

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

            #[cfg(target_os = "android")]
            canvas_c::API_LEVEL.get_or_init(|| ret);

            let canvas_class = env.find_class(NSC_CANVAS_CLASS).unwrap();


            let mut canvas_method_names = vec![
                "nativeInitWebGL",
                "nativeInitWebGLNoSurface",
                "nativeCreate2DContext",
                "nativeUpdateWebGLSurface",
                "nativeUpdate2DSurface",
                "nativeUpdate2DSurfaceNoSurface",
                "nativeUpdateWebGLNoSurface",
                "nativeReleaseWebGL",
                "nativeMakeWebGLCurrent",
                "nativeWriteCurrentWebGLContextToBitmap",
                "nativeInitContextWithCustomSurface",
                "nativeResizeCustomSurface",
                "nativeCustomWithBitmapFlush",
                "nativeContext2DTest",
                "nativeContext2DPathTest",
                "nativeContext2DRender",
                "nativeWebGLC2DRender",
                "nativeInitWebGPU",
                "nativeResizeWebGPU",
            ];

          //  #[cfg(feature = "vulkan")] {
                canvas_method_names.push("nativeCreate2dContextVulkan");
          //  }

            let canvas_signatures = if ret >= ANDROID_O {
                let mut ret = vec![
                    "(Landroid/view/Surface;ZZZZIZZZZZI)J",
                    "(IIZZZZIZZZZZI)J",
                    "(Landroid/view/Surface;ZFIFI)J",
                    "(Landroid/view/Surface;J)V",
                    "(Landroid/view/Surface;J)V",
                    "(IIJ)V",
                    "(IIJ)V",
                    "(J)V",
                    "(J)Z",
                    "(JLandroid/graphics/Bitmap;)V",
                    "(FFFZIFI)J",
                    "(JFFFZI)V",
                    "(JLandroid/graphics/Bitmap;)V",
                    "(J)V",
                    "(J)V",
                    "(J)V",
                    "(JJII)V",
                    "(JLandroid/view/Surface;II)J",
                    "(JLandroid/view/Surface;II)V"
                ];

           //     #[cfg(feature = "vulkan")]{
                    ret.push("(IILandroid/view/Surface;ZFIFI)J");
               // }

                ret
            } else {
                let mut ret = vec![
                    "!(Landroid/view/Surface;ZZZZIZZZZZI)J",
                    "!(IIZZZZIZZZZZI)J",
                    "!(Landroid/view/Surface;ZFIFI)J",
                    "!(Landroid/view/Surface;J)V",
                    "!(Landroid/view/Surface;J)V",
                    "!(IIJ)V",
                    "!(IIJ)V",
                    "!(J)V",
                    "!(J)Z",
                    "!(JLandroid/graphics/Bitmap;)V",
                    "!(FFFZIFI)J",
                    "!(JFFFZI)V",
                    "!(JLandroid/graphics/Bitmap;)V",
                    "!(J)V",
                    "!(J)V",
                    "!(J)V",
                    "!(JJII)V",
                    "!(JLandroid/view/Surface;II)J",
                    "!(JLandroid/view/Surface;II)V"
                ];

             //  #[cfg(feature = "vulkan")]{
                   ret.push("!(IILandroid/view/Surface;ZFIFI)J");
             //  }
                ret
            };



            let mut canvas_methods = if ret >= ANDROID_O {
                vec![
                    nativeInitWebGL as *mut c_void,
                    nativeInitWebGLNoSurface as *mut c_void,
                    nativeCreate2DContext as *mut c_void,
                    nativeUpdateWebGLSurface as *mut c_void,
                    nativeUpdate2DSurface as *mut c_void,
                    nativeUpdate2DSurfaceNoSurface as *mut c_void,
                    nativeUpdateGLNoSurface as *mut c_void,
                    nativeReleaseWebGL as *mut c_void,
                    nativeMakeWebGLCurrent as *mut c_void,
                    nativeWriteCurrentWebGLContextToBitmap as *mut c_void,
                    nativeInitContextWithCustomSurface as *mut c_void,
                    nativeResizeCustomSurface as *mut c_void,
                    nativeCustomWithBitmapFlush as *mut c_void,
                    nativeContext2DTest as *mut c_void,
                    nativeContext2DPathTest as *mut c_void,
                    nativeContext2DRender as *mut c_void,
                    nativeWebGLC2DRender as *mut c_void,
                    nativeInitWebGPU as *mut c_void,
                    nativeResizeWebGPU as *mut c_void,
                ]
            } else {
                vec![
                    nativeInitWebGL as *mut c_void,
                    nativeInitWebGLNoSurface as *mut c_void,
                    nativeCreate2DContext as *mut c_void,
                    nativeUpdateWebGLSurface as *mut c_void,
                    nativeUpdate2DSurface as *mut c_void,
                    nativeUpdate2DSurfaceNoSurfaceNormal as *mut c_void,
                    nativeUpdateWebGLNoSurfaceNormal as *mut c_void,
                    nativeReleaseWebGLNormal as *mut c_void,
                    nativeMakeWebGLCurrentNormal as *mut c_void,
                    nativeWriteCurrentWebGLContextToBitmap as *mut c_void,
                    nativeInitContextWithCustomSurfaceNormal as *mut c_void,
                    nativeResizeCustomSurfaceNormal as *mut c_void,
                    nativeCustomWithBitmapFlush as *mut c_void,
                    nativeContext2DTestNormal as *mut c_void,
                    nativeContext2DPathTestNormal as *mut c_void,
                    nativeContext2DRender as *mut c_void,
                    nativeWebGLC2DRender as *mut c_void,
                    nativeInitWebGPU as *mut c_void,
                    nativeResizeWebGPU as *mut c_void,
                ]
            };

          //  #[cfg(feature = "vulkan")] {
                canvas_methods.push(nativeCreate2dContextVulkan as *mut c_void);
         //   }

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
                "nativeScale"
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
                    "(JFF)V",
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
                    "!(JFF)V",
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
                nativeScale as *mut c_void,
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
                "nativeLoadFromUrl",
                "nativeLoadFromBytes",
                "nativeLoadFromBuffer",
            ];

            let image_asset_signatures = if ret >= ANDROID_O {
                [
                    "()J",
                    "(J)V",
                    "(JLandroid/graphics/Bitmap;)Z",
                    "(J[I)V",
                    "(JLjava/lang/String;)Z",
                    "(J)Ljava/lang/String;",
                    "(JLjava/lang/String;)Z",
                    "(J[B)Z",
                    "(JLjava/nio/ByteBuffer;)Z",
                ]
            } else {
                [
                    "!()J",
                    "!(J)V",
                    "!(JLandroid/graphics/Bitmap;)Z",
                    "!(J[I)V",
                    "!(JLjava/lang/String;)Z",
                    "!(J)Ljava/lang/String;",
                    "!(JLjava/lang/String;)Z",
                    "!(J[B)Z",
                    "!(JLjava/nio/ByteBuffer;)Z",
                ]
            };

            let image_asset_methods = [
                nativeCreateImageAsset as *mut c_void,
                nativeDestroyImageAsset as *mut c_void,
                nativeLoadFromBitmap as *mut c_void,
                nativeGetDimensions as *mut c_void,
                nativeLoadFromPath as *mut c_void,
                nativeGetError as *mut c_void,
                nativeLoadFromUrl as *mut c_void,
                nativeLoadFromBytes as *mut c_void,
                nativeLoadFromBuffer as *mut c_void,
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



            let image_bitmap_class = env
                .find_class("org/nativescript/canvas/NSCImageBitmap")
                .unwrap();

            let image_bitmap_method_names = [
                "nativeLoadFromBytes",
                "nativeLoadFromBuffer",
                "nativeLoadFromBytesOptions",
                "nativeLoadFromBufferOptions",
                "nativeLoadFromBytesRectOptions",
                "nativeLoadFromBufferRectOptions"
            ];

            let image_bitmap_signatures = if ret >= ANDROID_O {
                [
                    "(J[B)Z",
                    "(JLjava/nio/ByteBuffer;)Z",
                    "(J[BZIIIFF)Z",
                    "(JLjava/nio/ByteBuffer;ZIIIFF)Z",
                    "(J[BFFFFZIIIFF)Z",
                    "(JLjava/nio/ByteBuffer;FFFFZIIIFF)Z",
                ]
            } else {
                [
                    "!(J[B)Z",
                    "!(JLjava/nio/ByteBuffer;)Z",
                    "(J[BZIIIFF)Z",
                    "(JLjava/nio/ByteBuffer;ZIIIFF)Z",
                    "(J[BFFFFZIIIFF)Z",
                    "(JLjava/nio/ByteBuffer;FFFFZIIIFF)Z",
                ]
            };

            let image_bitmap_methods = [
                nativeLoadBitmapFromBytes as *mut c_void,
                nativeLoadBitmapFromBuffer as *mut c_void,
                nativeLoadBitmapFromBytesOptions as *mut c_void,
                nativeLoadBitmapFromBufferOptions as *mut c_void,
                nativeLoadBitmapFromBytesRectOptions as *mut c_void,
                nativeLoadBitmapFromBufferRectOptions as *mut c_void
            ];

            let image_bitmap_native_methods: Vec<NativeMethod> = izip!(
                image_bitmap_method_names,
                image_bitmap_signatures,
                image_bitmap_methods
            )
                .map(|(name, signature, method)| NativeMethod {
                    name: name.into(),
                    sig: signature.into(),
                    fn_ptr: method,
                })
                .collect();

            let _ = env
                .register_native_methods(&image_bitmap_class, image_bitmap_native_methods.as_slice());

            let webgl_rendering_class = env
                .find_class("org/nativescript/canvas/NSCWebGLRenderingContext")
                .unwrap();

            let webgl_rendering_method_names = ["nativeTexImage2D", "nativeTexSubImage2D"];

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

            let _ = env.register_native_methods(
                &webgl_rendering_class,
                webgl_rendering_native_methods.as_slice(),
            );

            ret
        });
    }

    SURFACE_TEXTURE.get_or_init(|| SurfaceTexture::new());

    JVM.get_or_init(|| vm);

    log::info!(target: "JS","Canvas library loaded");

    jni::sys::JNI_VERSION_1_6
}
