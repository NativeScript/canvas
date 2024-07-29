pub mod gl;
pub mod gl2;
pub mod result;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
#[repr(C)]
pub enum GLConstants {
    UnPackFlipYWebGL = 0x9240,
    UnpackPremultiplyAlphaWebGL = 0x9241,
    UnpackColorSpaceConversionWebGL = 0x9243,
}
