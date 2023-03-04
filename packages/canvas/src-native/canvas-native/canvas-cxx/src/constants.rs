#[cxx::bridge]
pub(crate) mod ffi {
    #[allow(non_camel_case_types)]
    #[repr(u32)]
    enum GLConstants {
        UNPACK_FLIP_Y_WEBGL = 0x9240,

        UNPACK_PREMULTIPLY_ALPHA_WEBGL = 0x9241,

        UNPACK_COLORSPACE_CONVERSION_WEBGL = 0x9243,
    }
}
