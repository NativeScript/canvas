use napi::bindgen_prelude::{ObjectFinalize, Unknown};
use napi::*;
use napi_derive::napi;

#[napi(js_name = "ANGLE_instanced_arrays", custom_finalize)]
pub struct ANGLE_instanced_arrays(pub(crate) *const canvas_c::ANGLE_instanced_arrays);

impl ObjectFinalize for ANGLE_instanced_arrays {
    fn finalize(self, _: Env) -> Result<()> {
        canvas_c::canvas_native_webgl_ANGLE_instanced_arrays_destroy(self.0 as _);
        Ok(())
    }
}

#[napi]
impl ANGLE_instanced_arrays {
    #[napi(getter)]
    pub fn get_vertex_attrib_array_divisor_angle(&self) -> u32 {
        0x88FE
    }

    #[napi]
    pub fn draw_arrays_instanced_angle(&self, mode: u32, first: i32, count: i32, primcount: i32) {
        canvas_c::canvas_native_webgl_angle_instanced_arrays_draw_arrays_instanced_angle(mode, first, count, primcount, self.0)
    }

    #[napi(
        ts_args_type = "mode: number, count: number, type: number, offset: number, primcount: number"
    )]
    pub fn draw_elements_instanced_angle(
        &self,
        mode: u32,
        count: i32,
        type_: u32,
        offset: i32,
        primcount: i32,
    ) {
        canvas_c::canvas_native_webgl_angle_instanced_arrays_draw_elements_instanced_angle(mode, count, type_, offset, primcount, self.0)
    }

    #[napi]
    pub fn vertex_attrib_divisor_angle(&self, index: u32, divisor: u32) {
        canvas_c::canvas_native_webgl_angle_instanced_arrays_vertex_attrib_divisor_angle(index, divisor, self.0)
    }
}

#[napi(js_name = "OES_fbo_render_mipmap")]
pub struct OES_fbo_render_mipmap;

#[napi(js_name = "EXT_blend_minmax")]
pub struct EXT_blend_minmax;

#[napi(js_name = "EXT_color_buffer_half_float")]
pub struct EXT_color_buffer_half_float;

#[napi(js_name = "EXT_disjoint_timer_query", custom_finalize)]
pub struct EXT_disjoint_timer_query(pub(crate) *const canvas_c::EXT_disjoint_timer_query);

impl ObjectFinalize for EXT_disjoint_timer_query {
    fn finalize(self, _: Env) -> Result<()> {
        canvas_c::canvas_native_webgl_EXT_disjoint_timer_query_destroy(self.0 as _);
        Ok(())
    }
}

#[napi]
impl EXT_disjoint_timer_query {
    #[napi]
    pub fn create_query_ext(&self) -> u32 {
        canvas_c::canvas_native_webgl_ext_disjoint_timer_query_create_query_ext(self.0)
    }

    #[napi]
    pub fn delete_query_ext(&self, query: u32) {
        canvas_c::canvas_native_webgl_ext_disjoint_timer_query_delete_query_ext(query, self.0)
    }

    #[napi]
    pub fn is_query_ext(&self, query: u32) -> bool {
        canvas_c::canvas_native_webgl_ext_disjoint_timer_query_is_query_ext(query, self.0)
    }

    #[napi]
    pub fn begin_query_ext(&self, target: u32, query: u32) {
        canvas_c::canvas_native_webgl_ext_disjoint_timer_query_begin_query_ext(target, query, self.0)
    }

    #[napi]
    pub fn end_query_ext(&self, target: u32) {
        canvas_c::canvas_native_webgl_ext_disjoint_timer_query_end_query_ext(target, self.0)
    }

    #[napi]
    pub fn query_counter_ext(&self, query: u32, target: u32) {
        canvas_c::canvas_native_webgl_ext_disjoint_timer_query_query_counter_ext(query, target, self.0)
    }

    #[napi]
    pub fn get_query_ext(&self, target: u32, pname: u32) -> i32 {
        canvas_c::canvas_native_webgl_ext_disjoint_timer_query_get_query_ext(target, pname, self.0)
    }

    #[napi]
    pub fn get_query_object_ext(&self, env: Env, target: u32, pname: u32) -> Result<Unknown> {
        let obj = canvas_c::canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(target, pname, self.0);
        if obj.is_null() {
            return env.get_null().map(|v| v.into_unknown());
        }

        // QUERY_RESULT_AVAILABLE
        if pname == 0x8867 {
            let ret = canvas_c::canvas_native_webgl_result_get_bool(obj as _);
            return env.get_boolean(ret).map(|v| v.into_unknown());
        }
        let ret = canvas_c::canvas_native_webgl_result_get_i32(obj as _);
        env.create_int32(ret).map(|v| v.into_unknown())
    }

    #[napi(js_name = "QUERY_COUNTER_BITS_EXT", getter)]
    pub fn get_query_counter_bits_ext(&self) -> u32 {
        0x8864
    }

    #[napi(js_name = "CURRENT_QUERY_EXT", getter)]
    pub fn get_current_query_ext(&self) -> u32 {
        0x8865
    }

    #[napi(js_name = "QUERY_RESULT_EXT", getter)]
    pub fn get_query_result_ext(&self) -> u32 {
        0x8866
    }

    #[napi(js_name = "QUERY_RESULT_AVAILABLE_EXT", getter)]
    pub fn get_query_result_available_ext(&self) -> u32 {
        0x8867
    }

    #[napi(js_name = "TIME_ELAPSED_EXT", getter)]
    pub fn get_time_elapsed_ext(&self) -> u32 {
        0x88BF
    }

    #[napi(js_name = "TIMESTAMP_EXT", getter)]
    pub fn get_timestamp_ext(&self) -> u32 {
        0x88BF
    }

    #[napi(js_name = "GPU_DISJOINT_EXT", getter)]
    pub fn get_gpu_disjoint_ext(&self) -> u32 {
        0x8FBB
    }
}

#[napi(js_name = "EXT_sRGB")]
pub struct EXT_sRGB;

#[napi(js_name = "EXT_shader_texture_lod")]
pub struct EXT_shader_texture_lod;

#[napi(js_name = "EXT_texture_filter_anisotropic")]
pub struct EXT_texture_filter_anisotropic;

#[napi(js_name = "OES_element_index_uint")]
pub struct OES_element_index_uint;

#[napi(js_name = "OES_standard_derivatives")]
pub struct OES_standard_derivatives;

#[napi(js_name = "OES_texture_float")]
pub struct OES_texture_float;

#[napi(js_name = "OES_texture_float_linear")]
pub struct OES_texture_float_linear;

#[napi(js_name = "OES_texture_half_float")]
pub struct OES_texture_half_float;

#[napi(js_name = "OES_texture_half_float_linear")]
pub struct OES_texture_half_float_linear;

#[napi(js_name = "OES_vertex_array_object", custom_finalize)]
pub struct OES_vertex_array_object(pub(crate) *const canvas_c::OES_vertex_array_object);

impl ObjectFinalize for OES_vertex_array_object {
    fn finalize(self, _: Env) -> Result<()> {
        canvas_c::canvas_native_webgl_OES_vertex_array_object_destroy(self.0 as _);
        Ok(())
    }
}

#[napi]
impl OES_vertex_array_object {
    #[napi(js_name = "VERTEX_ARRAY_BINDING_OES", getter)]
    pub fn get_vertex_array_binding_oes(&self) -> u32 {
        0x85B5
    }

    pub fn create_vertex_array_oes(&self) -> u32 {
        canvas_c::canvas_native_webgl_oes_vertex_array_object_create_vertex_array_oes(self.0)
    }

    pub fn delete_vertex_array_oes(&self, array_object: u32) {
        canvas_c::canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(array_object, self.0)
    }

    pub fn is_vertex_array_oes(&self, array_object: u32) -> bool {
        canvas_c::canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(array_object, self.0)
    }

    pub fn bind_vertex_array_oes(&self, array_object: u32) {
        canvas_c::canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(array_object, self.0)
    }
}

#[napi(js_name = "WEBGL_color_buffer_float")]
pub struct WEBGL_color_buffer_float;

#[napi(js_name = "WEBGL_compressed_texture_atc")]
pub struct WEBGL_compressed_texture_atc;

#[napi(js_name = "WEBGL_compressed_texture_etc")]
pub struct WEBGL_compressed_texture_etc;

#[napi(js_name = "WEBGL_compressed_texture_etc1")]
pub struct WEBGL_compressed_texture_etc1;

#[napi(js_name = "WEBGL_compressed_texture_pvrtc")]
pub struct WEBGL_compressed_texture_pvrtc;

#[napi(js_name = "WEBGL_compressed_texture_s3tc")]
pub struct WEBGL_compressed_texture_s3tc;

#[napi(js_name = "WEBGL_lose_context", custom_finalize)]
pub struct WEBGL_lose_context(pub(crate) *const canvas_c::WEBGL_lose_context);

impl ObjectFinalize for WEBGL_lose_context {
    fn finalize(self, _: Env) -> Result<()> {
        canvas_c::canvas_native_webgl_WEBGL_lose_context_destroy(self.0 as _);
        Ok(())
    }
}

#[napi]
impl WEBGL_lose_context {
    pub fn lose_context(&self) {
        canvas_c::canvas_native_webgl_lose_context_lose_context(self.0)
    }

    pub fn restore_context(&self) {
        canvas_c::canvas_native_webgl_lose_context_restore_context(self.0)
    }
}

#[napi(js_name = "WEBGL_depth_texture")]
pub struct WEBGL_depth_texture;

#[napi(js_name = "WEBGL_draw_buffers", custom_finalize)]
pub struct WEBGL_draw_buffers(pub(crate) *const canvas_c::WEBGL_draw_buffers);

impl ObjectFinalize for WEBGL_draw_buffers {
    fn finalize(self, _: Env) -> Result<()> {
        canvas_c::canvas_native_webgl_WEBGL_draw_buffers_destroy(self.0 as _);
        Ok(())
    }
}

#[napi]
impl WEBGL_draw_buffers {
    #[napi]
    pub fn draw_buffers_webgl(&self, buffers: &[u32]) {
        canvas_c::canvas_native_webgl_draw_buffers_draw_buffers_webgl(buffers.as_ptr(), buffers.len(), self.0)
    }

    #[napi(getter)]
    pub fn get_color_attachment0_webgl(&self) -> u32 {
        gl_bindings::COLOR_ATTACHMENT0
    }

    #[napi(getter)]
    pub fn get_color_attachment1_webgl(&self) -> u32 { gl_bindings::COLOR_ATTACHMENT1 }

    #[napi(getter)]
    pub fn get_color_attachment2_webgl(&self) -> u32 { gl_bindings::COLOR_ATTACHMENT2 }

    #[napi(getter)]
    pub fn get_color_attachment3_webgl(&self) -> u32 { gl_bindings::COLOR_ATTACHMENT3 }

    #[napi(getter)]
    pub fn get_color_attachment4_webgl(&self) -> u32 { gl_bindings::COLOR_ATTACHMENT4 }

    #[napi(getter)]
    pub fn get_color_attachment5_webgl(&self) -> u32 { gl_bindings::COLOR_ATTACHMENT5 }

    #[napi(getter)]
    pub fn get_color_attachment6_webgl(&self) -> u32 { gl_bindings::COLOR_ATTACHMENT6 }

    #[napi(getter)]
    pub fn get_color_attachment7_webgl(&self) -> u32 { gl_bindings::COLOR_ATTACHMENT7 }

    #[napi(getter)]
    pub fn get_color_attachment8_webgl(&self) -> u32 { gl_bindings::COLOR_ATTACHMENT8 }

    #[napi(getter)]
    pub fn get_color_attachment9_webgl(&self) -> u32 { gl_bindings::COLOR_ATTACHMENT9 }

    #[napi(getter)]
    pub fn get_color_attachment10_webgl(&self) -> u32 { gl_bindings::COLOR_ATTACHMENT10 }

    #[napi(getter)]
    pub fn get_color_attachment11_webgl(&self) -> u32 { gl_bindings::COLOR_ATTACHMENT11 }

    #[napi(getter)]
    pub fn get_color_attachment12_webgl(&self) -> u32 { gl_bindings::COLOR_ATTACHMENT12 }

    #[napi(getter)]
    pub fn get_color_attachment13_webgl(&self) -> u32 { gl_bindings::COLOR_ATTACHMENT13 }

    #[napi(getter)]
    pub fn get_color_attachment14_webgl(&self) -> u32 { gl_bindings::COLOR_ATTACHMENT14 }

    #[napi(getter)]
    pub fn get_color_attachment15_webgl(&self) -> u32 { gl_bindings::COLOR_ATTACHMENT15 }

    #[napi(getter)]
    pub fn get_draw_buffer0_webgl(&self) -> u32 { gl_bindings::DRAW_BUFFER0 }
    #[napi(getter)]
    pub fn draw_buffer1_webgl(&self) -> u32 { gl_bindings::DRAW_BUFFER1 }
    #[napi(getter)]
    pub fn draw_buffer2_webgl(&self) -> u32 { gl_bindings::DRAW_BUFFER2 }
    #[napi(getter)]
    pub fn draw_buffer3_webgl(&self) -> u32 { gl_bindings::DRAW_BUFFER3 }
    #[napi(getter)]
    pub fn draw_buffer4_webgl(&self) -> u32 { gl_bindings::DRAW_BUFFER4 }
    #[napi(getter)]
    pub fn draw_buffer5_webgl(&self) -> u32 { gl_bindings::DRAW_BUFFER5 }
    #[napi(getter)]
    pub fn draw_buffer6_webgl(&self) -> u32 { gl_bindings::DRAW_BUFFER6 }
    #[napi(getter)]
    pub fn draw_buffer7_webgl(&self) -> u32 { gl_bindings::DRAW_BUFFER7 }
    #[napi(getter)]
    pub fn draw_buffer8_webgl(&self) -> u32 { gl_bindings::DRAW_BUFFER8 }
    #[napi(getter)]
    pub fn draw_buffer9_webgl(&self) -> u32 { gl_bindings::DRAW_BUFFER9 }
    #[napi(getter)]
    pub fn draw_buffer10_webgl(&self) -> u32 { gl_bindings::DRAW_BUFFER10 }
    #[napi(getter)]
    pub fn draw_buffer11_webgl(&self) -> u32 { gl_bindings::DRAW_BUFFER11 }
    #[napi(getter)]
    pub fn draw_buffer12_webgl(&self) -> u32 { gl_bindings::DRAW_BUFFER12 }
    #[napi(getter)]
    pub fn draw_buffer13_webgl(&self) -> u32 { gl_bindings::DRAW_BUFFER13 }
    #[napi(getter)]
    pub fn draw_buffer14_webgl(&self) -> u32 { gl_bindings::DRAW_BUFFER14 }
    #[napi(getter)]
    pub fn draw_buffer15_webgl(&self) -> u32 { gl_bindings::DRAW_BUFFER15 }
    #[napi(getter)]
    pub fn max_color_attachments_webgl(&self) -> u32 { gl_bindings::MAX_COLOR_ATTACHMENTS }
    #[napi(getter)]
    pub fn max_draw_buffers_webgl(&self) -> u32 { gl_bindings::MAX_DRAW_BUFFERS }
}












