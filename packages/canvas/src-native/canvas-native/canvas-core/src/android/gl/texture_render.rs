use std::os::raw::c_void;

use jni::objects::{JClass, JObject, ReleaseMode};
use jni::sys::{jfloatArray, jint};
use jni::JNIEnv;

const SURFACE_TEXTURE_CLASS: &'static str = "android/graphics/SurfaceTexture";

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TextureRender_nativeDrawFrame(
    env: JNIEnv,
    _: JClass,
    surface_texture: JObject,
    fbo: jint,
    program: jint,
    external_texture: jint,
    array_buffer: jint,
    matrix: jfloatArray,
    matrix_pos: jint,
    width: jint,
    height: jint,
    render_width: jint,
    render_height: jint,
    internal_format: jint,
    format: jint,
    draw_count: jint,
) {
    let _ = env.call_method(surface_texture, "updateTexImage", "()V", &[]);
    let _ = env.call_method(
        surface_texture,
        "getTransformMatrix",
        "([F)V",
        &[matrix.into()],
    );
    if let Ok(matrix) = env.get_primitive_array_critical(matrix, ReleaseMode::CopyBack) {
        let mut previous_view_port = [0_i32; 4];
        let mut previous_active_texture = [-1_i32; 1];
        let mut previous_texture = [-1_i32; 1];
        let mut previous_program = [-1_i32; 1];
        let mut previous_frame_buffer = [-1_i32; 1];

        gl_bindings::glGetIntegerv(gl_bindings::GL_VIEWPORT, previous_view_port.as_mut_ptr());
        gl_bindings::glGetIntegerv(
            gl_bindings::GL_ACTIVE_TEXTURE,
            previous_active_texture.as_mut_ptr(),
        );
        gl_bindings::glGetIntegerv(
            gl_bindings::GL_TEXTURE_BINDING_2D,
            previous_texture.as_mut_ptr(),
        );
        gl_bindings::glGetIntegerv(
            gl_bindings::GL_CURRENT_PROGRAM,
            previous_program.as_mut_ptr(),
        );
        gl_bindings::glGetIntegerv(
            gl_bindings::GL_FRAMEBUFFER_BINDING,
            previous_frame_buffer.as_mut_ptr(),
        );

        gl_bindings::glBindFramebuffer(gl_bindings::GL_FRAMEBUFFER, fbo as u32);

        if render_width != width || render_height != width {
            gl_bindings::glBindTexture(gl_bindings::GL_TEXTURE_2D, previous_texture[0] as u32);

            gl_bindings::glTexImage2D(
                gl_bindings::GL_TEXTURE_2D,
                0,
                internal_format,
                width,
                height,
                0,
                format as u32,
                gl_bindings::GL_UNSIGNED_BYTE,
                0 as *const c_void,
            );
            gl_bindings::glTexParameteri(
                gl_bindings::GL_TEXTURE_2D,
                gl_bindings::GL_TEXTURE_MAG_FILTER,
                gl_bindings::GL_LINEAR as i32,
            );
            gl_bindings::glTexParameteri(
                gl_bindings::GL_TEXTURE_2D,
                gl_bindings::GL_TEXTURE_MIN_FILTER,
                gl_bindings::GL_LINEAR as i32,
            );

            gl_bindings::glTexParameteri(
                gl_bindings::GL_TEXTURE_2D,
                gl_bindings::GL_TEXTURE_WRAP_S,
                gl_bindings::GL_CLAMP_TO_EDGE as i32,
            );
            gl_bindings::glTexParameteri(
                gl_bindings::GL_TEXTURE_2D,
                gl_bindings::GL_TEXTURE_WRAP_T,
                gl_bindings::GL_CLAMP_TO_EDGE as i32,
            );

            gl_bindings::glBindTexture(gl_bindings::GL_TEXTURE_2D, 0);
            gl_bindings::glFramebufferTexture2D(
                gl_bindings::GL_FRAMEBUFFER,
                gl_bindings::GL_COLOR_ATTACHMENT0,
                gl_bindings::GL_TEXTURE_2D,
                previous_texture[0] as u32,
                0,
            );

            if gl_bindings::glCheckFramebufferStatus(gl_bindings::GL_FRAMEBUFFER)
                != gl_bindings::GL_FRAMEBUFFER_COMPLETE
            {
                log::debug!("TextureRender Error: Could not setup frame buffer.")
            }
        }

        let size = matrix.size().unwrap_or(0) as usize;
        let mut matrix = std::slice::from_raw_parts_mut(matrix.as_ptr() as *mut f32, size);
        identity(matrix);
        gl_bindings::glUseProgram(program as u32);
        gl_bindings::glActiveTexture(gl_bindings::GL_TEXTURE0);
        gl_bindings::glBindTexture(
            gl_bindings::GL_TEXTURE_EXTERNAL_OES,
            external_texture as u32,
        );

        gl_bindings::glBindBuffer(gl_bindings::GL_ARRAY_BUFFER, array_buffer as u32);
        gl_bindings::glUniformMatrix4fv(matrix_pos, 1, 0, matrix.as_ptr());
        gl_bindings::glViewport(0, 0, width, height);
        gl_bindings::glDrawArrays(gl_bindings::GL_TRIANGLE_STRIP, 0, draw_count);
        gl_bindings::glFinish();

        gl_bindings::glBindTexture(gl_bindings::GL_TEXTURE_EXTERNAL_OES, 0);

        gl_bindings::glBindFramebuffer(
            gl_bindings::GL_FRAMEBUFFER,
            previous_frame_buffer[0] as u32,
        );

        gl_bindings::glViewport(previous_view_port[0], previous_view_port[1], previous_view_port[2], previous_view_port[3]);

        gl_bindings::glBindTexture(gl_bindings::GL_TEXTURE_2D, previous_texture[0] as u32);

        gl_bindings::glUseProgram(previous_program[0] as u32);
    }
}

fn identity(out: &mut [f32]) {
    out[0] = 1.;
    out[1] = 0.;
    out[2] = 0.;
    out[3] = 0.;
    out[4] = 0.;
    out[5] = 1.;
    out[6] = 0.;
    out[7] = 0.;
    out[8] = 0.;
    out[9] = 0.;
    out[10] = 1.;
    out[11] = 0.;
    out[12] = 0.;
    out[13] = 0.;
    out[14] = 0.;
    out[15] = 1.;
}
