//! GL rasterization. EGL plumbing is `canvas_core`'s; this only owns the Skia side.

use canvas_core::context_attributes::ContextAttributes;
use canvas_core::gpu::gl::GLContext;
use skia_safe::gpu::{self, gl::FramebufferInfo, DirectContext, SurfaceOrigin};
use skia_safe::{ColorType, PixelGeometry, Surface, SurfaceProps, SurfacePropsFlags};

use super::Frame;

/// Field order is drop order: Skia's GL objects only exist while the EGL context does.
pub struct GlSurface {
    surface: Surface,
    direct: DirectContext,
    gl: GLContext,
    width: i32,
    height: i32,
}

impl GlSurface {
    pub fn new(window: *mut std::ffi::c_void, width: i32, height: i32) -> Option<Self> {
        let mut attrs = ContextAttributes::new(
            true,  // alpha
            false, // antialias: Skia antialiases its own geometry
            false, // depth
            false, // fail_if_major_performance_caveat
            canvas_core::context_attributes::PowerPreference::Default,
            true,  // premultiplied_alpha
            false, // preserve_drawing_buffer
            true,  // stencil: Skia clips and paths need it
            false, // desynchronized
            false, // xr_compatible
            true,  // is_canvas
            false, // gl_legacy
            canvas_core::context_attributes::ColorSpace::Srgb,
        );

        let handle = super::window_handle(window)?;
        let Some(gl) = GLContext::create_window_context(&mut attrs, width, height, handle) else {
            log::warn!("svg gpu: no gl window context");
            return None;
        };
        if !gl.make_current() {
            log::warn!("svg gpu: gl context would not go current");
            return None;
        }

        let interface = gpu::gl::Interface::new_native()?;
        let Some(mut direct) = gpu::direct_contexts::make_gl(interface, None) else {
            log::warn!("svg gpu: skia rejected the gl interface");
            return None;
        };
        let surface = wrap_framebuffer(&mut direct, width, height)?;

        Some(Self {
            gl,
            direct,
            surface,
            width,
            height,
        })
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        if width == self.width && height == self.height {
            return;
        }
        if !self.gl.make_current() {
            return;
        }
        self.width = width;
        self.height = height;
        if let Some(surface) = wrap_framebuffer(&mut self.direct, width, height) {
            self.surface = surface;
        }
    }

    /// Every failure is terminal: EGL only refuses current/swap once the context is gone.
    pub fn render(&mut self, paint: &dyn Fn(&skia_safe::Canvas, i32, i32)) -> Frame {
        if !self.gl.make_current() {
            log::warn!("svg gpu: gl context would not go current");
            return Frame::Lost;
        }
        if self.direct.is_device_lost() {
            return Frame::Lost;
        }
        let (width, height) = (self.width, self.height);
        {
            let canvas = self.surface.canvas();
            canvas.clear(skia_safe::Color::TRANSPARENT);
            paint(canvas, width, height);
        }
        self.direct.flush_and_submit();
        if !self.gl.swap_buffers() {
            log::warn!("svg gpu: gl swap failed");
            return Frame::Lost;
        }
        Frame::Presented
    }
}

impl Drop for GlSurface {
    fn drop(&mut self) {
        // The context must be current for Skia to release its GL objects.
        self.gl.make_current();
        self.direct.abandon();
        self.gl.remove_if_current();
    }
}

fn wrap_framebuffer(direct: &mut DirectContext, width: i32, height: i32) -> Option<Surface> {
    let mut bound = [0i32];
    unsafe { gl_bindings::GetIntegerv(gl_bindings::FRAMEBUFFER_BINDING, bound.as_mut_ptr()) }

    let mut framebuffer = FramebufferInfo::from_fboid(bound[0] as u32);
    framebuffer.format = gpu::gl::Format::RGBA8.into();

    let target = gpu::backend_render_targets::make_gl((width, height), Some(0), 8, framebuffer);
    let props = SurfaceProps::new(SurfacePropsFlags::default(), PixelGeometry::Unknown);

    gpu::surfaces::wrap_backend_render_target(
        direct,
        &target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        Some(&props),
    )
}
