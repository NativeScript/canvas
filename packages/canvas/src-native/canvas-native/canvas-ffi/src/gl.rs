use std::os::raw::{c_int, c_longlong, c_uint, c_void};

use canvas_core::context::image_asset::ImageAsset;

use super::arrays::canvas_native_destroy_u8_array;

const RGBA: u32 = 0x1908;
const RGBA_INTEGER: u32 = 0x8D99;

#[no_mangle]
pub extern "C" fn canvas_native_gl_tex_image_2D_asset(
    target: c_uint,
    level: c_int,
    internalformat: c_int,
    border: c_int,
    format: c_uint,
    image_type: c_uint,
    asset: *mut ImageAsset,
    flip_y: bool,
) {
    assert!(!asset.is_null());
    unsafe {
        let asset = &mut *asset;
        let mut data;
        match format as u32 {
            RGBA | RGBA_INTEGER => data = asset.rgba_internal_bytes(),
            _ => data = asset.rgb_internal_bytes(),
        }
        let data_array = data.as_mut_slice();
        let width = asset.width();
        let height = asset.height();
        if flip_y {
            canvas_core::utils::gl::flip_in_place(
                data_array.as_mut_ptr(),
                data_array.len(),
                (canvas_core::utils::gl::bytes_per_pixel(image_type, format) as i32 * width as i32)
                    as usize,
                height as usize,
            );
        }
        gl_bindings::glTexImage2D(
            target,
            level,
            internalformat,
            asset.width() as i32,
            asset.height() as i32,
            border,
            format,
            image_type,
            data_array.as_ptr() as *const c_void,
        );
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_gl_tex_sub_image_2D_asset(
    target: c_uint,
    level: c_int,
    xoffset: c_int,
    yoffset: c_int,
    format: c_uint,
    image_type: c_uint,
    asset: *mut ImageAsset,
    flip_y: bool,
) {
    assert!(!asset.is_null());
    unsafe {
        let data;
        let asset = &mut *asset;
        match format as u32 {
            RGBA | RGBA_INTEGER => {
                data = asset.rgba_bytes();
            }
            _ => {
                data = asset.rgb_bytes();
            }
        }
        let width = asset.width();
        let height = asset.height();
        let data_array = &mut *data;
        if flip_y {
            canvas_core::utils::gl::flip_in_place(
                data_array.data,
                data_array.data_len,
                (canvas_core::utils::gl::bytes_per_pixel(image_type, format) as i32 * width as i32)
                    as usize,
                height as usize,
            );
        }
        gl_bindings::glTexSubImage2D(
            target,
            level,
            xoffset,
            yoffset,
            asset.width() as i32,
            asset.height() as i32,
            format,
            image_type,
            data_array.data as *const c_void,
        );
        canvas_native_destroy_u8_array(data)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_gl_tex_image_3D_asset(
    target: c_uint,
    level: c_int,
    internalformat: c_int,
    width: c_int,
    height: c_int,
    depth: c_int,
    border: c_int,
    format: c_uint,
    image_type: c_uint,
    asset: *mut ImageAsset,
    flip_y: bool,
) {
    assert!(!asset.is_null());
    unsafe {
        let data;
        let asset = &mut *asset;
        match format as u32 {
            RGBA | RGBA_INTEGER => {
                data = asset.rgba_bytes();
            }
            _ => {
                data = asset.rgb_bytes();
            }
        }
        let data_array = &mut *data;
        if flip_y {
            canvas_core::utils::gl::flip_in_place_3d(
                data_array.data,
                data_array.data_len,
                (canvas_core::utils::gl::bytes_per_pixel(image_type, format) as i32
                    * asset.width() as i32) as usize,
                asset.height() as usize,
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
            data_array.data as *const c_void,
        );
        canvas_native_destroy_u8_array(data)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_gl_tex_sub_image_3D_asset(
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
    asset: *mut ImageAsset,
    flip_y: bool,
) {
    assert!(!asset.is_null());
    unsafe {
        let data;
        let asset = &mut *asset;
        match format as u32 {
            RGBA | RGBA_INTEGER => {
                data = asset.rgba_bytes();
            }
            _ => {
                data = asset.rgba_bytes();
            }
        }
        let data_array = &mut *data;
        if flip_y {
            canvas_core::utils::gl::flip_in_place_3d(
                data_array.data,
                data_array.data_len,
                (canvas_core::utils::gl::bytes_per_pixel(image_type, format) as i32
                    * asset.width() as i32) as usize,
                asset.height() as usize,
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
            data_array.data as *const c_void,
        );
        canvas_native_destroy_u8_array(data)
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_gl_vertex_attrib_pointer(
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
pub unsafe extern "C" fn canvas_native_gl_get_vertex_attrib_offset(
    index: c_uint,
    pname: c_uint,
) -> c_longlong {
    let mut buf = [0i64; 1];
    let ptr_ptr: *mut *mut c_void = buf.as_mut_ptr() as *mut _;
    gl_bindings::glGetVertexAttribPointerv(index, pname, ptr_ptr);
    buf[0]
}
