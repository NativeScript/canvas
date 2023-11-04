use std::os::raw::c_void;

use skia_safe::{gpu, Color, ColorType, PixelGeometry, Surface, SurfaceProps, SurfacePropsFlags};

use crate::context::paths::path::Path;
use crate::context::text_styles::text_direction::TextDirection;
use crate::context::{Context, Device, State};

#[cfg(feature = "metal")]
impl Context {
    pub fn new_metal(
        width: f32,
        height: f32,
        device: *mut c_void,
        queue: *mut c_void,
        view: *mut c_void,
        density: f32,
        samples: i32,
        alpha: bool,
        font_color: i32,
        ppi: f32,
        direction: u8,
    ) -> Self {
        let context_device = Device {
            width,
            height,
            density,
            non_gpu: false,
            samples: samples as usize,
            alpha,
            ppi,
        };

        let backend = unsafe {
            skia_safe::gpu::mtl::BackendContext::new(
                device.as_ptr() as skia_safe::gpu::mtl::Handle,
                queue.as_ptr() as skia_safe::gpu::mtl::Handle,
                std::ptr::null(),
            )
        };
        let mut context = skia_safe::gpu::DirectContext::new_metal(backend, None).unwrap();
        let surface_props = SurfaceProps::new(SurfacePropsFlags::default(), PixelGeometry::Unknown);
        let surface_holder = unsafe { gpu::surfaces::wrap_mtk_view(
            &mut context,
            view as skia_safe::gpu::mtl::Handle,
            gpu::SurfaceOrigin::TopLeft,
            Some(samples),
            ColorType::BGRA8888,
            None,
            Some(&surface_props),
        )};

        Context {
            surface: surface_holder.unwrap(),
            path: Path::default(),
            state: State::from_device(context_device, TextDirection::from(direction as u32)),
            state_stack: vec![],
            font_color: Color::new(font_color as u32),
            device: context_device,
        }
    }
}
