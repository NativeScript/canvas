//! Wrapped native objects.
//!
//! Every object the binding hands to JS is a `Wrapped<T>` attached with `napi_wrap`. The header
//! (`magic` + `kind`) mirrors the V8 bindings' `ObjectWrapperImpl::type_`: one `napi_unwrap` and a
//! header read tell a method what it was passed (`drawImage(source)`, `fillStyle = pattern`, …)
//! without a chain of trial conversions. It needs no Node-API type tags, so it also works on
//! engines that lack them.

use std::ffi::c_void;
use std::ptr;

use napi::sys;

/// Mirrors `NativeType.h` in the V8 bindings.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum NativeType {
    None,
    CanvasGradient,
    CanvasPattern,
    ImageData,
    ImageAsset,
    CanvasRenderingContext2D,
    WebGLRenderingContextBase,
    Path2D,
    Matrix,
    ImageBitmap,
    TextMetrics,
    TextEncoder,
    TextDecoder,

    WebGLQuery,
    WebGLProgram,
    WebGLShader,
    WebGLBuffer,
    WebGLFramebuffer,
    WebGLRenderbuffer,
    WebGLTexture,
    WebGLActiveInfo,
    OES_fbo_render_mipmap,
    EXT_blend_minmax,
    EXT_color_buffer_half_float,
    EXT_disjoint_timer_query,
    EXT_sRGB,
    EXT_shader_texture_lod,
    EXT_texture_filter_anisotropic,
    OES_element_index_uint,
    OES_standard_derivatives,
    OES_texture_float,
    OES_texture_float_linear,
    OES_texture_half_float_linear,
    OES_texture_half_float,
    WEBGL_color_buffer_float,
    OES_vertex_array_object,
    WebGLVertexArrayObject,
    WEBGL_compressed_texture_atc,
    WEBGL_compressed_texture_etc1,
    WEBGL_compressed_texture_s3tc,
    WEBGL_compressed_texture_s3tc_srgb,
    WEBGL_compressed_texture_etc,
    WEBGL_compressed_texture_pvrtc,
    WEBGL_lose_context,
    ANGLE_instanced_arrays,
    WEBGL_depth_texture,
    WEBGL_draw_buffers,
    WebGLShaderPrecisionFormat,
    WebGLUniformLocation,
    WebGLSampler,
    WebGLTransformFeedback,
    WebGLSync,

    GPUAdapter,
    GPUSupportedLimits,
    GPUDevice,
    GPUQueue,
    GPUBuffer,
    GPUInstance,
    GPUCanvasContext,
    GPUTexture,
    GPUAdapterInfo,
    GPUCommandEncoder,
    GPUComputePass,
    GPUQuerySet,
    GPUShaderModule,
    GPUPipelineLayout,
    GPURenderPipeline,
    GPUBindGroupLayout,
    GPUTextureView,
    GPURenderPassEncoder,
    GPUCommandBuffer,
    GPUBindGroup,
    GPUComputePipeline,
    GPUSampler,
    GPURenderBundleEncoder,
    GPURenderBundle,
    GPUCompilationInfo,
    GPUCompilationMessage,
    GPUExternalTexture,

    /// The desktop host view (`CanvasModule.NSCCanvas`).
    CanvasHost,
}

/// "NSCV": marks memory this binding wrapped, so objects wrapped by other addons are rejected.
const MAGIC: u32 = 0x4E53_4356;

/// A type that can live inside a JS object created by this binding.
pub trait Native: Sized + 'static {
    const KIND: NativeType;
}

#[repr(C)]
struct Header {
    magic: u32,
    kind: NativeType,
}

#[repr(C)]
pub struct Wrapped<T> {
    header: Header,
    pub value: T,
}

impl<T: Native> Wrapped<T> {
    pub fn boxed(value: T) -> *mut Wrapped<T> {
        Box::into_raw(Box::new(Wrapped {
            header: Header {
                magic: MAGIC,
                kind: T::KIND,
            },
            value,
        }))
    }
}

unsafe extern "C" fn finalize<T: Native>(_env: sys::napi_env, data: *mut c_void, _hint: *mut c_void) {
    drop(Box::from_raw(data as *mut Wrapped<T>));
}

/// Attaches an already boxed value to `object`; ownership moves to the JS object.
pub unsafe fn attach<T: Native>(env: sys::napi_env, object: sys::napi_value, boxed: *mut Wrapped<T>) -> bool {
    let status = sys::napi_wrap(
        env,
        object,
        boxed as *mut c_void,
        Some(finalize::<T>),
        ptr::null_mut(),
        ptr::null_mut(),
    );
    if status != sys::Status::napi_ok {
        drop(Box::from_raw(boxed));
        return false;
    }
    true
}

/// Moves `value` into `object`.
pub unsafe fn wrap<T: Native>(env: sys::napi_env, object: sys::napi_value, value: T) -> bool {
    attach(env, object, Wrapped::boxed(value))
}

/// What `value` wraps, if this binding wrapped it. Primitives and foreign objects give `None`.
#[inline]
pub unsafe fn kind_of(env: sys::napi_env, value: sys::napi_value) -> Option<(NativeType, *mut c_void)> {
    if value.is_null() {
        return None;
    }
    let mut raw: *mut c_void = ptr::null_mut();
    if sys::napi_unwrap(env, value, &mut raw) != sys::Status::napi_ok || raw.is_null() {
        return None;
    }
    let header = &*(raw as *const Header);
    if header.magic != MAGIC {
        return None;
    }
    Some((header.kind, raw))
}

/// The `T` inside `value`, if it is one.
#[inline]
pub unsafe fn unwrap<'a, T: Native>(env: sys::napi_env, value: sys::napi_value) -> Option<&'a mut T> {
    match kind_of(env, value) {
        Some((kind, raw)) if kind == T::KIND => Some(cast::<T>(raw)),
        _ => None,
    }
}

/// Reinterprets a pointer from [`kind_of`] whose kind was checked to be `T::KIND`.
#[inline]
pub unsafe fn cast<'a, T: Native>(raw: *mut c_void) -> &'a mut T {
    &mut (*(raw as *mut Wrapped<T>)).value
}
