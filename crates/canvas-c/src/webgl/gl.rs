/* GL */
use canvas_2d::utils::image::from_image_slice;
use canvas_core::context_attributes::PowerPreference;
use canvas_core::gpu::gl::GLContext;
use canvas_webgl::prelude::WebGLVersion;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::ptr::NonNull;

use crate::buffers::{F32Buffer, I32Buffer, StringBuffer, U32Buffer, U8Buffer};
use crate::c2d::CanvasRenderingContext2D;
use crate::c2d::PaintStyle;
use crate::enums::CanvasRepetition;
use crate::image_asset::ImageAsset;
use crate::webgl::result::WebGLResultType;

/* GL */

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
#[repr(C)]
pub enum WebGLExtensionType {
    WebGLExtensionTypeEXT_blend_minmax,
    WebGLExtensionTypeEXT_color_buffer_half_float,
    WebGLExtensionTypeEXT_disjoint_timer_query,
    WebGLExtensionTypeEXT_sRGB,
    WebGLExtensionTypeEXT_shader_texture_lod,
    WebGLExtensionTypeEXT_texture_filter_anisotropic,
    WebGLExtensionTypeOES_element_index_uint,
    WebGLExtensionTypeOES_standard_derivatives,
    WebGLExtensionTypeOES_texture_float,
    WebGLExtensionTypeOES_texture_float_linear,
    WebGLExtensionTypeOES_texture_half_float,
    WebGLExtensionTypeOES_texture_half_float_linear,
    WebGLExtensionTypeOES_vertex_array_object,
    WebGLExtensionTypeWEBGL_color_buffer_float,
    WebGLExtensionTypeWEBGL_compressed_texture_atc,
    WebGLExtensionTypeWEBGL_compressed_texture_etc1,
    WebGLExtensionTypeWEBGL_compressed_texture_s3tc,
    WebGLExtensionTypeWEBGL_compressed_texture_s3tc_srgb,
    WebGLExtensionTypeWEBGL_compressed_texture_etc,
    WebGLExtensionTypeWEBGL_compressed_texture_pvrtc,
    WebGLExtensionTypeWEBGL_lose_context,
    WebGLExtensionTypeANGLE_instanced_arrays,
    WebGLExtensionTypeWEBGL_depth_texture,
    WebGLExtensionTypeWEBGL_draw_buffers,
    WebGLExtensionTypeOES_fbo_render_mipmap,
    WebGLExtensionTypeNone,
}

impl Into<WebGLExtensionType> for canvas_webgl::prelude::WebGLExtensionType {
    fn into(self) -> WebGLExtensionType {
        match self {
            canvas_webgl::prelude::WebGLExtensionType::EXT_blend_minmax => {
                WebGLExtensionType::WebGLExtensionTypeEXT_blend_minmax
            }
            canvas_webgl::prelude::WebGLExtensionType::EXT_color_buffer_half_float => {
                WebGLExtensionType::WebGLExtensionTypeEXT_color_buffer_half_float
            }
            canvas_webgl::prelude::WebGLExtensionType::EXT_disjoint_timer_query => {
                WebGLExtensionType::WebGLExtensionTypeEXT_disjoint_timer_query
            }
            canvas_webgl::prelude::WebGLExtensionType::EXT_sRGB => {
                WebGLExtensionType::WebGLExtensionTypeEXT_sRGB
            }
            canvas_webgl::prelude::WebGLExtensionType::EXT_shader_texture_lod => {
                WebGLExtensionType::WebGLExtensionTypeEXT_shader_texture_lod
            }
            canvas_webgl::prelude::WebGLExtensionType::EXT_texture_filter_anisotropic => {
                WebGLExtensionType::WebGLExtensionTypeEXT_texture_filter_anisotropic
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_element_index_uint => {
                WebGLExtensionType::WebGLExtensionTypeOES_element_index_uint
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_standard_derivatives => {
                WebGLExtensionType::WebGLExtensionTypeOES_standard_derivatives
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_texture_float => {
                WebGLExtensionType::WebGLExtensionTypeOES_texture_float
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_texture_float_linear => {
                WebGLExtensionType::WebGLExtensionTypeOES_texture_float_linear
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_texture_half_float => {
                WebGLExtensionType::WebGLExtensionTypeOES_texture_half_float
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_texture_half_float_linear => {
                WebGLExtensionType::WebGLExtensionTypeOES_texture_half_float_linear
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_vertex_array_object => {
                WebGLExtensionType::WebGLExtensionTypeOES_vertex_array_object
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_color_buffer_float => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_color_buffer_float
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_atc => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_atc
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_etc1 => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_etc1
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_s3tc => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_s3tc
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_s3tc_srgb => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_s3tc_srgb
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_etc => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_etc
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_pvrtc => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_pvrtc
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_lose_context => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_lose_context
            }
            canvas_webgl::prelude::WebGLExtensionType::ANGLE_instanced_arrays => {
                WebGLExtensionType::WebGLExtensionTypeANGLE_instanced_arrays
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_depth_texture => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_depth_texture
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_draw_buffers => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_draw_buffers
            }
            canvas_webgl::prelude::WebGLExtensionType::None => {
                WebGLExtensionType::WebGLExtensionTypeNone
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_fbo_render_mipmap => {
                WebGLExtensionType::WebGLExtensionTypeOES_fbo_render_mipmap
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_pattern_webgl(
    source: *mut WebGLState,
    context: *mut CanvasRenderingContext2D,
    repetition: CanvasRepetition,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    assert!(!source.is_null());
    let context = unsafe { &mut *context };

    let source = unsafe { &*source };


    context.remove_if_current();
    let state = source.get_inner();
    state.make_current();
    let width = state.get_drawing_buffer_width();
    let height = state.get_drawing_buffer_height();

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

    #[cfg(feature = "gl")]{
        context.make_current();
    }

    let ret = from_image_slice(buf.as_slice(), width, height).map(|image| {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
            context.context.create_pattern(image, repetition.into()),
        )
    });

    match ret {
        None => std::ptr::null_mut(),
        Some(ret) => Box::into_raw(Box::new(PaintStyle(ret))),
    }
}

/* GL */

#[no_mangle]
pub extern "C" fn canvas_native_webgl_make_current(state: *mut WebGLState) -> bool {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    state.get_inner().make_current()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_swap_buffers(state: *mut WebGLState) -> bool {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    state.get_inner().swap_buffers()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_make_current_and_swap_buffers(
    state: *mut WebGLState,
) -> bool {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    state.get_inner().make_current_and_swap_buffers()
}

/* GL */

/* WebGL */

#[no_mangle]
pub extern "C" fn canvas_native_webgl_resized(_state: *mut WebGLState) {
    //state.get_inner_mut().resized();
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_to_data_url(
    state: *mut WebGLState,
    format: *const c_char,
    quality: u32,
) -> *const c_char {
    assert!(!state.is_null());
    assert!(!format.is_null());
    let format = unsafe { CStr::from_ptr(format) };
    let format = format.to_string_lossy();
    let state = unsafe { &mut *state };

    let info = state.get_inner();
    info.make_current();
    let width = info.drawing_buffer_width();
    let height = info.drawing_buffer_height();
    // gl_bindings::PixelStorei(gl_bindings::UNPACK_ALIGNMENT, 1);
    let mut buffer = vec![0u8; (width * height * 4) as usize];
    unsafe {
        gl_bindings::ReadPixels(
            0,
            0,
            width,
            height,
            gl_bindings::RGBA,
            gl_bindings::UNSIGNED_BYTE,
            buffer.as_mut_ptr() as *mut c_void,
        );
    }

    CString::new(canvas_2d::bytes_to_data_url(
        width,
        height,
        buffer.as_slice(),
        format.as_ref(),
        quality,
    ))
        .unwrap()
        .into_raw()
}

#[derive(Debug)]
pub struct WebGLState(pub(crate) canvas_webgl::prelude::WebGLState);

impl WebGLState {
    pub fn get_dimensions(&self) -> (i32, i32) {
        self.0.get_dimensions()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_state_destroy(state: *mut WebGLState) {
    if state.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(state) };
}

impl WebGLState {
    #[cfg(target_os = "android")]
    pub fn new_with_view(
        view: *mut c_void,
        width: i32,
        height: i32,
        version: WebGLVersion,
        alpha: bool,
        antialias: bool,
        depth: bool,
        fail_if_major_performance_caveat: bool,
        power_preference: PowerPreference,
        premultiplied_alpha: bool,
        preserve_drawing_buffer: bool,
        stencil: bool,
        desynchronized: bool,
        xr_compatible: bool,
        is_canvas: bool,
    ) -> Option<Self> {
        let mut attr = canvas_core::context_attributes::ContextAttributes::new(
            alpha, antialias, depth, fail_if_major_performance_caveat, power_preference.into(), premultiplied_alpha, preserve_drawing_buffer, stencil, desynchronized, xr_compatible, false,
        );

        let context = if !view.is_null() {
            let handle = raw_window_handle::AndroidNdkWindowHandle::new(NonNull::new(view).unwrap());
            let handle = raw_window_handle::RawWindowHandle::AndroidNdk(handle);
            GLContext::create_window_context(
                &mut attr, width, height, handle,
            )
        } else {
            GLContext::create_offscreen_context(&mut attr, width, height)
        }?;

        Some(Self(
            canvas_webgl::prelude::WebGLState::new_with_context_attributes(
                context,
                version,
                attr.get_alpha(),
                attr.get_antialias(),
                attr.get_depth(),
                fail_if_major_performance_caveat,
                power_preference,
                premultiplied_alpha,
                attr.get_preserve_drawing_buffer(),
                attr.get_stencil(),
                desynchronized,
                xr_compatible,
                is_canvas,
            )))
    }

    #[cfg(not(target_os = "android"))]
    pub fn new_with_view(
        view: *mut c_void,
        version: WebGLVersion,
        alpha: bool,
        antialias: bool,
        depth: bool,
        fail_if_major_performance_caveat: bool,
        power_preference: PowerPreference,
        premultiplied_alpha: bool,
        preserve_drawing_buffer: bool,
        stencil: bool,
        desynchronized: bool,
        xr_compatible: bool,
        is_canvas: bool,
    ) -> Option<Self> {
        let mut attr = canvas_core::context_attributes::ContextAttributes::new(
            alpha, antialias, depth, fail_if_major_performance_caveat, power_preference.into(), premultiplied_alpha, preserve_drawing_buffer, stencil, desynchronized, xr_compatible, false,
        );
        let context = GLContext::create_window_context(
            &mut attr, NonNull::new(view)?,
        );
        Some(Self(
            canvas_webgl::prelude::WebGLState::new_with_context_attributes(
                context?,
                version,
                attr.get_alpha(),
                attr.get_antialias(),
                attr.get_depth(),
                fail_if_major_performance_caveat,
                power_preference,
                premultiplied_alpha,
                attr.get_preserve_drawing_buffer(),
                attr.get_stencil(),
                desynchronized,
                xr_compatible,
                is_canvas,
            )))
    }
    pub fn new_with_context(
        context: GLContext,
        version: WebGLVersion,
        alpha: bool,
        antialias: bool,
        depth: bool,
        fail_if_major_performance_caveat: bool,
        power_preference: PowerPreference,
        premultiplied_alpha: bool,
        preserve_drawing_buffer: bool,
        stencil: bool,
        desynchronized: bool,
        xr_compatible: bool,
        is_canvas: bool,
    ) -> Self {
        Self(
            canvas_webgl::prelude::WebGLState::new_with_context_attributes(
                context,
                version,
                alpha,
                antialias,
                depth,
                fail_if_major_performance_caveat,
                power_preference,
                premultiplied_alpha,
                preserve_drawing_buffer,
                stencil,
                desynchronized,
                xr_compatible,
                is_canvas,
            ))
    }
    pub fn get_inner(&self) -> &canvas_webgl::prelude::WebGLState {
        &self.0
    }

    pub fn get_inner_mut(&mut self) -> &mut canvas_webgl::prelude::WebGLState {
        &mut self.0
    }
}

pub struct WebGLActiveInfo(pub(crate) canvas_webgl::prelude::WebGLActiveInfo);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_active_info_destroy(info: *mut WebGLActiveInfo) {
    if info.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(info) };
}

impl WebGLActiveInfo {
    pub fn get_name(&self) -> &str {
        self.0.get_name()
    }

    pub fn get_size(&self) -> i32 {
        self.0.get_size()
    }

    pub fn get_type(&self) -> u32 {
        self.0.get_type()
    }

    pub fn get_is_empty(&self) -> bool {
        self.0.get_is_empty()
    }
}

pub struct ContextAttributes(canvas_core::context_attributes::ContextAttributes);

#[no_mangle]
pub extern "C" fn canvas_native_context_attributes_destroy(attr: *mut ContextAttributes) {
    if attr.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(attr) };
}

impl ContextAttributes {
    pub fn get_alpha(&self) -> bool {
        self.0.get_alpha()
    }
    pub fn get_antialias(&self) -> bool {
        self.0.get_antialias()
    }
    pub fn get_depth(&self) -> bool {
        self.0.get_depth()
    }
    pub fn get_fail_if_major_performance_caveat(&self) -> bool {
        self.0.get_fail_if_major_performance_caveat()
    }
    pub fn get_power_preference(&self) -> PowerPreference {
        self.0.get_power_preference()
    }
    pub fn get_premultiplied_alpha(&self) -> bool {
        self.0.get_premultiplied_alpha()
    }
    pub fn get_preserve_drawing_buffer(&self) -> bool {
        self.0.get_preserve_drawing_buffer()
    }
    pub fn get_stencil(&self) -> bool {
        self.0.get_stencil()
    }
    pub fn get_desynchronized(&self) -> bool {
        self.0.get_desynchronized()
    }
    pub fn get_xr_compatible(&self) -> bool {
        self.0.get_xr_compatible()
    }
}

pub struct WebGLFramebufferAttachmentParameter(
    canvas_webgl::prelude::WebGLFramebufferAttachmentParameter,
);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_framebuffer_attachment_parameter_destroy(
    parameter: *mut WebGLFramebufferAttachmentParameter,
) {
    if parameter.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(parameter) };
}

impl WebGLFramebufferAttachmentParameter {
    pub fn get_is_texture(&self) -> bool {
        self.0.get_is_texture()
    }

    pub fn get_is_renderbuffer(&self) -> bool {
        self.0.get_is_renderbuffer()
    }

    pub fn get_value(&self) -> i32 {
        self.0.get_value()
    }
}

pub struct WebGLShaderPrecisionFormat(canvas_webgl::prelude::WebGLShaderPrecisionFormat);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_shader_precision_format_destroy(
    value: *mut WebGLShaderPrecisionFormat,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

impl WebGLShaderPrecisionFormat {
    pub fn get_precision(&self) -> i32 {
        self.0.get_precision()
    }

    pub fn get_range_min(&self) -> i32 {
        self.0.get_range_min()
    }

    pub fn get_range_max(&self) -> i32 {
        self.0.get_range_max()
    }
}

pub struct WebGLExtension(Option<Box<dyn canvas_webgl::prelude::WebGLExtension>>);

impl WebGLExtension {
    pub fn new(extension: Option<Box<dyn canvas_webgl::prelude::WebGLExtension>>) -> Self {
        Self(extension)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_extension_destroy(value: *mut WebGLExtension) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

impl WebGLExtension {
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    pub fn extension_type(&self) -> WebGLExtensionType {
        if self.is_none() {
            return WebGLExtensionType::WebGLExtensionTypeNone;
        }
        self.0.as_ref().unwrap().extension_type().into()
    }

    pub fn into_ext_disjoint_timer_query(self) -> Box<EXT_disjoint_timer_query> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext =
            unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::EXT_disjoint_timer_query) };
        Box::new(EXT_disjoint_timer_query(*ext))
    }

    pub fn into_angle_instanced_arrays(self) -> Box<ANGLE_instanced_arrays> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext =
            unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::ANGLE_instanced_arrays) };
        Box::new(ANGLE_instanced_arrays(*ext))
    }

    pub fn into_lose_context(self) -> Box<WEBGL_lose_context> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext = unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::WEBGL_lose_context) };
        Box::new(WEBGL_lose_context(*ext))
    }

    pub fn into_draw_buffers(self) -> Box<WEBGL_draw_buffers> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext = unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::WEBGL_draw_buffers) };
        Box::new(WEBGL_draw_buffers(*ext))
    }

    pub fn into_oes_vertex_array_object(self) -> Box<OES_vertex_array_object> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext =
            unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::OES_vertex_array_object) };
        Box::new(OES_vertex_array_object(*ext))
    }
}

#[allow(non_camel_case_types)]
pub struct EXT_blend_minmax;

#[no_mangle]
pub extern "C" fn canvas_native_webgl_EXT_blend_minmax_destroy(value: *mut EXT_blend_minmax) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct EXT_color_buffer_half_float;

#[no_mangle]
pub extern "C" fn canvas_native_webgl_EXT_color_buffer_half_float_destroy(
    value: *mut EXT_color_buffer_half_float,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct EXT_disjoint_timer_query(canvas_webgl::prelude::EXT_disjoint_timer_query);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_EXT_disjoint_timer_query_destroy(
    value: *mut EXT_disjoint_timer_query,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct EXT_sRGB;

#[no_mangle]
pub extern "C" fn canvas_native_webgl_EXT_sRGB_destroy(value: *mut EXT_disjoint_timer_query) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct EXT_shader_texture_lod(canvas_webgl::prelude::EXT_shader_texture_lod);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_EXT_shader_texture_lod_destroy(
    value: *mut EXT_shader_texture_lod,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct EXT_texture_filter_anisotropic;

#[no_mangle]
pub extern "C" fn canvas_native_webgl_EXT_texture_filter_anisotropic_destroy(
    value: *mut EXT_texture_filter_anisotropic,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct OES_element_index_uint;

#[no_mangle]
pub extern "C" fn canvas_native_webgl_OES_element_index_uint_destroy(
    value: *mut OES_element_index_uint,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct OES_standard_derivatives;

#[no_mangle]
pub extern "C" fn canvas_native_webgl_OES_standard_derivatives_destroy(
    value: *mut OES_standard_derivatives,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct OES_texture_float(canvas_webgl::prelude::OES_texture_float);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_OES_texture_float_destroy(value: *mut OES_texture_float) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct OES_texture_float_linear(canvas_webgl::prelude::OES_texture_float_linear);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_OES_texture_float_linear_destroy(
    value: *mut OES_texture_float_linear,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct OES_texture_half_float;

#[no_mangle]
pub extern "C" fn canvas_native_webgl_OES_texture_half_float_destroy(
    value: *mut OES_texture_half_float,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct OES_texture_half_float_linear(canvas_webgl::prelude::OES_texture_half_float_linear);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_OES_texture_half_float_linear_destroy(
    value: *mut OES_texture_half_float_linear,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct OES_vertex_array_object(canvas_webgl::prelude::OES_vertex_array_object);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_OES_vertex_array_object_destroy(
    value: *mut OES_vertex_array_object,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_color_buffer_float;

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_color_buffer_float_destroy(
    value: *mut WEBGL_color_buffer_float,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_atc;

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_compressed_texture_atc_destroy(
    value: *mut WEBGL_compressed_texture_atc,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_etc1;

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_compressed_texture_etc1_destroy(
    value: *mut WEBGL_compressed_texture_etc1,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_s3tc;

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_compressed_texture_s3tc_destroy(
    value: *mut WEBGL_compressed_texture_s3tc,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_s3tc_srgb;

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_compressed_texture_s3tc_srgb_destroy(
    value: *mut WEBGL_compressed_texture_s3tc_srgb,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_etc;

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_compressed_texture_etc_destroy(
    value: *mut WEBGL_compressed_texture_etc,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_pvrtc;

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_compressed_texture_pvrtc_destroy(
    value: *mut WEBGL_compressed_texture_pvrtc,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_lose_context(canvas_webgl::prelude::WEBGL_lose_context);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_lose_context_destroy(value: *mut WEBGL_lose_context) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct ANGLE_instanced_arrays(canvas_webgl::prelude::ANGLE_instanced_arrays);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_ANGLE_instanced_arrays_destroy(
    value: *mut ANGLE_instanced_arrays,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_depth_texture;

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_depth_texture_destroy(value: *mut WEBGL_depth_texture) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_draw_buffers(canvas_webgl::prelude::WEBGL_draw_buffers);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_draw_buffers_destroy(value: *mut WEBGL_draw_buffers) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

pub struct WebGLResult(pub(crate) canvas_webgl::prelude::WebGLResult);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WebGLResult_destroy(value: *mut WebGLResult) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

/* WebGLActiveInfo */

#[no_mangle]
pub extern "C" fn canvas_native_webgl_active_info_get_name(
    info: *const WebGLActiveInfo,
) -> *const c_char {
    if info.is_null() {
        return std::ptr::null();
    }
    let info = unsafe { &*info };
    CString::new(info.get_name()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_active_info_get_size(info: *const WebGLActiveInfo) -> i32 {
    if info.is_null() {
        return 0;
    }
    let info = unsafe { &*info };
    info.get_size()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_active_info_get_type(info: *const WebGLActiveInfo) -> u32 {
    if info.is_null() {
        return 0;
    }
    let info = unsafe { &*info };
    info.get_type()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_active_info_get_is_empty(
    info: *const WebGLActiveInfo,
) -> bool {
    if info.is_null() {
        return false;
    }
    let info = unsafe { &*info };
    info.get_is_empty()
}

/* WebGLActiveInfo */

/* WebGLShaderPrecisionFormat */

#[no_mangle]
pub extern "C" fn canvas_native_webgl_shader_precision_format_get_range_min(
    shader: *const WebGLShaderPrecisionFormat,
) -> i32 {
    if shader.is_null() {
        return 0;
    }
    let shader = unsafe { &*shader };
    shader.get_range_min()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_shader_precision_format_get_range_max(
    shader: *const WebGLShaderPrecisionFormat,
) -> i32 {
    if shader.is_null() {
        return 0;
    }
    let shader = unsafe { &*shader };
    shader.get_range_max()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_shader_precision_format_get_precision(
    shader: *const WebGLShaderPrecisionFormat,
) -> i32 {
    if shader.is_null() {
        return 0;
    }
    let shader = unsafe { &*shader };
    shader.get_precision()
}

/* WebGLShaderPrecisionFormat */

/* ContextAttributes */
#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_alpha(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_alpha()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_antialias(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_antialias()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_depth(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_depth()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_fail_if_major_performance_caveat(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_fail_if_major_performance_caveat()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_power_preference(
    attr: *const ContextAttributes,
) -> i32 {
    if attr.is_null() {
        return 0;
    }
    let attr = unsafe { &*attr };
    attr.get_power_preference().into()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_premultiplied_alpha(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_premultiplied_alpha()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_preserve_drawing_buffer(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_preserve_drawing_buffer()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_stencil(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_stencil()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_desynchronized(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_desynchronized()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_xr_compatible(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_xr_compatible()
}

/* ContextAttributes */

/* WebGLExtension */
#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_extension_is_none(
    extension: *const WebGLExtension,
) -> bool {
    assert!(!extension.is_null());
    let extension = unsafe { &*extension };
    extension.is_none()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_extension_get_type(
    extension: *const WebGLExtension,
) -> WebGLExtensionType {
    let extension = unsafe { &*extension };
    extension.extension_type().into()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_extension_to_ext_disjoint_timer_query(
    extension: *mut WebGLExtension,
) -> *mut EXT_disjoint_timer_query {
    let extension = unsafe { Box::from_raw(&mut *extension) };
    Box::into_raw(extension.into_ext_disjoint_timer_query())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_extension_to_angle_instanced_arrays(
    extension: *mut WebGLExtension,
) -> *mut ANGLE_instanced_arrays {
    let extension = unsafe { Box::from_raw(&mut *extension) };
    Box::into_raw(extension.into_angle_instanced_arrays())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_extension_to_lose_context(
    extension: *mut WebGLExtension,
) -> *mut WEBGL_lose_context {
    let extension = unsafe { Box::from_raw(&mut *extension) };
    Box::into_raw(extension.into_lose_context())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_extension_to_draw_buffers(
    extension: *mut WebGLExtension,
) -> *mut WEBGL_draw_buffers {
    let extension = unsafe { Box::from_raw(&mut *extension) };
    Box::into_raw(extension.into_draw_buffers())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_extension_to_oes_vertex_array_object(
    extension: *mut WebGLExtension,
) -> *mut OES_vertex_array_object {
    let extension = unsafe { Box::from_raw(&mut *extension) };
    Box::into_raw(extension.into_oes_vertex_array_object())
}

/* WebGLExtension */

/* WebGLResult */
#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_type(
    result: *const WebGLResult,
) -> WebGLResultType {
    assert!(!result.is_null());
    let result = unsafe { &*result };
    match result.0 {
        canvas_webgl::prelude::WebGLResult::Boolean(_) => WebGLResultType::Boolean,
        canvas_webgl::prelude::WebGLResult::I32Array(_) => WebGLResultType::I32Array,
        canvas_webgl::prelude::WebGLResult::U32Array(_) => WebGLResultType::U32Array,
        canvas_webgl::prelude::WebGLResult::F32Array(_) => WebGLResultType::F32Array,
        canvas_webgl::prelude::WebGLResult::BooleanArray(_) => WebGLResultType::BooleanArray,
        canvas_webgl::prelude::WebGLResult::U32(_) => WebGLResultType::U32,
        canvas_webgl::prelude::WebGLResult::I32(_) => WebGLResultType::I32,
        canvas_webgl::prelude::WebGLResult::F32(_) => WebGLResultType::F32,
        canvas_webgl::prelude::WebGLResult::String(_) => WebGLResultType::String,
        canvas_webgl::prelude::WebGLResult::None => WebGLResultType::None,
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_bool(result: *const WebGLResult) -> bool {
    let result = unsafe { &*result };
    match result.0 {
        canvas_webgl::prelude::WebGLResult::Boolean(value) => value,
        _ => false,
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_i32_array(
    result: *const WebGLResult,
) -> *mut I32Buffer {
    let result = unsafe { &*result };
    Box::into_raw(Box::new(I32Buffer::from(match &result.0 {
        canvas_webgl::prelude::WebGLResult::I32Array(value) => value.to_vec(),
        _ => Vec::new(),
    })))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_into_i32_array(
    result: *mut WebGLResult,
) -> *mut I32Buffer {
    if result.is_null() {
        return std::ptr::null_mut();
    }
    let result = unsafe { *Box::from_raw(result) };
    Box::into_raw(Box::new(I32Buffer::from(match result.0 {
        canvas_webgl::prelude::WebGLResult::I32Array(value) => value,
        _ => Vec::new(),
    })))
}


#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_u32_array(
    result: *const WebGLResult,
) -> *mut U32Buffer {
    let result = unsafe { &*result };
    Box::into_raw(Box::new(U32Buffer::from(match &result.0 {
        canvas_webgl::prelude::WebGLResult::U32Array(value) => value.to_vec(),
        _ => Vec::new(),
    })))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_into_u32_array(
    result: *mut WebGLResult,
) -> *mut U32Buffer {
    if result.is_null() {
        return std::ptr::null_mut();
    }

    let result = unsafe { *Box::from_raw(result) };
    Box::into_raw(Box::new(U32Buffer::from(match result.0 {
        canvas_webgl::prelude::WebGLResult::U32Array(value) => value,
        _ => Vec::new(),
    })))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_f32_array(
    result: *const WebGLResult,
) -> *mut F32Buffer {
    let result = unsafe { &*result };
    Box::into_raw(Box::new(F32Buffer::from(match &result.0 {
        canvas_webgl::prelude::WebGLResult::F32Array(value) => value.to_vec(),
        _ => Vec::new(),
    })))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_into_f32_array(
    result: *mut WebGLResult,
) -> *mut F32Buffer {
    if result.is_null() {
        return std::ptr::null_mut();
    }
    let result = unsafe { *Box::from_raw(result) };

    Box::into_raw(Box::new(F32Buffer::from(match result.0 {
        canvas_webgl::prelude::WebGLResult::F32Array(value) => value,
        _ => Vec::new(),
    })))
}


#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_bool_array(
    result: *const WebGLResult,
) -> *mut U8Buffer {
    let result = unsafe { &*result };
    Box::into_raw(Box::new(U8Buffer::from(match &result.0 {
        canvas_webgl::prelude::WebGLResult::BooleanArray(value) => unsafe {
            std::slice::from_raw_parts(value.as_ptr() as *const u8, value.len()).to_vec()
        },
        _ => Vec::new(),
    })))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_u32(result: *const WebGLResult) -> u32 {
    let result = unsafe { &*result };
    match result.0 {
        canvas_webgl::prelude::WebGLResult::U32(value) => value,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_i32(result: *const WebGLResult) -> i32 {
    let result = unsafe { &*result };
    match result.0 {
        canvas_webgl::prelude::WebGLResult::I32(value) => value,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_f32(result: *const WebGLResult) -> f32 {
    let result = unsafe { &*result };
    match result.0 {
        canvas_webgl::prelude::WebGLResult::F32(value) => value,
        _ => 0.,
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_string(
    result: *const WebGLResult,
) -> *const c_char {
    let result = unsafe { &*result };
    let ret;
    match result.0 {
        canvas_webgl::prelude::WebGLResult::String(ref result) => {
            let val = result.to_string_lossy();

            if val.contains("OpenGL ES 3.0") {
                return CString::new("WebGL 2.0 (OpenGL ES 3.0 NativeScript)")
                    .unwrap()
                    .into_raw();
            } else if val.contains("OpenGL ES 2.") {
                ret = CString::new("WebGL 1.0 (OpenGL ES 2.0 NativeScript)")
                    .unwrap()
                    .into_raw()
            } else {
                ret = result.clone().into_raw()
            }
        }
        _ => {
            ret = std::ptr::null_mut();
        }
    };
    ret
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_is_none(result: *const WebGLResult) -> bool {
    let result = unsafe { &*result };
    match result.0 {
        canvas_webgl::prelude::WebGLResult::None => true,
        _ => false,
    }
}

/* WebGLResult */

/* WebGLState */
#[no_mangle]
pub extern "C" fn canvas_native_webgl_state_get_unpack_colorspace_conversion_webgl(
    state: *mut WebGLState,
) -> i32 {
    let state = unsafe { &*state };
    state.get_inner().get_unpack_colorspace_conversion_webgl()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_state_get_flip_y(state: *mut WebGLState) -> bool {
    let state = unsafe { &*state };
    state.get_inner().get_flip_y()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_state_get_premultiplied_alpha(
    state: *mut WebGLState,
) -> bool {
    let state = unsafe { &*state };
    state.get_inner().get_premultiplied_alpha()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_state_get_drawing_buffer_width(
    state: *mut WebGLState,
) -> i32 {
    let state = unsafe { &*state };
    state.get_inner().get_drawing_buffer_width()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_state_get_drawing_buffer_height(
    state: *mut WebGLState,
) -> i32 {
    let state = unsafe { &*state };
    state.get_inner().get_drawing_buffer_height()
}

/* WebGLState */

/* EXT_disjoint_timer_query */
#[no_mangle]
pub extern "C" fn canvas_native_webgl_ext_disjoint_timer_query_create_query_ext(
    query: *const EXT_disjoint_timer_query,
) -> u32 {
    let query = unsafe { &*query };
    query.0.create_query_ext()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_ext_disjoint_timer_query_delete_query_ext(
    value: u32,
    query: *const EXT_disjoint_timer_query,
) {
    let query = unsafe { &*query };
    query.0.delete_query_ext(value)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_ext_disjoint_timer_query_is_query_ext(
    value: u32,
    query: *const EXT_disjoint_timer_query,
) -> bool {
    let query = unsafe { &*query };
    query.0.is_query_ext(value)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_ext_disjoint_timer_query_begin_query_ext(
    target: u32,
    value: u32,
    query: *const EXT_disjoint_timer_query,
) {
    let query = unsafe { &*query };
    query.0.begin_query_ext(target, value)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_ext_disjoint_timer_query_end_query_ext(
    target: u32,
    query: *const EXT_disjoint_timer_query,
) {
    let query = unsafe { &*query };
    query.0.end_query_ext(target)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_ext_disjoint_timer_query_query_counter_ext(
    value: u32,
    target: u32,
    query: *const EXT_disjoint_timer_query,
) {
    let query = unsafe { &*query };
    query.0.query_counter_ext(value, target)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_ext_disjoint_timer_query_get_query_ext(
    target: u32,
    pname: u32,
    query: *const EXT_disjoint_timer_query,
) -> i32 {
    let query = unsafe { &*query };
    query.0.get_query_ext(target, pname)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(
    target: u32,
    pname: u32,
    query: *const EXT_disjoint_timer_query,
) -> *mut WebGLResult {
    let query = unsafe { &*query };
    Box::into_raw(Box::new(WebGLResult(
        query.0.get_query_object_ext(target, pname),
    )))
}

/* EXT_disjoint_timer_query */

/* ANGLE_instanced_arrays */
#[no_mangle]
pub extern "C" fn canvas_native_webgl_angle_instanced_arrays_draw_arrays_instanced_angle(
    mode: u32,
    first: i32,
    count: i32,
    primcount: i32,
    arrays: *const ANGLE_instanced_arrays,
) {
    let arrays = unsafe { &*arrays };
    arrays
        .0
        .draw_arrays_instanced_angle(mode, first, count, primcount)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_angle_instanced_arrays_draw_elements_instanced_angle(
    mode: u32,
    count: i32,
    type_: u32,
    offset: i32,
    primcount: i32,
    arrays: *const ANGLE_instanced_arrays,
) {
    let arrays = unsafe { &*arrays };
    arrays
        .0
        .draw_elements_instanced_angle(mode, count, type_, offset, primcount)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_angle_instanced_arrays_vertex_attrib_divisor_angle(
    index: u32,
    divisor: u32,
    arrays: *const ANGLE_instanced_arrays,
) {
    let arrays = unsafe { &*arrays };
    arrays.0.vertex_attrib_divisor_angle(index, divisor)
}
/* ANGLE_instanced_arrays */

/* WEBGL_lose_context */
#[no_mangle]
pub extern "C" fn canvas_native_webgl_lose_context_lose_context(
    context: *const WEBGL_lose_context,
) {
    let context = unsafe { &*context };

    context.0.lose_context()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_lose_context_restore_context(
    context: *const WEBGL_lose_context,
) {
    let context = unsafe { &*context };
    context.0.restore_context()
}
/* WEBGL_lose_context */

/* WEBGL_draw_buffers */

#[no_mangle]
pub extern "C" fn canvas_native_webgl_draw_buffers_draw_buffers_webgl(
    buffers: *const u32,
    size: usize,
    context: *const WEBGL_draw_buffers,
) {
    assert!(!context.is_null());
    let buffers = unsafe { std::slice::from_raw_parts(buffers, size) };
    let context = unsafe { &*context };
    context.0.draw_buffers_webgl(buffers);
}

/* WEBGL_draw_buffers */

/* OES_vertex_array_object */

#[no_mangle]
pub extern "C" fn canvas_native_webgl_oes_vertex_array_object_create_vertex_array_oes(
    object: *const OES_vertex_array_object,
) -> u32 {
    let object = unsafe { &*object };
    object.0.create_vertex_array_oes()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(
    array_object: u32,
    object: *const OES_vertex_array_object,
) {
    let object = unsafe { &*object };
    object.0.delete_vertex_array_oes(array_object)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(
    array_object: u32,
    object: *const OES_vertex_array_object,
) -> bool {
    let object = unsafe { &*object };
    object.0.is_vertex_array_oes(array_object)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(
    array_object: u32,
    object: *const OES_vertex_array_object,
) {
    let object = unsafe { &*object };
    object.0.bind_vertex_array_oes(array_object)
}

/* OES_vertex_array_object */


#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn canvas_native_webgl_create(
    view: *mut c_void,
    width: i32,
    height: i32,
    version: i32,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: i32,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
) -> *mut WebGLState {
    match (
        WebGLVersion::try_from(version).ok(),
        PowerPreference::try_from(power_preference).ok(),
    ) {
        (Some(version), Some(power_preference)) => {
            match WebGLState::new_with_view(
                view,
                width,
                height,
                version,
                alpha,
                antialias,
                depth,
                fail_if_major_performance_caveat,
                power_preference,
                premultiplied_alpha,
                preserve_drawing_buffer,
                stencil,
                desynchronized,
                xr_compatible,
                false,
            ) {
                None => std::ptr::null_mut(),
                Some(state) => {
                    Box::into_raw(Box::new(state))
                }
            }
        }
        _ => std::ptr::null_mut(),
    }
}


#[cfg(not(target_os = "android"))]
#[no_mangle]
pub extern "C" fn canvas_native_webgl_create(
    view: *mut c_void,
    version: i32,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: i32,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
) -> *mut WebGLState {
    match (
        WebGLVersion::try_from(version).ok(),
        PowerPreference::try_from(power_preference).ok(),
    ) {
        (Some(version), Some(power_preference)) => {
            match WebGLState::new_with_view(
                view,
                version,
                alpha,
                antialias,
                depth,
                fail_if_major_performance_caveat,
                power_preference,
                premultiplied_alpha,
                preserve_drawing_buffer,
                stencil,
                desynchronized,
                xr_compatible,
                false,
            ) {
                None => std::ptr::null_mut(),
                Some(state) => {
                    Box::into_raw(Box::new(state))
                }
            }
        }
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_create_no_window(
    width: i32,
    height: i32,
    version: i32,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: i32,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
    is_canvas: bool,
) -> *mut WebGLState {
    match (
        WebGLVersion::try_from(version).ok(),
        PowerPreference::try_from(power_preference).ok(),
    ) {
        (Some(version), Some(power_preference)) => {
            match canvas_native_webgl_create_no_window_internal(
                width,
                height,
                version,
                alpha,
                antialias,
                depth,
                fail_if_major_performance_caveat,
                power_preference,
                premultiplied_alpha,
                preserve_drawing_buffer,
                stencil,
                desynchronized,
                xr_compatible,
                is_canvas,
            ) {
                None => std::ptr::null_mut(),
                Some(state) => {
                    Box::into_raw(Box::new(state))
                }
            }
        }
        _ => std::ptr::null_mut(),
    }
}

pub(crate) fn canvas_native_webgl_create_no_window_internal(
    width: i32,
    height: i32,
    version: WebGLVersion,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: PowerPreference,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
    is_canvas: bool,
) -> Option<WebGLState> {
    let mut attrs = canvas_core::context_attributes::ContextAttributes::new(
        alpha,
        antialias,
        depth,
        fail_if_major_performance_caveat,
        power_preference.into(),
        premultiplied_alpha,
        preserve_drawing_buffer,
        stencil,
        desynchronized,
        xr_compatible,
        is_canvas,
    );

    let ctx = GLContext::create_offscreen_context(&mut attrs, width, height)?;

    Some(
        WebGLState::new_with_context(
            ctx,
            version,
            attrs.get_alpha(),
            attrs.get_antialias(),
            attrs.get_depth(),
            attrs.get_fail_if_major_performance_caveat(),
            PowerPreference::from(attrs.get_power_preference()),
            attrs.get_premultiplied_alpha(),
            attrs.get_preserve_drawing_buffer(),
            attrs.get_stencil(),
            attrs.get_desynchronized(),
            attrs.get_xr_compatible(),
            attrs.get_is_canvas(),
        )
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_active_texture(texture: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_active_texture(texture, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_attach_shader(
    program: u32,
    shader: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_attach_shader(program, shader, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_bind_attrib_location(
    program: u32,
    index: u32,
    name: *const c_char,
    state: *mut WebGLState,
) {
    if name.is_null() {
        return;
    }
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_string_lossy();
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_bind_attrib_location(
        program,
        index,
        name.as_ref(),
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_bind_buffer(
    target: u32,
    buffer: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_bind_buffer(target, buffer, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_bind_frame_buffer(
    target: u32,
    framebuffer: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_bind_frame_buffer(
        target,
        framebuffer,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_bind_render_buffer(
    target: u32,
    renderbuffer: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_bind_render_buffer(
        target,
        renderbuffer,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_bind_texture(
    target: u32,
    texture: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_bind_texture(target, texture, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_blend_color(
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_blend_color(
        red,
        green,
        blue,
        alpha,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_blend_equation_separate(
    mode_rgb: u32,
    mode_alpha: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_blend_equation_separate(
        mode_rgb,
        mode_alpha,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_blend_equation(mode: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_blend_equation(mode, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_blend_func_separate(
    src_rgb: u32,
    dst_rgb: u32,
    src_alpha: u32,
    dst_alpha: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_blend_func_separate(
        src_rgb,
        dst_rgb,
        src_alpha,
        dst_alpha,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_blend_func(
    sfactor: u32,
    dfactor: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_blend_func(sfactor, dfactor, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_data(
    target: u32,
    src_data: *const u8,
    size: usize,
    usage: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_data(
        target,
        src_data,
        usage,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_data_i8(
    target: u32,
    src_data: *const i8,
    size: usize,
    usage: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_data_i8(
        target,
        src_data,
        usage,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_data_u16(
    target: u32,
    src_data: *const u16,
    size: usize,
    usage: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_data_u16(
        target,
        src_data,
        usage,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_data_i16(
    target: u32,
    src_data: *const i16,
    size: usize,
    usage: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_data_i16(
        target,
        src_data,
        usage,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_data_u32(
    target: u32,
    src_data: *const u32,
    size: usize,
    usage: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_data_u32(
        target,
        src_data,
        usage,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_data_i32(
    target: u32,
    src_data: *const i32,
    size: usize,
    usage: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_data_i32(
        target,
        src_data,
        usage,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_data_f32(
    target: u32,
    src_data: *const f32,
    size: usize,
    usage: u32,
    state: *mut WebGLState,
) {
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_data_f32(
        target,
        src_data,
        usage,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_data_f64(
    target: u32,
    src_data: *const f64,
    size: usize,
    usage: u32,
    state: *mut WebGLState,
) {
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_data_f64(
        target,
        src_data,
        usage,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_data_none(
    target: u32,
    size: isize,
    usage: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_data_none(
        target,
        size,
        usage,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_sub_data(
    target: u32,
    offset: isize,
    src_data: *const u8,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_sub_data(
        target,
        offset,
        src_data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_sub_data_i8(
    target: u32,
    offset: isize,
    src_data: *const i8,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_sub_data_i8(
        target,
        offset,
        src_data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_sub_data_i16(
    target: u32,
    offset: isize,
    src_data: *const i16,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_sub_data_i16(
        target,
        offset,
        src_data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_sub_data_u16(
    target: u32,
    offset: isize,
    src_data: *const u16,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_sub_data_u16(
        target,
        offset,
        src_data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_sub_data_i32(
    target: u32,
    offset: isize,
    src_data: *const i32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_sub_data_i32(
        target,
        offset,
        src_data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_sub_data_u32(
    target: u32,
    offset: isize,
    src_data: *const u32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_sub_data_u32(
        target,
        offset,
        src_data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_sub_data_f32(
    target: u32,
    offset: isize,
    src_data: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_sub_data_f32(
        target,
        offset,
        src_data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_sub_data_f64(
    target: u32,
    offset: isize,
    src_data: *const f64,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_sub_data_f64(
        target,
        offset,
        src_data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_sub_data_none(
    target: u32,
    offset: isize,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_sub_data_none(
        target,
        offset,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_check_frame_buffer_status(
    target: u32,
    state: *mut WebGLState,
) -> u32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_check_frame_buffer_status(
        target,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_clear(mask: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_clear(mask, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_clear_color(
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_clear_color(
        red,
        green,
        blue,
        alpha,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_clear_depth(depth: f32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_clear_depth(depth, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_clear_stencil(stencil: i32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_clear_stencil(stencil, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_color_mask(
    red: bool,
    green: bool,
    blue: bool,
    alpha: bool,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_color_mask(
        red,
        green,
        blue,
        alpha,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_commit(_: *mut WebGLState) {
    // noop
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_compile_shader(shader: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_compile_shader(shader, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_compressed_tex_image2d(
    target: u32,
    level: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    border: i32,
    pixels: *const u8,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let pixels = unsafe { std::slice::from_raw_parts(pixels, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_compressed_tex_image2d(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        pixels,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_compressed_tex_image2d_none(
    target: u32,
    level: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    border: i32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_compressed_tex_image2d_none(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_compressed_tex_sub_image2d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    width: i32,
    height: i32,
    format: u32,
    pixels: *const u8,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let pixels = unsafe { std::slice::from_raw_parts(pixels, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_compressed_tex_sub_image2d(
        target,
        level,
        xoffset,
        yoffset,
        width,
        height,
        format,
        pixels,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_copy_tex_image2d(
    target: u32,
    level: i32,
    internalformat: u32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    border: i32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_copy_tex_image2d(
        target,
        level,
        internalformat,
        x,
        y,
        width,
        height,
        border,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_copy_tex_sub_image2d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_copy_tex_sub_image2d(
        target,
        level,
        xoffset,
        yoffset,
        x,
        y,
        width,
        height,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_create_buffer(state: *mut WebGLState) -> u32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_create_buffer(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_create_framebuffer(state: *mut WebGLState) -> u32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_create_framebuffer(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_create_program(state: *mut WebGLState) -> u32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_create_program(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_create_renderbuffer(state: *mut WebGLState) -> u32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_create_renderbuffer(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_create_shader(
    shader_type: u32,
    state: *mut WebGLState,
) -> u32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_create_shader(shader_type, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_create_texture(state: *mut WebGLState) -> u32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_create_texture(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_cull_face(mode: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_cull_face(mode, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_delete_buffer(buffer: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_delete_buffer(buffer, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_delete_framebuffer(
    frame_buffer: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_delete_framebuffer(frame_buffer, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_delete_program(program: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_delete_program(program, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_delete_renderbuffer(
    render_buffer: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_delete_renderbuffer(
        render_buffer,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_delete_shader(shader: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_delete_shader(shader, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_delete_texture(texture: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_delete_texture(texture, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_depth_func(func: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_depth_func(func, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_depth_mask(flag: bool, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_depth_mask(flag, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_depth_range(z_near: f32, z_far: f32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_depth_range(z_near, z_far, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_detach_shader(
    program: u32,
    shader: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_detach_shader(program, shader, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_disable(cap: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_disable(cap, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_disable_vertex_attrib_array(
    index: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_disable_vertex_attrib_array(
        index,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_draw_arrays(
    mode: u32,
    first: i32,
    count: i32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_draw_arrays(mode, first, count, state.get_inner_mut())
    // Flush Context
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_draw_elements(
    mode: u32,
    count: i32,
    element_type: u32,
    offset: isize,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_draw_elements(
        mode,
        count,
        element_type,
        offset,
        state.get_inner_mut(),
    )
    // Flush Context
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_enable(cap: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_enable(cap, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_enable_vertex_attrib_array(
    index: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_enable_vertex_attrib_array(
        index,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_finish(state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_finish(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_flush(state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_flush(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_framebuffer_renderbuffer(
    target: u32,
    attachment: u32,
    renderbuffertarget: u32,
    renderbuffer: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_framebuffer_renderbuffer(
        target,
        attachment,
        renderbuffertarget,
        renderbuffer,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_framebuffer_texture2d(
    target: u32,
    attachment: u32,
    textarget: u32,
    texture: u32,
    level: i32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_framebuffer_texture2d(
        target,
        attachment,
        textarget,
        texture,
        level,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_front_face(mode: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_front_face(mode, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_generate_mipmap(target: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_generate_mipmap(target, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_active_attrib(
    program: u32,
    index: u32,
    state: *mut WebGLState,
) -> *mut WebGLActiveInfo {
    let state = unsafe { &mut *state };
    let info = canvas_webgl::webgl::canvas_native_webgl_get_active_attrib(
        program,
        index,
        state.get_inner_mut(),
    );

    Box::into_raw(Box::new(WebGLActiveInfo(info)))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_active_uniform(
    program: u32,
    index: u32,
    state: *mut WebGLState,
) -> *mut WebGLActiveInfo {
    let state = unsafe { &mut *state };
    let info = canvas_webgl::webgl::canvas_native_webgl_get_active_uniform(
        program,
        index,
        state.get_inner_mut(),
    );

    Box::into_raw(Box::new(WebGLActiveInfo(info)))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_attached_shaders(
    program: u32,
    state: *mut WebGLState,
) -> *mut U32Buffer {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(U32Buffer::from(
        canvas_webgl::webgl::canvas_native_webgl_get_attached_shaders(
            program,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_attrib_location(
    program: u32,
    name: *const c_char,
    state: *mut WebGLState,
) -> i32 {
    assert!(!state.is_null());
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_string_lossy();
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_get_attrib_location(
        program,
        name.as_ref(),
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_buffer_parameter(
    target: u32,
    pname: u32,
    state: *mut WebGLState,
) -> i32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_get_buffer_parameter(
        target,
        pname,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_context_attributes(
    state: *mut WebGLState,
) -> *mut ContextAttributes {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(ContextAttributes(
        canvas_webgl::webgl::canvas_native_webgl_get_context_attributes(state.get_inner()),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_error(state: *mut WebGLState) -> u32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_get_error(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_extension(
    name: *const c_char,
    state: *mut WebGLState,
) -> *mut WebGLExtension {
    assert!(!state.is_null());
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_string_lossy();
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLExtension(
        canvas_webgl::webgl::canvas_native_webgl_get_extension(
            name.as_ref(),
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_framebuffer_attachment_parameter(
    target: u32,
    attachment: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLFramebufferAttachmentParameter {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLFramebufferAttachmentParameter(
        canvas_webgl::webgl::canvas_native_webgl_get_framebuffer_attachment_parameter(
            target,
            attachment,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_framebuffer_attachment_parameter_get_is_texture(
    param: *const WebGLFramebufferAttachmentParameter,
) -> bool {
    assert!(!param.is_null());
    let param = unsafe { &*param };
    param.get_is_texture()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_framebuffer_attachment_parameter_get_is_renderbuffer(
    param: *const WebGLFramebufferAttachmentParameter,
) -> bool {
    assert!(!param.is_null());
    let param = unsafe { &*param };

    param.get_is_renderbuffer()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_framebuffer_attachment_parameter_get_value(
    param: *const WebGLFramebufferAttachmentParameter,
) -> i32 {
    assert!(!param.is_null());
    let param = unsafe { &*param };
    param.get_value()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_parameter(
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_parameter(pname, state.get_inner_mut()),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_program_info_log(
    program: u32,
    state: *mut WebGLState,
) -> *const c_char {
    let state = unsafe { &mut *state };
    CString::new(
        canvas_webgl::webgl::canvas_native_webgl_get_program_info_log(
            program,
            state.get_inner_mut(),
        ),
    )
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_program_parameter(
    program: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_program_parameter(
            program,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_renderbuffer_parameter(
    target: u32,
    pname: u32,
    state: *mut WebGLState,
) -> i32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_get_renderbuffer_parameter(
        target,
        pname,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_shader_info_log(
    shader: u32,
    state: *mut WebGLState,
) -> *const c_char {
    let state = unsafe { &mut *state };
    CString::new(
        canvas_webgl::webgl::canvas_native_webgl_get_shader_info_log(shader, state.get_inner_mut()),
    )
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_shader_parameter(
    shader: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_shader_parameter(
            shader,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_shader_precision_format(
    shader_type: u32,
    precision_type: u32,
    state: *mut WebGLState,
) -> *mut WebGLShaderPrecisionFormat {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLShaderPrecisionFormat(
        canvas_webgl::webgl::canvas_native_webgl_get_shader_precision_format(
            shader_type,
            precision_type,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_shader_source(
    shader: u32,
    state: *mut WebGLState,
) -> *const c_char {
    let state = unsafe { &mut *state };
    CString::new(canvas_webgl::webgl::canvas_native_webgl_get_shader_source(
        shader,
        state.get_inner_mut(),
    ))
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_supported_extensions(
    state: *mut WebGLState,
) -> *mut StringBuffer {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(StringBuffer::from(
        canvas_webgl::webgl::canvas_native_webgl_get_supported_extensions(state.get_inner_mut()),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_supported_extensions_to_string(
    state: *mut WebGLState,
) -> *const c_char {
    let state = unsafe { &mut *state };
    let ret =
        canvas_webgl::webgl::canvas_native_webgl_get_supported_extensions(state.get_inner_mut());

    CString::new(ret.join(",").to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_tex_parameter(
    target: u32,
    pname: u32,
    state: *mut WebGLState,
) -> i32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_get_tex_parameter(target, pname, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_uniform_location(
    program: u32,
    name: *const c_char,
    state: *mut WebGLState,
) -> i32 {
    assert!(!state.is_null());
    assert!(!name.is_null());
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_string_lossy();
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_get_uniform_location(
        program,
        name.as_ref(),
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_uniform(
    program: u32,
    location: i32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    let state = unsafe { &mut *state };

    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_uniform(
            program,
            location,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_vertex_attrib_offset(
    index: u32,
    pname: u32,
    state: *mut WebGLState,
) -> usize {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_get_vertex_attrib_offset(
        index,
        pname,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_vertex_attrib(
    index: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_vertex_attrib(
            index,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_is_context_lost(_: *mut WebGLState) -> bool {
    // TODO improve
    false
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_hint(target: u32, mode: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_hint(target, mode, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_is_buffer(buffer: u32, state: *mut WebGLState) -> bool {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_is_buffer(buffer, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_is_enabled(cap: u32, state: *mut WebGLState) -> bool {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_is_enabled(cap, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_is_framebuffer(
    framebuffer: u32,
    state: *mut WebGLState,
) -> bool {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_is_framebuffer(framebuffer, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_is_program(program: u32, state: *mut WebGLState) -> bool {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_is_program(program, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_is_renderbuffer(
    renderbuffer: u32,
    state: *mut WebGLState,
) -> bool {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_is_renderbuffer(renderbuffer, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_is_shader(shader: u32, state: *mut WebGLState) -> bool {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_is_shader(shader, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_is_texture(texture: u32, state: *mut WebGLState) -> bool {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_is_texture(texture, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_line_width(width: f32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_line_width(width, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_link_program(program: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_link_program(program, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_pixel_storei(pname: u32, param: i32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_pixel_storei(pname, param, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_polygon_offset(
    factor: f32,
    units: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_polygon_offset(factor, units, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_read_pixels_u8(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    pixel_type: u32,
    pixels: *mut u8,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    let pixels = unsafe { std::slice::from_raw_parts_mut(pixels, size) };
    canvas_webgl::webgl::canvas_native_webgl_read_pixels_u8(
        x,
        y,
        width,
        height,
        format,
        pixel_type,
        pixels,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_read_pixels_u16(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    pixel_type: u32,
    pixels: *mut u16,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    let pixels = unsafe { std::slice::from_raw_parts_mut(pixels, size) };
    canvas_webgl::webgl::canvas_native_webgl_read_pixels_u16(
        x,
        y,
        width,
        height,
        format,
        pixel_type,
        pixels,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_read_pixels_f32(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    pixel_type: u32,
    pixels: *mut f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    let pixels = unsafe { std::slice::from_raw_parts_mut(pixels, size) };
    canvas_webgl::webgl::canvas_native_webgl_read_pixels_f32(
        x,
        y,
        width,
        height,
        format,
        pixel_type,
        pixels,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_renderbuffer_storage(
    target: u32,
    internal_format: u32,
    width: i32,
    height: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_renderbuffer_storage(
        target,
        internal_format,
        width,
        height,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_sample_coverage(
    value: f32,
    invert: bool,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_sample_coverage(value, invert, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_scissor(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_scissor(x, y, width, height, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_shader_source(
    shader: u32,
    source: *const c_char,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!source.is_null());
    let source = unsafe { CStr::from_ptr(source) };
    let source = source.to_string_lossy();
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_shader_source(
        shader,
        source.as_ref(),
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_stencil_func(
    func: u32,
    reference: i32,
    mask: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_stencil_func(
        func,
        reference,
        mask,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_stencil_func_separate(
    face: u32,
    func: u32,
    reference: i32,
    mask: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_stencil_func_separate(
        face,
        func,
        reference,
        mask,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_stencil_mask(mask: u32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_stencil_mask(mask, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_stencil_mask_separate(
    face: u32,
    mask: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_stencil_mask_separate(
        face,
        mask,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_stencil_op(
    fail: u32,
    zfail: u32,
    zpass: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_stencil_op(fail, zfail, zpass, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_stencil_op_separate(
    face: u32,
    fail: u32,
    zfail: u32,
    zpass: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_stencil_op_separate(
        face,
        fail,
        zfail,
        zpass,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_image2d_image_none(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d_image_none(
        target,
        level,
        internalformat,
        format,
        image_type,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_image2d_canvas2d(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    canvas: *mut CanvasRenderingContext2D,
    state: *mut WebGLState,
) {
    assert!(!canvas.is_null());
    assert!(!state.is_null());
    let canvas = unsafe { &mut *canvas };
    let state = unsafe { &mut *state };

    let (width, height) = canvas.context.dimensions();

    let mut bytes = vec![0u8; (width * height * 4.) as usize];

    canvas.context.get_pixels(bytes.as_mut_slice(), (0, 0), (width as i32, height as i32));

    state.0.make_current();

    canvas_webgl::webgl::canvas_native_webgl_tex_image2d(
        target,
        level,
        internalformat,
        width as i32,
        height as i32,
        0,
        format,
        image_type,
        bytes.as_slice(),
        state.get_inner_mut(),
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_image2d_webgl(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    webgl: *mut WebGLState,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!webgl.is_null());
    let state = unsafe { &mut *state };
    let webgl = unsafe { &mut *webgl };
    let mut pixels = canvas_webgl::webgl::canvas_native_webgl_read_webgl_pixels(
        &mut webgl.0,
        &mut state.0,
        internalformat,
        format,
    );
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d(
        target,
        level,
        internalformat,
        pixels.0,
        pixels.1,
        0,
        format,
        image_type,
        pixels.2.as_mut_slice(),
        state.get_inner_mut(),
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_image2d(
    target: i32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    border: i32,
    format: i32,
    image_type: i32,
    buf: *const u8,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    let buf = unsafe { std::slice::from_raw_parts(buf, size) };
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        format,
        image_type,
        buf,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_image2d_none(
    target: i32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    border: i32,
    format: i32,
    image_type: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d_none(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        format,
        image_type,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_image2d_image_asset(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    image_asset: *const ImageAsset,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!image_asset.is_null());
    let image_asset = unsafe { &*image_asset };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d_asset(
        target,
        level,
        internalformat,
        format,
        image_type,
        &image_asset.0,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_parameterf(
    target: u32,
    pname: u32,
    param: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_tex_parameterf(target, pname, param, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_parameteri(
    target: u32,
    pname: u32,
    param: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_tex_parameteri(target, pname, param, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_sub_image2d_asset(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    format: u32,
    image_type: i32,
    asset: *const ImageAsset,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!asset.is_null());
    let asset = unsafe { &*asset };
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_tex_sub_image2d_asset(
        target,
        level,
        xoffset,
        yoffset,
        format,
        image_type,
        &asset.0,
        state.get_inner(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_sub_image2d_canvas2d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    format: u32,
    image_type: i32,
    canvas: *mut CanvasRenderingContext2D,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!canvas.is_null());
    let state = unsafe { &mut *state };
    let canvas = unsafe { &mut *canvas };

    let (width, height) = canvas.context.dimensions();

    let mut bytes = vec![0u8; (width * height * 4.) as usize];

    canvas.context.get_pixels(bytes.as_mut_slice(), (0, 0), (width as i32, height as i32));

    state.0.make_current();

    canvas_webgl::webgl::canvas_native_webgl_tex_sub_image2d(
        target,
        level,
        xoffset,
        yoffset,
        width as i32,
        height as i32,
        format,
        image_type,
        bytes.as_slice(),
        state.get_inner_mut(),
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_sub_image2d_webgl(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    format: u32,
    image_type: i32,
    webgl: *mut WebGLState,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!webgl.is_null());
    let state = unsafe { &mut *state };
    let webgl = unsafe { &mut *webgl };
    {
        let state = &state.0;
        state.remove_if_current();
    }
    let source = webgl.get_inner();
    source.make_current();
    let width = source.drawing_buffer_width();
    let height = source.drawing_buffer_height();

    let mut pixels = canvas_webgl::webgl::canvas_native_webgl_read_webgl_pixels(
        &mut webgl.0,
        &mut state.0,
        image_type,
        format as i32,
    );

    canvas_webgl::webgl::canvas_native_webgl_tex_sub_image2d(
        target,
        level,
        xoffset,
        yoffset,
        width,
        height,
        format,
        image_type,
        pixels.2.as_mut_slice(),
        state.get_inner_mut(),
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_sub_image2d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    width: i32,
    height: i32,
    format: u32,
    image_type: i32,
    buf: *const u8,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let buf = unsafe { std::slice::from_raw_parts(buf, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_tex_sub_image2d(
        target,
        level,
        xoffset,
        yoffset,
        width,
        height,
        format,
        image_type,
        buf,
        state.get_inner(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform1f(location: i32, v0: f32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform1f(location, v0, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform1fv(
    location: i32,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform1fv(location, value, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform1i(location: i32, v0: i32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform1i(location, v0, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform1iv(
    location: i32,
    value: *const i32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform1iv(location, value, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform2f(
    location: i32,
    v0: f32,
    v1: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform2f(location, v0, v1, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform2fv(
    location: i32,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &*state };

    canvas_webgl::webgl::canvas_native_webgl_uniform2fv(location, value, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform2i(
    location: i32,
    v0: i32,
    v1: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform2i(location, v0, v1, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform2iv(
    location: i32,
    value: *const i32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform2iv(location, value, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform3f(
    location: i32,
    v0: f32,
    v1: f32,
    v2: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform3f(location, v0, v1, v2, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform3fv(
    location: i32,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform3fv(location, value, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform3i(
    location: i32,
    v0: i32,
    v1: i32,
    v2: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform3i(location, v0, v1, v2, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform3iv(
    location: i32,
    value: *const i32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform3iv(location, value, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform4f(
    location: i32,
    v0: f32,
    v1: f32,
    v2: f32,
    v3: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform4f(location, v0, v1, v2, v3, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform4fv(
    location: i32,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform4fv(location, value, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform4i(
    location: i32,
    v0: i32,
    v1: i32,
    v2: i32,
    v3: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform4i(
        location,
        v0,
        v1,
        v2,
        v3,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform4iv(
    location: i32,
    value: *const i32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform4iv(location, value, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform_matrix2fv(
    location: i32,
    transpose: bool,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform_matrix2fv(
        location,
        transpose,
        value,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform_matrix3fv(
    location: i32,
    transpose: bool,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform_matrix3fv(
        location,
        transpose,
        value,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform_matrix4fv(
    location: i32,
    transpose: bool,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform_matrix4fv(
        location,
        transpose,
        value,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_use_program(program: u32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_use_program(program, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_validate_program(program: u32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_validate_program(program, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib1f(index: u32, v0: f32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib1f(index, v0, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib1fv(
    index: u32,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib1fv(index, value, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib2f(
    index: u32,
    v0: f32,
    v1: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib2f(index, v0, v1, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib2fv(
    index: u32,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib2fv(index, value, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib3f(
    index: u32,
    v0: f32,
    v1: f32,
    v2: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib3f(
        index,
        v0,
        v1,
        v2,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib3fv(
    index: u32,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib3fv(index, value, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib4f(
    index: u32,
    v0: f32,
    v1: f32,
    v2: f32,
    v3: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib4f(
        index,
        v0,
        v1,
        v2,
        v3,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib4fv(
    index: u32,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib4fv(index, value, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib_pointer(
    index: u32,
    size: i32,
    d_type: u32,
    normalized: bool,
    stride: i32,
    offset: isize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib_pointer(
        index,
        size,
        d_type,
        normalized,
        stride,
        offset,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_viewport(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_viewport(x, y, width, height, state.get_inner_mut())
}
