//! WebGL extensions on ANGLE (Windows).
//!
//! WebGL contexts run in ANGLE's WebGL-compatibility mode, which exposes a GL extension only once
//! it is requested (`GL_ANGLE_request_extension`), the way Chromium enables them in
//! `getExtension`. A WebGL extension is offered when every GL extension it needs is enabled or
//! requestable, and only on the WebGL versions it exists in (WebGL 2 has most of WebGL 1's
//! extensions in core). WebGL 1 contexts are OpenGL ES 2 contexts, so the extension objects call
//! the suffixed (`ANGLE` / `OES` / `EXT`) entry points.

use std::ffi::{c_char, CStr, CString};
use std::sync::OnceLock;

use crate::prelude::*;

const REQUESTABLE_EXTENSIONS_ANGLE: u32 = 0x93A8;

const V1: u8 = 1;
const V2: u8 = 2;
const BOTH: u8 = V1 | V2;

/// WebGL extension, the WebGL versions it exists in, the GL extensions it needs on ANGLE.
const EXTENSIONS: &[(&str, u8, &[&str])] = &[
    ("ANGLE_instanced_arrays", V1, &["GL_ANGLE_instanced_arrays"]),
    ("EXT_blend_minmax", V1, &["GL_EXT_blend_minmax"]),
    ("EXT_color_buffer_float", V2, &["GL_EXT_color_buffer_float"]),
    ("EXT_color_buffer_half_float", BOTH, &["GL_EXT_color_buffer_half_float"]),
    ("EXT_sRGB", V1, &["GL_EXT_sRGB"]),
    ("EXT_shader_texture_lod", V1, &["GL_EXT_shader_texture_lod"]),
    ("EXT_texture_filter_anisotropic", BOTH, &["GL_EXT_texture_filter_anisotropic"]),
    ("OES_element_index_uint", V1, &["GL_OES_element_index_uint"]),
    ("OES_standard_derivatives", V1, &["GL_OES_standard_derivatives"]),
    ("OES_texture_float", V1, &["GL_OES_texture_float"]),
    ("OES_texture_float_linear", BOTH, &["GL_OES_texture_float_linear"]),
    ("OES_texture_half_float", V1, &["GL_OES_texture_half_float"]),
    ("OES_texture_half_float_linear", V1, &["GL_OES_texture_half_float_linear"]),
    ("OES_vertex_array_object", V1, &["GL_OES_vertex_array_object"]),
    ("WEBGL_color_buffer_float", V1, &["GL_CHROMIUM_color_buffer_float_rgba"]),
    ("WEBGL_compressed_texture_etc", BOTH, &["GL_ANGLE_compressed_texture_etc"]),
    ("WEBGL_compressed_texture_etc1", BOTH, &["GL_OES_compressed_ETC1_RGB8_texture"]),
    (
        "WEBGL_compressed_texture_s3tc",
        BOTH,
        &[
            "GL_EXT_texture_compression_dxt1",
            "GL_ANGLE_texture_compression_dxt3",
            "GL_ANGLE_texture_compression_dxt5",
        ],
    ),
    ("WEBGL_depth_texture", V1, &["GL_ANGLE_depth_texture"]),
    ("WEBGL_draw_buffers", V1, &["GL_EXT_draw_buffers"]),
    ("WEBGL_lose_context", BOTH, &[]),
];

fn version_bit(state: &WebGLState) -> u8 {
    match state.get_webgl_version() {
        WebGLVersion::V2 => V2,
        _ => V1,
    }
}

fn gl_string(name: u32) -> String {
    let value = unsafe { gl_bindings::GetString(name) };
    if value.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(value as *const c_char) }
        .to_string_lossy()
        .into_owned()
}

fn listed(list: &str, extension: &str) -> bool {
    list.split(' ').any(|e| e == extension)
}

type RequestExtension = unsafe extern "system" fn(name: *const c_char);

fn request(extension: &str) {
    static REQUEST: OnceLock<Option<RequestExtension>> = OnceLock::new();
    let request = REQUEST.get_or_init(|| {
        let proc = canvas_core::gpu::gl::get_proc_address("glRequestExtensionANGLE");
        (!proc.is_null()).then(|| unsafe { std::mem::transmute::<_, RequestExtension>(proc) })
    });
    if let (Some(request), Ok(name)) = (request, CString::new(extension)) {
        unsafe { request(name.as_ptr()) };
    }
}

/// The WebGL extensions the current context can enable.
pub fn get_supported_extensions(state: &mut WebGLState) -> Vec<String> {
    state.make_current();
    let bit = version_bit(state);
    let enabled = gl_string(gl_bindings::EXTENSIONS);
    let requestable = gl_string(REQUESTABLE_EXTENSIONS_ANGLE);
    EXTENSIONS
        .iter()
        .filter(|(_, versions, needs)| {
            versions & bit != 0
                && needs
                    .iter()
                    .all(|e| listed(&enabled, e) || listed(&requestable, e))
        })
        .map(|(name, _, _)| name.to_string())
        .collect()
}

/// Enables `name` (requesting its GL extensions) and returns its extension object, or `None`
/// when it is not offered on this context.
pub fn get_extension(name: &str, state: &mut WebGLState) -> Option<Box<dyn WebGLExtension>> {
    let bit = version_bit(state);
    let (_, _, needs) = EXTENSIONS
        .iter()
        .find(|(n, versions, _)| *n == name && versions & bit != 0)?;
    state.make_current();
    let enabled = gl_string(gl_bindings::EXTENSIONS);
    let missing: Vec<&str> = needs.iter().copied().filter(|e| !listed(&enabled, e)).collect();
    if !missing.is_empty() {
        let requestable = gl_string(REQUESTABLE_EXTENSIONS_ANGLE);
        if !missing.iter().all(|e| listed(&requestable, e)) {
            return None;
        }
        missing.iter().for_each(|e| request(e));
        let enabled = gl_string(gl_bindings::EXTENSIONS);
        if !missing.iter().all(|e| listed(&enabled, e)) {
            return None;
        }
    }

    Some(match name {
        "ANGLE_instanced_arrays" => Box::new(ANGLE_instanced_arrays::new(state)),
        "EXT_blend_minmax" => Box::new(EXT_blend_minmax::new()),
        "EXT_color_buffer_float" | "WEBGL_color_buffer_float" => {
            Box::new(WEBGL_color_buffer_float::new())
        }
        "EXT_color_buffer_half_float" => Box::new(EXT_color_buffer_half_float::new()),
        "EXT_sRGB" => Box::new(EXT_sRGB::new()),
        "EXT_shader_texture_lod" => Box::new(EXT_shader_texture_lod::new()),
        "EXT_texture_filter_anisotropic" => Box::new(EXT_texture_filter_anisotropic::new()),
        "OES_element_index_uint" => Box::new(OES_element_index_uint::new()),
        "OES_standard_derivatives" => Box::new(OES_standard_derivatives::new()),
        "OES_texture_float" => Box::new(OES_texture_float::new()),
        "OES_texture_float_linear" => Box::new(OES_texture_float_linear::new()),
        "OES_texture_half_float" => Box::new(OES_texture_half_float::new()),
        "OES_texture_half_float_linear" => Box::new(OES_texture_half_float_linear::new()),
        "OES_vertex_array_object" => Box::new(OES_vertex_array_object::new(state)),
        "WEBGL_compressed_texture_etc" => Box::new(WEBGL_compressed_texture_etc::new()),
        "WEBGL_compressed_texture_etc1" => Box::new(WEBGL_compressed_texture_etc1::new()),
        "WEBGL_compressed_texture_s3tc" => Box::new(WEBGL_compressed_texture_s3tc::new()),
        "WEBGL_depth_texture" => Box::new(WEBGL_depth_texture::new()),
        "WEBGL_draw_buffers" => Box::new(WEBGL_draw_buffers::new(state)),
        "WEBGL_lose_context" => Box::new(WEBGL_lose_context::new(state)),
        _ => return None,
    })
}
