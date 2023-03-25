use std::cell::RefCell;
use std::ffi::{c_int, c_long, c_void};
use std::fmt::{Debug, Formatter};
use std::ptr::NonNull;
use std::rc::Rc;
use std::sync::Once;

use cocoa::foundation::NSInteger;
use core_foundation::base::TCFType;
use core_foundation::bundle::{CFBundleGetBundleWithIdentifier, CFBundleGetFunctionPointerForName};
use core_foundation::string::CFString;
use objc::declare::ClassDecl;
use objc::rc::StrongPtr;
use objc::runtime::{Class, Object, Sel};
use objc::Message;

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

#[derive(Clone)]
pub(crate) struct EAGLContext(StrongPtr);

impl Debug for EAGLContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EAGLContext")
            .field("context", &((*self.0) as i64))
            .finish()
    }
}

impl EAGLContext {
    pub fn new_with_api(api: EAGLRenderingAPI) -> Option<Self> {
        unsafe {
            let cls = objc::class!(EAGLContext);
            let context: *mut Object = objc::msg_send![cls, alloc];
            let mut context: *mut Object = objc::msg_send![context, initWithAPI: api];
            if context.is_null() {
                return None;
            }

            Some(Self(StrongPtr::new(context)))
        }
    }

    pub fn new_with_api_sharegroup(
        api: EAGLRenderingAPI,
        sharegroup: &EAGLContext,
    ) -> Option<Self> {
        unsafe {
            let cls = objc::class!(EAGLContext);
            let context: *mut Object = objc::msg_send![cls, alloc];
            let context: *mut Object =
                objc::msg_send![context, initWithAPI: api sharegroup: *sharegroup.0];
            if context.is_null() {
                return None;
            }
            Some(Self(StrongPtr::new(context)))
        }
    }

    pub fn set_current_context(context: Option<&EAGLContext>) -> bool {
        let cls = objc::class!(EAGLContext);
        return match context {
            Some(ctx) => unsafe {
                let instance: *mut Object = objc::msg_send![cls, alloc];
                objc::msg_send![instance, setCurrentContext: *ctx.0]
            },
            None => unsafe {
                let instance: *mut Object = objc::msg_send![cls, alloc];
                let nil: *mut Object = std::ptr::null_mut();
                objc::msg_send![instance, setCurrentContext: nil]
            },
        };
    }

    pub fn get_current_context() -> Option<Self> {
        unsafe {
            let cls = objc::class!(EAGLContext);
            let instance: *mut Object = objc::msg_send![cls, alloc];
            let context: *mut Object = objc::msg_send![instance, currentContext];
            if context.is_null() {
                return None;
            }
            Some(Self(StrongPtr::new(context)))
        }
    }

    pub fn remove_if_current(&self) -> bool {
        unsafe {
            let cls = objc::class!(EAGLContext);
            let context: *mut Object = objc::msg_send![cls, alloc];
            let current: *mut Object = objc::msg_send![context, currentContext];
            if current.is_null() {
                return false;
            }

            if current == *self.0 {
                let nil: *mut Object = std::ptr::null_mut();
                return objc::msg_send![context, setCurrentContext: nil];
            }

            false
        }
    }
}

#[repr(i32)]
pub enum GLKViewDrawableColorFormat {
    RGBA8888 = 0,
    RGB565 = 1,
    SRGBA8888 = 2,
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

#[derive(Clone)]
pub(crate) struct GLKView(StrongPtr);

impl Debug for GLKView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GLKView")
            .field("view", &((*self.0) as i64))
            .finish()
    }
}

impl GLKView {
    pub fn new() -> Self {
        unsafe {
            let cls = objc::class!(GLKView);
            let instance: *mut Object = objc::msg_send![cls, alloc];
            GLKView(StrongPtr::new(objc::msg_send![
                instance,
                initWithFrame: core_graphics_types::geometry::CG_ZERO_RECT
            ]))
        }
    }

    pub fn new_with_frame(x: f32, y: f32, width: f32, height: f32) -> Self {
        unsafe {
            let cls = objc::class!(GLKView);
            let instance: *mut Object = objc::msg_send![cls, alloc];
            let point = core_graphics_types::geometry::CGPoint::new(x as f64, y as f64);
            let size = core_graphics_types::geometry::CGSize::new(width as f64, height as f64);
            let frame = core_graphics_types::geometry::CGRect::new(&point, &size);
            GLKView(StrongPtr::new(objc::msg_send![
                instance,
                initWithFrame: frame
            ]))
        }
    }

    pub fn display(&self) {
        let _: () = unsafe { objc::msg_send![*self.0, display] };
    }

    pub fn drawable_width(&self) -> NSInteger {
        return unsafe { objc::msg_send![*self.0, drawableWidth] };
    }

    pub fn drawable_height(&self) -> NSInteger {
        return unsafe { objc::msg_send![*self.0, drawableHeight] };
    }

    pub fn set_drawable_color_format(&self, format: GLKViewDrawableColorFormat) {
        let _: () = unsafe { objc::msg_send![*self.0, drawableColorFormat: format] };
    }

    pub fn get_drawable_color_format(&self) -> GLKViewDrawableColorFormat {
        let format: i32 = unsafe { objc::msg_send![*self.0, drawableColorFormat] };
        GLKViewDrawableColorFormat::try_from(format).unwrap()
    }

    pub fn set_drawable_depth_format(&self, format: GLKViewDrawableDepthFormat) {
        let _: () = unsafe { objc::msg_send![*self.0, drawableDepthFormat: format] };
    }

    pub fn get_drawable_depth_format(&self) -> GLKViewDrawableDepthFormat {
        let depth: i32 = unsafe { objc::msg_send![*self.0, drawableDepthFormat] };
        GLKViewDrawableDepthFormat::try_from(depth).unwrap()
    }

    pub fn set_drawable_stencil_format(&self, format: GLKViewDrawableStencilFormat) {
        let _: () = unsafe { objc::msg_send![*self.0, drawableStencilFormat: format] };
    }

    pub fn get_drawable_stencil_format(&self) -> GLKViewDrawableStencilFormat {
        let stencil: i32 = unsafe { objc::msg_send![*self.0, drawableStencilFormat] };
        GLKViewDrawableStencilFormat::try_from(stencil).unwrap()
    }

    pub fn set_context(&self, context: Option<&EAGLContext>) {
        match context {
            Some(context) => {
                let _: () = unsafe { objc::msg_send![*self.0, context: *context.0] };
            }
            None => {
                let nil: *mut Object = std::ptr::null_mut();
                let _: () = unsafe { objc::msg_send![*self.0, context: nil] };
            }
        }
    }

    pub fn get_context(&self) -> Option<EAGLContext> {
        let context: *mut Object = unsafe { objc::msg_send![*self.0, context] };
        if context.is_null() {
            return None;
        }
        Some(EAGLContext(unsafe { StrongPtr::new(context) }))
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

#[cfg(target_os = "ios")]
impl GLContext {
    pub fn set_surface(&mut self, view: NonNull<c_void>) -> bool {
        self.inner.borrow_mut().view = Some(GLKView(unsafe { StrongPtr::new(view.as_ptr() as _) }));
        true
    }

    pub fn create_window_context(
        context_attrs: &mut ContextAttributes,
        view: NonNull<c_void>,
    ) -> Option<GLContext> {
        let glview = GLKView(unsafe { StrongPtr::new(view.as_ptr() as _) });

        GLContext::create_window_context_with_gl_view(context_attrs, glview)
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
