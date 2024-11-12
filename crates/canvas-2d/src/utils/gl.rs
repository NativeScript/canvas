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
const GL_RGBA: u32 = 0x1908;

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

pub fn bytes_per_pixel(pixel_type: u32, format: u32) -> u32 {
    let mut bytes_per_component = 0;
    let mut do_return = 0;
    match pixel_type {
        GL_UNSIGNED_BYTE => {
            bytes_per_component = 1;
        }
        GL_FLOAT => {
            bytes_per_component = 4;
        }
        GL_HALF_FLOAT => {
            bytes_per_component = 2;
        }
        GL_UNSIGNED_SHORT_5_6_5 | GL_UNSIGNED_SHORT_4_4_4_4 | GL_UNSIGNED_SHORT_5_5_5_1 => {
            do_return = 2;
        }
        _ => {}
    }

    if do_return > 2 {
        return 2;
    }
    match format {
        GL_LUMINANCE | GL_ALPHA => 1 * bytes_per_component,
        GL_LUMINANCE_ALPHA => 2 * bytes_per_component,
        GL_RGB => 3 * bytes_per_component,
        GL_RGBA => 4 * bytes_per_component,
        _ => 0,
    }
}
