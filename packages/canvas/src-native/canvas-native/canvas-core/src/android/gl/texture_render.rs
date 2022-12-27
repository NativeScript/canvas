use std::os::raw::c_void;

use jni::JNIEnv;
use jni::objects::{JClass, JObject, ReleaseMode};
use jni::sys::{jfloatArray, jint};

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TextureRender_nativeDrawFrame(
    env: JNIEnv,
    _: JClass,
    surface_texture_object: JObject,
    flip_y_web_gl: bool,
    fbo: jint,
    rbo: jint,
    program: jint,
    external_texture: jint,
    sampler_pos: jint,
    array_buffer: jint,
    pos: jint,
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
    let mut previous_view_port = [0_i32; 4];
    let mut previous_active_texture = [-1_i32; 1];
    let mut previous_texture = [-1_i32; 1];
    let mut previous_program = [-1_i32; 1];
    let mut previous_frame_buffer = [-1_i32; 1];
    let mut previous_render_buffer = [-1_i32; 1];
    let mut previous_vertex_array = [-1_i32; 1];

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

    gl_bindings::glGetIntegerv(
        gl_bindings::GL_RENDERBUFFER_BINDING,
        previous_render_buffer.as_mut_ptr(),
    );

    gl_bindings::glGetIntegerv(
        gl_bindings::GL_VERTEX_ARRAY_BINDING,
        previous_vertex_array.as_mut_ptr(),
    );

    gl_bindings::glBindFramebuffer(gl_bindings::GL_FRAMEBUFFER, fbo as u32);
    gl_bindings::glBindRenderbuffer(gl_bindings::GL_RENDERBUFFER, rbo as u32);

    if render_width != width || render_height != height {
        gl_bindings::glRenderbufferStorage(
            gl_bindings::GL_RENDERBUFFER,
            gl_bindings::GL_DEPTH24_STENCIL8,
            width,
            height,
        );
        gl_bindings::glFramebufferRenderbuffer(
            gl_bindings::GL_FRAMEBUFFER,
            gl_bindings::GL_DEPTH_STENCIL_ATTACHMENT,
            gl_bindings::GL_RENDERBUFFER,
            rbo as u32,
        );
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

        // gl_bindings::glGenerateMipmap(gl_bindings::GL_TEXTURE_2D);
        //
        // gl_bindings::glTexParameteri(gl_bindings::GL_TEXTURE_2D, gl_bindings::GL_TEXTURE_MIN_FILTER, gl_bindings::GL_LINEAR_MIPMAP_LINEAR as i32);
        // gl_bindings::glTexParameteri(gl_bindings::GL_TEXTURE_2D, gl_bindings::GL_TEXTURE_MAG_FILTER, gl_bindings::GL_LINEAR_MIPMAP_LINEAR as i32);

        // gl_bindings::glBindTexture(gl_bindings::GL_TEXTURE_2D, 0);
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

        // gl_bindings::glBindRenderbuffer(gl_bindings::GL_RENDERBUFFER,rbo as u32);
        //  gl_bindings::glRenderbufferStorage(gl_bindings::GL_RENDERBUFFER, gl_bindings::GL_DEPTH24_STENCIL8_OES, width, height);
        //  gl_bindings::glBindRenderbuffer(gl_bindings::GL_RENDERBUFFER, 0);

        //gl_bindings::glFramebufferRenderbuffer(gl_bindings::GL_FRAMEBUFFER, gl_bindings::GL_DEPTH_ATTACHMENT, gl_bindings::GL_RENDERBUFFER, rbo as u32);

        //  gl_bindings::glFramebufferRenderbuffer(gl_bindings::GL_FRAMEBUFFER, gl_bindings::GL_STENCIL_ATTACHMENT, gl_bindings::GL_RENDERBUFFER, rbo as u32);
    }

    gl_bindings::glClearColor(0., 0., 0., 1.);
    gl_bindings::glClear(gl_bindings::GL_COLOR_BUFFER_BIT | gl_bindings::GL_DEPTH_BUFFER_BIT);

    gl_bindings::glUseProgram(program as u32);
    gl_bindings::glBindBuffer(gl_bindings::GL_ARRAY_BUFFER, array_buffer as u32);
    gl_bindings::glVertexAttribPointer(
        pos as u32,
        2,
        gl_bindings::GL_FLOAT,
        0,
        2 * std::mem::size_of::<f32>() as i32,
        0 as *const std::ffi::c_void,
    );

    gl_bindings::glEnableVertexAttribArray(pos as u32);

    let api = unsafe { super::android_get_device_api_level() };

    if api >= 28 {
        let jni_st = crate::android::SURFACE_TEXTURE.get().unwrap();

        let st = jni_st.from_surface_texture().unwrap()(env.get_native_interface() as _, surface_texture_object.into_raw());

        jni_st.update_tex_image().unwrap()(st);

        let mut mtx = [0f32; 16];

        jni_st.get_transform_matrix().unwrap()(st, mtx.as_mut_ptr());

        jni_st.release().unwrap()(st);

        if flip_y_web_gl {
            identity(mtx.as_mut_slice());
        }

        gl_bindings::glBindTexture(
            gl_bindings::GL_TEXTURE_EXTERNAL_OES,
            external_texture as u32,
        );

        gl_bindings::glUniform1i(
            sampler_pos,
            previous_active_texture[0] - gl_bindings::GL_TEXTURE0 as i32,
        );

        gl_bindings::glUniformMatrix4fv(matrix_pos, 1, 0, mtx.as_ptr() as _);
        gl_bindings::glViewport(0, 0, width, height);

        gl_bindings::glDrawArrays(gl_bindings::GL_TRIANGLE_STRIP, 0, draw_count);
    } else {
        let method = crate::android::find_static_method_id(crate::android::TEXTURE_RENDER_STATIC_UPDATE_TEX_IMAGE_AND_GET_TRANSFORM_MATRIX_METHOD).unwrap();

        let _ =  env.call_static_method_unchecked(
            method.clazz(), method.id, crate::android::JAVA_VOID_TYPE, &[
                jni::objects::JValue::Object(surface_texture_object).to_jni(),
                jni::objects::JValue::Object(
                    JObject::from_raw(matrix)
                ).to_jni()
            ]
        );
        if let Ok(matrix) = env.get_primitive_array_critical(matrix, ReleaseMode::CopyBack) {
            // super::surface_texture::ASurfaceTexture_updateTexImage(std::mem::transmute(
            //     surface_texture_object.into_inner(),
            // ));
            //
            // super::surface_texture::ASurfaceTexture_getTransformMatrix(
            //     std::mem::transmute(surface_texture_object.into_inner()),
            //     matrix.as_ptr() as *mut _,
            // );

            //  let clazz = env.find_class("android/opengl/Matrix").unwrap();
            // env.call_static_method(clazz, "setIdentityM", "([F;I)V", &[])

            let size = matrix.size().unwrap_or(0) as usize;

            let matrix = std::slice::from_raw_parts_mut(matrix.as_ptr() as *mut f32, size);

            if flip_y_web_gl {
                identity(matrix);
            }

            gl_bindings::glBindTexture(
                gl_bindings::GL_TEXTURE_EXTERNAL_OES,
                external_texture as u32,
            );

            //      previous_active_texture[0] - gl_bindings::GL_TEXTURE0
            // gl_bindings::glUniform1i(
            //     sampler_pos,
            //     0, // previous_active_texture[0] - gl_bindings::GL_TEXTURE0 as i32,
            // );

            gl_bindings::glUniform1i(
                sampler_pos,
                previous_active_texture[0] - gl_bindings::GL_TEXTURE0 as i32,
            );

            /*  let name = std::ffi::CString::new("aTexCoord").unwrap();
            let pos = gl_bindings::glGetAttribLocation(program as u32, name.as_ptr()) as u32;

            gl_bindings::glVertexAttribPointer(
                pos,
                2,
                gl_bindings::GL_FLOAT,
                0,
                2 * std::mem::size_of::<f32>() as i32,
                0 as *const std::ffi::c_void,
            );

            gl_bindings::glEnableVertexAttribArray(pos);
            */

            gl_bindings::glUniformMatrix4fv(matrix_pos, 1, 0, matrix.as_ptr() as _);
            gl_bindings::glViewport(0, 0, width, height);

            gl_bindings::glDrawArrays(gl_bindings::GL_TRIANGLE_STRIP, 0, draw_count);

            //  gl_bindings::glFinish();

            //gl_bindings::glBindTexture(gl_bindings::GL_TEXTURE_EXTERNAL_OES, 0);

            //  gl_bindings::glBindRenderbuffer(gl_bindings::GL_RENDERBUFFER, previous_render_buffer[0] as u32);
        }
    }


    gl_bindings::glBindRenderbuffer(
        gl_bindings::GL_RENDERBUFFER,
        previous_render_buffer[0] as u32,
    );
    gl_bindings::glBindFramebuffer(gl_bindings::GL_FRAMEBUFFER, previous_frame_buffer[0] as u32);

    gl_bindings::glViewport(
        previous_view_port[0],
        previous_view_port[1],
        previous_view_port[2],
        previous_view_port[3],
    );

    gl_bindings::glBindTexture(gl_bindings::GL_TEXTURE_2D, previous_texture[0] as u32);

    gl_bindings::glUseProgram(previous_program[0] as u32);

    gl_bindings::glBindVertexArray(previous_vertex_array[0] as u32);
}

#[inline(always)]
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
