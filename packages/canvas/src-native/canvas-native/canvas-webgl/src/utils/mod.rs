use crate::prelude::{WebGLResult, WebGLState};

pub mod gl;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum ShaderSourceType {
    Vertex,
    Fragment,
}

pub struct ShaderSource {
    src: &'static str,
    source_type: ShaderSourceType,
}

impl ShaderSource {
    pub fn new(src: &'static str, source_type: ShaderSourceType) -> Self {
        Self { src, source_type }
    }
}

pub fn create_program_from_scripts(
    state: &mut WebGLState,
    shader_sources: &[ShaderSource],
) -> Result<u32, String> {
    // setup GLSL programs

    let mut shaders = Vec::new();

    for source in shader_sources {
        if source.source_type == ShaderSourceType::Vertex {
            // Create the shader object
            let vertex_shader =
                crate::webgl::canvas_native_webgl_create_shader(gl_bindings::VERTEX_SHADER, state);
            // Load the shader source
            crate::webgl::canvas_native_webgl_shader_source(vertex_shader, source.src, state);

            // Compile the shader
            crate::webgl::canvas_native_webgl_compile_shader(vertex_shader, state);

            // Check the compile status
            let compiled = crate::webgl::canvas_native_webgl_get_shader_parameter(
                vertex_shader,
                gl_bindings::COMPILE_STATUS,
                state,
            );

            match compiled {
                WebGLResult::Boolean(compiled) => {
                    if !compiled {
                        let last_error = crate::webgl::canvas_native_webgl_get_shader_info_log(
                            vertex_shader,
                            state,
                        );
                        crate::webgl::canvas_native_webgl_delete_shader(vertex_shader, state);
                        return Err(format!(
                            "*** Error compiling shader {}:{}",
                            vertex_shader, last_error
                        ));
                    }
                }
                _ => {}
            }

            shaders.push(vertex_shader);
        }

        if source.source_type == ShaderSourceType::Fragment {
            // Create the shader object
            let fragment_shader = crate::webgl::canvas_native_webgl_create_shader(
                gl_bindings::FRAGMENT_SHADER,
                state,
            );

            // Load the shader source
            crate::webgl::canvas_native_webgl_shader_source(fragment_shader, source.src, state);

            // Compile the shader
            crate::webgl::canvas_native_webgl_compile_shader(fragment_shader, state);

            // Check the compile status
            let compiled = crate::webgl::canvas_native_webgl_get_shader_parameter(
                fragment_shader,
                gl_bindings::COMPILE_STATUS,
                state,
            );

            match compiled {
                WebGLResult::Boolean(compiled) => {
                   if !compiled {
                       // Something went wrong during compilation; get the error
                       let last_error = crate::webgl::canvas_native_webgl_get_shader_info_log(
                           fragment_shader,
                           state,
                       );
                       crate::webgl::canvas_native_webgl_delete_shader(fragment_shader, state);
                       return Err(format!(
                           "*** Error compiling shader {}:{}",
                           fragment_shader, last_error
                       ));
                   }
                }
                _ => {}
            }

            shaders.push(fragment_shader);
        }
    }

    let program = crate::webgl::canvas_native_webgl_create_program(state);

    for shader in shaders {
        crate::webgl::canvas_native_webgl_attach_shader(program, shader, state);
    }

    crate::webgl::canvas_native_webgl_link_program(program, state);

    // Check the link status
    let linked = crate::webgl::canvas_native_webgl_get_program_parameter(
        program,
        gl_bindings::LINK_STATUS,
        state,
    );
    match linked {
        WebGLResult::Boolean(linked) => {
           if !linked {
               let last_error = crate::webgl::canvas_native_webgl_get_program_info_log(program, state);
               crate::webgl::canvas_native_webgl_delete_program(program, state);
               return Err(format!("Error in program linking: {}", last_error));
           }
        }
        _ => {}
    }

    Ok(program)
}
