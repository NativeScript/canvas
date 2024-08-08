use std::os::raw::c_void;
#[cfg(feature = "vulkan")]
use ash::vk;
#[cfg(feature = "vulkan")]
use ash::vk::Handle;
use skia_safe::gpu;
use crate::context::{Context, State, SurfaceData, SurfaceEngine};
use crate::context::text_styles::text_direction::TextDirection;

#[cfg(feature = "vulkan")]
impl Context {
    #[cfg(target_os = "android")]
    pub fn new_vulkan(
        width: f32,
        height: f32,
        window: *mut c_void,
        density: f32,
        samples: u32,
        alpha: bool,
        font_color: i32,
        ppi: f32,
        direction: u8,
    ) -> Self {
        let ash_graphics =
            unsafe { crate::context::surface_vulkan::AshGraphics::new("ns-canvas") }.unwrap();

        let mut context = {
            let get_proc = |of| unsafe {
                match ash_graphics.get_proc(of) {
                    Some(f) => f as _,
                    None => {
                        println!("resolve of {} failed", of.name().to_str().unwrap());
                        std::ptr::null()
                    }
                }
            };

            let backend_context = unsafe {
                gpu::vk::BackendContext::new(
                    ash_graphics.instance.handle().as_raw() as _,
                    ash_graphics.physical_device.as_raw() as _,
                    ash_graphics.device.handle().as_raw() as _,
                    (
                        ash_graphics.queue_and_index.0.as_raw() as _,
                        ash_graphics.queue_and_index.1,
                    ),
                    &get_proc,
                )
            };

            gpu::direct_contexts::make_vulkan(&backend_context, None)
        }
            .unwrap();

        let surface_fn =
            ash::khr::android_surface::Instance::new(&ash_graphics.entry, &ash_graphics.instance);

        let mut info = vk::AndroidSurfaceCreateInfoKHR::default();
        let _ = info.window(window);

        let vk_surface = unsafe { surface_fn.create_android_surface(&info, None).ok() };

        let alpha_type = if alpha {
            skia_safe::AlphaType::Unpremul
        } else {
            skia_safe::AlphaType::Premul
        };

        let info = skia_safe::ImageInfo::new(
            skia_safe::ISize::new(width as i32, height as i32),
            skia_safe::ColorType::N32,
            alpha_type,
            Some(skia_safe::ColorSpace::new_srgb()),
        );

        let surface = gpu::surfaces::render_target(
            &mut context,
            gpu::Budgeted::Yes,
            &info,
            Some(samples as usize),
            Some(gpu::SurfaceOrigin::TopLeft),
            None,
            false,
            None,
        )
            .unwrap();

        let mut state = State::default();
        state.direction = TextDirection::from(direction as u32);

        let bounds = skia_safe::Rect::from_wh(width, height);
        Context {
            direct_context: Some(context),
            surface_data: SurfaceData {
                bounds,
                scale: density,
                ppi,
                engine: SurfaceEngine::Vulkan,
            },
            surface,
            vk_surface,
            ash_graphics: None,
            path: Default::default(),
            state,
            state_stack: vec![],
            font_color: skia_safe::Color::new(font_color as u32),
            surface_state: false,
        }
    }

    #[cfg(target_os = "android")]
    pub fn resize_vulkan(
        context: &mut Context,
        width: f32,
        height: f32,
        samples: u32,
        alpha: bool,
    ) {
        if let Some(direct_context) = context.direct_context.as_mut() {
            let alpha_type = if alpha {
                skia_safe::AlphaType::Unpremul
            } else {
                skia_safe::AlphaType::Premul
            };

            let info = skia_safe::ImageInfo::new(
                skia_safe::ISize::new(width as i32, height as i32),
                skia_safe::ColorType::N32,
                alpha_type,
                Some(skia_safe::ColorSpace::new_srgb()),
            );

            let surface = gpu::surfaces::render_target(
                direct_context,
                gpu::Budgeted::Yes,
                &info,
                Some(samples as usize),
                Some(gpu::SurfaceOrigin::TopLeft),
                None,
                false,
                None,
            )
                .unwrap();

            let bounds = skia_safe::Rect::from_wh(width, height);
            context.surface_data.bounds = bounds;
            context.surface = surface;
        }
    }
}
