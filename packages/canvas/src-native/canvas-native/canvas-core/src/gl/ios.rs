use std::cell::RefCell;
use std::ffi::{c_int, c_long, c_void};
use std::fmt::{Debug, Formatter};
use std::ptr::NonNull;
use std::rc::Rc;
use std::sync::Once;

use core_foundation::base::TCFType;
use core_foundation::bundle::{CFBundleGetBundleWithIdentifier, CFBundleGetFunctionPointerForName};
use core_foundation::string::CFString;
use icrate::objc2::ffi::{objc_class, BOOL};
use icrate::objc2::rc::{Allocated, Owned};
use icrate::objc2::{
    class, msg_send, msg_send_id, rc::Id, rc::Shared, runtime::Object, Encode, Encoding,
};
use icrate::Foundation::NSInteger;

use crate::context_attributes::ContextAttributes;

#[derive(Debug, Default)]
pub struct GLContextInner {
    context: Option<EAGLContext>,
    view: Option<GLKView>,
}

unsafe impl Sync for GLContextInner {}

unsafe impl Send for GLContextInner {}

#[derive(Debug, Default)]
pub struct GLContext {
    inner: Rc<RefCell<GLContextInner>>,
}

impl GLContext {
    // pointer has to
    pub fn as_raw_inner(&self) -> *const RefCell<GLContextInner> {
        Rc::into_raw(Rc::clone(&self.inner))
    }

    pub fn from_raw_inner(raw: *const RefCell<GLContextInner>) -> Self {
        Self {
            inner: unsafe { Rc::from_raw(raw) },
        }
    }
}

impl Clone for GLContext {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

pub enum EAGLRenderingAPI {
    GLES1 = 1,
    GLES2 = 2,
    GLES3 = 3,
}

unsafe impl Encode for EAGLRenderingAPI {
    const ENCODING: Encoding = Encoding::Long;
}
#[derive(Clone, Debug)]
pub(crate) struct EAGLContext(Id<Object, Shared>);

impl EAGLContext {
    pub fn new_with_api(api: EAGLRenderingAPI) -> Option<Self> {
        unsafe {
            let cls = class!(EAGLContext);
            let context = msg_send_id![cls, alloc];
            let mut context: Option<Id<Object, Shared>> = msg_send_id![context, initWithAPI: api];

            context.map(EAGLContext)
        }
    }

    pub fn new_with_api_sharegroup(
        api: EAGLRenderingAPI,
        sharegroup: &EAGLContext,
    ) -> Option<Self> {
        unsafe {
            let cls = class!(EAGLContext);
            let context = msg_send_id![cls, alloc];
            let context: Option<Id<Object, Shared>> =
                msg_send_id![context, initWithAPI: api, sharegroup: &*sharegroup.0];
            context.map(EAGLContext)
        }
    }

    pub fn set_current_context(context: Option<&EAGLContext>) -> bool {
        let cls = class!(EAGLContext);
        return match context {
            Some(ctx) => unsafe {
                let instance: BOOL = msg_send![cls, setCurrentContext: &*ctx.0];

                instance
            },
            None => unsafe {
                let nil: *mut Object = std::ptr::null_mut();
                let instance: BOOL = msg_send![cls, setCurrentContext: nil];
                instance
            },
        };
    }

    pub fn get_current_context() -> Option<Self> {
        unsafe {
            let cls = class!(EAGLContext);
            let context: Option<Id<Object, Shared>> = msg_send_id![cls, currentContext];

            context.map(EAGLContext)
        }
    }

    pub fn remove_if_current(&self) -> bool {
        unsafe {
            let cls = class!(EAGLContext);
            let current: Option<Id<Object, Shared>> = msg_send_id![cls, currentContext];

            match current {
                Some(current) => {
                    let is_equal: bool = unsafe { msg_send![&current, isEqual: &*self.0] };
                    if is_equal {
                        let nil: *mut Object = std::ptr::null_mut();
                        return msg_send![cls, setCurrentContext: nil];
                    }
                    false
                }
                None => false,
            }
        }
    }
}

#[repr(i32)]
pub enum GLKViewDrawableColorFormat {
    RGBA8888 = 0,
    RGB565 = 1,
    SRGBA8888 = 2,
}

unsafe impl Encode for GLKViewDrawableColorFormat {
    const ENCODING: Encoding = Encoding::ULong;
}

impl TryFrom<i32> for GLKViewDrawableColorFormat {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GLKViewDrawableColorFormat::RGBA8888),
            1 => Ok(GLKViewDrawableColorFormat::RGB565),
            2 => Ok(GLKViewDrawableColorFormat::SRGBA8888),
            _ => Err("Invalid GLKViewDrawableColorFormat"),
        }
    }
}

#[repr(i32)]
pub enum GLKViewDrawableDepthFormat {
    DepthFormatNone = 0,
    DepthFormat16 = 1,
    DepthFormat24 = 2,
}

unsafe impl Encode for GLKViewDrawableDepthFormat {
    const ENCODING: Encoding = Encoding::ULong;
}

impl TryFrom<i32> for GLKViewDrawableDepthFormat {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GLKViewDrawableDepthFormat::DepthFormatNone),
            1 => Ok(GLKViewDrawableDepthFormat::DepthFormat16),
            2 => Ok(GLKViewDrawableDepthFormat::DepthFormat24),
            _ => Err("Invalid GLKViewDrawableDepthFormat"),
        }
    }
}

#[repr(i32)]
pub enum GLKViewDrawableStencilFormat {
    StencilFormatNone = 0,
    StencilFormat8 = 1,
}

unsafe impl Encode for GLKViewDrawableStencilFormat {
    const ENCODING: Encoding = Encoding::ULong;
}

impl TryFrom<i32> for GLKViewDrawableStencilFormat {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GLKViewDrawableStencilFormat::StencilFormatNone),
            1 => Ok(GLKViewDrawableStencilFormat::StencilFormat8),
            _ => Err("Invalid GLKViewDrawableStencilFormat"),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct GLKView(Id<Object, Shared>);

impl GLKView {
    pub fn new() -> Self {
        let cls = class!(GLKView);
        let instance = unsafe { msg_send_id![cls, alloc] };
        GLKView(unsafe {
            msg_send_id![
                instance,
                initWithFrame: icrate::Foundation::CGRect::default()
            ]
        })
    }

    pub fn new_with_frame(x: f32, y: f32, width: f32, height: f32) -> Self {
        unsafe {
            let cls = class!(GLKView);
            let instance = msg_send_id![cls, alloc];
            let point = icrate::Foundation::CGPoint::new(x as f64, y as f64);
            let size = icrate::Foundation::CGSize::new(width as f64, height as f64);
            let frame = icrate::Foundation::CGRect::new(point, size);
            GLKView(msg_send_id![instance, initWithFrame: frame])
        }
    }

    pub fn display(&self) {
        let _: () = unsafe { msg_send![&self.0, display] };
    }

    pub fn drawable_width(&self) -> NSInteger {
        return unsafe { msg_send![&self.0, drawableWidth] };
    }

    pub fn drawable_height(&self) -> NSInteger {
        return unsafe { msg_send![&self.0, drawableHeight] };
    }

    pub fn set_drawable_color_format(&self, format: GLKViewDrawableColorFormat) {
        let _: () = unsafe { msg_send![&self.0, drawableColorFormat: format] };
    }

    pub fn get_drawable_color_format(&self) -> GLKViewDrawableColorFormat {
        let format: i32 = unsafe { msg_send![&self.0, drawableColorFormat] };
        GLKViewDrawableColorFormat::try_from(format).unwrap()
    }

    pub fn set_drawable_depth_format(&self, format: GLKViewDrawableDepthFormat) {
        let _: () = unsafe { msg_send![&self.0, drawableDepthFormat: format] };
    }

    pub fn get_drawable_depth_format(&self) -> GLKViewDrawableDepthFormat {
        let depth: i32 = unsafe { msg_send![&self.0, drawableDepthFormat] };
        GLKViewDrawableDepthFormat::try_from(depth).unwrap()
    }

    pub fn set_drawable_stencil_format(&self, format: GLKViewDrawableStencilFormat) {
        let _: () = unsafe { msg_send![&self.0, drawableStencilFormat: format] };
    }

    pub fn get_drawable_stencil_format(&self) -> GLKViewDrawableStencilFormat {
        let stencil: i32 = unsafe { msg_send![&self.0, drawableStencilFormat] };
        GLKViewDrawableStencilFormat::try_from(stencil).unwrap()
    }

    pub fn set_context(&self, context: Option<&EAGLContext>) {
        match context {
            Some(context) => {
                let _: () = unsafe { msg_send![&self.0, context: &*context.0] };
            }
            None => {
                let nil: *mut Object = std::ptr::null_mut();
                let _: () = unsafe { msg_send![&self.0, context: nil] };
            }
        }
    }

    pub fn get_context(&self) -> Option<EAGLContext> {
        let context: Option<Id<Object, Shared>> = unsafe { msg_send_id![&self.0, context] };
        context.map(EAGLContext)
    }

    fn get_proc_address(&self, addr: &str) -> *const c_void {
        let symbol_name = CFString::new(addr);
        let framework_name = CFString::new("com.apple.opengl");
        unsafe {
            let framework = CFBundleGetBundleWithIdentifier(framework_name.as_concrete_TypeRef());
            CFBundleGetFunctionPointerForName(framework, symbol_name.as_concrete_TypeRef()).cast()
        }
    }
}

//#[cfg(target_os = "ios")]
impl GLContext {
    pub fn set_surface(&mut self, view: NonNull<c_void>) -> bool {
        let glview = unsafe { Id::<Object, Shared>::new(view.as_ptr() as _) };
        match glview {
            None => false,
            Some(glview) => {
                let id = glview.clone();
                let glview = GLKView(id);
                self.inner.borrow_mut().view = Some(glview);
                true
            }
        }
    }

    pub fn create_window_context(
        context_attrs: &mut ContextAttributes,
        view: NonNull<c_void>,
    ) -> Option<GLContext> {
        let glview = unsafe { Id::<Object, Shared>::new(view.as_ptr() as _) };
        match glview {
            None => None,
            Some(glview) => {
                let id = glview.clone();
                let glview = GLKView(id.into());
                GLContext::create_window_context_with_gl_view(context_attrs, glview)
            }
        }
    }

    pub(crate) fn create_window_context_with_gl_view(
        context_attrs: &mut ContextAttributes,
        view: GLKView,
    ) -> Option<GLContext> {
        gl_bindings::load_with(|symbol| view.get_proc_address(symbol).cast());

        let api = if context_attrs.get_is_canvas() {
            EAGLRenderingAPI::GLES2
        } else {
            EAGLRenderingAPI::GLES3
        };

        let context = EAGLContext::new_with_api(api);

        if context.is_none() {
            return None;
        }

        view.set_context(context.as_ref());

        let inner = GLContextInner {
            context,
            view: Some(view),
        };
        Some(GLContext {
            inner: Rc::new(RefCell::new(inner)),
        })
    }

    pub fn create_offscreen_context(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
    ) -> Option<GLContext> {
        let view = GLKView::new_with_frame(0., 0., width as f32, height as f32);

        GLContext::create_window_context_with_gl_view(context_attrs, view)
    }

    fn has_extension(extensions: &str, name: &str) -> bool {
        !extensions.split(' ').into_iter().any(|s| s == name)
    }

    pub fn has_gl2support() -> bool {
        true
    }

    #[inline(always)]
    pub fn set_vsync(&self, sync: bool) -> bool {
        true
    }

    #[inline(always)]
    pub fn make_current(&self) -> bool {
        let inner = self.inner.borrow();
        EAGLContext::set_current_context(inner.context.as_ref())
    }

    #[inline(always)]
    pub fn remove_if_current(&self) {
        let inner = self.inner.borrow();
        if let Some(context) = inner.context.as_ref() {
            context.remove_if_current();
        }
    }

    #[inline(always)]
    pub fn swap_buffers(&self) -> bool {
        let inner = self.inner.borrow();
        if let Some(view) = inner.view.as_ref() {
            view.display();
            return true;
        }
        false
    }

    #[inline(always)]
    pub fn get_surface_width(&self) -> i32 {
        self.inner
            .borrow()
            .view
            .as_ref()
            .map(|v| v.drawable_width().try_into().unwrap_or_default())
            .unwrap()
    }

    #[inline(always)]
    pub fn get_surface_height(&self) -> i32 {
        self.inner
            .borrow()
            .view
            .as_ref()
            .map(|v| v.drawable_height().try_into().unwrap_or_default())
            .unwrap()
    }
}
