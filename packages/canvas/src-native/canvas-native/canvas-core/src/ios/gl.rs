use std::os::raw::{c_int, c_longlong, c_uint, c_void};

use crate::common::context::image_asset::ImageAsset;
use crate::common::ffi::u8_array::destroy_u8_array;

const RGBA: u32 = 0x1908;
const RGBA_INTEGER: u32 = 0x8D99;

#[no_mangle]
pub extern "C" fn gl_tex_image_2D_asset(
    target: c_uint,
    level: c_int,
    internalformat: c_int,
    border: c_int,
    format: c_uint,
    image_type: c_uint,
    asset: c_longlong,
    flip_y: bool,
) {
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        let mut data = asset.get_bytes();
        if let Some(data) = data {
            let width = asset.width();
            let height = asset.height();
            if flip_y {
                let mut data = data.to_vec();
                crate::common::utils::gl::flip_in_place(
                    data.as_mut_ptr(),
                    data.len(),
                    (crate::common::utils::gl::bytes_per_pixel(image_type, format) as i32
                        * width as i32) as usize,
                    height as usize,
                );

                gl_bindings::glTexImage2D(
                    target,
                    level,
                    internalformat,
                    asset.width() as i32,
                    asset.height() as i32,
                    border,
                    format,
                    image_type,
                    data.as_ptr() as *const c_void,
                );
            } else {
                gl_bindings::glTexImage2D(
                    target,
                    level,
                    internalformat,
                    asset.width() as i32,
                    asset.height() as i32,
                    border,
                    format,
                    image_type,
                    data.as_ptr() as *const c_void,
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn gl_tex_sub_image_2D_asset(
    target: c_uint,
    level: c_int,
    xoffset: c_int,
    yoffset: c_int,
    format: c_uint,
    image_type: c_uint,
    asset: c_longlong,
    flip_y: bool,
) {
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        let data = asset.get_bytes();
        let width = asset.width();
        let height = asset.height();

        if let Some(data) = data {
            if flip_y {
                let mut data = data.to_vec();
                crate::common::utils::gl::flip_in_place(
                    data.as_mut_ptr(),
                    data.len(),
                    (crate::common::utils::gl::bytes_per_pixel(image_type, format) as i32
                        * width as i32) as usize,
                    height as usize,
                );
                gl_bindings::glTexSubImage2D(
                    target,
                    level,
                    xoffset,
                    yoffset,
                    asset.width() as i32,
                    asset.height() as i32,
                    format,
                    image_type,
                    data.as_ptr() as *const c_void,
                );
            } else {
                gl_bindings::glTexSubImage2D(
                    target,
                    level,
                    xoffset,
                    yoffset,
                    asset.width() as i32,
                    asset.height() as i32,
                    format,
                    image_type,
                    data.as_ptr() as *const c_void,
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn gl_tex_image_3D_asset(
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
    flip_y: bool,
) {
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        let data = asset.get_bytes();
        if let Some(data) = data {
            if flip_y {
                let mut data = data.to_vec();
                crate::common::utils::gl::flip_in_place_3d(
                    data.as_mut_ptr(),
                    data.len(),
                    (crate::common::utils::gl::bytes_per_pixel(image_type, format) as i32
                        * asset.width() as i32) as usize,
                    asset.height() as usize,
                    depth as usize,
                );
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
                    data.as_ptr() as *const c_void,
                );
            } else {
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
                    data.as_ptr() as *const c_void,
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn gl_tex_sub_image_3D_asset(
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
    flip_y: bool,
) {
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        let data = asset.get_bytes();
        if let Some(data) = data {
            if flip_y {
                let mut data = data.to_vec();
                crate::common::utils::gl::flip_in_place_3d(
                    data.as_mut_ptr(),
                    data.len(),
                    (crate::common::utils::gl::bytes_per_pixel(image_type, format) as i32
                        * asset.width() as i32) as usize,
                    asset.height() as usize,
                    depth as usize,
                );
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
                    data.as_ptr() as *const c_void,
                );
            } else {
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
                    data.as_ptr() as *const c_void,
                );
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn gl_vertex_attrib_pointer(
    index: c_uint,
    size: c_int,
    pointer_type: c_uint,
    normalized: bool,
    stride: c_int,
    offset: c_longlong,
) {
    gl_bindings::glVertexAttribPointer(
        index,
        size,
        pointer_type,
        normalized as u8,
        stride,
        offset as *const c_void,
    )
}

#[no_mangle]
pub unsafe extern "C" fn gl_get_vertex_attrib_offset(index: c_uint, pname: c_uint) -> c_longlong {
    let mut buf = [0i64; 1];
    let ptr_ptr: *mut *mut c_void = buf.as_mut_ptr() as *mut _;
    gl_bindings::glGetVertexAttribPointerv(index, pname, ptr_ptr);
    buf[0]
}
