use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

use canvas_2d::utils::image::from_image_slice;

use crate::c2d::context::CanvasRenderingContext2D;

mod gradient;
mod pattern;

#[derive(Clone)]
pub struct PaintStyle(pub(crate) canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle);

impl PaintStyle {
    pub fn new_with_color(color: *const c_char) -> Option<Self> {
        assert!(!color.is_null());
        let color = unsafe { CStr::from_ptr(color) };
        let color = color.to_string_lossy();
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::new_color_str(color.as_ref())
            .map(Self)
    }

    pub fn new(style: canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle) -> Self {
        Self(style)
    }

    pub fn style_type(&self) -> PaintStyleType {
        return match &self.0 {
            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(_) => {
                PaintStyleType::Color
            }
            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
                PaintStyleType::Gradient
            }
            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
                PaintStyleType::Pattern
            }
        };
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_paint_style_release(value: *mut PaintStyle) {
    if value.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(value)) };
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum PaintStyleType {
    Color,
    Gradient,
    Pattern,
}

#[no_mangle]
pub extern "C" fn canvas_native_paint_style_from_bytes(
    context: *const CanvasRenderingContext2D,
    repetition: i32,
    width: i32,
    height: i32,
    bytes: *const u8,
    size: usize,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    let bytes = unsafe { std::slice::from_raw_parts(bytes, size) };
    from_image_slice(bytes, width, height)
        .map(|image| {
            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                context.context.create_pattern(
                    image,
                    canvas_2d::context::fill_and_stroke_styles::pattern::Repetition::from(
                        repetition,
                    ),
                ),
            )
        })
        .map(|style| Box::into_raw(Box::new(PaintStyle(style))))
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_pattern_from_ptr(ptr: i64) -> *mut PaintStyle {
    if ptr == 0 {
        return ptr::null_mut();
    }
    ptr as *mut PaintStyle
}
