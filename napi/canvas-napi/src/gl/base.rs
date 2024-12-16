#[macro_export]
macro_rules! impl_webgl_context {
  ($struct_name:ident) => {
    use crate::webgl_context_imports;
    webgl_context_imports!();

    const UNPACK_FLIP_Y_WEBGL: u32 = 37440;
    const UNPACK_PREMULTIPLY_ALPHA_WEBGL: u32 = 37441;
    const UNPACK_COLOR_SPACE_CONVERSION_WEBGL: u32 = 37443;

    #[napi]
    impl $struct_name {

          pub fn update_invalidate_state(&mut self) {
    let state = self.invalidate_state();
    self.invalidate_state = state | InvalidateState::Pending as u32
  }

  pub fn invalidate_state(&self) -> u32 {
    self.invalidate_state
  }

  pub fn set_invalidate_state(&mut self, state: u32) {
    self.invalidate_state = state;
  }


      #[napi]
      pub fn render_immediate(&self) {
        canvas_c::canvas_native_webgl_make_current_and_swap_buffers(self.state);
      }

          #[napi]
          pub fn render(&mut self) {
    let state = self.invalidate_state & InvalidateState::Pending as u32;
    if (state == InvalidateState::Pending as u32) {
      self.invalidate_state = InvalidateState::Invalidating as u32;
      canvas_c::canvas_native_webgl_make_current_and_swap_buffers(self.state);
      self.invalidate_state = InvalidateState::None as u32;
    }
  }


      #[napi(getter)]
      pub fn get_drawing_buffer_width(&self) -> i32 {
        canvas_c::canvas_native_webgl_state_get_drawing_buffer_width(self.state)
      }

      #[napi(getter)]
      pub fn get_drawing_buffer_height(&self) -> i32 {
        canvas_c::canvas_native_webgl_state_get_drawing_buffer_height(self.state)
      }

      #[napi]
      pub fn active_texture(&self, texture: u32) {
        canvas_c::canvas_native_webgl_active_texture(texture, self.state);
      }

      #[napi]
      pub fn attach_shader(
        &self,
        program: ClassInstance<WebGLProgram>,
        shader: ClassInstance<WebGLShader>,
      ) {
        canvas_c::canvas_native_webgl_attach_shader(program.0, shader.0, self.state);
      }

      #[napi]
      pub fn bind_attrib_location(
        &self,
        program: ClassInstance<WebGLProgram>,
        index: u32,
        name: JsString,
      ) -> Result<()> {
        let name = name.into_utf8()?;
        let name = name.as_str()?;
        let state = unsafe { &mut *self.state };
        unsafe {
          canvas_webgl::webgl::canvas_native_webgl_bind_attrib_location(
            program.0,
            index,
            name,
            state.get_inner_mut(),
          )
        }
        Ok(())
      }

      #[napi]
      pub fn bind_buffer(&self, target: u32, buffer: &WebGLBuffer) {
        canvas_c::canvas_native_webgl_bind_buffer(target, buffer.0, self.state);
      }

      #[napi]
      pub fn bind_framebuffer(&self, target: u32, framebuffer: Option<&web_g_l_framebuffer>) {
       canvas_c::canvas_native_webgl_bind_frame_buffer(target, framebuffer.map(|framebuffer| framebuffer.buffer).unwrap_or(0) , self.state);
      }

      #[napi]
      pub fn bind_renderbuffer(&self, target: u32, renderbuffer: Option<&WebGLRenderbuffer>) {
        canvas_c::canvas_native_webgl_bind_render_buffer(target, renderbuffer.map(|renderbuffer|renderbuffer.0).unwrap_or(0), self.state);
      }

      #[napi]
      pub fn bind_texture(&self, target: u32, texture: Option<&WebGLTexture>) {
        canvas_c::canvas_native_webgl_bind_texture(target, texture.map(|texture| texture.0).unwrap_or(0), self.state);
      }

      #[napi]
      pub fn blend_color(&self, red: f64, green: f64, blue: f64, alpha: f64) {
        canvas_c::canvas_native_webgl_blend_color(
          red as f32,
          green as f32,
          blue as f32,
          alpha as f32,
          self.state,
        );
      }

      #[napi]
      pub fn blend_equation_separate(&self, mode_r_g_b: u32, mode_alpha: u32) {
        canvas_c::canvas_native_webgl_blend_equation_separate(mode_r_g_b, mode_alpha, self.state)
      }

      #[napi]
      pub fn blend_equation(&self, mode: u32) {
        canvas_c::canvas_native_webgl_blend_equation(mode, self.state)
      }

      #[napi]
      pub fn blend_func_separate(
        &self,
        src_r_g_b: Option<u32>,
        dst_r_g_b: Option<u32>,
        src_alpha: Option<u32>,
        dst_alpha: Option<u32>,
      ) {
        let src_r_g_b = src_r_g_b.unwrap_or(1);
        let dst_r_g_b = dst_r_g_b.unwrap_or(0);
        let src_alpha = src_alpha.unwrap_or(1);
        let dst_alpha = dst_alpha.unwrap_or(0);
        canvas_c::canvas_native_webgl_blend_func_separate(
          src_r_g_b, dst_r_g_b, src_alpha, dst_alpha, self.state,
        );
      }

      #[napi]
      pub fn blend_func(&self, sfactor: Option<u32>, dfactor: Option<u32>) {
        let sfactor = sfactor.unwrap_or(1);
        let dfactor = dfactor.unwrap_or(0);
        canvas_c::canvas_native_webgl_blend_func(sfactor, dfactor, self.state);
      }

      #[napi]
      pub fn buffer_data(
        &self,
        target: u32,
        size_or_src_data: Option<Either3<i64, Buffer, Float32Array>>,
        usage: Option<u32>,
      ) {
        match size_or_src_data {
          Some(size_or_src_data) => match size_or_src_data {
            Either3::A(size) => match usage {
              Some(usage) => {
                canvas_c::canvas_native_webgl_buffer_data_none(
                  target,
                  size as isize,
                  usage,
                  self.state,
                );
              }
              None => {
                canvas_c::canvas_native_webgl_buffer_data_none(target, 0, size as u32, self.state);
              }
            },
            Either3::B(src_data) => {
              if let Some(usage) = usage {
                canvas_c::canvas_native_webgl_buffer_data(
                  target,
                  src_data.as_ptr(),
                  src_data.len(),
                  usage,
                  self.state,
                );
              }
            }
            Either3::C(src_data) => {
                if let Some(usage) = usage {
                  canvas_c::canvas_native_webgl_buffer_data_f32(
                    target,
                    src_data.as_ptr(),
                    src_data.len(),
                    usage,
                    self.state,
                  );
                }
              }
          },
          _ => {
            if let Some(usage) = usage {
              canvas_c::canvas_native_webgl_buffer_data_none(target, 0, usage, self.state);
            }
          }
        }
      }

      #[napi]
      pub fn buffer_sub_data(&self, target: u32, offset: i64, src_data: &[u8]) {
        canvas_c::canvas_native_webgl_buffer_sub_data(
          target,
          offset as isize,
          src_data.as_ptr(),
          src_data.len(),
          self.state,
        )
      }


       #[napi]
    pub fn check_framebuffer_status(&self, target: u32) -> u32 {
        canvas_c::canvas_native_webgl_check_frame_buffer_status(target, self.state)
    }

    #[napi]
    pub fn clear_color(&self,
                       red: f64,
                       green: f64,
                       blue: f64,
                       alpha: f64) {
        canvas_c::canvas_native_webgl_clear_color(red as f32, green as f32, blue as f32, alpha as f32, self.state);
    }

    #[napi]
    pub fn clear_depth(&self, depth: f64) {
        canvas_c::canvas_native_webgl_clear_depth(depth as f32, self.state);
    }

    #[napi]
    pub fn clear_stencil(&self, stencil: i32) {
        canvas_c::canvas_native_webgl_clear_stencil(stencil, self.state);
    }

    #[napi]
    pub fn clear(&mut self, mask: u32) {
        canvas_c::canvas_native_webgl_clear(mask, self.state);
              self.update_invalidate_state();
    }

    #[napi]
    pub fn color_mask(&self, red: bool, green: bool, blue: bool, alpha: bool) {
        canvas_c::canvas_native_webgl_color_mask(red, green, blue, alpha, self.state);
    }


    #[napi]
    pub fn commit(&self) {
        canvas_c::canvas_native_webgl_commit(self.state);
    }

    #[napi]
    pub fn compile_shader(&self, shader: ClassInstance<WebGLShader>) {
        canvas_c::canvas_native_webgl_compile_shader(shader.0, self.state);
    }

    #[napi]
    pub fn compressed_tex_image_2_d(&self, target: u32, level: i32, internalformat: u32, width: i64, height: i64, border: i32, pixels: &[u8]) {
        canvas_c::canvas_native_webgl_compressed_tex_image2d(
            target, level, internalformat, width as i32, height as i32, border, pixels.as_ptr(), pixels.len(), self.state,
        )
    }

    #[napi]
    pub fn compressed_tex_sub_image_2_d(&self, target: u32, level: i32, xoffset: i64, yoffset: i64, width: f64, height: f64, format: u32, pixels: &[u8]) {
        canvas_c::canvas_native_webgl_compressed_tex_sub_image2d(
            target, level, xoffset as i32, yoffset as i32, width as i32, height as i32, format, pixels.as_ptr(), pixels.len(), self.state,
        )
    }

    #[napi]
    pub fn copy_tex_image_2_d(&self, target: u32, level: i32, internalformat: u32, x: i64, y: i64, width: i64, height: i64, border: i32) {
        canvas_c::canvas_native_webgl_copy_tex_image2d(
            target, level, internalformat, x as i32, y as i32, width as i32, height as i32, border, self.state,
        )
    }

    #[napi]
    pub fn copy_tex_sub_image_2_d(&self, target: u32, level: i32, xoffset: i64, yoffset: i64, x: i64, y: i64, width: i64, height: i64) {
        canvas_c::canvas_native_webgl_copy_tex_sub_image2d(
            target, level, xoffset as i32, yoffset as i32, x as i32, y as i32, width as i32, height as i32, self.state,
        )
    }

    #[napi]
    pub fn create_buffer(&self) -> WebGLBuffer {
        WebGLBuffer(
            canvas_c::canvas_native_webgl_create_buffer(self.state)
        )
    }

    #[napi]
    pub fn create_framebuffer(&self) -> web_g_l_framebuffer {
        web_g_l_framebuffer{
            buffer: canvas_c::canvas_native_webgl_create_framebuffer(self.state)
        }
    }

    #[napi]
    pub fn create_program(&self) -> WebGLProgram {
        WebGLProgram(
            canvas_c::canvas_native_webgl_create_program(self.state)
        )
    }
    #[napi]

    pub fn create_renderbuffer(&self) -> WebGLRenderbuffer {
        WebGLRenderbuffer(
            canvas_c::canvas_native_webgl_create_renderbuffer(self.state)
        )
    }

    #[napi(ts_args_type = "type: number")]
    pub fn create_shader(&self, type_: u32) -> WebGLShader {
        WebGLShader(
            canvas_c::canvas_native_webgl_create_shader(type_, self.state),
        )
    }

    #[napi]
    pub fn create_texture(&self, env: Env) -> WebGLTexture {
        WebGLTexture(
            canvas_c::canvas_native_webgl_create_texture(self.state)
        )
    }

    #[napi]
    pub fn cull_face(&self, mode: u32) {
        canvas_c::canvas_native_webgl_cull_face(mode, self.state)
    }


    #[napi]
    pub fn delete_buffer(&self, buffer: &WebGLBuffer) {
        canvas_c::canvas_native_webgl_delete_buffer(buffer.0, self.state)
    }

    #[napi]
    pub fn delete_framebuffer(&self, frame_buffer: &web_g_l_framebuffer) {
        canvas_c::canvas_native_webgl_delete_framebuffer(frame_buffer.buffer, self.state)
    }

    #[napi]
    pub fn delete_program(&self, program: &WebGLProgram) {
        canvas_c::canvas_native_webgl_delete_program(program.0, self.state)
    }

    #[napi]
    pub fn delete_renderbuffer(&self, render_buffer: &WebGLRenderbuffer) {
        canvas_c::canvas_native_webgl_delete_renderbuffer(render_buffer.0, self.state)
    }

    #[napi]
    pub fn delete_shader(&self, shader: &WebGLRenderbuffer) {
        canvas_c::canvas_native_webgl_delete_shader(shader.0, self.state)
    }

    #[napi]
    pub fn delete_texture(&self, texture: &WebGLTexture) {
        canvas_c::canvas_native_webgl_delete_texture(texture.0, self.state)
    }

    #[napi]
    pub fn depth_func(&self, func: u32) {
        canvas_c::canvas_native_webgl_depth_func(func, self.state);
    }

    #[napi]
    pub fn depth_mask(&self, flag: bool) {
        canvas_c::canvas_native_webgl_depth_mask(flag, self.state);
    }

    #[napi]
    pub fn depth_range(&self, z_near: f64, z_far: f64) {
        canvas_c::canvas_native_webgl_depth_range(z_near as f32, z_far as f32, self.state)
    }

    #[napi]
    pub fn detach_shader(&self, program: ClassInstance<WebGLProgram>, shader: ClassInstance<WebGLShader>) {
        canvas_c::canvas_native_webgl_detach_shader(program.0, shader.0, self.state)
    }

    #[napi]
    pub fn disable_vertex_attrib_array(&self, index: u32) {
        canvas_c::canvas_native_webgl_disable_vertex_attrib_array(index, self.state);
    }

    #[napi]
    pub fn disable(&self, cap: u32) {
        canvas_c::canvas_native_webgl_disable(cap, self.state);
    }

    #[napi]
    pub fn draw_arrays(&mut self, mode: u32, first: i32, count: i32) {
        canvas_c::canvas_native_webgl_draw_arrays(mode, first, count, self.state);
               self.update_invalidate_state();
    }

    #[napi(ts_args_type = "mode: number, count: number, type: number, offset: number")]
    pub fn draw_elements(&mut self, mode: u32, count: i32, type_: u32, offset: i64) {
        canvas_c::canvas_native_webgl_draw_elements(mode, count, type_, offset as isize, self.state);
                 self.update_invalidate_state();
    }

    #[napi]
    pub fn enable_vertex_attrib_array(&self, index: u32) {
        canvas_c::canvas_native_webgl_enable_vertex_attrib_array(index, self.state);
    }

    #[napi]
    pub fn enable(&self, cap: u32) {
        canvas_c::canvas_native_webgl_enable(cap, self.state);
    }

    #[napi]
    pub fn finish(&self) {
        canvas_c::canvas_native_webgl_finish(self.state);
    }

    #[napi]
    pub fn flush(&self) {
        canvas_c::canvas_native_webgl_flush(self.state);
    }

    #[napi]
    pub fn framebuffer_renderbuffer(&self, target: u32, attachment: u32, renderbuffertarget: u32, renderbuffer: ClassInstance<WebGLRenderbuffer>) {
        canvas_c::canvas_native_webgl_framebuffer_renderbuffer(target, attachment, renderbuffertarget, renderbuffer.0, self.state)
    }

    #[napi]
    pub fn framebuffer_texture_2_d(&self, target: u32, attachment: u32, textarget: u32, texture: ClassInstance<WebGLTexture>, level: i32) {
        canvas_c::canvas_native_webgl_framebuffer_texture2d(
            target, attachment, textarget, texture.0, level, self.state,
        )
    }

    #[napi]
    pub fn front_face(&self, mode: u32) {
        canvas_c::canvas_native_webgl_front_face(mode, self.state)
    }

    #[napi]
    pub fn generate_mipmap(&self, target: u32) {
        canvas_c::canvas_native_webgl_generate_mipmap(target, self.state);
    }


    #[napi]
    pub fn get_active_attrib(&self, env: Env, program: ClassInstance<WebGLProgram>, index: u32) -> Result<ClassInstance<web_g_l_active_info>> {
        web_g_l_active_info(
            canvas_c::canvas_native_webgl_get_active_attrib(
                program.0, index, self.state,
            )
        ).into_instance(env)
    }

    #[napi]
    pub fn get_active_uniform(&self, env: Env, program: ClassInstance<WebGLProgram>, index: u32) -> Result<ClassInstance<web_g_l_active_info>> {
        web_g_l_active_info(
            canvas_c::canvas_native_webgl_get_active_uniform(
                program.0, index, self.state,
            )
        ).into_instance(env)
    }

    #[napi]
    pub fn get_attached_shaders(&self, env: Env, program: ClassInstance<WebGLProgram>) -> Vec<ClassInstance<WebGLShader>> {
        let state = unsafe { &mut *self.state };
        let shaders = canvas_webgl::webgl::canvas_native_webgl_get_attached_shaders(program.0, state.get_inner_mut());
        shaders.into_iter()
            .map(|shader| {
                WebGLShader(shader).into_instance(env).unwrap()
            }).collect::<Vec<_>>()
    }

    #[napi]
    pub fn get_attrib_location(&self, program: ClassInstance<WebGLProgram>, name: JsString) -> Result<i32> {
        let name = name.into_utf8()?;
        let name = name.as_str()?;
        let state = unsafe { &mut *self.state };
        Ok(canvas_webgl::webgl::canvas_native_webgl_get_attrib_location(program.0, name, state.get_inner_mut()))
    }

    #[napi]
    pub fn get_buffer_parameter(&self, target: u32, pname: u32) -> i32 {
        canvas_c::canvas_native_webgl_get_buffer_parameter(target, pname, self.state)
    }

    #[napi]
    pub fn get_context_attributes(&self) -> context_attributes::ContextAttributes {
        let attributes = canvas_c::canvas_native_webgl_get_context_attributes(self.state);
        let ret = context_attributes::ContextAttributes::from_c(attributes);
        canvas_c::canvas_native_context_attributes_destroy(attributes);
        ret
    }

    #[napi]
    pub fn get_error(&self) -> u32 {
        canvas_c::canvas_native_webgl_get_error(self.state)
    }

    #[napi]
    pub fn get_extension(&self, env: Env, name: JsString) -> Result<JsUnknown> {
        let name = name.into_utf8()?;
        let name = name.as_str()?;

        if name == "EXT_disjoint_timer_query_webgl2" {
            return env.get_null().map(|null| null.into_unknown());
        }
        let state = unsafe { &mut *self.state };

        let ext = canvas_webgl::webgl::canvas_native_webgl_get_extension(
            name.as_ref(),
            state.get_inner_mut(),
        );


        match ext {
            None => {
                env.get_null().map(|null| null.into_unknown())
            }
            Some(ext) => {
                let ext = Box::into_raw(Box::new(WebGLExtension::new(Some(ext))));

                match name {
                    "ANGLE_instanced_arrays" => {
                        let ret = canvas_c::canvas_native_webgl_context_extension_to_angle_instanced_arrays(ext);
                        ANGLE_instanced_arrays(ret)
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "EXT_blend_minmax" => {
                        if ext.is_null() {
                            return env.get_null().map(|null| null.into_unknown());
                        }
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        EXT_blend_minmax
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "EXT_color_buffer_half_float" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        EXT_color_buffer_half_float
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "EXT_disjoint_timer_query" => {
                        let ext = canvas_c::canvas_native_webgl_context_extension_to_ext_disjoint_timer_query(ext);
                        EXT_disjoint_timer_query(ext)
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "EXT_sRGB" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        EXT_sRGB
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "EXT_shader_texture_lod" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        EXT_shader_texture_lod
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "EXT_texture_filter_anisotropic" => {
                        EXT_texture_filter_anisotropic
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "OES_element_index_uint" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        OES_element_index_uint
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "OES_standard_derivatives" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        OES_standard_derivatives
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "OES_texture_float" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        OES_texture_float
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "OES_texture_float_linear" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        OES_texture_float_linear
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "OES_texture_half_floatr" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        OES_texture_half_float
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "OES_texture_half_float_linear" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        OES_texture_half_float_linear
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "OES_vertex_array_object" => {
                        let ext = canvas_c::canvas_native_webgl_context_extension_to_oes_vertex_array_object(ext);
                        OES_vertex_array_object(ext)
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_color_buffer_float" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        WEBGL_color_buffer_float
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_compressed_texture_atc" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        WEBGL_compressed_texture_atc
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_compressed_texture_etc" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        WEBGL_compressed_texture_etc
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_compressed_texture_etc1" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        WEBGL_compressed_texture_etc1
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_compressed_texture_pvrtc" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        WEBGL_compressed_texture_pvrtc
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_compressed_texture_s3tc" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        WEBGL_compressed_texture_s3tc
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_lose_context" => {
                        let ext = canvas_c::canvas_native_webgl_context_extension_to_lose_context(ext);
                        WEBGL_lose_context(ext)
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_depth_texture" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);
                        WEBGL_depth_texture
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "WEBGL_draw_buffers" => {
                        let ext = canvas_c::canvas_native_webgl_context_extension_to_draw_buffers(ext);
                        WEBGL_draw_buffers(ext)
                            .into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    "OES_fbo_render_mipmap" => {
                        canvas_c::canvas_native_webgl_extension_destroy(ext);

                        OES_fbo_render_mipmap.into_instance(env).map(|ext| ext.as_object(env).into_unknown())
                    }
                    _ => env.get_null().map(|null| null.into_unknown()),
                }
            }
        }
    }

    #[napi]
    pub fn get_framebuffer_attachment_parameter(&self, env: Env, target: u32, attachment: u32, pname: u32) -> Result<Unknown> {
        let mut state = unsafe { &mut *self.state };
        let ret = canvas_webgl::webgl::canvas_native_webgl_get_framebuffer_attachment_parameter(
            target,
            attachment,
            pname,
            state.get_inner_mut(),
        );


        if ret.get_is_renderbuffer() {
            return WebGLRenderbuffer(ret.get_value() as _)
                .into_instance(env).map(|v| v.as_object(env).into_unknown());
        } else if ret.get_is_texture() {
            return WebGLTexture(ret.get_value() as _)
                .into_instance(env).map(|v| v.as_object(env).into_unknown())
        }

        env.create_int32(ret.get_value() as _).map(|v| v.into_unknown())
    }


    #[napi]
    pub fn get_program_info_log(&self, program: ClassInstance<WebGLProgram>) -> String {
        let state = unsafe { &mut *self.state };
        canvas_webgl::webgl::canvas_native_webgl_get_program_info_log(program.0, state.get_inner_mut())
    }

    #[napi]
    pub fn get_program_parameter(&self, env: Env, program: ClassInstance<WebGLProgram>, pname: u32) -> Result<Unknown> {
        let result = canvas_c::canvas_native_webgl_get_program_parameter(program.0, pname, self.state);
        if canvas_c::canvas_native_webgl_result_get_is_none(result) {
            // todo
            canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
            return env.get_null().map(|v| v.into_unknown());
        }


        match pname {
            gl_bindings::DELETE_STATUS |
            gl_bindings::LINK_STATUS |
            gl_bindings::VALIDATE_STATUS => {
                let ret = canvas_c::canvas_native_webgl_result_get_bool(result);
                canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
                env.get_boolean(ret).map(|v| v.into_unknown())
            }
            _ => {
                let ret = canvas_c::canvas_native_webgl_result_get_i32(result);
                canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
                env.create_int32(ret).map(|v| v.into_unknown())
            }
        }
    }


    #[napi]
    pub fn get_renderbuffer_parameter(&self, target: u32, pname: u32) -> i32 {
        canvas_c::canvas_native_webgl_get_renderbuffer_parameter(target, pname, self.state)
    }

    #[napi]
    pub fn get_shader_info_log(&self, shader: ClassInstance<WebGLShader>) -> String {
        let state = unsafe { &mut *self.state };
        canvas_webgl::webgl::canvas_native_webgl_get_shader_info_log(shader.0, state.get_inner_mut())
    }

    #[napi]
    pub fn get_shader_parameter(&self, env: Env, shader: ClassInstance<WebGLShader>, pname: u32) -> Result<Unknown> {
        let result = canvas_c::canvas_native_webgl_get_shader_parameter(shader.0, pname, self.state);
        match pname {
            gl_bindings::DELETE_STATUS | gl_bindings::COMPILE_STATUS => {
                let ret = canvas_c::canvas_native_webgl_result_get_bool(result);
                canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
                env.get_boolean(ret).map(|v| v.into_unknown())
            }
            _ => {
                let ret = canvas_c::canvas_native_webgl_result_get_i32(result);
                canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
                env.create_int32(ret).map(|v| v.into_unknown())
            }
        }
    }

    #[napi]
    pub fn get_shader_precision_format(&self, env: Env, shader_type: u32, precision_type: u32) -> Result<ClassInstance<web_g_l_shader_precision_format>> {
        let precision = canvas_c::canvas_native_webgl_get_shader_precision_format(shader_type, precision_type, self.state);
        web_g_l_shader_precision_format(precision)
            .into_instance(env)
    }

    #[napi]
    pub fn get_shader_source(&self, shader: ClassInstance<WebGLShader>) -> String {
        let state = unsafe { &mut *self.state };
        canvas_webgl::webgl::canvas_native_webgl_get_shader_source(shader.0, state.get_inner_mut())
    }

    #[napi]
    pub fn get_supported_extensions(&self) -> Vec<String> {
        let state = unsafe { &mut *self.state };
        canvas_webgl::webgl::canvas_native_webgl_get_supported_extensions(state.get_inner_mut())
    }

    #[napi]
    pub fn get_tex_parameter(&self, target: u32, pname: u32) -> i32 {
        canvas_c::canvas_native_webgl_get_tex_parameter(target, pname, self.state)
    }

    #[napi]
    pub fn get_uniform_location(&self, env: Env, program: ClassInstance<WebGLProgram>, name: String) -> Result<Unknown> {
        let state = unsafe { &mut *self.state };
        let location = canvas_webgl::webgl::canvas_native_webgl_get_uniform_location(program.0, name.as_str(), state.get_inner_mut());

        if location == -1 {
            return env.get_null().map(|v| v.into_unknown());
        }

        webgl_uniform_location::WebGLUniformLocation(location)
            .into_instance(env)
            .map(|v| v.as_object(env).into_unknown())
    }

    #[napi]
    pub fn get_uniform(&self, env: Env, program: ClassInstance<WebGLProgram>, location: ClassInstance<WebGLUniformLocation>) -> Result<Unknown> {
        let result = canvas_c::canvas_native_webgl_get_uniform(program.0, location.0, self.state);
        let type_ = canvas_c::canvas_native_webgl_result_get_type(result);
        let uniform = match type_ {
            WebGLResultType::Boolean => {
                let ret = canvas_c::canvas_native_webgl_result_get_bool(result);
                env.get_boolean(ret).map(|v| v.into_unknown())
            }
            WebGLResultType::I32Array => unsafe {
                let ret = canvas_c::canvas_native_webgl_result_into_i32_array(result);


                if ret.is_null() {
                    return env.get_null().map(|v| v.into_unknown());
                }

                let ret = *Box::from_raw(ret);
                let mut ret = ret.into_vec();

                let ptr = ret.as_mut_ptr();
                let len = ret.len();

                let buffer = env.create_arraybuffer_with_borrowed_data(ptr as _, len * size_of::<i32>(), ret, |_, _| {})?;
                buffer.value.into_typedarray(
                    TypedArrayType::Int32,
                    len,
                    0,
                ).map(|v| v.into_unknown())
            }
            WebGLResultType::U32Array => unsafe {
                let ret = canvas_c::canvas_native_webgl_result_into_u32_array(result);


                if ret.is_null() {
                    return env.get_null().map(|v| v.into_unknown());
                }

                let ret = *Box::from_raw(ret);
                let mut ret = ret.into_vec();

                let ptr = ret.as_mut_ptr();
                let len = ret.len();

                let buffer = env.create_arraybuffer_with_borrowed_data(ptr as _, len * size_of::<u32>(), ret, |_, _| {})?;
                buffer.value.into_typedarray(
                    TypedArrayType::Uint32,
                    len,
                    0,
                ).map(|v| v.into_unknown())
            }
            WebGLResultType::F32Array => unsafe {
                let ret = canvas_c::canvas_native_webgl_result_into_f32_array(result);


                if ret.is_null() {
                    return env.get_null().map(|v| v.into_unknown());
                }

                let ret = *Box::from_raw(ret);
                let mut ret = ret.into_vec();

                let ptr = ret.as_mut_ptr();
                let len = ret.len();

                let buffer = env.create_arraybuffer_with_borrowed_data(ptr as _, len * size_of::<f32>(), ret, |_, _| {})?;
                buffer.value.into_typedarray(
                    TypedArrayType::Float32,
                    len,
                    0,
                ).map(|v| v.into_unknown())
            }
            WebGLResultType::BooleanArray => {
                let ret = canvas_c::canvas_native_webgl_result_get_bool_array(result);
                let len = canvas_c::canvas_native_u8_buffer_get_length(ret);
                let buf = canvas_c::canvas_native_u8_buffer_get_bytes(ret);
                let buf = unsafe { std::slice::from_raw_parts(buf, len) };
                let mut array = env.create_array(len as u32)?;

                for i in 0..len {
                    array.set(i as u32, buf[i] == 1)?;
                }
                array.coerce_to_object().map(|v| v.into_unknown())
            }
            WebGLResultType::U32 => {
                let ret = canvas_c::canvas_native_webgl_result_get_u32(result);
                env.create_uint32(ret).map(|v| v.into_unknown())
            }
            WebGLResultType::I32 => {
                let ret = canvas_c::canvas_native_webgl_result_get_i32(result);
                env.create_int32(ret).map(|v| v.into_unknown())
            }
            WebGLResultType::F32 => {
                let ret = canvas_c::canvas_native_webgl_result_get_f32(result);
                env.create_double(ret as f64).map(|v| v.into_unknown())
            }
            WebGLResultType::String => unsafe {
                let ret = canvas_c::canvas_native_webgl_result_get_string(result);
                if ret.is_null() {
                    return Err(Error::from_status(Status::GenericFailure));
                }
                let ret = CString::from_raw(ret as _);
                match ret.into_string() {
                    Ok(s) => env.create_string_from_std(s),
                    Err(_) => Err(Error::from_status(Status::GenericFailure)),
                }.map(|v| v.into_unknown())
            }
            WebGLResultType::None => {
                env.get_null().map(|v| v.into_unknown())
            }
        };

        canvas_c::canvas_native_webgl_WebGLResult_destroy(result);

        uniform
    }

    #[napi]
    pub fn get_vertex_attrib_offset(&self, index: u32, pname: u32) -> i64 {
        canvas_c::canvas_native_webgl_get_vertex_attrib_offset(index, pname, self.state) as i64
    }

    #[napi]
    pub fn get_vertex_attrib(&self, env: Env, index: u32, pname: u32) -> Result<Unknown> {
        let result = canvas_c::canvas_native_webgl_get_vertex_attrib(index, pname, self.state);

        match pname {
            gl_bindings::CURRENT_VERTEX_ATTRIB => unsafe {
                let ret = canvas_c::canvas_native_webgl_result_get_f32_array(result);
                if ret.is_null() {
                    return env.get_null().map(|v| v.into_unknown());
                }

                let ret = *Box::from_raw(ret);
                let mut ret = ret.into_vec();

                let ptr = ret.as_mut_ptr();
                let len = ret.len();

                let buffer = env.create_arraybuffer_with_borrowed_data(ptr as _, len * size_of::<f32>(), ret, |_, _| {})?;
                buffer.value.into_typedarray(
                    TypedArrayType::Float32,
                    len,
                    0,
                ).map(|v| v.into_unknown())
            }
            gl_bindings::VERTEX_ATTRIB_ARRAY_ENABLED | gl_bindings::VERTEX_ATTRIB_ARRAY_NORMALIZED => {
                let ret = canvas_c::canvas_native_webgl_result_get_bool(result);
                env.get_boolean(ret).map(|v| v.into_unknown())
            }
            _ => {
                let ret = canvas_c::canvas_native_webgl_result_get_i32(result);
                env.create_int32(ret).map(|v| v.into_unknown())
            }
        }
    }

    #[napi]
    pub fn hint(&self, target: u32, mode: u32) {
        canvas_c::canvas_native_webgl_hint(target, mode, self.state);
    }


    #[napi]
    pub fn is_buffer(&self, buffer: ClassInstance<WebGLBuffer>) -> bool {
        canvas_c::canvas_native_webgl_is_buffer(buffer.0, self.state)
    }

    #[napi]
    pub fn is_context_lost(&self) -> bool {
        canvas_c::canvas_native_webgl_get_is_context_lost(self.state)
    }

    #[napi]
    pub fn is_enabled(&self, cap: u32) -> bool {
        canvas_c::canvas_native_webgl_is_enabled(cap, self.state)
    }

    #[napi]
    pub fn is_framebuffer(&self, framebuffer: Option<&web_g_l_framebuffer>) -> bool {
        let framebuffer = match framebuffer {
            None => 0,
            Some(framebuffer) => framebuffer.buffer
        };
        canvas_c::canvas_native_webgl_is_framebuffer(framebuffer, self.state)
    }

    #[napi]
    pub fn is_program(&self, program: ClassInstance<WebGLProgram>) -> bool {
        canvas_c::canvas_native_webgl_is_program(program.0, self.state)
    }

    #[napi]
    pub fn is_renderbuffer(&self, renderbuffer: ClassInstance<WebGLRenderbuffer>) -> bool {
        canvas_c::canvas_native_webgl_is_renderbuffer(renderbuffer.0, self.state)
    }

    #[napi]
    pub fn is_shader(&self, shader: ClassInstance<WebGLShader>) -> bool {
        canvas_c::canvas_native_webgl_is_shader(shader.0, self.state)
    }

    #[napi]
    pub fn is_texture(&self, texture: ClassInstance<WebGLTexture>) -> bool {
        canvas_c::canvas_native_webgl_is_texture(texture.0, self.state)
    }

    #[napi]
    pub fn line_width(&self, width: f64) {
        canvas_c::canvas_native_webgl_line_width(width as f32, self.state);
    }

    #[napi]
    pub fn link_program(&self, program: ClassInstance<WebGLProgram>) {
        canvas_c::canvas_native_webgl_link_program(program.0, self.state)
    }

    #[napi]
    pub fn pixel_storei(&self, pname: u32, param: Either<bool, i32>) {
        match param {
            Either::A(param) => {
                let param = if param { 1 } else { 0 };
                match pname {
                    UNPACK_FLIP_Y_WEBGL | UNPACK_PREMULTIPLY_ALPHA_WEBGL => {
                        canvas_c::canvas_native_webgl_pixel_storei(pname, param, self.state)
                    }
                    _ => {}
                }
            }
            Either::B(param) => {
                match pname {
                    UNPACK_COLORSPACE_CONVERSION_WEBGL => {
                        canvas_c::canvas_native_webgl_pixel_storei(pname, 0x9244, self.state)
                    }
                    _ => {
                        canvas_c::canvas_native_webgl_pixel_storei(pname, param, self.state)
                    }
                }
            }
        }
    }


    #[napi]
    pub fn polygon_offset(&self, factor: f64, units: f64) {
        canvas_c::canvas_native_webgl_polygon_offset(factor as f32, units as f32, self.state)
    }

    #[napi(
        ts_args_type = "x: number, y: number, width: number, height: number, format: number, type: number, pixels: ArrayBuffer | ArrayBufferView"
    )]
    pub fn read_pixels(&self, x: i32, y: i32, width: i32, height: i32, format: u32, type_: u32, mut pixels: Buffer) {
        // todo
        canvas_c::canvas_native_webgl_read_pixels_u8(
            x, y, width, height, format, type_, pixels.as_mut_ptr(), pixels.len(), self.state,
        )
    }

    #[napi]
    pub fn renderbuffer_storage(&self, target: u32, internal_format: u32, width: i32, height: i32) {
        let mut internal_format = internal_format;
        if (internal_format == gl_bindings::DEPTH_STENCIL) {
            // DEPTH24_STENCIL8 = 35056
            // DEPTH24_STENCIL8_OES = 0x88F0
            internal_format = 0x88f0;
        }
        canvas_c::canvas_native_webgl_renderbuffer_storage(target, internal_format, width, height, self.state)
    }

    #[napi]
    pub fn sample_coverage(&self, value: f64, invert: bool) {
        canvas_c::canvas_native_webgl_sample_coverage(value as f32, invert, self.state);
    }

    #[napi]
    pub fn scissor(&self, x: i32, y: i32, width: i32, height: i32) {
        canvas_c::canvas_native_webgl_scissor(x, y, width, height, self.state)
    }

    #[napi]
    pub fn shader_source(&self, shader: ClassInstance<WebGLShader>, source: String) {
        let state = unsafe { &mut *self.state };
        let mut source = source;
        if(source.contains("#version 300 es")){
              source =  source.replace("#version 300 es", "#version 330 core");
        }
        canvas_webgl::webgl::canvas_native_webgl_shader_source(shader.0, source.as_str(), state.get_inner_mut());
    }

    #[napi(ts_args_type = "face:number, func: number, ref: number, mask: number")]
    pub fn stencil_func_separate(&self, face: u32, func: u32, ref_: i32, mask: u32) {
        canvas_c::canvas_native_webgl_stencil_func_separate(face, func, ref_, mask, self.state)
    }

    #[napi(ts_args_type = "func: number, ref: number, mask: number")]
    pub fn stencil_func(&self, func: u32, ref_: i32, mask: u32) {
        canvas_c::canvas_native_webgl_stencil_func(func, ref_, mask, self.state)
    }

    #[napi]
    pub fn stencil_mask_separate(&self, face: u32, mask: u32) {
        canvas_c::canvas_native_webgl_stencil_mask_separate(face, mask, self.state);
    }

    #[napi]
    pub fn stencil_mask(&self, mask: u32) {
        canvas_c::canvas_native_webgl_stencil_mask(mask, self.state);
    }

    #[napi]
    pub fn stencil_op_separate(&self, face: u32, fail: u32, zfail: u32, zpass: u32) {
        canvas_c::canvas_native_webgl_stencil_op_separate(face, fail, zfail, zpass, self.state);
    }

    #[napi]
    pub fn stencil_op(&self, fail: u32, zfail: u32, zpass: u32) {
        canvas_c::canvas_native_webgl_stencil_op(fail, zfail, zpass, self.state);
    }

    #[napi]
    pub fn tex_image_2_d(&self, target: i32, level: i32, internalformat: i32, width_or_format: i32, height_or_type: i32, border_or_pixels: Either4<i32, ClassInstance<crate::c2d::CanvasRenderingContext2D>, ClassInstance<web_g_l_rendering_context>, ClassInstance<crate::image_asset::ImageAsset>>, format: Option<i32>, type_: Option<i32>, pixels: Option<Either<Buffer, i64>>, offset: Option<i64>) -> Result<()> {
        match border_or_pixels {
            Either4::A(border) => {
                match (format, type_, pixels) {
                    (Some(format), Some(type_), Some(pixels)) => {
                        match pixels {
                            Either::A(buffer) => {
                                canvas_c::canvas_native_webgl_tex_image2d(
                                    target, level, internalformat, width_or_format, height_or_type, border, format, type_, buffer.as_ptr(), buffer.len(), self.state,
                                )
                            }
                            Either::B(offset) => {
                                canvas_c::canvas_native_webgl2_tex_image2d_offset(
                                    target, level, internalformat, width_or_format as u32, height_or_type as u32, border, format, type_, offset as u64, self.state,
                                )
                            }
                        }
                    }
                    (Some(format), Some(type_), None) => {
                        canvas_c::canvas_native_webgl_tex_image2d_none(
                            target, level, internalformat, width_or_format, height_or_type, border, format, type_, self.state,
                        )
                    }
                    _ => {}
                }
            }
            Either4::B(c2d) => {
                canvas_c::canvas_native_webgl_tex_image2d_canvas2d(
                    target, level, internalformat, width_or_format, height_or_type, c2d.context, self.state,
                )
            }
            Either4::C(gl) => {
                canvas_c::canvas_native_webgl_tex_image2d_webgl(
                    target, level, internalformat, width_or_format, height_or_type, gl.state, self.state,
                )
            }
            Either4::D(image) => {
                canvas_c::canvas_native_webgl_tex_image2d_image_asset(
                    target, level, internalformat, width_or_format, height_or_type, Arc::as_ptr(&image.asset), self.state,
                )
            }
        }
        Ok(())
    }


    #[napi]
    pub fn tex_parameterf(&self, target: u32, pname: u32, param: f64) {
        canvas_c::canvas_native_webgl_tex_parameterf(target, pname, param as f32, self.state);
    }

    #[napi]
    pub fn tex_parameteri(&self, target: u32, pname: u32, param: i32) {
        canvas_c::canvas_native_webgl_tex_parameteri(target, pname, param, self.state);
    }

    #[napi]
    pub fn tex_sub_image_2_d(
    &self,
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    format_or_width: i32,
    type_or_height: i32,
    pixels_or_format: Either7<
      u32,
      &crate::image_bitmap::ImageBitmap,
      &crate::image_asset::ImageAsset,
      &crate::c2d::CanvasRenderingContext2D,
      &crate::gl::web_g_l_rendering_context,
      &crate::gl2::web_g_l_2_rendering_context,
      &crate::c2d::image_data::ImageData,
    >,
    type_: Option<i32>,
    pixels: Option<Either5<&[u8], &[u16], &[f32], JsArrayBuffer, i64>>,
    offset: Option<i64>,
  ) -> Result<()> {
    match pixels_or_format {
      Either7::A(format) => match (type_, pixels) {
        (Some(type_), Some(pixels)) => match pixels {
          Either5::A(buf) => {
            canvas_c::canvas_native_webgl_tex_sub_image2d(
              target,
              level,
              xoffset,
              yoffset,
              format_or_width,
              type_or_height,
              format,
              type_,
              buf.as_ptr(),
              buf.len(),
              self.state,
            );
          }
          Either5::B(short) => {
            canvas_c::canvas_native_webgl_tex_sub_image2d(
              target,
              level,
              xoffset,
              yoffset,
              format_or_width,
              type_or_height,
              format,
              type_,
              short.as_ptr() as *const u8,
              short.len() * size_of::<u16>(),
              self.state,
            );
          }
          Either5::C(float) => {
            canvas_c::canvas_native_webgl_tex_sub_image2d(
              target,
              level,
              xoffset,
              yoffset,
              format_or_width,
              type_or_height,
              format,
              type_,
              float.as_ptr() as *const u8,
              float.len() * size_of::<f32>(),
              self.state,
            );
          }
          Either5::D(ab) => {
            let buf = ab.into_value()?;
            canvas_c::canvas_native_webgl_tex_sub_image2d(
              target,
              level,
              xoffset,
              yoffset,
              format_or_width,
              type_or_height,
              format,
              type_,
              buf.as_ptr(),
              buf.len(),
              self.state,
            );
          }
          Either5::E(offset) => {
            canvas_c::canvas_native_webgl_tex_sub_image2d_offset(
              target,
              level,
              xoffset,
              yoffset,
              format_or_width,
              type_or_height,
              format,
              type_,
              offset,
              self.state,
            );
          }
        },
        _ => {}
      },
      Either7::B(bitmap) => canvas_c::canvas_native_webgl_tex_sub_image2d_asset(
        target,
        level,
        xoffset,
        yoffset,
        format_or_width as u32,
        type_or_height,
        Arc::as_ptr(&bitmap.asset),
        self.state,
      ),
      Either7::C(asset) => canvas_c::canvas_native_webgl_tex_sub_image2d_asset(
        target,
        level,
        xoffset,
        yoffset,
        format_or_width as u32,
        type_or_height,
        Arc::as_ptr(&asset.asset),
        self.state,
      ),
      Either7::D(c2d) => {
        // todo
        canvas_c::canvas_native_webgl_tex_sub_image2d_canvas2d(
          target,
          level,
          xoffset,
          yoffset,
          format_or_width as u32,
          type_or_height,
          c2d.context as _,
          self.state,
        )
      }
      Either7::E(gl) => canvas_c::canvas_native_webgl_tex_sub_image2d_webgl(
        target,
        level,
        xoffset,
        yoffset,
        format_or_width as u32,
        type_or_height,
        gl.state as _,
        self.state,
      ),
      Either7::F(gl2) => canvas_c::canvas_native_webgl_tex_sub_image2d_webgl(
        target,
        level,
        xoffset,
        yoffset,
        format_or_width as u32,
        type_or_height,
        gl2.state as _,
        self.state,
      ),
      Either7::G(data) => {
        let inner = data.data.inner();
        let (width, height) = inner.dimensions();
        let data = inner.data();
        canvas_c::canvas_native_webgl_tex_sub_image2d(
          target,
          level,
          xoffset,
          yoffset,
          width,
          height,
          format_or_width as u32,
          gl_bindings::RGBA as i32,
          data.as_ptr(),
          data.len(),
          self.state,
        );
      }
    }
    Ok(())
  }


    #[napi(js_name = "uniform1f")]
    pub fn uniform1f(&self, location: ClassInstance<WebGLUniformLocation>, v0: f64) {
        canvas_c::canvas_native_webgl_uniform1f(location.0, v0 as f32, self.state);
    }


    #[napi(js_name = "uniform1iv")]
    pub fn uniform1iv(&self, location: ClassInstance<WebGLUniformLocation>, value: Either<Vec<i32>, Int32Array>) {
        match value {
            Either::A(array) => {
                canvas_c::canvas_native_webgl_uniform1iv(location.0, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform1iv(location.0, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniform1fv")]
    pub fn uniform1fv(&self, location: ClassInstance<WebGLUniformLocation>, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_uniform1fv(location.0, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform1fv(location.0, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniform1i")]
    pub fn uniform1i(&self, location: ClassInstance<WebGLUniformLocation>, v0: Either<i32, bool>) {
          let v0 = match v0 {
              Either::A(v) => v,
              Either::B(v) => if v { 1 } else { 0 },
          };
        canvas_c::canvas_native_webgl_uniform1i(location.0, v0, self.state);
    }

    #[napi(js_name = "uniform2f")]
    pub fn uniform2f(&self, location: ClassInstance<WebGLUniformLocation>, v0: f64, v1: f64) {
        canvas_c::canvas_native_webgl_uniform2f(location.0, v0 as f32, v1 as f32, self.state);
    }

    #[napi(js_name = "uniform2iv")]
    pub fn uniform2iv(&self, location: ClassInstance<WebGLUniformLocation>, value: Either<Vec<i32>, Int32Array>) {
        match value {
            Either::A(array) => {
                canvas_c::canvas_native_webgl_uniform2iv(location.0, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform2iv(location.0, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniform2fv")]
    pub fn uniform2fv(&self, location: ClassInstance<WebGLUniformLocation>, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_uniform2fv(location.0, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform2fv(location.0, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniform2i")]
    pub fn uniform2i(&self, location: ClassInstance<WebGLUniformLocation>, v0: i32, v1: i32) {
        canvas_c::canvas_native_webgl_uniform2i(location.0, v0, v1, self.state);
    }

    #[napi(js_name = "uniform3f")]
    pub fn uniform3f(&self, location: ClassInstance<WebGLUniformLocation>, v0: f64, v1: f64, v2: f64) {
        canvas_c::canvas_native_webgl_uniform3f(location.0, v0 as f32, v1 as f32, v2 as f32, self.state);
    }

    #[napi(js_name = "uniform3iv")]
    pub fn uniform3iv(&self, location: ClassInstance<WebGLUniformLocation>, value: Either<Vec<i32>, Int32Array>) {
        match value {
            Either::A(array) => {
                canvas_c::canvas_native_webgl_uniform3iv(location.0, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform3iv(location.0, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniform3fv")]
    pub fn uniform3fv(&self, location: ClassInstance<WebGLUniformLocation>, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_uniform3fv(location.0, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform3fv(location.0, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniform3i")]
    pub fn uniform3i(&self, location: ClassInstance<WebGLUniformLocation>, v0: i32, v1: i32, v2: i32) {
        canvas_c::canvas_native_webgl_uniform3i(location.0, v0, v1, v2, self.state);
    }

    #[napi(js_name = "uniform4f")]
    pub fn uniform4f(&self, location: ClassInstance<WebGLUniformLocation>, v0: f64, v1: f64, v2: f64, v3: f64) {
        canvas_c::canvas_native_webgl_uniform4f(location.0, v0 as f32, v1 as f32, v2 as f32, v3 as f32, self.state);
    }

    #[napi(js_name = "uniform4iv")]
    pub fn uniform4iv(&self, location: ClassInstance<WebGLUniformLocation>, value: Either<Vec<i32>, Int32Array>) {
        match value {
            Either::A(array) => {
                canvas_c::canvas_native_webgl_uniform4iv(location.0, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform4iv(location.0, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniform4fv")]
    pub fn uniform4fv(&self, location: ClassInstance<WebGLUniformLocation>, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_uniform4fv(location.0, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform4fv(location.0, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniform4i")]
    pub fn uniform4i(&self, location: ClassInstance<WebGLUniformLocation>, v0: i32, v1: i32, v2: i32, v3: i32) {
        canvas_c::canvas_native_webgl_uniform4i(location.0, v0, v1, v2, v3, self.state);
    }

    #[napi(js_name = "uniformMatrix2fv")]
    pub fn uniform_matrix2fv(&self, location: ClassInstance<WebGLUniformLocation>, transpose: bool, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_uniform_matrix2fv(location.0, transpose, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform_matrix2fv(location.0, transpose, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniformMatrix3fv")]
    pub fn uniform_matrix3fv(&self, location: ClassInstance<WebGLUniformLocation>, transpose: bool, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_uniform_matrix3fv(location.0, transpose, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform_matrix3fv(location.0, transpose, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "uniformMatrix4fv")]
    pub fn uniform_matrix4fv(&self, location: ClassInstance<WebGLUniformLocation>, transpose: bool, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_uniform_matrix4fv(location.0, transpose, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_uniform_matrix4fv(location.0, transpose, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi]
    pub fn use_program(&self, program: Option<ClassInstance<WebGLProgram>>) {
        let program = program.map(|p| p.0).unwrap_or(0);
        canvas_c::canvas_native_webgl_use_program(program, self.state)
    }

    #[napi]
    pub fn validate_program(&self, program: ClassInstance<WebGLProgram>) {
        canvas_c::canvas_native_webgl_validate_program(program.0, self.state)
    }

    #[napi(js_name = "vertexAttrib1f")]
    pub fn vertex_attrib1f(&self, index: u32, v0: f64) {
        canvas_c::canvas_native_webgl_vertex_attrib1f(index, v0 as f32, self.state);
    }

    #[napi(js_name = "vertexAttrib1fv")]
    pub fn vertex_attrib1fv(&self, index: u32, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_vertex_attrib1fv(index, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_vertex_attrib1fv(index, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }


    #[napi(js_name = "vertexAttrib2f")]
    pub fn vertex_attrib2f(&self, index: u32, v0: f64, v1: f64) {
        canvas_c::canvas_native_webgl_vertex_attrib2f(index, v0 as f32, v1 as f32, self.state);
    }

    #[napi(js_name = "vertexAttrib2fv")]
    pub fn vertex_attrib2fv(&self, index: u32, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_vertex_attrib2fv(index, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_vertex_attrib2fv(index, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "vertexAttrib3f")]
    pub fn vertex_attrib3f(&self, index: u32, v0: f64, v1: f64, v2: f64) {
        canvas_c::canvas_native_webgl_vertex_attrib3f(index, v0 as f32, v1 as f32, v2 as f32, self.state);
    }

    #[napi(js_name = "vertexAttrib3fv")]
    pub fn vertex_attrib3fv(&self, index: u32, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_vertex_attrib3fv(index, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_vertex_attrib3fv(index, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(js_name = "vertexAttrib4f")]
    pub fn vertex_attrib4f(&self, index: u32, v0: f64, v1: f64, v2: f64, v3: f64) {
        canvas_c::canvas_native_webgl_vertex_attrib4f(index, v0 as f32, v1 as f32, v2 as f32, v3 as f32, self.state);
    }

    #[napi(js_name = "vertexAttrib4fv")]
    pub fn vertex_attrib4fv(&self, index: u32, value: Either<Vec<f64>, Float32Array>) {
        match value {
            Either::A(array) => {
                let array = array.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                canvas_c::canvas_native_webgl_vertex_attrib4fv(index, array.as_ptr(), array.len(), self.state);
            }
            Either::B(buffer) => {
                canvas_c::canvas_native_webgl_vertex_attrib4fv(index, buffer.as_ptr(), buffer.len(), self.state);
            }
        }
    }

    #[napi(
        ts_args_type = "index: number, size: number, type: number, normalized: boolean, stride: number, offset: number"
    )]
    pub fn vertex_attrib_pointer(&self, index: u32, size: i32, type_: u32, normalized: bool, stride: i32, offset: i64) {
        canvas_c::canvas_native_webgl_vertex_attrib_pointer(
            index, size, type_, normalized, stride, offset as isize, self.state,
        )
    }

    #[napi]
    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        canvas_c::canvas_native_webgl_viewport(x, y, width, height, self.state);
    }


    #[napi(js_name = "toDataURL")]
    pub fn to_data_url(&self, format: Option<String>, encoderOptions: Option<f64>) -> String {
        let c_str = CString::new(format.unwrap_or("image/png".to_string())).unwrap();
        let quality = encoderOptions
            .map(|v| v as f32)
            .unwrap_or(0.92)
            .try_into()
            .unwrap_or(0.92);
        let quality: u32 = (quality * 100.) as u32;
        let ret = canvas_c::canvas_native_webgl_to_data_url(self.state, c_str.as_ptr(), quality);
        unsafe { CString::from_raw(ret as _).to_string_lossy().to_string() }
    }
    }
  };
}
