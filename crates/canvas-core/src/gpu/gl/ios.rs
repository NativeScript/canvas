use std::cell::Cell;
use std::ffi::c_void;
use std::fmt::Debug;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;

use objc2::ffi::BOOL;
use objc2::{class, msg_send, msg_send_id, rc::Id, Encode, Encoding};
use objc2_foundation::{NSObject, NSUInteger};

use crate::context_attributes::ContextAttributes;

pub static IS_GL_SYMBOLS_LOADED: OnceLock<bool> = OnceLock::new();

thread_local! {
    /// Mirrors what this thread last bound, so `make_current` can skip the call.
    ///
    /// Only valid while every `setCurrentContext:` is reflected here: bind through
    /// `set_current_context`, and clear the mirror anywhere else a context gets
    /// bound.
    static CURRENT_EAGL_BINDING: Cell<Binding> = const { Cell::new(UNBOUND) };
}

/// (context, epoch).
type Binding = (usize, usize);

const UNBOUND: Binding = (0, 0);

/// Bumped on every context allocation, so a recycled `EAGLContext` address cannot
/// match a stale mirror entry.
static CONTEXT_EPOCH: AtomicUsize = AtomicUsize::new(1);

fn context_epoch() -> usize {
    CONTEXT_EPOCH.load(Ordering::Relaxed)
}

fn invalidate_context_bindings() {
    CONTEXT_EPOCH.fetch_add(1, Ordering::Relaxed);
}

fn binding(context: usize) -> Binding {
    (context, context_epoch())
}

fn binding_is_current(binding: Binding) -> bool {
    binding != UNBOUND && CURRENT_EAGL_BINDING.with(|c| c.get()) == binding
}

fn set_current_binding(binding: Binding) {
    CURRENT_EAGL_BINDING.with(|c| c.set(binding));
}

/// Drops this thread's mirror, so the next `make_current` does a real bind.
fn forget_current_binding() {
    set_current_binding(UNBOUND);
}

#[derive(Debug, Default)]
pub(crate) struct GLContextInner {
    context: Option<EAGLContext>,
    sharegroup: EAGLSharegroup,
    drawable: Drawable,
}

#[derive(Debug, Clone)]
pub struct GLContextRaw {
    context: Option<EAGLContext>,
    sharegroup: EAGLSharegroup,
}

impl GLContextRaw {
    pub fn make_current(&self) -> bool {
        if let Some(context) = self.context.as_ref() {
            return EAGLContext::set_current_context(Some(context));
        }
        false
    }

    pub fn remove_if_current(&self) {
        if let Some(context) = self.context.as_ref() {
            context.remove_if_current();
        }
    }
}

unsafe impl Sync for GLContextInner {}

unsafe impl Send for GLContextInner {}

impl Drop for GLContextInner {
    fn drop(&mut self) {
        let Some(context) = self.context.as_ref() else {
            return;
        };
        if EAGLContext::set_current_context(Some(context)) {
            self.drawable.delete();
            // EAGL keeps the current context alive until another replaces it.
            EAGLContext::set_current_context(None);
        }
    }
}

#[derive(Debug, Default)]
pub struct GLContext(GLContextInner);

pub enum EAGLRenderingAPI {
    GLES1 = 1,
    GLES2 = 2,
    GLES3 = 3,
}

unsafe impl Encode for EAGLRenderingAPI {
    const ENCODING: Encoding = Encoding::ULongLong;
}

#[derive(Clone, Debug)]
pub(crate) struct EAGLSharegroup(Id<NSObject>);

impl EAGLSharegroup {
    pub fn new() -> Self {
        unsafe {
            let cls = class!(EAGLSharegroup);
            let sharegroup = msg_send_id![cls, alloc];
            let sharegroup: Id<NSObject> =
                msg_send_id![sharegroup, initWithAPI: EAGLRenderingAPI::GLES3];

            Self(sharegroup)
        }
    }

    pub fn legacy() -> Self {
        unsafe {
            let cls = class!(EAGLSharegroup);
            let sharegroup = msg_send_id![cls, alloc];
            let sharegroup: Id<NSObject> =
                msg_send_id![sharegroup, initWithAPI: EAGLRenderingAPI::GLES2];

            Self(sharegroup)
        }
    }
}

impl Default for EAGLSharegroup {
    fn default() -> Self {
        EAGLSharegroup::new()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct EAGLContext(Id<NSObject>);

impl EAGLContext {
    pub fn new_with_api(api: EAGLRenderingAPI) -> Option<Self> {
        let context = unsafe {
            let cls = class!(EAGLContext);
            let context = msg_send_id![cls, alloc];
            let context: Option<Id<NSObject>> = msg_send_id![context, initWithAPI: api];
            context.map(EAGLContext)
        };
        invalidate_context_bindings();
        context
    }

    pub fn new_with_api_sharegroup(
        api: EAGLRenderingAPI,
        sharegroup: &EAGLSharegroup,
    ) -> Option<Self> {
        let context = unsafe {
            let cls = class!(EAGLContext);
            let context = msg_send_id![cls, alloc];
            let context: Option<Id<NSObject>> =
                msg_send_id![context, initWithAPI: api, sharegroup: &*sharegroup.0];
            context.map(EAGLContext)
        };
        invalidate_context_bindings();
        context
    }

    fn raw_id(&self) -> usize {
        Id::as_ptr(&self.0) as usize
    }

    /// Binds `context` on the calling thread, skipping the call when it is already
    /// bound. The skip never elides a sharegroup flush, since the context is not
    /// changing.
    pub fn set_current_context(context: Option<&EAGLContext>) -> bool {
        let cls = class!(EAGLContext);
        match context {
            Some(ctx) => {
                let binding = binding(ctx.raw_id());
                if binding_is_current(binding) {
                    return true;
                }
                let bound: bool = unsafe {
                    let instance: BOOL = msg_send![cls, setCurrentContext: &*ctx.0];
                    instance.into()
                };
                set_current_binding(if bound { binding } else { UNBOUND });
                bound
            }
            None => {
                let unbound: bool = unsafe {
                    let nil: *mut NSObject = std::ptr::null_mut();
                    let instance: BOOL = msg_send![cls, setCurrentContext: nil];
                    instance.into()
                };
                if unbound {
                    forget_current_binding();
                }
                unbound
            }
        }
    }

    pub fn get_current_context() -> Option<Self> {
        unsafe {
            let cls = class!(EAGLContext);
            let context: Option<Id<NSObject>> = msg_send_id![cls, currentContext];

            context.map(EAGLContext)
        }
    }

    pub fn remove_if_current(&self) -> bool {
        unsafe {
            let cls = class!(EAGLContext);
            let current: Option<Id<NSObject>> = msg_send_id![cls, currentContext];

            match current {
                Some(current) => {
                    let is_equal: bool = unsafe { msg_send![&current, isEqual: &*self.0] };
                    if is_equal {
                        return Self::set_current_context(None);
                    }
                    false
                }
                None => false,
            }
        }
    }

    pub fn present_renderbuffer(&self) -> bool {
        // GL_RENDERBUFFER
        let result: BOOL = unsafe { msg_send![&self.0, presentRenderbuffer: 0x8d41 as NSUInteger] };
        result.into()
    }

    /// Stores the bound renderbuffer in `layer` (a `CAEAGLLayer`), at the layer's size in pixels.
    pub fn renderbuffer_storage_from_drawable(&self, layer: &NSObject) -> bool {
        let result: BOOL = unsafe {
            msg_send![&self.0, renderbufferStorage: 0x8d41 as NSUInteger, fromDrawable: layer]
        };
        result.into()
    }
}

type DiscardFramebuffer = unsafe extern "C" fn(target: u32, count: i32, attachments: *const u32);

/// `glDiscardFramebufferEXT`, for GLES2 contexts, which lack `glInvalidateFramebuffer`.
fn discard_framebuffer_ext() -> Option<DiscardFramebuffer> {
    static DISCARD: OnceLock<Option<DiscardFramebuffer>> = OnceLock::new();
    *DISCARD.get_or_init(|| {
        let function = super::get_proc_address("glDiscardFramebufferEXT");
        (!function.is_null())
            .then(|| unsafe { std::mem::transmute::<*const c_void, DiscardFramebuffer>(function) })
    })
}

fn bound(binding: u32) -> u32 {
    let mut name = 0;
    unsafe { gl_bindings::GetIntegerv(binding, &mut name) };
    name as u32
}

/// What a context draws to by default (WebGL's null framebuffer, a 2D surface): a framebuffer
/// whose color buffer is stored in a view's `CAEAGLLayer` and presented to it, or a plain one
/// offscreen. Everything here runs with the context current, on whichever thread owns it: none of
/// it needs the main thread.
#[derive(Debug, Default)]
struct Drawable {
    layer: Option<Id<NSObject>>,
    framebuffer: u32,
    color: u32,
    /// Packed depth and stencil, attached as either or both.
    depth_stencil: u32,
    depth: bool,
    stencil: bool,
    /// Depth and stencil outlive a present (preserveDrawingBuffer).
    preserve: bool,
    legacy: bool,
    width: i32,
    height: i32,
}

impl Drawable {
    fn new(layer: Option<Id<NSObject>>, attrs: &ContextAttributes) -> Self {
        let mut drawable = Drawable {
            layer,
            depth: attrs.get_depth(),
            // 2D clips through the stencil.
            stencil: attrs.get_stencil() || attrs.get_is_canvas(),
            preserve: attrs.get_preserve_drawing_buffer(),
            legacy: attrs.get_gl_legacy(),
            ..Default::default()
        };
        unsafe {
            gl_bindings::GenFramebuffers(1, &mut drawable.framebuffer);
            gl_bindings::GenRenderbuffers(1, &mut drawable.color);
            if drawable.depth || drawable.stencil {
                gl_bindings::GenRenderbuffers(1, &mut drawable.depth_stencil);
            }
        }
        drawable
    }

    /// (Re)allocates the buffers, at the layer's size in pixels or `width` x `height` offscreen, and
    /// leaves the framebuffer bound. A layer with no size yet leaves it incomplete.
    fn store(&mut self, context: &EAGLContext, width: i32, height: i32) {
        unsafe {
            let renderbuffer = bound(gl_bindings::RENDERBUFFER_BINDING);
            gl_bindings::BindRenderbuffer(gl_bindings::RENDERBUFFER, self.color);
            match self.layer.as_ref() {
                Some(layer) => {
                    context.renderbuffer_storage_from_drawable(layer);
                }
                None => gl_bindings::RenderbufferStorage(
                    gl_bindings::RENDERBUFFER,
                    gl_bindings::RGBA8,
                    width.max(1),
                    height.max(1),
                ),
            }
            gl_bindings::GetRenderbufferParameteriv(
                gl_bindings::RENDERBUFFER,
                gl_bindings::RENDERBUFFER_WIDTH,
                &mut self.width,
            );
            gl_bindings::GetRenderbufferParameteriv(
                gl_bindings::RENDERBUFFER,
                gl_bindings::RENDERBUFFER_HEIGHT,
                &mut self.height,
            );
            if self.depth_stencil != 0 && self.width > 0 && self.height > 0 {
                gl_bindings::BindRenderbuffer(gl_bindings::RENDERBUFFER, self.depth_stencil);
                gl_bindings::RenderbufferStorage(
                    gl_bindings::RENDERBUFFER,
                    gl_bindings::DEPTH24_STENCIL8,
                    self.width,
                    self.height,
                );
            }
            gl_bindings::BindRenderbuffer(gl_bindings::RENDERBUFFER, renderbuffer);

            gl_bindings::BindFramebuffer(gl_bindings::FRAMEBUFFER, self.framebuffer);
            gl_bindings::FramebufferRenderbuffer(
                gl_bindings::FRAMEBUFFER,
                gl_bindings::COLOR_ATTACHMENT0,
                gl_bindings::RENDERBUFFER,
                self.color,
            );
            if self.depth {
                gl_bindings::FramebufferRenderbuffer(
                    gl_bindings::FRAMEBUFFER,
                    gl_bindings::DEPTH_ATTACHMENT,
                    gl_bindings::RENDERBUFFER,
                    self.depth_stencil,
                );
            }
            if self.stencil {
                gl_bindings::FramebufferRenderbuffer(
                    gl_bindings::FRAMEBUFFER,
                    gl_bindings::STENCIL_ATTACHMENT,
                    gl_bindings::RENDERBUFFER,
                    self.depth_stencil,
                );
            }
        }
    }

    fn bind(&self) {
        unsafe { gl_bindings::BindFramebuffer(gl_bindings::FRAMEBUFFER, self.framebuffer) };
    }

    /// Tells the GPU depth and stencil needn't be written back to memory: the next frame starts
    /// them over.
    fn discard_depth_stencil(&self) {
        let mut attachments = [0u32; 2];
        let mut count = 0;
        if self.depth {
            attachments[count] = gl_bindings::DEPTH_ATTACHMENT;
            count += 1;
        }
        if self.stencil {
            attachments[count] = gl_bindings::STENCIL_ATTACHMENT;
            count += 1;
        }
        if count == 0 {
            return;
        }
        unsafe {
            let framebuffer = bound(gl_bindings::FRAMEBUFFER_BINDING);
            self.bind();
            if self.legacy {
                if let Some(discard) = discard_framebuffer_ext() {
                    discard(gl_bindings::FRAMEBUFFER, count as i32, attachments.as_ptr());
                }
            } else {
                gl_bindings::InvalidateFramebuffer(
                    gl_bindings::FRAMEBUFFER,
                    count as i32,
                    attachments.as_ptr(),
                );
            }
            gl_bindings::BindFramebuffer(gl_bindings::FRAMEBUFFER, framebuffer);
        }
    }

    /// Hands the color buffer to the layer. Callable from any thread: the layer takes it with the
    /// next Core Animation commit, without waiting on the main thread.
    fn present(&self, context: &EAGLContext) -> bool {
        if self.layer.is_none() || self.width == 0 || self.height == 0 {
            return false;
        }
        if !self.preserve {
            self.discard_depth_stencil();
        }
        unsafe {
            let renderbuffer = bound(gl_bindings::RENDERBUFFER_BINDING);
            gl_bindings::BindRenderbuffer(gl_bindings::RENDERBUFFER, self.color);
            let presented = context.present_renderbuffer();
            gl_bindings::BindRenderbuffer(gl_bindings::RENDERBUFFER, renderbuffer);
            presented
        }
    }

    /// The color buffer as top-down RGBA.
    fn read(&self) -> Vec<u8> {
        let (width, height) = (self.width.max(0) as usize, self.height.max(0) as usize);
        let row = width * 4;
        let mut pixels = vec![0u8; row * height];
        if pixels.is_empty() {
            return pixels;
        }
        unsafe {
            let framebuffer = bound(gl_bindings::FRAMEBUFFER_BINDING);
            let pack_buffer = if self.legacy {
                0
            } else {
                bound(gl_bindings::PIXEL_PACK_BUFFER_BINDING)
            };
            let mut alignment = 4;
            gl_bindings::GetIntegerv(gl_bindings::PACK_ALIGNMENT, &mut alignment);
            if pack_buffer != 0 {
                gl_bindings::BindBuffer(gl_bindings::PIXEL_PACK_BUFFER, 0);
            }
            self.bind();
            gl_bindings::PixelStorei(gl_bindings::PACK_ALIGNMENT, 4);
            gl_bindings::ReadPixels(
                0,
                0,
                width as i32,
                height as i32,
                gl_bindings::RGBA,
                gl_bindings::UNSIGNED_BYTE,
                pixels.as_mut_ptr() as *mut c_void,
            );
            gl_bindings::PixelStorei(gl_bindings::PACK_ALIGNMENT, alignment);
            gl_bindings::BindFramebuffer(gl_bindings::FRAMEBUFFER, framebuffer);
            if pack_buffer != 0 {
                gl_bindings::BindBuffer(gl_bindings::PIXEL_PACK_BUFFER, pack_buffer);
            }
        }
        // GL reads bottom-up.
        for y in 0..height / 2 {
            let (upper, lower) = pixels.split_at_mut((height - 1 - y) * row);
            upper[y * row..(y + 1) * row].swap_with_slice(&mut lower[..row]);
        }
        pixels
    }

    fn delete(&mut self) {
        unsafe {
            if self.framebuffer != 0 {
                gl_bindings::DeleteFramebuffers(1, &self.framebuffer);
            }
            if self.color != 0 {
                gl_bindings::DeleteRenderbuffers(1, &self.color);
            }
            if self.depth_stencil != 0 {
                gl_bindings::DeleteRenderbuffers(1, &self.depth_stencil);
            }
        }
        self.framebuffer = 0;
        self.color = 0;
        self.depth_stencil = 0;
    }
}

impl GLContext {
    pub fn as_raw(&self) -> GLContextRaw {
        GLContextRaw {
            context: self.0.context.clone(),
            sharegroup: self.0.sharegroup.clone(),
        }
    }

    /// Moves the drawable to another `CAEAGLLayer`.
    pub fn set_surface(&mut self, layer: NonNull<c_void>) -> bool {
        let Some(layer) = (unsafe { Id::<NSObject>::retain(layer.as_ptr() as _) }) else {
            return false;
        };
        let Some(context) = self.0.context.clone() else {
            return false;
        };
        if !EAGLContext::set_current_context(Some(&context)) {
            return false;
        }
        let framebuffer = bound(gl_bindings::FRAMEBUFFER_BINDING);
        self.0.drawable.layer = Some(layer);
        self.0.drawable.store(&context, 0, 0);
        unsafe { gl_bindings::BindFramebuffer(gl_bindings::FRAMEBUFFER, framebuffer) };
        true
    }

    /// Reallocates the drawable after its layer changed size (`width` and `height` size an
    /// offscreen one). Its contents are lost; the bindings are kept.
    pub fn resize_drawable(&mut self, width: i32, height: i32) {
        let Some(context) = self.0.context.clone() else {
            return;
        };
        if !EAGLContext::set_current_context(Some(&context)) {
            return;
        }
        let framebuffer = bound(gl_bindings::FRAMEBUFFER_BINDING);
        self.0.drawable.store(&context, width, height);
        unsafe { gl_bindings::BindFramebuffer(gl_bindings::FRAMEBUFFER, framebuffer) };
    }

    /// `layer` is the view's `CAEAGLLayer`.
    pub fn create_shared_window_context(
        context_attrs: &mut ContextAttributes,
        layer: NonNull<c_void>,
        context: &GLContext,
    ) -> Option<Self> {
        let layer = unsafe { Id::<NSObject>::retain(layer.as_ptr() as _) }?;
        GLContext::create(context_attrs, Some(layer), 0, 0, Some(context))
    }

    /// `layer` is the view's `CAEAGLLayer`.
    pub fn create_window_context(
        context_attrs: &mut ContextAttributes,
        layer: NonNull<c_void>,
    ) -> Option<Self> {
        let layer = unsafe { Id::<NSObject>::retain(layer.as_ptr() as _) }?;
        GLContext::create(context_attrs, Some(layer), 0, 0, None)
    }

    /// Makes the context current on the calling thread, with its drawable bound.
    fn create(
        context_attrs: &mut ContextAttributes,
        layer: Option<Id<NSObject>>,
        width: i32,
        height: i32,
        shared_context: Option<&GLContext>,
    ) -> Option<Self> {
        IS_GL_SYMBOLS_LOADED.get_or_init(|| {
            gl_bindings::load_with(|symbol| super::get_proc_address(symbol).cast());
            true
        });

        let legacy = context_attrs.get_gl_legacy();
        let api = if legacy {
            EAGLRenderingAPI::GLES2
        } else {
            EAGLRenderingAPI::GLES3
        };

        let share_group = match shared_context {
            Some(context) => context.0.sharegroup.clone(),
            _ => {
                if legacy {
                    EAGLSharegroup::legacy()
                } else {
                    EAGLSharegroup::new()
                }
            }
        };

        let context = EAGLContext::new_with_api_sharegroup(api, &share_group)?;

        if !EAGLContext::set_current_context(Some(&context)) {
            return None;
        }

        let mut drawable = Drawable::new(layer, context_attrs);
        drawable.store(&context, width, height);
        unsafe { gl_bindings::Viewport(0, 0, drawable.width, drawable.height) };

        Some(GLContext(GLContextInner {
            context: Some(context),
            sharegroup: share_group,
            drawable,
        }))
    }

    pub fn create_offscreen_context(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
    ) -> Option<GLContext> {
        GLContext::create(context_attrs, None, width, height, None)
    }

    pub fn create_shared_offscreen_context(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
        shared_context: &GLContext,
    ) -> Option<GLContext> {
        GLContext::create(context_attrs, None, width, height, Some(shared_context))
    }

    fn has_extension(extensions: &str, name: &str) -> bool {
        !extensions.split(' ').into_iter().any(|s| s == name)
    }

    pub fn has_gl2support() -> bool {
        true
    }

    /// The drawable as top-down RGBA.
    pub fn snapshot(&self) -> Option<Vec<u8>> {
        if !self.make_current() {
            return None;
        }
        Some(self.0.drawable.read())
    }

    pub fn set_vsync(&self, _sync: bool) -> bool {
        true
    }

    pub fn make_current(&self) -> bool {
        if let Some(context) = self.0.context.as_ref() {
            return EAGLContext::set_current_context(Some(context));
        }

        false
    }

    pub fn remove_if_current(&self) -> bool {
        if let Some(context) = self.0.context.as_ref() {
            return context.remove_if_current();
        }
        false
    }

    pub fn bind_drawable(&self) {
        self.0.drawable.bind();
    }

    pub fn swap_buffers(&self) -> bool {
        match self.0.context.as_ref() {
            Some(context) => self.0.drawable.present(context),
            None => false,
        }
    }

    pub fn get_surface_width(&self) -> i32 {
        self.0.drawable.width
    }

    pub fn get_surface_height(&self) -> i32 {
        self.0.drawable.height
    }

    pub fn get_surface_dimensions(&self) -> (i32, i32) {
        (self.0.drawable.width, self.0.drawable.height)
    }
}
