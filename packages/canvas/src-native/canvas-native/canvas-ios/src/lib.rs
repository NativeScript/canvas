use std::cell::RefCell;
use std::ffi::{c_float, c_int, c_long, c_longlong, c_uchar, c_void, CStr};
use std::ptr::NonNull;
use std::sync::Once;

use icrate::objc2::declare::ClassBuilder;
use icrate::objc2::ffi::BOOL;
use icrate::objc2::rc::{autoreleasepool, Id, Owned, Shared};
use icrate::objc2::runtime::{Bool, Class, Object, Sel};
use icrate::objc2::{declare_class, sel, ClassType, Encoding, Message, RefEncode};
use icrate::Foundation::{NSData, NSObject, NSString};
use raw_window_handle::HasRawWindowHandle;

use canvas_2d::context::fill_and_stroke_styles::pattern::Repetition;
use canvas_2d::utils::image::from_image_slice;
use canvas_core::context_attributes::ContextAttributes;
use canvas_core::gl::GLContext;
use canvas_core::image_asset::ImageAsset;
use canvas_cxx::CanvasRenderingContext2D;
use canvas_cxx::PaintStyle;

// #[cfg(any(target_os = "macos", target_os = "ios"))]
// #[link(name = "OpenGLES", kind = "framework")]
// #[link(name = "GLKit", kind = "framework")]
// #[link(name = "Foundation", kind = "framework")]
// extern "C" {}

declare_class!(
    #[derive(Debug)]
    pub struct CanvasHelpers {}

    unsafe impl ClassType for CanvasHelpers {
        type Super = NSObject;
        const NAME: &'static str = "CanvasHelpers";
    }

    unsafe impl CanvasHelpers {
        #[method(initGLWithView:width:height:alpha:antialias:depth:fail_if_major_performance_caveat:power_preference:premultiplied_alpha:preserve_drawing_buffer:stencil:desynchronized:xr_compatible:version:is_canvas:)]
        fn init_gl_with_view(
            view: c_longlong,
            width: c_int,
            height: c_int,
            alpha: BOOL,
            antialias: BOOL,
            depth: BOOL,
            fail_if_major_performance_caveat: BOOL,
            power_preference: *mut Object,
            premultiplied_alpha: BOOL,
            preserve_drawing_buffer: BOOL,
            stencil: BOOL,
            desynchronized: BOOL,
            xr_compatible: BOOL,
            version: c_int,
            is_canvas: BOOL,
        ) -> c_longlong {
            autoreleasepool(|pool| {
                if let Some(power_preference) =
                    unsafe { Id::<NSString, Shared>::new(power_preference.cast()) }
                {
                    let power_preference = power_preference.as_str(pool);

                    return canvas_native_init_ios_gl(
                        view,
                        width,
                        height,
                        alpha,
                        antialias,
                        depth,
                        fail_if_major_performance_caveat,
                        power_preference,
                        premultiplied_alpha,
                        preserve_drawing_buffer,
                        stencil,
                        desynchronized,
                        xr_compatible,
                        version,
                        is_canvas,
                    );
                }
                0
            })
        }

        #[method(releaseGLWithContext:)]
        fn release_gl_with_context(context: c_longlong) {
            canvas_native_release_ios_gl(context)
        }

        #[method(getGLPointerWithContext:)]
        fn get_gl_pointer_with_context(context: c_longlong) -> c_longlong {
            canvas_native_get_gl_pointer(context)
        }

        #[method(releaseGLPointerWithContext:)]
        fn release_gl_pointer_with_context(context: c_longlong) {
            canvas_native_release_gl_pointer(context)
        }

        #[method(updateGLSurfaceWithView:width:height:context:)]
        fn update_gl_surface_with_view(
            view: c_longlong,
            width: c_int,
            height: c_int,
            context: c_longlong,
        ) {
            canvas_native_update_gl_surface(view, width, height, context);
        }

        #[method(create2DContext:width:height:alpha:density:samples:font_color:ppi:direction:)]
        fn create_2d_context(
            context: c_longlong,
            width: c_int,
            height: c_int,
            alpha: BOOL,
            density: c_float,
            samples: c_int,
            font_color: c_int,
            ppi: c_float,
            direction: c_int,
        ) -> c_longlong {
            canvas_native_create_2d_context(
                context, width, height, alpha, density, samples, font_color, ppi, direction,
            )
        }

        #[method(test2D:)]
        fn test2D(context: c_longlong) {
            canvas_native_context_2d_test(context);
        }

        #[method(loadImageAssetFromData:data:)]
        fn load_image_asset_from_data(asset: c_longlong, data: *mut NSData) -> bool {
            let data = unsafe { Id::<NSData, Shared>::new(data) };
            if let Some(data) = data {
                return canvas_native_imageasset_load_from_bytes(asset, data.bytes()).into();
            }
            false
        }

        #[method(createPattern:width:height:data:repetition:)]
        fn create_pattern(
            context: c_longlong,
            width: c_int,
            height: c_int,
            data: *mut NSData,
            repetition: *mut NSString,
        ) -> c_longlong {
            let data = unsafe { Id::<NSData, Shared>::new(data) };
            let repetition = unsafe { Id::<NSString, Shared>::new(repetition) };
            if let (Some(data), Some(repetition)) = (data, repetition) {
                return autoreleasepool(|pool| {
                    let repetition = repetition.as_str(pool);
                    return canvas_native_context_create_pattern(
                        context,
                        width,
                        height,
                        data.bytes(),
                        repetition,
                    );
                });
            }
            0
        }

        #[method(drawImageWithContext:width:height:data:dx:dy:)]
        fn draw_image_dx_dy_with_bytes(
            context: c_longlong,
            width: c_float,
            height: c_float,
            data: *mut NSData,
            dx: c_float,
            dy: c_float,
        ) {
            let data = unsafe { Id::<NSData, Shared>::new(data) };
            if let Some(data) = data {
                canvas_native_context_draw_image_dx_dy_with_bytes(
                    context,
                    data.bytes(),
                    width,
                    height,
                    dx,
                    dy,
                );
            }
        }

        #[method(drawImageWithContext:width:height:data:dx:dy:dw:dh:)]
        fn draw_image_dx_dy_dw_dh_with_bytes(
            context: c_longlong,
            width: c_float,
            height: c_float,
            data: *mut NSData,
            dx: c_float,
            dy: c_float,
            dw: c_float,
            dh: c_float,
        ) {
            let data = unsafe { Id::<NSData, Shared>::new(data) };
            if let Some(data) = data {
                canvas_native_context_draw_image_dx_dy_dw_dh_with_bytes(
                    context,
                    data.bytes(),
                    width,
                    height,
                    dx,
                    dy,
                    dw,
                    dh,
                );
            }
        }

        #[method(drawImageWithContext:width:height:data:sx:sy:sw:sh:dx:dy:dw:dh:)]
        fn draw_image_with_bytes(
            context: c_longlong,
            width: c_float,
            height: c_float,
            data: *mut NSData,
            sx: c_float,
            sy: c_float,
            sw: c_float,
            sh: c_float,
            dx: c_float,
            dy: c_float,
            dw: c_float,
            dh: c_float,
        ) {
            let data = unsafe { Id::<NSData, Shared>::new(data) };
            if let Some(data) = data {
                canvas_native_context_draw_image_with_bytes(
                    context,
                    data.bytes(),
                    width,
                    height,
                    sx,
                    sy,
                    sw,
                    sh,
                    dx,
                    dy,
                    dw,
                    dh,
                );
            }
        }
    }
);

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub(crate) struct iOSGLContext {
    pub(crate) context_attributes: ContextAttributes,
    pub(crate) gl_context: GLContext,
    ios_view: NonNull<c_void>,
}

pub fn canvas_native_init_ios_gl(
    view: i64,
    width: i32,
    height: i32,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: &str,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
    version: i32,
    is_canvas: bool,
) -> c_longlong {
    if version == 1 && !GLContext::has_gl2support() {
        return 0;
    }

    if let Some(ios_view) = NonNull::new(view as *mut c_void) {
        let mut attrs = ContextAttributes::new(
            alpha,
            antialias,
            depth,
            fail_if_major_performance_caveat,
            power_preference.as_ref(),
            premultiplied_alpha,
            preserve_drawing_buffer,
            stencil,
            desynchronized,
            xr_compatible,
            is_canvas,
        );

        if let Some(gl_context) = GLContext::create_window_context(&mut attrs, ios_view) {
            return Box::into_raw(Box::new(iOSGLContext {
                ios_view,
                gl_context,
                context_attributes: attrs,
            })) as i64;
        }
    }

    0
}

pub fn canvas_native_create_2d_context(
    context: i64,
    width: i32,
    height: i32,
    alpha: bool,
    density: f32,
    samples: i32,
    font_color: i32,
    ppi: f32,
    direction: i32,
) -> i64 {
    if context == 0 {
        return 0;
    }

    let context = context as *mut iOSGLContext;
    let context = unsafe { &mut *context };

    context.gl_context.make_current();
    let mut frame_buffers = [0];
    unsafe {
        gl_bindings::GetIntegerv(gl_bindings::FRAMEBUFFER_BINDING, frame_buffers.as_mut_ptr())
    };

    let mut ctx_2d = canvas_cxx::CanvasRenderingContext2D::new(
        canvas_2d::context::ContextWrapper::new(canvas_2d::context::Context::new_gl(
            width as f32,
            height as f32,
            density,
            frame_buffers[0],
            samples,
            alpha,
            font_color,
            ppi,
            canvas_2d::context::text_styles::text_direction::TextDirection::from(direction as u32),
        )),
        context.gl_context.clone(),
        alpha,
    );

    Box::into_raw(Box::new(ctx_2d)) as i64
}

pub fn canvas_native_update_gl_surface(view: i64, width: i32, height: i32, context: i64) {
    if context == 0 {
        return;
    }

    if let Some(ios_view) = NonNull::new(view as *mut c_void) {
        let context = context as *mut iOSGLContext;
        let context = unsafe { &mut *context };

        context.gl_context.set_surface(ios_view);

        context.ios_view = ios_view;
    }
}

pub fn canvas_native_release_ios_gl(context: i64) {
    if context == 0 {
        return;
    }
    let context = context as *mut GLContext;
    let _ = unsafe { Box::from_raw(context) };
}

pub fn canvas_native_get_gl_pointer(gl_context: i64) -> i64 {
    if gl_context == 0 {
        return 0;
    }
    let gl_context = gl_context as *mut iOSGLContext;
    let gl_context = unsafe { &*gl_context };
    gl_context.gl_context.as_raw_inner() as i64
}

pub fn canvas_native_release_gl_pointer(gl_context: i64) {
    if gl_context == 0 {
        return;
    }
    let gl_context = gl_context as *const RefCell<canvas_core::gl::GLContextInner>;
    let _ = GLContext::from_raw_inner(gl_context);
}

pub fn canvas_native_context_2d_test(context: i64) {
    if context == 0 {
        return;
    }

    let context = context as *mut canvas_cxx::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.make_current();
    {
        let mut ctx = context.get_context_mut();
        ctx.set_fill_style_with_color("red");
        ctx.fill_rect_xywh(0., 0., 300., 300.);
    }
    context.render();
}

pub fn canvas_native_imageasset_load_from_bytes(asset: i64, bytes: &[u8]) -> bool {
    if asset == 0 {
        return false;
    }

    let asset = asset as *mut ImageAsset;
    let asset = unsafe { &mut *asset };

    asset.load_from_bytes(bytes)
}

pub fn canvas_native_context_create_pattern(
    context: i64,
    width: i32,
    height: i32,
    bytes: &[u8],
    repetition: &str,
) -> i64 {
    if context == 0 {
        return 0;
    }

    let context = context as *mut CanvasRenderingContext2D;

    let context = unsafe { &mut *context };

    if let Some(image) = from_image_slice(bytes, width, height) {
        return Box::into_raw(Box::new(PaintStyle::new(Some(
            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                context.get_context().create_pattern(
                    image,
                    Repetition::try_from(repetition).unwrap_or(Repetition::NoRepeat),
                ),
            ),
        )))) as i64;
    }

    0
}

fn draw_image(
    context: i64,
    image_data: &[u8],
    width: f32,
    height: f32,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) -> bool {
    unsafe {
        if context == 0 {
            return false;
        }

        let context = context as *mut CanvasRenderingContext2D;

        let context = unsafe { &mut *context };

        if let Some(image) = from_image_slice(image_data, width as i32, height as i32) {
            context.make_current();
            let mut context = context.get_context_mut();
            context.draw_image_src_xywh_dst_xywh(
                &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
            );
            return true;
        }
        false
    }
}

pub fn canvas_native_context_draw_image_dx_dy_with_bytes(
    context: i64,
    bytes: &[u8],
    width: f32,
    height: f32,
    dx: f32,
    dy: f32,
) -> bool {
    return draw_image(
        context, bytes, width, height, 0.0, 0.0, width, height, dx, dy, width, height,
    );
}

pub fn canvas_native_context_draw_image_dx_dy_dw_dh_with_bytes(
    context: i64,
    bytes: &[u8],
    width: f32,
    height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) -> bool {
    draw_image(
        context, bytes, width, height, 0.0, 0.0, width, height, dx, dy, d_width, d_height,
    )
}

pub fn canvas_native_context_draw_image_with_bytes(
    context: i64,
    bytes: &[u8],
    width: f32,
    height: f32,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) -> bool {
    draw_image(
        context, bytes, width, height, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
    )
}
