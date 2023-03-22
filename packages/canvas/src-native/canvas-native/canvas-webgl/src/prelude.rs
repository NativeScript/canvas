#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::cell::{Ref, RefCell, RefMut};
use std::ffi::CString;
use std::os::raw::c_void;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::RawRwLock;

use canvas_core::gl::GLContext;

pub fn get_sdk_version() -> i32 {
    return 18;
}

/* Pixel storage modes */

pub const WEBGL_UNPACK_COLORSPACE_CONVERSION_WEBGL: u32 = 0x9243;

pub const WEBGL_UNPACK_FLIP_Y_WEBGL: u32 = 0x9240;

pub const WEBGL_UNPACK_PREMULTIPLY_ALPHA_WEBGL: u32 = 0x9241;

/* Pixel storage modes */

pub const WEBGL_BROWSER_DEFAULT_WEBGL: u32 = 0x9244;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum WebGLExtensionType {
    EXT_blend_minmax,
    EXT_color_buffer_half_float,
    EXT_disjoint_timer_query,
    EXT_sRGB,
    EXT_shader_texture_lod,
    EXT_texture_filter_anisotropic,
    OES_element_index_uint,
    OES_standard_derivatives,
    OES_texture_float,
    OES_texture_float_linear,
    OES_texture_half_float,
    OES_texture_half_float_linear,
    OES_vertex_array_object,
    WEBGL_color_buffer_float,
    WEBGL_compressed_texture_atc,
    WEBGL_compressed_texture_etc1,
    WEBGL_compressed_texture_s3tc,
    WEBGL_compressed_texture_s3tc_srgb,
    WEBGL_compressed_texture_etc,
    WEBGL_compressed_texture_pvrtc,
    WEBGL_lose_context,
    ANGLE_instanced_arrays,
    WEBGL_depth_texture,
    WEBGL_draw_buffers,
    None,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum HowToClear {
    // Skip clearing the backbuffer.
    Skipped,
    // Clear the backbuffer.
    JustClear, // Combine webgl.clear() API with the backbuffer clear, so webgl.clear()

    // doesn't have to call glClear() again.
    CombinedClear,
}

pub enum WebGLResult {
    Boolean(bool),
    I32Array(Vec<i32>),
    U32Array(Vec<u32>),
    F32Array(Vec<f32>),
    BooleanArray(Vec<bool>),
    U32(u32),
    I32(i32),
    F32(f32),
    String(CString),
    None,
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub enum WebGLVersion {
    V1,
    V2,
    NONE,
}

#[derive(Debug)]
struct WebGLStateInner {
    version: WebGLVersion,
    gl_context: GLContext,
    flip_y: bool,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: String,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
    is_canvas: bool,
    clear_stencil: i32,
    clear_color: [f32; 4],
    scissor_enabled: bool,
    clear_depth: f32,
    color_mask: [bool; 4],
    stencil_mask: u32,
    stencil_mask_back: u32,
    stencil_func_ref: i32,
    stencil_func_ref_back: i32,
    stencil_func_mask: u32,
    stencil_func_mask_back: u32,
    depth_mask: bool,
    unpack_colorspace_conversion_webgl: i32,
}

#[derive(Debug)]
pub struct WebGLState(Rc<RefCell<WebGLStateInner>>);

impl WebGLState {
    pub(crate) fn resized(&mut self) {
        //let mut state = self.0.write();
        //state.gl_context = GLContext::get_current();
    }

    fn get(&self) -> Ref<WebGLStateInner> {
        Ref::map(self.0.borrow(), |v| v)
    }

    fn get_mut(&self) -> RefMut<WebGLStateInner> {
        RefMut::map(self.0.borrow_mut(), |v| v)
    }

    pub fn new_with_context(context: GLContext, version: WebGLVersion) -> Self {
        let ctx = Self(Rc::new(RefCell::new(WebGLStateInner {
            version,
            alpha: true,
            antialias: true,
            depth: true,
            fail_if_major_performance_caveat: false,
            power_preference: "default".to_string(),
            premultiplied_alpha: true,
            preserve_drawing_buffer: false,
            stencil: false,
            desynchronized: false,
            xr_compatible: false,
            is_canvas: false,
            gl_context: context,
            clear_stencil: 0,
            clear_color: [0., 0., 0., 0.],
            scissor_enabled: false,
            clear_depth: 1.,
            color_mask: [true, true, true, true],
            stencil_mask: 0xFFFFFFFF,
            stencil_mask_back: 0xFFFFFFFF,
            stencil_func_ref: 0,
            stencil_func_ref_back: 0,
            stencil_func_mask: 0xFFFFFFFF,
            stencil_func_mask_back: 0xFFFFFFFF,
            depth_mask: true,
            flip_y: false,
            unpack_colorspace_conversion_webgl: WEBGL_BROWSER_DEFAULT_WEBGL as i32,
        })));

        ctx
    }

    pub fn new_with_context_attributes(
        context: GLContext,
        version: WebGLVersion,
        alpha: bool,
        antialias: bool,
        depth: bool,
        fail_if_major_performance_caveat: bool,
        power_preference: &str,
        premultiplied_alpha: bool,
        preserve_drawing_buffer: bool,
        stencil: bool,
        desynchronized: bool,
        xr_compatible: bool,
        is_canvas: bool,
    ) -> Self {
        let ctx = Self(Rc::new(RefCell::new(WebGLStateInner {
            version,
            alpha,
            antialias,
            depth,
            fail_if_major_performance_caveat,
            power_preference: power_preference.to_string(),
            premultiplied_alpha,
            preserve_drawing_buffer,
            stencil,
            desynchronized,
            xr_compatible,
            is_canvas,
            gl_context: context,
            clear_stencil: 0,
            clear_color: [0., 0., 0., 0.],
            scissor_enabled: false,
            clear_depth: 1.,
            color_mask: [true, true, true, true],
            stencil_mask: 0xFFFFFFFF,
            stencil_mask_back: 0xFFFFFFFF,
            stencil_func_ref: 0,
            stencil_func_ref_back: 0,
            stencil_func_mask: 0xFFFFFFFF,
            stencil_func_mask_back: 0xFFFFFFFF,
            depth_mask: true,
            flip_y: false,
            unpack_colorspace_conversion_webgl: WEBGL_BROWSER_DEFAULT_WEBGL as i32,
        })));

        ctx
    }

    // pub(crate) fn get_context(&self) -> &GLContext {
    //     &self.get().gl_context
    // }

    pub fn get_drawing_buffer_width(&self) -> i32 {
        self.get().gl_context.get_surface_width()
    }

    pub fn get_drawing_buffer_height(&self) -> i32 {
        self.get().gl_context.get_surface_height()
    }

    pub(crate) fn set_antialias(&self, value: bool) {
        self.get_mut().antialias = value;
    }

    pub(crate) fn set_alpha(&self, value: bool) {
        self.get_mut().alpha = value;
    }
    pub(crate) fn set_webgl_version(&self, version: WebGLVersion) {
        self.get_mut().version = version;
    }

    pub(crate) fn set_gl_context(&mut self, context: GLContext) {
        self.get_mut().gl_context = context;
    }
    pub fn get_webgl_version(&mut self) -> WebGLVersion {
        self.get().version
    }
    pub fn get_context_attributes(&self) -> ContextAttributes {
        let lock = self.get();
        ContextAttributes {
            alpha: lock.alpha,
            antialias: lock.antialias,
            depth: lock.depth,
            fail_if_major_performance_caveat: lock.fail_if_major_performance_caveat,
            power_preference: lock.power_preference.clone(),
            premultiplied_alpha: lock.premultiplied_alpha,
            preserve_drawing_buffer: lock.preserve_drawing_buffer,
            stencil: lock.stencil,
            desynchronized: lock.desynchronized,
            xr_compatible: lock.xr_compatible,
            is_canvas: lock.is_canvas,
        }
    }
    pub fn set_clear_color(&mut self, colors: [f32; 4]) {
        self.get_mut().clear_color = colors;
    }
    pub fn set_stencil_mask(&mut self, mask: u32) {
        self.get_mut().stencil_mask = mask;
    }
    pub fn set_stencil_mask_back(&mut self, mask: u32) {
        self.get_mut().stencil_mask_back = mask;
    }
    pub fn set_clear_depth(&mut self, depth: f32) {
        self.get_mut().clear_depth = depth;
    }
    pub fn set_clear_stencil(&mut self, stencil: i32) {
        self.get_mut().clear_stencil = stencil;
    }
    pub fn set_color_mask(&mut self, mask: [bool; 4]) {
        self.get_mut().color_mask = mask;
    }
    pub fn set_stencil_func_ref(&mut self, reference: i32) {
        self.get_mut().stencil_func_ref = reference;
    }
    pub fn set_stencil_func_ref_back(&mut self, reference: i32) {
        self.get_mut().stencil_func_ref_back = reference;
    }

    pub fn set_stencil_func_mask(&mut self, mask: u32) {
        self.get_mut().stencil_func_mask = mask;
    }

    pub fn set_stencil_func_mask_back(&mut self, mask: u32) {
        self.get_mut().stencil_func_mask_back = mask;
    }

    pub fn set_premultiplied_alpha(&mut self, premultiply: bool) {
        self.get_mut().premultiplied_alpha = premultiply;
    }

    pub fn set_unpack_colorspace_conversion_webgl(&mut self, color_space: i32) {
        self.get_mut().unpack_colorspace_conversion_webgl = color_space;
    }

    pub fn set_flip_y(&mut self, flip: bool) {
        self.get_mut().flip_y = flip;
    }

    pub fn get_alpha(&self) -> bool {
        self.get().alpha
    }

    pub fn get_antialias(&self) -> bool {
        self.get().antialias
    }

    pub fn get_depth(&self) -> bool {
        self.get().depth
    }

    pub fn get_fail_if_major_performance_caveat(&self) -> bool {
        self.get().fail_if_major_performance_caveat
    }

    pub fn get_power_preference(&self) -> String {
        self.get().power_preference.clone()
    }

    pub fn get_premultiplied_alpha(&self) -> bool {
        self.get().premultiplied_alpha
    }

    pub fn get_preserve_drawing_buffer(&self) -> bool {
        self.get().preserve_drawing_buffer
    }

    pub fn get_stencil(&self) -> bool {
        self.get().stencil
    }

    pub fn get_desynchronized(&self) -> bool {
        self.get().desynchronized
    }

    pub fn get_xr_compatible(&self) -> bool {
        self.get().xr_compatible
    }

    pub fn get_scissor_enabled(&self) -> bool {
        self.get().scissor_enabled
    }

    pub fn get_depth_mask(&self) -> bool {
        self.get().depth_mask
    }

    pub fn get_clear_stencil(&self) -> i32 {
        self.get().clear_stencil
    }

    pub fn get_stencil_mask(&self) -> u32 {
        self.get().stencil_mask
    }

    pub fn get_clear_color(&self) -> [f32; 4] {
        self.get().clear_color
    }

    pub fn get_color_mask(&self) -> [bool; 4] {
        self.get().color_mask
    }

    pub fn get_clear_depth(&self) -> f32 {
        self.get().clear_depth
    }

    pub fn get_flip_y(&self) -> bool {
        self.get().flip_y
    }

    pub fn get_unpack_colorspace_conversion_webgl(&self) -> i32 {
        self.get().unpack_colorspace_conversion_webgl
    }

    pub fn drawing_buffer_width(&self) -> i32 {
        self.get().gl_context.get_surface_width()
    }

    pub fn drawing_buffer_height(&self) -> i32 {
        self.get().gl_context.get_surface_height()
    }

    pub fn make_current(&self) -> bool {
        self.get().gl_context.make_current()
    }

    pub fn swap_buffers(&self) -> bool {
        self.get().gl_context.swap_buffers()
    }

    pub fn remove_if_current(&self) {
        self.get().gl_context.remove_if_current();
    }
}

impl Clone for WebGLState {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }

    fn clone_from(&mut self, source: &Self) {
        self.0 = Rc::clone(&source.0)
    }
}

impl Default for WebGLState {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(WebGLStateInner {
            version: WebGLVersion::NONE,
            alpha: true,
            antialias: true,
            depth: true,
            fail_if_major_performance_caveat: false,
            power_preference: "default".to_string(),
            premultiplied_alpha: true,
            preserve_drawing_buffer: false,
            stencil: false,
            desynchronized: false,
            xr_compatible: false,
            is_canvas: false,
            gl_context: Default::default(),
            clear_stencil: 0,
            clear_color: [0., 0., 0., 0.],
            scissor_enabled: false,
            clear_depth: 1.,
            color_mask: [true, true, true, true],
            stencil_mask: 0xFFFFFFFF,
            stencil_mask_back: 0xFFFFFFFF,
            stencil_func_ref: 0,
            stencil_func_ref_back: 0,
            stencil_func_mask: 0xFFFFFFFF,
            stencil_func_mask_back: 0xFFFFFFFF,
            depth_mask: true,
            flip_y: false,
            unpack_colorspace_conversion_webgl: WEBGL_BROWSER_DEFAULT_WEBGL as i32,
        })))
    }
}

#[derive(Clone)]
pub struct WebGLActiveInfo {
    name: String,
    size: i32,
    info_type: u32,
    is_empty: bool,
}

impl WebGLActiveInfo {
    pub fn new(name: String, size: i32, info_type: u32) -> Self {
        Self {
            name,
            size,
            info_type,
            is_empty: false,
        }
    }

    pub fn empty() -> Self {
        Self {
            name: String::new(),
            size: 0,
            info_type: 0,
            is_empty: true,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn get_type(&self) -> u32 {
        self.info_type
    }

    pub fn get_is_empty(&self) -> bool {
        self.is_empty
    }
}

#[derive(Copy, Clone)]
pub struct WebGLShaderPrecisionFormat {
    precision: i32,
    range_min: i32,
    range_max: i32,
}

impl WebGLShaderPrecisionFormat {
    pub fn new(precision: i32, range_min: i32, range_max: i32) -> Self {
        Self {
            precision,
            range_min,
            range_max,
        }
    }

    pub fn get_precision(&self) -> i32 {
        self.precision
    }

    pub fn get_range_min(&self) -> i32 {
        self.range_min
    }

    pub fn get_range_max(&self) -> i32 {
        self.range_max
    }
}

#[derive(Copy, Clone)]
pub struct WebGLFramebufferAttachmentParameter {
    pub(crate) is_texture: bool,
    pub(crate) is_renderbuffer: bool,
    pub(crate) value: i32,
}

impl WebGLFramebufferAttachmentParameter {
    pub fn new(is_texture: bool, is_renderbuffer: bool, value: i32) -> Self {
        Self {
            is_texture,
            is_renderbuffer,
            value,
        }
    }

    pub fn get_is_texture(&self) -> bool {
        self.is_texture
    }

    pub fn get_is_renderbuffer(&self) -> bool {
        self.is_renderbuffer
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

impl Default for WebGLFramebufferAttachmentParameter {
    fn default() -> Self {
        Self {
            is_texture: false,
            is_renderbuffer: false,
            value: 0,
        }
    }
}

pub struct ContextAttributes {
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: String,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
    is_canvas: bool,
}

impl ContextAttributes {
    pub fn new(
        alpha: bool,
        antialias: bool,
        depth: bool,
        fail_if_major_performance_caveat: bool,
        power_preference: &str,
        premultiplied_alpha: bool,
        preserve_drawing_buffer: bool,
        stencil: bool,
        desynchronized: bool,
        xr_compatible: bool,
        is_canvas: bool,
    ) -> Self {
        Self {
            alpha,
            antialias,
            depth,
            fail_if_major_performance_caveat,
            power_preference: power_preference.to_string(),
            premultiplied_alpha,
            preserve_drawing_buffer,
            stencil,
            desynchronized,
            xr_compatible,
            is_canvas,
        }
    }

    pub fn get_is_canvas(&self) -> bool {
        self.is_canvas
    }
    pub fn get_alpha(&self) -> bool {
        self.alpha
    }
    pub fn get_antialias(&self) -> bool {
        self.antialias
    }
    pub fn get_depth(&self) -> bool {
        self.depth
    }
    pub fn get_fail_if_major_performance_caveat(&self) -> bool {
        self.fail_if_major_performance_caveat
    }
    pub fn get_power_preference(&self) -> String {
        self.power_preference.clone()
    }
    pub fn get_premultiplied_alpha(&self) -> bool {
        self.premultiplied_alpha
    }
    pub fn get_preserve_drawing_buffer(&self) -> bool {
        self.preserve_drawing_buffer
    }
    pub fn get_stencil(&self) -> bool {
        self.stencil
    }
    pub fn get_desynchronized(&self) -> bool {
        self.desynchronized
    }
    pub fn get_xr_compatible(&self) -> bool {
        self.xr_compatible
    }

    pub fn set_alpha(&mut self, value: bool) {
        self.alpha = value;
    }
    pub fn set_antialias(&mut self, value: bool) {
        self.antialias = value;
    }
    pub fn set_depth(&mut self, value: bool) {
        self.depth = value;
    }
    pub fn set_fail_if_major_performance_caveat(&mut self, value: bool) {
        self.fail_if_major_performance_caveat = value;
    }

    pub fn set_power_preference(&mut self, value: &str) {
        self.power_preference = value.to_string();
    }

    pub fn set_premultiplied_alpha(&mut self, value: bool) {
        self.premultiplied_alpha = value;
    }

    pub fn set_preserve_drawing_buffer(&mut self, value: bool) {
        self.preserve_drawing_buffer = value;
    }
    pub fn set_stencil(&mut self, value: bool) {
        self.stencil = value;
    }

    pub fn set_desynchronized(&mut self, value: bool) {
        self.desynchronized = value;
    }

    pub fn set_xr_compatible(&mut self, value: bool) {
        self.xr_compatible = value;
    }
}

pub trait WebGLExtension {
    fn extension_type(&self) -> WebGLExtensionType;
}

pub struct EXT_blend_minmax {
    min_ext: u32,
    max_ext: u32,
}

impl EXT_blend_minmax {
    pub fn new() -> Self {
        let min_ext = gl_bindings::MIN;
        let max_ext = gl_bindings::MAX;
        Self { min_ext, max_ext }
    }
    pub fn get_min_ext(&self) -> u32 {
        self.min_ext
    }

    pub fn get_max_ext(&self) -> u32 {
        self.max_ext
    }
}

impl WebGLExtension for EXT_blend_minmax {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::EXT_blend_minmax
    }
}

pub struct EXT_color_buffer_half_float {
    rgba16f_ext: u32,
    rgb16f_ext: u32,
    framebuffer_attachment_component_type_ext: u32,
    unsigned_normalized_ext: u32,
}

impl EXT_color_buffer_half_float {
    pub fn new() -> Self {
        let rgba16f_ext = gl_bindings::RGBA16F;
        let rgb16f_ext = gl_bindings::RGB16F;
        let framebuffer_attachment_component_type_ext =
            gl_bindings::FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE;
        let unsigned_normalized_ext = gl_bindings::UNSIGNED_NORMALIZED;
        Self {
            rgba16f_ext,
            rgb16f_ext,
            framebuffer_attachment_component_type_ext,
            unsigned_normalized_ext,
        }
    }

    pub fn get_rgba16f_ext(&self) -> u32 {
        self.rgba16f_ext
    }

    pub fn get_rgb16f_ext(&self) -> u32 {
        self.rgb16f_ext
    }

    pub fn get_framebuffer_attachment_component_type_ext(&self) -> u32 {
        self.framebuffer_attachment_component_type_ext
    }

    pub fn get_unsigned_normalized_ext(&self) -> u32 {
        self.unsigned_normalized_ext
    }
}

impl WebGLExtension for EXT_color_buffer_half_float {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::EXT_color_buffer_half_float
    }
}

#[derive(Clone)]
pub struct EXT_disjoint_timer_query {
    webgl_state: WebGLState,
    query_counter_bits_ext: u32,
    current_query_ext: u32,
    query_result_ext: u32,
    query_result_available_ext: u32,
    time_elapsed_ext: u32,
    timestamp_ext: u32,
    gpu_disjoint_ext: u32,
}

impl EXT_disjoint_timer_query {
    pub fn new(state: WebGLState) -> Self {
        let query_counter_bits_ext = gl_bindings::QUERY_COUNTER_BITS_EXT;
        let current_query_ext = gl_bindings::CURRENT_QUERY;
        let query_result_ext = gl_bindings::QUERY_RESULT;
        let query_result_available_ext = gl_bindings::QUERY_RESULT_AVAILABLE;
        let time_elapsed_ext = 0x88BF;
        let timestamp_ext = 0x88BF;
        let gpu_disjoint_ext = 0x8FBB;

        Self {
            webgl_state: state,
            query_counter_bits_ext,
            current_query_ext,
            query_result_ext,
            query_result_available_ext,
            time_elapsed_ext,
            timestamp_ext,
            gpu_disjoint_ext,
        }
    }

    pub fn create_query_ext(&self) -> u32 {
        self.webgl_state.make_current();
        let mut query = [0u32; 1];
        unsafe { gl_bindings::GenQueries(1, query.as_mut_ptr()) }
        query[0]
    }

    pub fn delete_query_ext(&self, query: u32) {
        self.webgl_state.make_current();
        let query = [query];
        unsafe { gl_bindings::DeleteQueries(1, query.as_ptr()) }
    }

    pub fn is_query_ext(&self, query: u32) -> bool {
        self.webgl_state.make_current();
        unsafe { gl_bindings::IsQuery(query) == 1 }
    }

    pub fn begin_query_ext(&self, target: u32, query: u32) {
        self.webgl_state.make_current();
        unsafe { gl_bindings::BeginQuery(target, query) }
    }

    pub fn end_query_ext(&self, target: u32) {
        self.webgl_state.make_current();
        unsafe { gl_bindings::EndQuery(target) }
    }

    pub fn query_counter_ext(&self, _query: u32, _target: u32) {
        // noop
    }

    pub fn get_query_ext(&self, target: u32, pname: u32) -> i32 {
        let mut query = [0i32; 1];
        unsafe { gl_bindings::GetQueryiv(target, pname, query.as_mut_ptr()) }
        return query[0];
    }

    pub fn get_query_object_ext(&self, target: u32, pname: u32) -> WebGLResult {
        let mut query = [0i32; 1];
        unsafe { gl_bindings::GetQueryiv(target, pname, query.as_mut_ptr()) }
        if pname == gl_bindings::QUERY_RESULT_AVAILABLE {
            return WebGLResult::Boolean(query[0] == 1);
        }
        WebGLResult::I32(query[0])
    }

    pub fn get_query_counter_bits_ext(&self) -> u32 {
        self.query_counter_bits_ext
    }

    pub fn get_current_query_ext(&self) -> u32 {
        self.current_query_ext
    }

    pub fn get_query_result_ext(&self) -> u32 {
        self.query_result_ext
    }

    pub fn get_query_result_available_ext(&self) -> u32 {
        self.query_result_available_ext
    }

    pub fn get_time_elapsed_ext(&self) -> u32 {
        self.time_elapsed_ext
    }

    pub fn get_timestamp_ext(&self) -> u32 {
        self.timestamp_ext
    }

    pub fn get_gpu_disjoint_ext(&self) -> u32 {
        self.gpu_disjoint_ext
    }
}

impl WebGLExtension for EXT_disjoint_timer_query {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::EXT_disjoint_timer_query
    }
}

pub struct EXT_sRGB {
    srgb_ext: u32,
    srgb_alpha_ext: u32,
    srgb8_alpha8_ext: u32,
    framebuffer_attachment_color_encoding_ext: u32,
}

impl EXT_sRGB {
    pub fn new() -> Self {
        let srgb_ext = gl_bindings::SRGB_EXT;
        let srgb_alpha_ext = gl_bindings::SRGB_ALPHA_EXT;
        let srgb8_alpha8_ext = gl_bindings::SRGB8_ALPHA8;
        let framebuffer_attachment_color_encoding_ext =
            gl_bindings::FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT;
        Self {
            srgb_ext,
            srgb_alpha_ext,
            srgb8_alpha8_ext,
            framebuffer_attachment_color_encoding_ext,
        }
    }
}

impl WebGLExtension for EXT_sRGB {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::EXT_sRGB
    }
}

pub struct EXT_shader_texture_lod {}

impl EXT_shader_texture_lod {
    pub fn new() -> Self {
        Self {}
    }
}

impl WebGLExtension for EXT_shader_texture_lod {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::EXT_shader_texture_lod
    }
}

pub struct EXT_texture_filter_anisotropic {
    max_texture_max_anisotropy_ext: u32,
    texture_max_anisotropy_ext: u32,
}

impl EXT_texture_filter_anisotropic {
    pub fn new() -> Self {
        let max_texture_max_anisotropy_ext = 0x84FF;
        let texture_max_anisotropy_ext = 0x84FE;
        Self {
            max_texture_max_anisotropy_ext,
            texture_max_anisotropy_ext,
        }
    }
}

impl WebGLExtension for EXT_texture_filter_anisotropic {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::EXT_texture_filter_anisotropic
    }
}

pub struct OES_element_index_uint {
    unsigned_int: u32,
}

impl OES_element_index_uint {
    pub fn new() -> Self {
        Self {
            unsigned_int: gl_bindings::UNSIGNED_INT,
        }
    }
}

impl WebGLExtension for OES_element_index_uint {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::OES_element_index_uint
    }
}

pub struct OES_standard_derivatives {
    fragment_shader_derivative_hint_oes: u32,
}

impl OES_standard_derivatives {
    pub fn new() -> Self {
        Self {
            fragment_shader_derivative_hint_oes: gl_bindings::FRAGMENT_SHADER_DERIVATIVE_HINT,
        }
    }
}

impl WebGLExtension for OES_standard_derivatives {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::OES_standard_derivatives
    }
}

pub struct OES_texture_float {}

impl OES_texture_float {
    pub fn new() -> Self {
        Self {}
    }
}

impl WebGLExtension for OES_texture_float {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::OES_texture_float
    }
}

pub struct OES_texture_float_linear {}

impl OES_texture_float_linear {
    pub fn new() -> Self {
        Self {}
    }
}

impl WebGLExtension for OES_texture_float_linear {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::OES_texture_float_linear
    }
}

pub struct OES_texture_half_float {
    half_float_oes: u32,
}

impl OES_texture_half_float {
    pub fn new() -> Self {
        Self {
            half_float_oes: gl_bindings::HALF_FLOAT,
        }
    }
}

impl WebGLExtension for OES_texture_half_float {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::OES_texture_half_float
    }
}

pub struct OES_texture_half_float_linear {}

impl OES_texture_half_float_linear {
    pub fn new() -> Self {
        Self {}
    }
}

impl WebGLExtension for OES_texture_half_float_linear {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::OES_texture_half_float_linear
    }
}

#[derive(Clone)]
pub struct OES_vertex_array_object {
    webgl_state: WebGLState,
    vertex_array_binding_oes: u32,
}

impl OES_vertex_array_object {
    pub fn new(state: WebGLState) -> Self {
        Self {
            webgl_state: state,
            vertex_array_binding_oes: gl_bindings::VERTEX_ARRAY_BINDING,
        }
    }

    pub fn create_vertex_array_oes(&self) -> u32 {
        self.webgl_state.make_current();
        let mut array = [0u32; 1];
        unsafe { gl_bindings::GenVertexArrays(1, array.as_mut_ptr()) };
        return array[0];
    }

    pub fn delete_vertex_array_oes(&self, array_object: u32) {
        self.webgl_state.make_current();
        let array = [array_object];
        unsafe { gl_bindings::DeleteVertexArrays(1, array.as_ptr()) };
    }

    pub fn is_vertex_array_oes(&self, array_object: u32) -> bool {
        self.webgl_state.make_current();
        unsafe { gl_bindings::IsVertexArray(array_object) != 0 }
    }

    pub fn bind_vertex_array_oes(&self, array_object: u32) {
        self.webgl_state.make_current();
        unsafe { gl_bindings::BindVertexArray(array_object) }
    }
}

impl WebGLExtension for OES_vertex_array_object {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::OES_vertex_array_object
    }
}

pub struct WEBGL_color_buffer_float {
    rgba32f_ext: u32,
    rgb32f_ext: u32,
    framebuffer_attachment_component_type_ext: u32,
    unsigned_normalized_ext: u32,
}

impl WEBGL_color_buffer_float {
    pub fn new() -> Self {
        let rgba32f_ext = gl_bindings::RGBA32F;
        let rgb32f_ext = gl_bindings::RGB32F;
        let framebuffer_attachment_component_type_ext =
            gl_bindings::FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE;
        let unsigned_normalized_ext = gl_bindings::UNSIGNED_NORMALIZED;
        Self {
            rgba32f_ext,
            rgb32f_ext,
            framebuffer_attachment_component_type_ext,
            unsigned_normalized_ext,
        }
    }
}

impl WebGLExtension for WEBGL_color_buffer_float {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::WEBGL_color_buffer_float
    }
}

pub struct WEBGL_compressed_texture_atc {
    compressed_rgb_atc_webgl: u32,
    compressed_rgba_atc_explicit_alpha_webgl: u32,
    compressed_rgba_atc_interpolated_alpha_webgl: u32,
}

impl WEBGL_compressed_texture_atc {
    pub fn new() -> Self {
        Self {
            compressed_rgb_atc_webgl: gl_bindings::ATC_RGB_AMD,
            compressed_rgba_atc_explicit_alpha_webgl: gl_bindings::ATC_RGBA_EXPLICIT_ALPHA_AMD,
            compressed_rgba_atc_interpolated_alpha_webgl:
                gl_bindings::ATC_RGBA_INTERPOLATED_ALPHA_AMD,
        }
    }
}

impl WebGLExtension for WEBGL_compressed_texture_atc {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::WEBGL_compressed_texture_atc
    }
}

pub struct WEBGL_compressed_texture_etc1 {
    compressed_rgb_etc1_webgl: u32,
}

impl WEBGL_compressed_texture_etc1 {
    pub fn new() -> Self {
        Self {
            compressed_rgb_etc1_webgl: gl_bindings::ETC1_RGB8_OES,
        }
    }
}

impl WebGLExtension for WEBGL_compressed_texture_etc1 {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::WEBGL_compressed_texture_etc1
    }
}

pub struct WEBGL_compressed_texture_s3tc {
    compressed_rgb_s3tc_dxt1_ext: u32,
    compressed_rgba_s3tc_dxt1_ext: u32,
    compressed_rgba_s3tc_dxt3_ext: u32,
    compressed_rgba_s3tc_dxt5_ext: u32,
}

impl WEBGL_compressed_texture_s3tc {
    pub fn new() -> Self {
        Self {
            compressed_rgb_s3tc_dxt1_ext: gl_bindings::COMPRESSED_RGB_S3TC_DXT1_EXT,
            compressed_rgba_s3tc_dxt1_ext: gl_bindings::COMPRESSED_RGBA_S3TC_DXT1_EXT,
            compressed_rgba_s3tc_dxt3_ext: gl_bindings::COMPRESSED_RGBA_S3TC_DXT3_EXT,
            compressed_rgba_s3tc_dxt5_ext: gl_bindings::COMPRESSED_RGBA_S3TC_DXT5_EXT,
        }
    }
}

impl WebGLExtension for WEBGL_compressed_texture_s3tc {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::WEBGL_compressed_texture_s3tc
    }
}

pub struct WEBGL_compressed_texture_s3tc_srgb {
    compressed_srgb_s3tc_dxt1_ext: u32,
    compressed_srgb_alpha_s3tc_dxt1_ext: u32,
    compressed_srgb_alpha_s3tc_dxt3_ext: u32,
    compressed_srgb_alpha_s3tc_dxt5_ext: u32,
}

impl WEBGL_compressed_texture_s3tc_srgb {
    pub fn new() -> Self {
        Self {
            compressed_srgb_s3tc_dxt1_ext: gl_bindings::COMPRESSED_SRGB_S3TC_DXT1_EXT,
            compressed_srgb_alpha_s3tc_dxt1_ext: gl_bindings::COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT,
            compressed_srgb_alpha_s3tc_dxt3_ext: gl_bindings::COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT,
            compressed_srgb_alpha_s3tc_dxt5_ext: gl_bindings::COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT,
        }
    }
}

impl WebGLExtension for WEBGL_compressed_texture_s3tc_srgb {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::WEBGL_compressed_texture_s3tc_srgb
    }
}

pub struct WEBGL_compressed_texture_etc {
    compressed_r11_eac: u32,
    compressed_signed_r11_eac: u32,
    compressed_rg11_eac: u32,
    compressed_signed_rg11_eac: u32,
    compressed_rgb8_etc2: u32,
    compressed_rgba8_etc2_eac: u32,
    compressed_srgb8_etc2: u32,
    compressed_srgb8_alpha8_etc2_eac: u32,
    compressed_rgb8_punchthrough_alpha1_etc2: u32,
    compressed_srgb8_punchthrough_alpha1_etc2: u32,
}

impl WEBGL_compressed_texture_etc {
    pub fn new() -> Self {
        let compressed_r11_eac = gl_bindings::COMPRESSED_R11_EAC;
        let compressed_signed_r11_eac = gl_bindings::COMPRESSED_SIGNED_R11_EAC;
        let compressed_rg11_eac = gl_bindings::COMPRESSED_RG11_EAC;
        let compressed_signed_rg11_eac = gl_bindings::COMPRESSED_SIGNED_RG11_EAC;
        let compressed_rgb8_etc2 = gl_bindings::COMPRESSED_RGB8_ETC2;
        let compressed_rgba8_etc2_eac = gl_bindings::COMPRESSED_RGBA8_ETC2_EAC;
        let compressed_srgb8_etc2 = gl_bindings::COMPRESSED_SRGB8_ETC2;
        let compressed_srgb8_alpha8_etc2_eac = gl_bindings::COMPRESSED_SRGB8_ALPHA8_ETC2_EAC;
        let compressed_rgb8_punchthrough_alpha1_etc2 =
            gl_bindings::COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2;
        let compressed_srgb8_punchthrough_alpha1_etc2 =
            gl_bindings::COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2;

        Self {
            compressed_r11_eac,
            compressed_signed_r11_eac,
            compressed_rg11_eac,
            compressed_signed_rg11_eac,
            compressed_rgb8_etc2,
            compressed_rgba8_etc2_eac,
            compressed_srgb8_etc2,
            compressed_srgb8_alpha8_etc2_eac,
            compressed_rgb8_punchthrough_alpha1_etc2,
            compressed_srgb8_punchthrough_alpha1_etc2,
        }
    }
}

impl WebGLExtension for WEBGL_compressed_texture_etc {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::WEBGL_compressed_texture_etc
    }
}

pub struct WEBGL_compressed_texture_pvrtc {
    compressed_rgb_pvrtc_4bppv1_img: u32,
    compressed_rgba_pvrtc_4bppv1_img: u32,
    compressed_rgb_pvrtc_2bppv1_img: u32,
    compressed_rgba_pvrtc_2bppv1_img: u32,
}

impl WEBGL_compressed_texture_pvrtc {
    pub fn new() -> Self {
        Self {
            compressed_rgb_pvrtc_4bppv1_img: gl_bindings::COMPRESSED_RGB_PVRTC_4BPPV1_IMG,
            compressed_rgba_pvrtc_4bppv1_img: gl_bindings::COMPRESSED_RGBA_PVRTC_4BPPV1_IMG,
            compressed_rgb_pvrtc_2bppv1_img: gl_bindings::COMPRESSED_RGB_PVRTC_2BPPV1_IMG,
            compressed_rgba_pvrtc_2bppv1_img: gl_bindings::COMPRESSED_RGBA_PVRTC_2BPPV1_IMG,
        }
    }
}

impl WebGLExtension for WEBGL_compressed_texture_pvrtc {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::WEBGL_compressed_texture_pvrtc
    }
}

#[derive(Clone)]
pub struct WEBGL_lose_context {
    webgl_state: WebGLState,
}

impl WEBGL_lose_context {
    pub fn new(state: WebGLState) -> Self {
        Self { webgl_state: state }
    }
    pub fn lose_context(&self) {
        self.webgl_state.remove_if_current();
    }

    pub fn restore_context(&self) {
        self.webgl_state.make_current();
    }
}

impl WebGLExtension for WEBGL_lose_context {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::WEBGL_lose_context
    }
}

#[derive(Clone)]
pub struct ANGLE_instanced_arrays {
    webgl_state: WebGLState,
    vertex_attrib_array_divisor_angle: u32,
}

impl ANGLE_instanced_arrays {
    pub fn new(state: WebGLState) -> Self {
        Self {
            webgl_state: state,
            vertex_attrib_array_divisor_angle: gl_bindings::VERTEX_ATTRIB_ARRAY_DIVISOR,
        }
    }
    pub fn draw_arrays_instanced_angle(&self, mode: u32, first: i32, count: i32, primcount: i32) {
        self.webgl_state.make_current();
        unsafe { gl_bindings::DrawArraysInstanced(mode, first, count, primcount) }
    }

    pub fn draw_elements_instanced_angle(
        &self,
        mode: u32,
        count: i32,
        type_: u32,
        offset: i32,
        primcount: i32,
    ) {
        self.webgl_state.make_current();
        unsafe {
            gl_bindings::DrawElementsInstanced(
                mode,
                count,
                type_,
                offset as *const c_void,
                primcount,
            )
        }
    }

    pub fn vertex_attrib_divisor_angle(&self, index: u32, divisor: u32) {
        self.webgl_state.make_current();
        unsafe { gl_bindings::VertexAttribDivisor(index, divisor) }
    }
}

impl WebGLExtension for ANGLE_instanced_arrays {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::ANGLE_instanced_arrays
    }
}

pub struct WEBGL_depth_texture {
    unsigned_int_24_8_webgl: u32,
}

impl WEBGL_depth_texture {
    pub fn new() -> Self {
        Self {
            unsigned_int_24_8_webgl: 0x84FA,
        }
    }
}

impl WebGLExtension for WEBGL_depth_texture {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::WEBGL_depth_texture
    }
}

#[derive(Clone)]
pub struct WEBGL_draw_buffers {
    webgl_state: WebGLState,
    color_attachment0_webgl: u32,
    color_attachment1_webgl: u32,
    color_attachment2_webgl: u32,
    color_attachment3_webgl: u32,
    color_attachment4_webgl: u32,
    color_attachment5_webgl: u32,
    color_attachment6_webgl: u32,
    color_attachment7_webgl: u32,
    color_attachment8_webgl: u32,
    color_attachment9_webgl: u32,
    color_attachment10_webgl: u32,
    color_attachment11_webgl: u32,
    color_attachment12_webgl: u32,
    color_attachment13_webgl: u32,
    color_attachment14_webgl: u32,
    color_attachment15_webgl: u32,
    draw_buffer0_webgl: u32,
    draw_buffer1_webgl: u32,
    draw_buffer2_webgl: u32,
    draw_buffer3_webgl: u32,
    draw_buffer4_webgl: u32,
    draw_buffer5_webgl: u32,
    draw_buffer6_webgl: u32,
    draw_buffer7_webgl: u32,
    draw_buffer8_webgl: u32,
    draw_buffer9_webgl: u32,
    draw_buffer10_webgl: u32,
    draw_buffer11_webgl: u32,
    draw_buffer12_webgl: u32,
    draw_buffer13_webgl: u32,
    draw_buffer14_webgl: u32,
    draw_buffer15_webgl: u32,
    max_color_attachments_webgl: u32,
    max_draw_buffers_webgl: u32,
}

impl WEBGL_draw_buffers {
    pub fn new(state: WebGLState) -> Self {
        Self {
            webgl_state: state,
            color_attachment0_webgl: gl_bindings::COLOR_ATTACHMENT0,
            color_attachment1_webgl: gl_bindings::COLOR_ATTACHMENT1,
            color_attachment2_webgl: gl_bindings::COLOR_ATTACHMENT2,
            color_attachment3_webgl: gl_bindings::COLOR_ATTACHMENT3,
            color_attachment4_webgl: gl_bindings::COLOR_ATTACHMENT4,
            color_attachment5_webgl: gl_bindings::COLOR_ATTACHMENT5,
            color_attachment6_webgl: gl_bindings::COLOR_ATTACHMENT6,
            color_attachment7_webgl: gl_bindings::COLOR_ATTACHMENT7,
            color_attachment8_webgl: gl_bindings::COLOR_ATTACHMENT8,
            color_attachment9_webgl: gl_bindings::COLOR_ATTACHMENT9,
            color_attachment10_webgl: gl_bindings::COLOR_ATTACHMENT10,
            color_attachment11_webgl: gl_bindings::COLOR_ATTACHMENT11,
            color_attachment12_webgl: gl_bindings::COLOR_ATTACHMENT12,
            color_attachment13_webgl: gl_bindings::COLOR_ATTACHMENT13,
            color_attachment14_webgl: gl_bindings::COLOR_ATTACHMENT14,
            color_attachment15_webgl: gl_bindings::COLOR_ATTACHMENT15,
            draw_buffer0_webgl: gl_bindings::DRAW_BUFFER0,
            draw_buffer1_webgl: gl_bindings::DRAW_BUFFER1,
            draw_buffer2_webgl: gl_bindings::DRAW_BUFFER2,
            draw_buffer3_webgl: gl_bindings::DRAW_BUFFER3,
            draw_buffer4_webgl: gl_bindings::DRAW_BUFFER4,
            draw_buffer5_webgl: gl_bindings::DRAW_BUFFER5,
            draw_buffer6_webgl: gl_bindings::DRAW_BUFFER6,
            draw_buffer7_webgl: gl_bindings::DRAW_BUFFER7,
            draw_buffer8_webgl: gl_bindings::DRAW_BUFFER8,
            draw_buffer9_webgl: gl_bindings::DRAW_BUFFER9,
            draw_buffer10_webgl: gl_bindings::DRAW_BUFFER10,
            draw_buffer11_webgl: gl_bindings::DRAW_BUFFER11,
            draw_buffer12_webgl: gl_bindings::DRAW_BUFFER12,
            draw_buffer13_webgl: gl_bindings::DRAW_BUFFER13,
            draw_buffer14_webgl: gl_bindings::DRAW_BUFFER14,
            draw_buffer15_webgl: gl_bindings::DRAW_BUFFER15,
            max_color_attachments_webgl: gl_bindings::MAX_COLOR_ATTACHMENTS,
            max_draw_buffers_webgl: gl_bindings::MAX_DRAW_BUFFERS,
        }
    }

    pub fn draw_buffers_webgl(&self, buffers: &[u32]) {
        self.webgl_state.make_current();
        unsafe { gl_bindings::DrawBuffers(buffers.len().try_into().unwrap(), buffers.as_ptr()) }
    }
}

impl WebGLExtension for WEBGL_draw_buffers {
    fn extension_type(&self) -> WebGLExtensionType {
        WebGLExtensionType::WEBGL_draw_buffers
    }
}

#[derive(Copy, Clone)]
pub struct WebGLIndexedParameter {
    pub(crate) is_buffer: bool,
    pub(crate) buffer_value: isize,
    pub(crate) value: isize,
}

impl WebGLIndexedParameter {
    pub fn new(is_buffer: bool, buffer_value: isize, value: isize) -> Self {
        Self {
            is_buffer,
            buffer_value,
            value,
        }
    }

    pub fn get_is_buffer(&self) -> bool {
        self.is_buffer
    }

    pub fn get_buffer_value(&self) -> isize {
        self.buffer_value
    }

    pub fn get_value(&self) -> isize {
        self.value
    }
}

impl Default for WebGLIndexedParameter {
    fn default() -> Self {
        Self {
            is_buffer: false,
            buffer_value: 0,
            value: -1,
        }
    }
}
