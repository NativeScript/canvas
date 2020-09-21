#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

use std::ffi::{CStr, CString};
use std::io::Read;
use std::mem;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::string::String;

use android_logger::Config;
use jni::objects::JByteBuffer;
use jni::{
    objects::{JClass, JObject, JString, JValue},
    strings::JavaStr,
    sys::{jboolean, jint, jintArray, jlong, jstring},
    JNIEnv,
};
use jni_sys::{jbyteArray, jfloat, jfloatArray, jobject, JNI_FALSE, JNI_TRUE};
use libc::size_t;
use log::Level;
use log::{debug, info};
use skia_safe::{
    gpu::gl::Interface,
    gpu::{gl, BackendRenderTarget, Context, SurfaceOrigin},
    paint::{Cap, Join, Style},
    Color, ColorSpace, ColorType, Font, FontStyle, ISize, Paint, Path, PixelGeometry, Point, Rect,
    Surface, SurfaceProps, SurfacePropsFlags, Typeface,
};

use crate::android::bitmap::{
    AndroidBitmapInfo, AndroidBitmap_getInfo, AndroidBitmap_lockPixels, AndroidBitmap_unlockPixels,
    ANDROID_BITMAP_RESULT_SUCCESS,
};
use crate::common::{
    add_path_to_path_with_matrix, arc, arc_to, begin_path, bezier_curve_to, clear_rect, clip,
    clip_path_rule, clip_rule, close_path, create_image_data, create_matrix,
    create_path_2d_from_path_data, create_path_from_path, create_pattern, create_pattern_encoded,
    draw_image, draw_image_dw, draw_image_dw_encoded, draw_image_encoded, draw_image_sw,
    draw_image_sw_encoded, draw_rect, draw_text, ellipse, fill, fill_path_rule, fill_rule, flush,
    flush_custom_surface, free_matrix, free_path_2d, free_pattern, get_current_transform,
    get_direction, get_image_data, get_matrix, get_measure_text, image_asset_flip_y_in_place_owned,
    is_point_in_path, is_point_in_stroke, line_to, move_to, put_image_data, quadratic_curve_to,
    rect, reset_transform, restore, rotate, save, scale, set_current_transform, set_direction,
    set_fill_color, set_fill_color_rgba, set_fill_pattern, set_font, set_global_alpha,
    set_global_composite_operation, set_gradient_linear, set_gradient_radial,
    set_image_smoothing_enabled, set_image_smoothing_quality, set_line_cap, set_line_dash,
    set_line_dash_offset, set_line_join, set_line_width, set_matrix, set_miter_limit,
    set_pattern_transform, set_shadow_blur, set_shadow_color, set_shadow_offset_x,
    set_shadow_offset_y, set_stroke_color, set_stroke_color_rgba, set_stroke_pattern,
    set_text_align, set_transform, snapshot_canvas, stroke, stroke_path, to_byte_slice, to_data,
    to_data_url, transform, translate, CanvasCompositeOperationType, CanvasNative,
    CanvasNativeInitialState, CanvasTextMetrics, NativeByteArray, SurfaceKind, COLOR_TRANSPARENT,
};

#[no_mangle]
pub extern "system" fn JNI_OnLoad() -> jint {
    {
        android_logger::init_once(Config::default().with_min_level(Level::Debug));
        info!("Canvas Native library loaded");
    }

    jni::sys::JNI_VERSION_1_6
}

unsafe fn drawText(
    env: JNIEnv,
    canvas_native_ptr: jlong,
    text: JString,
    x: jfloat,
    y: jfloat,
    width: jfloat,
    is_stoke: bool,
) -> jlong {
    draw_text(
        canvas_native_ptr,
        env.get_string(text).unwrap().as_ptr() as _,
        x,
        y,
        width,
        is_stoke,
    )
}

unsafe fn drawRect(
    _env: JNIEnv,
    canvas_native_ptr: jlong,
    x: jfloat,
    y: jfloat,
    width: jfloat,
    height: jfloat,
    is_stoke: bool,
) -> jlong {
    draw_rect(canvas_native_ptr, x, y, width, height, is_stoke)
}

fn init(
    cpu: bool,
    buffer_id: jint,
    width: jint,
    height: jint,
    scale: jfloat,
    direction: String,
) -> CanvasNative {
    let mut stroke_paint = Paint::default();
    stroke_paint.set_anti_alias(true);
    stroke_paint.set_color(Color::BLACK);
    stroke_paint.set_stroke_width(1.0);
    stroke_paint.set_style(Style::Stroke);
    stroke_paint.set_stroke_join(Join::Miter);
    stroke_paint.set_stroke_cap(Cap::Butt);
    stroke_paint.set_stroke_miter(10.0);
    let mut fill_paint = Paint::default();
    fill_paint.set_anti_alias(true);
    fill_paint.set_color(Color::BLACK);
    fill_paint.set_style(Style::Fill);
    fill_paint.set_stroke_miter(10.0);
    fill_paint.set_stroke_join(Join::Miter);
    fill_paint.set_stroke_cap(Cap::Butt);
    // "10px sans-serif" Default
    let default_type_face =
        Typeface::from_name("sans-serif", FontStyle::default()).unwrap_or(Typeface::default());
    let font = Font::from_typeface(&default_type_face, Some(10.0));
    let surface;
    let surface_kind;
    let mut ctx = None;
    if !cpu {
        let interface = gl::Interface::new_native();
        let context = Context::new_gl(interface.unwrap());
        let mut internal_ctx = context.unwrap();
        let mut frame_buffer = gl::FramebufferInfo::from_fboid(buffer_id as u32);
        frame_buffer.format = 0x8058; //GR_GL_RGBA8 (https://github.com/google/skia/blob/master/src/gpu/gl/GrGLDefines.h#L511)
        let target =
            BackendRenderTarget::new_gl((width as i32, height as i32), Some(0), 8, frame_buffer);
        let surface_props = SurfaceProps::new(SurfacePropsFlags::default(), PixelGeometry::Unknown);
        let color_space = ColorSpace::new_srgb();
        let surface_holder = Surface::from_backend_render_target(
            &mut internal_ctx,
            &target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            Some(color_space),
            Some(&surface_props),
        );
        ctx = Some(internal_ctx);
        surface = surface_holder.unwrap();
        surface_kind = SurfaceKind::GPU;
    } else {
        surface = Surface::new_raster_n32_premul(ISize::new(width, height)).unwrap();
        surface_kind = SurfaceKind::CPU;
    }

    let native_canvas = CanvasNative {
        surface,
        stroke_paint,
        fill_paint,
        path: Path::new(),
        context: ctx,
        font,
        state: Vec::new(),
        line_dash_offset: 0.0,
        shadow_blur: 0.0,
        shadow_color: COLOR_TRANSPARENT as u32,
        shadow_offset_x: 0.0,
        shadow_offset_y: 0.0,
        image_smoothing_enabled: true,
        image_smoothing_quality: "low".to_string(),
        device_scale: scale,
        text_align: "start".to_string(),
        ios: 0,
        global_composite_operation: CanvasCompositeOperationType::SourceOver,
        line_cap: "butt".to_string(),
        line_join: "miter".to_string(),
        direction: direction.clone(),
        miter_limit: 10.0,
        surface_kind,
        initial_state: CanvasNativeInitialState {
            direction: direction.clone(),
            device_scale: scale,
            surface_kind,
        },
    };

    native_canvas
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_FileReader_nativeRead(
    env: JNIEnv,
    _: JClass,
    file: JString,
) -> jbyteArray {
    let empty = env.new_string("").unwrap();
    let value = env
        .get_string(file)
        .unwrap_or(JavaStr::from_env(&env, empty).unwrap());
    let real_file = std::fs::File::open(std::path::Path::new(value.to_str().unwrap()));
    let result = match real_file {
        Ok(mut file) => {
            let len: usize = match file.metadata() {
                Ok(len) => len.len(),
                Err(_) => 0,
            } as usize;
            let mut data = vec![0u8; len];
            let _ = file.read_to_end(&mut data);
            data
        }
        Err(_) => Vec::new(),
    };

    env.byte_array_from_slice(result.as_slice()).unwrap()
}

#[allow(unused)]
pub(crate) unsafe fn flip_in_place_3d(
    env: JNIEnv,
    pixels: jbyteArray,
    width: jint,
    height: jint,
    depth: jint,
) {
    let size = env.get_array_length(pixels).unwrap_or(0);
    let mut array = vec![0i8; size as usize];
    let _ = env.get_byte_array_region(pixels, 0, array.as_mut_slice());
    let mut data = array.as_mut_ptr();
    for _z in 0..depth {
        flip_in_place_native(data, width, height);
        data = data.offset((4 * width * height) as isize);
    }
    let _didWrite = env.set_byte_array_region(pixels, 0, array.as_mut_slice());
}

#[allow(unused)]
pub(crate) unsafe fn flip_in_place_native(pixels: *mut i8, width: i32, height: i32) {
    image_asset_flip_y_in_place_owned(
        width as u32,
        height as u32,
        pixels as *mut u8,
        (width * height * 4) as usize,
    );
}

#[allow(unused)]
pub(crate) unsafe fn flip_in_place(env: JNIEnv, pixels: jbyteArray, width: jint, height: jint) {
    let line_size = width * 4;
    let mut line_buffer_storage = vec![0i8; line_size as usize];
    let mut _line_buffer = line_buffer_storage.as_mut_ptr();
    let storage_size = env.get_array_length(pixels).unwrap_or(0);
    let mut data_storage = vec![0i8; storage_size as usize];
    let _ = env.get_byte_array_region(pixels, 0, data_storage.as_mut_slice());
    let data = data_storage.as_mut_ptr();

    image_asset_flip_y_in_place_owned(
        width as u32,
        height as u32,
        data as *mut u8,
        storage_size as usize,
    );

    let flipped = std::slice::from_raw_parts(data_storage.as_ptr(), data_storage.len());
    let _didWrite = env.set_byte_array_region(pixels, 0, flipped);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasView_nativeDestroy(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
) {
    let canvas: Box<CanvasNative> = Box::from_raw(canvas_native_ptr as *mut _);
    let mut ctx = canvas.context.unwrap();
    ctx.abandon();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasView_nativeInit(
    env: JNIEnv,
    _: JClass,
    cpu: jboolean,
    buffer_id: jint,
    width: jint,
    height: jint,
    scale: jfloat,
    direction: JString,
) -> jlong {
    let dir = env.get_string(direction).unwrap();
    let dir = dir.to_str().unwrap();
    debug!("nativeInit");
    let canvas = Box::new(init(
        cpu == JNI_TRUE,
        buffer_id,
        width,
        height,
        scale,
        dir.to_owned(),
    ));
    canvas.into_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasView_nativeResize(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    buffer_id: jint,
    width: jint,
    height: jint,
    _scale: jfloat,
) -> jlong {
    if canvas_native_ptr == 0 {
        return canvas_native_ptr;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_native_ptr);
    let interface = Interface::new_native();
    let context = Context::new_gl(interface.unwrap());
    let mut internal_ctx = context.unwrap();
    let mut frame_buffer = gl::FramebufferInfo::from_fboid(buffer_id as u32);
    frame_buffer.format = 0x8058; //GR_GL_RGBA8 (https://github.com/google/skia/blob/master/src/gpu/gl/GrGLDefines.h#L511)
    let target =
        BackendRenderTarget::new_gl((width as i32, height as i32), Some(0), 0, frame_buffer);
    let surface_props = SurfaceProps::new(SurfacePropsFlags::default(), PixelGeometry::Unknown);
    let color_space = ColorSpace::new_srgb();
    let surface_holder = Surface::from_backend_render_target(
        &mut internal_ctx,
        &target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        Some(color_space),
        Some(&surface_props),
    );
    let surface = surface_holder.unwrap();
    canvas_native.context = Some(internal_ctx);
    canvas_native.surface = surface;
    canvas_native.reset_state();
    canvas_native.into_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasView_nativeRecreate(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    buffer_id: jint,
    width: jint,
    height: jint,
    _scale: jfloat,
) -> jlong {
    if canvas_native_ptr < 0 {
        return canvas_native_ptr;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_native_ptr);
    // let mut ctx = canvas_native.context.unwrap();
    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();
    canvas.flush();
    // surface.flush();
    let ss = surface.image_snapshot();
    let interface = gl::Interface::new_native();
    let mut ctx = Context::new_gl(interface.unwrap()).unwrap();
    let mut frame_buffer = gl::FramebufferInfo::from_fboid(buffer_id as u32);
    frame_buffer.format = 0x8058; //GR_GL_RGBA8 (https://github.com/google/skia/blob/master/src/gpu/gl/GrGLDefines.h#L511)
    let target = BackendRenderTarget::new_gl((width, height), Some(0), 0, frame_buffer);
    let surface_props = SurfaceProps::new(SurfacePropsFlags::default(), PixelGeometry::Unknown);
    let color_space = ColorSpace::new_srgb();
    let surface_holder = Surface::from_backend_render_target(
        &mut ctx,
        &target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        Some(color_space),
        Some(&surface_props),
    );
    let mut new_surface = surface_holder.unwrap();
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    new_surface
        .canvas()
        .draw_image(ss, Point::new(0f32, 0f32), Some(&paint));
    paint.set_color(Color::RED);
    new_surface
        .canvas()
        .draw_rect(Rect::new(0f32, 0f32, width as f32, height as f32), &paint);
    new_surface.canvas().flush();
    new_surface.flush_and_submit();
    canvas_native.surface = new_surface;
    canvas_native.context = Some(ctx);
    canvas_native.into_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasView_nativeFlush(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
) -> jlong {
    flush(canvas_native_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasView_nativeCpuFlush(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    view: JObject,
) -> jlong {
    let native_interface = env.get_native_interface();
    let bitmap_to_draw = view.into_inner();
    let bitmapInfo_to_draw = Box::into_raw(Box::new(AndroidBitmapInfo::default()));

    if AndroidBitmap_getInfo(native_interface, bitmap_to_draw, bitmapInfo_to_draw)
        < ANDROID_BITMAP_RESULT_SUCCESS
    {
        debug!("Get Bitmap Info Failed");
        return 0;
    }
    let info_to_draw: Box<AndroidBitmapInfo> = Box::from_raw(bitmapInfo_to_draw);
    let mut _dstPixelsToDraw = null_mut() as *mut c_void;
    let dstPixelsToDraw: *mut *mut c_void = &mut _dstPixelsToDraw;
    if AndroidBitmap_lockPixels(native_interface, bitmap_to_draw, dstPixelsToDraw)
        < ANDROID_BITMAP_RESULT_SUCCESS
    {
        debug!("Get Bitmap Lock Failed");
        return 0;
    }
    let ratio_to_draw = mem::size_of_val(&dstPixelsToDraw) / mem::size_of::<u8>();
    let length_to_draw =
        ((info_to_draw.width * info_to_draw.height) * ratio_to_draw as u32) as usize;
    let ptr_to_draw = _dstPixelsToDraw as *mut _;
    let pixels_to_draw: &mut [u8] =
        std::slice::from_raw_parts_mut(ptr_to_draw, length_to_draw as usize);

    let ptr = flush_custom_surface(
        canvas_native_ptr,
        info_to_draw.width as i32,
        info_to_draw.height as i32,
        pixels_to_draw,
    );
    if AndroidBitmap_unlockPixels(native_interface, bitmap_to_draw) < ANDROID_BITMAP_RESULT_SUCCESS
    {
        debug!("Unlock Bitmap Failed");
    }
    return ptr;
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasView_nativeToData(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
) -> jbyteArray {
    let mut data = to_data(canvas_native_ptr);
    env.byte_array_from_slice(data.as_mut_slice()).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasView_nativeSnapshotCanvas(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
) -> jbyteArray {
    let array = NativeByteArray::from_raw(snapshot_canvas(canvas_native_ptr));
    let jArray = env.new_byte_array(array.length as i32).unwrap();
    let slice = std::slice::from_raw_parts_mut(array.array as *mut i8, array.length);
    let _ = env.set_byte_array_region(jArray, 0, slice);
    jArray
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasView_nativeSnapshotCanvasBuffer(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
) -> jobject {
    let array = NativeByteArray::from_raw(snapshot_canvas(canvas_native_ptr));
    let slice = std::slice::from_raw_parts_mut(array.array, array.length);
    env.new_direct_byte_buffer(slice).unwrap().into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasView_nativeToDataUrl(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    format: JString,
    quality: jfloat,
) -> jstring {
    let default = env.new_string("image/png").unwrap();
    let javaStr = JavaStr::from_env(&env, default);
    let format = env.get_string(format).unwrap_or(javaStr.unwrap());
    let result = to_data_url(
        canvas_native_ptr,
        format.as_ptr(),
        (quality * 100f32) as i32,
    );
    let string = CStr::from_ptr(result).to_str();
    env.new_string(string.unwrap()).unwrap().into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetDirection(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    direction: JString,
) -> jlong {
    set_direction(
        canvas_native_ptr,
        env.get_string(direction).unwrap().get_raw(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeGetDirection(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
) -> jstring {
    let direction = get_direction(canvas_native_ptr);
    let direction = CStr::from_ptr(direction).to_str();
    let direction = direction.unwrap();
    env.new_string(direction).unwrap().into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetMiterLimit(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    limit: f32,
) -> jlong {
    set_miter_limit(canvas_native_ptr, limit)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeArc(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    x: jfloat,
    y: jfloat,
    radius: jfloat,
    start_angle: jfloat,
    end_angle: jfloat,
    anticlockwise: jboolean,
) -> jlong {
    arc(
        canvas_native_ptr,
        true,
        x,
        y,
        radius,
        start_angle,
        end_angle,
        anticlockwise == JNI_TRUE,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeArcTo(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    x1: jfloat,
    y1: jfloat,
    x2: jfloat,
    y2: jfloat,
    radius: jfloat,
) -> jlong {
    arc_to(canvas_native_ptr, true, x1, y1, x2, y2, radius)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeBeginPath(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
) -> jlong {
    begin_path(canvas_native_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeBezierCurveTo(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    cp1x: jfloat,
    cp1y: jfloat,
    cp2x: jfloat,
    cp2y: jfloat,
    x: jfloat,
    y: jfloat,
) -> jlong {
    bezier_curve_to(canvas_native_ptr, true, cp1x, cp1y, cp2x, cp2y, x, y)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeClearRect(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    x: jfloat,
    y: jfloat,
    width: jfloat,
    height: jfloat,
) -> jlong {
    clear_rect(canvas_native_ptr, x, y, width, height)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeClipPathRule(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    path: jlong,
    fill_rule: JString,
) -> jlong {
    clip_path_rule(
        canvas_native_ptr,
        path,
        env.get_string(fill_rule).unwrap().as_ptr() as _,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeClip(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
) -> jlong {
    clip(canvas_native_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeClipRule(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    fill_rule: JString,
) -> jlong {
    clip_rule(
        canvas_native_ptr,
        env.get_string(fill_rule).unwrap().as_ptr() as _,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeClosePath(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
) -> jlong {
    close_path(canvas_native_ptr, true)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_Pattern_nativeFree(
    _env: JNIEnv,
    _: JClass,
    pattern: jlong,
) {
    free_pattern(pattern)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_Pattern_nativeCreatePatternRaw(
    env: JNIEnv,
    _: JClass,
    image: jbyteArray,
    original_width: jint,
    original_height: jint,
    repetition: JString,
) -> jlong {
    let length = env.get_array_length(image).unwrap_or(0);
    let mut pixels_to_draw = vec![0i8; length as usize];
    let _ = env.get_byte_array_region(image, 0, pixels_to_draw.as_mut_slice());
    let buf = to_byte_slice(pixels_to_draw.as_mut_slice());
    let image_pixels_ptr = buf.as_mut_ptr();
    let default = env.new_string("repeat").unwrap();
    let rep = env
        .get_string(repetition)
        .unwrap_or(JavaStr::from_env(&env, default).unwrap());
    create_pattern(
        image_pixels_ptr,
        pixels_to_draw.len(),
        original_width,
        original_height,
        rep.get_raw(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_Pattern_nativeCreatePatternRawBuffer(
    env: JNIEnv,
    _: JClass,
    image: JByteBuffer,
    original_width: jint,
    original_height: jint,
    repetition: JString,
) -> jlong {
    let mut empty = [0u8; 0];
    let buf = env.get_direct_buffer_address(image).unwrap_or(&mut empty);
    let image_pixels_ptr = buf.as_mut_ptr();
    let default = env.new_string("repeat").unwrap();
    let rep = env
        .get_string(repetition)
        .unwrap_or(JavaStr::from_env(&env, default).unwrap());
    create_pattern(
        image_pixels_ptr,
        buf.len(),
        original_width,
        original_height,
        rep.get_raw(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_Pattern_nativeCreatePatternCanvas(
    env: JNIEnv,
    _: JClass,
    image: jbyteArray,
    repetition: JString,
) -> jlong {
    let length = env.get_array_length(image).unwrap_or(0);
    let mut pixels_to_draw = vec![0i8; length as usize];
    let _ = env.get_byte_array_region(image, 0, pixels_to_draw.as_mut_slice());
    let buf = to_byte_slice(pixels_to_draw.as_mut_slice());
    let image_pixels_ptr = buf.as_mut_ptr();
    let default = env.new_string("repeat").unwrap();
    let rep = env
        .get_string(repetition)
        .unwrap_or(JavaStr::from_env(&env, default).unwrap());
    create_pattern_encoded(image_pixels_ptr, pixels_to_draw.len(), rep.get_raw())
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_Pattern_nativeCreatePattern(
    env: JNIEnv,
    _: JClass,
    image: JObject,
    repetition: JString,
) -> jlong {
    let default = env.new_string("repeat").unwrap();
    let rep = env
        .get_string(repetition)
        .unwrap_or(JavaStr::from_env(&env, default).unwrap());

    let native_interface = env.get_native_interface();
    let bitmap_to_draw = image.into_inner();
    let bitmapInfo_to_draw = Box::into_raw(Box::new(AndroidBitmapInfo::default()));

    if AndroidBitmap_getInfo(native_interface, bitmap_to_draw, bitmapInfo_to_draw)
        < ANDROID_BITMAP_RESULT_SUCCESS
    {
        debug!("Get Bitmap Info Failed");
        return 0;
    }
    let info_to_draw = Box::from_raw(bitmapInfo_to_draw);
    let mut _dstPixelsToDraw = null_mut() as *mut c_void;
    let dstPixelsToDraw: *mut *mut c_void = &mut _dstPixelsToDraw;
    if AndroidBitmap_lockPixels(native_interface, bitmap_to_draw, dstPixelsToDraw)
        < ANDROID_BITMAP_RESULT_SUCCESS
    {
        debug!("Get Bitmap Lock Failed");
        return 0;
    }
    let ratio_to_draw = mem::size_of_val(&dstPixelsToDraw) / mem::size_of::<u8>();
    let length_to_draw =
        ((info_to_draw.width * info_to_draw.height) * ratio_to_draw as u32) as usize;
    let ptr_to_draw = _dstPixelsToDraw as *mut _;
    let pixels_to_draw: &mut [u8] =
        std::slice::from_raw_parts_mut(ptr_to_draw, length_to_draw as usize);

    let image_pixels_ptr = pixels_to_draw.as_mut_ptr();

    let ptr = create_pattern(
        image_pixels_ptr,
        pixels_to_draw.len(),
        info_to_draw.width as _,
        info_to_draw.height as _,
        rep.get_raw(),
    );

    if AndroidBitmap_unlockPixels(native_interface, bitmap_to_draw) < ANDROID_BITMAP_RESULT_SUCCESS
    {
        debug!("Unlock Bitmap Failed");
    }
    return ptr;
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_Pattern_nativeFreePattern(
    _: JNIEnv,
    _: JClass,
    pattern: jlong,
) {
    free_pattern(pattern)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_Pattern_nativeSetPatternTransform(
    _: JNIEnv,
    _: JClass,
    pattern: jlong,
    matrix: jlong,
) -> jlong {
    set_pattern_transform(pattern, matrix)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetFillPattern(
    _: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    pattern: jlong,
) -> jlong {
    set_fill_pattern(canvas_native_ptr, pattern)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetStrokePattern(
    _: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    pattern: jlong,
) -> jlong {
    set_stroke_pattern(canvas_native_ptr, pattern)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetFillColorRgba(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
) -> jlong {
    set_fill_color_rgba(canvas_native_ptr, red, green, blue, alpha)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetFillColor(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    color: jint,
) -> jlong {
    set_fill_color(canvas_native_ptr, color as u32)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetStrokeColorRgba(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
) -> jlong {
    set_stroke_color_rgba(canvas_native_ptr, red, green, blue, alpha)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetStrokeColor(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    color: jint,
) -> jlong {
    set_stroke_color(canvas_native_ptr, color as u32)
}

// set from createLinearGradient()

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetFillGradientLinear(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    x0: jfloat,
    y0: jfloat,
    x1: jfloat,
    y1: jfloat,
    colors: jintArray,
    positions: jfloatArray,
) -> jlong {
    let colors_len = env.get_array_length(colors).unwrap_or(0) as usize;
    let positions_len = env.get_array_length(positions).unwrap_or(0) as usize;
    let mut colors_array = vec![0i32; colors_len];
    let _ = env
        .get_int_array_region(colors, 0, colors_array.as_mut_slice())
        .unwrap();
    let mut positions_array = vec![0f32; positions_len];
    let _ = env.get_float_array_region(positions, 0, positions_array.as_mut_slice());
    set_gradient_linear(
        canvas_native_ptr,
        x0,
        y0,
        x1,
        y1,
        colors_array.len(),
        colors_array.as_mut_ptr() as _,
        positions_array.len(),
        positions_array.as_mut_ptr() as _,
        false,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetStrokeGradientLinear(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    x0: jfloat,
    y0: jfloat,
    x1: jfloat,
    y1: jfloat,
    colors: jintArray,
    positions: jfloatArray,
) -> jlong {
    let colors_len = env.get_array_length(colors).unwrap_or(0) as usize;
    let positions_len = env.get_array_length(positions).unwrap_or(0) as usize;
    let mut colors_array = vec![0i32; colors_len];
    let _ = env
        .get_int_array_region(colors, 0, colors_array.as_mut_slice())
        .unwrap();
    let mut positions_array = vec![0f32; positions_len];
    let _ = env.get_float_array_region(positions, 0, positions_array.as_mut_slice());
    set_gradient_linear(
        canvas_native_ptr,
        x0,
        y0,
        x1,
        y1,
        colors_array.len(),
        colors_array.as_mut_ptr() as _,
        positions_array.len(),
        positions_array.as_mut_ptr() as _,
        true,
    )
}

// set from createRadialGradient()

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetFillGradientRadial(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    x0: jfloat,
    y0: jfloat,
    radius_0: jfloat,
    x1: jfloat,
    y1: jfloat,
    radius_1: jfloat,
    colors: jintArray,
    positions: jfloatArray,
) -> jlong {
    let colors_len = env.get_array_length(colors).unwrap_or(0) as usize;
    let positions_len = env.get_array_length(positions).unwrap_or(0) as usize;
    let mut colors_array = vec![0i32; colors_len];
    let _ = env
        .get_int_array_region(colors, 0, colors_array.as_mut_slice())
        .unwrap();
    let mut positions_array = vec![0f32; positions_len];
    let _ = env.get_float_array_region(positions, 0, positions_array.as_mut_slice());
    set_gradient_radial(
        canvas_native_ptr,
        x0,
        y0,
        radius_0,
        x1,
        y1,
        radius_1,
        colors_array.len(),
        colors_array.as_mut_ptr() as _,
        positions_array.len(),
        positions_array.as_mut_ptr() as _,
        false,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetStrokeGradientRadial(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    x0: jfloat,
    y0: jfloat,
    radius_0: jfloat,
    x1: jfloat,
    y1: jfloat,
    radius_1: jfloat,
    colors: jintArray,
    positions: jfloatArray,
) -> jlong {
    let colors_len = env.get_array_length(colors).unwrap_or(0) as usize;
    let positions_len = env.get_array_length(positions).unwrap_or(0) as usize;
    let mut colors_array = vec![0i32; colors_len];
    let _ = env
        .get_int_array_region(colors, 0, colors_array.as_mut_slice())
        .unwrap();
    let mut positions_array = vec![0f32; positions_len];
    let _ = env.get_float_array_region(positions, 0, positions_array.as_mut_slice());
    set_gradient_radial(
        canvas_native_ptr,
        x0,
        y0,
        radius_0,
        x1,
        y1,
        radius_1,
        colors_array.len(),
        colors_array.as_mut_ptr() as _,
        positions_array.len(),
        positions_array.as_mut_ptr() as _,
        true,
    )
}

// drawImage()

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeDrawImageCanvas(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    canvas_to_draw: jbyteArray,
    width: jint,
    height: jint,
    dx: jfloat,
    dy: jfloat,
) -> jlong {
    let len = env.get_array_length(canvas_to_draw).unwrap_or(0);
    let mut data = vec![0i8; len as usize];
    let _ = env.get_byte_array_region(canvas_to_draw, 0, data.as_mut_slice());
    draw_image_encoded(
        canvas_native_ptr,
        data.as_ptr() as *const u8,
        data.len(),
        width,
        height,
        dx,
        dy,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeDrawImageRaw(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    _image: jbyteArray,
    original_width: jint,
    original_height: jint,
    dx: jfloat,
    dy: jfloat,
) -> jlong {
    let length = env.get_array_length(_image).unwrap_or(0);
    let mut pixels_to_draw = vec![0i8; length as usize];
    let _ = env.get_byte_array_region(_image, 0, pixels_to_draw.as_mut_slice());
    let buf = to_byte_slice(pixels_to_draw.as_mut_slice());
    let image_pixels_ptr = buf.as_mut_ptr();
    let ptr = draw_image(
        canvas_native_ptr,
        image_pixels_ptr,
        pixels_to_draw.len(),
        original_width,
        original_height,
        dx,
        dy,
    );
    return ptr;
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeDrawImage(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    _image: JObject,
    dx: jfloat,
    dy: jfloat,
) -> jlong {
    let native_interface = env.get_native_interface();
    let bitmap_to_draw = _image.into_inner();
    let bitmapInfo_to_draw = Box::into_raw(Box::new(AndroidBitmapInfo::default()));

    if AndroidBitmap_getInfo(native_interface, bitmap_to_draw, bitmapInfo_to_draw)
        < ANDROID_BITMAP_RESULT_SUCCESS
    {
        debug!("Get Bitmap Info Failed");
        return 0;
    }
    let info_to_draw = Box::from_raw(bitmapInfo_to_draw);
    let mut _dstPixelsToDraw = null_mut() as *mut c_void;
    let dstPixelsToDraw: *mut *mut c_void = &mut _dstPixelsToDraw;
    if AndroidBitmap_lockPixels(native_interface, bitmap_to_draw, dstPixelsToDraw)
        < ANDROID_BITMAP_RESULT_SUCCESS
    {
        debug!("Get Bitmap Lock Failed");
        return 0;
    }
    let ratio_to_draw = mem::size_of_val(&dstPixelsToDraw) / mem::size_of::<u8>();
    let length_to_draw =
        ((info_to_draw.width * info_to_draw.height) * ratio_to_draw as u32) as usize;
    let ptr_to_draw = _dstPixelsToDraw as *mut _;
    let pixels_to_draw: &mut [u8] =
        std::slice::from_raw_parts_mut(ptr_to_draw, length_to_draw as usize);

    let image_pixels_ptr = pixels_to_draw.as_mut_ptr();
    let ptr = draw_image(
        canvas_native_ptr,
        image_pixels_ptr,
        pixels_to_draw.len(),
        info_to_draw.width as _,
        info_to_draw.height as _,
        dx,
        dy,
    );
    if AndroidBitmap_unlockPixels(native_interface, bitmap_to_draw) < ANDROID_BITMAP_RESULT_SUCCESS
    {
        debug!("Unlock Bitmap Failed");
    }
    return ptr;
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeDrawImageCanvasDw(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    canvas_to_draw: jbyteArray,
    width: jint,
    height: jint,
    dx: jfloat,
    dy: jfloat,
    d_width: jfloat,
    d_height: jfloat,
) -> jlong {
    let len = env.get_array_length(canvas_to_draw).unwrap_or(0);
    let mut data = vec![0i8; len as usize];
    let _ = env.get_byte_array_region(canvas_to_draw, 0, data.as_mut_slice());
    draw_image_dw_encoded(
        canvas_native_ptr,
        data.as_ptr() as *const u8,
        data.len(),
        width,
        height,
        dx,
        dy,
        d_width,
        d_height,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeDrawImageDwRaw(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    _image: jbyteArray,
    original_width: jint,
    original_height: jint,
    dx: jfloat,
    dy: jfloat,
    d_width: jfloat,
    d_height: jfloat,
) -> jlong {
    let length = env.get_array_length(_image).unwrap_or(0);
    let mut pixels_to_draw = vec![0i8; length as usize];
    let _ = env.get_byte_array_region(_image, 0, pixels_to_draw.as_mut_slice());
    let buf = to_byte_slice(pixels_to_draw.as_mut_slice());
    let image_pixels_ptr = buf.as_mut_ptr();
    draw_image_dw(
        canvas_native_ptr,
        image_pixels_ptr,
        pixels_to_draw.len(),
        original_width,
        original_height,
        dx,
        dy,
        d_width,
        d_height,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeDrawImageDw(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    _image: JObject,
    dx: jfloat,
    dy: jfloat,
    d_width: jfloat,
    d_height: jfloat,
) -> jlong {
    let native_interface = env.get_native_interface();
    let bitmap_to_draw = _image.into_inner();
    let bitmapInfo_to_draw = Box::into_raw(Box::new(AndroidBitmapInfo::default()));

    if AndroidBitmap_getInfo(native_interface, bitmap_to_draw, bitmapInfo_to_draw)
        < ANDROID_BITMAP_RESULT_SUCCESS
    {
        debug!("Get Bitmap Info Failed Dw");
        return 0;
    }
    let info_to_draw = Box::from_raw(bitmapInfo_to_draw);
    let mut _dstPixelsToDraw = null_mut() as *mut c_void;
    let dstPixelsToDraw: *mut *mut c_void = &mut _dstPixelsToDraw;
    if AndroidBitmap_lockPixels(native_interface, bitmap_to_draw, dstPixelsToDraw)
        < ANDROID_BITMAP_RESULT_SUCCESS
    {
        debug!("Get Bitmap Lock Failed Dw");
        return 0;
    }
    let ratio_to_draw = mem::size_of_val(&dstPixelsToDraw) / mem::size_of::<u8>();
    let length_to_draw =
        ((info_to_draw.width * info_to_draw.height) * ratio_to_draw as u32) as usize;
    let ptr_to_draw = _dstPixelsToDraw as *mut _;
    let pixels_to_draw: &mut [u8] =
        std::slice::from_raw_parts_mut(ptr_to_draw, length_to_draw as usize);

    let image_pixels_ptr = pixels_to_draw.as_mut_ptr();
    let ptr = draw_image_dw(
        canvas_native_ptr,
        image_pixels_ptr,
        pixels_to_draw.len(),
        info_to_draw.width as _,
        info_to_draw.height as _,
        dx,
        dy,
        d_width,
        d_height,
    );
    AndroidBitmap_unlockPixels(native_interface, bitmap_to_draw);
    return ptr;
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeDrawImageCanvasSw(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    canvas_to_draw: jbyteArray,
    width: jint,
    height: jint,
    sx: jfloat,
    sy: jfloat,
    s_width: jfloat,
    s_height: jfloat,
    dx: jfloat,
    dy: jfloat,
    d_width: jfloat,
    d_height: jfloat,
) -> jlong {
    let len = env.get_array_length(canvas_to_draw).unwrap_or(0);
    let mut data = vec![0i8; len as usize];
    let _ = env.get_byte_array_region(canvas_to_draw, 0, data.as_mut_slice());
    draw_image_sw_encoded(
        canvas_native_ptr,
        data.as_ptr() as *const u8,
        data.len(),
        width,
        height,
        sx,
        sy,
        s_width,
        s_height,
        dx,
        dy,
        d_width,
        d_height,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeDrawImageSwRaw(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    _image: jbyteArray,
    original_width: jint,
    original_height: jint,
    sx: jfloat,
    sy: jfloat,
    s_width: jfloat,
    s_height: jfloat,
    dx: jfloat,
    dy: jfloat,
    d_width: jfloat,
    d_height: jfloat,
) -> jlong {
    let length = env.get_array_length(_image).unwrap_or(0);
    let mut pixels_to_draw = vec![0i8; length as usize];
    let _ = env.get_byte_array_region(_image, 0, pixels_to_draw.as_mut_slice());
    let buf = to_byte_slice(pixels_to_draw.as_mut_slice());
    let image_pixels_ptr = buf.as_mut_ptr();

    draw_image_sw(
        canvas_native_ptr,
        image_pixels_ptr,
        pixels_to_draw.len(),
        original_width,
        original_height,
        sx,
        sy,
        s_width,
        s_height,
        dx,
        dy,
        d_width,
        d_height,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeDrawImageSw(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    _image: JObject,
    sx: jfloat,
    sy: jfloat,
    s_width: jfloat,
    s_height: jfloat,
    dx: jfloat,
    dy: jfloat,
    d_width: jfloat,
    d_height: jfloat,
) -> jlong {
    let native_interface = env.get_native_interface();
    let bitmap_to_draw = _image.into_inner();
    let bitmapInfo_to_draw = Box::into_raw(Box::new(AndroidBitmapInfo::default()));

    if AndroidBitmap_getInfo(native_interface, bitmap_to_draw, bitmapInfo_to_draw)
        < ANDROID_BITMAP_RESULT_SUCCESS
    {
        debug!("Get Bitmap Info Failed Sw");
        return 0;
    }
    let info_to_draw = Box::from_raw(bitmapInfo_to_draw);
    let mut _dstPixelsToDraw = null_mut() as *mut c_void;
    let dstPixelsToDraw: *mut *mut c_void = &mut _dstPixelsToDraw;
    if AndroidBitmap_lockPixels(native_interface, bitmap_to_draw, dstPixelsToDraw)
        < ANDROID_BITMAP_RESULT_SUCCESS
    {
        debug!("Get Bitmap Lock Failed Sw");
        return 0;
    }
    let ratio_to_draw = mem::size_of_val(&dstPixelsToDraw) / mem::size_of::<u8>();
    let length_to_draw =
        ((info_to_draw.width * info_to_draw.height) * ratio_to_draw as u32) as usize;
    let ptr_to_draw = _dstPixelsToDraw as *mut _;
    let pixels_to_draw: &mut [u8] =
        std::slice::from_raw_parts_mut(ptr_to_draw, length_to_draw as usize);
    let image_pixels_ptr = pixels_to_draw.as_mut_ptr();
    let ptr = draw_image_sw(
        canvas_native_ptr,
        image_pixels_ptr,
        pixels_to_draw.len(),
        info_to_draw.width as _,
        info_to_draw.height as _,
        sx,
        sy,
        s_width,
        s_height,
        dx,
        dy,
        d_width,
        d_height,
    );
    AndroidBitmap_unlockPixels(native_interface, bitmap_to_draw);

    return ptr;
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeEllipse(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    x: jfloat,
    y: jfloat,
    radius_x: jfloat,
    radius_y: jfloat,
    rotation: jfloat,
    start_angle: jfloat,
    end_angle: jfloat,
    anticlockwise: jboolean,
) -> jlong {
    ellipse(
        canvas_native_ptr,
        true,
        x,
        y,
        radius_x,
        radius_y,
        rotation,
        start_angle,
        end_angle,
        anticlockwise == JNI_TRUE,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeFillPathRule(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    path: jlong,
    rule: JString,
) -> jlong {
    fill_path_rule(
        canvas_native_ptr,
        path,
        env.get_string(rule).unwrap().as_ptr() as _,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeFillRule(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    rule: JString,
) -> jlong {
    fill_rule(
        canvas_native_ptr,
        env.get_string(rule).unwrap().as_ptr() as _,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeFill(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
) -> jlong {
    debug!("nativeFill b4 {:?}", canvas_native_ptr);
    let val = fill(canvas_native_ptr);
    debug!("nativeFill after {:?}", canvas_native_ptr);
    val
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeFillRect(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    x: jfloat,
    y: jfloat,
    width: jfloat,
    height: jfloat,
) -> jlong {
    drawRect(env, canvas_native_ptr, x, y, width, height, false)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeFillText(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    text: JString,
    x: jfloat,
    y: jfloat,
    width: jfloat,
) -> jlong {
    drawText(env, canvas_native_ptr, text, x, y, width, false)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeLineTo(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    x: jfloat,
    y: jfloat,
) -> jlong {
    line_to(canvas_native_ptr, true, x, y)
}

static CANVAS_TEXT_METRICS: &str = "com/github/triniwiz/canvas/CanvasTextMetrics";

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeMeasureText(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    text: JString,
) -> jobject {
    let object = env.new_object(CANVAS_TEXT_METRICS, "()V", &[]);
    let result = object.unwrap();
    let txt = env.get_string(text).unwrap();
    let measurement =
        CanvasTextMetrics::from_raw(get_measure_text(canvas_native_ptr, txt.as_ptr() as _));
    let value = JValue::from(measurement.width);
    let _ = env.set_field(result, "width", "F", value);
    result.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeMoveTo(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    x: jfloat,
    y: jfloat,
) -> jlong {
    move_to(canvas_native_ptr, true, x, y)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeQuadraticCurveTo(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    cpx: jfloat,
    cpy: jfloat,
    x: jfloat,
    y: jfloat,
) -> jlong {
    quadratic_curve_to(canvas_native_ptr, true, cpx, cpy, x, y)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeRect(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    x: jfloat,
    y: jfloat,
    width: jfloat,
    height: jfloat,
) -> jlong {
    debug!("nativeRect b4 {:?}", canvas_native_ptr);
    let val = rect(canvas_native_ptr, true, x, y, width, height);
    debug!("nativeRect after {:?}", canvas_native_ptr);
    val
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeRestore(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
) -> jlong {
    restore(canvas_native_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeRotate(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    angle: jfloat,
) -> jlong {
    rotate(canvas_native_ptr, angle)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSave(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
) -> jlong {
    save(canvas_native_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeScale(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    x: jfloat,
    y: jfloat,
) -> jlong {
    scale(canvas_native_ptr, x, y)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetLineDash(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    array: jfloatArray,
) -> jlong {
    let size = env.get_array_length(array).unwrap_or(0) as usize;
    let mut buffer = vec![0f32; size];
    let _ = env.get_float_array_region(array, 0, buffer.as_mut_slice());
    set_line_dash(canvas_native_ptr, size, buffer.as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetLineDashOffset(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    offset: jfloat,
) -> jlong {
    set_line_dash_offset(canvas_native_ptr, offset)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetTransform(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    a: jfloat,
    b: jfloat,
    c: jfloat,
    d: jfloat,
    e: jfloat,
    f: jfloat,
) -> jlong {
    set_transform(canvas_native_ptr, a, b, c, d, e, f)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeStroke(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
) -> jlong {
    stroke(canvas_native_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeStrokePath(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    path: jlong,
) -> jlong {
    stroke_path(canvas_native_ptr, path)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeStrokeRect(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    x: jfloat,
    y: jfloat,
    width: jfloat,
    height: jfloat,
) -> jlong {
    drawRect(env, canvas_native_ptr, x, y, width, height, true)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeStrokeText(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    text: JString,
    x: jfloat,
    y: jfloat,
    width: jfloat,
) -> jlong {
    drawText(env, canvas_native_ptr, text, x, y, width, true)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeTransform(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    a: jfloat,
    b: jfloat,
    c: jfloat,
    d: jfloat,
    e: jfloat,
    f: jfloat,
) -> jlong {
    transform(canvas_native_ptr, a, b, c, d, e, f)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeTranslate(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    x: jfloat,
    y: jfloat,
) -> jlong {
    translate(canvas_native_ptr, x, y)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetLineWidth(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    line_width: jfloat,
) -> jlong {
    set_line_width(canvas_native_ptr, line_width)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetGlobalCompositeOperation(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    composite: JString,
) -> jlong {
    set_global_composite_operation(
        canvas_native_ptr,
        env.get_string(composite).unwrap().as_ptr() as _,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetGlobalAlpha(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    alpha: u8,
) -> jlong {
    set_global_alpha(canvas_native_ptr, alpha)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetLineCap(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    line_cap: JString,
) -> jlong {
    set_line_cap(
        canvas_native_ptr,
        env.get_string(line_cap).unwrap().as_ptr() as _,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetLineJoin(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    line_cap: JString,
) -> jlong {
    set_line_join(
        canvas_native_ptr,
        env.get_string(line_cap).unwrap().as_ptr() as _,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetShadowBlur(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    level: jfloat,
) -> jlong {
    set_shadow_blur(canvas_native_ptr, level)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetShadowColor(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    color: jint,
) -> jlong {
    set_shadow_color(canvas_native_ptr, color as u32)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetShadowOffsetX(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    x: jfloat,
) -> jlong {
    set_shadow_offset_x(canvas_native_ptr, x)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetShadowOffsetY(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    y: jfloat,
) -> jlong {
    set_shadow_offset_y(canvas_native_ptr, y)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetFont(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    font: JString,
) -> jlong {
    set_font(
        canvas_native_ptr,
        env.get_string(font).unwrap().as_ptr() as _,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeCreateImageData(
    env: JNIEnv,
    _: JClass,
    width: jint,
    height: jint,
) -> jobject {
    let mut image_data = create_image_data(width, height);
    env.new_direct_byte_buffer(image_data.as_mut_slice())
        .unwrap()
        .into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativePutImageData(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    width: jint,
    height: jint,
    array: JByteBuffer,
    x: jfloat,
    y: jfloat,
    dirty_x: jfloat,
    dirty_y: jfloat,
    dirty_width: jint,
    dirty_height: jint,
) -> jlong {
    let mut slice = env.get_direct_buffer_address(array);
    match slice {
        Ok(slice) => put_image_data(
            canvas_native_ptr,
            slice.as_mut_ptr(),
            slice.len(),
            width,
            height,
            x,
            y,
            dirty_x,
            dirty_y,
            dirty_width,
            dirty_height,
        ),
        _ => canvas_native_ptr,
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeGetImageData(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    sx: jfloat,
    sy: jfloat,
    sw: size_t,
    sh: size_t,
) -> jobject {
    let mut result = get_image_data(canvas_native_ptr, sx, sy, sw, sh);
    let mut empty_slice = [0u8; 0];
    env.new_direct_byte_buffer(result.1.as_mut_slice())
        .unwrap_or(env.new_direct_byte_buffer(&mut empty_slice).unwrap())
        .into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetImageSmoothingEnabled(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    enabled: jboolean,
) -> jlong {
    set_image_smoothing_enabled(canvas_native_ptr, enabled == JNI_TRUE)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetImageSmoothingQuality(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    quality: JString,
) -> jlong {
    set_image_smoothing_quality(
        canvas_native_ptr,
        env.get_string(quality).unwrap().as_ptr() as _,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetTextAlignment(
    env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
    alignment: JString,
) -> jlong {
    let string = env.get_string(alignment);
    if string.is_ok() {
        let text_alignment = string.unwrap();
        return set_text_align(canvas_native_ptr, text_alignment.as_ptr() as _);
    }
    canvas_native_ptr
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeResetTransform(
    _env: JNIEnv,
    _: JClass,
    canvas_native_ptr: jlong,
) -> jlong {
    reset_transform(canvas_native_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeIsPointInPath(
    _env: JNIEnv,
    _: JClass,
    canvas_ptr: i64,
    x: f32,
    y: f32,
) -> jboolean {
    let canvas_native: Box<CanvasNative> = Box::from_raw(canvas_ptr as *mut _);
    let path = canvas_native.path.clone();
    let _ = Box::into_raw(canvas_native);
    let path = Box::into_raw(Box::new(path)) as i64;
    let rule = CString::new("nonzero").unwrap().into_raw();
    let result = is_point_in_path(canvas_ptr, path, x, y, rule);
    let _ = CString::from_raw(rule);
    let _ = Box::from_raw(path as *mut c_void);
    if result {
        return JNI_TRUE;
    }
    return JNI_FALSE;
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeIsPointInPathWithRule(
    env: JNIEnv,
    _: JClass,
    canvas_ptr: i64,
    x: f32,
    y: f32,
    fill_rule: JString,
) -> jboolean {
    let canvas_native: Box<CanvasNative> = Box::from_raw(canvas_ptr as *mut _);
    let path = canvas_native.path.clone();
    let _ = Box::into_raw(canvas_native);
    let path = Box::into_raw(Box::new(path)) as i64;
    let default = env.new_string("nonzero").unwrap();
    let rule = env
        .get_string(fill_rule)
        .unwrap_or(JavaStr::from_env(&env, default).unwrap());
    let result = is_point_in_path(canvas_ptr, path, x, y, rule.get_raw());
    let _ = Box::from_raw(path as *mut c_void);
    if result {
        return JNI_TRUE;
    }
    return JNI_FALSE;
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeIsPointInPathWithPathRule(
    env: JNIEnv,
    _: JClass,
    canvas_ptr: i64,
    path: jlong,
    x: f32,
    y: f32,
    fill_rule: JString,
) -> jboolean {
    let default = env.new_string("nonzero").unwrap();
    let rule = env
        .get_string(fill_rule)
        .unwrap_or(JavaStr::from_env(&env, default).unwrap());
    let result = is_point_in_path(canvas_ptr, path, x, y, rule.get_raw());
    if result {
        return JNI_TRUE;
    }
    return JNI_FALSE;
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeIsPointInStroke(
    _env: JNIEnv,
    _: JClass,
    canvas_ptr: i64,
    x: f32,
    y: f32,
) -> jboolean {
    let canvas_native: Box<CanvasNative> = Box::from_raw(canvas_ptr as *mut _);
    let path = canvas_native.path.clone();
    let _ = Box::into_raw(canvas_native);
    let path = Box::into_raw(Box::new(path)) as i64;
    let result = is_point_in_stroke(canvas_ptr, path, x, y);
    let _ = Box::from_raw(path as *mut c_void);
    if result {
        return JNI_TRUE;
    }
    return JNI_FALSE;
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeIsPointInStrokeWithPath(
    _env: JNIEnv,
    _: JClass,
    canvas_ptr: i64,
    path: jlong,
    x: f32,
    y: f32,
) -> jboolean {
    let result = is_point_in_stroke(canvas_ptr, path, x, y);
    if result {
        return JNI_TRUE;
    }
    return JNI_FALSE;
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasPath2D_nativeFreePath(
    _env: JNIEnv,
    _: JClass,
    path: jlong,
) {
    free_path_2d(path)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasPath2D_nativeInit(
    _env: JNIEnv,
    _: JClass,
) -> jlong {
    Box::into_raw(Box::new(Path::new())) as *mut _ as i64
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasPath2D_nativeInitWithPath(
    _env: JNIEnv,
    _: JClass,
    path_ptr: jlong,
) -> jlong {
    create_path_from_path(path_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasPath2D_nativeInitWithData(
    env: JNIEnv,
    _: JClass,
    data: JString,
) -> jlong {
    create_path_2d_from_path_data(env.get_string(data).unwrap().as_ptr() as _)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasPath2D_nativeAddPath(
    _env: JNIEnv,
    _: JClass,
    path_native_ptr: jlong,
    path_to_add_ptr: jlong,
    matrix: jlong,
) -> jlong {
    add_path_to_path_with_matrix(path_native_ptr, path_to_add_ptr, matrix)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasPath2D_nativeClosePath(
    _env: JNIEnv,
    _: JClass,
    path_native_ptr: jlong,
) -> jlong {
    close_path(path_native_ptr, false)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasPath2D_nativeMoveTo(
    _env: JNIEnv,
    _: JClass,
    path_native_ptr: jlong,
    x: jfloat,
    y: jfloat,
) -> jlong {
    move_to(path_native_ptr, false, x, y)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasPath2D_nativeLineTo(
    _env: JNIEnv,
    _: JClass,
    path_native_ptr: jlong,
    x: jfloat,
    y: jfloat,
) -> jlong {
    line_to(path_native_ptr, false, x, y)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasPath2D_nativeBezierCurveTo(
    _env: JNIEnv,
    _: JClass,
    path_native_ptr: jlong,
    cp1x: jfloat,
    cp1y: jfloat,
    cp2x: jfloat,
    cp2y: jfloat,
    x: jfloat,
    y: jfloat,
) -> jlong {
    bezier_curve_to(path_native_ptr, false, cp1x, cp1y, cp2x, cp2y, x, y)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasPath2D_nativeQuadraticCurveTo(
    _env: JNIEnv,
    _: JClass,
    path_native_ptr: jlong,
    cpx: jfloat,
    cpy: jfloat,
    x: jfloat,
    y: jfloat,
) -> jlong {
    quadratic_curve_to(path_native_ptr, false, cpx, cpy, x, y)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasPath2D_nativeArc(
    _env: JNIEnv,
    _: JClass,
    path_native_ptr: jlong,
    x: jfloat,
    y: jfloat,
    radius: jfloat,
    start_angle: jfloat,
    end_angle: jfloat,
    anticlockwise: jboolean,
) -> jlong {
    arc(
        path_native_ptr,
        false,
        x,
        y,
        radius,
        start_angle,
        end_angle,
        anticlockwise == JNI_TRUE,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasPath2D_nativeEllipse(
    _env: JNIEnv,
    _: JClass,
    path_native_ptr: jlong,
    x: jfloat,
    y: jfloat,
    radius_x: jfloat,
    radius_y: jfloat,
    rotation: jfloat,
    start_angle: jfloat,
    end_angle: jfloat,
    anticlockwise: jboolean,
) -> jlong {
    ellipse(
        path_native_ptr,
        false,
        x,
        y,
        radius_x,
        radius_y,
        rotation,
        start_angle,
        end_angle,
        anticlockwise == JNI_TRUE,
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasPath2D_nativeArcTo(
    _env: JNIEnv,
    _: JClass,
    path_native_ptr: jlong,
    x1: jfloat,
    y1: jfloat,
    x2: jfloat,
    y2: jfloat,
    radius: jfloat,
) -> jlong {
    arc_to(path_native_ptr, false, x1, y1, x2, y2, radius)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasPath2D_nativeRect(
    _env: JNIEnv,
    _: JClass,
    path_native_ptr: jlong,
    x: jfloat,
    y: jfloat,
    width: jfloat,
    height: jfloat,
) -> jlong {
    rect(path_native_ptr, false, x, y, width, height)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeSetCurrentTransform(
    _env: JNIEnv,
    _: JClass,
    canvas_ptr: jlong,
    matrix: jlong,
) -> jlong {
    set_current_transform(canvas_ptr, matrix)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasRenderingContext2D_nativeGetCurrentTransform(
    _env: JNIEnv,
    _: JClass,
    canvas_ptr: jlong,
) -> jlong {
    get_current_transform(canvas_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasDOMMatrix_nativeInit(
    _env: JNIEnv,
    _: JClass,
) -> jlong {
    create_matrix()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasDOMMatrix_nativeFreeMatrix(
    _env: JNIEnv,
    _: JClass,
    matrix: jlong,
) {
    free_matrix(matrix)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasDOMMatrix_nativeSetMatrix(
    env: JNIEnv,
    _: JClass,
    matrix: jlong,
    matrix_data: jfloatArray,
) -> jlong {
    let length = env.get_array_length(matrix_data).unwrap_or(0);
    let mut buffer = vec![0f32; length as usize];
    let _ = env
        .get_float_array_region(matrix_data, 0, buffer.as_mut_slice())
        .unwrap();
    set_matrix(matrix, buffer.as_mut_ptr() as *const c_void, buffer.len())
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_CanvasDOMMatrix_nativeGetMatrix(
    env: JNIEnv,
    _: JClass,
    matrix: jlong,
) -> jfloatArray {
    let data = get_matrix(matrix);
    let array = env.new_float_array(data.len() as i32).unwrap();
    let _ = env.set_float_array_region(array, 0, data.as_slice());
    array
}
