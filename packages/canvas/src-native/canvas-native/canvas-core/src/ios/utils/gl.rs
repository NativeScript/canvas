#[no_mangle]
pub extern "C" fn flip_y_in_place(
    data: *mut u8,
    length: usize,
    bytes_per_row: usize,
    height: usize,
) {
    crate::common::utils::gl::flip_in_place(data, length, bytes_per_row, height)
}

#[no_mangle]
pub extern "C" fn flip_y_in_place_3d(
    data: *mut u8,
    length: usize,
    bytes_per_row: usize,
    height: usize,
    depth: usize,
) {
    crate::common::utils::gl::flip_in_place_3d(data, length, bytes_per_row, height, depth)
}

#[no_mangle]
pub extern "C" fn flip_y_in_place_3d_i8(
    data: *mut i8,
    length: usize,
    bytes_per_row: usize,
    height: usize,
    depth: usize,
) {
    crate::common::utils::gl::flip_in_place_3d(data as *mut u8, length, bytes_per_row, height, depth)
}

#[no_mangle]
pub extern "C" fn flip_y_in_place_3d_u16(
    data: *mut u16,
    length: usize,
    bytes_per_row: usize,
    height: usize,
    depth: usize,
) {
    crate::common::utils::gl::flip_in_place_3d(
        data as *mut u8,
        length * std::mem::size_of::<u16>(),
        bytes_per_row,
        height,
        depth,
    )
}

#[no_mangle]
pub extern "C" fn flip_y_in_place_3d_i16(
    data: *mut i16,
    length: usize,
    bytes_per_row: usize,
    height: usize,
    depth: usize,
) {
    crate::common::utils::gl::flip_in_place_3d(
        data as *mut u8,
        length * std::mem::size_of::<i16>(),
        bytes_per_row,
        height,
        depth,
    )
}

#[no_mangle]
pub extern "C" fn flip_y_in_place_3d_u32(
    data: *mut u32,
    length: usize,
    bytes_per_row: usize,
    height: usize,
    depth: usize,
) {
    crate::common::utils::gl::flip_in_place_3d(
        data as *mut u8,
        length * std::mem::size_of::<u32>(),
        bytes_per_row,
        height,
        depth,
    )
}

#[no_mangle]
pub extern "C" fn flip_y_in_place_3d_i32(
    data: *mut i32,
    length: usize,
    bytes_per_row: usize,
    height: usize,
    depth: usize,
) {
    crate::common::utils::gl::flip_in_place_3d(
        data as *mut u8,
        length * std::mem::size_of::<i32>(),
        bytes_per_row,
        height,
        depth,
    )
}

#[no_mangle]
pub extern "C" fn flip_y_in_place_3d_f32(
    data: *mut f32,
    length: usize,
    bytes_per_row: usize,
    height: usize,
    depth: usize,
) {
    crate::common::utils::gl::flip_in_place_3d(
        data as *mut u8,
        length * std::mem::size_of::<f32>(),
        bytes_per_row,
        height,
        depth,
    )
}

#[no_mangle]
pub extern "C" fn flip_y_in_place_3d_f64(
    data: *mut f64,
    length: usize,
    bytes_per_row: usize,
    height: usize,
    depth: usize,
) {
    crate::common::utils::gl::flip_in_place_3d(
        data as *mut u8,
        length * std::mem::size_of::<f64>(),
        bytes_per_row,
        height,
        depth,
    )
}

#[no_mangle]
pub extern "C" fn flip_y_in_place_i8(
    data: *mut i8,
    length: usize,
    bytes_per_row: usize,
    height: usize,
) {
    crate::common::utils::gl::flip_in_place(data as *mut u8, length, bytes_per_row, height)
}

#[no_mangle]
pub extern "C" fn flip_y_in_place_u16(
    data: *mut u16,
    length: usize,
    bytes_per_row: usize,
    height: usize,
) {
    crate::common::utils::gl::flip_in_place(
        data as *mut u8,
        length * std::mem::size_of::<u16>(),
        bytes_per_row,
        height,
    )
}

#[no_mangle]
pub extern "C" fn flip_y_in_place_i16(
    data: *mut i16,
    length: usize,
    bytes_per_row: usize,
    height: usize,
) {
    crate::common::utils::gl::flip_in_place(
        data as *mut u8,
        length * std::mem::size_of::<i16>(),
        bytes_per_row,
        height,
    )
}

#[no_mangle]
pub extern "C" fn flip_y_in_place_u32(
    data: *mut u32,
    length: usize,
    bytes_per_row: usize,
    height: usize,
) {
    crate::common::utils::gl::flip_in_place(
        data as *mut u8,
        length * std::mem::size_of::<u32>(),
        bytes_per_row,
        height,
    )
}

#[no_mangle]
pub extern "C" fn flip_y_in_place_i32(
    data: *mut i32,
    length: usize,
    bytes_per_row: usize,
    height: usize,
) {
    crate::common::utils::gl::flip_in_place(
        data as *mut u8,
        length * std::mem::size_of::<i32>(),
        bytes_per_row,
        height,
    )
}

#[no_mangle]
pub extern "C" fn flip_y_in_place_f32(
    data: *mut f32,
    length: usize,
    bytes_per_row: usize,
    height: usize,
) {
    crate::common::utils::gl::flip_in_place(
        data as *mut u8,
        length * std::mem::size_of::<f32>(),
        bytes_per_row,
        height,
    )
}

#[no_mangle]
pub extern "C" fn flip_y_in_place_f64(
    data: *mut f64,
    length: usize,
    bytes_per_row: usize,
    height: usize,
) {
    crate::common::utils::gl::flip_in_place(
        data as *mut u8,
        length * std::mem::size_of::<i64>(),
        bytes_per_row,
        height,
    )
}