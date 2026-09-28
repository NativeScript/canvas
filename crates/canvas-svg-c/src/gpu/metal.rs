//! Metal rasterization. The layer hands out a new drawable each frame, so the surface is rebuilt
//! around `next_drawable` before each draw.

use canvas_core::gpu::metal::MetalContext;
use objc2::rc::Retained;
use objc2_metal::MTLTexture;
use objc2_quartz_core::CAMetalDrawable;
use skia_safe::gpu::mtl::TextureInfo;
use skia_safe::gpu::{self, DirectContext, SurfaceOrigin};
use skia_safe::{ColorType, Surface};

use super::Frame;

/// Field order is drop order: the surface wraps a drawable's texture, so `mtl` drops last.
pub struct MetalSurface {
    surface: Option<Surface>,
    direct: DirectContext,
    mtl: MetalContext,
    width: i32,
    height: i32,
}

impl MetalSurface {
    /// `view` is a `CAMetalLayer`-backed `UIView*`/`NSView*`.
    pub fn new(view: *mut std::ffi::c_void, width: i32, height: i32) -> Option<Self> {
        // `MetalContext` releases the view on drop and rebuilds create new ones, so each takes
        // its own reference; the caller keeps theirs.
        let view = unsafe { objc2::ffi::objc_retain(view.cast()) }.cast();
        let mtl = MetalContext::new(view);
        let backend = unsafe {
            gpu::mtl::BackendContext::new(
                mtl.device() as gpu::mtl::Handle,
                mtl.queue() as gpu::mtl::Handle,
            )
        };
        let direct = gpu::direct_contexts::make_metal(&backend, None)?;

        Some(Self {
            mtl,
            direct,
            surface: None,
            width,
            height,
        })
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        if width == self.width && height == self.height {
            return;
        }
        self.direct.flush_and_submit();
        self.mtl.set_drawable_size(width as f64, height as f64);
        self.width = width;
        self.height = height;
        self.surface = None;
    }

    pub fn render(&mut self, paint: &dyn Fn(&skia_safe::Canvas, i32, i32)) -> Frame {
        let _pool = MetalContext::new_release_pool();

        if self.direct.is_device_lost() {
            return Frame::Lost;
        }

        // A drawable is only valid for the frame it was acquired for.
        self.surface = self.wrap_next_drawable();
        let Some(surface) = self.surface.as_mut() else {
            // All drawables in flight: congestion, not loss.
            return Frame::Skipped;
        };

        let (width, height) = (self.width, self.height);
        {
            let canvas = surface.canvas();
            canvas.clear(skia_safe::Color::TRANSPARENT);
            paint(canvas, width, height);
        }
        self.direct.flush_and_submit();
        // Not `MetalContext::present`: that sends `-present` to the view, which `SVGMetalView`
        // lacks, and UIKit forbids messaging a UIView from the render thread.
        self.mtl.present_drawable();
        Frame::Presented
    }

    fn wrap_next_drawable(&mut self) -> Option<Surface> {
        let drawable = self.mtl.next_drawable()?;
        let info =
            unsafe { TextureInfo::new(Retained::as_ptr(&drawable.texture()) as gpu::mtl::Handle) };
        let texture = unsafe {
            gpu::backend_textures::make_mtl(
                (self.width, self.height),
                gpu::Mipmapped::No,
                &info,
                "",
            )
        };
        gpu::surfaces::wrap_backend_texture(
            &mut self.direct,
            &texture,
            SurfaceOrigin::TopLeft,
            None,
            ColorType::BGRA8888,
            None,
            None,
        )
    }
}

impl Drop for MetalSurface {
    fn drop(&mut self) {
        self.direct.flush_submit_and_sync_cpu();
        self.surface = None;
        self.direct.abandon();
    }
}
