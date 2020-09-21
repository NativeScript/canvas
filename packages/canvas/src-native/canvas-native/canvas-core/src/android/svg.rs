#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_SVGView_nativeInit(
    env: JNIEnv,
    _: JClass,
    svg_canvas_native_ptr: jlong,
    _bitmap: JObject,
) -> jlong {
    if svg_canvas_native_ptr > 0 {
        return svg_canvas_native_ptr;
    }

    let native_interface = env.get_native_interface();
    let bitmap = _bitmap.into_inner();

    let bitmapInfo = Box::into_raw(Box::new(AndroidBitmapInfo::default()));
    let get_info_success = AndroidBitmap_getInfo(native_interface, bitmap, bitmapInfo);
    if get_info_success < ANDROID_BITMAP_RESULT_SUCCESS {
        debug!("Get Bitmap Info Failed");
        return 0;
    }
    let info = Box::from_raw(bitmapInfo);
    let image_info =
        ImageInfo::new_n32_premul(ISize::new(info.width as i32, info.height as i32), None);
    let mut _dstPixels = null_mut() as *mut c_void;
    let dstPixels: *mut *mut c_void = &mut _dstPixels;
    let _get_lock_success = AndroidBitmap_lockPixels(native_interface, bitmap, dstPixels);
    if _get_lock_success < ANDROID_BITMAP_RESULT_SUCCESS {
        debug!("Get Bitmap Lock Failed");
        return 0;
    }
    let ratio = mem::size_of_val(&dstPixels) / mem::size_of::<u8>();
    let length = ((info.width * info.height) * ratio as u32) as usize;
    let new_len = &length * ratio;
    let new_cap = &length * ratio;
    let ptr = _dstPixels as *mut _;
    let pixels: &mut [u8] = std::slice::from_raw_parts_mut(ptr, length as usize);

    let surface_holder =
        Surface::new_raster_direct(&image_info, pixels, Some(info.stride as usize), None);
    if surface_holder.is_none() {
        return 0;
    }
    let mut surface = surface_holder.unwrap().deref().to_owned();
    let mut native_svg_canvas = SVGCanvasNative {
        surface,
        context: None,
    };

    let ptr = Box::into_raw(Box::new(native_svg_canvas)) as *mut _ as i64;

    AndroidBitmap_unlockPixels(native_interface, bitmap);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_SVGView_drawSVG(
    env: JNIEnv,
    _: JClass,
    svg_canvas_native_ptr: jlong,
    _bitmap: JObject,
    svg: JString,
) -> jlong {
    let native_interface = env.get_native_interface();
    let bitmap = _bitmap.into_inner();

    let bitmapInfo = Box::into_raw(Box::new(AndroidBitmapInfo::default()));
    let get_info_success = AndroidBitmap_getInfo(native_interface, bitmap, bitmapInfo);
    if get_info_success < ANDROID_BITMAP_RESULT_SUCCESS {
        debug!("Get Bitmap Info Failed");
        return 0;
    }
    let info = Box::from_raw(bitmapInfo);
    let image_info =
        ImageInfo::new_n32_premul(ISize::new(info.width as i32, info.height as i32), None);
    let mut _dstPixels = null_mut() as *mut c_void;
    let dstPixels: *mut *mut c_void = &mut _dstPixels;
    let _get_lock_success = AndroidBitmap_lockPixels(native_interface, bitmap, dstPixels);
    if _get_lock_success < ANDROID_BITMAP_RESULT_SUCCESS {
        debug!("Get Bitmap Lock Failed");
        return 0;
    }
    let ratio = mem::size_of_val(&dstPixels) / mem::size_of::<u8>();
    let length = ((info.width * info.height) * ratio as u32) as usize;
    let new_len = &length * ratio;
    let new_cap = &length * ratio;
    let ptr = _dstPixels as *mut _;
    let pixels: &mut [u8] = std::slice::from_raw_parts_mut(ptr, length as usize);

    let surface_holder =
        Surface::new_raster_direct(&image_info, pixels, Some(info.stride as usize), None);
    if surface_holder.is_none() {
        return svg_canvas_native_ptr;
    }

    let mut svg_canvas_native: Box<SVGCanvasNative> =
        unsafe { Box::from_raw(svg_canvas_native_ptr as *mut _) };
    let surface = surface_holder.unwrap();
    svg_canvas_native.surface = surface.deref().to_owned();

    let svg_canvas_native_ptr = Box::into_raw(svg_canvas_native) as *mut _ as i64;
    let ptr = draw_svg_image(
        svg_canvas_native_ptr,
        env.get_string(svg).unwrap().as_ptr() as _,
    );
    AndroidBitmap_unlockPixels(native_interface, bitmap);
    return ptr;
}