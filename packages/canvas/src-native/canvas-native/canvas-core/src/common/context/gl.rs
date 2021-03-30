use libc::size_t;
use std::ffi::{CStr, CString};
use std::mem;
//use gl_bindings;
use std::os::raw::{c_char, c_void};
use std::ptr::{null, null_mut};

pub type GLbitfield = u32;
pub type GLboolean = u8;
pub type GLbyte = i8;
pub type GLclampf = f32;
pub type GLenum = u32;
pub type GLfloat = f32;
pub type GLint = i32;
pub type GLshort = i16;
pub type GLsizei = i32;
pub type GLubyte = u8;
pub type GLuint = u32;
pub type GLushort = u16;
pub type GLvoid = ::std::os::raw::c_void;
pub type GLchar = ::std::os::raw::c_char;
pub type GLclampx = i32;
pub type GLfixed = i32;
pub type GLhalf = u16;

const SIZE_OF_U8: i32 = mem::size_of::<u8>() as i32;
const SIZE_OF_I8: i32 = mem::size_of::<i8>() as i32;
const SIZE_OF_U16: i32 = mem::size_of::<u16>() as i32;
const SIZE_OF_I16: i32 = mem::size_of::<i16>() as i32;
const SIZE_OF_I32: i32 = mem::size_of::<i32>() as i32;
const SIZE_OF_U32: i32 = mem::size_of::<u32>() as i32;
const SIZE_OF_F32: i32 = mem::size_of::<f32>() as i32;
const SIZE_OF_F64: i32 = mem::size_of::<f64>() as i32;

const GL_SCISSOR_TEST: u32 = 0x0C11;
const VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE: u32 = 0x88FE;
//
// type EAGLContext = gl_bindings::EAGLContext;
//
// #[no_mangle]
// pub extern "C" fn gl_set_current_context(context: *mut objc::runtime::Object) {
//     unsafe {
//         let cls = class!(EAGLContext);
//         //gl_bindings::EAGLContext::setCurrentContext_(context)
//         let _: () = msg_send![cls, setCurrentContext: context];
//     }
// }
//
// // #[no_mangle]
// // pub extern "C" fn gl_get_current_context() -> *mut NSObject {
// //     unsafe {
// //         let cls = class!(EAGLContext);
// //         //gl_bindings::EAGLContext::currentContext()
// //         return msg_send!(cls, currentContext);
// //     }
// // }

#[no_mangle]
pub extern "C" fn gl_active_texture(texture: GLuint) {
    unsafe {
        gl_bindings::glActiveTexture(texture)
    }
}

#[no_mangle]
pub extern "C" fn gl_begin_query(target: GLuint, query: GLuint) {
    unsafe {
        gl_bindings::glBeginQuery(target, query)
    }
}


#[no_mangle]
pub extern "C" fn gl_attach_shader(program: GLuint, shader: GLuint) {
    unsafe {
        gl_bindings::glAttachShader(program, shader)
    }
}

#[no_mangle]
pub extern "C" fn gl_bind_attrib_location(program: GLuint, index: GLuint, name: *const c_char) {
    unsafe {
        gl_bindings::glBindAttribLocation(program, index, name)
    }
}

#[no_mangle]
pub extern "C" fn gl_bind_buffer(target: GLuint, buffer: GLuint) {
    unsafe {
        gl_bindings::glBindBuffer(target, buffer)
    }
}

#[no_mangle]
pub extern "C" fn gl_bind_framebuffer(target: GLuint, framebuffer: GLuint) {
    unsafe {
        gl_bindings::glBindFramebuffer(target, framebuffer)
    }
}

#[no_mangle]
pub extern "C" fn gl_bind_renderbuffer(target: GLuint, renderbuffer: GLuint) {
    unsafe {
        gl_bindings::glBindRenderbuffer(target, renderbuffer)
    }
}

#[no_mangle]
pub extern "C" fn gl_bind_texture(target: GLuint, texture: GLuint) {
    unsafe {
        gl_bindings::glBindTexture(target, texture)
    }
}

#[no_mangle]
pub extern "C" fn gl_blend_color(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    unsafe {
        gl_bindings::glBlendColor(red, green, blue, alpha)
    }
}

#[no_mangle]
pub extern "C" fn gl_blend_equation(mode: GLuint) {
    unsafe {
        gl_bindings::glBlendEquation(mode)
    }
}

#[no_mangle]
pub extern "C" fn gl_blend_equation_separate(mode_rgb: GLuint, mode_alpha: GLuint) {
    unsafe {
        gl_bindings::glBlendEquationSeparate(mode_rgb, mode_alpha)
    }
}

#[no_mangle]
pub extern "C" fn gl_blend_func(sfactor: GLuint, dfactor: GLuint) {
    unsafe {
        gl_bindings::glBlendFunc(sfactor, dfactor)
    }
}

#[no_mangle]
pub extern "C" fn gl_blend_func_separate(src_rgb: GLuint, dst_rgb: GLuint, src_alpha: GLuint, dst_alpha: GLuint) {
    unsafe {
        gl_bindings::glBlendFuncSeparate(src_rgb, dst_rgb, src_alpha, dst_alpha)
    }
}

#[no_mangle]
pub extern "C" fn gl_buffer_data(target: GLuint, size: isize, data: *const c_void, usage: GLuint) {
    unsafe {
        gl_bindings::glBufferData(target, size, data, usage)
    }
}

#[no_mangle]
pub extern "C" fn gl_buffer_data_no_data(target: GLuint, src_data: *const c_void, usage: GLuint) {
    unsafe {
        gl_bindings::glBufferData(target, 0, src_data, usage)
    }
}

#[no_mangle]
pub extern "C" fn gl_buffer_data_u8(target: GLuint, data: *const u8, size: isize, usage: GLuint) {
    let count = std::mem::size_of::<u8>() * (size as usize);
    let buffer = unsafe { std::slice::from_raw_parts(data, size as usize) };
    unsafe {
        gl_bindings::glBufferData(target, count as isize, buffer.as_ptr() as *const c_void, usage)
    }
}

#[no_mangle]
pub extern "C" fn gl_buffer_data_i8(target: GLuint, data: *const i8, size: isize, usage: GLuint) {
    let count = std::mem::size_of::<i8>() * (size as usize);
    let buffer = unsafe { std::slice::from_raw_parts(data, size as usize) };
    unsafe {
        gl_bindings::glBufferData(target, count as isize, buffer.as_ptr() as *const c_void, usage)
    }
}

#[no_mangle]
pub extern "C" fn gl_buffer_data_u16(target: GLuint, data: *const u16, size: isize, usage: GLuint) {
    let count = std::mem::size_of::<u16>() * (size as usize);
    let buffer = unsafe { std::slice::from_raw_parts(data, size as usize) };
    unsafe {
        gl_bindings::glBufferData(target, count as isize, buffer.as_ptr() as *const c_void, usage)
    }
}

#[no_mangle]
pub extern "C" fn gl_buffer_data_i16(target: GLuint, data: *const i16, size: isize, usage: GLuint) {
    let count = std::mem::size_of::<i16>() * (size as usize);
    let buffer = unsafe { std::slice::from_raw_parts(data, size as usize) };
    unsafe {
        gl_bindings::glBufferData(target, count as isize, buffer.as_ptr() as *const c_void, usage)
    }
}

#[no_mangle]
pub extern "C" fn gl_buffer_data_u32(target: GLuint, data: *const GLuint, size: isize, usage: GLuint) {
    let count = std::mem::size_of::<GLuint>() * (size as usize);
    let buffer = unsafe { std::slice::from_raw_parts(data, size as usize) };
    unsafe {
        gl_bindings::glBufferData(target, count as isize, buffer.as_ptr() as *const c_void, usage)
    }
}


#[no_mangle]
pub extern "C" fn gl_buffer_data_i32(target: GLuint, data: *const i32, size: isize, usage: GLuint) {
    let count = std::mem::size_of::<i32>() * (size as usize);
    let buffer = unsafe { std::slice::from_raw_parts(data, size as usize) };
    unsafe {
        gl_bindings::glBufferData(target, count as isize, buffer.as_ptr() as *const c_void, usage)
    }
}


#[no_mangle]
pub extern "C" fn gl_buffer_data_f32(target: GLuint, data: *const GLfloat, size: isize, usage: GLuint) {
    let count = std::mem::size_of::<GLfloat>() * (size as usize);
    let buffer = unsafe { std::slice::from_raw_parts(data, size as usize) };
    unsafe {
        gl_bindings::glBufferData(target, count as isize, buffer.as_ptr() as *const c_void, usage)
    }
}


#[no_mangle]
pub extern "C" fn gl_buffer_data_f64(target: GLuint, data: *const f64, size: isize, usage: GLuint) {
    let count = std::mem::size_of::<f64>() * (size as usize);
    let buffer = unsafe { std::slice::from_raw_parts(data, size as usize) };
    unsafe {
        gl_bindings::glBufferData(target, count as isize, buffer.as_ptr() as *const c_void, usage)
    }
}


#[no_mangle]
pub extern "C" fn gl_buffer_sub_data(target: GLuint, offset: usize, src_data: *const c_void, size: usize) {
    unsafe {
        gl_bindings::glBufferSubData(target, offset as isize, size as isize, src_data);
    }
}


#[no_mangle]
pub extern "C" fn gl_buffer_sub_data_u8(target: GLuint, offset: usize, src_data: *const u8, size: usize) {
    unsafe {
        let buffer = std::slice::from_raw_parts(src_data, size);
        let count = mem::size_of::<u8>() * size;
        let os = mem::size_of::<u8>() * offset;
        gl_bindings::glBufferSubData(target, os as isize, count as isize, buffer.as_ptr() as *const c_void);
    }
}


#[no_mangle]
pub extern "C" fn gl_buffer_sub_data_i8(target: GLuint, offset: usize, src_data: *const i8, size: usize) {
    unsafe {
        let buffer = std::slice::from_raw_parts(src_data, size);
        let count = mem::size_of::<i8>() * size;
        let os = mem::size_of::<i8>() * offset;
        gl_bindings::glBufferSubData(target, os as isize, count as isize, buffer.as_ptr() as *const c_void);
    }
}


#[no_mangle]
pub extern "C" fn gl_buffer_sub_data_u16(target: GLuint, offset: usize, src_data: *const u16, size: usize) {
    unsafe {
        let buffer = std::slice::from_raw_parts(src_data, size);
        let count = mem::size_of::<u16>() * size;
        let os = mem::size_of::<u16>() * offset;
        gl_bindings::glBufferSubData(target, os as isize, count as isize, buffer.as_ptr() as *const c_void);
    }
}


#[no_mangle]
pub extern "C" fn gl_buffer_sub_data_i16(target: GLuint, offset: usize, src_data: *const i16, size: usize) {
    unsafe {
        let buffer = std::slice::from_raw_parts(src_data, size);
        let count = mem::size_of::<i16>() * size;
        let os = mem::size_of::<i16>() * offset;
        gl_bindings::glBufferSubData(target, os as isize, count as isize, buffer.as_ptr() as *const c_void);
    }
}


#[no_mangle]
pub extern "C" fn gl_buffer_sub_data_u32(target: GLuint, offset: usize, src_data: *const GLuint, size: usize) {
    unsafe {
        let buffer = std::slice::from_raw_parts(src_data, size);
        let count = mem::size_of::<GLuint>() * size;
        let os = mem::size_of::<GLuint>() * offset;
        gl_bindings::glBufferSubData(target, os as isize, count as isize, buffer.as_ptr() as *const c_void);
    }
}


#[no_mangle]
pub extern "C" fn gl_buffer_sub_data_i32(target: GLuint, offset: usize, src_data: *const i32, size: usize) {
    unsafe {
        let buffer = std::slice::from_raw_parts(src_data, size);
        let count = mem::size_of::<i32>() * size;
        let os = mem::size_of::<i32>() * offset;
        gl_bindings::glBufferSubData(target, os as isize, count as isize, buffer.as_ptr() as *const c_void);
    }
}


#[no_mangle]
pub extern "C" fn gl_buffer_sub_data_f32(target: GLuint, offset: usize, src_data: *const GLfloat, size: usize) {
    unsafe {
        let buffer = std::slice::from_raw_parts(src_data, size);
        let count = mem::size_of::<GLfloat>() * size;
        let os = mem::size_of::<GLfloat>() * offset;
        gl_bindings::glBufferSubData(target, os as isize, count as isize, buffer.as_ptr() as *const c_void);
    }
}


#[no_mangle]
pub extern "C" fn gl_buffer_sub_data_f64(target: GLuint, offset: usize, src_data: *const f64, size: usize) {
    unsafe {
        let buffer = std::slice::from_raw_parts(src_data, size);
        let count = mem::size_of::<f64>() * size;
        let os = mem::size_of::<f64>() * offset;
        gl_bindings::glBufferSubData(target, os as isize, count as isize, buffer.as_ptr() as *const c_void);
    }
}


#[no_mangle]
pub extern "C" fn gl_check_framebuffer_status(target: GLenum) -> GLenum {
    return unsafe { gl_bindings::glCheckFramebufferStatus(target) };
}

#[no_mangle]
pub extern "C" fn gl_clear(mask: GLbitfield) {
    unsafe {
        gl_bindings::glClear(mask)
    }
}

#[no_mangle]
pub extern "C" fn gl_clear_color(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    unsafe {
        gl_bindings::glClearColor(red, green, blue, alpha)
    }
}

#[no_mangle]
pub extern "C" fn gl_clear_depth(depth: GLclampf) {
    unsafe {
        gl_bindings::glClearDepthf(depth)
    }
}

#[no_mangle]
pub extern "C" fn gl_clear_stencil(stencil: GLint) {
    unsafe {
        gl_bindings::glClearStencil(stencil)
    }
}

#[no_mangle]
pub extern "C" fn gl_color_mask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
    unsafe {
        gl_bindings::glColorMask(red, green, blue, alpha)
    }
}

#[no_mangle]
pub extern "C" fn gl_commit() {
// NOOP
}

#[no_mangle]
pub extern "C" fn gl_compile_shader(shader: GLuint) {
    unsafe {
        gl_bindings::glCompileShader(shader)
    }
}


#[no_mangle]
pub extern "C" fn gl_compressed_tex_image2d(target: GLuint, level: GLint, internalformat: GLuint, width: GLint, height: GLint, border: GLint) {
    unsafe {
        gl_bindings::glCompressedTexImage2D(target, level, internalformat, width, height, border, 0, null())
    }
}


#[no_mangle]
pub extern "C" fn gl_compressed_tex_image2d_u8(target: GLuint, level: GLint, internalformat: GLuint, width: GLint, height: GLint, border: GLint, pixels: *const u8, size: i32) {
    let buffer = unsafe { std::slice::from_raw_parts(pixels, size as usize) };
    let count = SIZE_OF_U8 * size;
    unsafe {
        gl_bindings::glCompressedTexImage2D(target, level, internalformat, width, height, border, count, buffer.as_ptr() as *const c_void)
    }
}

#[no_mangle]
pub extern "C" fn gl_compressed_tex_image2d_i8(target: GLuint, level: GLint, internalformat: GLuint, width: GLint, height: GLint, border: GLint, pixels: *const i8, size: i32) {
    let buffer = unsafe { std::slice::from_raw_parts(pixels, size as usize) };
    let count = SIZE_OF_I8 * size;
    unsafe {
        gl_bindings::glCompressedTexImage2D(target, level, internalformat, width, height, border, count, buffer.as_ptr() as *const c_void)
    }
}


#[no_mangle]
pub extern "C" fn gl_compressed_tex_image2d_u16(target: GLuint, level: GLint, internalformat: GLuint, width: GLint, height: GLint, border: GLint, pixels: *const u16, size: i32) {
    let buffer = unsafe { std::slice::from_raw_parts(pixels, size as usize) };
    let count = SIZE_OF_U16 * size;
    unsafe {
        gl_bindings::glCompressedTexImage2D(target, level, internalformat, width, height, border, count, buffer.as_ptr() as *const c_void)
    }
}

#[no_mangle]
pub extern "C" fn gl_compressed_tex_image2d_i16(target: GLuint, level: GLint, internalformat: GLuint, width: GLint, height: GLint, border: GLint, pixels: *const i16, size: i32) {
    let buffer = unsafe { std::slice::from_raw_parts(pixels, size as usize) };
    let count = SIZE_OF_I16 * size;
    unsafe {
        gl_bindings::glCompressedTexImage2D(target, level, internalformat, width, height, border, count, buffer.as_ptr() as *const c_void)
    }
}


#[no_mangle]
pub extern "C" fn gl_compressed_tex_image2d_u32(target: GLuint, level: GLint, internalformat: GLuint, width: GLint, height: GLint, border: GLint, pixels: *const GLuint, size: i32) {
    let buffer = unsafe { std::slice::from_raw_parts(pixels, size as usize) };
    let count = SIZE_OF_U32 * size;
    unsafe {
        gl_bindings::glCompressedTexImage2D(target, level, internalformat, width, height, border, count, buffer.as_ptr() as *const c_void)
    }
}

#[no_mangle]
pub extern "C" fn gl_compressed_tex_image2d_i32(target: GLuint, level: GLint, internalformat: GLuint, width: GLint, height: GLint, border: GLint, pixels: *const GLint, size: i32) {
    let buffer = unsafe { std::slice::from_raw_parts(pixels, size as usize) };
    let count = SIZE_OF_I32 * size;
    unsafe {
        gl_bindings::glCompressedTexImage2D(target, level, internalformat, width, height, border, count, buffer.as_ptr() as *const c_void)
    }
}


#[no_mangle]
pub extern "C" fn gl_compressed_tex_image2d_f32(target: GLuint, level: GLint, internalformat: GLuint, width: GLint, height: GLint, border: GLint, pixels: *const GLfloat, size: i32) {
    let buffer = unsafe { std::slice::from_raw_parts(pixels, size as usize) };
    let count = SIZE_OF_F32 * size;
    unsafe {
        gl_bindings::glCompressedTexImage2D(target, level, internalformat, width, height, border, count, buffer.as_ptr() as *const c_void)
    }
}

#[no_mangle]
pub extern "C" fn gl_compressed_tex_image2d_f64(target: GLuint, level: GLint, internalformat: GLuint, width: GLint, height: GLint, border: GLint, pixels: *const f64, size: i32) {
    let buffer = unsafe { std::slice::from_raw_parts(pixels, size as usize) };
    let count = SIZE_OF_F64 * size;
    unsafe {
        gl_bindings::glCompressedTexImage2D(target, level, internalformat, width, height, border, count, buffer.as_ptr() as *const c_void)
    }
}


#[no_mangle]
pub extern "C" fn gl_compressed_tex_sub_image_2d(target: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLint, height: GLint, format: GLuint) {
    unsafe {
        gl_bindings::glCompressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, 0, null())
    }
}


#[no_mangle]
pub extern "C" fn gl_compressed_tex_sub_image_2d_u8(target: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLint, height: GLint, format: GLuint, pixels: *const u8, size: i32) {
    let buffer = unsafe { std::slice::from_raw_parts(pixels, size as usize) };
    let count = SIZE_OF_U8 * size;
    let xos = SIZE_OF_U8 * xoffset;
    let yos = SIZE_OF_U8 * yoffset;
    unsafe {
        gl_bindings::glCompressedTexSubImage2D(target, level, xos, yos, width, height, format, count, buffer.as_ptr() as *const c_void)
    }
}


#[no_mangle]
pub extern "C" fn gl_compressed_tex_sub_image_2d_i8(target: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLint, height: GLint, format: GLuint, pixels: *const i8, size: i32) {
    let buffer = unsafe { std::slice::from_raw_parts(pixels, size as usize) };
    let count = SIZE_OF_I8 * size;
    let xos = SIZE_OF_I8 * xoffset;
    let yos = SIZE_OF_I8 * yoffset;
    unsafe {
        gl_bindings::glCompressedTexSubImage2D(target, level, xos, yos, width, height, format, count, buffer.as_ptr() as *const c_void)
    }
}


#[no_mangle]
pub extern "C" fn gl_compressed_tex_sub_image_2d_u16(target: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLint, height: GLint, format: GLuint, pixels: *const u16, size: i32) {
    let buffer = unsafe { std::slice::from_raw_parts(pixels, size as usize) };
    let count = SIZE_OF_U16 * size;
    let xos = SIZE_OF_U16 * xoffset;
    let yos = SIZE_OF_U16 * yoffset;
    unsafe {
        gl_bindings::glCompressedTexSubImage2D(target, level, xos, yos, width, height, format, count, buffer.as_ptr() as *const c_void)
    }
}


#[no_mangle]
pub extern "C" fn gl_compressed_tex_sub_image_2d_i16(target: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLint, height: GLint, format: GLuint, pixels: *const i16, size: i32) {
    let buffer = unsafe { std::slice::from_raw_parts(pixels, size as usize) };
    let count = SIZE_OF_I16 * size;
    let xos = SIZE_OF_I16 * xoffset;
    let yos = SIZE_OF_I16 * yoffset;
    unsafe {
        gl_bindings::glCompressedTexSubImage2D(target, level, xos, yos, width, height, format, count, buffer.as_ptr() as *const c_void)
    }
}

#[no_mangle]
pub extern "C" fn gl_compressed_tex_sub_image_2d_u32(target: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLint, height: GLint, format: GLuint, pixels: *const GLuint, size: i32) {
    let buffer = unsafe { std::slice::from_raw_parts(pixels, size as usize) };
    let count = SIZE_OF_U32 * size;
    let xos = SIZE_OF_U32 * xoffset;
    let yos = SIZE_OF_U32 * yoffset;
    unsafe {
        gl_bindings::glCompressedTexSubImage2D(target, level, xos, yos, width, height, format, count, buffer.as_ptr() as *const c_void)
    }
}


#[no_mangle]
pub extern "C" fn gl_compressed_tex_sub_image_2d_i32(target: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLint, height: GLint, format: GLuint, pixels: *const i32, size: i32) {
    let buffer = unsafe { std::slice::from_raw_parts(pixels, size as usize) };
    let count = SIZE_OF_I32 * size;
    let xos = SIZE_OF_I32 * xoffset;
    let yos = SIZE_OF_I32 * yoffset;
    unsafe {
        gl_bindings::glCompressedTexSubImage2D(target, level, xos, yos, width, height, format, count, buffer.as_ptr() as *const c_void)
    }
}


#[no_mangle]
pub extern "C" fn gl_compressed_tex_sub_image_2df32(target: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLint, height: GLint, format: GLuint, pixels: *const GLfloat, size: i32) {
    let buffer = unsafe { std::slice::from_raw_parts(pixels, size as usize) };
    let count = SIZE_OF_F32 * size;
    let xos = SIZE_OF_F32 * xoffset;
    let yos = SIZE_OF_F32 * yoffset;
    unsafe {
        gl_bindings::glCompressedTexSubImage2D(target, level, xos, yos, width, height, format, count as i32, buffer.as_ptr() as *const c_void)
    }
}


#[no_mangle]
pub extern "C" fn gl_compressed_tex_sub_image_2d_f64(target: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLint, height: GLint, format: GLuint, pixels: *const f64, size: i32) {
    let buffer = unsafe { std::slice::from_raw_parts(pixels, size as usize) };
    let count = SIZE_OF_F64 * size;
    let xos = SIZE_OF_F64 * xoffset;
    let yos = SIZE_OF_F64 * yoffset;
    unsafe {
        gl_bindings::glCompressedTexSubImage2D(target, level, xos as i32, yos as i32, width, height, format, count as i32, buffer.as_ptr() as *const c_void)
    }
}


#[no_mangle]
pub extern "C" fn gl_copy_tex_image2d(target: GLuint, level: GLint, internalformat: GLuint, x: GLint, y: GLint, width: GLint, height: GLint, border: GLint) {
    unsafe {
        gl_bindings::glCopyTexImage2D(target, level, internalformat, x, y, width, height, border)
    }
}


#[no_mangle]
pub extern "C" fn gl_copy_tex_sub_image2d(target: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLint, height: GLint) {
    unsafe {
        gl_bindings::glCopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height)
    }
}

#[no_mangle]
pub extern "C" fn gl_create_buffer() -> GLuint {
    let mut buffer_id: GLuint = 0;
    unsafe {
        gl_bindings::glGenBuffers(1, &mut buffer_id);
    }
    return buffer_id;
}

#[no_mangle]
pub extern "C" fn gl_create_framebuffer() -> GLuint {
    let mut frame_buffer_id: GLuint = 0;
    unsafe {
        gl_bindings::glGenFramebuffers(1, &mut frame_buffer_id)
    }
    return frame_buffer_id;
}

#[no_mangle]
pub extern "C" fn gl_create_program() -> GLuint {
    return unsafe { gl_bindings::glCreateProgram() };
}

#[no_mangle]
pub extern "C" fn gl_create_renderbuffer() -> GLuint {
    let mut render_buffer_id: GLuint = 0;
    unsafe { gl_bindings::glGenRenderbuffers(1, &mut render_buffer_id) }
    return render_buffer_id;
}


#[no_mangle]
pub extern "C" fn gl_create_shader(shader_type: GLuint) -> GLuint {
    return unsafe { gl_bindings::glCreateShader(shader_type) };
}

#[no_mangle]
pub extern "C" fn gl_create_texture() -> GLuint {
    let mut texture_id: GLuint = 0;
    unsafe { gl_bindings::glGenTextures(1, &mut texture_id) }
    return texture_id;
}

#[no_mangle]
pub extern "C" fn gl_cull_face(mode: GLuint) {
    unsafe { gl_bindings::glCullFace(mode) }
}

#[no_mangle]
pub extern "C" fn gl_delete_buffer(buffer: GLuint) {
    let mut buf = buffer;
    unsafe { gl_bindings::glDeleteBuffers(1, &mut buf) }
}

#[no_mangle]
pub extern "C" fn gl_delete_framebuffer(frame_buffer: GLuint) {
    unsafe { gl_bindings::glDeleteFramebuffers(1, &frame_buffer) }
}

#[no_mangle]
pub extern "C" fn gl_delete_program(program: GLuint) {
    unsafe { gl_bindings::glDeleteProgram(program) }
}

#[no_mangle]
pub extern "C" fn gl_delete_renderbuffer(render_buffer: GLuint) {
    unsafe { gl_bindings::glDeleteRenderbuffers(1, &render_buffer) }
}

#[no_mangle]
pub extern "C" fn gl_delete_shader(shader: GLuint) {
    unsafe { gl_bindings::glDeleteShader(shader) }
}

#[no_mangle]
pub extern "C" fn gl_delete_texture(texture: GLuint) {
    unsafe { gl_bindings::glDeleteTextures(1, &texture) }
}

#[no_mangle]
pub extern "C" fn gl_depth_func(func: GLuint) {
    unsafe { gl_bindings::glDepthFunc(func) }
}

#[no_mangle]
pub extern "C" fn gl_depth_mask(flag: GLboolean) {
    unsafe { gl_bindings::glDepthMask(flag) }
}

#[no_mangle]
pub extern "C" fn gl_depth_range(z_near: GLfloat, z_far: GLfloat) {
    unsafe { gl_bindings::glDepthRangef(z_near, z_far) }
}

#[no_mangle]
pub extern "C" fn gl_detach_shader(program: GLuint, shader: GLuint) {
    unsafe { gl_bindings::glDetachShader(program, shader) }
}

#[no_mangle]
pub extern "C" fn gl_disable(cap: GLuint) {
    unsafe { gl_bindings::glDisable(cap) }
}

#[no_mangle]
pub extern "C" fn gl_disable_vertex_attrib_array(index: GLuint) {
    unsafe { gl_bindings::glDisableVertexAttribArray(index) }
}

#[no_mangle]
pub extern "C" fn gl_draw_arrays(mode: GLuint, first: GLint, count: GLsizei) {
    unsafe { gl_bindings::glDrawArrays(mode, first, count) }
}

#[no_mangle]
pub extern "C" fn gl_draw_elements(mode: GLuint, count: GLint, element_type: GLuint, offset: isize) {
    let mut os = null() as *const c_void;
    unsafe { gl_bindings::glDrawElements(mode, count, element_type, null()) }
}

#[no_mangle]
pub extern "C" fn gl_enable(cap: GLuint) {
    unsafe { gl_bindings::glEnable(cap) }
}

#[no_mangle]
pub extern "C" fn gl_enable_vertex_attrib_array(index: GLuint) {
    unsafe { gl_bindings::glEnableVertexAttribArray(index) }
}

#[no_mangle]
pub extern "C" fn gl_finish() {
    unsafe { gl_bindings::glFinish() }
}

#[no_mangle]
pub extern "C" fn gl_flush() {
    unsafe { gl_bindings::glFlush() }
}


#[no_mangle]
pub extern "C" fn gl_framebuffer_renderbuffer(target: GLuint, attachment: GLuint, renderbuffertarget: GLuint, renderbuffer: GLuint) {
    /*if(attachment == GL_DEPTH_ATTACHMENT){
     if let renderer = canvas.renderer as? GLRenderer {
     glFramebufferRenderbuffer(GLenum(target), GLenum(GL_DEPTH_ATTACHMENT), GLenum(renderbuffertarget),renderer.displayRenderbuffer)

     glFramebufferRenderbuffer(GLenum(target), GLenum(GL_DEPTH_ATTACHMENT), GLenum(renderbuffertarget), renderbuffer)

     glFramebufferRenderbuffer(GLenum(target), GLenum(GL_STENCIL_ATTACHMENT), GLenum(renderbuffertarget), renderer.displayRenderbuffer)
     }
     }else if(attachment == GL_STENCIL_ATTACHMENT){
     if let renderer = canvas.renderer as? GLRenderer {
     glFramebufferRenderbuffer(GLenum(target), GLenum(GL_DEPTH_STENCIL_ATTACHMENT), GLenum(GL_RENDERBUFFER),renderer.displayRenderbuffer)

     glFramebufferRenderbuffer(GLenum(target), GLenum(GL_DEPTH_ATTACHMENT), GLenum(GL_RENDERBUFFER), renderer.displayRenderbuffer)


     glFramebufferRenderbuffer(GLenum(target), GLenum(GL_STENCIL_ATTACHMENT), GLenum(GL_RENDERBUFFER), renderbuffer)

     }
     }else if(attachment == GL_DEPTH_STENCIL_ATTACHMENT){
     if let renderer = canvas.renderer as? GLRenderer {
     glFramebufferRenderbuffer(GLenum(target), GLenum(GL_DEPTH_ATTACHMENT), GLenum(GL_RENDERBUFFER),renderer.displayRenderbuffer)


     glFramebufferRenderbuffer(GLenum(target), GLenum(GL_STENCIL_ATTACHMENT), GLenum(GL_RENDERBUFFER), renderer.displayRenderbuffer)

     glFramebufferRenderbuffer(GLenum(target), GLenum(GL_DEPTH_STENCIL_ATTACHMENT), GLenum(GL_RENDERBUFFER), renderbuffer)


     }
     }else {
     /*
     glFramebufferRenderbuffer(GLenum(target), GLenum(attachment), GLenum(renderbuffertarget), renderbuffer)
     */
     if let renderer = canvas.renderer as? GLRenderer {
     glFramebufferRenderbuffer(GLenum(target), GLenum(GL_DEPTH_STENCIL_ATTACHMENT), GLenum(GL_RENDERBUFFER), renderer.displayRenderbuffer)

     glFramebufferRenderbuffer(GLenum(target), GLenum(GL_DEPTH_ATTACHMENT), GLenum(GL_RENDERBUFFER), renderer.displayRenderbuffer)

     glFramebufferRenderbuffer(GLenum(target), GLenum(GL_STENCIL_ATTACHMENT), GLenum(GL_RENDERBUFFER), renderer.displayRenderbuffer)
     }
     }
     */

    unsafe { gl_bindings::glFramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer) }
}

#[no_mangle]
pub extern "C" fn gl_framebuffer_texture2d(target: GLuint, attachment: GLuint, textarget: GLuint, texture: GLuint, level: GLint) {
    unsafe { gl_bindings::glFramebufferTexture2D(target, attachment, textarget, texture, level) }
}

#[no_mangle]
pub extern "C" fn gl_front_face(mode: GLuint) {
    unsafe { gl_bindings::glFrontFace(mode) }
}

#[no_mangle]
pub extern "C" fn gl_generate_mipmap(target: GLuint) {
    unsafe { gl_bindings::glGenerateMipmap(target) }
}

#[repr(C)]
pub struct NativeWebGLActiveInfo {
    pub(crate) name: *const c_char,
    pub(crate) size: GLint,
    pub(crate) info_type: GLuint,
}

impl NativeWebGLActiveInfo {
    pub fn new(name: *const c_char, size: GLint, info_type: GLuint) -> Self {
        Self {
            name,
            size,
            info_type,
        }
    }
}

#[no_mangle]
pub extern "C" fn gl_get_active_attrib(program: GLuint, index: GLuint) -> *const NativeWebGLActiveInfo {
    let mut length: GLint = 0;
    let mut size: GLint = 0;
    let mut info_type: GLenum = 0;
    let mut name_length: GLint = 0;
    unsafe { gl_bindings::glGetProgramiv(program, gl_bindings::GL_ACTIVE_ATTRIBUTE_MAX_LENGTH, &mut length) }
    let mut name_buffer = String::with_capacity(length as usize);
    unsafe { gl_bindings::glGetActiveAttrib(program, index, length, &mut name_length, &mut size, &mut info_type, name_buffer.as_mut_ptr() as *mut c_char) }
    // TODO test if it needs to resize ???
    // name_buffer.resize(name_length, 0);
    let name = CString::new(name_buffer).unwrap_or(CString::default());
    Box::into_raw(Box::new(NativeWebGLActiveInfo::new(name.into_raw(), size, info_type)))
}


#[no_mangle]
pub extern "C" fn gl_get_active_uniform(program: GLuint, index: GLuint) -> *const NativeWebGLActiveInfo {
    let mut length: GLint = 0;
    let mut size: GLint = 0;
    let mut info_type: GLenum = 0;
    let mut name_length: GLint = 0;
    unsafe { gl_bindings::glGetProgramiv(program, gl_bindings::GL_ACTIVE_UNIFORM_MAX_LENGTH, &mut length) }
    let mut name_buffer = String::with_capacity(length as usize);
    unsafe { gl_bindings::glGetActiveUniform(program, index, length, &mut name_length, &mut size, &mut info_type, name_buffer.as_mut_ptr() as *mut c_char) };
    // TODO test resize
    // name.resize(name_length, 0);
    let name = CString::new(name_buffer).unwrap_or(CString::default());
    Box::into_raw(Box::new(NativeWebGLActiveInfo::new(name.into_raw(), size, info_type)))
}


#[repr(C)]
pub struct NativeU32Array {
    pub array: *const u32,
    pub length: size_t,
}

#[no_mangle]
pub extern "C" fn gl_get_attached_shaders(program: GLuint) -> NativeU32Array {
    let mut count: GLint = 0;
    unsafe { gl_bindings::glGetProgramiv(program, gl_bindings::GL_ATTACHED_SHADERS, &mut count) }
    let mut shaders = vec![0u32; count as usize];
    let mut len = 0;
    unsafe { gl_bindings::glGetAttachedShaders(program, count, &mut len, shaders.as_mut_ptr()) }
    let shaders = shaders.into_boxed_slice();
    NativeU32Array {
        array: shaders.as_ptr(),
        length: shaders.len(),
    }
}

#[no_mangle]
pub extern "C" fn gl_get_attrib_location(program: GLuint, name: *const c_char) -> GLint {
    unsafe { gl_bindings::glGetAttribLocation(program, name) }
}

#[no_mangle]
pub extern "C" fn gl_get_buffer_parameter(target: GLuint, pname: GLuint) -> GLint {
    let mut params: GLint = 0;
    unsafe { gl_bindings::glGetBufferParameteriv(target, pname, &mut params) }
    return params;
}


// private var alpha: Bool = true
// private var antialias: Bool = false
// private var depth:Bool = true
// private var failIfMajorPerformanceCaveat:Bool = false
// private var powerPreference:String = "default"
// private var premultipliedAlpha: Bool = false
// private var preserveDrawingBuffer:Bool = false
// private var stencil:Bool = false
// private var desynchronized:Bool = false


#[repr(C)]
pub struct NativeWebGLShaderPrecisionFormat {
    pub range_min: i32,
    pub range_max: i32,
    pub precision: i32,
}

#[repr(C)]
pub struct NativeAnyItem {
    pub name: *const c_char,
    pub value: NativeAnyValue,
}

impl NativeAnyItem {
    pub fn new(name: *const c_char, value: NativeAnyValue) -> Self {
        Self {
            name,
            value,
        }
    }
}

#[repr(C)]
pub enum NativeAnyValue {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    F32(f32),
    F64(f64),
    Bool(u8),
    String(*const c_char),
    Null,
    I8Array(*const i8, usize),
    U8Array(*const u8, usize),
    I16Array(*const i16, usize),
    U16Array(*const u16, usize),
    I32Array(*const i32, usize),
    U32Array(*const u32, usize),
    F32Array(*const f32, usize),
    F64Array(*const f64, usize),
    BoolArray(*const u8, usize),
    StringArray(*const c_char, usize),
}


impl Drop for NativeAnyValue {
    fn drop(&mut self) {
        match self {
            NativeAnyValue::I8Array(value, size) => {
                if value.is_null() {
                    return;
                }
                let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(value, *size)) };
            }
            NativeAnyValue::U8Array(value, size) => {
                if value.is_null() {
                    return;
                }
                let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(value, *size)) };
            }
            NativeAnyValue::I16Array(value, size) => {
                if value.is_null() {
                    return;
                }
                let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(value, *size)) };
            }
            NativeAnyValue::U16Array(value, size) => {
                if value.is_null() {
                    return;
                }
                let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(value, *size)) };
            }
            NativeAnyValue::I32Array(value, size) => {
                if value.is_null() {
                    return;
                }
                let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(value, *size)) };
            }
            NativeAnyValue::U32Array(value, size) => {
                if value.is_null() {
                    return;
                }
                let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(value, *size)) };
            }

            NativeAnyValue::F32Array(value, size) => {
                if value.is_null() {
                    return;
                }
                let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(value, *size)) };
            }

            NativeAnyValue::F64Array(value, size) => {
                if value.is_null() {
                    return;
                }
                let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(value, *size)) };
            }
            NativeAnyValue::BoolArray(value, size) => {
                if value.is_null() {
                    return;
                }
                let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(value, *size)) };
            }

            NativeAnyValue::StringArray(value, size) => {
                if value.is_null() {
                    return;
                }
                let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(value, *size)) };
            }
            _ => {}
        }
    }
}


#[repr(C)]
pub struct NativeAnyItemArray {
    pub array: *const NativeAnyItem,
    pub length: size_t,
}

#[no_mangle]
pub extern "C" fn gl_get_context_attributes() -> *mut NativeAnyItemArray {
    let mut attributes: Vec<NativeAnyItem> = Vec::with_capacity(9);
    attributes.push(NativeAnyItem::new(CString::new("alpha").unwrap().into_raw(), NativeAnyValue::Bool(1)));
    attributes.push(NativeAnyItem::new(CString::new("antialias").unwrap().into_raw(), NativeAnyValue::Bool(0)));
    attributes.push(NativeAnyItem::new(CString::new("depth").unwrap().into_raw(), NativeAnyValue::Bool(1)));
    attributes.push(NativeAnyItem::new(CString::new("failIfMajorPerformanceCaveat").unwrap().into_raw(), NativeAnyValue::Bool(0)));
    attributes.push(NativeAnyItem::new(CString::new("powerPreference").unwrap().into_raw(), NativeAnyValue::String(CString::new("default").unwrap().into_raw())));
    attributes.push(NativeAnyItem::new(CString::new("premultipliedAlpha").unwrap().into_raw(), NativeAnyValue::Bool(0)));
    attributes.push(NativeAnyItem::new(CString::new("preserveDrawingBuffer").unwrap().into_raw(), NativeAnyValue::Bool(0)));
    attributes.push(NativeAnyItem::new(CString::new("stencil").unwrap().into_raw(), NativeAnyValue::Bool(0)));
    attributes.push(NativeAnyItem::new(CString::new("desynchronized").unwrap().into_raw(), NativeAnyValue::Bool(0)));
    let attr = attributes.into_boxed_slice();
    Box::into_raw(Box::new(NativeAnyItemArray {
        array: attr.as_ptr(),
        length: attr.len(),
    }))
}

#[no_mangle]
pub extern "C" fn gl_get_error() -> GLuint {
    unsafe { gl_bindings::glGetError() }
}


fn get_real_ext_name(name: String) -> String {
    if name.starts_with("WEBGL_") {
        return name;
    }
    return format!("GL_{:?}", name);
}

fn to_upper_case(name: String) -> String {
    return name.to_uppercase();
}

/*
#[no_mangle]
pub extern "C" fn get_extension(name: String) -> Any ? {

let realName = getRealExtName(name: name)
if let extPtr = glGetString(GLenum(GL_EXTENSIONS)) {
let extensions = String(cString: extPtr)
if(extensions.isEmpty){
return NSNull()
}
if (name.elementsEqual("WEBGL_compressed_texture_etc1") & & extensions.contains("GL_IMG_texture_compression_pvrtc")){
return Canvas_WEBGL_compressed_texture_pvrtc()
}else if (name.elementsEqual("WEBGL_compressed_texture_etc1")){
return Canvas_WEBGL_compressed_texture_etc1()
}
if let renderer = canvas.renderer as ? GLRenderer {
if (renderer.context.api ==.openGLES3){
switch name {
gl_bindings::"EXT_blend_minmax":
return Canvas_EXT_blend_minmax()
gl_bindings::"WEBGL_compressed_texture_etc":
return Canvas_WEBGL_compressed_texture_etc()
gl_bindings::"WEBGL_depth_texture":
return Canvas_WEBGL_depth_texture()
gl_bindings::"WEBGL_color_buffer_GLfloat":
return Canvas_WEBGL_color_buffer_GLfloat()
gl_bindings::"WEBGL_lose_context":
return Canvas_WEBGL_lose_context(canvas: self.canvas)
gl_bindings::"OES_texture_half_GLfloat":
return Canvas_OES_texture_half_GLfloat()
gl_bindings::"OES_texture_half_GLfloat_linear":
return Canvas_OES_texture_half_GLfloat_linear()
gl_bindings::"OES_texture_GLfloat":
//EXT_color_buffer_half_GLfloat
return Canvas_OES_texture_GLfloat()
gl_bindings::"OES_element_index_uint":
return Canvas_OES_element_index_uint()
gl_bindings::"OES_fbo_render_mipmap":
return Canvas_OES_fbo_render_mipmap()
gl_bindings::"OES_standard_derivatives":
return Canvas_OES_standard_derivatives()
gl_bindings::"OES_texture_GLfloat_linear":
return Canvas_OES_texture_GLfloat_linear()
gl_bindings::"OES_depth_texture":
return Canvas_WEBGL_depth_texture()
gl_bindings::"WEBGL_draw_buffers":
return Canvas_WEBGL_draw_buffers()
default: break
}
}
}
if (name.elementsEqual("ANGLE_instanced_arrays")){
return Canvas_ANGLE_instanced_arrays(context: self )
}
if (extensions.contains(realName)){
switch (realName){
gl_bindings::getRealExtName(name: "EXT_blend_minmax"):
return Canvas_EXT_blend_minmax()
gl_bindings::getRealExtName(name: "EXT_color_buffer_GLfloat"):
return Canvas_EXT_color_buffer_GLfloat()
gl_bindings::getRealExtName(name: "EXT_color_buffer_half_GLfloat"):
return Canvas_EXT_color_buffer_half_GLfloat()
gl_bindings::getRealExtName(name: "EXT_sRGB"):
return Canvas_EXT_sRGB()
gl_bindings::getRealExtName(name: "EXT_shader_texture_lod"):
return Canvas_EXT_shader_texture_lod()
gl_bindings::getRealExtName(name: "EXT_texture_filter_anisotropic"):
return Canvas_EXT_texture_filter_anisotropic()
gl_bindings::getRealExtName(name: "OES_element_index_uint"):
return Canvas_OES_element_index_uint()
gl_bindings::getRealExtName(name: "EXT_texture_filter_anisotropic"):
return Canvas_EXT_texture_filter_anisotropic()
gl_bindings::getRealExtName(name: "OES_fbo_render_mipmap"):
return Canvas_OES_fbo_render_mipmap()
gl_bindings::getRealExtName(name: "OES_standard_derivatives"):
return Canvas_OES_standard_derivatives()
gl_bindings::getRealExtName(name: "OES_texture_GLfloat_linear"):
return Canvas_OES_texture_GLfloat_linear()
gl_bindings::getRealExtName(name: "OES_texture_half_GLfloat"):
return Canvas_OES_texture_half_GLfloat()
gl_bindings::getRealExtName(name: "OES_texture_half_GLfloat_linear"):
return Canvas_OES_texture_half_GLfloat_linear()
gl_bindings::getRealExtName(name: "OES_depth_texture"):
return Canvas_WEBGL_depth_texture()
// N/A
//EXT_GLfloat_blend
//EXT_frag_depth
//EXT_texture_compression_bptc
//EXT_texture_compression_rgtc
//OVR_multiview2
//WEBGL_compressed_texture_astc
//WEBGL_compressed_texture_atc
//WEBGL_compressed_texture_s3tc
//WEBGL_compressed_texture_s3tc_srgb
//WEBGL_debug_renderer_info
//EBGL_debug_shaders
default:
return NSNull()
}
}
return NSNull()
}
return NSNull()
}
*/

#[repr(C)]
pub struct NativeFramebufferAttachmentParameter {
    pub(crate) is_texture: GLboolean,
    pub(crate) is_renderbuffer: GLboolean,
    pub(crate) value: GLint,
}

impl Default for NativeFramebufferAttachmentParameter {
    fn default() -> Self {
        Self {
            is_texture: 0,
            is_renderbuffer: 0,
            value: 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn gl_get_framebuffer_attachment_parameter(target: GLuint, attachment: GLuint, pname: GLuint) -> *const NativeFramebufferAttachmentParameter {
    let mut params: GLint = 0;
    let mut result = NativeFramebufferAttachmentParameter::default();
    if attachment == gl_bindings::GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME {
        unsafe { gl_bindings::glGetFramebufferAttachmentParameteriv(target, attachment, pname, &mut params) };
        let mut name: GLint = 0;
        unsafe { gl_bindings::glGetFramebufferAttachmentParameteriv(target, attachment, gl_bindings::GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE, &mut name) };
        match name as u32 {
            gl_bindings::GL_RENDERBUFFER => {
                result.is_renderbuffer = 1;
                result.value = params
            }
            gl_bindings::GL_TEXTURE => {
                result.is_texture = 1;
                result.value = params
            }
            _ => { result.value = params }
        }
    } else {
        unsafe { gl_bindings::glGetFramebufferAttachmentParameteriv(target, attachment, pname, &mut params) };
        result.value = params;
    }
    Box::into_raw(Box::new(result))
}


/* Pixel storage modes */

pub const WEBGL_UNPACK_COLORSPACE_CONVERSION_WEBGL: u32 = 0x9243;

pub const WEBGL_UNPACK_FLIP_Y_WEBGL: u32 = 0x9240;

pub const WEBGL_UNPACK_PREMULTIPLY_ALPHA_WEBGL: u32 = 0x9241;

/* Pixel storage modes */

pub const WEBGL_BROWSER_DEFAULT_WEBGL: u32 = 0x9244;

#[no_mangle]
pub extern "C" fn gl_get_parameter(pname: GLuint) -> NativeAnyValue {
    match pname {
        gl_bindings::GL_ACTIVE_TEXTURE | gl_bindings::GL_ALPHA_BITS | gl_bindings::GL_ARRAY_BUFFER_BINDING | gl_bindings::GL_BLEND_DST_ALPHA | gl_bindings::GL_BLEND_DST_RGB | gl_bindings::GL_BLEND_EQUATION | gl_bindings::GL_BLEND_EQUATION_ALPHA | gl_bindings::GL_BLEND_EQUATION_RGB | gl_bindings::GL_BLEND_SRC_ALPHA | gl_bindings::GL_BLEND_SRC_RGB | gl_bindings::GL_BLUE_BITS | gl_bindings::GL_CULL_FACE_MODE | gl_bindings::GL_CURRENT_PROGRAM | gl_bindings::GL_DEPTH_BITS | gl_bindings::GL_DEPTH_FUNC | gl_bindings::GL_ELEMENT_ARRAY_BUFFER_BINDING | gl_bindings::GL_FRAMEBUFFER_BINDING | gl_bindings::GL_FRONT_FACE | gl_bindings::GL_GENERATE_MIPMAP_HINT | gl_bindings::GL_GREEN_BITS | gl_bindings::GL_IMPLEMENTATION_COLOR_READ_FORMAT | gl_bindings::GL_IMPLEMENTATION_COLOR_READ_TYPE | gl_bindings::GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS | gl_bindings::GL_MAX_CUBE_MAP_TEXTURE_SIZE | gl_bindings::GL_MAX_FRAGMENT_UNIFORM_VECTORS | gl_bindings::GL_MAX_RENDERBUFFER_SIZE | gl_bindings::GL_MAX_TEXTURE_IMAGE_UNITS | gl_bindings::GL_MAX_TEXTURE_SIZE | gl_bindings::GL_MAX_VARYING_VECTORS | gl_bindings::GL_MAX_VERTEX_ATTRIBS | gl_bindings::GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS | gl_bindings::GL_MAX_VERTEX_UNIFORM_VECTORS | gl_bindings::GL_PACK_ALIGNMENT | gl_bindings::GL_RED_BITS | gl_bindings::GL_RENDERBUFFER_BINDING | gl_bindings::GL_SAMPLE_BUFFERS | gl_bindings::GL_SAMPLES | gl_bindings::GL_STENCIL_BACK_FAIL | gl_bindings::GL_STENCIL_BACK_FUNC | gl_bindings::GL_STENCIL_BACK_PASS_DEPTH_FAIL | gl_bindings::GL_STENCIL_BACK_PASS_DEPTH_PASS | gl_bindings::GL_STENCIL_BACK_REF | gl_bindings::GL_STENCIL_BACK_VALUE_MASK | gl_bindings::GL_STENCIL_BACK_WRITEMASK | gl_bindings::GL_STENCIL_BITS | gl_bindings::GL_STENCIL_CLEAR_VALUE | gl_bindings::GL_STENCIL_FAIL | gl_bindings::GL_STENCIL_FUNC | gl_bindings::GL_STENCIL_PASS_DEPTH_FAIL | gl_bindings::GL_STENCIL_PASS_DEPTH_PASS | gl_bindings::GL_STENCIL_REF | gl_bindings::GL_STENCIL_VALUE_MASK | gl_bindings::GL_STENCIL_WRITEMASK | gl_bindings::GL_SUBPIXEL_BITS | gl_bindings::GL_TEXTURE_BINDING_2D | gl_bindings::GL_TEXTURE_BINDING_CUBE_MAP | gl_bindings::GL_UNPACK_ALIGNMENT => {
            let mut param: GLint = 0;
            unsafe { gl_bindings::glGetIntegerv(pname, &mut param) };
            if (pname == gl_bindings::GL_TEXTURE_BINDING_2D || pname == gl_bindings::GL_TEXTURE_BINDING_CUBE_MAP || pname == gl_bindings::GL_RENDERBUFFER_BINDING || pname == gl_bindings::GL_FRAMEBUFFER_BINDING) && param == 0 {
                return NativeAnyValue::Null;
            }
            return NativeAnyValue::I32(param);
        }
        gl_bindings::GL_ALIASED_LINE_WIDTH_RANGE | gl_bindings::GL_ALIASED_POINT_SIZE_RANGE | gl_bindings::GL_DEPTH_RANGE => {
            let mut param: Vec<GLfloat> = Vec::with_capacity(2);
            unsafe { gl_bindings::glGetFloatv(pname, param.as_mut_ptr()) }
            let mut param = param.into_boxed_slice();
            let value = NativeAnyValue::F32Array(param.as_mut_ptr(), param.len());
            Box::into_raw(param);
            return value;
        }
        gl_bindings::GL_BLEND_COLOR | gl_bindings::GL_COLOR_CLEAR_VALUE => {
            let mut param: Vec<GLfloat> = Vec::with_capacity(4);
            unsafe { gl_bindings::glGetFloatv(pname, param.as_mut_ptr()) };
            let mut param = param.into_boxed_slice();
            let value = NativeAnyValue::F32Array(param.as_mut_ptr(), param.len());
            Box::into_raw(param);
            return value;
        }
        gl_bindings::GL_BLEND | gl_bindings::GL_CULL_FACE | gl_bindings::GL_DEPTH_TEST | gl_bindings::GL_DEPTH_WRITEMASK | gl_bindings::GL_DITHER | gl_bindings::GL_POLYGON_OFFSET_FILL | gl_bindings::GL_SAMPLE_COVERAGE_INVERT | GL_SCISSOR_TEST | gl_bindings::GL_STENCIL_TEST => {
            let mut param: GLboolean = 0;
            unsafe { gl_bindings::glGetBooleanv(pname, &mut param) };
            return NativeAnyValue::Bool(param);
        }
        WEBGL_UNPACK_PREMULTIPLY_ALPHA_WEBGL => {
            //premultiplyAlphaWebGL
            return NativeAnyValue::Bool(0);
        }
        WEBGL_UNPACK_FLIP_Y_WEBGL => {
            // flipYWebGL
            return NativeAnyValue::Bool(0);
        }
        WEBGL_UNPACK_COLORSPACE_CONVERSION_WEBGL => {
            // var cs = colorSpaceConversionWebGL;
            // if (cs == -1) {
            //     cs = BROWSER_DEFAULT_WEBGL
            // }
            return NativeAnyValue::U32(WEBGL_BROWSER_DEFAULT_WEBGL);
        }
        gl_bindings::GL_COLOR_WRITEMASK => {
            let mut param: Vec<GLboolean> = Vec::with_capacity(4);
            unsafe { gl_bindings::glGetBooleanv(pname, param.as_mut_ptr()) };
            let mut param = param.into_boxed_slice();
            let value = NativeAnyValue::BoolArray(param.as_ptr() as *const u8, param.len());
            Box::into_raw(param);
            return value;
        }
        gl_bindings::GL_COMPRESSED_TEXTURE_FORMATS => {
            let mut count: GLint = 0;
            unsafe { gl_bindings::glGetIntegerv(gl_bindings::GL_NUM_COMPRESSED_TEXTURE_FORMATS, &mut count) }
            let mut params: Vec<GLint> = Vec::with_capacity(count as usize);
            unsafe { gl_bindings::glGetIntegerv(gl_bindings::GL_COMPRESSED_TEXTURE_FORMATS, params.as_mut_ptr()) }
            let mut params = params.into_boxed_slice();
            let value = NativeAnyValue::I32Array(params.as_ptr(), params.len());
            Box::into_raw(params);
            return value;
        }
        gl_bindings::GL_DEPTH_CLEAR_VALUE | gl_bindings::GL_LINE_WIDTH | gl_bindings::GL_POLYGON_OFFSET_FACTOR | gl_bindings::GL_POLYGON_OFFSET_UNITS | gl_bindings::GL_SAMPLE_COVERAGE_VALUE => {
            let mut param: GLfloat = 0.0;
            unsafe { gl_bindings::glGetFloatv(pname, &mut param) }
            return NativeAnyValue::F32(param);
        }
        gl_bindings::GL_MAX_VIEWPORT_DIMS => {
            let mut params: Vec<GLint> = Vec::with_capacity(2);
            unsafe { gl_bindings::glGetIntegerv(pname, params.as_mut_ptr()) }
            let mut params = params.into_boxed_slice();
            let value = NativeAnyValue::I32Array(params.as_ptr(), params.len());
            Box::into_raw(params);
            return value;
        }
        gl_bindings::GL_SCISSOR_BOX | gl_bindings::GL_VIEWPORT => {
            let mut params = vec![0i32; 4];
            unsafe { gl_bindings::glGetIntegerv(pname, params.as_mut_ptr()) }
            let mut params = params.into_boxed_slice();
            let value = NativeAnyValue::I32Array(params.as_ptr(), params.len());
            Box::into_raw(params);
            return value;
        }
        gl_bindings::GL_RENDERER | gl_bindings::GL_SHADING_LANGUAGE_VERSION | gl_bindings::GL_VENDOR | gl_bindings::GL_VERSION => {
            let params = unsafe { gl_bindings::glGetString(pname) };
            if params.is_null() {
                return NativeAnyValue::Null;
            }
            return NativeAnyValue::String(params as *const i8);
        }
        _ => NativeAnyValue::Null
    }
}

#[no_mangle]
pub extern "C" fn gl_get_program_info_log(program: GLuint) -> *const c_char {
    let mut length: GLint = 0;
    unsafe { gl_bindings::glGetProgramiv(program, gl_bindings::GL_INFO_LOG_LENGTH, &mut length) }
    let mut info = String::with_capacity(length as usize);
    let mut len = 0;
    unsafe { gl_bindings::glGetProgramInfoLog(program, length, &mut len, info.as_mut_ptr() as *mut GLchar) }
    let string = CString::new(info).unwrap_or(CString::default());
    return string.into_raw();
}


#[no_mangle]
pub extern "C" fn gl_get_program_parameter(program: GLuint, pname: GLuint) -> NativeAnyValue {
    let mut param: GLint = 0;
    unsafe { gl_bindings::glGetProgramiv(program, pname, &mut param) }
    match pname {
        gl_bindings::GL_DELETE_STATUS | gl_bindings::GL_LINK_STATUS | gl_bindings::GL_VALIDATE_STATUS => {
            if param as u32 == gl_bindings::GL_TRUE {
                return NativeAnyValue::Bool(1);
            }
            return NativeAnyValue::Bool(0);
        }
        gl_bindings::GL_ATTACHED_SHADERS | gl_bindings::GL_ACTIVE_ATTRIBUTES | gl_bindings::GL_ACTIVE_UNIFORMS => {
            return NativeAnyValue::I32(param);
        }
        _ => NativeAnyValue::Null
    }
}


#[no_mangle]
pub extern "C" fn gl_get_renderbuffer_parameter(target: GLenum, pname: GLenum) -> GLint {
    let mut params: GLint = 0;
    unsafe { gl_bindings::glGetRenderbufferParameteriv(target, pname, &mut params) }
    return params;
}

#[no_mangle]
pub extern "C" fn gl_get_shader_info_log(shader: GLuint) -> *const i8 {
    let mut length: GLint = 0;
    unsafe { gl_bindings::glGetShaderiv(shader, gl_bindings::GL_INFO_LOG_LENGTH, &mut length) }
    let mut ptr = String::new();
    let mut len = 0;
    unsafe {
        gl_bindings::glGetShaderInfoLog(shader, length, &mut len, ptr.as_mut_ptr() as *mut i8)
    }

    return CString::new(ptr).unwrap_or(CString::default()).into_raw();
}

#[no_mangle]
pub extern "C" fn gl_get_shader_parameter(shader: GLuint, pname: GLuint) -> NativeAnyValue {
    let mut params: GLint = 0;
    unsafe {
        gl_bindings::glGetShaderiv(shader, pname, &mut params)
    }
    match pname {
        gl_bindings::GL_DELETE_STATUS | gl_bindings::GL_COMPILE_STATUS => {
            return if params as u32 == gl_bindings::GL_TRUE {
                NativeAnyValue::Bool(1)
            } else {
                NativeAnyValue::Bool(0)
            };
        }
        _ => {
            return NativeAnyValue::U32(params as u32);
        }
    }
}

#[no_mangle]
pub extern "C" fn gl_get_shader_precision_format(shader_type: GLuint, precision_type: GLuint) -> *const NativeWebGLShaderPrecisionFormat {
    let mut range = vec![0i32; 2];
    let mut precision: GLint = 0;
    unsafe {
        gl_bindings::glGetShaderPrecisionFormat(shader_type, precision_type, range.as_mut_ptr(), &mut precision)
    }
    return unsafe {
        Box::into_raw(Box::new(NativeWebGLShaderPrecisionFormat {
            precision,
            range_min: *range.get_unchecked(0),
            range_max: *range.get_unchecked(1),
        }))
    };
}

#[no_mangle]
pub extern "C" fn gl_get_shader_source(shader: GLuint) -> *const c_char {
    let mut length: GLint = 0;
    unsafe { gl_bindings::glGetShaderiv(shader, gl_bindings::GL_SHADER_SOURCE_LENGTH, &mut length) }
    let mut source = String::with_capacity(length as usize);
    let mut len = 0;
    unsafe { gl_bindings::glGetShaderSource(shader, length, &mut len, source.as_mut_ptr() as *mut i8) }
    CString::new(source).unwrap_or(CString::default()).into_raw()
}

/*
#[no_mangle]
pub extern "C" fn get_supported_extensions() -> [String] {
    let extensions = String(cString: glGetString(GLenum(GL_EXTENSIONS)))
    var
    list = extensions.components(separatedBy: .whitespaces)
    if let last = list.last {
        if (last.isEmpty) {
            let _ = list.popLast()
        }
    }

    list.append("EXT_blend_minmax")
    list.append("EXT_color_buffer_GLfloat")
    list.append("EXT_color_buffer_half_GLfloat")
    list.append("EXT_sRGB")
    list.append("EXT_shader_texture_lod")
    list.append("EXT_texture_filter_anisotropic")
    list.append("OES_element_index_uint")
    list.append("OES_fbo_render_mipmap")
    list.append("OES_standard_derivatives")
    list.append("OES_texture_GLfloat")
    list.append("OES_texture_GLfloat_linear")
    list.append("OES_texture_half_GLfloat")
    list.append("OES_texture_half_GLfloat_linear")
    list.append("WEBGL_color_buffer_GLfloat")
    list.append("WEBGL_compressed_texture_etc")
    list.append("WEBGL_compressed_texture_etc1")
    list.append("WEBGL_compressed_texture_pvrtc")
    list.append("WEBGL_depth_texture")
    list.append("WEBGL_lose_context")

    return list;
}
*/

#[no_mangle]
pub extern "C" fn gl_get_tex_parameter(target: GLuint, pname: GLuint) -> GLint {
    let mut params: GLint = 0;
    unsafe { gl_bindings::glGetTexParameteriv(target, pname, &mut params) }
    return params;
}

fn get_glfloat_slice(count: usize) -> Vec<GLfloat> {
    return vec![0f32; count];
}

fn get_int_slice(count: usize) -> Vec<GLint> {
    return vec![0i32; count];
}

#[no_mangle]
pub extern "C" fn gl_get_uniform(program: GLuint, location: GLint) -> NativeAnyValue {
    let mut uniform_type: GLuint = 0;
    let mut size = 0;
    let mut length = 0;
    unsafe { gl_bindings::glGetActiveUniform(program, location as u32, 0, &mut length, &mut size, &mut uniform_type, null_mut()) }
    match uniform_type {
        gl_bindings::GL_FLOAT => {
            let mut single = get_glfloat_slice(1);
            unsafe {
                gl_bindings::glGetUniformfv(program, location, single.as_mut_ptr())
            }
            return NativeAnyValue::F32(single[0]);
        }

        gl_bindings::GL_FLOAT_VEC2 => {
            let mut vec2 = get_glfloat_slice(2);
            unsafe {
                gl_bindings::glGetUniformfv(program, location, vec2.as_mut_ptr())
            }
            let value = vec2.into_boxed_slice();
            let array = NativeAnyValue::F32Array(value.as_ptr(), value.len());
            Box::into_raw(value);
            return array;
        }
        gl_bindings::GL_FLOAT_VEC3 => {
            let mut vec3 = get_glfloat_slice(3);
            unsafe {
                gl_bindings::glGetUniformfv(program, location, vec3.as_mut_ptr())
            }
            let value = vec3.into_boxed_slice();
            let array = NativeAnyValue::F32Array(value.as_ptr(), value.len());
            Box::into_raw(value);
            return array;
        }
        gl_bindings::GL_FLOAT_VEC4 => {
            let mut vec4 = get_glfloat_slice(4);
            unsafe {
                gl_bindings::glGetUniformfv(program, location, vec4.as_mut_ptr())
            }
            let value = vec4.into_boxed_slice();
            let array = NativeAnyValue::F32Array(value.as_ptr(), value.len());
            Box::into_raw(value);
            return array;
        }
        gl_bindings::GL_INT | gl_bindings::GL_SAMPLER_2D | gl_bindings::GL_SAMPLER_CUBE => {
            let mut single_int = get_int_slice(1);
            unsafe {
                gl_bindings::glGetUniformiv(program, location, single_int.as_mut_ptr());
            }

            return NativeAnyValue::I32(single_int[0]);
        }
        gl_bindings::GL_INT_VEC2 => {
            let mut int_vec2 = get_int_slice(2);
            unsafe {
                gl_bindings::glGetUniformiv(program, location, int_vec2.as_mut_ptr())
            }
            let value = int_vec2.into_boxed_slice();
            let array = NativeAnyValue::I32Array(value.as_ptr(), value.len());
            Box::into_raw(value);
            return array;
        }
        gl_bindings::GL_INT_VEC3 => {
            let mut int_vec3 = get_int_slice(3);
            unsafe {
                gl_bindings::glGetUniformiv(program, location, int_vec3.as_mut_ptr())
            }
            let value = int_vec3.into_boxed_slice();
            let array = NativeAnyValue::I32Array(value.as_ptr(), value.len());
            Box::into_raw(value);
            return array;
        }
        gl_bindings::GL_INT_VEC4 => {
            let mut int_vec4 = get_int_slice(4);
            unsafe {
                gl_bindings::glGetUniformiv(program, location, int_vec4.as_mut_ptr())
            }
            let value = int_vec4.into_boxed_slice();
            let array = NativeAnyValue::I32Array(value.as_ptr(), value.len());
            Box::into_raw(value);
            return array;
        }
        gl_bindings::GL_BOOL => {
            let mut single_bool = get_int_slice(1);
            unsafe {
                gl_bindings::glGetUniformiv(program, location, single_bool.as_mut_ptr())
            }
            return NativeAnyValue::Bool(single_bool[0] as u8);
        }
        gl_bindings::GL_BOOL_VEC2 => {
            let mut bool_vec2 = get_int_slice(2);
            unsafe {
                gl_bindings::glGetUniformiv(program, location, bool_vec2.as_mut_ptr())
            }
            let mut value = bool_vec2.into_boxed_slice();
            let array = NativeAnyValue::BoolArray(value.as_ptr() as *const u8, value.len());
            Box::into_raw(value);
            return array;
        }
        gl_bindings::GL_BOOL_VEC3 => {
            let mut bool_vec3 = get_int_slice(3);
            unsafe {
                gl_bindings::glGetUniformiv(program, location, bool_vec3.as_mut_ptr())
            }
            let mut value = bool_vec3.into_boxed_slice();
            let array = NativeAnyValue::BoolArray(value.as_ptr() as *const u8, value.len());
            Box::into_raw(value);
            return array;
        }
        gl_bindings::GL_BOOL_VEC4 => {
            let mut bool_vec4 = get_int_slice(4);
            unsafe {
                gl_bindings::glGetUniformiv(program, location, bool_vec4.as_mut_ptr())
            }
            let mut value = bool_vec4.into_boxed_slice();
            let array = NativeAnyValue::BoolArray(value.as_ptr() as *const u8, value.len());
            Box::into_raw(value);
            return array;
        }
        gl_bindings::GL_FLOAT_MAT2 => {
            let mut mat2 = get_glfloat_slice(2);
            unsafe {
                gl_bindings::glGetUniformfv(program, location, mat2.as_mut_ptr());
            }
            let value = mat2.into_boxed_slice();
            let array = NativeAnyValue::F32Array(value.as_ptr(), value.len());
            Box::into_raw(value);
            return array;
        }
        gl_bindings::GL_FLOAT_MAT3 => {
            let mut mat3 = get_glfloat_slice(3);
            unsafe {
                gl_bindings::glGetUniformfv(program, location, mat3.as_mut_ptr());
            }
            let value = mat3.into_boxed_slice();
            let array = NativeAnyValue::F32Array(value.as_ptr(), value.len());
            Box::into_raw(value);
            return array;
        }
        gl_bindings::GL_FLOAT_MAT4 => {
            let mut mat4 = get_glfloat_slice(4);
            unsafe {
                gl_bindings::glGetUniformfv(program, location, mat4.as_mut_ptr());
            }
            let value = mat4.into_boxed_slice();
            let array = NativeAnyValue::F32Array(value.as_ptr(), value.len());
            Box::into_raw(value);
            return array;
        }
        _ => NativeAnyValue::Null
    }
}

#[no_mangle]
pub extern "C" fn gl_get_uniform_location(program: GLuint, name: *const GLchar) -> GLint {
    return unsafe {
        gl_bindings::glGetUniformLocation(program, name)
    };
}


#[no_mangle]
pub extern "C" fn gl_get_vertex_attrib(index: GLuint, pname: GLuint) -> NativeAnyValue {
    if pname == gl_bindings::GL_CURRENT_VERTEX_ATTRIB {
        let mut params = vec![0f32; 4];
        unsafe {
            gl_bindings::glGetVertexAttribfv(index, pname, params.as_mut_ptr())
        }
        let value = params.into_boxed_slice();
        let array = NativeAnyValue::F32Array(value.as_ptr(), value.len());
        Box::into_raw(value);
        return array;
    }
    let mut params: GLint = 0;
    unsafe {
        gl_bindings::glGetVertexAttribiv(index, pname, &mut params)
    }
    match pname {
        gl_bindings::GL_VERTEX_ATTRIB_ARRAY_ENABLED | gl_bindings::GL_VERTEX_ATTRIB_ARRAY_NORMALIZED | gl_bindings::GL_VERTEX_ATTRIB_ARRAY_INTEGER => {
            return NativeAnyValue::Bool(params as u8);
        }
        gl_bindings::GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING => {
            return NativeAnyValue::U32(params as u32);
        }
        gl_bindings::GL_VERTEX_ATTRIB_ARRAY_SIZE | gl_bindings::GL_VERTEX_ATTRIB_ARRAY_STRIDE | gl_bindings::GL_VERTEX_ATTRIB_ARRAY_DIVISOR => {
            return NativeAnyValue::I32(params);
        }
        _ => {
            return NativeAnyValue::Null;
        }
    }
}


#[no_mangle]
pub extern "C" fn gl_get_vertex_attrib_offset(index: GLuint, pname: GLenum) -> size_t {
    let mut ptr = null_mut();
    let mut ptr_ptr: *mut *mut c_void = &mut ptr as _;
    unsafe {
        gl_bindings::glGetVertexAttribPointerv(index, pname, ptr_ptr)
    }

    return ptr as size_t;
}


#[no_mangle]
pub extern "C" fn gl_hint(target: GLenum, mode: GLenum) {
    unsafe {
        gl_bindings::glHint(target, mode)
    }
}


#[no_mangle]
pub extern "C" fn gl_is_buffer(buffer: GLuint) -> GLboolean {
    return unsafe {
        gl_bindings::glIsBuffer(buffer)
    };
}


#[no_mangle]
pub extern "C" fn gl_is_enabled(cap: GLenum) -> GLboolean {
    return unsafe {
        gl_bindings::glIsEnabled(cap)
    };
}


#[no_mangle]
pub extern "C" fn gl_is_framebuffer(framebuffer: GLuint) -> GLboolean {
    return unsafe { gl_bindings::glIsFramebuffer(framebuffer) };
}


#[no_mangle]
pub extern "C" fn gl_is_program(program: GLuint) -> GLboolean {
    return unsafe { gl_bindings::glIsProgram(program) };
}

#[no_mangle]
pub extern "C" fn gl_is_renderbuffer(renderbuffer: GLuint) -> GLboolean {
    return unsafe { gl_bindings::glIsRenderbuffer(renderbuffer) };
}

#[no_mangle]
pub extern "C" fn gl_is_shader(shader: GLuint) -> GLboolean {
    return unsafe { gl_bindings::glIsShader(shader) };
}

#[no_mangle]
pub extern "C" fn gl_is_texture(texture: GLuint) -> GLboolean {
    return unsafe { gl_bindings::glIsTexture(texture) };
}

#[no_mangle]
pub extern "C" fn gl_line_width(width: GLfloat) {
    unsafe { gl_bindings::glLineWidth(width) }
}

#[no_mangle]
pub extern "C" fn gl_link_program(program: GLuint) {
    unsafe { gl_bindings::glLinkProgram(program) }
}


/*
private func anyToInt(_ value: Any?, _ defaultValue: GLint) -> GLint {
if (value != nil) {
if let intVal = value as ? GLint {
return intVal
}
return defaultValue
}
return defaultValue
}

private func anyToBoolean(_ value: Any?, _ defaultValue: Bool) -> Bool {
if (value != nil) {
if let boolVal = value as ? Bool {
return boolVal
}
return defaultValue
}
return defaultValue
}

private func anyToColorSpace(_ value: Any?, _ defaultValue: GLint) -> GLint {
if (value != nil) {
if let intVal = value as ? GLint {
if (intVal == BROWSER_DEFAULT_WEBGL | | intVal == GL_NONE){
return intVal
}
return BROWSER_DEFAULT_WEBGL
}
return defaultValue
}
return defaultValue
}*/

// var flipYWebGL: Bool = false
// var premultiplyAlphaWebGL: Bool = false
// var colorSpaceConversionWebGL: GLint = - 1
#[no_mangle]
pub extern "C" fn gl_pixel_storei(pname: GLenum, param: GLint) {
    match pname {
        gl_bindings::GL_UNPACK_ALIGNMENT | gl_bindings::GL_PACK_ALIGNMENT | gl_bindings::GL_PACK_ROW_LENGTH | gl_bindings::GL_PACK_SKIP_PIXELS |
        gl_bindings::GL_PACK_SKIP_ROWS |
        gl_bindings::GL_UNPACK_ROW_LENGTH |
        gl_bindings::GL_UNPACK_IMAGE_HEIGHT |
        gl_bindings::GL_UNPACK_SKIP_PIXELS |
        gl_bindings::GL_UNPACK_SKIP_ROWS |
        gl_bindings::GL_UNPACK_SKIP_IMAGES => {
            unsafe {
                gl_bindings::glPixelStorei(pname, param)
            }
        }
        WEBGL_UNPACK_FLIP_Y_WEBGL => {}
        WEBGL_UNPACK_PREMULTIPLY_ALPHA_WEBGL => {}
        WEBGL_UNPACK_COLORSPACE_CONVERSION_WEBGL => {}
        _ => {}
    }
}


#[no_mangle]
pub extern "C" fn gl_polygon_offset(factor: GLfloat, units: GLfloat) {
    unsafe {
        gl_bindings::glPolygonOffset(factor, units)
    }
}

#[no_mangle]
pub extern "C" fn gl_read_pixels_u8(x: GLint, y: GLint, width: GLint, height: GLint, format: GLenum, pixel_type: GLenum, pixels: *mut GLubyte) {
    unsafe {
        gl_bindings::glReadPixels(x, y, width, height, format, pixel_type, pixels as *mut c_void)
    }
}

#[no_mangle]
pub extern "C" fn gl_read_pixels_u16(x: GLint, y: GLint, width: GLint, height: GLint, format: GLenum, pixel_type: GLenum, pixels: *mut GLushort) {
    unsafe {
        gl_bindings::glReadPixels(x, y, width, height, format, pixel_type, pixels as *mut c_void)
    }
}

#[no_mangle]
pub extern "C" fn gl_read_pixels_f32(x: GLint, y: GLint, width: GLint, height: GLint, format: GLenum, pixel_type: GLenum, pixels: *mut GLfloat) {
    unsafe {
        gl_bindings::glReadPixels(x, y, width, height, format, pixel_type, pixels as *mut c_void)
    }
}


#[no_mangle]
pub extern "C" fn gl_renderbuffer_storage(target: GLenum, internal_format: GLenum, width: GLint, height: GLint) {
    unsafe {
        gl_bindings::glRenderbufferStorage(target, internal_format, width, height)
    }
}


#[no_mangle]
pub extern "C" fn gl_sample_coverage(value: GLfloat, invert: GLboolean) {
    unsafe {
        gl_bindings::glSampleCoverage(value, invert)
    }
}


#[no_mangle]
pub extern "C" fn gl_scissor(x: GLint, y: GLint, width: GLint, height: GLint) {
    unsafe {
        gl_bindings::glScissor(x, y, width, height)
    }
}

#[no_mangle]
pub extern "C" fn gl_shader_source(shader: GLuint, source: *const GLchar) {
    unsafe {
        gl_bindings::glShaderSource(shader, 1, &source, null())
    }
}

#[no_mangle]
pub extern "C" fn gl_stencil_func(func: GLenum, reference: GLint, mask: GLuint) {
    unsafe {
        gl_bindings::glStencilFunc(func, reference, mask)
    }
}


#[no_mangle]
pub extern "C" fn gl_stencil_func_separate(face: GLenum, func: GLenum, reference: GLint, mask: GLuint) {
    unsafe {
        gl_bindings::glStencilFuncSeparate(face, func, reference, mask)
    }
}

#[no_mangle]
pub extern "C" fn gl_stencil_mask(mask: GLuint) {
    unsafe {
        gl_bindings::glStencilMask(mask)
    }
}

#[no_mangle]
pub extern "C" fn gl_stencil_mask_separate(face: GLenum, mask: GLuint) {
    unsafe {
        gl_bindings::glStencilMaskSeparate(face, mask)
    }
}

#[no_mangle]
pub extern "C" fn gl_stencil_op(fail: GLenum, zfail: GLenum, zpass: GLenum) {
    unsafe {
        gl_bindings::glStencilOp(fail, zfail, zpass)
    }
}

#[no_mangle]
pub extern "C" fn gl_stencil_op_separate(face: GLenum, fail: GLenum, zfail: GLenum, zpass: GLenum) {
    unsafe {
        gl_bindings::glStencilOpSeparate(face, fail, zfail, zpass)
    }
}


#[no_mangle]
pub extern "C" fn gl_tex_image_2d(target: GLenum, level: GLint, internalformat: GLint, width: GLint, height: GLint, border: GLint, format: GLenum, image_type: GLenum) {
    unsafe {
        gl_bindings::glTexImage2D(target, level, internalformat, width, height, border, format, image_type, null())
    }
}


#[no_mangle]
pub extern "C" fn gl_tex_image_2d_u8(target: GLenum, level: GLint, internalformat: GLint, width: GLint, height: GLint, border: GLint, format: GLenum, image_type: GLenum, pixels: *const GLubyte) {
    /*  if (flipYWebGL) {
          native_image_asset_flip_x_in_place_owned(GLuint(width), GLuint(height), &data, UInt(pixels.count))
      }*/
    unsafe {
        gl_bindings::glTexImage2D(target, level, internalformat, width, height, border, format, image_type, pixels as *const c_void)
    }
}

#[no_mangle]
pub extern "C" fn gl_tex_image_2d_u16(target: GLenum, level: GLint, internalformat: GLint, width: GLint, height: GLint, border: GLint, format: GLenum, image_type: GLenum, pixels: *const GLushort) {
    /*  if (flipYWebGL) {
          native_image_asset_flip_x_in_place_owned(GLuint(width), GLuint(height), &data, UInt(pixels.count))
      }*/
    unsafe {
        gl_bindings::glTexImage2D(target, level, internalformat, width, height, border, format, image_type, pixels as *const c_void)
    }
}

#[no_mangle]
pub extern "C" fn gl_tex_image_2d_u32(target: GLenum, level: GLint, internalformat: GLint, width: GLint, height: GLint, border: GLint, format: GLenum, image_type: GLenum, pixels: *const GLuint) {
    /*  if (flipYWebGL) {
          native_image_asset_flip_x_in_place_owned(GLuint(width), GLuint(height), &data, UInt(pixels.count))
      }*/
    unsafe {
        gl_bindings::glTexImage2D(target, level, internalformat, width, height, border, format, image_type, pixels as *const c_void)
    }
}

#[no_mangle]
pub extern "C" fn gl_tex_image_2d_f32(target: GLenum, level: GLint, internalformat: GLint, width: GLint, height: GLint, border: GLint, format: GLenum, image_type: GLenum, pixels: *const GLfloat) {
    /*  if (flipYWebGL) {
          native_image_asset_flip_x_in_place_owned(GLuint(width), GLuint(height), &data, UInt(pixels.count))
      }*/
    unsafe {
        gl_bindings::glTexImage2D(target, level, internalformat, width, height, border, format, image_type, pixels as *const c_void)
    }
}

#[no_mangle]
pub extern "C" fn gl_tex_parameterf(target: GLenum, pname: GLenum, param: GLfloat) {
    unsafe {
        gl_bindings::glTexParameterf(target, pname, param)
    }
}

#[no_mangle]
pub extern "C" fn gl_tex_parameteri(target: GLenum, pname: GLenum, param: GLint) {
    unsafe {
        gl_bindings::glTexParameteri(target, pname, param)
    }
}


#[no_mangle]
pub extern "C" fn gl_tex_sub_image_2d(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLint, height: GLint, format: GLenum, image_type: GLenum) {
    unsafe {
        gl_bindings::glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, image_type, null())
    }
}

#[no_mangle]
pub extern "C" fn gl_tex_sub_image_2d_u8(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLint, height: GLint, format: GLenum, image_type: GLenum, pixels: *const GLubyte) {
    // if (flipYWebGL) {
    //     native_image_asset_flip_y_in_place_owned(GLuint(width), GLuint(height), &data, UInt(data.count))
    // }
    unsafe {
        gl_bindings::glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, image_type, pixels as *const c_void)
    }
}

#[no_mangle]
pub extern "C" fn gl_tex_sub_image_2d_u16(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLint, height: GLint, format: GLenum, image_type: GLenum, pixels: *const GLushort) {
    // if (flipYWebGL) {
    //     native_image_asset_flip_y_in_place_owned(GLuint(width), GLuint(height), &data, UInt(data.count))
    // }
    unsafe {
        gl_bindings::glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, image_type, pixels as *const c_void)
    }
}

#[no_mangle]
pub extern "C" fn gl_tex_sub_image_2d_f32(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLint, height: GLint, format: GLenum, image_type: GLenum, pixels: *const GLfloat) {
    // if (flipYWebGL) {
    //     native_image_asset_flip_y_in_place_owned(GLuint(width), GLuint(height), &data, UInt(data.count))
    // }
    unsafe {
        gl_bindings::glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, image_type, pixels as *const c_void)
    }
}


#[no_mangle]
pub extern "C" fn gl_tex_sub_image_2d_none(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, format: GLenum, image_type: GLenum) {
    unsafe {
        gl_bindings::glTexSubImage2D(target, level, xoffset, yoffset, 0, 0, format, image_type, null())
    }
}


#[no_mangle]
pub extern "C" fn gl_uniform1f(location: GLint, v0: GLfloat) {
    unsafe {
        gl_bindings::glUniform1f(location, v0)
    }
}

#[no_mangle]
pub extern "C" fn gl_uniform1fv(location: GLint, v0: *const GLfloat) {
    unsafe {
        gl_bindings::glUniform1fv(location, 1, v0)
    }
}


#[no_mangle]
pub extern "C" fn gl_uniform1i(location: GLint, v0: GLint) {
    unsafe {
        gl_bindings::glUniform1i(location, v0)
    }
}

#[no_mangle]
pub extern "C" fn gl_uniform1iv(location: GLint, v0: *const GLint) {
    unsafe {
        gl_bindings::glUniform1iv(location, 1, v0)
    }
}


#[no_mangle]
pub extern "C" fn gl_uniform2f(location: GLint, v0: GLfloat, v1: GLfloat) {
    unsafe {
        gl_bindings::glUniform2f(location, v0, v1)
    }
}

#[no_mangle]
pub extern "C" fn gl_uniform2fv(location: GLint, v0: *const GLfloat) {
    unsafe {
        gl_bindings::glUniform1fv(location, 1, v0)
    }
}


#[no_mangle]
pub extern "C" fn gl_uniform2i(location: GLint, v0: GLint, v1: GLint) {
    unsafe {
        gl_bindings::glUniform2i(location, v0, v1)
    }
}

#[no_mangle]
pub extern "C" fn gl_uniform2iv(location: GLint, v0: *const GLint) {
    unsafe {
        gl_bindings::glUniform2iv(location, 1, v0)
    }
}


#[no_mangle]
pub extern "C" fn gl_uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
    unsafe {
        gl_bindings::glUniform3f(location, v0, v1, v2)
    }
}

#[no_mangle]
pub extern "C" fn gl_uniform3fv(location: GLint, v0: *const GLfloat) {
    unsafe {
        gl_bindings::glUniform3fv(location, 1, v0)
    }
}


#[no_mangle]
pub extern "C" fn gl_uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) {
    unsafe {
        gl_bindings::glUniform3i(location, v0, v1, v2)
    }
}

#[no_mangle]
pub extern "C" fn gl_uniform3iv(location: GLint, v0: *const GLint) {
    unsafe {
        gl_bindings::glUniform3iv(location, 1, v0)
    }
}

#[no_mangle]
pub extern "C" fn gl_uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
    unsafe {
        gl_bindings::glUniform4f(location, v0, v1, v2, v3)
    }
}

#[no_mangle]
pub extern "C" fn gl_uniform4fv(location: GLint, v0: *const GLfloat) {
    unsafe {
        gl_bindings::glUniform4fv(location, 1, v0)
    }
}

#[no_mangle]
pub extern "C" fn gl_uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
    unsafe {
        gl_bindings::glUniform4i(location, v0, v1, v2, v3)
    }
}

#[no_mangle]
pub extern "C" fn gl_uniform4iv(location: GLint, v0: *const GLint) {
    unsafe {
        gl_bindings::glUniform4iv(location, 1, v0)
    }
}

#[no_mangle]
pub extern "C" fn gl_uniform_matrix2fv(location: i32, transpose: u8, value: *const GLfloat) {
    unsafe {
        gl_bindings::glUniformMatrix2fv(location, 1, transpose, value)
    }
}

#[no_mangle]
pub extern "C" fn gl_uniform_matrix3fv(location: i32, transpose: u8, value: *const GLfloat) {
    unsafe {
        gl_bindings::glUniformMatrix3fv(location, 1, transpose, value)
    }
}

#[no_mangle]
pub extern "C" fn gl_uniform_matrix4fv(location: i32, transpose: u8, value: *const GLfloat) {
    unsafe {
        gl_bindings::glUniformMatrix4fv(location, 1, transpose, value)
    }
}

#[no_mangle]
pub extern "C" fn gl_use_program(program: GLuint) {
    unsafe {
        gl_bindings::glUseProgram(program)
    }
}

#[no_mangle]
pub extern "C" fn gl_validate_program(program: GLuint) {
    unsafe {
        gl_bindings::glValidateProgram(program)
    }
}

#[no_mangle]
pub extern "C" fn gl_vertex_attrib1f(index: GLuint, v0: GLfloat) {
    unsafe {
        gl_bindings::glVertexAttrib1f(index, v0)
    }
}

#[no_mangle]
pub extern "C" fn gl_vertex_attrib2f(index: GLuint, v0: GLfloat, v1: GLfloat) {
    unsafe {
        gl_bindings::glVertexAttrib2f(index, v0, v1)
    }
}

#[no_mangle]
pub extern "C" fn gl_vertex_attrib3f(index: GLuint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
    unsafe {
        gl_bindings::glVertexAttrib3f(index, v0, v1, v2)
    }
}

#[no_mangle]
pub extern "C" fn gl_vertex_attrib4f(index: GLuint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
    unsafe {
        gl_bindings::glVertexAttrib4f(index, v0, v1, v2, v3)
    }
}

#[no_mangle]
pub extern "C" fn gl_vertex_attrib1fv(index: GLuint, value: *const GLfloat) {
    unsafe {
        gl_bindings::glVertexAttrib1fv(index, value)
    }
}

#[no_mangle]
pub extern "C" fn gl_vertex_attrib2fv(index: GLuint, value: *const GLfloat) {
    unsafe {
        gl_bindings::glVertexAttrib2fv(index, value)
    }
}

#[no_mangle]
pub extern "C" fn gl_vertex_attrib3fv(index: GLuint, value: *const GLfloat) {
    unsafe {
        gl_bindings::glVertexAttrib3fv(index, value)
    }
}

#[no_mangle]
pub extern "C" fn gl_vertex_attrib4fv(index: GLuint, value: *const GLfloat) {
    unsafe {
        gl_bindings::glVertexAttrib4fv(index, value)
    }
}

#[no_mangle]
pub extern "C" fn gl_vertex_attrib_pointer(index: GLuint, size: i32, d_type: GLuint, normalized: u8, stride: i32, offset: *const c_void) {
    unsafe {
        gl_bindings::glVertexAttribPointer(index, size, d_type, normalized, stride, offset)
    }
}

#[no_mangle]
pub extern "C" fn gl_viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        gl_bindings::glViewport(x, y, width, height)
    }
}