use std::ffi::CString;

use canvas_webgl::prelude::WebGLVersion;
use canvas_webgl::utils::{create_program_from_scripts, ShaderSource, ShaderSourceType};

use crate::{CanvasRenderingContext2D, WebGLState};


pub fn draw_image_space_test(state: &mut WebGLState, canvas: &mut CanvasRenderingContext2D, internalFormat: i32, format: i32){
    {
        let state = state.get_inner_mut();
        state.make_current();
        if state.get_webgl_version() != WebGLVersion::V2 {
            return;
        }
    }

    let (drawingBufferWidth, drawingBufferHeight) = state.get_dimensions();

    let vs = r#"#version 300 es
    precision highp float;
    precision highp int;

    void main()
    {
        gl_Position = vec4(2.f * float(uint(gl_VertexID) % 2u) - 1.f, 2.f * float(uint(gl_VertexID) / 2u) - 1.f, 0.0, 1.0);
    }
    "#;
    let fs = r#"#version 300 es
    precision highp float;
    precision highp int;

    uniform sampler2D diffuse;

    uniform vec2 u_imageSize;

    out vec4 color;

    void main()
    {
        color = texture(diffuse, vec2(gl_FragCoord.x, u_imageSize.y - gl_FragCoord.y) / u_imageSize);
    }
    "#;

    let program = create_program_from_scripts(state.get_inner_mut(), &[
        ShaderSource::new(vs, ShaderSourceType::Vertex),
        ShaderSource::new(fs, ShaderSourceType::Fragment)
    ]);


    if let Err(program) = program {
        log::log!(target: "JS", log::Level::Trace, "Failed to compile program {:?}", program);
        return;
    }

    let program = program.unwrap();

  //  let state = state as *mut _;
    let diffuse = CString::new("diffuse").unwrap();
    let u_imageSize = CString::new("u_imageSize").unwrap();
    let diffuseLocation = crate::canvas_native_webgl_get_uniform_location(program, diffuse.as_ptr(), state);
    let imageSizeLocation = crate::canvas_native_webgl_get_uniform_location(program, u_imageSize.as_ptr(), state);

    // -- Init VertexArray
    let vertexArray = crate::canvas_native_webgl2_create_vertex_array(state);
    crate::canvas_native_webgl2_bind_vertex_array(vertexArray, state);
    crate::canvas_native_webgl2_bind_vertex_array(0, state);



    let texture = crate::canvas_native_webgl_create_texture(state);
    crate::canvas_native_webgl_active_texture(gl_bindings::TEXTURE0, state);
    crate::canvas_native_webgl_bind_texture(gl_bindings::TEXTURE_2D, texture, state);
    crate::canvas_native_webgl_pixel_storei(crate::GLConstants::UNPACK_FLIP_Y_WEBGL as u32, 0, state);

   // gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, asset);

    unsafe {
        crate::canvas_native_webgl_tex_image2d_canvas2d(
            gl_bindings::TEXTURE_2D as i32,
            0,
            internalFormat,
            format,
            gl_bindings::UNSIGNED_BYTE as i32,
            canvas,
            state
        );
    }

    crate::canvas_native_webgl_tex_parameteri(gl_bindings::TEXTURE_2D, gl_bindings::TEXTURE_MAG_FILTER, gl_bindings::LINEAR as i32, state);
    crate::canvas_native_webgl_tex_parameteri(gl_bindings::TEXTURE_2D, gl_bindings::TEXTURE_MIN_FILTER, gl_bindings::LINEAR as i32, state);

    // -- Render

    crate::canvas_native_webgl_clear_color(0.,0.,0.,0., state);
    crate::canvas_native_webgl_clear(gl_bindings::COLOR_BUFFER_BIT, state);

    crate::canvas_native_webgl_use_program(program, state);
    crate::canvas_native_webgl_uniform1i(diffuseLocation, 0, state);
    crate::canvas_native_webgl_uniform2f(imageSizeLocation, (drawingBufferWidth / 2) as f32, (drawingBufferHeight / 2) as f32, state);

    crate::canvas_native_webgl2_bind_vertex_array(vertexArray, state);

    crate::canvas_native_webgl_draw_arrays(gl_bindings::TRIANGLES, 0, 3, state);

    // Delete WebGL resources
    crate::canvas_native_webgl_delete_texture(texture, state);
    crate::canvas_native_webgl_delete_program(program, state);
    crate::canvas_native_webgl2_delete_vertex_array_with_vertex_array(vertexArray, state);

}