use std::os::raw::c_longlong;

use crate::common::context::fill_and_stroke_styles::paint::PaintStyle;

#[repr(C)]
pub enum PaintStyleValueType {
    PaintStyleValueTypeColor = 0,
    PaintStyleValueTypeGradient = 1,
    PaintStyleValueTypePattern = 2,
}

impl Into<i32> for PaintStyleValueType {
    fn into(self) -> i32 {
        match self {
            PaintStyleValueType::PaintStyleValueTypeColor => 0,
            PaintStyleValueType::PaintStyleValueTypeGradient => 1,
            PaintStyleValueType::PaintStyleValueTypePattern => 2,
        }
    }
}

impl PaintStyleValueType {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(PaintStyleValueType::PaintStyleValueTypeColor),
            1 => Some(PaintStyleValueType::PaintStyleValueTypeGradient),
            2 => Some(PaintStyleValueType::PaintStyleValueTypePattern),
            _ => None,
        }
    }
}

#[repr(C)]
pub struct PaintStyleValue {
    pub value: c_longlong,
    pub value_type: PaintStyleValueType,
}

impl PaintStyleValue {
    pub fn new(value: PaintStyle, value_type: PaintStyleValueType) -> Self {
        Self {
            value: Box::into_raw(Box::new(value.clone())) as c_longlong,
            value_type,
        }
    }
}

#[no_mangle]
pub extern "C" fn destroy_paint_style_value(value: c_longlong) {
    if value == 0 {
        return;
    }
    unsafe {
        let value: *mut PaintStyleValue = value as _;
        Box::from_raw(value);
    }
}
