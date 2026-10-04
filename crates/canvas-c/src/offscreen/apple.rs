//! A view's layers, used off the main thread only for drawables, drawable size and presents.

use std::ffi::{c_void, CStr};

use objc2::msg_send;
use objc2::rc::Retained;
use objc2::runtime::{AnyClass, AnyObject};

use super::WebGLAttributes;
use crate::WebGLState;

#[derive(Clone)]
pub(super) struct ObjRef(Retained<AnyObject>);

// Retain and release are thread-safe.
unsafe impl Send for ObjRef {}
unsafe impl Sync for ObjRef {}

impl ObjRef {
    unsafe fn retain(value: *mut c_void) -> Option<Self> {
        Retained::retain(value as *mut AnyObject).map(Self)
    }

    pub(super) fn ptr(&self) -> *mut c_void {
        Retained::as_ptr(&self.0) as *mut c_void
    }
}

#[derive(Clone)]
pub(super) struct MetalLayer {
    pub(super) layer: ObjRef,
    pub(super) device: ObjRef,
    pub(super) queue: ObjRef,
    pub(super) samples: usize,
}

pub(super) struct View {
    pub(super) metal: Option<MetalLayer>,
    /// None on visionOS.
    pub(super) gl_layer: Option<ObjRef>,
}

impl View {
    pub(super) fn new(
        metal_layer: *mut c_void,
        device: *mut c_void,
        queue: *mut c_void,
        samples: usize,
        gl_layer: *mut c_void,
    ) -> Self {
        let metal = unsafe {
            match (ObjRef::retain(metal_layer), ObjRef::retain(device), ObjRef::retain(queue)) {
                (Some(layer), Some(device), Some(queue)) => Some(MetalLayer {
                    layer,
                    device,
                    queue,
                    samples: samples.max(1),
                }),
                _ => None,
            }
        };
        Self {
            metal,
            gl_layer: unsafe { ObjRef::retain(gl_layer) },
        }
    }
}

/// OpenGLES's `NSString * const` keys, looked up rather than hard-coded.
unsafe fn eagl_constant(name: &CStr) -> *const AnyObject {
    let symbol = libc::dlsym(libc::RTLD_DEFAULT, name.as_ptr());
    if symbol.is_null() {
        return std::ptr::null();
    }
    *(symbol as *const *const AnyObject)
}

/// Set before storage: retained backing is preserveDrawingBuffer.
fn set_drawable_properties(layer: &ObjRef, retained: bool) {
    objc2::rc::autoreleasepool(|_| unsafe {
        let retained_key = eagl_constant(c"kEAGLDrawablePropertyRetainedBacking");
        let format_key = eagl_constant(c"kEAGLDrawablePropertyColorFormat");
        let rgba8 = eagl_constant(c"kEAGLColorFormatRGBA8");
        let (Some(number), Some(dictionary)) = (AnyClass::get(c"NSNumber"), AnyClass::get(c"NSDictionary")) else {
            return;
        };
        if retained_key.is_null() || format_key.is_null() || rgba8.is_null() {
            return;
        }
        let retained: *mut AnyObject = msg_send![number, numberWithBool: retained];
        let keys = [retained_key, format_key];
        let objects = [retained as *const AnyObject, rgba8];
        let properties: *mut AnyObject = msg_send![
            dictionary,
            dictionaryWithObjects: objects.as_ptr(),
            forKeys: keys.as_ptr(),
            count: 2usize
        ];
        let _: () = msg_send![&*layer.0, setDrawableProperties: properties];
    });
}

pub(super) fn create_webgl(layer: ObjRef, attributes: &WebGLAttributes) -> *mut WebGLState {
    if attributes.version == 2 && !canvas_core::gpu::gl::GLContext::has_gl2support() {
        return std::ptr::null_mut();
    }
    let Ok(version) = canvas_webgl::prelude::WebGLVersion::try_from(attributes.version) else {
        return std::ptr::null_mut();
    };
    let a = *attributes;
    WebGLState::create_threaded(move || {
        set_drawable_properties(&layer, a.preserve_drawing_buffer);
        WebGLState::new_with_view(
            layer.ptr(),
            version,
            a.alpha,
            a.antialias,
            a.depth,
            a.fail_if_major_performance_caveat,
            a.power_preference(),
            a.premultiplied_alpha,
            a.preserve_drawing_buffer,
            a.stencil,
            a.desynchronized,
            a.xr_compatible,
            false,
        )
    })
}
