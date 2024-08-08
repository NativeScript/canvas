use crate::image_asset::ImageAsset;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
#[repr(C)]
pub enum ImageBitmapPremultiplyAlpha {
    Default,
    Premultiply,
    AlphaNone,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
#[repr(C)]
pub enum ImageBitmapColorSpaceConversion {
    Default,
    None,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
#[repr(C)]
pub enum ImageBitmapResizeQuality {
    Low,
    Medium,
    High,
    Pixelated,
}

impl Into<i32> for ImageBitmapPremultiplyAlpha {
    fn into(self) -> i32 {
        if self == ImageBitmapPremultiplyAlpha::Premultiply {
            return 1;
        } else if self == ImageBitmapPremultiplyAlpha::AlphaNone {
            return 2;
        }
        return 0;
    }
}

impl Into<i32> for ImageBitmapColorSpaceConversion {
    fn into(self) -> i32 {
        if self == ImageBitmapColorSpaceConversion::None {
            return 1;
        }
        0
    }
}

impl Into<i32> for ImageBitmapResizeQuality {
    fn into(self) -> i32 {
        if self == ImageBitmapResizeQuality::Medium {
            return 1;
        } else if self == ImageBitmapResizeQuality::High {
            return 2;
        } else if self == ImageBitmapResizeQuality::Pixelated {
            return 3;
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_bitmap_create_from_asset(
    asset: *mut ImageAsset,
    flip_y: bool,
    premultiply_alpha: ImageBitmapPremultiplyAlpha,
    color_space_conversion: ImageBitmapColorSpaceConversion,
    resize_quality: ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
) -> *mut ImageAsset {
    if asset.is_null() {
        return std::ptr::null_mut();
    }
    let asset = unsafe { &mut *asset };

    Box::into_raw(Box::new(ImageAsset(
        canvas_2d::image_bitmap::create_from_image_asset_src_rect(
            &mut asset.0,
            None,
            flip_y,
            premultiply_alpha.into(),
            color_space_conversion.into(),
            resize_quality.into(),
            resize_width,
            resize_height,
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_bitmap_create_from_asset_src_rect(
    asset: *mut ImageAsset,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    flip_y: bool,
    premultiply_alpha: ImageBitmapPremultiplyAlpha,
    color_space_conversion: ImageBitmapColorSpaceConversion,
    resize_quality: ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
) -> *mut ImageAsset {
    if asset.is_null() {
        return std::ptr::null_mut();
    }
    let asset = unsafe { &mut *asset };
    Box::into_raw(Box::new(ImageAsset(
        canvas_2d::image_bitmap::create_from_image_asset_src_rect(
            &mut asset.0,
            Some((sx, sy, s_width, s_height).into()),
            flip_y,
            premultiply_alpha.into(),
            color_space_conversion.into(),
            resize_quality.into(),
            resize_width,
            resize_height,
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_bitmap_create_from_encoded_bytes(
    bytes: *const u8,
    size: usize,
    flip_y: bool,
    premultiply_alpha: ImageBitmapPremultiplyAlpha,
    color_space_conversion: ImageBitmapColorSpaceConversion,
    resize_quality: ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
) -> *mut ImageAsset {
    let bytes = unsafe { std::slice::from_raw_parts(bytes, size) };
    Box::into_raw(Box::new(ImageAsset(
        canvas_2d::image_bitmap::create_image_asset_encoded(
            bytes,
            None,
            flip_y,
            premultiply_alpha.into(),
            color_space_conversion.into(),
            resize_quality.into(),
            resize_width,
            resize_height,
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_bitmap_create_from_encoded_bytes_with_output(
    bytes: *const u8,
    size: usize,
    flip_y: bool,
    premultiply_alpha: ImageBitmapPremultiplyAlpha,
    color_space_conversion: ImageBitmapColorSpaceConversion,
    resize_quality: ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
    output: *mut ImageAsset,
) -> bool {
    assert!(!output.is_null());
    let bytes = unsafe { std::slice::from_raw_parts(bytes, size) };
    let output = unsafe { &mut *output };
    canvas_2d::image_bitmap::create_image_asset_with_output(
        bytes,
        None,
        flip_y,
        premultiply_alpha.into(),
        color_space_conversion.into(),
        resize_quality.into(),
        resize_width,
        resize_height,
        &mut output.0,
    );

    output.is_valid()
}

#[no_mangle]
pub extern "C" fn canvas_native_image_bitmap_create_from_encoded_bytes_src_rect(
    bytes: *const u8,
    size: usize,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    flip_y: bool,
    premultiply_alpha: ImageBitmapPremultiplyAlpha,
    color_space_conversion: ImageBitmapColorSpaceConversion,
    resize_quality: ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
) -> *mut ImageAsset {
    let bytes = unsafe { std::slice::from_raw_parts(bytes, size) };
    Box::into_raw(Box::new(ImageAsset(
        canvas_2d::image_bitmap::create_image_asset_encoded(
            bytes,
            Some((sx, sy, s_width, s_height).into()),
            flip_y,
            premultiply_alpha.into(),
            color_space_conversion.into(),
            resize_quality.into(),
            resize_width,
            resize_height,
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(
    bytes: *const u8,
    size: usize,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    flip_y: bool,
    premultiply_alpha: ImageBitmapPremultiplyAlpha,
    color_space_conversion: ImageBitmapColorSpaceConversion,
    resize_quality: ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
    output: *mut ImageAsset,
) -> bool {
    assert!(!output.is_null());
    let bytes = unsafe { std::slice::from_raw_parts(bytes, size) };
    let output = unsafe { &mut *output };
    canvas_2d::image_bitmap::create_image_asset_with_output(
        bytes,
        Some((sx, sy, s_width, s_height).into()),
        flip_y,
        premultiply_alpha.into(),
        color_space_conversion.into(),
        resize_quality.into(),
        resize_width,
        resize_height,
        &mut output.0,
    );

    output.is_valid()
}
