const GL_UNSIGNED_BYTE: u32 = 0x1401;
const GL_FLOAT: u32 = 0x1406;
const GL_HALF_FLOAT: u32 = 0x140B;
const GL_UNSIGNED_SHORT_5_6_5: u32 = 0x8363;
const GL_UNSIGNED_SHORT_4_4_4_4: u32 = 0x8033;
const GL_UNSIGNED_SHORT_5_5_5_1: u32 = 0x8034;
const GL_LUMINANCE: u32 = 0x1909;
const GL_ALPHA: u32 = 0x1906;
const GL_LUMINANCE_ALPHA: u32 = 0x190A;
const GL_RGB: u32 = 0x1907;
const GL_RGB8: u32 = 0x8051;
const GL_RGBA: u32 = 0x1908;
const GL_RGBA8: u32 = 0x8058;

pub enum GLImageAssetBytesType {
    RGB,
    RGBA8,
    Luminance,
    Alpha,
    None,
}

pub fn get_image_asset_bytes_type(format: i32, image_type: i32) -> GLImageAssetBytesType {
    match (format as u32, image_type as u32) {
        (GL_RGB, GL_UNSIGNED_BYTE)
        | (GL_RGB, GL_UNSIGNED_SHORT_5_6_5)
        | (GL_RGBA, GL_UNSIGNED_BYTE)
        | (GL_RGBA, GL_UNSIGNED_SHORT_4_4_4_4)
        | (GL_RGBA, GL_UNSIGNED_SHORT_5_5_5_1) => GLImageAssetBytesType::RGBA8,
        (GL_LUMINANCE, GL_UNSIGNED_BYTE) | (GL_LUMINANCE_ALPHA, GL_UNSIGNED_BYTE) => {
            GLImageAssetBytesType::Luminance
        }
        (GL_ALPHA, GL_UNSIGNED_BYTE) => GLImageAssetBytesType::Alpha,
        _ => GLImageAssetBytesType::None,
    }
}

#[allow(unused)]
pub fn flip_in_place_3d(
    pixels: *mut u8,
    length: usize,
    bytes_per_row: usize,
    height: usize,
    depth: usize,
) {
    unsafe {
        let mut data = pixels;
        for _ in 0..depth {
            flip_in_place(data, length, bytes_per_row, height);
            data = pixels.offset((bytes_per_row * height) as isize);
        }
    }
}

#[allow(unused)]
pub fn flip_in_place(pixels: *mut u8, length: usize, bytes_per_row: usize, height: usize) {
    let mut slice = unsafe { std::slice::from_raw_parts_mut(pixels, length) };
    flip_pixels(slice, height)
}

#[allow(unused)]
pub fn flip_in_place_integer(pixels: *mut u32, length: usize, bytes_per_row: usize, height: usize) {
    let mut slice = unsafe { std::slice::from_raw_parts_mut(pixels, length) };
    flip_pixels_integer(slice, height)
}


fn flip_pixels(pixels: &'_ mut [u8], rows: usize) {
    let bytes_per_row = pixels.len() / rows;
    let (first_half, second_half) = pixels.split_at_mut(pixels.len() / 2);
    let top_rows = first_half.chunks_mut(bytes_per_row);
    let bot_rows = second_half.chunks_mut(bytes_per_row).rev();
    for (top, bot) in top_rows.zip(bot_rows) {
        const SWAP_SIZE: usize = 4;
        let mut i_tops = top.chunks_exact_mut(SWAP_SIZE);
        let mut i_bots = bot.chunks_exact_mut(SWAP_SIZE);
        for (i_top, i_bot) in Iterator::zip(i_tops.by_ref(), i_bots.by_ref()) {
            type SwapBlock = [u8; SWAP_SIZE];
            let i_top: &mut SwapBlock = i_top.try_into().unwrap();
            let i_bot: &mut SwapBlock = i_bot.try_into().unwrap();
            std::mem::swap(i_top, i_bot);
        }
        for (b_top, b_bot) in Iterator::zip(
            i_tops.into_remainder().iter_mut(),
            i_bots.into_remainder().iter_mut(),
        ) {
            std::mem::swap(b_top, b_bot);
        }
    }
}

fn flip_pixels_integer(pixels: &'_ mut [u32], rows: usize) {
    let values_per_row = pixels.len() / rows;
    let (first_half, second_half) = pixels.split_at_mut(pixels.len() / 2);

    let top_rows = first_half.chunks_mut(values_per_row);
    let bot_rows = second_half.chunks_mut(values_per_row).rev();

    for (top, bot) in top_rows.zip(bot_rows) {
        for (i_top, i_bot) in Iterator::zip(top.iter_mut(), bot.iter_mut()) {
            std::mem::swap(i_top, i_bot);
        }
    }
}


pub fn bytes_per_pixel(pixel_type: u32, format: u32) -> u32 {
    let bytes_per_component = match pixel_type {
        GL_UNSIGNED_BYTE => 1,
        GL_FLOAT => 4,
        GL_HALF_FLOAT => 2,
        GL_UNSIGNED_SHORT_5_6_5 | GL_UNSIGNED_SHORT_4_4_4_4 | GL_UNSIGNED_SHORT_5_5_5_1 => 2,
        GL_UNSIGNED_INT_10F_11F_11F_REV => 4, // 32 bits for 10, 11, and 11 bits for RGB
        GL_UNSIGNED_INT_2_10_10_10_REV => 4,  // 32 bits for 10, 10, 10, and 2 bits
        GL_UNSIGNED_INT_24_8 => 4,            // 32 bits for depth and stencil
        _ => 0,
    };

    match format {
        gl_bindings::RED | gl_bindings::RED_INTEGER => bytes_per_component, // Only one component for GL_RED and GL_RED_INTEGER
        gl_bindings::RG => 2 * bytes_per_component, // Two components (red and green) for GL_RG
        gl_bindings::RG_INTEGER => 2 * bytes_per_component, // Two components for GL_RG_INTEGER
        GL_RGB => match pixel_type {
            GL_UNSIGNED_BYTE => 3 * bytes_per_component, // 3 components for GL_RGB with GL_UNSIGNED_BYTE
            GL_UNSIGNED_SHORT_5_6_5 => 2, // Special case for RGB 565 format
            gl_bindings::UNSIGNED_INT_10F_11F_11F_REV => 4, // Special case for RGB 10F 11F 11F format
            _ => 3 * bytes_per_component,
        },
        gl_bindings::RGB_INTEGER => 3 * bytes_per_component, // Three components for GL_RGB_INTEGER
        GL_RGBA => match pixel_type {
            GL_UNSIGNED_SHORT_4_4_4_4 | GL_UNSIGNED_SHORT_5_5_5_1 => 2, // 16-bit RGBA formats
            gl_bindings::UNSIGNED_INT_2_10_10_10_REV => 4, // RGBA 2_10_10_10 format (32 bits)
            _ => 4 * bytes_per_component,
        },
        gl_bindings::RGBA_INTEGER => 4 * bytes_per_component, // Four components for GL_RGBA_INTEGER
        GL_LUMINANCE | GL_ALPHA => 1 * bytes_per_component,
        GL_LUMINANCE_ALPHA => 2 * bytes_per_component,
        gl_bindings::DEPTH_COMPONENT => 4, // Typically 32 bits for depth
        gl_bindings::DEPTH_STENCIL => 4,   // 32 bits for depth and stencil
        _ => 0,
    }
}
