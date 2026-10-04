//! A view's window arriving is queued to the context's thread (waiting can deadlock the buffer
//! queue); a window going is waited on, as the context must let go of it first.

use std::ffi::c_void;
use std::ptr::NonNull;

use canvas_core::context_attributes::{ContextAttributes, PowerPreference};
use ndk::native_window::NativeWindow;
use raw_window_handle::{AndroidNdkWindowHandle, RawWindowHandle};

use super::{Binding, CanvasOffscreenSurface};
use crate::c2d::render_thread::TargetHandle;
use crate::{CanvasRenderingContext2D, WebGLState};

#[derive(Default)]
pub(super) struct View {
    pub(super) window: Option<NativeWindow>,
}

struct SendWindow(NativeWindow);

// ANativeWindow is reference counted and thread-safe.
unsafe impl Send for SendWindow {}

fn surface_attributes(context: &canvas_2d::context::Context) -> ContextAttributes {
    ContextAttributes::new(
        !context.surface_data().is_opaque(),
        false,
        false,
        false,
        PowerPreference::Default,
        true,
        false,
        false,
        false,
        false,
        true,
        false,
        context.surface_data().color_space(),
    )
}

/// `resize`: take the window's size as the bitmap's (an OffscreenCanvas keeps its own).
pub fn update_2d_surface(
    context: &mut CanvasRenderingContext2D,
    window: NonNull<c_void>,
    width: i32,
    height: i32,
    resize: bool,
) {
    {
        let context = context.get_context_mut();
        // A new EGL surface starts blank; resize() only clears on a size change.
        let offscreen = context.presents_through_window();
        let pixels = if context.gl_context.is_some()
            && !offscreen
            && context.surface_data().width() as i32 == width
            && context.surface_data().height() as i32 == height
        {
            context.get_image()
        } else {
            None
        };
        let mut attr = surface_attributes(context);
        if let Some(gl_context) = context.gl_context.as_mut() {
            let handle = RawWindowHandle::AndroidNdk(AndroidNdkWindowHandle::new(window));
            gl_context.set_window_surface(&mut attr, width, height, handle);
            gl_context.make_current();
        }
        context.use_offscreen_for_window();
        if let Some(pixels) = pixels {
            context.draw_pixels(&pixels);
        }
        if context.presents_through_window() {
            context.present_to_window();
            if let Some(gl_context) = context.gl_context.as_ref() {
                gl_context.swap_buffers();
            }
        }

        #[cfg(feature = "vulkan")]
        if let Some(vulkan_context) = context.vulkan_context.as_mut() {
            vulkan_context.set_view(window.as_ptr(), width as u32, height as u32);
        }
    }
    if resize {
        context.resize(width as f32, height as f32)
    }
}

pub fn detach_2d_surface(context: &mut CanvasRenderingContext2D) {
    let context = context.get_context_mut();

    #[cfg(feature = "vulkan")]
    if context.vulkan_context.is_some() {
        context.detach_vulkan_view();
        return;
    }

    let width = context.surface_data().width() as i32;
    let height = context.surface_data().height() as i32;
    let mut attr = surface_attributes(context);
    context.flush_and_render_to_surface();
    if let Some(gl_context) = context.gl_context.as_mut() {
        gl_context.resize_pbuffer(&mut attr, width, height);
        gl_context.make_current();
    }
}

fn window_ptr(window: &NativeWindow) -> NonNull<c_void> {
    window.ptr().cast()
}

pub(super) fn attach_2d(handle: &TargetHandle, window: NativeWindow, width: u32, height: u32) {
    let window = SendWindow(window);
    handle.post(move |real| {
        let window = window;
        update_2d_surface(real, window_ptr(&window.0), width as i32, height as i32, false)
    });
}

pub(super) fn attach_webgl(state: &WebGLState, window: NativeWindow) {
    let window = SendWindow(window);
    state.post(move |state| {
        let window = window;
        state.set_window_surface(window.0.width(), window.0.height(), window_ptr(&window.0));
        state.make_current();
    });
}

impl CanvasOffscreenSurface {
    pub fn set_window(&self, window: NativeWindow) {
        let (width, height) = self.size();
        let mut inner = self.inner();
        let Some(view) = inner.view.as_mut() else {
            return;
        };
        view.window = Some(window.clone());
        match &inner.binding {
            Binding::TwoD(handle) => attach_2d(handle, window, width, height),
            Binding::WebGL(state) => attach_webgl(state.state(), window),
            Binding::WebGPU(context) => unsafe {
                crate::webgpu::gpu_canvas_context::canvas_native_webgpu_context_resize(
                    std::sync::Arc::as_ptr(context) as *mut _,
                    window.ptr().as_ptr() as *mut c_void,
                    width,
                    height,
                )
            },
            Binding::TwoDDirect | Binding::None => {}
        }
    }

    pub fn window_destroyed(&self) {
        let (width, height) = self.size();
        let (binding, window) = {
            let mut inner = self.inner();
            let window = inner.view.as_mut().and_then(|view| view.window.take());
            let binding = match &inner.binding {
                Binding::TwoD(handle) => Some(Detach::TwoD(handle.clone())),
                Binding::WebGL(state) => {
                    crate::canvas_native_webgl_state_reference(state.0);
                    Some(Detach::WebGL(super::GlRef(state.0)))
                }
                Binding::WebGPU(context) => Some(Detach::WebGPU(std::sync::Arc::clone(context))),
                _ => None,
            };
            (binding, window)
        };
        if window.is_none() {
            return;
        }
        // Outside the lock: these wait on the context's thread.
        match binding {
            Some(Detach::TwoD(handle)) => {
                handle.sync(detach_2d_surface);
            }
            Some(Detach::WebGL(state)) => {
                let (width, height) = (width as i32, height as i32);
                state.state().detach(move |state| {
                    state.make_current();
                    state.resize_pbuffer(width, height);
                });
            }
            Some(Detach::WebGPU(context)) => unsafe {
                crate::webgpu::gpu_canvas_context::canvas_native_webgpu_context_detach_surface(
                    std::sync::Arc::as_ptr(&context),
                );
            },
            None => {}
        }
        drop(window);
    }
}

enum Detach {
    TwoD(TargetHandle),
    WebGL(super::GlRef),
    WebGPU(std::sync::Arc<crate::webgpu::gpu_canvas_context::CanvasGPUCanvasContext>),
}

/// UI thread. Takes its own reference to the window.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_set_window(
    surface: *const CanvasOffscreenSurface,
    window: *mut c_void,
) {
    let (Some(surface), Some(window)) = (surface.as_ref(), NonNull::new(window)) else {
        return;
    };
    surface.set_window(NativeWindow::clone_from_ptr(window.cast()));
}

/// UI thread, synchronously.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_window_destroyed(surface: *const CanvasOffscreenSurface) {
    if let Some(surface) = surface.as_ref() {
        surface.window_destroyed();
    }
}
