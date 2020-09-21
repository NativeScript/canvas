use std::os::raw::{c_int, c_longlong, c_uint, c_void};

use crate::common::{image_asset_get_rgb_bytes, image_asset_get_rgba_bytes};
use std::mem::MaybeUninit;
use std::ptr::null_mut;

const RGBA: u32 = 0x1908;
const RGBA_INTEGER: u32 = 0x8D99;

#[no_mangle]
pub extern "C" fn native_gl_tex_image_2D_asset(
    target: c_uint,
    level: c_int,
    internalformat: c_int,
    width: c_int,
    height: c_int,
    border: c_int,
    format: c_uint,
    image_type: c_uint,
    asset: c_longlong,
    flip_y: u8,
) {
    let data;
    match format as u32 {
        RGBA | RGBA_INTEGER => {
            data = image_asset_get_rgba_bytes(asset);
        }
        _ => {
            data = image_asset_get_rgb_bytes(asset);
        }
    }
    let flip = flip_y == 1;
    if flip {
        crate::common::utils::flip_in_place(
            data.array,
            data.length,
            (crate::common::utils::bytes_per_pixel(image_type, format) as i32 * width) as usize,
            height as usize,
        );
    }
    unsafe {
        gl_bindings::glTexImage2D(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            format,
            image_type,
            data.array as *const c_void,
        );
    }
}

#[no_mangle]
pub extern "C" fn native_gl_tex_sub_image_2D_asset(
    target: c_uint,
    level: c_int,
    xoffset: c_int,
    yoffset: c_int,
    width: c_int,
    height: c_int,
    format: c_uint,
    image_type: c_uint,
    asset: c_longlong,
    flip_y: u8,
) {
    let data;
    match format as u32 {
        RGBA | RGBA_INTEGER => {
            data = image_asset_get_rgba_bytes(asset);
        }
        _ => {
            data = image_asset_get_rgb_bytes(asset);
        }
    }
    let flip = flip_y == 1;
    if flip {
        crate::common::utils::flip_in_place(
            data.array,
            data.length,
            (crate::common::utils::bytes_per_pixel(image_type, format) as i32 * width) as usize,
            height as usize,
        );
    }
    unsafe {
        gl_bindings::glTexSubImage2D(
            target,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format,
            image_type,
            data.array as *const c_void,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn native_tex_image_3D_asset(
    target: c_uint,
    level: c_int,
    internalformat: c_int,
    width: c_int,
    height: c_int,
    depth: c_int,
    border: c_int,
    format: c_uint,
    image_type: c_uint,
    asset: c_longlong,
    flip_y: u8,
) {
    let data;
    match format as u32 {
        RGBA | RGBA_INTEGER => {
            data = image_asset_get_rgba_bytes(asset);
        }
        _ => {
            data = image_asset_get_rgb_bytes(asset);
        }
    }
    let flip = flip_y == 1;
    if flip {
        crate::common::utils::flip_in_place_3d(
            data.array,
            data.length,
            (crate::common::utils::bytes_per_pixel(image_type, format) as i32 * width) as usize,
            height as usize,
            depth as usize,
        );
    }
    gl_bindings::glTexImage3D(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        image_type,
        data.array as *const c_void,
    );
}

#[no_mangle]
pub unsafe extern "C" fn native_tex_sub_image_3D_asset(
    target: c_uint,
    level: c_int,
    xoffset: c_int,
    yoffset: c_int,
    zoffset: c_int,
    width: c_int,
    height: c_int,
    depth: c_int,
    format: c_uint,
    image_type: c_uint,
    asset: c_longlong,
    flip_y: u8,
) {
    let data;
    match format as u32 {
        RGBA | RGBA_INTEGER => {
            data = image_asset_get_rgba_bytes(asset);
        }
        _ => {
            data = image_asset_get_rgb_bytes(asset);
        }
    }
    let flip = flip_y == 1;
    if flip {
        crate::common::utils::flip_in_place_3d(
            data.array,
            data.length,
            (crate::common::utils::bytes_per_pixel(image_type, format) as i32 * width) as usize,
            height as usize,
            depth as usize,
        );
    }
    gl_bindings::glTexSubImage3D(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        image_type,
        data.array as *const c_void,
    );
}

#[no_mangle]
pub unsafe extern "C" fn native_vertex_attrib_pointer(
    index: c_uint,
    size: c_int,
    pointer_type: c_uint,
    normalized: u8,
    stride: c_int,
    offset: c_longlong,
) {
    gl_bindings::glVertexAttribPointer(
        index,
        size,
        pointer_type,
        normalized,
        stride,
        offset as *const c_void,
    )
}

#[no_mangle]
pub unsafe extern "C" fn native_get_vertex_attrib_offset(
    index: c_uint,
    pname: c_uint,
) -> c_longlong {
    let mut buf = [0i64; 1];
    let ptr_ptr: *mut *mut c_void = buf.as_mut_ptr() as *mut _;
    gl_bindings::glGetVertexAttribPointerv(index, pname, ptr_ptr);
    buf[0]
}
