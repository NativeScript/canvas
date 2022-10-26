use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uchar, c_void};

use libc::size_t;

use crate::gl::prelude::*;
use crate::{__log, console_log, console_log_rust, ImageAsset, LogPriority};


pub fn canvas_native_webgl_resized(state: &mut WebGLState) {
    state.resized();
}



pub fn canvas_native_webgl_active_texture(texture: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::glActiveTexture(texture);
    }
}

pub fn canvas_native_webgl_attach_shader(program: u32, shader: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::glAttachShader(program, shader);
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
        gl_bindings::glBindAttribLocation(program, index, name.as_ptr());
    }
}

pub fn canvas_native_webgl_bind_buffer(target: u32, buffer: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::glBindBuffer(target, buffer);
    }
}

pub fn canvas_native_webgl_bind_frame_buffer(
    target: u32,
    framebuffer: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::glBindFramebuffer(target, framebuffer);
    }
}

pub fn canvas_native_webgl_bind_render_buffer(
    target: u32,
    renderbuffer: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::glBindRenderbuffer(target, renderbuffer);
    }
}

pub fn canvas_native_webgl_bind_texture(target: u32, texture: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::glBindTexture(target, texture);
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
        gl_bindings::glBlendColor(red, green, blue, alpha);
    }
}

pub fn canvas_native_webgl_blend_equation_separate(
    mode_rgb: u32,
    mode_alpha: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe {
        gl_bindings::glBlendEquationSeparate(mode_rgb, mode_alpha);
    }
}

pub fn canvas_native_webgl_blend_equation(mode: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::glBlendEquation(mode);
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
        gl_bindings::glBlendFuncSeparate(src_rgb, dst_rgb, src_alpha, dst_alpha);
    }
}

pub fn canvas_native_webgl_blend_func(sfactor: u32, dfactor: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::glBlendFunc(sfactor, dfactor);
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
        gl_bindings::glBufferData(
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
        gl_bindings::glBufferData(
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
        gl_bindings::glBufferData(
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
        gl_bindings::glBufferData(target, size.try_into().unwrap(), std::ptr::null(), usage);
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
        gl_bindings::glBufferSubData(
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
        gl_bindings::glBufferSubData(target, offset.try_into().unwrap(), 0, std::ptr::null());
    }
}

pub fn canvas_native_webgl_check_frame_buffer_status(target: u32, state: &mut WebGLState) -> u32 {
    state.make_current();
    return unsafe { gl_bindings::glCheckFramebufferStatus(target) };
}

fn reset(state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::glDisable(3089 /* GL_SCISSOR_TEST */);
        gl_bindings::glClearColor(0., 0., 0., 0.);
        gl_bindings::glColorMask(1u8, 1u8, 1u8, 1u8);
        let mut clear_mask = 16384; /* GL_COLOR_BUFFER_BIT */
        if state.get_depth() {
            gl_bindings::glClearDepthf(1.);
            clear_mask = clear_mask | 256; /* GL_DEPTH_BUFFER_BIT */
            gl_bindings::glDepthMask(1u8);
        }

        if state.get_stencil() {
            gl_bindings::glClearStencil(0);
            clear_mask = clear_mask | 1024; /* GL_STENCIL_BUFFER_BIT */
            gl_bindings::glStencilMaskSeparate(1028 /* GL_FRONT */, 0xFFFFFFFF);
        }
    }
}

fn restore_state_after_clear(state: &mut WebGLState) {
    // Restore the state that the context set.
    state.make_current();
    if state.get_scissor_enabled() {
        unsafe { gl_bindings::glEnable(gl_bindings::GL_SCISSOR_TEST) }
    }

    unsafe {
        let clear_color = state.get_clear_color();
        let color_mask = state.get_color_mask();
        gl_bindings::glClearColor(
            clear_color[0],
            clear_color[1],
            clear_color[2],
            clear_color[3],
        );
        gl_bindings::glColorMask(
            color_mask[0] as u8,
            color_mask[1] as u8,
            color_mask[2] as u8,
            color_mask[3] as u8,
        );
        gl_bindings::glClearDepthf(state.get_clear_depth());
        gl_bindings::glClearStencil(state.get_clear_stencil());
        gl_bindings::glStencilMaskSeparate(gl_bindings::GL_FRONT, state.get_stencil_mask());
        gl_bindings::glDepthMask(state.get_depth_mask() as u8);
    }
}

fn clear_if_composited(mask: u32, state: &mut WebGLState) -> HowToClear {
    state.make_current();
    let combined_clear = mask > 0 && !state.get_scissor_enabled();

    let m = mask & gl_bindings::GL_COLOR_BUFFER_BIT;

    unsafe {
        gl_bindings::glDisable(gl_bindings::GL_SCISSOR_TEST);
    }

    if combined_clear && m == gl_bindings::GL_COLOR_BUFFER_BIT {
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
            gl_bindings::glClearColor(colors[0], colors[1], colors[2], colors[3])
        }
    } else {
        unsafe { gl_bindings::glClearColor(0., 0., 0., 0.) }
    }

    unsafe { gl_bindings::glColorMask(1u8, 1u8, 1u8, 1u8) }
    let mut clear_mask = 16384;
    if state.get_depth() {
        if !combined_clear || !state.get_depth_mask() || mask & 256 /* GL_DEPTH_BUFFER_BIT */ == 0 {
            unsafe { gl_bindings::glClearDepthf(1.) }
            clear_mask = clear_mask | 256;
            unsafe { gl_bindings::glDepthMask(1u8) }
        }
    }

    if state.get_stencil() {
        if combined_clear && mask & 1024 /* GL_STENCIL_BUFFER_BIT */ != 0 {
            unsafe {
                gl_bindings::glClearStencil(
                    state.get_clear_stencil() & state.get_stencil_mask() as i32,
                )
            }
        } else {
            unsafe {
                gl_bindings::glClearStencil(0);
            }
            clear_mask = clear_mask | 1024;

            unsafe { gl_bindings::glStencilMaskSeparate(gl_bindings::GL_FRONT, 0xFFFFFFFF) }
        }
    }

    unsafe {
        gl_bindings::glClear(mask);
    }
    restore_state_after_clear(state);
    if combined_clear {
        return HowToClear::CombinedClear;
    }
    HowToClear::JustClear
}

pub fn canvas_native_webgl_clear(mask: u32, state: &mut WebGLState) {
    state.make_current();
    if clear_if_composited(mask, state) != HowToClear::CombinedClear {
        unsafe { gl_bindings::glClear(mask) }
    }
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
    unsafe { gl_bindings::glClearColor(red, green, blue, alpha) }
}

pub fn canvas_native_webgl_clear_depth(depth: f32, state: &mut WebGLState) {
    state.make_current();
    state.set_clear_depth(depth);
    unsafe { gl_bindings::glClearDepthf(depth) }
}

pub fn canvas_native_webgl_clear_stencil(stencil: i32, state: &mut WebGLState) {
    state.make_current();
    state.set_clear_stencil(stencil);
    unsafe { gl_bindings::glClearStencil(stencil) }
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
    unsafe { gl_bindings::glColorMask(red as u8, green as u8, blue as u8, alpha as u8) }
}

pub fn canvas_native_webgl_commit(_: &mut WebGLState) {
    // noop
}

pub fn canvas_native_webgl_compile_shader(shader: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glCompileShader(shader) }
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
        gl_bindings::glCompressedTexImage2D(
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
        gl_bindings::glCompressedTexImage2D(
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
        gl_bindings::glCompressedTexSubImage2D(
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
        gl_bindings::glCopyTexImage2D(target, level, internalformat, x, y, width, height, border)
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
    unsafe {
        gl_bindings::glCopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height)
    }
}

pub fn canvas_native_webgl_create_buffer(state: &mut WebGLState) -> u32 {
    state.make_current();
    let mut buffers = [0u32; 1];
    unsafe {
        gl_bindings::glGenBuffers(1, buffers.as_mut_ptr());
    }
    buffers[0]
}

pub fn canvas_native_webgl_create_framebuffer(state: &mut WebGLState) -> u32 {
    state.make_current();
    let mut frame_buffers = [0u32; 1];
    unsafe { gl_bindings::glGenFramebuffers(1, frame_buffers.as_mut_ptr()) }
    frame_buffers[0]
}

pub fn canvas_native_webgl_create_program(state: &mut WebGLState) -> u32 {
    state.make_current();
    unsafe { gl_bindings::glCreateProgram() }
}

pub fn canvas_native_webgl_create_renderbuffer(state: &mut WebGLState) -> u32 {
    state.make_current();
    let mut render_buffers = [0u32; 1];
    unsafe { gl_bindings::glGenRenderbuffers(1, render_buffers.as_mut_ptr()) }
    render_buffers[0]
}

pub fn canvas_native_webgl_create_shader(shader_type: u32, state: &mut WebGLState) -> u32 {
    state.make_current();
    return unsafe { gl_bindings::glCreateShader(shader_type) };
}

pub fn canvas_native_webgl_create_texture(state: &mut WebGLState) -> u32 {
    state.make_current();
    let mut textures = [0u32; 1];
    unsafe { gl_bindings::glGenTextures(1, textures.as_mut_ptr()) }
    textures[0]
}

pub fn canvas_native_webgl_cull_face(mode: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glCullFace(mode) }
}

pub fn canvas_native_webgl_delete_buffer(buffer: u32, state: &mut WebGLState) {
    state.make_current();
    let mut buf = [buffer];
    unsafe { gl_bindings::glDeleteBuffers(1, buf.as_ptr()) }
}

pub fn canvas_native_webgl_delete_framebuffer(frame_buffer: u32, state: &mut WebGLState) {
    state.make_current();
    let buf = [frame_buffer];
    unsafe { gl_bindings::glDeleteFramebuffers(1, buf.as_ptr()) }
}

pub fn canvas_native_webgl_delete_program(program: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glDeleteProgram(program) }
}

pub fn canvas_native_webgl_delete_renderbuffer(render_buffer: u32, state: &mut WebGLState) {
    state.make_current();
    let buf = [render_buffer];
    unsafe { gl_bindings::glDeleteRenderbuffers(1, buf.as_ptr()) }
}

pub fn canvas_native_webgl_delete_shader(shader: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glDeleteShader(shader) }
}

pub fn canvas_native_webgl_delete_texture(texture: u32, state: &mut WebGLState) {
    state.make_current();
    let buf = [texture];
    unsafe { gl_bindings::glDeleteTextures(1, buf.as_ptr()) }
}

pub fn canvas_native_webgl_depth_func(func: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glDepthFunc(func) }
}

pub fn canvas_native_webgl_depth_mask(flag: bool, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glDepthMask(flag as u8) }
}

pub fn canvas_native_webgl_depth_range(z_near: f32, z_far: f32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glDepthRangef(z_near, z_far) }
}
pub fn canvas_native_webgl_detach_shader(program: u32, shader: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glDetachShader(program, shader) }
}

pub fn canvas_native_webgl_disable(cap: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glDisable(cap) }
}

pub fn canvas_native_webgl_disable_vertex_attrib_array(index: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glDisableVertexAttribArray(index) }
}

pub fn canvas_native_webgl_draw_arrays(mode: u32, first: i32, count: i32, state: &mut WebGLState) {
    state.make_current();
    clear_if_composited(0, state);
    unsafe { gl_bindings::glDrawArrays(mode, first, count) }
    // Flush Context
}

pub fn canvas_native_webgl_draw_elements(
    mode: u32,
    count: i32,
    element_type: u32,
    offset: isize,
    state: &mut WebGLState,
) {
    state.make_current();
    clear_if_composited(0, state);
    unsafe { gl_bindings::glDrawElements(mode, count, element_type, offset as *const c_void) }
    // Flush Context
}

pub fn canvas_native_webgl_enable(cap: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glEnable(cap) }
}

pub fn canvas_native_webgl_enable_vertex_attrib_array(index: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glEnableVertexAttribArray(index) }
}

pub fn canvas_native_webgl_finish(state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glFinish() }
}

pub fn canvas_native_webgl_flush(state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glFlush() }
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
        gl_bindings::glFramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer)
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
    unsafe { gl_bindings::glFramebufferTexture2D(target, attachment, textarget, texture, level) }
}

pub fn canvas_native_webgl_front_face(mode: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glFrontFace(mode) }
}

pub fn canvas_native_webgl_generate_mipmap(target: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glGenerateMipmap(target) }
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
        gl_bindings::glGetProgramiv(
            program,
            gl_bindings::GL_ACTIVE_ATTRIBUTE_MAX_LENGTH,
            &mut length,
        )
    }
    let mut name_buffer = vec![0; length as usize];

    unsafe {
        gl_bindings::glGetActiveAttrib(
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
        gl_bindings::glGetProgramiv(
            program,
            gl_bindings::GL_ACTIVE_UNIFORM_MAX_LENGTH,
            &mut length,
        )
    }
    let mut name_buffer = vec![0; length as usize];

    unsafe {
        gl_bindings::glGetActiveUniform(
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
    unsafe { gl_bindings::glGetProgramiv(program, gl_bindings::GL_ATTACHED_SHADERS, &mut count) }
    let mut shaders = vec![0u32; count as usize];
    let mut len = 0;
    unsafe { gl_bindings::glGetAttachedShaders(program, count, &mut len, shaders.as_mut_ptr()) }
    shaders
}

pub fn canvas_native_webgl_get_attrib_location(
    program: u32,
    name: &str,
    state: &mut WebGLState,
) -> i32 {
    state.make_current();
    let name = CString::new(name).unwrap();
    unsafe { gl_bindings::glGetAttribLocation(program, name.as_ptr()) }
}

pub fn canvas_native_webgl_get_buffer_parameter(
    target: u32,
    pname: u32,
    state: &mut WebGLState,
) -> i32 {
    state.make_current();
    let mut params = 0i32;
    unsafe { gl_bindings::glGetBufferParameteriv(target, pname, &mut params) }
    params
}

pub fn canvas_native_webgl_get_context_attributes(state: &WebGLState) -> ContextAttributes {
    state.get_context_attributes()
}

pub fn canvas_native_webgl_get_error(state: &mut WebGLState) -> u32 {
    state.make_current();
    unsafe { gl_bindings::glGetError() }
}
pub fn canvas_native_webgl_get_extension(
    name: &str,
    state: &mut WebGLState,
) -> Option<Box<dyn WebGLExtension>> {
    state.make_current();
    let version = state.get_webgl_version();
    let extensions = unsafe { gl_bindings::glGetString(gl_bindings::GL_EXTENSIONS) };
    if extensions.is_null() {
        return None;
    }
    // API_LEVEL
    #[allow(non_snake_case)]
    let JELLY_BEAN_MR2 = 18;

    let ext = unsafe { CStr::from_ptr(std::mem::transmute(extensions)) };
    let extensions = ext.to_string_lossy();

    let extension = if name.eq("EXT_blend_minmax") {
        if version == WebGLVersion::V2 || extensions.contains("GL_EXT_blend_minmax") {
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
    } else if name.eq("WEBGL_color_buffer_float")
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

pub fn canvas_native_webgl_get_framebuffer_attachment_parameter(
    target: u32,
    attachment: u32,
    pname: u32,
    state: &mut WebGLState,
) -> WebGLFramebufferAttachmentParameter {
    state.make_current();
    let mut params = 0i32;
    let mut result = WebGLFramebufferAttachmentParameter::default();
    if attachment == gl_bindings::GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME {
        unsafe {
            gl_bindings::glGetFramebufferAttachmentParameteriv(
                target,
                attachment,
                pname,
                &mut params,
            )
        };
        let mut name = 0i32;
        unsafe {
            gl_bindings::glGetFramebufferAttachmentParameteriv(
                target,
                attachment,
                gl_bindings::GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE,
                &mut name,
            )
        };
        match name as u32 {
            gl_bindings::GL_RENDERBUFFER => {
                result.is_renderbuffer = true;
                result.value = params
            }
            gl_bindings::GL_TEXTURE => {
                result.is_texture = true;
                result.value = params
            }
            _ => result.value = params,
        }
    } else {
        unsafe {
            gl_bindings::glGetFramebufferAttachmentParameteriv(
                target,
                attachment,
                pname,
                &mut params,
            )
        };
        result.value = params;
    }

    result
}

pub fn canvas_native_webgl_get_parameter(pname: u32, state: &mut WebGLState) -> WebGLResult {
    state.make_current();
    console_log_rust(&format!("canvas_native_webgl_get_parameter: {pname} {:?} {:?}",gl_bindings::GL_SCISSOR_BOX, gl_bindings::GL_VIEWPORT));
    match pname {
        gl_bindings::GL_ACTIVE_TEXTURE
        | gl_bindings::GL_ALPHA_BITS
        | gl_bindings::GL_ARRAY_BUFFER_BINDING
        | gl_bindings::GL_BLEND_DST_ALPHA
        | gl_bindings::GL_BLEND_DST_RGB
        | gl_bindings::GL_BLEND_EQUATION
        | gl_bindings::GL_BLEND_EQUATION_ALPHA
        | gl_bindings::GL_BLEND_EQUATION_RGB
        | gl_bindings::GL_BLEND_SRC_ALPHA
        | gl_bindings::GL_BLEND_SRC_RGB
        | gl_bindings::GL_BLUE_BITS
        | gl_bindings::GL_CULL_FACE_MODE
        | gl_bindings::GL_CURRENT_PROGRAM
        | gl_bindings::GL_DEPTH_BITS
        | gl_bindings::GL_DEPTH_FUNC
        | gl_bindings::GL_ELEMENT_ARRAY_BUFFER_BINDING
        | gl_bindings::GL_FRAMEBUFFER_BINDING
        | gl_bindings::GL_FRONT_FACE
        | gl_bindings::GL_GENERATE_MIPMAP_HINT
        | gl_bindings::GL_GREEN_BITS
        | gl_bindings::GL_IMPLEMENTATION_COLOR_READ_FORMAT
        | gl_bindings::GL_IMPLEMENTATION_COLOR_READ_TYPE
        | gl_bindings::GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS
        | gl_bindings::GL_MAX_CUBE_MAP_TEXTURE_SIZE
        | gl_bindings::GL_MAX_FRAGMENT_UNIFORM_VECTORS
        | gl_bindings::GL_MAX_RENDERBUFFER_SIZE
        | gl_bindings::GL_MAX_TEXTURE_IMAGE_UNITS
        | gl_bindings::GL_MAX_TEXTURE_SIZE
        | gl_bindings::GL_MAX_VARYING_VECTORS
        | gl_bindings::GL_MAX_VERTEX_ATTRIBS
        | gl_bindings::GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS
        | gl_bindings::GL_MAX_VERTEX_UNIFORM_VECTORS
        | gl_bindings::GL_PACK_ALIGNMENT
        | gl_bindings::GL_RED_BITS
        | gl_bindings::GL_RENDERBUFFER_BINDING
        | gl_bindings::GL_SAMPLE_BUFFERS
        | gl_bindings::GL_SAMPLES
        | gl_bindings::GL_STENCIL_BACK_FAIL
        | gl_bindings::GL_STENCIL_BACK_FUNC
        | gl_bindings::GL_STENCIL_BACK_PASS_DEPTH_FAIL
        | gl_bindings::GL_STENCIL_BACK_PASS_DEPTH_PASS
        | gl_bindings::GL_STENCIL_BACK_REF
        | gl_bindings::GL_STENCIL_BACK_VALUE_MASK
        | gl_bindings::GL_STENCIL_BACK_WRITEMASK
        | gl_bindings::GL_STENCIL_BITS
        | gl_bindings::GL_STENCIL_CLEAR_VALUE
        | gl_bindings::GL_STENCIL_FAIL
        | gl_bindings::GL_STENCIL_FUNC
        | gl_bindings::GL_STENCIL_PASS_DEPTH_FAIL
        | gl_bindings::GL_STENCIL_PASS_DEPTH_PASS
        | gl_bindings::GL_STENCIL_REF
        | gl_bindings::GL_STENCIL_VALUE_MASK
        | gl_bindings::GL_STENCIL_WRITEMASK
        | gl_bindings::GL_SUBPIXEL_BITS
        | gl_bindings::GL_TEXTURE_BINDING_2D
        | gl_bindings::GL_TEXTURE_BINDING_CUBE_MAP
        | gl_bindings::GL_UNPACK_ALIGNMENT => {
            let mut param = 0i32;
            unsafe { gl_bindings::glGetIntegerv(pname, &mut param) };
            if (pname == gl_bindings::GL_TEXTURE_BINDING_2D
                || pname == gl_bindings::GL_TEXTURE_BINDING_CUBE_MAP
                || pname == gl_bindings::GL_RENDERBUFFER_BINDING
                || pname == gl_bindings::GL_FRAMEBUFFER_BINDING)
                && param == 0
            {
                return WebGLResult::None;
            }
            return WebGLResult::I32(param);
        }
        gl_bindings::GL_ALIASED_LINE_WIDTH_RANGE
        | gl_bindings::GL_ALIASED_POINT_SIZE_RANGE
        | gl_bindings::GL_DEPTH_RANGE => {
            let mut param: Vec<f32> = Vec::with_capacity(2);
            unsafe { gl_bindings::glGetFloatv(pname, param.as_mut_ptr()) }
            return WebGLResult::F32Array(param);
        }
        gl_bindings::GL_BLEND_COLOR | gl_bindings::GL_COLOR_CLEAR_VALUE => {
            let mut param: Vec<f32> = Vec::with_capacity(4);
            unsafe { gl_bindings::glGetFloatv(pname, param.as_mut_ptr()) };
            return WebGLResult::F32Array(param);
        }
        gl_bindings::GL_BLEND
        | gl_bindings::GL_CULL_FACE
        | gl_bindings::GL_DEPTH_TEST
        | gl_bindings::GL_DEPTH_WRITEMASK
        | gl_bindings::GL_DITHER
        | gl_bindings::GL_POLYGON_OFFSET_FILL
        | gl_bindings::GL_SAMPLE_COVERAGE_INVERT
        | gl_bindings::GL_SCISSOR_TEST
        | gl_bindings::GL_STENCIL_TEST => {
            let mut param = false;
            unsafe { gl_bindings::glGetBooleanv(pname, &mut (param as u8)) };
            return WebGLResult::Boolean(param);
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
        gl_bindings::GL_COLOR_WRITEMASK => {
            let mut param: Vec<bool> = Vec::with_capacity(4);
            unsafe { gl_bindings::glGetBooleanv(pname, param.as_mut_ptr() as *mut c_uchar) };
            return WebGLResult::BooleanArray(param);
        }
        gl_bindings::GL_COMPRESSED_TEXTURE_FORMATS => {
            let mut count = 0i32;
            unsafe {
                gl_bindings::glGetIntegerv(
                    gl_bindings::GL_NUM_COMPRESSED_TEXTURE_FORMATS,
                    &mut count,
                )
            }
            let mut params: Vec<i32> = Vec::with_capacity(count as usize);
            unsafe {
                gl_bindings::glGetIntegerv(
                    gl_bindings::GL_COMPRESSED_TEXTURE_FORMATS,
                    params.as_mut_ptr(),
                )
            }
            return WebGLResult::I32Array(params);
        }
        gl_bindings::GL_DEPTH_CLEAR_VALUE
        | gl_bindings::GL_LINE_WIDTH
        | gl_bindings::GL_POLYGON_OFFSET_FACTOR
        | gl_bindings::GL_POLYGON_OFFSET_UNITS
        | gl_bindings::GL_SAMPLE_COVERAGE_VALUE => {
            let mut param = 0f32;
            unsafe { gl_bindings::glGetFloatv(pname, &mut param) }
            return WebGLResult::F32(param);
        }
        gl_bindings::GL_MAX_VIEWPORT_DIMS => {
            let mut params: Vec<i32> = Vec::with_capacity(2);
            unsafe { gl_bindings::glGetIntegerv(pname, params.as_mut_ptr()) }
            return WebGLResult::I32Array(params);
        }
        gl_bindings::GL_SCISSOR_BOX | gl_bindings::GL_VIEWPORT => {
            let mut params: Vec<i32> = Vec::with_capacity(4);
            unsafe { gl_bindings::glGetIntegerv(pname, params.as_mut_ptr()) }
            return WebGLResult::I32Array(params);
        }
        gl_bindings::GL_RENDERER
        | gl_bindings::GL_SHADING_LANGUAGE_VERSION
        | gl_bindings::GL_VENDOR
        | gl_bindings::GL_VERSION => {
            let params = unsafe { gl_bindings::glGetString(pname) };
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
    }
}

pub fn canvas_native_webgl_get_program_info_log(program: u32, state: &mut WebGLState) -> String {
    state.make_current();
    let mut length = 0i32;
    unsafe { gl_bindings::glGetProgramiv(program, gl_bindings::GL_INFO_LOG_LENGTH, &mut length) }
    let mut info = vec![0; length as usize];
    let mut len = 0;
    unsafe {
        gl_bindings::glGetProgramInfoLog(
            program,
            length,
            &mut len,
            info.as_mut_ptr() as *mut c_char,
        )
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
    unsafe { gl_bindings::glGetProgramiv(program, pname, &mut param) }
    match pname {
        gl_bindings::GL_DELETE_STATUS
        | gl_bindings::GL_LINK_STATUS
        | gl_bindings::GL_VALIDATE_STATUS => {
            if param as u32 == gl_bindings::GL_TRUE {
                return WebGLResult::Boolean(true);
            }
            return WebGLResult::Boolean(false);
        }
        gl_bindings::GL_ATTACHED_SHADERS
        | gl_bindings::GL_ACTIVE_ATTRIBUTES
        | gl_bindings::GL_ACTIVE_UNIFORMS => {
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
    unsafe { gl_bindings::glGetRenderbufferParameteriv(target, pname, &mut params) }
    params
}

pub fn canvas_native_webgl_get_shader_info_log(shader: u32, state: &mut WebGLState) -> String {
    state.make_current();
    let mut length = 0i32;
    unsafe { gl_bindings::glGetShaderiv(shader, gl_bindings::GL_INFO_LOG_LENGTH, &mut length) }
    let mut log = vec![0; length as usize];
    let mut len = 0;
    unsafe {
        gl_bindings::glGetShaderInfoLog(shader, length, &mut len, log.as_mut_ptr() as *mut c_char)
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
    unsafe { gl_bindings::glGetShaderiv(shader, pname, &mut params) }
    return match pname {
        gl_bindings::GL_DELETE_STATUS | gl_bindings::GL_COMPILE_STATUS => {
            if params as u32 == gl_bindings::GL_TRUE {
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
        gl_bindings::glGetShaderPrecisionFormat(
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
    unsafe { gl_bindings::glGetShaderiv(shader, gl_bindings::GL_SHADER_SOURCE_LENGTH, &mut length) }
    let mut source = vec![0; length as usize];
    let mut len = 0;
    unsafe {
        gl_bindings::glGetShaderSource(shader, length, &mut len, source.as_mut_ptr() as *mut c_char)
    }
    source.shrink_to(len.try_into().unwrap());
    let c_str = unsafe { CStr::from_ptr(source.as_ptr()) };
    c_str.to_string_lossy().to_string()
}

pub fn canvas_native_webgl_get_supported_extensions(state: &mut WebGLState) -> Vec<String> {
    state.make_current();
    let ext = unsafe { gl_bindings::glGetString(gl_bindings::GL_EXTENSIONS) as *const c_char};
    if ext.is_null() {
        return Vec::with_capacity(0);
    }
    let version = unsafe { String::from_utf8(CStr::from_ptr(ext).to_bytes().to_vec()).unwrap()};

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
    unsafe { gl_bindings::glGetTexParameteriv(target, pname, &mut params) }
    return params;
}

pub fn canvas_native_webgl_get_uniform_location(
    program: u32,
    name: &str,
    state: &mut WebGLState,
) -> i32 {
    state.make_current();
    let name = CString::new(name).unwrap();
    return unsafe { gl_bindings::glGetUniformLocation(program, name.as_ptr()) };
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
        gl_bindings::glGetActiveUniform(
            program,
            location as u32,
            0,
            &mut length,
            &mut size,
            &mut uniform_type,
            std::ptr::null_mut(),
        )
    }
    match uniform_type {
        gl_bindings::GL_FLOAT => {
            let mut single = [0f32; 1];
            unsafe { gl_bindings::glGetUniformfv(program, location, single.as_mut_ptr()) }
            return WebGLResult::F32(single[0]);
        }
        gl_bindings::GL_FLOAT_VEC2 => {
            let mut vec2: Vec<f32> = Vec::with_capacity(2);
            unsafe { gl_bindings::glGetUniformfv(program, location, vec2.as_mut_ptr()) }
            return WebGLResult::F32Array(vec2);
        }
        gl_bindings::GL_FLOAT_VEC3 => {
            let mut vec3: Vec<f32> = Vec::with_capacity(3);
            unsafe { gl_bindings::glGetUniformfv(program, location, vec3.as_mut_ptr()) }
            return WebGLResult::F32Array(vec3);
        }
        gl_bindings::GL_FLOAT_VEC4 => {
            let mut vec4: Vec<f32> = Vec::with_capacity(4);
            unsafe { gl_bindings::glGetUniformfv(program, location, vec4.as_mut_ptr()) }
            return WebGLResult::F32Array(vec4);
        }
        gl_bindings::GL_INT | gl_bindings::GL_SAMPLER_2D | gl_bindings::GL_SAMPLER_CUBE => {
            let mut single_int = [0i32; 1];
            unsafe {
                gl_bindings::glGetUniformiv(program, location, single_int.as_mut_ptr());
            }
            return WebGLResult::I32(single_int[0]);
        }
        gl_bindings::GL_INT_VEC2 => {
            let mut int_vec2: Vec<i32> = Vec::with_capacity(2);
            unsafe { gl_bindings::glGetUniformiv(program, location, int_vec2.as_mut_ptr()) }
            return WebGLResult::I32Array(int_vec2);
        }
        gl_bindings::GL_INT_VEC3 => {
            let mut int_vec3: Vec<i32> = Vec::with_capacity(3);
            unsafe { gl_bindings::glGetUniformiv(program, location, int_vec3.as_mut_ptr()) }
            return WebGLResult::I32Array(int_vec3);
        }
        gl_bindings::GL_INT_VEC4 => {
            let mut int_vec4: Vec<i32> = Vec::with_capacity(4);
            unsafe { gl_bindings::glGetUniformiv(program, location, int_vec4.as_mut_ptr()) }
            return WebGLResult::I32Array(int_vec4);
        }
        gl_bindings::GL_BOOL => {
            let mut single_bool = [0i32; 1];
            unsafe { gl_bindings::glGetUniformiv(program, location, single_bool.as_mut_ptr()) }
            return WebGLResult::Boolean(single_bool[0] == 1);
        }
        gl_bindings::GL_BOOL_VEC2 => {
            let mut bool_vec2 = [0i32; 2];
            unsafe { gl_bindings::glGetUniformiv(program, location, bool_vec2.as_mut_ptr()) }
            return WebGLResult::BooleanArray(unsafe {
                std::slice::from_raw_parts(bool_vec2.as_ptr() as *const bool, 2).to_vec()
            });
        }
        gl_bindings::GL_BOOL_VEC3 => {
            let mut bool_vec3 = [0i32; 3];
            unsafe { gl_bindings::glGetUniformiv(program, location, bool_vec3.as_mut_ptr()) }
            return WebGLResult::BooleanArray(unsafe {
                std::slice::from_raw_parts(bool_vec3.as_ptr() as *const bool, 3).to_vec()
            });
        }
        gl_bindings::GL_BOOL_VEC4 => {
            let mut bool_vec4 = [0i32; 4];
            unsafe { gl_bindings::glGetUniformiv(program, location, bool_vec4.as_mut_ptr()) }
            return WebGLResult::BooleanArray(unsafe {
                std::slice::from_raw_parts(bool_vec4.as_ptr() as *const bool, 4).to_vec()
            });
        }
        gl_bindings::GL_FLOAT_MAT2 => {
            let mut mat2: Vec<f32> = Vec::with_capacity(2);
            unsafe {
                gl_bindings::glGetUniformfv(program, location, mat2.as_mut_ptr());
            }
            return WebGLResult::F32Array(mat2);
        }
        gl_bindings::GL_FLOAT_MAT3 => {
            let mut mat3: Vec<f32> = Vec::with_capacity(3);
            unsafe {
                gl_bindings::glGetUniformfv(program, location, mat3.as_mut_ptr());
            }
            return WebGLResult::F32Array(mat3);
        }
        gl_bindings::GL_FLOAT_MAT4 => {
            let mut mat4: Vec<f32> = Vec::with_capacity(4);
            unsafe {
                gl_bindings::glGetUniformfv(program, location, mat4.as_mut_ptr());
            }
            return WebGLResult::F32Array(mat4);
        }
        _ => WebGLResult::None,
    }
}

pub fn canvas_native_webgl_get_vertex_attrib_offset(
    index: u32,
    pname: u32,
    state: &mut WebGLState,
) -> size_t {
    state.make_current();
    let mut ptr = [0 as size_t; 1];
    let mut ptr_ptr = &mut (ptr.as_mut_ptr() as *mut c_void);
    unsafe { gl_bindings::glGetVertexAttribPointerv(index, pname, ptr_ptr) }
    ptr[0]
}

pub fn canvas_native_webgl_get_vertex_attrib(
    index: u32,
    pname: u32,
    state: &mut WebGLState,
) -> WebGLResult {
    state.make_current();
    if pname == gl_bindings::GL_CURRENT_VERTEX_ATTRIB {
        let mut params: Vec<f32> = Vec::with_capacity(4);
        unsafe { gl_bindings::glGetVertexAttribfv(index, pname, params.as_mut_ptr()) }
        return WebGLResult::F32Array(params);
    }
    let mut params = 0i32;
    unsafe { gl_bindings::glGetVertexAttribiv(index, pname, &mut params) }
    match pname {
        gl_bindings::GL_VERTEX_ATTRIB_ARRAY_ENABLED
        | gl_bindings::GL_VERTEX_ATTRIB_ARRAY_NORMALIZED
        | gl_bindings::GL_VERTEX_ATTRIB_ARRAY_INTEGER => return WebGLResult::Boolean(params == 1),
        gl_bindings::GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING => WebGLResult::U32(params as u32),
        gl_bindings::GL_VERTEX_ATTRIB_ARRAY_SIZE
        | gl_bindings::GL_VERTEX_ATTRIB_ARRAY_STRIDE
        | gl_bindings::GL_VERTEX_ATTRIB_ARRAY_DIVISOR => return WebGLResult::I32(params),
        _ => return WebGLResult::None,
    }
}

pub fn canvas_native_webgl_get_is_context_lost(state: &mut WebGLState) -> bool {
    // TODO improve
    false
}

pub fn canvas_native_webgl_hint(target: u32, mode: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glHint(target, mode) }
}

pub fn canvas_native_webgl_is_buffer(buffer: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    unsafe { gl_bindings::glIsBuffer(buffer) == 1 }
}

pub fn canvas_native_webgl_is_enabled(cap: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    unsafe { gl_bindings::glIsEnabled(cap) == 1 }
}

pub fn canvas_native_webgl_is_framebuffer(framebuffer: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    unsafe { gl_bindings::glIsFramebuffer(framebuffer) == 1 }
}

pub fn canvas_native_webgl_is_program(program: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    unsafe { gl_bindings::glIsProgram(program) == 1 }
}

pub fn canvas_native_webgl_is_renderbuffer(renderbuffer: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    unsafe { gl_bindings::glIsRenderbuffer(renderbuffer) == 1 }
}

pub fn canvas_native_webgl_is_shader(shader: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    unsafe { gl_bindings::glIsShader(shader) == 1 }
}

pub fn canvas_native_webgl_is_texture(texture: u32, state: &mut WebGLState) -> bool {
    state.make_current();
    unsafe { gl_bindings::glIsTexture(texture) == 1 }
}

pub fn canvas_native_webgl_line_width(width: f32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glLineWidth(width) }
}

pub fn canvas_native_webgl_link_program(program: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glLinkProgram(program) }
}

pub fn canvas_native_webgl_pixel_storei(pname: u32, param: i32, state: &mut WebGLState) {
    match pname {
        gl_bindings::GL_UNPACK_ALIGNMENT | gl_bindings::GL_PACK_ALIGNMENT => unsafe {
            gl_bindings::glPixelStorei(pname, param)
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
    unsafe { gl_bindings::glPolygonOffset(factor, units) }
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
        gl_bindings::glReadPixels(
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
        gl_bindings::glReadPixels(
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
        gl_bindings::glReadPixels(
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
    unsafe { gl_bindings::glRenderbufferStorage(target, internal_format, width, height) }
}

pub fn canvas_native_webgl_sample_coverage(value: f32, invert: bool, state: &mut WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glSampleCoverage(value, invert as u8) }
}

pub fn canvas_native_webgl_scissor(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe { gl_bindings::glScissor(x, y, width, height) }
}

pub fn canvas_native_webgl_shader_source(shader: u32, source: &str, state: &mut WebGLState) {
    state.make_current();
    let src = CString::new(source).unwrap();
    unsafe { gl_bindings::glShaderSource(shader, 1, &src.as_ptr(), std::ptr::null()) }
}

pub fn canvas_native_webgl_stencil_func(
    func: u32,
    reference: i32,
    mask: u32,
    state: &mut WebGLState,
) {
    state.make_current();
    unsafe { gl_bindings::glStencilFunc(func, reference, mask) }
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
        gl_bindings::GL_FRONT_AND_BACK => {
            state.set_stencil_func_ref(reference);
            state.set_stencil_func_ref_back(reference);
            state.set_stencil_func_mask(mask);
            state.set_stencil_func_mask_back(mask);
        }
        gl_bindings::GL_FRONT => {
            state.set_stencil_func_ref(reference);
            state.set_stencil_func_mask(mask);
        }
        gl_bindings::GL_BACK => {
            state.set_stencil_func_ref_back(reference);
            state.set_stencil_func_mask_back(mask);
        }
        _ => {}
    }
    unsafe { gl_bindings::glStencilFuncSeparate(face, func, reference, mask) }
}

pub fn canvas_native_webgl_stencil_mask(mask: u32, state: &mut WebGLState) {
    state.make_current();
    state.set_stencil_mask(mask);
    state.set_stencil_mask_back(mask);
    unsafe { gl_bindings::glStencilMask(mask) }
}

pub fn canvas_native_webgl_stencil_mask_separate(face: u32, mask: u32, state: &mut WebGLState) {
    state.make_current();
    match face {
        gl_bindings::GL_FRONT_AND_BACK => {
            state.set_stencil_mask(mask);
            state.set_stencil_mask_back(mask);
        }
        gl_bindings::GL_FRONT => {
            state.set_stencil_mask(mask);
        }
        gl_bindings::GL_BACK => {
            state.set_stencil_mask_back(mask);
        }
        _ => {}
    }
    unsafe { gl_bindings::glStencilMaskSeparate(face, mask) }
}

pub fn canvas_native_webgl_stencil_op(fail: u32, zfail: u32, zpass: u32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glStencilOp(fail, zfail, zpass) }
}

pub fn canvas_native_webgl_stencil_op_separate(
    face: u32,
    fail: u32,
    zfail: u32,
    zpass: u32,
    state: &WebGLState,
) {
    state.make_current();
    unsafe { gl_bindings::glStencilOpSeparate(face, fail, zfail, zpass) }
}

pub fn canvas_native_webgl_tex_image2d_asset(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    asset: &crate::ImageAsset,
    state: &WebGLState,
) {
    if let Some(bytes) = asset.get_bytes() {
        let width = asset.width() as i32;
        let height = asset.height() as i32;
        state.make_current();
        unsafe {
            if state.get_flip_y() {
                let mut buffer = bytes.to_vec();
                canvas_core::utils::gl::flip_in_place(
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    (canvas_core::utils::gl::bytes_per_pixel(image_type as u32, format as u32)
                        as i32
                        * width) as usize,
                    height as usize,
                );

                gl_bindings::glTexImage2D(
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

            gl_bindings::glTexImage2D(
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
    let mut width = source.get_drawing_buffer_width();
    let mut height = source.get_drawing_buffer_height();

    let mut buf = vec![0u8; (width * height * 4) as usize];

    unsafe {
        gl_bindings::glFinish();
        gl_bindings::glReadPixels(
            0,
            0,
            width,
            height,
            gl_bindings::GL_RGBA,
            gl_bindings::GL_UNSIGNED_BYTE,
            buf.as_mut_ptr() as *mut c_void,
        );
    }

    source.remove_if_current();

    context.make_current();

    (width as i32, height as i32, buf)
}

pub fn canvas_native_webgl_read_canvas2d_pixels(
    source: &mut crate::CanvasRenderingContext2D,
    context: &mut WebGLState,
) -> (i32, i32, Vec<u8>) {
    context.remove_if_current();
    let mut width = 0f32;
    let mut height = 0f32;
    let mut source_non_gpu = false;
    {
        let ctx = source.get_context();
        let device = ctx.device();
        width = device.width;
        height = device.height;
        source_non_gpu = device.non_gpu;
    }

    let mut source_ctx = source.get_context_mut();
    let mut buf;
    if !source_non_gpu {
        source.gl_context.make_current();
        buf = vec![0u8; (width as i32 * height as i32 * 4) as usize];
        unsafe {
            gl_bindings::glFinish();
            gl_bindings::glReadPixels(
                0,
                0,
                width as i32,
                height as i32,
                gl_bindings::GL_RGBA,
                gl_bindings::GL_UNSIGNED_BYTE,
                buf.as_mut_ptr() as *mut c_void,
            );
        }
        source.gl_context.remove_if_current();
    } else {
        buf = source_ctx.read_pixels();
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
        gl_bindings::glTexImage2D(
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
    buf: &mut [u8],
    state: &WebGLState,
) {
    state.make_current();
    unsafe {
        if state.get_flip_y() {
            let mut buffer = buf.to_vec();
            canvas_core::utils::gl::flip_in_place(
                buffer.as_mut_ptr(),
                buffer.len(),
                (canvas_core::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32
                    * width) as usize,
                height as usize,
            );

            gl_bindings::glTexImage2D(
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

        gl_bindings::glTexImage2D(
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
        gl_bindings::glTexImage2D(
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
    internalformat: i32,
    format: i32,
    image_type: i32,
    image_asset: &crate::ImageAsset,
    state: &WebGLState,
) {
    state.make_current();
    let bytes = image_asset.get_bytes().unwrap_or(&[0u8]);
    unsafe {
        gl_bindings::glTexImage2D(
            target as u32,
            level,
            internalformat,
            0,
            format,
            image_type,
            gl_bindings::GL_RGBA as u32,
            image_type as u32,
            bytes.as_ptr() as *const c_void,
        );
    }
}

pub fn canvas_native_webgl_tex_parameterf(target: u32, pname: u32, param: f32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glTexParameterf(target, pname, param) }
}

pub fn canvas_native_webgl_tex_parameteri(target: u32, pname: u32, param: i32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glTexParameteri(target, pname, param) }
}

pub fn canvas_native_webgl_tex_sub_image2d_asset(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    format: u32,
    image_type: i32,
    asset: &crate::ImageAsset,
    state: &WebGLState,
) {
    if let Some(bytes) = asset.get_bytes() {
        let width = asset.width() as i32;
        let height = asset.height() as i32;
        state.make_current();
        if state.get_flip_y() {
            let mut buffer = bytes.to_vec();
            canvas_core::utils::gl::flip_in_place(
                buffer.as_mut_ptr(),
                bytes.len(),
                (canvas_core::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32
                    * width as i32) as usize,
                height as usize,
            );

            unsafe {
                gl_bindings::glTexSubImage2D(
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
            gl_bindings::glTexSubImage2D(
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
        canvas_core::utils::gl::flip_in_place(
            buffer.as_mut_ptr(),
            buf.len(),
            (canvas_core::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32
                * width as i32) as usize,
            height as usize,
        );

        unsafe {
            gl_bindings::glTexSubImage2D(
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
        gl_bindings::glTexSubImage2D(
            target as u32,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format as u32,
            image_type as u32,
            buf.as_ptr() as *const c_void,
        );
    }
}

pub fn canvas_native_webgl_uniform1f(location: i32, v0: f32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glUniform1f(location, v0) }
}

pub fn canvas_native_webgl_uniform1fv(location: i32, value: &[f32], state: &WebGLState) {
    state.make_current();
    unsafe {
        let count = value.len();
        gl_bindings::glUniform1fv(location, count.try_into().unwrap(), value.as_ptr())
    }
}

pub fn canvas_native_webgl_uniform1i(location: i32, v0: i32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glUniform1i(location, v0) }
}

pub fn canvas_native_webgl_uniform1iv(location: i32, value: &[i32], state: &WebGLState) {
    state.make_current();
    unsafe {
        let count = value.len();
        gl_bindings::glUniform1iv(location, count.try_into().unwrap(), value.as_ptr())
    }
}

pub fn canvas_native_webgl_uniform2f(location: i32, v0: f32, v1: f32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glUniform2f(location, v0, v1) }
}

pub fn canvas_native_webgl_uniform2fv(location: i32, value: &[f32], state: &WebGLState) {
    state.make_current();
    unsafe {
        let count = value.len() / 2;
        gl_bindings::glUniform2fv(location, count.try_into().unwrap(), value.as_ptr())
    }
}

pub fn canvas_native_webgl_uniform2i(location: i32, v0: i32, v1: i32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glUniform2i(location, v0, v1) }
}

pub fn canvas_native_webgl_uniform2iv(location: i32, value: &[i32], state: &WebGLState) {
    state.make_current();
    unsafe {
        let count = value.len() / 2;
        gl_bindings::glUniform2iv(location, count.try_into().unwrap(), value.as_ptr())
    }
}

pub fn canvas_native_webgl_uniform3f(location: i32, v0: f32, v1: f32, v2: f32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glUniform3f(location, v0, v1, v2) }
}

pub fn canvas_native_webgl_uniform3fv(location: i32, value: &[f32], state: &WebGLState) {
    state.make_current();
    unsafe {
        let count = value.len() / 3;
        gl_bindings::glUniform3fv(location, count.try_into().unwrap(), value.as_ptr())
    }
}

pub fn canvas_native_webgl_uniform3i(location: i32, v0: i32, v1: i32, v2: i32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glUniform3i(location, v0, v1, v2) }
}

pub fn canvas_native_webgl_uniform3iv(location: i32, value: &[i32], state: &WebGLState) {
    state.make_current();
    unsafe {
        let count = value.len() / 3;
        gl_bindings::glUniform3iv(location, count.try_into().unwrap(), value.as_ptr())
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
    unsafe { gl_bindings::glUniform4f(location, v0, v1, v2, v3) }
}

pub fn canvas_native_webgl_uniform4fv(location: i32, value: &[f32], state: &WebGLState) {
    state.make_current();
    unsafe {
        let count = value.len() / 4;
        gl_bindings::glUniform4fv(location, count.try_into().unwrap(), value.as_ptr())
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
    unsafe { gl_bindings::glUniform4i(location, v0, v1, v2, v3) }
}

pub fn canvas_native_webgl_uniform4iv(location: i32, value: &[i32], state: &WebGLState) {
    state.make_current();
    unsafe {
        let count = value.len() / 4;
        gl_bindings::glUniform4iv(location, count.try_into().unwrap(), value.as_ptr())
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
        gl_bindings::glUniformMatrix2fv(
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
        gl_bindings::glUniformMatrix3fv(
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
        gl_bindings::glUniformMatrix4fv(
            location,
            count.try_into().unwrap(),
            transpose as u8,
            value.as_ptr(),
        )
    }
}

pub fn canvas_native_webgl_use_program(program: u32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glUseProgram(program) }
}

pub fn canvas_native_webgl_validate_program(program: u32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glValidateProgram(program) }
}

pub fn canvas_native_webgl_vertex_attrib1f(index: u32, v0: f32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glVertexAttrib1f(index, v0) }
}

pub fn canvas_native_webgl_vertex_attrib1fv(index: u32, value: &[f32], state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glVertexAttrib1fv(index, value.as_ptr()) }
}

pub fn canvas_native_webgl_vertex_attrib2f(index: u32, v0: f32, v1: f32, state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glVertexAttrib2f(index, v0, v1) }
}

pub fn canvas_native_webgl_vertex_attrib2fv(index: u32, value: &[f32], state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glVertexAttrib2fv(index, value.as_ptr()) }
}

pub fn canvas_native_webgl_vertex_attrib3f(
    index: u32,
    v0: f32,
    v1: f32,
    v2: f32,
    state: &WebGLState,
) {
    state.make_current();
    unsafe { gl_bindings::glVertexAttrib3f(index, v0, v1, v2) }
}

pub fn canvas_native_webgl_vertex_attrib3fv(index: u32, value: &[f32], state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glVertexAttrib3fv(index, value.as_ptr()) }
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
    unsafe { gl_bindings::glVertexAttrib4f(index, v0, v1, v2, v3) }
}

pub fn canvas_native_webgl_vertex_attrib4fv(index: u32, value: &[f32], state: &WebGLState) {
    state.make_current();
    unsafe { gl_bindings::glVertexAttrib4fv(index, value.as_ptr()) }
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
        gl_bindings::glVertexAttribPointer(
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
    unsafe { gl_bindings::glViewport(x, y, width, height) }
}
