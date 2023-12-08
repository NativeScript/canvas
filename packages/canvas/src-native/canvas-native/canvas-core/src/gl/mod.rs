#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "ios")]
pub use ios::*;
use std::ffi::CString;

#[cfg(target_os = "macos")]
mod mac;

#[cfg(target_os = "macos")]
pub use mac::*;

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "android")]
pub use android::*;

#[cfg(any(target_os = "android", target_os = "ios"))]
const VERTEX_SHADER: &str = "
precision highp float;
attribute vec4 aPosition;
uniform mat4 uTextureMatrix;
varying vec2 TexCoord;
void main(){
vec2 clipSpace = (1.0 - 2.0 * aPosition.xy);
TexCoord = (uTextureMatrix * aPosition).xy;
gl_Position = vec4(clipSpace, 0.0, 1.0);
}
";

#[cfg(target_os = "macos")]
const VERTEX_SHADER: &str = "
#version 330 core
precision highp float;
in vec4 aPosition;
out vec2 TexCoord;
uniform mat4 uTextureMatrix;
void main() {
    vec2 clipSpace = (1.0 - 2.0 * aPosition.xy);
    TexCoord = (uTextureMatrix * aPosition).xy;
    gl_Position = vec4(clipSpace, 0.0, 1.0);
}
";

#[cfg(any(target_os = "android", target_os = "ios"))]
const FRAGMENT_SHADER: &str = "
			#version 330 core
			precision highp float;
			varying vec2 TexCoord;
uniform samplerExternalOES uSampler;
void main(){
gl_FragColor = texture2D(uSampler, TexCoord);
}
";

#[cfg(target_os = "macos")]
const FRAGMENT_SHADER: &str = "
#version 330 core

precision highp float;
in vec2 TexCoord;
uniform sampler2D uSampler;
out vec4 FragColor;

void main() {
FragColor = texture(uSampler, TexCoord);
}
";

const VERTEX_COORDS: [f32; 8] = [0., 0., 1., 0., 0., 1., 1., 1.];

#[derive(Copy, Clone, Debug, Default)]
pub struct TransferSurface {
    program: u32,
    rbo: u32,
    fbo: u32,
    width: u32,
    height: u32,
    ab: u32,
    pos: i32,
    matrix_pos: i32,
    sampler_pos: i32,
    matrix: [f32; 16],
    initialized: bool,
}

fn get_shader_parameter(shader: u32, pname: u32) -> i32 {
    let mut params = 0i32;
    unsafe { gl_bindings::GetShaderiv(shader, pname, &mut params) }
    params
}

pub fn get_shader_info_log(shader: u32) -> String {
    let mut length = 0i32;
    unsafe { gl_bindings::GetShaderiv(shader, gl_bindings::INFO_LOG_LENGTH, &mut length) }

    if length == 0 {
        return String::new();
    }

    let mut log = vec![0; length as usize];
    let mut len = 0;
    unsafe {
        gl_bindings::GetShaderInfoLog(
            shader,
            length,
            &mut len,
            log.as_mut_ptr() as *mut std::ffi::c_char,
        )
    }

    log.shrink_to(len.try_into().unwrap());
    let c_str = unsafe { std::ffi::CStr::from_ptr(log.as_ptr()) };
    c_str.to_string_lossy().to_string()
}

impl TransferSurface {
    pub fn init(&mut self, texture_id: u32) {
        unsafe {
            if self.initialized {
                return;
            }

            self.program = gl_bindings::CreateProgram();
            let vs = gl_bindings::CreateShader(gl_bindings::VERTEX_SHADER);
            let vs_source = CString::new(VERTEX_SHADER).unwrap();
            gl_bindings::ShaderSource(vs, 1, &vs_source.as_ptr(), std::ptr::null());

            let fs = gl_bindings::CreateShader(gl_bindings::FRAGMENT_SHADER);
            let fs_source = CString::new(FRAGMENT_SHADER).unwrap();

            gl_bindings::ShaderSource(fs, 1, &fs_source.as_ptr(), std::ptr::null());

            gl_bindings::CompileShader(vs);

            // let status = get_shader_parameter(vs, 0x8B81);
            //
            // if status == 0 {
            //     println!("Shader compilation failed: {:?}", get_shader_info_log(vs));
            // }

            gl_bindings::CompileShader(fs);

            // let status = get_shader_parameter(fs, 0x8B81);
            //
            // if status == 0 {
            //     println!("Shader compilation failed: {:?}", get_shader_info_log(fs));
            // }

            gl_bindings::AttachShader(self.program, vs);
            gl_bindings::AttachShader(self.program, fs);

            gl_bindings::LinkProgram(self.program);

            let mut buffers = [0_u32];

            gl_bindings::GenBuffers(1, buffers.as_mut_ptr());
            self.ab = buffers[0];

            let mut rbos = [0_u32];
            gl_bindings::GenRenderbuffers(1, rbos.as_mut_ptr());
            self.rbo = rbos[0];

            let mut fbos = [0_u32];
            gl_bindings::GenFramebuffers(1, fbos.as_mut_ptr());
            self.fbo = fbos[0];

            // let mut textures = [0_u32];
            // GLES20.glGenTextures(1, textures, 0)
            // textureId = textures[0]

            gl_bindings::BindBuffer(gl_bindings::ARRAY_BUFFER, self.ab);
            gl_bindings::BufferData(
                gl_bindings::ARRAY_BUFFER,
                (std::mem::size_of::<f32>() * VERTEX_COORDS.len()) as isize,
                VERTEX_COORDS.as_ptr() as _,
                gl_bindings::STATIC_DRAW,
            );

            let u_sampler = CString::new("uSampler").unwrap();
            let u_texture_matrix = CString::new("uTextureMatrix").unwrap();
            let a_position = CString::new("aPosition").unwrap();
            self.sampler_pos = gl_bindings::GetUniformLocation(self.program, u_sampler.as_ptr());
            self.matrix_pos =
                gl_bindings::GetUniformLocation(self.program, u_texture_matrix.as_ptr());
            self.pos = gl_bindings::GetAttribLocation(self.program, a_position.as_ptr());

            gl_bindings::VertexAttribPointer(
                self.pos as u32,
                2,
                gl_bindings::FLOAT,
                0,
                (2 * std::mem::size_of::<f32>()) as _,
                std::ptr::null(),
            );
            gl_bindings::EnableVertexAttribArray(self.pos as u32);

            let mut previous_texture = [0_i32];

            gl_bindings::GetIntegerv(
                gl_bindings::TEXTURE_BINDING_2D,
                previous_texture.as_mut_ptr(),
            );

            gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, texture_id);

            gl_bindings::TexParameteri(
                gl_bindings::TEXTURE_2D,
                gl_bindings::TEXTURE_MIN_FILTER,
                gl_bindings::LINEAR as _,
            );

            gl_bindings::TexParameteri(
                gl_bindings::TEXTURE_2D,
                gl_bindings::TEXTURE_MAG_FILTER,
                gl_bindings::LINEAR as _,
            );

            gl_bindings::TexParameteri(
                gl_bindings::TEXTURE_2D,
                gl_bindings::TEXTURE_WRAP_S,
                gl_bindings::CLAMP_TO_EDGE as _,
            );

            gl_bindings::TexParameteri(
                gl_bindings::TEXTURE_2D,
                gl_bindings::TEXTURE_WRAP_T,
                gl_bindings::CLAMP_TO_EDGE as _,
            );

            gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, previous_texture[0] as u32);

            self.initialized = true;
        }
    }

    pub fn draw_tex_image_2d(
        &mut self,
        target: u32,
        level: i32,
        width: u32,
        height: u32,
        internal_format: i32,
        format: i32,
        flip_y_webgl: bool,
        texture_id: u32,
    ) {
        let mut previous_texture = -1_i32;

        unsafe {
            gl_bindings::GetIntegerv(
                gl_bindings::TEXTURE_BINDING_2D,
                &mut previous_texture,
            );

            gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, texture_id);


            gl_bindings::TexParameteri(
                gl_bindings::TEXTURE_2D,
                gl_bindings::TEXTURE_MAG_FILTER,
                gl_bindings::LINEAR as i32,
            );
            gl_bindings::TexParameteri(
                gl_bindings::TEXTURE_2D,
                gl_bindings::TEXTURE_MIN_FILTER,
                gl_bindings::LINEAR as i32,
            );

            gl_bindings::TexParameteri(
                gl_bindings::TEXTURE_2D,
                gl_bindings::TEXTURE_WRAP_S,
                gl_bindings::CLAMP_TO_EDGE as i32,
            );
            gl_bindings::TexParameteri(
                gl_bindings::TEXTURE_2D,
                gl_bindings::TEXTURE_WRAP_T,
                gl_bindings::CLAMP_TO_EDGE as i32,
            );

            gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, previous_texture as u32);


            gl_bindings::CopyTexImage2D(
                target,
                level,
                internal_format as u32,
                0,
                0,
                width as i32,
                height as i32,
                0
            );

            gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, previous_texture as u32);
        }

        self.width = width;
        self.height = height;
    }



    pub fn draw_tex_sub_image_2d(
        &mut self,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: u32,
        height: u32,
        flip_y_webgl: bool,
        texture_id: u32,
    ) {

        let mut previous_texture = -1_i32;

        unsafe {
            gl_bindings::GetIntegerv(
                gl_bindings::TEXTURE_BINDING_2D,
                &mut previous_texture,
            );

            gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, texture_id);

            gl_bindings::TexParameteri(
                gl_bindings::TEXTURE_2D,
                gl_bindings::TEXTURE_MAG_FILTER,
                gl_bindings::LINEAR as i32,
            );
            gl_bindings::TexParameteri(
                gl_bindings::TEXTURE_2D,
                gl_bindings::TEXTURE_MIN_FILTER,
                gl_bindings::LINEAR as i32,
            );

            gl_bindings::TexParameteri(
                gl_bindings::TEXTURE_2D,
                gl_bindings::TEXTURE_WRAP_S,
                gl_bindings::CLAMP_TO_EDGE as i32,
            );
            gl_bindings::TexParameteri(
                gl_bindings::TEXTURE_2D,
                gl_bindings::TEXTURE_WRAP_T,
                gl_bindings::CLAMP_TO_EDGE as i32,
            );

            gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, previous_texture as u32);


            gl_bindings::CopyTexSubImage2D(
                target,
                level,
                xoffset,
                yoffset,
                0,
                0,
                width as i32,
                height as i32,
            );

            gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, previous_texture as u32);
        }
    }
}
