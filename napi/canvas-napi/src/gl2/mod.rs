pub mod webgl_query;
pub mod webgl_sampler;
pub mod webgl_sync;
pub mod webgl_transform_feedback;
pub mod webgl_vertex_array_object;

use napi::*;
use napi_derive::napi;
use std::ops::Deref;

use crate::gl::*;

use crate::gl2::webgl_query::WebGLQuery;
use crate::gl2::webgl_sampler::WebGLSampler;
use crate::gl2::webgl_sync::WebGLSync;
use crate::gl2::webgl_transform_feedback::WebGLTransformFeedback;
use crate::gl2::webgl_vertex_array_object::WebGLVertexArrayObject;
use napi::{Either, Env};

use crate::{impl_webgl2_context_constants, impl_webgl_context, impl_webgl_context_constants};

#[napi(custom_finalize)]
pub struct web_g_l_2_rendering_context {
  state: *mut WebGLState,
}

impl_webgl_context!(web_g_l_2_rendering_context);

impl ObjectFinalize for web_g_l_2_rendering_context {
  fn finalize(self, _: Env) -> napi::Result<()> {
    canvas_c::canvas_native_webgl_state_destroy(self.state);
    Ok(())
  }
}

#[napi]
impl web_g_l_2_rendering_context {
  #[napi(factory)]
  pub fn with_view(
    view: i64,
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
  ) -> Result<Self> {
    let ret = canvas_c::canvas_native_webgl_create(
      view as _,
      2,
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
    );

    if ret.is_null() {
      return Err(napi::Error::from_reason("Invalid parameter"));
    }

    Ok(web_g_l_2_rendering_context { state: ret })
  }

  #[napi(factory)]
  pub fn offscreen(
    width: i32,
    height: i32,
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
  ) -> Result<Self> {
    let ret = canvas_c::canvas_native_webgl_create_no_window(
      width,
      height,
      2,
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
    );

    if ret.is_null() {
      return Err(napi::Error::from_reason("Invalid parameter"));
    }

    Ok(web_g_l_2_rendering_context { state: ret })
  }

  /* Transform feedback */

  #[napi]
  pub fn begin_query(&self, target: u32, query: ClassInstance<WebGLQuery>) {
    canvas_c::canvas_native_webgl2_begin_query(target, query.0, self.state);
  }

  #[napi]
  pub fn begin_transform_feedback(&self, primitive_mode: u32) {
    canvas_c::canvas_native_webgl2_begin_transform_feedback(primitive_mode, self.state);
  }

  #[napi]
  pub fn bind_buffer_base(&self, target: u32, index: u32, buffer: ClassInstance<WebGLBuffer>) {
    canvas_c::canvas_native_webgl2_bind_buffer_base(target, index, buffer.0, self.state);
  }

  #[napi]
  pub fn bind_buffer_range(
    &self,
    target: u32,
    index: u32,
    buffer: ClassInstance<WebGLBuffer>,
    offset: i64,
    size: i64,
  ) {
    canvas_c::canvas_native_webgl2_bind_buffer_range(
      target,
      index,
      buffer.0,
      offset as isize,
      size as isize,
      self.state,
    );
  }

  #[napi]
  pub fn bind_sampler(&self, unit: u32, sampler: ClassInstance<WebGLSampler>) {
    canvas_c::canvas_native_webgl2_bind_sampler(unit, sampler.0, self.state);
  }

  #[napi]
  pub fn bind_transform_feedback(&self, target: u32, transform_feedback: &WebGLTransformFeedback) {
    canvas_c::canvas_native_webgl2_bind_transform_feedback(
      target,
      transform_feedback.0,
      self.state,
    );
  }

  #[napi]
  pub fn bind_vertex_array(&self, vertex_array: &WebGLVertexArrayObject) {
    canvas_c::canvas_native_webgl2_bind_vertex_array(vertex_array.0, self.state);
  }

  #[napi]
  pub fn blit_framebuffer(
    &self,
    src_x0: i32,
    src_y0: i32,
    src_x1: i32,
    src_y1: i32,
    dst_x0: i32,
    dst_y0: i32,
    dst_x1: i32,
    dst_y1: i32,
    mask: u32,
    filter: u32,
  ) {
    canvas_c::canvas_native_webgl2_blit_framebuffer(
      src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter, self.state,
    );
  }

  #[napi]
  pub fn clear_bufferfi(
    &self,
    buffer: ClassInstance<WebGLBuffer>,
    drawbuffer: i32,
    depth: f64,
    stencil: i32,
  ) {
    canvas_c::canvas_native_webgl2_clear_bufferfi(
      buffer.0,
      drawbuffer,
      depth as f32,
      stencil,
      self.state,
    );
  }

  #[napi]
  pub fn clear_bufferfv(
    &self,
    buffer: ClassInstance<WebGLBuffer>,
    drawbuffer: i32,
    values: Either<Vec<f64>, Float32Array>,
  ) {
    match values {
      Either::A(array) => {
        let array = array.into_iter().map(|v| v as f32).collect::<Vec<_>>();
        canvas_c::canvas_native_webgl2_clear_bufferfv(
          buffer.0,
          drawbuffer,
          array.as_ptr(),
          array.len(),
          self.state,
        )
      }
      Either::B(value) => canvas_c::canvas_native_webgl2_clear_bufferfv(
        buffer.0,
        drawbuffer,
        value.as_ptr(),
        value.len(),
        self.state,
      ),
    }
  }

  #[napi]
  pub fn clear_bufferiv(
    &self,
    buffer: ClassInstance<WebGLBuffer>,
    drawbuffer: i32,
    values: Either<Vec<i32>, Int32Array>,
  ) {
    match values {
      Either::A(array) => canvas_c::canvas_native_webgl2_clear_bufferiv(
        buffer.0,
        drawbuffer,
        array.as_ptr(),
        array.len(),
        self.state,
      ),
      Either::B(value) => canvas_c::canvas_native_webgl2_clear_bufferiv(
        buffer.0,
        drawbuffer,
        value.as_ptr(),
        value.len(),
        self.state,
      ),
    }
  }

  #[napi]
  pub fn clear_bufferuiv(
    &self,
    buffer: ClassInstance<WebGLBuffer>,
    drawbuffer: i32,
    values: Either<Vec<u32>, Uint32Array>,
  ) {
    match values {
      Either::A(array) => canvas_c::canvas_native_webgl2_clear_bufferuiv(
        buffer.0,
        drawbuffer,
        array.as_ptr(),
        array.len(),
        self.state,
      ),
      Either::B(value) => canvas_c::canvas_native_webgl2_clear_bufferuiv(
        buffer.0,
        drawbuffer,
        value.as_ptr(),
        value.len(),
        self.state,
      ),
    }
  }

  #[napi]
  pub fn client_wait_sync(&self, sync: &WebGLSync, flags: u32, timeout: i64) -> u32 {
    canvas_c::canvas_native_webgl2_client_wait_sync(sync.0, flags, timeout as isize, self.state)
  }

  #[napi]
  pub fn compressed_tex_image_3_d(
    &self,
    target: u32,
    level: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    image_size_or_src_data: Either<i32, Buffer>,
    src_offset: Option<i64>,
    src_length_override: Option<i64>,
  ) {
    match image_size_or_src_data {
      Either::A(size) => canvas_c::canvas_native_webgl2_compressed_tex_image3d_none(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        size,
        src_offset.unwrap_or_default() as usize,
        self.state,
      ),
      Either::B(buffer) => canvas_c::canvas_native_webgl2_compressed_tex_image3d(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        buffer.as_ptr(),
        buffer.len(),
        src_offset.unwrap_or_default() as usize,
        src_length_override.unwrap_or_default() as usize,
        self.state,
      ),
    }
  }

  #[napi]
  pub fn compressed_tex_sub_image_3_d(
    &self,
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    image_size_or_src_data: Either<i32, Buffer>,
    src_offset: Option<i64>,
    src_length_override: Option<i64>,
  ) {
    match image_size_or_src_data {
      Either::A(size) => canvas_c::canvas_native_webgl2_compressed_tex_sub_image3d_none(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        size,
        src_offset.unwrap_or_default() as usize,
        self.state,
      ),
      Either::B(buffer) => canvas_c::canvas_native_webgl2_compressed_tex_sub_image3d(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        buffer.as_ptr(),
        buffer.len(),
        src_offset.unwrap_or_default() as usize,
        src_length_override.unwrap_or_default() as usize,
        self.state,
      ),
    }
  }

  /* Transform feedback */

  /* Framebuffers and renderbuffers */

  #[napi]
  pub fn copy_buffer_sub_data(
    &self,
    read_target: u32,
    write_target: u32,
    read_offset: i64,
    write_offset: i64,
    size: i64,
  ) {
    canvas_c::canvas_native_webgl2_copy_buffer_sub_data(
      read_target,
      write_target,
      read_offset as isize,
      write_offset as isize,
      size as isize,
      self.state,
    )
  }

  #[napi]
  pub fn copy_tex_sub_image_3_d(
    &self,
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
  ) {
    canvas_c::canvas_native_webgl2_copy_tex_sub_image3d(
      target, level, xoffset, yoffset, zoffset, x, y, width, height, self.state,
    );
  }

  #[napi]
  pub fn create_query(&self, env: Env) -> Result<ClassInstance<WebGLQuery>> {
    WebGLQuery(canvas_c::canvas_native_webgl2_create_query(self.state)).into_instance(env)
  }

  #[napi]
  pub fn create_sampler(&self, env: Env) -> Result<ClassInstance<WebGLSampler>> {
    WebGLSampler(canvas_c::canvas_native_webgl2_create_sampler(self.state)).into_instance(env)
  }

  #[napi]
  pub fn create_transform_feedback(
    &self,
    env: Env,
  ) -> Result<ClassInstance<WebGLTransformFeedback>> {
    WebGLTransformFeedback(canvas_c::canvas_native_webgl2_create_transform_feedback(
      self.state,
    ))
    .into_instance(env)
  }

  #[napi]
  pub fn create_vertex_array(&self, env: Env) -> Result<ClassInstance<WebGLVertexArrayObject>> {
    WebGLVertexArrayObject(canvas_c::canvas_native_webgl2_create_vertex_array(
      self.state,
    ))
    .into_instance(env)
  }

  #[napi]
  pub fn delete_query(&self, query: ClassInstance<WebGLQuery>) {
    canvas_c::canvas_native_webgl2_delete_query_with_query(query.0, self.state);
  }

  #[napi]
  pub fn delete_sampler(&self, sampler: &WebGLSampler) {
    canvas_c::canvas_native_webgl2_delete_sampler_with_sampler(sampler.0, self.state);
  }

  #[napi]
  pub fn delete_sync(&self, sync: &WebGLSync) {
    canvas_c::canvas_native_webgl2_delete_sync_with_sync(sync.0, self.state);
  }

  #[napi]
  pub fn delete_transform_feedback(
    &self,
    transform_feedback: ClassInstance<WebGLTransformFeedback>,
  ) {
    canvas_c::canvas_native_webgl2_delete_transform_feedback(transform_feedback.0, self.state);
  }

  #[napi]
  pub fn delete_vertex_array(&self, vertex_array: &WebGLVertexArrayObject) {
    canvas_c::canvas_native_webgl2_delete_vertex_array_with_vertex_array(
      vertex_array.0,
      self.state,
    );
  }

  #[napi]
  pub fn draw_arrays_instanced(&self, mode: u32, first: i32, count: i32, instance_count: i32) {
    canvas_c::canvas_native_webgl2_draw_arrays_instanced(
      mode,
      first,
      count,
      instance_count,
      self.state,
    )
  }

  #[napi]
  pub fn draw_buffers(&self, buffers: Vec<u32>) {
    canvas_c::canvas_native_webgl2_draw_buffers(buffers.as_ptr(), buffers.len(), self.state)
  }

  #[napi(
    ts_args_type = "mode: number, count: number, type: number, offset: number, instanceCount: number"
  )]
  pub fn draw_elements_instanced(
    &self,
    mode: u32,
    count: i32,
    type_: u32,
    offset: i64,
    instance_count: i32,
  ) {
    canvas_c::canvas_native_webgl2_draw_elements_instanced(
      mode,
      count,
      type_,
      offset as isize,
      instance_count,
      self.state,
    )
  }

  #[napi(
    ts_args_type = "mode: number, start: number,end: number, count: number, type: number, offset: number"
  )]
  pub fn draw_range_elements(
    &self,
    mode: u32,
    start: u32,
    end: u32,
    count: i32,
    type_: u32,
    offset: i64,
  ) {
    canvas_c::canvas_native_webgl2_draw_range_elements(
      mode,
      start,
      end,
      count,
      type_,
      offset as isize,
      self.state,
    )
  }

  #[napi]
  pub fn end_query(&self, target: u32) {
    canvas_c::canvas_native_webgl2_end_query(target, self.state);
  }

  #[napi]
  pub fn end_transform_feedback(&self) {
    canvas_c::canvas_native_webgl2_end_transform_feedback(self.state);
  }

  #[napi]
  pub fn fence_sync(
    &self,
    env: Env,
    condition: u32,
    flags: u32,
  ) -> Result<ClassInstance<WebGLSync>> {
    WebGLSync(canvas_c::canvas_native_webgl2_fence_sync(
      condition, flags, self.state,
    ))
    .into_instance(env)
  }

  #[napi]
  pub fn framebuffer_texture_layer(
    &self,
    target: u32,
    attachment: u32,
    texture: ClassInstance<WebGLTexture>,
    level: i32,
    layer: i32,
  ) {
    canvas_c::canvas_native_webgl2_framebuffer_texture_layer(
      target, attachment, texture.0, level, level, self.state,
    )
  }

  /* Framebuffers and renderbuffers */

  /* Uniforms */

  #[napi]
  pub fn get_active_uniform_block_name(
    &self,
    program: ClassInstance<WebGLProgram>,
    uniform_block_index: u32,
  ) -> String {
    let state = unsafe { &mut *self.state };

    canvas_webgl::webgl2::canvas_native_webgl2_get_active_uniform_block_name(
      program.0,
      uniform_block_index,
      state.get_inner_mut(),
    )
  }

  #[napi]
  pub fn get_active_uniform_block_parameter(
    &self,
    env: Env,
    program: ClassInstance<WebGLProgram>,
    uniform_block_index: u32,
    pname: u32,
  ) -> Result<Unknown> {
    let result = canvas_c::canvas_native_webgl2_get_active_uniform_block_parameter(
      program.0,
      uniform_block_index,
      pname,
      self.state,
    );

    match pname {
      gl_bindings::UNIFORM_BLOCK_BINDING
      | gl_bindings::UNIFORM_BLOCK_DATA_SIZE
      | gl_bindings::UNIFORM_BLOCK_ACTIVE_UNIFORMS => {
        let ret = canvas_c::canvas_native_webgl_result_get_i32(result);
        canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
        env.create_int32(ret).map(|v| v.into_unknown())
      }
      gl_bindings::UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES => unsafe {
        let ret = canvas_c::canvas_native_webgl_result_into_u32_array(result);

        if ret.is_null() {
          return env.get_null().map(|v| v.into_unknown());
        }

        let ret = *Box::from_raw(ret);
        let mut ret = ret.into_vec();

        let ptr = ret.as_mut_ptr();
        let len = ret.len();

        let buffer = env.create_arraybuffer_with_borrowed_data(
          ptr as _,
          len * size_of::<u32>(),
          ret,
          |_, _| {},
        )?;
        buffer
          .value
          .into_typedarray(TypedArrayType::Uint32, len, 0)
          .map(|v| v.into_unknown())
      },
      gl_bindings::UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER
      | gl_bindings::UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER => {
        let ret = canvas_c::canvas_native_webgl_result_get_bool(result);
        canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
        env.get_boolean(ret).map(|v| v.into_unknown())
      }
      _ => env.get_null().map(|v| v.into_unknown()),
    }
  }

  #[napi]
  pub fn get_active_uniforms(
    &self,
    env: Env,
    program: ClassInstance<WebGLProgram>,
    uniform_indices: Vec<u32>,
    pname: u32,
  ) -> Result<Unknown> {
    let result = canvas_c::canvas_native_webgl2_get_active_uniforms(
      program.0,
      uniform_indices.as_ptr(),
      uniform_indices.len(),
      pname,
      self.state,
    );

    match pname {
      gl_bindings::UNIFORM_TYPE | gl_bindings::UNIFORM_SIZE => unsafe {
        let ret = canvas_c::canvas_native_webgl_result_into_u32_array(result);

        if ret.is_null() {
          return env.get_null().map(|v| v.into_unknown());
        }

        let ret = *Box::from_raw(ret);
        let mut ret = ret.into_vec();

        let ptr = ret.as_mut_ptr();
        let len = ret.len();

        let buffer = env.create_arraybuffer_with_borrowed_data(
          ptr as _,
          len * size_of::<u32>(),
          ret,
          |_, _| {},
        )?;
        buffer
          .value
          .into_typedarray(TypedArrayType::Uint32, len, 0)
          .map(|v| v.into_unknown())
      },
      gl_bindings::UNIFORM_BLOCK_INDEX
      | gl_bindings::UNIFORM_OFFSET
      | gl_bindings::UNIFORM_ARRAY_STRIDE
      | gl_bindings::UNIFORM_MATRIX_STRIDE => unsafe {
        let ret = canvas_c::canvas_native_webgl_result_into_i32_array(result);

        if ret.is_null() {
          return env.get_null().map(|v| v.into_unknown());
        }

        let ret = *Box::from_raw(ret);
        let mut ret = ret.into_vec();

        let ptr = ret.as_mut_ptr();
        let len = ret.len();

        let buffer = env.create_arraybuffer_with_borrowed_data(
          ptr as _,
          len * size_of::<i32>(),
          ret,
          |_, _| {},
        )?;
        buffer
          .value
          .into_typedarray(TypedArrayType::Int32, len, 0)
          .map(|v| v.into_unknown())
      },
      gl_bindings::UNIFORM_IS_ROW_MAJOR => {
        let ret = canvas_c::canvas_native_webgl_result_into_bool_array(result);

        if ret.is_null() {
          return env.get_null().map(|v| v.into_unknown());
        }

        let ret = unsafe { *Box::from_raw(ret) };
        let mut ret = ret.into_vec();
        Array::from_vec(&env, ret)?
          .coerce_to_object()
          .map(|v| v.into_unknown())
      }
      _ => env.get_null().map(|v| v.into_unknown()),
    }
  }

  #[napi]
  pub fn get_buffer_sub_data(
    &self,
    target: u32,
    src_byte_offset: i64,
    mut dst_data: Buffer,
    dst_offset: Option<i64>,
    length: Option<i64>,
  ) {
    canvas_c::canvas_native_webgl2_get_buffer_sub_data(
      target,
      src_byte_offset as isize,
      dst_data.as_mut_ptr(),
      dst_data.len(),
      dst_offset.unwrap_or_default() as usize,
      length.unwrap_or_default() as usize,
      self.state,
    )
  }

  #[napi]
  pub fn get_frag_data_location(
    &self,
    program: ClassInstance<WebGLProgram>,
    name: String,
  ) -> Option<u32> {
    let state = unsafe { &mut *self.state };
    let ret = canvas_webgl::webgl2::canvas_native_webgl2_get_frag_data_location(
      program.0,
      name.as_str(),
      state.get_inner_mut(),
    );
    if ret != -1 {
      Some(ret as u32)
    } else {
      None
    }
  }

  #[napi]
  pub fn get_indexed_parameter(&self, env: Env, target: u32, index: u32) -> Result<Unknown> {
    let state = unsafe { &mut *self.state };
    let ret = canvas_webgl::webgl2::canvas_native_webgl2_get_indexed_parameter(
      target,
      index,
      state.get_inner_mut(),
    );

    if ret.get_is_buffer() {
      WebGLBuffer(ret.get_buffer_value() as u32)
        .into_instance(env)
        .map(|v| v.as_object(env).into_unknown())
    } else {
      env
        .create_int64(ret.get_value() as i64)
        .map(|v| v.into_unknown())
    }
  }

  #[napi]
  pub fn get_internalformat_parameter(
    &self,
    env: Env,
    target: u32,
    internalformat: u32,
    pname: u32,
  ) -> Result<Unknown> {
    match internalformat {
      gl_bindings::RGB
      | gl_bindings::RGBA
      | gl_bindings::R8UI
      | gl_bindings::R8I
      | gl_bindings::R16UI
      | gl_bindings::R16I
      | gl_bindings::R32UI
      | gl_bindings::R32I
      | gl_bindings::RG8UI
      | gl_bindings::RG8I
      | gl_bindings::RG16UI
      | gl_bindings::RG16I
      | gl_bindings::RG32UI
      | gl_bindings::RG32I
      | gl_bindings::RGBA8UI
      | gl_bindings::RGBA8I
      | gl_bindings::RGB10_A2UI
      | gl_bindings::RGBA16UI
      | gl_bindings::RGBA16I
      | gl_bindings::RGBA32UI
      | gl_bindings::RGBA32I => {
        return env
          .create_arraybuffer(0)?
          .value
          .into_typedarray(TypedArrayType::Int32, 0, 0)
          .map(|v| v.into_unknown())
      }

      gl_bindings::R8
      | gl_bindings::RG8
      | gl_bindings::RGB565
      | gl_bindings::RGBA8
      | gl_bindings::SRGB8_ALPHA8
      | gl_bindings::RGB5_A1
      | gl_bindings::RGBA4
      | gl_bindings::RGB10_A2
      | gl_bindings::DEPTH_COMPONENT16
      | gl_bindings::DEPTH_COMPONENT24
      | gl_bindings::DEPTH_COMPONENT32F
      | gl_bindings::DEPTH24_STENCIL8
      | gl_bindings::DEPTH32F_STENCIL8
      | gl_bindings::STENCIL_INDEX8 => {}
      gl_bindings::R16F
      | gl_bindings::RG16F
      | gl_bindings::R32F
      | gl_bindings::RG32F
      | gl_bindings::RGBA32F
      | gl_bindings::R11F_G11F_B10F => {}

      _ => return env.get_null().map(|v| v.into_unknown()),
    }

    let result = canvas_c::canvas_native_webgl2_get_internalformat_parameter(
      target,
      internalformat,
      pname,
      self.state,
    );

    if pname == gl_bindings::SAMPLES {
      let ret = canvas_c::canvas_native_webgl_result_into_i32_array(result);

      if ret.is_null() {
        return env.get_null().map(|v| v.into_unknown());
      }

      let ret = unsafe { *Box::from_raw(ret) };
      let mut ret = ret.into_vec();

      let ptr = ret.as_mut_ptr();
      let len = ret.len();

      let buffer = unsafe {
        env.create_arraybuffer_with_borrowed_data(
          ptr as _,
          len * size_of::<i32>(),
          ret,
          |_, _| {},
        )?
      };
      return buffer
        .value
        .into_typedarray(TypedArrayType::Int32, len, 0)
        .map(|v| v.into_unknown());
    }

    canvas_c::canvas_native_webgl_WebGLResult_destroy(result);

    env.get_null().map(|v| v.into_unknown())
  }

  #[napi]
  pub fn get_query_parameter(
    &self,
    env: Env,
    query: ClassInstance<WebGLQuery>,
    pname: u32,
  ) -> Result<Unknown> {
    let result = canvas_c::canvas_native_webgl2_get_query_parameter(query.0, pname, self.state);

    match pname {
      gl_bindings::QUERY_RESULT => {
        let ret = canvas_c::canvas_native_webgl_result_get_bool(result);

        canvas_c::canvas_native_webgl_WebGLResult_destroy(result);

        env.get_boolean(ret).map(|v| v.into_unknown())
      }
      gl_bindings::QUERY_RESULT_AVAILABLE => {
        let ret = canvas_c::canvas_native_webgl_result_get_u32(result);

        canvas_c::canvas_native_webgl_WebGLResult_destroy(result);

        env.create_uint32(ret).map(|v| v.into_unknown())
      }
      _ => env.get_null().map(|v| v.into_unknown()),
    }
  }

  #[napi]
  pub fn get_parameter(&self, env: Env, pname: u32) -> Result<Unknown> {
    let result = canvas_c::canvas_native_webgl2_get_parameter(pname, self.state);
    match pname {
      gl_bindings::COPY_READ_BUFFER_BINDING | gl_bindings::COPY_WRITE_BUFFER_BINDING => {
        let ret = canvas_c::canvas_native_webgl_result_get_i32(result);
        canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
        WebGLBuffer(ret as u32)
          .into_instance(env)
          .map(|v| v.as_object(env).into_unknown())
      }
      gl_bindings::DRAW_FRAMEBUFFER_BINDING => {
        let ret = canvas_c::canvas_native_webgl_result_get_i32(result);
        canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
        WebGLFramebuffer(ret as u32)
          .into_instance(env)
          .map(|v| v.as_object(env).into_unknown())
      }

      _ => get_parameter_inner(self.state, env, pname),
    }
  }

  #[napi]
  pub fn get_query(&self, env: Env, target: u32, pname: u32) -> Result<Unknown> {
    let result = canvas_c::canvas_native_webgl2_get_query(target, pname, self.state);
    if pname == gl_bindings::CURRENT_QUERY {
      let ret = canvas_c::canvas_native_webgl_result_get_i32(result);
      canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
      return WebGLQuery(ret as u32)
        .into_instance(env)
        .map(|v| v.as_object(env).into_unknown());
    }

    env.get_null().map(|v| v.into_unknown())
  }

  #[napi]
  pub fn get_sampler_parameter(
    &self,
    env: Env,
    sampler: &WebGLSampler,
    pname: u32,
  ) -> Result<Unknown> {
    let result = canvas_c::canvas_native_webgl2_get_sampler_parameter(sampler.0, pname, self.state);
    match pname {
      gl_bindings::TEXTURE_MAX_LOD | gl_bindings::TEXTURE_MIN_LOD => {
        let ret = canvas_c::canvas_native_webgl_result_get_f32(result);
        canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
        env.create_double(ret as f64).map(|v| v.into_unknown())
      }
      gl_bindings::TEXTURE_COMPARE_FUNC
      | gl_bindings::TEXTURE_COMPARE_MODE
      | gl_bindings::TEXTURE_MAG_FILTER
      | gl_bindings::TEXTURE_MIN_FILTER
      | gl_bindings::TEXTURE_WRAP_R
      | gl_bindings::TEXTURE_WRAP_S
      | gl_bindings::TEXTURE_WRAP_T => {
        let ret = canvas_c::canvas_native_webgl_result_get_i32(result);
        canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
        env.create_int32(ret).map(|v| v.into_unknown())
      }
      _ => env.get_null().map(|v| v.into_unknown()),
    }
  }

  #[napi]
  pub fn get_sync_parameter(&self, env: Env, sync: &WebGLSync, pname: u32) -> Result<Unknown> {
    let result = canvas_c::canvas_native_webgl2_get_sync_parameter(sync.0, pname, self.state);
    match pname {
      gl_bindings::OBJECT_TYPE
      | gl_bindings::SYNC_STATUS
      | gl_bindings::SYNC_CONDITION
      | gl_bindings::SYNC_FLAGS => {
        let ret = canvas_c::canvas_native_webgl_result_get_i32(result);
        canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
        env.create_int32(ret).map(|v| v.into_unknown())
      }
      _ => env.get_null().map(|v| v.into_unknown()),
    }
  }

  #[napi]
  pub fn get_transform_feedback_varying(
    &self,
    program: ClassInstance<WebGLProgram>,
    index: u32,
  ) -> Option<WebGLActiveInfo> {
    let result =
      canvas_c::canvas_native_webgl2_get_transform_feedback_varying(program.0, index, self.state);
    if result.is_null() {
      return None;
    }
    if canvas_c::canvas_native_webgl_active_info_get_is_empty(result) {
      canvas_c::canvas_native_webgl_active_info_destroy(result);
      return None;
    }
    Some(WebGLActiveInfo(result))
  }

  #[napi]
  pub fn get_uniform_block_index(
    &self,
    program: ClassInstance<WebGLProgram>,
    uniform_block_name: String,
  ) -> u32 {
    let state = unsafe { &mut *self.state };
    canvas_webgl::webgl2::canvas_native_webgl2_get_uniform_block_index(
      program.0,
      uniform_block_name.as_str(),
      state.get_inner_mut(),
    )
  }

  #[napi]
  pub fn get_uniform_indices(
    &self,
    program: &WebGLProgram,
    uniform_names: Vec<String>,
  ) -> Result<Vec<u32>> {
    let state = unsafe { &mut *self.state };
    Ok(
      canvas_webgl::webgl2::canvas_native_webgl2_get_uniform_indices(
        program.0,
        uniform_names.deref(),
        state.get_inner_mut(),
      ),
    )
  }

  #[napi]
  pub fn invalidate_framebuffer(&self, target: u32, attachments: Vec<u32>) {
    let state = unsafe { &mut *self.state };
    canvas_webgl::webgl2::canvas_native_webgl2_invalidate_framebuffer(
      target,
      attachments.as_slice(),
      state.get_inner_mut(),
    )
  }

  #[napi]
  pub fn invalidate_sub_framebuffer(
    &self,
    target: u32,
    attachments: Vec<u32>,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
  ) {
    let state = unsafe { &mut *self.state };
    canvas_webgl::webgl2::canvas_native_webgl2_invalidate_sub_framebuffer(
      target,
      attachments.as_slice(),
      x,
      y,
      width,
      height,
      state.get_inner_mut(),
    )
  }

  #[napi]
  pub fn is_query(&self, query: &WebGLQuery) -> bool {
    canvas_c::canvas_native_webgl2_is_query(query.0, self.state)
  }

  #[napi]
  pub fn is_sampler(&self, sampler: ClassInstance<WebGLSampler>) -> bool {
    canvas_c::canvas_native_webgl2_is_sampler(sampler.0, self.state)
  }

  #[napi]
  pub fn is_sync(&self, sync: &WebGLSync) -> bool {
    canvas_c::canvas_native_webgl2_is_sync(sync.0, self.state)
  }

  #[napi]
  pub fn is_transform_feedback(&self, transform_feedback: &WebGLTransformFeedback) -> bool {
    canvas_c::canvas_native_webgl2_is_transform_feedback(transform_feedback.0, self.state)
  }

  #[napi]
  pub fn is_vertex_array(&self, vertex_array: &WebGLVertexArrayObject) -> bool {
    canvas_c::canvas_native_webgl2_is_vertex_array(vertex_array.0, self.state)
  }

  #[napi]
  pub fn pause_transform_feedback(&self) {
    canvas_c::canvas_native_webgl2_pause_transform_feedback(self.state)
  }

  #[napi]
  pub fn read_buffer(&self, src: u32) {
    canvas_c::canvas_native_webgl2_read_buffer(src, self.state)
  }

  #[napi]
  pub fn renderbuffer_storage_multisample(
    &self,
    target: u32,
    samples: i32,
    internal_format: u32,
    width: i32,
    height: i32,
  ) {
    canvas_c::canvas_native_webgl2_renderbuffer_storage_multisample(
      target,
      samples,
      internal_format,
      width,
      height,
      self.state,
    )
  }

  #[napi]
  pub fn resume_transform_feedback(&self) {
    canvas_c::canvas_native_webgl2_resume_transform_feedback(self.state)
  }

  #[napi]
  pub fn sampler_parameterf(&self, sampler: ClassInstance<WebGLSampler>, pname: u32, param: f64) {
    canvas_c::canvas_native_webgl2_sampler_parameterf(sampler.0, pname, param as f32, self.state)
  }

  /* Uniforms */

  /* Sync objects */

  #[napi]
  pub fn sampler_parameteri(&self, sampler: ClassInstance<WebGLSampler>, pname: u32, param: i32) {
    canvas_c::canvas_native_webgl2_sampler_parameteri(sampler.0, pname, param, self.state)
  }

  #[napi]
  pub fn tex_image_3_d(
    &self,
    target: u32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: u32,
    offset_or_source_or_src_data: Either5<
      i64,
      Buffer,
      &crate::c2d::CanvasRenderingContext2D,
      &web_g_l_rendering_context,
      &web_g_l_2_rendering_context,
    >,
    src_offset: Option<i64>,
  ) {
    match offset_or_source_or_src_data {
      Either5::A(offset) => canvas_c::canvas_native_webgl2_tex_image3d_none(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        offset as usize,
        self.state,
      ),
      Either5::B(buffer) => canvas_c::canvas_native_webgl2_tex_image3d_offset(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        buffer.as_ptr(),
        buffer.len(),
        src_offset.unwrap_or_default() as usize,
        self.state,
      ),
      Either5::C(c2d) => canvas_c::canvas_native_webgl2_tex_image3d_canvas2d(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        c2d.context,
        self.state,
      ),
      Either5::D(gl) => canvas_c::canvas_native_webgl2_tex_image3d_webgl(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        gl.state,
        self.state,
      ),
      Either5::E(gl2) => canvas_c::canvas_native_webgl2_tex_image3d_webgl(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        gl2.state,
        self.state,
      ),
    }
  }

  #[napi]
  pub fn tex_storage_2_d(
    &self,
    target: u32,
    levels: i32,
    internalformat: u32,
    width: i32,
    height: i32,
  ) {
    canvas_c::canvas_native_webgl2_tex_storage2d(
      target,
      levels,
      internalformat,
      width,
      height,
      self.state,
    )
  }

  #[napi]
  pub fn tex_storage_3_d(
    &self,
    target: u32,
    levels: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    depth: i32,
  ) {
    canvas_c::canvas_native_webgl2_tex_storage3d(
      target,
      levels,
      internalformat,
      width,
      height,
      depth,
      self.state,
    )
  }

  #[napi]
  pub fn tex_sub_image_3_d(
    &self,
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    type_: u32,
    src_data: Either5<
      i64,
      Buffer,
      &crate::c2d::CanvasRenderingContext2D,
      &web_g_l_rendering_context,
      &web_g_l_2_rendering_context,
    >,
    src_offset: Option<u32>,
  ) {
    match src_data {
      Either5::A(offset) => canvas_c::canvas_native_webgl2_tex_sub_image3d_none(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        type_,
        offset as usize,
        self.state,
      ),
      Either5::B(buffer) => canvas_c::canvas_native_webgl2_tex_sub_image3d_offset(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        type_,
        buffer.as_ptr(),
        buffer.len(),
        src_offset.unwrap_or_default() as usize,
        self.state,
      ),
      Either5::C(c2d) => canvas_c::canvas_native_webgl2_tex_sub_image3d_canvas2d(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        type_,
        c2d.context,
        self.state,
      ),
      Either5::D(gl) => canvas_c::canvas_native_webgl2_tex_sub_image3d_webgl(
        target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, gl.state,
        self.state,
      ),
      Either5::E(gl2) => canvas_c::canvas_native_webgl2_tex_sub_image3d_webgl(
        target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, gl2.state,
        self.state,
      ),
    }
  }

  #[napi]
  pub fn transform_feedback_varyings(
    &self,
    program: &WebGLProgram,
    varyings: Vec<String>,
    buffer_mode: u32,
  ) {
    let state = unsafe { &mut *self.state };
    canvas_webgl::webgl2::canvas_native_webgl2_transform_feedback_varyings(
      program.0,
      varyings.deref(),
      buffer_mode,
      state.get_inner_mut(),
    )
  }

  #[napi(js_name = "uniform1ui")]
  pub fn uniform1ui(&self, location: &WebGLUniformLocation, v0: u32) {
    canvas_c::canvas_native_webgl2_uniform1ui(location.0, v0, self.state)
  }

  #[napi(js_name = "uniform1uiv")]
  pub fn uniform1uiv(&self, location: &WebGLUniformLocation, data: Either<Vec<u32>, Uint32Array>) {
    match data {
      Either::A(array) => canvas_c::canvas_native_webgl2_uniform1uiv(
        location.0,
        array.as_ptr(),
        array.len(),
        self.state,
      ),
      Either::B(buffer) => canvas_c::canvas_native_webgl2_uniform1uiv(
        location.0,
        buffer.as_ptr(),
        buffer.len(),
        self.state,
      ),
    }
  }

  #[napi(js_name = "uniform2ui")]
  pub fn uniform2ui(&self, location: &WebGLUniformLocation, v0: u32, v1: u32) {
    canvas_c::canvas_native_webgl2_uniform2ui(location.0, v0, v1, self.state)
  }

  #[napi(js_name = "uniform2uiv")]
  pub fn uniform2uiv(&self, location: &WebGLUniformLocation, data: Either<Vec<u32>, Uint32Array>) {
    match data {
      Either::A(array) => canvas_c::canvas_native_webgl2_uniform2uiv(
        location.0,
        array.as_ptr(),
        array.len(),
        self.state,
      ),
      Either::B(buffer) => canvas_c::canvas_native_webgl2_uniform2uiv(
        location.0,
        buffer.as_ptr(),
        buffer.len(),
        self.state,
      ),
    }
  }

  /* Sync objects */

  /* Miscellaneous constants */

  #[napi(js_name = "uniform3ui")]
  pub fn uniform3ui(&self, location: &WebGLUniformLocation, v0: u32, v1: u32, v2: u32) {
    canvas_c::canvas_native_webgl2_uniform3ui(location.0, v0, v1, v2, self.state)
  }

  #[napi(js_name = "uniform3uiv")]
  pub fn uniform3uiv(&self, location: &WebGLUniformLocation, data: Either<Vec<u32>, Uint32Array>) {
    match data {
      Either::A(array) => canvas_c::canvas_native_webgl2_uniform3uiv(
        location.0,
        array.as_ptr(),
        array.len(),
        self.state,
      ),
      Either::B(buffer) => canvas_c::canvas_native_webgl2_uniform3uiv(
        location.0,
        buffer.as_ptr(),
        buffer.len(),
        self.state,
      ),
    }
  }

  #[napi(js_name = "uniform4ui")]
  pub fn uniform4ui(&self, location: &WebGLUniformLocation, v0: u32, v1: u32, v2: u32, v3: u32) {
    canvas_c::canvas_native_webgl2_uniform4ui(location.0, v0, v1, v2, v3, self.state)
  }

  #[napi(js_name = "uniform4uiv")]
  pub fn uniform4uiv(&self, location: &WebGLUniformLocation, data: Either<Vec<u32>, Uint32Array>) {
    match data {
      Either::A(array) => canvas_c::canvas_native_webgl2_uniform4uiv(
        location.0,
        array.as_ptr(),
        array.len(),
        self.state,
      ),
      Either::B(buffer) => canvas_c::canvas_native_webgl2_uniform4uiv(
        location.0,
        buffer.as_ptr(),
        buffer.len(),
        self.state,
      ),
    }
  }

  #[napi]
  pub fn uniform_block_binding(
    &self,
    program: &WebGLProgram,
    uniform_block_index: u32,
    uniform_block_binding: u32,
  ) {
    canvas_c::canvas_native_webgl2_uniform_block_binding(
      program.0,
      uniform_block_index,
      uniform_block_binding,
      self.state,
    )
  }

  #[napi(js_name = "uniformMatrix2x3fv")]
  pub fn uniform_matrix2x3fv(
    &self,
    location: &WebGLUniformLocation,
    transpose: bool,
    data: Float32Array,
  ) {
    canvas_c::canvas_native_webgl2_uniform_matrix2x3fv(
      location.0,
      transpose,
      data.as_ptr(),
      data.len(),
      self.state,
    )
  }

  #[napi(js_name = "uniformMatrix2x4fv")]
  pub fn uniform_matrix2x4fv(
    &self,
    location: &WebGLUniformLocation,
    transpose: bool,
    data: Float32Array,
  ) {
    canvas_c::canvas_native_webgl2_uniform_matrix2x4fv(
      location.0,
      transpose,
      data.as_ptr(),
      data.len(),
      self.state,
    )
  }

  #[napi(js_name = "uniformMatrix3x2fv")]
  pub fn uniform_matrix3x2fv(
    &self,
    location: &WebGLUniformLocation,
    transpose: bool,
    data: Float32Array,
  ) {
    canvas_c::canvas_native_webgl2_uniform_matrix3x2fv(
      location.0,
      transpose,
      data.as_ptr(),
      data.len(),
      self.state,
    )
  }

  #[napi(js_name = "uniformMatrix3x4fv")]
  pub fn uniform_matrix3x4fv(
    &self,
    location: &WebGLUniformLocation,
    transpose: bool,
    data: Float32Array,
  ) {
    canvas_c::canvas_native_webgl2_uniform_matrix3x4fv(
      location.0,
      transpose,
      data.as_ptr(),
      data.len(),
      self.state,
    )
  }

  #[napi(js_name = "uniformMatrix4x2fv")]
  pub fn uniform_matrix4x2fv(
    &self,
    location: &WebGLUniformLocation,
    transpose: bool,
    data: Float32Array,
  ) {
    canvas_c::canvas_native_webgl2_uniform_matrix4x2fv(
      location.0,
      transpose,
      data.as_ptr(),
      data.len(),
      self.state,
    )
  }

  #[napi(js_name = "uniformMatrix4x3fv")]
  pub fn uniform_matrix4x3fv(
    &self,
    location: &WebGLUniformLocation,
    transpose: bool,
    data: Float32Array,
  ) {
    canvas_c::canvas_native_webgl2_uniform_matrix4x3fv(
      location.0,
      transpose,
      data.as_ptr(),
      data.len(),
      self.state,
    )
  }

  #[napi]
  pub fn vertex_attrib_divisor(&self, index: u32, divisor: u32) {
    canvas_c::canvas_native_webgl2_vertex_attrib_divisor(index, divisor, self.state)
  }

  #[napi(js_name = "vertexAttribI4i")]
  pub fn vertex_attrib_i4i(&self, index: u32, v0: i32, v1: i32, v2: i32, v3: i32) {
    canvas_c::canvas_native_webgl2_vertex_attrib_i4i(index, v0, v1, v2, v3, self.state)
  }

  #[napi(js_name = "vertexAttribI4ui")]
  pub fn vertex_attrib_i4ui(&self, index: u32, v0: u32, v1: u32, v2: u32, v3: u32) {
    canvas_c::canvas_native_webgl2_vertex_attrib_i4ui(index, v0, v1, v2, v3, self.state)
  }

  #[napi(js_name = "vertexAttribI4iv")]
  pub fn vertex_attrib_i4iv(&self, index: u32, value: Either<Vec<i32>, Int32Array>) {
    match value {
      Either::A(array) => canvas_c::canvas_native_webgl2_vertex_attrib_i4iv(
        index,
        array.as_ptr(),
        array.len(),
        self.state,
      ),
      Either::B(buffer) => canvas_c::canvas_native_webgl2_vertex_attrib_i4iv(
        index,
        buffer.as_ptr(),
        buffer.len(),
        self.state,
      ),
    }
  }

  #[napi(js_name = "vertexAttribI4uiv")]
  pub fn vertex_attrib_i4uiv(&self, index: u32, value: Either<Vec<u32>, Uint32Array>) {
    match value {
      Either::A(array) => canvas_c::canvas_native_webgl2_vertex_attrib_i4uiv(
        index,
        array.as_ptr(),
        array.len(),
        self.state,
      ),
      Either::B(buffer) => canvas_c::canvas_native_webgl2_vertex_attrib_i4uiv(
        index,
        buffer.as_ptr(),
        buffer.len(),
        self.state,
      ),
    }
  }

  /* Miscellaneous constants */
}

impl_webgl_context_constants!(web_g_l_2_rendering_context);
impl_webgl2_context_constants!(web_g_l_2_rendering_context);
