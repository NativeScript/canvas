use std::os::raw::c_void;

use skia_safe::{Color, ColorType, gpu, PixelGeometry, SurfaceProps, SurfacePropsFlags};

use crate::context::{Context, State, SurfaceData, SurfaceEngine};
use crate::context::paths::path::Path;
use crate::context::text_styles::text_direction::TextDirection;

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
        let backend = unsafe {
            gpu::mtl::BackendContext::new(
                device as gpu::mtl::Handle,
                queue as gpu::mtl::Handle,
            )
        };
        let mut context = gpu::direct_contexts::make_metal(&backend, None).unwrap();
        let surface_props = SurfaceProps::new(SurfacePropsFlags::default(), PixelGeometry::Unknown);
        let surface_holder = unsafe {
            gpu::surfaces::wrap_mtk_view(
                &mut context,
                view as gpu::mtl::Handle,
                gpu::SurfaceOrigin::TopLeft,
                samples.try_into().ok(),
                ColorType::BGRA8888,
                None,
                Some(&surface_props),
            )
        };

        let mut state = State::default();
        state.direction = TextDirection::from(direction as u32);

        let bounds = skia_safe::Rect::from_wh(width, height);
        Context {
            surface_data: SurfaceData {
                bounds,
                scale: density,
                ppi,
                engine: SurfaceEngine::Metal,
            },
            surface: surface_holder.unwrap(),
            is_dirty: false,
            direct_context: Some(context),
            path: Path::default(),
            state,
            state_stack: vec![],
            font_color: Color::new(font_color as u32),
        }
    }

    pub fn resize_metal(context: &mut Context, width: f32, height: f32) {
        let bounds = skia_safe::Rect::from_wh(width, height);
        context.surface_data.bounds = bounds;
    }
}
