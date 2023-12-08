#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uchar, c_void};

use canvas_core::image_asset::ImageAsset;
use gl_bindings::types::GLenum;

use crate::prelude::*;
use crate::utils;
use crate::utils::gl::GLImageAssetBytesType;

pub fn canvas_native_webgl_resized(state: &mut WebGLState) {
    state.resized();
}

pub fn canvas_native_webgl_active_texture(texture: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::ActiveTexture(texture);
    }
}

pub fn canvas_native_webgl_attach_shader(program: u32, shader: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::AttachShader(program, shader);
    }
}

pub fn canvas_native_webgl_bind_attrib_location(
    program: u32,
    index: u32,
    name: &str,
    state: &mut WebGLState,
) {
    state.make_current();
    let name = CString::new(name).unwrap();
    unsafe {
        gl_bindings::BindAttribLocation(program, index, name.as_ptr());
    }
}

pub fn canvas_native_webgl_bind_buffer(target: u32, buffer: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::BindBuffer(target, buffer);
    }
}

#[cfg(not(target_os = "ios"))]
pub fn canvas_native_webgl_bind_frame_buffer(
    target: u32,
    framebuffer: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::BindFramebuffer(target, framebuffer);
    }
}

#[cfg(target_os = "ios")]
pub fn canvas_native_webgl_bind_frame_buffer(
    target: u32,
    framebuffer: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    // will bind to default buffer
    if framebuffer == 0 {
        let inner = state.0.borrow();
        inner.gl_context.bind_drawable();
        return;
    }
    unsafe {
        gl_bindings::BindFramebuffer(target, framebuffer);
    }
}

pub fn canvas_native_webgl_bind_render_buffer(
    target: u32,
    renderbuffer: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::BindRenderbuffer(target, renderbuffer);
    }
}

pub fn canvas_native_webgl_bind_texture(target: u32, texture: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::BindTexture(target, texture);
    }
}

pub fn canvas_native_webgl_blend_color(
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::BlendColor(red, green, blue, alpha);
    }
}

pub fn canvas_native_webgl_blend_equation_separate(
    mode_rgb: u32,
    mode_alpha: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::BlendEquationSeparate(mode_rgb, mode_alpha);
    }
}

pub fn canvas_native_webgl_blend_equation(mode: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::BlendEquation(mode);
    }
}

pub fn canvas_native_webgl_blend_func_separate(
    src_rgb: u32,
    dst_rgb: u32,
    src_alpha: u32,
    dst_alpha: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::BlendFuncSeparate(src_rgb, dst_rgb, src_alpha, dst_alpha);
    }
}

pub fn canvas_native_webgl_blend_func(sfactor: u32, dfactor: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::BlendFunc(sfactor, dfactor);
    }
}

pub fn canvas_native_webgl_buffer_data(
    target: u32,
    src_data: &[u8],
    usage: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::BufferData(
            target,
            src_data.len().try_into().unwrap(),
            src_data.as_ptr() as *const c_void,
            usage,
        );
    }
}

pub fn canvas_native_webgl_buffer_data_u16(
    target: u32,
    src_data: &[u16],
    usage: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    let len = src_data.len() * std::mem::size_of::<u16>();
    unsafe {
        gl_bindings::BufferData(
            target,
            len.try_into().unwrap(),
            src_data.as_ptr() as *const c_void,
            usage,
        );
    }
}

pub fn canvas_native_webgl_buffer_data_f32(
    target: u32,
    src_data: &[f32],
    usage: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    let len = src_data.len() * std::mem::size_of::<f32>();
    unsafe {
        gl_bindings::BufferData(
            target,
            len.try_into().unwrap(),
            src_data.as_ptr() as *const c_void,
            usage,
        );
    }
}

pub fn canvas_native_webgl_buffer_data_none(
    target: u32,
    size: isize,
    usage: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::BufferData(target, size.try_into().unwrap(), std::ptr::null(), usage);
    }
}

pub fn canvas_native_webgl_buffer_sub_data(
    target: u32,
    offset: isize,
    src_data: &[u8],
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::BufferSubData(
            target,
            offset.try_into().unwrap(),
            src_data.len().try_into().unwrap(),
            src_data.as_ptr() as *const c_void,
        );
    }
}

pub fn canvas_native_webgl_buffer_sub_data_none(
    target: u32,
    offset: isize,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::BufferSubData(target, offset.try_into().unwrap(), 0, std::ptr::null());
    }
}

pub fn canvas_native_webgl_check_frame_buffer_status(target: u32, state: &mut WebGLState) -> u32 {
    state.make_current();
    unsafe { gl_bindings::CheckFramebufferStatus(target) }
}

#[allow(unused_assignments)]
fn reset(state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::Disable(3089 /* GL_SCISSOR_TEST */);
        gl_bindings::ClearColor(0., 0., 0., 0.);
        gl_bindings::ColorMask(1u8, 1u8, 1u8, 1u8);
        let mut clear_mask = 16384; /* GL_COLOR_BUFFER_BIT */
        if state.get_depth() {
            gl_bindings::ClearDepthf(1.);
            clear_mask = clear_mask | 256; /* GL_DEPTH_BUFFER_BIT */
            gl_bindings::DepthMask(1u8);
        }

        if state.get_stencil() {
            gl_bindings::ClearStencil(0);
            clear_mask = clear_mask | 1024; /* GL_STENCIL_BUFFER_BIT */
            gl_bindings::StencilMaskSeparate(1028 /* GL_FRONT */, 0xFFFFFFFF);
        }
    }
}

pub(crate) fn restore_state_after_clear(state: &mut WebGLState) {
    // Restore the state that the context set.
    state.make_current();
    if state.get_scissor_enabled() {
        unsafe { gl_bindings::Enable(gl_bindings::SCISSOR_TEST) }
    }

    unsafe {
        let clear_color = state.get_clear_color();
        let color_mask = state.get_color_mask();
        gl_bindings::ClearColor(
            clear_color[0],
            clear_color[1],
            clear_color[2],
            clear_color[3],
        );
        gl_bindings::ColorMask(
            color_mask[0] as u8,
            color_mask[1] as u8,
            color_mask[2] as u8,
            color_mask[3] as u8,
        );
        gl_bindings::ClearDepthf(state.get_clear_depth());
        gl_bindings::ClearStencil(state.get_clear_stencil());
        gl_bindings::StencilMaskSeparate(gl_bindings::FRONT, state.get_stencil_mask());
        gl_bindings::DepthMask(state.get_depth_mask() as u8);
    }
}

#[allow(unused_assignments)]
fn clear_if_composited(mask: u32, state: &mut WebGLState) -> HowToClear {
    state.make_current();
    let combined_clear = mask > 0 && !state.get_scissor_enabled();

    let m = mask & gl_bindings::COLOR_BUFFER_BIT;

    unsafe {
        gl_bindings::Disable(gl_bindings::SCISSOR_TEST);
    }

    if combined_clear && m == gl_bindings::COLOR_BUFFER_BIT {
        let get_clear = |state: &mut WebGLState| {
            let mut clear = [0f32; 4];
            let clear_mask = state.get_color_mask();
            let clear_color = state.get_clear_color();
            for (i, (cm, cc)) in clear_mask.iter().zip(clear_color).enumerate() {
                if *cm {
                    clear[i] = cc;
                }
            }
            clear
        };
        unsafe {
            let colors = get_clear(state);
            gl_bindings::ClearColor(colors[0], colors[1], colors[2], colors[3])
        }
    } else {
        unsafe { gl_bindings::ClearColor(0., 0., 0., 0.) }
    }

    unsafe { gl_bindings::ColorMask(1u8, 1u8, 1u8, 1u8) }
    let mut clear_mask = 16384;
    if state.get_depth() {
        if !combined_clear || !state.get_depth_mask() || mask & 256 /* GL_DEPTH_BUFFER_BIT */ == 0 {
            unsafe { gl_bindings::ClearDepthf(1.) }
            clear_mask = clear_mask | 256;
            unsafe { gl_bindings::DepthMask(1u8) }
        }
    }

    if state.get_stencil() {
        if combined_clear && mask & 1024 /* GL_STENCIL_BUFFER_BIT */ != 0 {
            unsafe {
                gl_bindings::ClearStencil(
                    state.get_clear_stencil() & state.get_stencil_mask() as i32,
                )
            }
        } else {
            unsafe {
                gl_bindings::ClearStencil(0);
            }
            clear_mask = clear_mask | 1024;

            unsafe { gl_bindings::StencilMaskSeparate(gl_bindings::FRONT, 0xFFFFFFFF) }
        }
    }

    unsafe {
        gl_bindings::Clear(mask);
    }
    restore_state_after_clear(state);
    if combined_clear {
        return HowToClear::CombinedClear;
    }
    HowToClear::JustClear
}

// #[cfg(target_os = "ios")]
// pub fn canvas_native_webgl_clear(mask: u32, state: &mut WebGLState) {
//     state.make_current();
//     if clear_if_composited(mask, state) != HowToClear::CombinedClear {
//         unsafe { gl_bindings::Clear(mask) }
//     }
//     // Flush context
// }

// #[cfg(not(target_os = "ios"))]
pub fn canvas_native_webgl_clear(mask: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::Clear(mask) }
    // Flush context
}

pub fn canvas_native_webgl_clear_color(
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
    state: &mut WebGLState,
) {
    state.make_current();
    state.set_clear_color([red, green, blue, alpha]);
    unsafe { gl_bindings::ClearColor(red, green, blue, alpha) }
}

pub fn canvas_native_webgl_clear_depth(depth: f32, state: &mut WebGLState) {
    state.make_current();
    state.set_clear_depth(depth);
    unsafe { gl_bindings::ClearDepthf(depth) }
}

pub fn canvas_native_webgl_clear_stencil(stencil: i32, state: &mut WebGLState) {
    state.make_current();
    state.set_clear_stencil(stencil);
    unsafe { gl_bindings::ClearStencil(stencil) }
}

pub fn canvas_native_webgl_color_mask(
    red: bool,
    green: bool,
    blue: bool,
    alpha: bool,
    state: &mut WebGLState,
) {
    state.make_current();
    state.set_color_mask([red, green, blue, alpha]);
    unsafe { gl_bindings::ColorMask(red as u8, green as u8, blue as u8, alpha as u8) }
}

pub fn canvas_native_webgl_commit(_: &mut WebGLState) {
    // noop
}

pub fn canvas_native_webgl_compile_shader(shader: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::CompileShader(shader) }
}

pub fn canvas_native_webgl_compressed_tex_image2d(
    target: u32,
    level: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    border: i32,
    pixels: &[u8],
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::CompressedTexImage2D(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            pixels.len().try_into().unwrap(),
            pixels.as_ptr() as *const c_void,
        )
    }
}

pub fn canvas_native_webgl_compressed_tex_image2d_none(
    target: u32,
    level: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    border: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::CompressedTexImage2D(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            0,
            0 as _,
        )
    }
}

pub fn canvas_native_webgl_compressed_tex_sub_image2d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    width: i32,
    height: i32,
    format: u32,
    pixels: &[u8],
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::CompressedTexSubImage2D(
            target,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format,
            pixels.len().try_into().unwrap(),
            pixels.as_ptr() as *const c_void,
        )
    }
}

pub fn canvas_native_webgl_copy_tex_image2d(
    target: u32,
    level: i32,
    internalformat: u32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    border: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::CopyTexImage2D(target, level, internalformat, x, y, width, height, border)
    }
}

pub fn canvas_native_webgl_copy_tex_sub_image2d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe { gl_bindings::CopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height) }
}

pub fn canvas_native_webgl_create_buffer(state: &mut WebGLState) -> u32 {
    state.make_current();
    let mut buffers = [0u32; 1];
    unsafe {
        gl_bindings::GenBuffers(1, buffers.as_mut_ptr());
    }

    buffers[0]
}

pub fn canvas_native_webgl_create_framebuffer(state: &mut WebGLState) -> u32 {
    state.make_current();
    let mut frame_buffers = [0u32; 1];
    unsafe { gl_bindings::GenFramebuffers(1, frame_buffers.as_mut_ptr()) }

    frame_buffers[0]
}

pub fn canvas_native_webgl_create_program(state: &mut WebGLState) -> u32 {
    state.make_current();
    let ret = unsafe { gl_bindings::CreateProgram() };

    ret
}

pub fn canvas_native_webgl_create_renderbuffer(state: &mut WebGLState) -> u32 {
    state.make_current();
    let mut render_buffers = [0u32; 1];
    unsafe { gl_bindings::GenRenderbuffers(1, render_buffers.as_mut_ptr()) }

    render_buffers[0]
}

pub fn canvas_native_webgl_create_shader(shader_type: u32, state: &mut WebGLState) -> u32 {
    state.make_current();
    let ret = unsafe { gl_bindings::CreateShader(shader_type) };

    ret
}

pub fn canvas_native_webgl_create_texture(state: &mut WebGLState) -> u32 {
    state.make_current();
    let mut textures = [0u32; 1];
    unsafe { gl_bindings::GenTextures(1, textures.as_mut_ptr()) }

    textures[0]
}

pub fn canvas_native_webgl_cull_face(mode: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::CullFace(mode) }
}

pub fn canvas_native_webgl_delete_buffer(buffer: u32, state: &mut WebGLState) {
    state.make_current();
    let buf = [buffer];
    unsafe { gl_bindings::DeleteBuffers(1, buf.as_ptr()) }
}

pub fn canvas_native_webgl_delete_framebuffer(frame_buffer: u32, state: &mut WebGLState) {
    state.make_current();
    let buf = [frame_buffer];
    unsafe { gl_bindings::DeleteFramebuffers(1, buf.as_ptr()) }
}

pub fn canvas_native_webgl_delete_program(program: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::DeleteProgram(program) }
}

pub fn canvas_native_webgl_delete_renderbuffer(render_buffer: u32, state: &mut WebGLState) {
    state.make_current();
    let buf = [render_buffer];
    unsafe { gl_bindings::DeleteRenderbuffers(1, buf.as_ptr()) }
}

pub fn canvas_native_webgl_delete_shader(shader: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::DeleteShader(shader) }
}

pub fn canvas_native_webgl_delete_texture(texture: u32, state: &mut WebGLState) {
    state.make_current();
    let buf = [texture];
    unsafe { gl_bindings::DeleteTextures(1, buf.as_ptr()) }
}

pub fn canvas_native_webgl_depth_func(func: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::DepthFunc(func) }
}

pub fn canvas_native_webgl_depth_mask(flag: bool, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::DepthMask(flag as u8) }
}

pub fn canvas_native_webgl_depth_range(z_near: f32, z_far: f32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::DepthRangef(z_near, z_far) }
}

pub fn canvas_native_webgl_detach_shader(program: u32, shader: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::DetachShader(program, shader) }
}

pub fn canvas_native_webgl_disable(cap: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::Disable(cap) }
}

pub fn canvas_native_webgl_disable_vertex_attrib_array(index: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::DisableVertexAttribArray(index) }
}

// #[cfg(target_os = "ios")]
// pub fn canvas_native_webgl_draw_arrays(mode: u32, first: i32, count: i32, state: &mut WebGLState) {
//     state.make_current();
//     clear_if_composited(0, state);
//     unsafe { gl_bindings::DrawArrays(mode, first, count) }
//     // Flush Context
// }

// #[cfg(not(target_os = "ios"))]
pub fn canvas_native_webgl_draw_arrays(mode: u32, first: i32, count: i32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::DrawArrays(mode, first, count) }

    // Flush Context
}

// #[cfg(target_os = "ios")]
// pub fn canvas_native_webgl_draw_elements(
//     mode: u32,
//     count: i32,
//     element_type: u32,
//     offset: isize,
//     state: &mut WebGLState,
// ) {
//     state.make_current();
//     clear_if_composited(0, state);
//     unsafe { gl_bindings::DrawElements(mode, count, element_type, offset as *const c_void) }
//     // Flush Context
// }

// #[cfg(not(target_os = "ios"))]
pub fn canvas_native_webgl_draw_elements(
    mode: u32,
    count: i32,
    element_type: u32,
    offset: isize,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe { gl_bindings::DrawElements(mode, count, element_type, offset as *const c_void) }
    // Flush Context
}

pub fn canvas_native_webgl_enable(cap: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::Enable(cap) }
}

pub fn canvas_native_webgl_enable_vertex_attrib_array(index: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::EnableVertexAttribArray(index) }
}

pub fn canvas_native_webgl_finish(state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::Finish() }
}

pub fn canvas_native_webgl_flush(state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::Flush() }
}

pub fn canvas_native_webgl_framebuffer_renderbuffer(
    target: u32,
    attachment: u32,
    renderbuffertarget: u32,
    renderbuffer: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::FramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer)
    }
}

pub fn canvas_native_webgl_framebuffer_texture2d(
    target: u32,
    attachment: u32,
    textarget: u32,
    texture: u32,
    level: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe { gl_bindings::FramebufferTexture2D(target, attachment, textarget, texture, level) }
}

pub fn canvas_native_webgl_front_face(mode: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::FrontFace(mode) }
}

pub fn canvas_native_webgl_generate_mipmap(target: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::GenerateMipmap(target) }
}

pub fn canvas_native_webgl_get_active_attrib(
    program: u32,
    index: u32,
    state: &mut WebGLState,
) -> WebGLActiveInfo {
    state.make_current();
    let mut length = 0i32;
    let mut size = 0i32;
    let mut info_type = 0u32;
    let mut name_length = 0i32;
    unsafe {
        gl_bindings::GetProgramiv(
            program,
            gl_bindings::ACTIVE_ATTRIBUTE_MAX_LENGTH,
            &mut length,
        )
    }
    let mut name_buffer = vec![0; length as usize];

    unsafe {
        gl_bindings::GetActiveAttrib(
            program,
            index,
            length,
            &mut name_length,
            &mut size,
            &mut info_type,
            name_buffer.as_mut_ptr() as *mut c_char,
        )
    }

    name_buffer.shrink_to(name_length as usize);

    let c_str = unsafe { CStr::from_ptr(name_buffer.as_ptr()) };

    WebGLActiveInfo::new(c_str.to_string_lossy().to_string(), size, info_type)
}

pub fn canvas_native_webgl_get_active_uniform(
    program: u32,
    index: u32,
    state: &mut WebGLState,
) -> WebGLActiveInfo {
    state.make_current();
    let mut length = 0i32;
    let mut size = 0i32;
    let mut info_type = 0u32;
    let mut name_length = 0i32;
    unsafe {
        gl_bindings::GetProgramiv(program, gl_bindings::ACTIVE_UNIFORM_MAX_LENGTH, &mut length)
    }
    let mut name_buffer = vec![0; length as usize];

    unsafe {
        gl_bindings::GetActiveUniform(
            program,
            index,
            length,
            &mut name_length,
            &mut size,
            &mut info_type,
            name_buffer.as_mut_ptr() as *mut c_char,
        )
    };

    name_buffer.shrink_to(name_length as usize);

    let c_str = unsafe { CStr::from_ptr(name_buffer.as_ptr()) };

    WebGLActiveInfo::new(c_str.to_string_lossy().to_string(), size, info_type)
}

pub fn canvas_native_webgl_get_attached_shaders(program: u32, state: &mut WebGLState) -> Vec<u32> {
    state.make_current();
    let mut count = 0i32;
    unsafe { gl_bindings::GetProgramiv(program, gl_bindings::ATTACHED_SHADERS, &mut count) }
    let mut shaders = vec![0u32; count as usize];
    let mut len = 0;
    unsafe { gl_bindings::GetAttachedShaders(program, count, &mut len, shaders.as_mut_ptr()) }

    shaders
}

pub fn canvas_native_webgl_get_attrib_location(
    program: u32,
    name: &str,
    state: &mut WebGLState,
) -> i32 {
    state.make_current();
    let name = CString::new(name).unwrap();
    let ret = unsafe { gl_bindings::GetAttribLocation(program, name.as_ptr()) };

    ret
}

pub fn canvas_native_webgl_get_buffer_parameter(
    target: u32,
    pname: u32,
    state: &mut WebGLState,
) -> i32 {
    state.make_current();
    let mut params = 0i32;
    unsafe { gl_bindings::GetBufferParameteriv(target, pname, &mut params) }

    params
}

pub fn canvas_native_webgl_get_context_attributes(state: &WebGLState) -> ContextAttributes {
    state.get_context_attributes()
}

pub fn canvas_native_webgl_get_error(state: &mut WebGLState) -> u32 {
    state.make_current();
    let ret = unsafe { gl_bindings::GetError() };

    ret
}

#[cfg(not(any(target_os = "ios", target_os = "macos")))]
pub fn canvas_native_webgl_get_extension(
    name: &str,
    state: &mut WebGLState,
) -> Option<Box<dyn WebGLExtension>> {
    state.make_current();
    let version = state.get_webgl_version();
    let extensions = unsafe { gl_bindings::GetString(gl_bindings::EXTENSIONS) };

    if extensions.is_null() {
        return None;
    }
    // API_LEVEL
    #[allow(non_snake_case)]
        let JELLY_BEAN_MR2 = 18;

    let ext = unsafe { CStr::from_ptr(std::mem::transmute(extensions)) };
    let extensions = ext.to_string_lossy();


    let extension = if name.eq("EXT_blend_minmax") {
        if extensions.contains("GL_EXT_blend_minmax") {
            return Some(Box::new(EXT_blend_minmax::new()));
        }
        None
    } else if name.eq("EXT_color_buffer_half_float")
        && extensions.contains("GL_EXT_color_buffer_half_float")
    {
        return Some(Box::new(EXT_color_buffer_half_float::new()));
    } else if name.eq("EXT_disjoint_timer_query")
        && extensions.contains("GL_EXT_disjoint_timer_query")
    {
        return if get_sdk_version() >= JELLY_BEAN_MR2 {
            Some(Box::new(EXT_disjoint_timer_query::new(state.clone())))
        } else {
            None
        };
    } else if name.eq("EXT_sRGB") && extensions.contains("GL_EXT_sRGB") {
        return Some(Box::new(EXT_sRGB::new()));
    } else if name.eq("EXT_shader_texture_lod") && extensions.contains("GL_EXT_shader_texture_lod")
    {
        return Some(Box::new(EXT_shader_texture_lod::new()));
    } else if name.eq("EXT_texture_filter_anisotropic") {
        return Some(Box::new(EXT_texture_filter_anisotropic::new()));
    } else if name.eq("OES_element_index_uint") && extensions.contains("GL_OES_element_index_uint")
    {
        return Some(Box::new(OES_element_index_uint::new()));
    } else if name.eq("OES_standard_derivatives")
        && extensions.contains("GL_OES_standard_derivatives")
    {
        return Some(Box::new(OES_standard_derivatives::new()));
    } else if name.eq("OES_texture_float")
        && (extensions.contains("GL_OES_texture_float") || version == WebGLVersion::V2)
    {
        return Some(Box::new(OES_texture_float::new()));
    } else if name.eq("OES_texture_float_linear")
        && (extensions.contains("GL_OES_texture_float_linear") || version == WebGLVersion::V2)
    {
        return Some(Box::new(OES_texture_float_linear::new()));
    } else if name.eq("OES_texture_half_float") && extensions.contains("GL_OES_texture_half_float")
    {
        return Some(Box::new(OES_texture_half_float::new()));
    } else if name.eq("OES_texture_half_float_linear")
        && extensions.contains("GL_OES_texture_half_float_linear")
    {
        return Some(Box::new(OES_texture_half_float_linear::new()));
    } else if name.eq("OES_vertex_array_object")
        && extensions.contains("GL_OES_vertex_array_object")
    {
        return if get_sdk_version() >= JELLY_BEAN_MR2 {
            Some(Box::new(OES_vertex_array_object::new(state.clone())))
        } else {
            None
        };
    } else if (name.eq("WEBGL_color_buffer_float") || name.eq("EXT_color_buffer_float"))
        && extensions.contains("GL_OES_packed_depth_stencil")
    {
        return Some(Box::new(WEBGL_color_buffer_float::new()));
    } else if name.eq("WEBGL_compressed_texture_atc")
        && extensions.contains("GL_AMD_compressed_ATC_texture")
    {
        return Some(Box::new(WEBGL_compressed_texture_atc::new()));
    } else if name.eq("WEBGL_compressed_texture_etc1")
        && extensions.contains("GL_OES_compressed_ETC1_RGB8_texture")
    {
        return Some(Box::new(WEBGL_compressed_texture_etc1::new()));
    } else if name.eq("WEBGL_compressed_texture_s3tc")
        && extensions.contains("GL_EXT_texture_compression_dxt1")
        && extensions.contains("GL_EXT_texture_compression_s3tc")
    {
        return Some(Box::new(WEBGL_compressed_texture_s3tc::new()));
    } else if name.eq("WEBGL_compressed_texture_etc") {
        return if version == WebGLVersion::V2 {
            return Some(Box::new(WEBGL_compressed_texture_etc::new()));
        } else {
            None
        };
    } else if name.eq("WEBGL_compressed_texture_pvrtc")
        && extensions.contains("GL_IMG_texture_compression_pvrtc")
    {
        return Some(Box::new(WEBGL_compressed_texture_pvrtc::new()));
    } else if name.eq("WEBGL_lose_context") {
        return Some(Box::new(WEBGL_lose_context::new(state.clone())));
    } else if name.eq("ANGLE_instanced_arrays") {
        if version == WebGLVersion::V2 {
            return if get_sdk_version() >= JELLY_BEAN_MR2 {
                Some(Box::new(ANGLE_instanced_arrays::new(state.clone())))
            } else {
                None
            };
        } else {
            None
        }
    } else if name.eq("WEBGL_depth_texture") && extensions.contains("GL_OES_depth_texture") {
        return Some(Box::new(WEBGL_depth_texture::new()));
    } else if name.eq("WEBGL_draw_buffers") {
        if get_sdk_version() >= JELLY_BEAN_MR2 {
            if version == WebGLVersion::V2 || extensions.contains("GL_EXT_draw_buffers") {
                return Some(Box::new(WEBGL_draw_buffers::new(state.clone())));
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    extension
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
pub fn canvas_native_webgl_get_extension(
    name: &str,
    state: &mut WebGLState,
) -> Option<Box<dyn WebGLExtension>> {
    state.make_current();
    let version = state.get_webgl_version();
    let extensions = unsafe { gl_bindings::GetString(gl_bindings::EXTENSIONS) };

    if extensions.is_null() {
        return None;
    }

    let ext = unsafe { CStr::from_ptr(std::mem::transmute(extensions)) };
    let extensions = ext.to_string_lossy();

    if name == "WEBGL_compressed_texture_etc1"
        && extensions.contains("GL_IMG_texture_compression_pvrtc")
    {
        return Some(Box::new(WEBGL_compressed_texture_pvrtc::new()));
    } else if name == "WEBGL_compressed_texture_etc1" {
        return Some(Box::new(WEBGL_compressed_texture_etc1::new()));
    }

    if version == WebGLVersion::V2 {
        match name {
            "EXT_blend_minmax" => {
                return Some(Box::new(EXT_blend_minmax::new()));
            }
            "WEBGL_compressed_texture_etc" => {
                return Some(Box::new(WEBGL_compressed_texture_etc::new()));
            }
            "WEBGL_depth_texture" => {
                return Some(Box::new(WEBGL_depth_texture::new()));
            }
            "WEBGL_color_buffer_float" => {
                return Some(Box::new(WEBGL_color_buffer_float::new()));
            }
            "OES_texture_half_float" => {
                return Some(Box::new(OES_texture_half_float::new()));
            }
            "OES_texture_half_float_linear" => {
                return Some(Box::new(OES_texture_half_float_linear::new()));
            }
            "OES_texture_float" => {
                return Some(Box::new(OES_texture_float::new()));
            }
            "OES_element_index_uint" => {
                return Some(Box::new(OES_element_index_uint::new()));
            }
            "OES_fbo_render_mipmap" => {
                return Some(Box::new(OES_fbo_render_mipmap::new()));
            }
            "OES_standard_derivatives" => {
                return Some(Box::new(OES_standard_derivatives::new()));
            }
            "OES_texture_float_linear" => {
                return Some(Box::new(OES_texture_float_linear::new()));
            }
            "OES_depth_texture" => {
                return Some(Box::new(WEBGL_depth_texture::new()));
            }
            "WEBGL_draw_buffers" => {
                return Some(Box::new(WEBGL_draw_buffers::new(state.clone())));
            }
            _ => {}
        }
    }

    if name == "ANGLE_instanced_arrays" {
        return Some(Box::new(ANGLE_instanced_arrays::new(state.clone())));
    }

    if extensions.contains(name) {
        return match name {
            "EXT_blend_minmax" => return Some(Box::new(EXT_blend_minmax::new())),
            "WEBGL_color_buffer_float" | "EXT_color_buffer_float" => {
                return Some(Box::new(WEBGL_color_buffer_float::new()))
            }
            "EXT_color_buffer_half_float" => {
                return Some(Box::new(EXT_color_buffer_half_float::new()))
            }
            "EXT_sRGB" => return Some(Box::new(EXT_sRGB::new())),
            "EXT_shader_texture_lod" => return Some(Box::new(EXT_shader_texture_lod::new())),
            "EXT_texture_filter_anisotropic" => {
                return Some(Box::new(EXT_texture_filter_anisotropic::new()))
            }
            "OES_element_index_uint" => return Some(Box::new(OES_element_index_uint::new())),
            "OES_fbo_render_mipmap" => return Some(Box::new(OES_fbo_render_mipmap::new())),
            "OES_standard_derivatives" => {
                return Some(Box::new(OES_standard_derivatives::new()))
            }
            "OES_texture_float_linear" => {
                return Some(Box::new(OES_texture_float_linear::new()))
            }
            "OES_texture_half_float" => return Some(Box::new(OES_texture_half_float::new())),
            "OES_texture_half_float_linear" => {
                return Some(Box::new(OES_texture_half_float_linear::new()))
            }
            "OES_depth_texture" => return Some(Box::new(WEBGL_depth_texture::new())),
            // N/A
            //EXT_float_blend
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
            _ => None,
        };
    }

    None
}


pub fn canvas_native_webgl_get_framebuffer_attachment_parameter(
    target: u32,
    attachment: u32,
    pname: u32,
    state: &mut WebGLState,
) -> WebGLFramebufferAttachmentParameter {
    state.make_current();
    let mut params = 0i32;
    let mut result = WebGLFramebufferAttachmentParameter::default();
    if attachment == gl_bindings::FRAMEBUFFER_ATTACHMENT_OBJECT_NAME {
        unsafe {
            gl_bindings::GetFramebufferAttachmentParameteriv(target, attachment, pname, &mut params)
        };
        let mut name = 0i32;
        unsafe {
            gl_bindings::GetFramebufferAttachmentParameteriv(
                target,
                attachment,
                gl_bindings::FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE,
                &mut name,
            )
        };
        match name as u32 {
            gl_bindings::RENDERBUFFER => {
                result.is_renderbuffer = true;
                result.value = params
            }
            gl_bindings::TEXTURE => {
                result.is_texture = true;
                result.value = params
            }
            _ => result.value = params,
        }
    } else {
        unsafe {
            gl_bindings::GetFramebufferAttachmentParameteriv(target, attachment, pname, &mut params)
        };
        result.value = params;
    }

    result
}

// BLEND_EQUATION & BLEND_EQUATION_RGB are the same
#[allow(unreachable_patterns)]
pub fn canvas_native_webgl_get_parameter(pname: u32, state: &mut WebGLState) -> WebGLResult {
    state.make_current();

    let ret = match pname {
        gl_bindings::ACTIVE_TEXTURE
        | gl_bindings::ALPHA_BITS
        | gl_bindings::ARRAY_BUFFER_BINDING
        | gl_bindings::BLEND_DST_ALPHA
        | gl_bindings::BLEND_DST_RGB
        | gl_bindings::BLEND_EQUATION
        | gl_bindings::BLEND_EQUATION_ALPHA
        | gl_bindings::BLEND_EQUATION_RGB
        | gl_bindings::BLEND_SRC_ALPHA
        | gl_bindings::BLEND_SRC_RGB
        | gl_bindings::BLUE_BITS
        | gl_bindings::CULL_FACE_MODE
        | gl_bindings::CURRENT_PROGRAM
        | gl_bindings::DEPTH_BITS
        | gl_bindings::DEPTH_FUNC
        | gl_bindings::ELEMENT_ARRAY_BUFFER_BINDING
        | gl_bindings::FRAMEBUFFER_BINDING
        | gl_bindings::FRONT_FACE
        | gl_bindings::GENERATE_MIPMAP_HINT
        | gl_bindings::GREEN_BITS
        | gl_bindings::IMPLEMENTATION_COLOR_READ_FORMAT
        | gl_bindings::IMPLEMENTATION_COLOR_READ_TYPE
        | gl_bindings::MAX_COMBINED_TEXTURE_IMAGE_UNITS
        | gl_bindings::MAX_CUBE_MAP_TEXTURE_SIZE
        | gl_bindings::MAX_FRAGMENT_UNIFORM_VECTORS
        | gl_bindings::MAX_RENDERBUFFER_SIZE
        | gl_bindings::MAX_TEXTURE_IMAGE_UNITS
        | gl_bindings::MAX_TEXTURE_SIZE
        | gl_bindings::MAX_VARYING_VECTORS
        | gl_bindings::MAX_VERTEX_ATTRIBS
        | gl_bindings::MAX_VERTEX_TEXTURE_IMAGE_UNITS
        | gl_bindings::MAX_VERTEX_UNIFORM_VECTORS
        | gl_bindings::PACK_ALIGNMENT
        | gl_bindings::RED_BITS
        | gl_bindings::RENDERBUFFER_BINDING
        | gl_bindings::SAMPLE_BUFFERS
        | gl_bindings::SAMPLES
        | gl_bindings::STENCIL_BACK_FAIL
        | gl_bindings::STENCIL_BACK_FUNC
        | gl_bindings::STENCIL_BACK_PASS_DEPTH_FAIL
        | gl_bindings::STENCIL_BACK_PASS_DEPTH_PASS
        | gl_bindings::STENCIL_BACK_REF
        | gl_bindings::STENCIL_BACK_VALUE_MASK
        | gl_bindings::STENCIL_BACK_WRITEMASK
        | gl_bindings::STENCIL_BITS
        | gl_bindings::STENCIL_CLEAR_VALUE
        | gl_bindings::STENCIL_FAIL
        | gl_bindings::STENCIL_FUNC
        | gl_bindings::STENCIL_PASS_DEPTH_FAIL
        | gl_bindings::STENCIL_PASS_DEPTH_PASS
        | gl_bindings::STENCIL_REF
        | gl_bindings::STENCIL_VALUE_MASK
        | gl_bindings::STENCIL_WRITEMASK
        | gl_bindings::SUBPIXEL_BITS
        | gl_bindings::TEXTURE_BINDING_2D
        | gl_bindings::TEXTURE_BINDING_CUBE_MAP
        | gl_bindings::UNPACK_ALIGNMENT => {
            let mut param = 0i32;
            unsafe { gl_bindings::GetIntegerv(pname, &mut param) };
            if (pname == gl_bindings::TEXTURE_BINDING_2D
                || pname == gl_bindings::TEXTURE_BINDING_CUBE_MAP
                || pname == gl_bindings::RENDERBUFFER_BINDING
                || pname == gl_bindings::FRAMEBUFFER_BINDING)
                && param == 0
            {
                return WebGLResult::None;
            }
            return WebGLResult::I32(param);
        }
        gl_bindings::ALIASED_LINE_WIDTH_RANGE
        | gl_bindings::ALIASED_POINT_SIZE_RANGE
        | gl_bindings::DEPTH_RANGE => {
            let mut param: Vec<f32> = vec![0., 0.];
            unsafe { gl_bindings::GetFloatv(pname, param.as_mut_ptr()) }
            return WebGLResult::F32Array(param);
        }
        gl_bindings::BLEND_COLOR | gl_bindings::COLOR_CLEAR_VALUE => {
            let mut param: Vec<f32> = vec![0., 0., 0., 0.];
            unsafe { gl_bindings::GetFloatv(pname, param.as_mut_ptr()) };
            return WebGLResult::F32Array(param);
        }
        gl_bindings::BLEND
        | gl_bindings::CULL_FACE
        | gl_bindings::DEPTH_TEST
        | gl_bindings::DEPTH_WRITEMASK
        | gl_bindings::DITHER
        | gl_bindings::POLYGON_OFFSET_FILL
        | gl_bindings::SAMPLE_COVERAGE_INVERT
        | gl_bindings::SCISSOR_TEST
        | gl_bindings::STENCIL_TEST => {
            let mut param = 0 as c_uchar;
            unsafe { gl_bindings::GetBooleanv(pname, &mut param) };
            return WebGLResult::Boolean(param == 1);
        }
        WEBGL_UNPACK_PREMULTIPLY_ALPHA_WEBGL => {
            return WebGLResult::Boolean(state.get_premultiplied_alpha());
        }
        WEBGL_UNPACK_FLIP_Y_WEBGL => {
            return WebGLResult::Boolean(state.get_flip_y());
        }
        WEBGL_UNPACK_COLORSPACE_CONVERSION_WEBGL => {
            return WebGLResult::U32(state.get_unpack_colorspace_conversion_webgl() as u32);
        }
        gl_bindings::COLOR_WRITEMASK => {
            let mut param: Vec<bool> = vec![false, false, false, false];
            unsafe { gl_bindings::GetBooleanv(pname, param.as_mut_ptr() as *mut c_uchar) };
            return WebGLResult::BooleanArray(param);
        }
        gl_bindings::COMPRESSED_TEXTURE_FORMATS => {
            let mut count = 0i32;
            unsafe {
                gl_bindings::GetIntegerv(gl_bindings::NUM_COMPRESSED_TEXTURE_FORMATS, &mut count)
            }
            let mut params: Vec<i32> = vec![0i32; count as usize];
            unsafe {
                gl_bindings::GetIntegerv(
                    gl_bindings::COMPRESSED_TEXTURE_FORMATS,
                    params.as_mut_ptr(),
                )
            }
            return WebGLResult::I32Array(params);
        }
        gl_bindings::DEPTH_CLEAR_VALUE
        | gl_bindings::LINE_WIDTH
        | gl_bindings::POLYGON_OFFSET_FACTOR
        | gl_bindings::POLYGON_OFFSET_UNITS
        | gl_bindings::SAMPLE_COVERAGE_VALUE => {
            let mut param = 0f32;
            unsafe { gl_bindings::GetFloatv(pname, &mut param) }
            return WebGLResult::F32(param);
        }
        gl_bindings::MAX_VIEWPORT_DIMS => {
            let mut params: Vec<i32> = vec![0, 0];
            unsafe { gl_bindings::GetIntegerv(pname, params.as_mut_ptr()) }
            return WebGLResult::I32Array(params);
        }
        gl_bindings::SCISSOR_BOX | gl_bindings::VIEWPORT => {
            let mut params: Vec<i32> = vec![0, 0, 0, 0];
            unsafe { gl_bindings::GetIntegerv(pname, params.as_mut_ptr()) }
            return WebGLResult::I32Array(params);
        }
        gl_bindings::RENDERER
        | gl_bindings::SHADING_LANGUAGE_VERSION
        | gl_bindings::VENDOR
        | gl_bindings::VERSION => {
            let params = unsafe { gl_bindings::GetString(pname) };
            if params.is_null() {
                return WebGLResult::None;
            }
            unsafe {
                return WebGLResult::String(CString::from(CStr::from_ptr(std::mem::transmute(
                    params,
                ))));
            }
        }
        _ => WebGLResult::None,
    };

    ret
}

pub fn canvas_native_webgl_get_program_info_log(program: u32, state: &mut WebGLState) -> String {
    state.make_current();
    let mut length = 0i32;
    unsafe { gl_bindings::GetProgramiv(program, gl_bindings::INFO_LOG_LENGTH, &mut length) }
    let mut info = vec![0; length as usize];
    let mut len = 0;
    unsafe {
        gl_bindings::GetProgramInfoLog(program, length, &mut len, info.as_mut_ptr() as *mut c_char)
    }

    if len == 0 {
        return String::with_capacity(0);
    }
    info.shrink_to(len.try_into().unwrap());
    let c_str = unsafe { CStr::from_ptr(info.as_ptr()) };
    c_str.to_string_lossy().to_string()
}

pub fn canvas_native_webgl_get_program_parameter(
    program: u32,
    pname: u32,
    state: &mut WebGLState,
) -> WebGLResult {
    state.make_current();
    let mut param = 0i32;
    unsafe { gl_bindings::GetProgramiv(program, pname, &mut param) }

    match pname {
        gl_bindings::DELETE_STATUS | gl_bindings::LINK_STATUS | gl_bindings::VALIDATE_STATUS => {
            if param as u32 == gl_bindings::TRUE as u32 {
                return WebGLResult::Boolean(true);
            }
            return WebGLResult::Boolean(false);
        }
        gl_bindings::ATTACHED_SHADERS
        | gl_bindings::ACTIVE_ATTRIBUTES
        | gl_bindings::ACTIVE_UNIFORMS => {
            return WebGLResult::I32(param);
        }
        _ => WebGLResult::None,
    }
}

pub fn canvas_native_webgl_get_renderbuffer_parameter(
    target: u32,
    pname: u32,
    state: &mut WebGLState,
) -> i32 {
    state.make_current();
    let mut params = 0i32;
    unsafe { gl_bindings::GetRenderbufferParameteriv(target, pname, &mut params) }

    params
}

pub fn canvas_native_webgl_get_shader_info_log(shader: u32, state: &mut WebGLState) -> String {
    state.make_current();
    let mut length = 0i32;
    unsafe { gl_bindings::GetShaderiv(shader, gl_bindings::INFO_LOG_LENGTH, &mut length) }

    if length == 0 {
        return String::new();
    }

    let mut log = vec![0; length as usize];
    let mut len = 0;
    unsafe {
        gl_bindings::GetShaderInfoLog(shader, length, &mut len, log.as_mut_ptr() as *mut c_char)
    }

    log.shrink_to(len.try_into().unwrap());
    let c_str = unsafe { CStr::from_ptr(log.as_ptr()) };
    c_str.to_string_lossy().to_string()
}

pub fn canvas_native_webgl_get_shader_parameter(
    shader: u32,
    pname: u32,
    state: &mut WebGLState,
) -> WebGLResult {
    state.make_current();
    let mut params = 0i32;
    unsafe { gl_bindings::GetShaderiv(shader, pname, &mut params) }

    return match pname {
        gl_bindings::DELETE_STATUS | gl_bindings::COMPILE_STATUS => {
            if params as u32 == gl_bindings::TRUE as u32 {
                WebGLResult::Boolean(true)
            } else {
                WebGLResult::Boolean(false)
            }
        }
        _ => WebGLResult::U32(params as u32),
    };
}

pub fn canvas_native_webgl_get_shader_precision_format(
    shader_type: u32,
    precision_type: u32,
    state: &mut WebGLState,
) -> WebGLShaderPrecisionFormat {
    state.make_current();
    let mut range = [0i32; 2];
    let mut precision = 0i32;
    unsafe {
        gl_bindings::GetShaderPrecisionFormat(
            shader_type,
            precision_type,
            range.as_mut_ptr(),
            &mut precision,
        )
    }

    WebGLShaderPrecisionFormat::new(precision, range[0], range[1])
}

pub fn canvas_native_webgl_get_shader_source(shader: u32, state: &mut WebGLState) -> String {
    state.make_current();
    let mut length = 0i32;
    unsafe { gl_bindings::GetShaderiv(shader, gl_bindings::SHADER_SOURCE_LENGTH, &mut length) }
    let mut source = vec![0; length as usize];
    let mut len = 0;
    unsafe {
        gl_bindings::GetShaderSource(shader, length, &mut len, source.as_mut_ptr() as *mut c_char)
    }

    source.shrink_to(len.try_into().unwrap());
    let c_str = unsafe { CStr::from_ptr(source.as_ptr()) };
    c_str.to_string_lossy().to_string()
}

pub fn canvas_native_webgl_get_supported_extensions(state: &mut WebGLState) -> Vec<String> {
    state.make_current();
    let ext = unsafe { gl_bindings::GetString(gl_bindings::EXTENSIONS) as *const c_char };

    if ext.is_null() {
        return Vec::with_capacity(0);
    }
    let version = unsafe { String::from_utf8(CStr::from_ptr(ext).to_bytes().to_vec()).unwrap() };

    // let extensions = unsafe { CStr::from_ptr(std::mem::transmute(ext)) };
    // let extensions = extensions.to_string_lossy();
    version.split(" ").map(|f| f.into()).collect()
}

pub fn canvas_native_webgl_get_tex_parameter(
    target: u32,
    pname: u32,
    state: &mut WebGLState,
) -> i32 {
    state.make_current();
    let mut params = 0i32;
    unsafe { gl_bindings::GetTexParameteriv(target, pname, &mut params) }

    return params;
}

pub fn canvas_native_webgl_get_uniform_location(
    program: u32,
    name: &str,
    state: &mut WebGLState,
) -> i32 {
    state.make_current();
    let name = CString::new(name).unwrap();
    let ret = unsafe { gl_bindings::GetUniformLocation(program, name.as_ptr()) };

    ret
}

pub fn canvas_native_webgl_get_uniform(
    program: u32,
    location: i32,
    state: &mut WebGLState,
) -> WebGLResult {
    state.make_current();
    let mut uniform_type = 0u32;
    let mut size = 0;
    let mut length = 0;
    unsafe {
        gl_bindings::GetActiveUniform(
            program,
            location as u32,
            0,
            &mut length,
            &mut size,
            &mut uniform_type,
            std::ptr::null_mut(),
        )
    }
    let ret = match uniform_type {
        gl_bindings::FLOAT => {
            let mut single = [0f32; 1];
            unsafe { gl_bindings::GetUniformfv(program, location, single.as_mut_ptr()) }
            return WebGLResult::F32(single[0]);
        }
        gl_bindings::FLOAT_VEC2 => {
            let mut vec2: Vec<f32> = vec![0., 0.];
            unsafe { gl_bindings::GetUniformfv(program, location, vec2.as_mut_ptr()) }
            return WebGLResult::F32Array(vec2);
        }
        gl_bindings::FLOAT_VEC3 => {
            let mut vec3: Vec<f32> = vec![0., 0., 0.];
            unsafe { gl_bindings::GetUniformfv(program, location, vec3.as_mut_ptr()) }
            return WebGLResult::F32Array(vec3);
        }
        gl_bindings::FLOAT_VEC4 => {
            let mut vec4: Vec<f32> = vec![0., 0., 0., 0.];
            unsafe { gl_bindings::GetUniformfv(program, location, vec4.as_mut_ptr()) }
            return WebGLResult::F32Array(vec4);
        }
        gl_bindings::INT | gl_bindings::SAMPLER_2D | gl_bindings::SAMPLER_CUBE => {
            let mut single_int = [0i32; 1];
            unsafe {
                gl_bindings::GetUniformiv(program, location, single_int.as_mut_ptr());
            }
            return WebGLResult::I32(single_int[0]);
        }
        gl_bindings::INT_VEC2 => {
            let mut int_vec2: Vec<i32> = vec![0, 0];
            unsafe { gl_bindings::GetUniformiv(program, location, int_vec2.as_mut_ptr()) }
            return WebGLResult::I32Array(int_vec2);
        }
        gl_bindings::INT_VEC3 => {
            let mut int_vec3: Vec<i32> = vec![0, 0, 0];
            unsafe { gl_bindings::GetUniformiv(program, location, int_vec3.as_mut_ptr()) }
            return WebGLResult::I32Array(int_vec3);
        }
        gl_bindings::INT_VEC4 => {
            let mut int_vec4: Vec<i32> = vec![0, 0, 0, 0];
            unsafe { gl_bindings::GetUniformiv(program, location, int_vec4.as_mut_ptr()) }
            return WebGLResult::I32Array(int_vec4);
        }
        gl_bindings::BOOL => {
            let mut single_bool = [0i32; 1];
            unsafe { gl_bindings::GetUniformiv(program, location, single_bool.as_mut_ptr()) }
            return WebGLResult::Boolean(single_bool[0] == 1);
        }
        gl_bindings::BOOL_VEC2 => {
            let mut bool_vec2 = [0i32; 2];
            unsafe { gl_bindings::GetUniformiv(program, location, bool_vec2.as_mut_ptr()) }
            return WebGLResult::BooleanArray(unsafe {
                std::slice::from_raw_parts(bool_vec2.as_ptr() as *const bool, 2).to_vec()
            });
        }
        gl_bindings::BOOL_VEC3 => {
            let mut bool_vec3 = [0i32; 3];
            unsafe { gl_bindings::GetUniformiv(program, location, bool_vec3.as_mut_ptr()) }
            return WebGLResult::BooleanArray(unsafe {
                std::slice::from_raw_parts(bool_vec3.as_ptr() as *const bool, 3).to_vec()
            });
        }
        gl_bindings::BOOL_VEC4 => {
            let mut bool_vec4 = [0i32; 4];
            unsafe { gl_bindings::GetUniformiv(program, location, bool_vec4.as_mut_ptr()) }
            return WebGLResult::BooleanArray(unsafe {
                std::slice::from_raw_parts(bool_vec4.as_ptr() as *const bool, 4).to_vec()
            });
        }
        gl_bindings::FLOAT_MAT2 => {
            let mut mat2: Vec<f32> = vec![0., 0.];
            unsafe {
                gl_bindings::GetUniformfv(program, location, mat2.as_mut_ptr());
            }
            return WebGLResult::F32Array(mat2);
        }
        gl_bindings::FLOAT_MAT3 => {
            let mut mat3: Vec<f32> = vec![0., 0., 0.];
            unsafe {
                gl_bindings::GetUniformfv(program, location, mat3.as_mut_ptr());
            }
            return WebGLResult::F32Array(mat3);
        }
        gl_bindings::FLOAT_MAT4 => {
            let mut mat4: Vec<f32> = vec![0., 0., 0., 0.];
            unsafe {
                gl_bindings::GetUniformfv(program, location, mat4.as_mut_ptr());
            }
            return WebGLResult::F32Array(mat4);
        }
        _ => WebGLResult::None,
    };

    ret
}

pub fn canvas_native_webgl_get_vertex_attrib_offset(
    index: u32,
    pname: u32,
    state: &mut WebGLState,
) -> usize {
    state.make_current();
    let mut ptr = [0 as usize; 1];
    let ptr_ptr = &mut (ptr.as_mut_ptr() as *mut c_void);
    unsafe { gl_bindings::GetVertexAttribPointerv(index, pname, ptr_ptr) }

    ptr[0]
}

pub fn canvas_native_webgl_get_vertex_attrib(
    index: u32,
    pname: u32,
    state: &mut WebGLState,
) -> WebGLResult {
    state.make_current();
    if pname == gl_bindings::CURRENT_VERTEX_ATTRIB {
        let mut params: Vec<f32> = vec![0., 0., 0., 0.];
        unsafe { gl_bindings::GetVertexAttribfv(index, pname, params.as_mut_ptr()) }

        return WebGLResult::F32Array(params);
    }
    let mut params = 0i32;
    unsafe { gl_bindings::GetVertexAttribiv(index, pname, &mut params) }

    match pname {
        gl_bindings::VERTEX_ATTRIB_ARRAY_ENABLED
        | gl_bindings::VERTEX_ATTRIB_ARRAY_NORMALIZED
        | gl_bindings::VERTEX_ATTRIB_ARRAY_INTEGER => return WebGLResult::Boolean(params == 1),
        gl_bindings::VERTEX_ATTRIB_ARRAY_BUFFER_BINDING => WebGLResult::U32(params as u32),
        gl_bindings::VERTEX_ATTRIB_ARRAY_SIZE
        | gl_bindings::VERTEX_ATTRIB_ARRAY_STRIDE
        | gl_bindings::VERTEX_ATTRIB_ARRAY_DIVISOR => return WebGLResult::I32(params),
        _ => return WebGLResult::None,
    }
}

pub fn canvas_native_webgl_get_is_context_lost(_state: &mut WebGLState) -> bool {
    // TODO improve
    false
}

pub fn canvas_native_webgl_hint(target: u32, mode: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::Hint(target, mode) }
}

pub fn canvas_native_webgl_is_buffer(buffer: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    let ret = unsafe { gl_bindings::IsBuffer(buffer) == 1 };

    ret
}

pub fn canvas_native_webgl_is_enabled(cap: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    let ret = unsafe { gl_bindings::IsEnabled(cap) == 1 };

    ret
}

pub fn canvas_native_webgl_is_framebuffer(framebuffer: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    let ret = unsafe { gl_bindings::IsFramebuffer(framebuffer) == 1 };

    ret
}

pub fn canvas_native_webgl_is_program(program: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    let ret = unsafe { gl_bindings::IsProgram(program) == 1 };

    ret
}

pub fn canvas_native_webgl_is_renderbuffer(renderbuffer: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    let ret = unsafe { gl_bindings::IsRenderbuffer(renderbuffer) == 1 };

    ret
}

pub fn canvas_native_webgl_is_shader(shader: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    let ret = unsafe { gl_bindings::IsShader(shader) == 1 };

    ret
}

pub fn canvas_native_webgl_is_texture(texture: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    let ret = unsafe { gl_bindings::IsTexture(texture) == 1 };

    ret
}

pub fn canvas_native_webgl_line_width(width: f32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::LineWidth(width) }
}

pub fn canvas_native_webgl_link_program(program: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::LinkProgram(program) }
}

pub fn canvas_native_webgl_pixel_storei(pname: u32, param: i32, state: &mut WebGLState) {
    match pname {
        gl_bindings::UNPACK_ALIGNMENT | gl_bindings::PACK_ALIGNMENT => unsafe {
            state.make_current();
            gl_bindings::PixelStorei(pname, param);
        },
        WEBGL_UNPACK_FLIP_Y_WEBGL => {
            if param == 1 {
                state.set_flip_y(true);
            } else {
                state.set_flip_y(false);
            }
        }
        WEBGL_UNPACK_PREMULTIPLY_ALPHA_WEBGL => {
            if param == 1 {
                state.set_premultiplied_alpha(true);
            } else {
                state.set_premultiplied_alpha(false);
            }
        }
        WEBGL_UNPACK_COLORSPACE_CONVERSION_WEBGL => {
            state.set_unpack_colorspace_conversion_webgl(param);
        }
        _ => {}
    }
}

pub fn canvas_native_webgl_polygon_offset(factor: f32, units: f32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::PolygonOffset(factor, units) }
}

pub fn canvas_native_webgl_read_pixels_u8(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    pixel_type: u32,
    pixels: &mut [u8],
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::ReadPixels(
            x,
            y,
            width,
            height,
            format,
            pixel_type,
            pixels.as_mut_ptr() as *mut c_void,
        )
    }
}

pub fn canvas_native_webgl_read_pixels_u16(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    pixel_type: u32,
    pixels: &mut [u16],
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::ReadPixels(
            x,
            y,
            width,
            height,
            format,
            pixel_type,
            pixels.as_mut_ptr() as *mut c_void,
        )
    }
}

pub fn canvas_native_webgl_read_pixels_f32(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    pixel_type: u32,
    pixels: &mut [f32],
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::ReadPixels(
            x,
            y,
            width,
            height,
            format,
            pixel_type,
            pixels.as_mut_ptr() as *mut c_void,
        )
    }
}

pub fn canvas_native_webgl_renderbuffer_storage(
    target: u32,
    internal_format: u32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe { gl_bindings::RenderbufferStorage(target, internal_format, width, height) }
}

pub fn canvas_native_webgl_sample_coverage(value: f32, invert: bool, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::SampleCoverage(value, invert as u8) }
}

pub fn canvas_native_webgl_scissor(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe { gl_bindings::Scissor(x, y, width, height) }
}

pub fn canvas_native_webgl_shader_source(shader: u32, source: &str, state: &mut WebGLState) {
    state.make_current();
    let src = CString::new(source).unwrap();
    unsafe { gl_bindings::ShaderSource(shader, 1, &src.as_ptr(), std::ptr::null()) }
}

pub fn canvas_native_webgl_stencil_func(
    func: u32,
    reference: i32,
    mask: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe { gl_bindings::StencilFunc(func, reference, mask) }
}

pub fn canvas_native_webgl_stencil_func_separate(
    face: u32,
    func: u32,
    reference: i32,
    mask: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    match face {
        gl_bindings::FRONT_AND_BACK => {
            state.set_stencil_func_ref(reference);
            state.set_stencil_func_ref_back(reference);
            state.set_stencil_func_mask(mask);
            state.set_stencil_func_mask_back(mask);
        }
        gl_bindings::FRONT => {
            state.set_stencil_func_ref(reference);
            state.set_stencil_func_mask(mask);
        }
        gl_bindings::BACK => {
            state.set_stencil_func_ref_back(reference);
            state.set_stencil_func_mask_back(mask);
        }
        _ => {}
    }
    unsafe { gl_bindings::StencilFuncSeparate(face, func, reference, mask) }
}

pub fn canvas_native_webgl_stencil_mask(mask: u32, state: &mut WebGLState) {
    state.make_current();
    state.set_stencil_mask(mask);
    state.set_stencil_mask_back(mask);
    unsafe { gl_bindings::StencilMask(mask) }
}

pub fn canvas_native_webgl_stencil_mask_separate(face: u32, mask: u32, state: &mut WebGLState) {
    state.make_current();
    match face {
        gl_bindings::FRONT_AND_BACK => {
            state.set_stencil_mask(mask);
            state.set_stencil_mask_back(mask);
        }
        gl_bindings::FRONT => {
            state.set_stencil_mask(mask);
        }
        gl_bindings::BACK => {
            state.set_stencil_mask_back(mask);
        }
        _ => {}
    }
    unsafe { gl_bindings::StencilMaskSeparate(face, mask) }
}

pub fn canvas_native_webgl_stencil_op(fail: u32, zfail: u32, zpass: u32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::StencilOp(fail, zfail, zpass) }
}

pub fn canvas_native_webgl_stencil_op_separate(
    face: u32,
    fail: u32,
    zfail: u32,
    zpass: u32,
    state: &WebGLState,
) {
    state.make_current();
    unsafe { gl_bindings::StencilOpSeparate(face, fail, zfail, zpass) }
}

pub fn canvas_native_webgl_tex_image2d_asset(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    asset: &ImageAsset,
    state: &WebGLState,
) {
    if let Some(bytes) = asset.get_bytes() {
        let width = asset.width() as i32;
        let height = asset.height() as i32;
        state.make_current();
        unsafe {
            if state.get_flip_y() {
                let mut buffer = bytes.to_vec();
                utils::gl::flip_in_place(
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    (utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32 * width)
                        as usize,
                    height as usize,
                );

                gl_bindings::TexImage2D(
                    target as u32,
                    level,
                    internalformat,
                    width,
                    height,
                    0,
                    format as u32,
                    image_type as u32,
                    buffer.as_ptr() as *const c_void,
                );

                return;
            }

            gl_bindings::TexImage2D(
                target as u32,
                level,
                internalformat,
                width,
                height,
                0,
                format as u32,
                image_type as u32,
                bytes.as_ptr() as *const c_void,
            );
        }
    }
}

pub fn canvas_native_webgl_read_webgl_pixels(
    source: &mut WebGLState,
    context: &mut WebGLState,
) -> (i32, i32, Vec<u8>) {
    context.remove_if_current();
    source.make_current();
    let width = source.get_drawing_buffer_width();
    let height = source.get_drawing_buffer_height();

    let mut buf = vec![0u8; (width * height * 4) as usize];

    unsafe {
        gl_bindings::Flush();
        gl_bindings::ReadPixels(
            0,
            0,
            width,
            height,
            gl_bindings::RGBA,
            gl_bindings::UNSIGNED_BYTE,
            buf.as_mut_ptr() as *mut c_void,
        );
    }

    context.make_current();

    (width as i32, height as i32, buf)
}

//    texImage2D(target, level, internalformat, width, height, border, format, type)
//    texImage2D(target, level, internalformat, width, height, border, format, type, pixels) // pixels is instance of ArrayBufferView
//    texImage2D(target, level, internalformat, format, type)
//    texImage2D(target, level, internalformat, format, type, pixels)

pub fn canvas_native_webgl_tex_image2d_image_none(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    state: &WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::TexImage2D(
            target as u32,
            level,
            internalformat,
            0,
            0,
            0,
            format as u32,
            image_type as u32,
            std::ptr::null(),
        );
    }
}

pub fn canvas_native_webgl_tex_image2d(
    target: i32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    border: i32,
    format: i32,
    image_type: i32,
    buf: &[u8],
    state: &WebGLState,
) {
    state.make_current();
    unsafe {
        if state.get_flip_y() {
            let mut buffer = buf.to_vec();
            utils::gl::flip_in_place(
                buffer.as_mut_ptr(),
                buffer.len(),
                (utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32 * width)
                    as usize,
                height as usize,
            );

            gl_bindings::TexImage2D(
                target as u32,
                level,
                internalformat,
                width,
                height,
                border,
                format as u32,
                image_type as u32,
                buffer.as_ptr() as *const c_void,
            );

            return;
        }

        gl_bindings::TexImage2D(
            target as u32,
            level,
            internalformat,
            width,
            height,
            border,
            format as u32,
            image_type as u32,
            buf.as_ptr() as *const c_void,
        );
    }
}

pub fn canvas_native_webgl_tex_image2d_none(
    target: i32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    border: i32,
    format: i32,
    image_type: i32,
    state: &WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::TexImage2D(
            target as u32,
            level,
            internalformat,
            width,
            height,
            border,
            format as u32,
            image_type as u32,
            std::ptr::null(),
        );
    }
}

pub fn canvas_native_webgl_tex_image2d_image_asset(
    target: i32,
    level: i32,
    _internalformat: i32,
    format: i32,
    image_type: i32,
    image_asset: &ImageAsset,
    state: &WebGLState,
) {
    state.make_current();


    let width = image_asset.width() as i32;
    let height = image_asset.height() as i32;
    let flip_y = state.get_flip_y();


    match utils::gl::get_image_asset_bytes_type(format, image_type) {
        GLImageAssetBytesType::RGBA8 => {
            if let Some(bytes) = image_asset.get_bytes() {
                unsafe {
                    if flip_y {
                        let mut buffer = bytes.to_vec();
                        utils::gl::flip_in_place(
                            buffer.as_mut_ptr(),
                            buffer.len(),
                            (utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32 * width)
                                as usize,
                            height as usize,
                        );


                        gl_bindings::TexImage2D(
                            target as u32,
                            level,
                            gl_bindings::RGBA as _,
                            width as _,
                            height as _,
                            0,
                            gl_bindings::RGBA as _,
                            gl_bindings::UNSIGNED_BYTE as _,
                            buffer.as_ptr() as *const c_void,
                        );

                        return;
                    }

                    gl_bindings::TexImage2D(
                        target as u32,
                        level,
                        gl_bindings::RGBA as _,
                        width as _,
                        height as _,
                        0,
                        gl_bindings::RGBA as _,
                        gl_bindings::UNSIGNED_BYTE as _,
                        bytes.as_ptr() as *const c_void,
                    );
                }
            }
        }
        GLImageAssetBytesType::Luminance => {
            if let Some(bytes) = image_asset.get_luminance_bytes() {
                unsafe {
                    if flip_y {
                        let mut buffer = bytes.to_vec();
                        utils::gl::flip_in_place(
                            buffer.as_mut_ptr(),
                            buffer.len(),
                            (utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32 * width)
                                as usize,
                            height as usize,
                        );


                        gl_bindings::TexImage2D(
                            target as u32,
                            level,
                            gl_bindings::LUMINANCE_ALPHA as _,
                            width as _,
                            height as _,
                            0,
                            gl_bindings::LUMINANCE_ALPHA as _,
                            gl_bindings::UNSIGNED_BYTE as _,
                            buffer.as_ptr() as *const c_void,
                        );

                        return;
                    }

                    gl_bindings::TexImage2D(
                        target as u32,
                        level,
                        gl_bindings::LUMINANCE_ALPHA as _,
                        width as _,
                        height as _,
                        0,
                        gl_bindings::LUMINANCE_ALPHA as _,
                        gl_bindings::UNSIGNED_BYTE as _,
                        bytes.as_ptr() as *const c_void,
                    );
                }
            }
        }
        GLImageAssetBytesType::Alpha => {
            // todo
        }
        GLImageAssetBytesType::None => {
            // noop
        }
    }
}

pub fn canvas_native_webgl_tex_parameterf(target: u32, pname: u32, param: f32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::TexParameterf(target, pname, param) }
}

pub fn canvas_native_webgl_tex_parameteri(target: u32, pname: u32, param: i32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::TexParameteri(target, pname, param) }
}

pub fn canvas_native_webgl_tex_sub_image2d_asset(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    format: u32,
    image_type: i32,
    asset: &ImageAsset,
    state: &WebGLState,
) {
    if let Some(bytes) = asset.get_bytes() {
        let width = asset.width() as i32;
        let height = asset.height() as i32;
        state.make_current();
        if state.get_flip_y() {
            let mut buffer = bytes.to_vec();
            utils::gl::flip_in_place(
                buffer.as_mut_ptr(),
                bytes.len(),
                (utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32 * width as i32)
                    as usize,
                height as usize,
            );

            unsafe {
                gl_bindings::TexSubImage2D(
                    target as u32,
                    level,
                    xoffset,
                    yoffset,
                    width,
                    height,
                    format as u32,
                    image_type as u32,
                    buffer.as_ptr() as *const c_void,
                );
            }

            return;
        }
        unsafe {
            gl_bindings::TexSubImage2D(
                target as u32,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format as u32,
                image_type as u32,
                bytes.as_ptr() as *const c_void,
            );
        }
    }
}

pub fn canvas_native_webgl_tex_sub_image2d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    width: i32,
    height: i32,
    format: u32,
    image_type: i32,
    buf: &[u8],
    state: &WebGLState,
) {
    state.make_current();
    if state.get_flip_y() {
        let mut buffer = buf.to_vec();
        utils::gl::flip_in_place(
            buffer.as_mut_ptr(),
            buf.len(),
            (utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32 * width as i32)
                as usize,
            height as usize,
        );

        unsafe {
            gl_bindings::TexSubImage2D(
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format,
                image_type as u32,
                buffer.as_ptr() as *const c_void,
            );
        }

        return;
    }
    unsafe {
        gl_bindings::TexSubImage2D(
            target,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format,
            image_type as u32,
            buf.as_ptr() as *const c_void,
        );
    }
}

pub fn canvas_native_webgl_uniform1f(location: i32, v0: f32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::Uniform1f(location, v0) }
}

pub fn canvas_native_webgl_uniform1fv(location: i32, value: &[f32], state: &WebGLState) {
    state.make_current();
    unsafe {
        let count = value.len();
        gl_bindings::Uniform1fv(location, count.try_into().unwrap(), value.as_ptr())
    }
}

pub fn canvas_native_webgl_uniform1i(location: i32, v0: i32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::Uniform1i(location, v0) }
}

pub fn canvas_native_webgl_uniform1iv(location: i32, value: &[i32], state: &WebGLState) {
    state.make_current();
    unsafe {
        let count = value.len();
        gl_bindings::Uniform1iv(location, count.try_into().unwrap(), value.as_ptr())
    }
}

pub fn canvas_native_webgl_uniform2f(location: i32, v0: f32, v1: f32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::Uniform2f(location, v0, v1) }
}

pub fn canvas_native_webgl_uniform2fv(location: i32, value: &[f32], state: &WebGLState) {
    state.make_current();
    unsafe {
        let count = value.len() / 2;
        gl_bindings::Uniform2fv(location, count.try_into().unwrap(), value.as_ptr())
    }
}

pub fn canvas_native_webgl_uniform2i(location: i32, v0: i32, v1: i32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::Uniform2i(location, v0, v1) }
}

pub fn canvas_native_webgl_uniform2iv(location: i32, value: &[i32], state: &WebGLState) {
    state.make_current();
    unsafe {
        let count = value.len() / 2;
        gl_bindings::Uniform2iv(location, count.try_into().unwrap(), value.as_ptr())
    }
}

pub fn canvas_native_webgl_uniform3f(location: i32, v0: f32, v1: f32, v2: f32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::Uniform3f(location, v0, v1, v2) }
}

pub fn canvas_native_webgl_uniform3fv(location: i32, value: &[f32], state: &WebGLState) {
    state.make_current();
    unsafe {
        let count = value.len() / 3;
        gl_bindings::Uniform3fv(location, count.try_into().unwrap(), value.as_ptr())
    }
}

pub fn canvas_native_webgl_uniform3i(location: i32, v0: i32, v1: i32, v2: i32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::Uniform3i(location, v0, v1, v2) }
}

pub fn canvas_native_webgl_uniform3iv(location: i32, value: &[i32], state: &WebGLState) {
    state.make_current();
    unsafe {
        let count = value.len() / 3;
        gl_bindings::Uniform3iv(location, count.try_into().unwrap(), value.as_ptr())
    }
}

pub fn canvas_native_webgl_uniform4f(
    location: i32,
    v0: f32,
    v1: f32,
    v2: f32,
    v3: f32,
    state: &WebGLState,
) {
    state.make_current();
    unsafe { gl_bindings::Uniform4f(location, v0, v1, v2, v3) }
}

pub fn canvas_native_webgl_uniform4fv(location: i32, value: &[f32], state: &WebGLState) {
    state.make_current();
    unsafe {
        let count = value.len() / 4;
        gl_bindings::Uniform4fv(location, count.try_into().unwrap(), value.as_ptr())
    }
}

pub fn canvas_native_webgl_uniform4i(
    location: i32,
    v0: i32,
    v1: i32,
    v2: i32,
    v3: i32,
    state: &WebGLState,
) {
    state.make_current();
    unsafe { gl_bindings::Uniform4i(location, v0, v1, v2, v3) }
}

pub fn canvas_native_webgl_uniform4iv(location: i32, value: &[i32], state: &WebGLState) {
    state.make_current();
    unsafe {
        let count = value.len() / 4;
        gl_bindings::Uniform4iv(location, count.try_into().unwrap(), value.as_ptr())
    }
}

pub fn canvas_native_webgl_uniform_matrix2fv(
    location: i32,
    transpose: bool,
    value: &[f32],
    state: &WebGLState,
) {
    state.make_current();
    unsafe {
        let count = value.len() / 4;
        gl_bindings::UniformMatrix2fv(
            location,
            count.try_into().unwrap(),
            transpose as u8,
            value.as_ptr(),
        )
    }
}

pub fn canvas_native_webgl_uniform_matrix3fv(
    location: i32,
    transpose: bool,
    value: &[f32],
    state: &WebGLState,
) {
    state.make_current();
    unsafe {
        let count = value.len() / 9;
        gl_bindings::UniformMatrix3fv(
            location,
            count.try_into().unwrap(),
            transpose as u8,
            value.as_ptr(),
        )
    }
}

pub fn canvas_native_webgl_uniform_matrix4fv(
    location: i32,
    transpose: bool,
    value: &[f32],
    state: &WebGLState,
) {
    state.make_current();
    unsafe {
        let count = value.len() / 16;
        gl_bindings::UniformMatrix4fv(
            location,
            count.try_into().unwrap(),
            transpose as u8,
            value.as_ptr(),
        )
    }
}

pub fn canvas_native_webgl_use_program(program: u32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::UseProgram(program) }
}

pub fn canvas_native_webgl_validate_program(program: u32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::ValidateProgram(program) }
}

pub fn canvas_native_webgl_vertex_attrib1f(index: u32, v0: f32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::VertexAttrib1f(index, v0) }
}

pub fn canvas_native_webgl_vertex_attrib1fv(index: u32, value: &[f32], state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::VertexAttrib1fv(index, value.as_ptr()) }
}

pub fn canvas_native_webgl_vertex_attrib2f(index: u32, v0: f32, v1: f32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::VertexAttrib2f(index, v0, v1) }
}

pub fn canvas_native_webgl_vertex_attrib2fv(index: u32, value: &[f32], state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::VertexAttrib2fv(index, value.as_ptr()) }
}

pub fn canvas_native_webgl_vertex_attrib3f(
    index: u32,
    v0: f32,
    v1: f32,
    v2: f32,
    state: &WebGLState,
) {
    state.make_current();
    unsafe { gl_bindings::VertexAttrib3f(index, v0, v1, v2) }
}

pub fn canvas_native_webgl_vertex_attrib3fv(index: u32, value: &[f32], state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::VertexAttrib3fv(index, value.as_ptr()) }
}

pub fn canvas_native_webgl_vertex_attrib4f(
    index: u32,
    v0: f32,
    v1: f32,
    v2: f32,
    v3: f32,
    state: &WebGLState,
) {
    state.make_current();
    unsafe { gl_bindings::VertexAttrib4f(index, v0, v1, v2, v3) }
}

pub fn canvas_native_webgl_vertex_attrib4fv(index: u32, value: &[f32], state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::VertexAttrib4fv(index, value.as_ptr()) }
}

pub fn canvas_native_webgl_vertex_attrib_pointer(
    index: u32,
    size: i32,
    d_type: u32,
    normalized: bool,
    stride: i32,
    offset: isize,
    state: &WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::VertexAttribPointer(
            index,
            size,
            d_type,
            normalized as u8,
            stride,
            offset as *mut c_void,
        )
    }
}

pub fn canvas_native_webgl_viewport(x: i32, y: i32, width: i32, height: i32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::Viewport(x, y, width, height) }
}

pub const DEPTH_BUFFER_BIT: u32 = 0x00000100;

pub const STENCIL_BUFFER_BIT: u32 = 0x00000400;

pub const COLOR_BUFFER_BIT: u32 = 0x00004000;

pub const POINTS: u32 = 0x0000;

pub const LINES: u32 = 0x0001;

pub const LINE_LOOP: u32 = 0x0002;

pub const LINE_STRIP: u32 = 0x0003;

pub const TRIANGLES: u32 = 0x0004;

pub const TRIANGLE_STRIP: u32 = 0x0005;

pub const TRIANGLE_FAN: u32 = 0x0006;

pub const ZERO: u32 = 0;

pub const ONE: u32 = 1;

pub const SRC_COLOR: u32 = 0x0300;

pub const ONE_MINUS_SRC_COLOR: u32 = 0x0301;

pub const SRC_ALPHA: u32 = 0x0302;

pub const ONE_MINUS_SRC_ALPHA: u32 = 0x0303;

pub const DST_ALPHA: u32 = 0x0304;

pub const ONE_MINUS_DST_ALPHA: u32 = 0x0305;

pub const DST_COLOR: u32 = 0x0306;

pub const ONE_MINUS_DST_COLOR: u32 = 0x0307;

pub const SRC_ALPHA_SATURATE: u32 = 0x0308;

pub const CONSTANT_COLOR: u32 = 0x8001;

pub const ONE_MINUS_CONSTANT_COLOR: u32 = 0x8002;

pub const CONSTANT_ALPHA: u32 = 0x8003;

pub const ONE_MINUS_CONSTANT_ALPHA: u32 = 0x8004;

/* Blending equations */
pub const FUNC_ADD: u32 = 0x8006;

pub const FUNC_SUBTRACT: u32 = 0x800A;

pub const FUNC_REVERSE_SUBTRACT: u32 = 0x800B;

pub const BLEND_EQUATION: u32 = 0x8009;

pub const BLEND_EQUATION_RGB: u32 = 0x8009;

pub const BLEND_EQUATION_ALPHA: u32 = 0x883D;

pub const BLEND_DST_RGB: u32 = 0x80C8;

pub const BLEND_SRC_RGB: u32 = 0x80C9;

pub const BLEND_DST_ALPHA: u32 = 0x80CA;

pub const BLEND_SRC_ALPHA: u32 = 0x80CB;

pub const BLEND_COLOR: u32 = 0x8005;

pub const ARRAY_BUFFER_BINDING: u32 = 0x8894;

pub const ELEMENT_ARRAY_BUFFER_BINDING: u32 = 0x8895;

pub const LINE_WIDTH: u32 = 0x0B21;

pub const ALIASED_POINT_SIZE_RANGE: u32 = 0x846D;

pub const ALIASED_LINE_WIDTH_RANGE: u32 = 0x846E;

pub const CULL_FACE_MODE: u32 = 0x0B45;

pub const FRONT_FACE: u32 = 0x0B46;

pub const DEPTH_RANGE: u32 = 0x0B70;

pub const DEPTH_WRITEMASK: u32 = 0x0B72;

pub const DEPTH_CLEAR_VALUE: u32 = 0x0B73;

pub const DEPTH_FUNC: u32 = 0x0B74;

pub const STENCIL_CLEAR_VALUE: u32 = 0x0B91;

pub const STENCIL_FUNC: u32 = 0x0B92;

pub const STENCIL_FAIL: u32 = 0x0B94;

pub const STENCIL_PASS_DEPTH_FAIL: u32 = 0x0B95;

pub const STENCIL_PASS_DEPTH_PASS: u32 = 0x0B96;

pub const STENCIL_REF: u32 = 0x0B97;

pub const STENCIL_VALUE_MASK: u32 = 0x0B93;

pub const STENCIL_WRITEMASK: u32 = 0x0B98;

pub const STENCIL_BACK_FUNC: u32 = 0x8800;

pub const STENCIL_BACK_FAIL: u32 = 0x8801;

pub const STENCIL_BACK_PASS_DEPTH_FAIL: u32 = 0x8802;

pub const STENCIL_BACK_PASS_DEPTH_PASS: u32 = 0x8803;

pub const STENCIL_BACK_REF: u32 = 0x8CA3;

pub const STENCIL_BACK_VALUE_MASK: u32 = 0x8CA4;

pub const STENCIL_BACK_WRITEMASK: u32 = 0x8CA5;

// getCanvas(): Canvas;

pub const VIEWPORT: u32 = 0x0BA2;

pub const SCISSOR_BOX: u32 = 0x0C10;

pub const COLOR_CLEAR_VALUE: u32 = 0x0C22;

pub const COLOR_WRITEMASK: u32 = 0x0C23;

pub const UNPACK_ALIGNMENT: u32 = 0x0CF5;

pub const PACK_ALIGNMENT: u32 = 0x0D05;

pub const MAX_TEXTURE_SIZE: u32 = 0x0D33;

pub const MAX_VIEWPORT_DIMS: u32 = 0x0D3A;

pub const SUBPIXEL_BITS: u32 = 0x0D50;

pub const RED_BITS: u32 = 0x0D52;

pub const GREEN_BITS: u32 = 0x0D53;

pub const BLUE_BITS: u32 = 0x0D54;

pub const ALPHA_BITS: u32 = 0x0D55;

pub const DEPTH_BITS: u32 = 0x0D56;

pub const STENCIL_BITS: u32 = 0x0D57;

pub const POLYGON_OFFSET_UNITS: u32 = 0x2A00;

pub const POLYGON_OFFSET_FACTOR: u32 = 0x8038;

pub const TEXTURE_BINDING_2D: u32 = 0x8069;

pub const SAMPLE_BUFFERS: u32 = 0x80A8;

pub const SAMPLES: u32 = 0x80A9;

pub const SAMPLE_COVERAGE_VALUE: u32 = 0x80AA;

pub const SAMPLE_COVERAGE_INVERT: u32 = 0x80AB;

pub const COMPRESSED_TEXTURE_FORMATS: u32 = 0x86A3;

pub const VENDOR: u32 = 0x1F00;

pub const RENDERER: u32 = 0x1F01;

pub const VERSION: u32 = 0x1F02;

pub const IMPLEMENTATION_COLOR_READ_TYPE: u32 = 0x8B9A;

pub const IMPLEMENTATION_COLOR_READ_FORMAT: u32 = 0x8B9B;

pub const BROWSER_DEFAULT_WEBGL: u32 = 0x9244;

pub const STATIC_DRAW: u32 = 0x88E4;

pub const STREAM_DRAW: u32 = 0x88E0;

pub const DYNAMIC_DRAW: u32 = 0x88E8;

pub const ARRAY_BUFFER: u32 = 0x8892;

pub const ELEMENT_ARRAY_BUFFER: u32 = 0x8893;

pub const BUFFER_SIZE: u32 = 0x8764;

pub const BUFFER_USAGE: u32 = 0x8765;

pub const CURRENT_VERTEX_ATTRIB: u32 = 0x8626;

pub const VERTEX_ATTRIB_ARRAY_ENABLED: u32 = 0x8622;

pub const VERTEX_ATTRIB_ARRAY_SIZE: u32 = 0x8623;

pub const VERTEX_ATTRIB_ARRAY_STRIDE: u32 = 0x8624;

pub const VERTEX_ATTRIB_ARRAY_TYPE: u32 = 0x8625;

pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: u32 = 0x886A;

pub const VERTEX_ATTRIB_ARRAY_POINTER: u32 = 0x8645;

pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: u32 = 0x889F;

pub const CULL_FACE: u32 = 0x0B44;

pub const FRONT: u32 = 0x0404;

pub const BACK: u32 = 0x0405;

pub const FRONT_AND_BACK: u32 = 0x0408;

pub const BLEND: u32 = 0x0BE2;

pub const DEPTH_TEST: u32 = 0x0B71;

pub const DITHER: u32 = 0x0BD0;

pub const POLYGON_OFFSET_FILL: u32 = 0x8037;

pub const SAMPLE_ALPHA_TO_COVERAGE: u32 = 0x809E;

pub const SAMPLE_COVERAGE: u32 = 0x80A0;

pub const SCISSOR_TEST: u32 = 0x0C11;

pub const STENCIL_TEST: u32 = 0x0B90;

/* Errors */
pub const NO_ERROR: u32 = 0;

pub const INVALID_ENUM: u32 = 0x0500;

pub const INVALID_VALUE: u32 = 0x0501;

pub const INVALID_OPERATION: u32 = 0x0502;

pub const OUT_OF_MEMORY: u32 = 0x0505;

pub const CONTEXT_LOST_WEBGL: u32 = 0x9242;

pub const CW: u32 = 0x0900;

pub const CCW: u32 = 0x0901;

pub const DONT_CARE: u32 = 0x1100;

pub const FASTEST: u32 = 0x1101;

pub const NICEST: u32 = 0x1102;

pub const GENERATE_MIPMAP_HINT: u32 = 0x8192;

pub const BYTE: u32 = 0x1400;

pub const UNSIGNED_BYTE: u32 = 0x1401;

pub const SHORT: u32 = 0x1402;

pub const UNSIGNED_SHORT: u32 = 0x1403;

pub const INT: u32 = 0x1404;

pub const UNSIGNED_INT: u32 = 0x1405;

pub const FLOAT: u32 = 0x1406;

pub const DEPTH_COMPONENT: u32 = 0x1902;

pub const ALPHA: u32 = 0x1906;

pub const RGB: u32 = 0x1907;

/* Clearing buffers */

pub const RGBA: u32 = 0x1908;

pub const LUMINANCE: u32 = 0x1909;

pub const LUMINANCE_ALPHA: u32 = 0x190A;

/* Clearing buffers */

/* Rendering primitives */

pub const UNSIGNED_SHORT_4_4_4_4: u32 = 0x8033;

pub const UNSIGNED_SHORT_5_5_5_1: u32 = 0x8034;

pub const UNSIGNED_SHORT_5_6_5: u32 = 0x8363;

pub const FRAGMENT_SHADER: u32 = 0x8B30;

pub const VERTEX_SHADER: u32 = 0x8B31;

pub const COMPILE_STATUS: u32 = 0x8B81;

pub const DELETE_STATUS: u32 = 0x8B80;

/* Rendering primitives */

/* Blending modes */

pub const LINK_STATUS: u32 = 0x8B82;

pub const VALIDATE_STATUS: u32 = 0x8B83;

pub const ATTACHED_SHADERS: u32 = 0x8B85;

pub const ACTIVE_ATTRIBUTES: u32 = 0x8B89;

pub const ACTIVE_UNIFORMS: u32 = 0x8B86;

pub const MAX_VERTEX_ATTRIBS: u32 = 0x8869;

pub const MAX_VERTEX_UNIFORM_VECTORS: u32 = 0x8DFB;

pub const MAX_VARYING_VECTORS: u32 = 0x8DFC;

pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 0x8B4D;

pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: u32 = 0x8B4C;

pub const MAX_TEXTURE_IMAGE_UNITS: u32 = 0x8872;

pub const MAX_FRAGMENT_UNIFORM_VECTORS: u32 = 0x8DFD;

pub const SHADER_TYPE: u32 = 0x8B4F;

pub const SHADING_LANGUAGE_VERSION: u32 = 0x8B8C;

pub const CURRENT_PROGRAM: u32 = 0x8B8D;

/* Blending modes */

pub const NEVER: u32 = 0x0200;

pub const LESS: u32 = 0x0201;

pub const EQUAL: u32 = 0x0202;

/* Blending equations */

/* Getting GL parameter information */

pub const LEQUAL: u32 = 0x0203;

pub const GREATER: u32 = 0x0204;

pub const NOTEQUAL: u32 = 0x0205;

pub const GEQUAL: u32 = 0x0206;

pub const ALWAYS: u32 = 0x0207;

pub const KEEP: u32 = 0x1E00;

pub const REPLACE: u32 = 0x1E01;

pub const INCR: u32 = 0x1E02;

pub const DECR: u32 = 0x1E03;

pub const INVERT: u32 = 0x150A;

pub const INCR_WRAP: u32 = 0x8507;

pub const DECR_WRAP: u32 = 0x8508;

pub const NEAREST: u32 = 0x2600;

pub const LINEAR: u32 = 0x2601;

pub const NEAREST_MIPMAP_NEAREST: u32 = 0x2700;

pub const LINEAR_MIPMAP_NEAREST: u32 = 0x2701;

pub const NEAREST_MIPMAP_LINEAR: u32 = 0x2702;

pub const LINEAR_MIPMAP_LINEAR: u32 = 0x2703;

pub const TEXTURE_MAG_FILTER: u32 = 0x2800;

pub const TEXTURE_MIN_FILTER: u32 = 0x2801;

pub const TEXTURE_WRAP_S: u32 = 0x2802;

pub const TEXTURE_WRAP_T: u32 = 0x2803;

pub const TEXTURE_2D: u32 = 0x0DE1;

pub const TEXTURE: u32 = 0x1702;

pub const TEXTURE_CUBE_MAP: u32 = 0x8513;

pub const TEXTURE_BINDING_CUBE_MAP: u32 = 0x8514;

pub const TEXTURE_CUBE_MAP_POSITIVE_X: u32 = 0x8515;

pub const TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = 0x8516;

pub const TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = 0x8517;

pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = 0x8518;

pub const TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = 0x8519;

pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = 0x851A;

pub const MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 0x851C;

pub const TEXTURE0: u32 = 0x84C0;

pub const TEXTURE1: u32 = 0x84C1;

pub const TEXTURE2: u32 = 0x84C2;

pub const TEXTURE3: u32 = 0x84C3;

pub const TEXTURE4: u32 = 0x84C4;

pub const TEXTURE5: u32 = 0x84C5;

pub const TEXTURE6: u32 = 0x84C6;

pub const TEXTURE7: u32 = 0x84C7;

pub const TEXTURE8: u32 = 0x84C8;

pub const TEXTURE9: u32 = 0x84C9;

pub const TEXTURE10: u32 = 0x84CA;

pub const TEXTURE11: u32 = 0x84CB;

pub const TEXTURE12: u32 = 0x84CC;

pub const TEXTURE13: u32 = 0x84CD;

pub const TEXTURE14: u32 = 0x84CE;

pub const TEXTURE15: u32 = 0x84CF;

pub const TEXTURE16: u32 = 0x84D0;

pub const TEXTURE17: u32 = 0x84D1;

pub const TEXTURE18: u32 = 0x84D2;

pub const TEXTURE19: u32 = 0x84D3;

pub const TEXTURE20: u32 = 0x84D4;

pub const TEXTURE21: u32 = 0x84D5;

pub const TEXTURE22: u32 = 0x84D6;

pub const TEXTURE23: u32 = 0x84D7;

pub const TEXTURE24: u32 = 0x84D8;

pub const TEXTURE25: u32 = 0x84D9;

pub const TEXTURE26: u32 = 0x84DA;

pub const TEXTURE27: u32 = 0x84DB;

pub const TEXTURE28: u32 = 0x84DC;

pub const TEXTURE29: u32 = 0x84DD;

/* Getting GL parameter information */

/* Buffers */

pub const TEXTURE30: u32 = 0x84DE;

pub const TEXTURE31: u32 = 0x84DF;

pub const ACTIVE_TEXTURE: u32 = 0x84E0;

pub const REPEAT: u32 = 0x2901;

pub const CLAMP_TO_EDGE: u32 = 0x812F;

pub const MIRRORED_REPEAT: u32 = 0x8370;

pub const FLOAT_VEC2: u32 = 0x8B50;

/* Buffers */

/* Vertex attributes */

pub const FLOAT_VEC3: u32 = 0x8B51;

pub const FLOAT_VEC4: u32 = 0x8B52;

pub const INT_VEC2: u32 = 0x8B53;

pub const INT_VEC3: u32 = 0x8B54;

pub const INT_VEC4: u32 = 0x8B55;

pub const BOOL: u32 = 0x8B56;

pub const BOOL_VEC2: u32 = 0x8B57;

pub const BOOL_VEC3: u32 = 0x8B58;

/* Vertex attributes */

/* Culling */

pub const BOOL_VEC4: u32 = 0x8B59;

pub const FLOAT_MAT2: u32 = 0x8B5A;

pub const FLOAT_MAT3: u32 = 0x8B5B;

pub const FLOAT_MAT4: u32 = 0x8B5C;

/* Culling */

/* Enabling and disabling */

pub const SAMPLER_2D: u32 = 0x8B5E;

pub const SAMPLER_CUBE: u32 = 0x8B60;

pub const LOW_FLOAT: u32 = 0x8DF0;

pub const MEDIUM_FLOAT: u32 = 0x8DF1;

pub const HIGH_FLOAT: u32 = 0x8DF2;

pub const LOW_INT: u32 = 0x8DF3;

pub const MEDIUM_INT: u32 = 0x8DF4;

pub const HIGH_INT: u32 = 0x8DF5;

/* Enabling and disabling */

pub const FRAMEBUFFER: u32 = 0x8D40;

pub const RENDERBUFFER: u32 = 0x8D41;

pub const RGBA4: u32 = 0x8056;

pub const RGB5_A1: u32 = 0x8057;

pub const RGB565: u32 = 0x8D62;

pub const DEPTH_COMPONENT16: u32 = 0x81A5;

pub const STENCIL_INDEX8: u32 = 0x8D48;

/* Errors */

/* Front face directions */

pub const DEPTH_STENCIL: u32 = 0x84F9;

pub const RENDERBUFFER_WIDTH: u32 = 0x8D42;

/* Front face directions */

/* Hints */

pub const RENDERBUFFER_HEIGHT: u32 = 0x8D43;

pub const RENDERBUFFER_INTERNAL_FORMAT: u32 = 0x8D44;

pub const RENDERBUFFER_RED_SIZE: u32 = 0x8D50;

pub const RENDERBUFFER_GREEN_SIZE: u32 = 0x8D51;

/* Hints */

/* Data types */

pub const RENDERBUFFER_BLUE_SIZE: u32 = 0x8D52;

pub const RENDERBUFFER_ALPHA_SIZE: u32 = 0x8D53;

pub const RENDERBUFFER_DEPTH_SIZE: u32 = 0x8D54;

pub const RENDERBUFFER_STENCIL_SIZE: u32 = 0x8D55;

pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: u32 = 0x8CD0;

pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: u32 = 0x8CD1;

pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: u32 = 0x8CD2;

/* Data types */

/* Pixel formats */

pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: u32 = 0x8CD3;

pub const COLOR_ATTACHMENT0: u32 = 0x8CE0;

pub const DEPTH_ATTACHMENT: u32 = 0x8D00;

pub const STENCIL_ATTACHMENT: u32 = 0x8D20;

pub const DEPTH_STENCIL_ATTACHMENT: u32 = 0x821A;

pub const NONE: u32 = 0;

/* Pixel formats */

/* Pixel types */

// pub const UNSIGNED_BYTE(): number { return this.native.UNSIGNED_BYTE

pub const FRAMEBUFFER_COMPLETE: u32 = 0x8CD5;

pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: u32 = 0x8CD6;

pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: u32 = 0x8CD7;

/* Pixel types */

/* Shaders */

pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: u32 = 0x8CD9;

pub const FRAMEBUFFER_UNSUPPORTED: u32 = 0x8CDD;

pub const FRAMEBUFFER_BINDING: u32 = 0x8CA6;

pub const RENDERBUFFER_BINDING: u32 = 0x8CA7;

pub const MAX_RENDERBUFFER_SIZE: u32 = 0x84E8;

pub const INVALID_FRAMEBUFFER_OPERATION: u32 = 0x0506;

pub const UNPACK_FLIP_Y_WEBGL: u32 = 0x9240;

pub const UNPACK_PREMULTIPLY_ALPHA_WEBGL: u32 = 0x9241;

pub const UNPACK_COLORSPACE_CONVERSION_WEBGL: u32 = 0x9243;
