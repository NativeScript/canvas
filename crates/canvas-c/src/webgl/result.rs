#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
#[repr(C)]
pub enum WebGLResultType {
    Boolean,
    I32Array,
    U32Array,
    F32Array,
    BooleanArray,
    U32,
    I32,
    F32,
    String,
    None,
}
