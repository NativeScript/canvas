//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once
#pragma process_pending_includes

#include <vector>
#include "Common.h"

#include "WebGLRenderingContextBase.h"

#include <cmath>
#include "Helpers.h"

#include "ImageAssetImpl.h"
#include "ImageBitmapImpl.h"
#include "RafImpl.h"

#include "CanvasRenderingContext2DImpl.h"

#include "WebGLBuffer.h"
#include "WebGLFramebuffer.h"
#include "WebGLProgram.h"
#include "WebGLRenderbuffer.h"
#include "WebGLShader.h"
#include "WebGLTexture.h"
#include "WebGLUniformLocation.h"
#include "WebGLShaderPrecisionFormatImpl.h"
#include "WebGLActiveInfoImpl.h"

#include "extensions/EXT_blend_minmaxImpl.h"
#include "extensions/EXT_color_buffer_half_floatImpl.h"
#include "extensions/EXT_disjoint_timer_queryImpl.h"
#include "extensions/EXT_sRGBImpl.h"
#include "extensions/EXT_shader_texture_lodImpl.h"
#include "extensions/EXT_texture_filter_anisotropicImpl.h"
#include "extensions/OES_element_index_uintImpl.h"
#include "extensions/OES_standard_derivativesImpl.h"
#include "extensions/OES_texture_floatImpl.h"
#include "extensions/OES_texture_float_linearImpl.h"
#include "extensions/OES_texture_half_floatImpl.h"
#include "extensions/OES_texture_half_float_linearImpl.h"
#include "extensions/OES_vertex_array_objectImpl.h"
#include "extensions/WEBGL_color_buffer_floatImpl.h"
#include "extensions/WEBGL_compressed_texture_atcImpl.h"
#include "extensions/WEBGL_compressed_texture_etcImpl.h"
#include "extensions/WEBGL_compressed_texture_etc1Impl.h"
#include "extensions/WEBGL_compressed_texture_pvrtcImpl.h"
#include "extensions/WEBGL_compressed_texture_s3tcImpl.h"
#include "extensions/WEBGL_compressed_texture_s3tc_srgbImpl.h"
#include "extensions/WEBGL_depth_textureImpl.h"
#include "extensions/WEBGL_lose_contextImpl.h"
#include "extensions/ANGLE_instanced_arraysImpl.h"
#include "extensions/WEBGL_draw_buffersImpl.h"
#include "extensions/OES_fbo_render_mipmap.h"
#include "gl.h"


class WebGLRenderingContext : public WebGLRenderingContextBase {
public:

    WebGLRenderingContext(WebGLState *state);

    WebGLRenderingContext(WebGLState *state, WebGLRenderingVersion version);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::CFunction fast_resized_;

    static v8::CFunction fast_start_raf_;

    static v8::CFunction fast_stop_raf_;

    static v8::CFunction fast_active_texture_;

    static v8::CFunction fast_attach_shader_;

    static v8::CFunction fast_bind_buffer_;

    static v8::CFunction fast_bind_buffer_null_;

    static v8::CFunction fast_bind_frame_buffer_;

    static v8::CFunction fast_bind_frame_buffer_null_;

    static v8::CFunction fast_bind_render_buffer_;

    static v8::CFunction fast_bind_render_buffer_null_;

    static v8::CFunction fast_bind_texture_;

    static v8::CFunction fast_bind_texture_null_;

    static v8::CFunction fast_blend_color_;
    static v8::CFunction fast_blend_equation_separate_;
    static v8::CFunction fast_blend_equation_;
    static v8::CFunction fast_blend_func_separate_;
    static v8::CFunction fast_blend_func_;

    static v8::CFunction fast_buffer_data_os_;
    static v8::CFunction fast_buffer_data_target_usage_;
    static v8::CFunction fast_buffer_data_u8_;
    static v8::CFunction fast_buffer_data_i8_;
    static v8::CFunction fast_buffer_data_u16_;
    static v8::CFunction fast_buffer_data_i16_;
    static v8::CFunction fast_buffer_data_u32_;
    static v8::CFunction fast_buffer_data_i32_;
    static v8::CFunction fast_buffer_data_f32_;
    static v8::CFunction fast_buffer_data_f64_;
    static v8::CFunction fast_buffer_data_array_buffer_;

    static v8::CFunction fast_buffer_sub_data_target_offset_;

    static v8::CFunction fast_buffer_sub_data_u8_;
    static v8::CFunction fast_buffer_sub_data_i8_;

    static v8::CFunction fast_buffer_sub_data_u16_;
    static v8::CFunction fast_buffer_sub_data_i16_;

    static v8::CFunction fast_buffer_sub_data_u32_;
    static v8::CFunction fast_buffer_sub_data_i32_;

    static v8::CFunction fast_buffer_sub_data_f32_;
    static v8::CFunction fast_buffer_sub_data_f64_;

    static v8::CFunction fast_buffer_sub_data_array_buffer_;

    static v8::CFunction fast_check_framebuffer_status_;

    static v8::CFunction fast_draw_elements_;

    static v8::CFunction fast_vertex_attrib_1f_;

    static v8::CFunction fast_vertex_attrib_1fv_;

    static v8::CFunction fast_vertex_attrib_2f_;

    static v8::CFunction fast_vertex_attrib_2fv_;

    static v8::CFunction fast_vertex_attrib_3f_;

    static v8::CFunction fast_vertex_attrib_3fv_;

    static v8::CFunction fast_vertex_attrib_4f_;

    static v8::CFunction fast_vertex_attrib_4fv_;


    static v8::CFunction fast_uniform1f_;

    static v8::CFunction fast_uniform1i_;

    static v8::CFunction fast_uniform2f_;

    static v8::CFunction fast_uniform2i_;

    static v8::CFunction fast_uniform3f_;

    static v8::CFunction fast_uniform3i_;

    static v8::CFunction fast_uniform4f_;

    static v8::CFunction fast_uniform4i_;

    static v8::CFunction fast_clear_depth_;

    static v8::CFunction fast_clear_stencil_;

    static v8::CFunction fast_clear_;

    static v8::CFunction fast_clear_color_;

    static v8::CFunction fast_viewport_;

    static v8::CFunction fast_enable_;

    static v8::CFunction fast_enable_vertex_attrib_array_;

    static v8::CFunction fast_use_program_;

    static v8::CFunction fast_use_program_null_;

    static v8::CFunction fast_draw_arrays_;

    static v8::CFunction fast_uniform_matrix2fv_;

    static v8::CFunction fast_uniform_matrix2fv_array_;

    static v8::CFunction fast_uniform_matrix3fv_;

    static v8::CFunction fast_uniform_matrix3fv_array_;

    static v8::CFunction fast_uniform_matrix4fv_;

    static v8::CFunction fast_uniform_matrix4fv_array_;

    static v8::CFunction fast_uniform_1fv_;

    static v8::CFunction fast_uniform_1fv_array_;

    static v8::CFunction fast_uniform_2fv_;

    static v8::CFunction fast_uniform_2fv_array_;

    static v8::CFunction fast_uniform_3fv_;

    static v8::CFunction fast_uniform_3fv_array_;

    static v8::CFunction fast_uniform_4fv_;

    static v8::CFunction fast_uniform_4fv_array_;

    static v8::CFunction fast_uniform_1iv_;

    static v8::CFunction fast_uniform_1iv_array_;

    static v8::CFunction fast_uniform_2iv_;

    static v8::CFunction fast_uniform_2iv_array_;

    static v8::CFunction fast_uniform_3iv_;

    static v8::CFunction fast_uniform_3iv_array_;

    static v8::CFunction fast_uniform_4iv_;

    static v8::CFunction fast_uniform_4iv_array_;

    static v8::CFunction fast_vertex_attrib_pointer_;

    static v8::CFunction fast_validate_program_;

    static v8::CFunction fast_delete_buffer_;

    static v8::CFunction fast_delete_buffer_framebuffer_;

    static v8::CFunction fast_delete_program_;
    static v8::CFunction fast_delete_renderbuffer_;
    static v8::CFunction fast_delete_shader_;
    static v8::CFunction fast_delete_texture_;

    static v8::CFunction fast_disable_;

    static v8::CFunction fast_compile_shader_;

    static v8::CFunction fast_framebuffer_texture_2d_;


    static v8::CFunction fast_color_mask_;
    static v8::CFunction fast_copy_tex_image_2d_;
    static v8::CFunction fast_copy_tex_sub_image_2d_;
    static v8::CFunction fast_cull_face_;
    static v8::CFunction fast_depth_func_;
    static v8::CFunction fast_depth_mask_;
    static v8::CFunction fast_depth_range_;
    static v8::CFunction fast_detach_shader_;
    static v8::CFunction fast_disable_vertex_attrib_array_;

    static v8::CFunction fast_finish_;
    static v8::CFunction fast_flush_;
    static v8::CFunction fast_framebuffer_renderbuffer_;
    static v8::CFunction fast_front_face_;
    static v8::CFunction fast_generate_mipmap_;
    static v8::CFunction fast_get_vertex_attrib_offset_;
    static v8::CFunction fast_hint_;
    static v8::CFunction fast_is_buffer_;
    static v8::CFunction fast_is_context_lost_;
    static v8::CFunction fast_is_enabled_;
    static v8::CFunction fast_is_framebuffer_;
    static v8::CFunction fast_is_program_;
    static v8::CFunction fast_is_renderbuffer_;
    static v8::CFunction fast_is_shader_;
    static v8::CFunction fast_is_texture_;
    static v8::CFunction fast_line_width_;
    static v8::CFunction fast_link_program_;
    static v8::CFunction fast_pixel_storei_bool_;
    static v8::CFunction fast_pixel_storei_;
    static v8::CFunction fast_polygon_offset_;

    static v8::CFunction fast_renderbuffer_storage_;
    static v8::CFunction fast_sample_coverage_;
    static v8::CFunction fast_scissor_;


    static v8::CFunction fast_stencil_func_separate_;
    static v8::CFunction fast_stencil_func_;
    static v8::CFunction fast_stencil_mask_separate_;
    static v8::CFunction fast_stencil_mask_;
    static v8::CFunction fast_stencil_op_separate_;
    static v8::CFunction fast_stencil_op_;

    static v8::CFunction fast_tex_parameterf_;
    static v8::CFunction fast_tex_parameteri_;



    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, WebGLRenderingContext *renderingContext) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WebGLRenderingContext::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::WebGLRenderingContextBase);
        object->SetAlignedPointerInInternalField(0, renderingContext);
        renderingContext->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WebGLRenderingContext *GetPointer(const v8::Local<v8::Object> &object);


    static void SetConstants(v8::Isolate *isolate, const v8::Local<v8::ObjectTemplate> &tmpl);

    static void
    SetProps(v8::Isolate *isolate, const v8::Local<v8::ObjectTemplate> &tmpl);

    static void SetMethods(v8::Isolate *isolate,
                           const v8::Local<v8::ObjectTemplate> &tmpl);


    v8::Local<v8::Value> GetParameterInternal(v8::Isolate *isolate, uint32_t pnameValue,
                                              WebGLResult *result);

    static void GetDrawingBufferWidth(v8::Local<v8::String> name,
                                      const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetFlipY(v8::Local<v8::String> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetDrawingBufferHeight(v8::Local<v8::String> name,
                                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void __Resized(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void __FastResized(v8::Local<v8::Object> receiver_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_resized(
                ptr->GetState());

    }

    static void __StartRaf(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void __FastStartRaf(v8::Local<v8::Object> receiver_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        ptr->StartRaf();

    }

    static void __StopRaf(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void __FastStopRaf(v8::Local<v8::Object> receiver_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        ptr->StopRaf();

    }

    static void ActiveTexture(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void FastActiveTexture(v8::Local<v8::Object> receiver_obj, uint32_t texture) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }
        canvas_native_webgl_active_texture(texture,
                                           ptr->GetState());
    }


    static void AttachShader(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    AttachShaderImpl(WebGLRenderingContext *ptr, WebGLProgram *program, WebGLShader *shader) {
        if (program == nullptr || shader == nullptr) {
            return;
        }

        canvas_native_webgl_attach_shader(
                program->GetProgram(),
                shader->GetShader(),
                ptr->GetState()
        );
    }

    static void
    FastAttachShader(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> program_obj,
                     v8::Local<v8::Object> shader_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto programType = GetNativeType(program_obj);
        auto shaderType = GetNativeType(shader_obj);

        WebGLProgram *program = nullptr;
        WebGLShader *shader = nullptr;
        if (programType == NativeType::WebGLProgram) {
            program = WebGLProgram::GetPointer(program_obj);
        }

        if (shaderType == NativeType::WebGLShader) {
            shader = WebGLShader::GetPointer(shader_obj);
        }

        AttachShaderImpl(
                ptr,
                program,
                shader
        );
    }

    static void BindAttribLocation(const v8::FunctionCallbackInfo<v8::Value> &args);


    // todo with fast string
    /*
    static void FastBindAttribLocation(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> program_obj, uint32_t index) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto type = GetNativeType(program_obj);
        if (type == NativeType::WebGLProgram) {
            auto program = WebGLProgram::GetPointer(program_obj);

            auto name = ConvertFromV8String(isolate, args[2]);
            canvas_native_webgl_bind_attrib_location(
                    program->GetProgram(),
                    index,
                    name.c_str(),
                    ptr->GetState()
            );

        }
    }
*/
    static void BindBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastBindBuffer(v8::Local<v8::Object> receiver_obj, uint32_t target,
                               v8::Local<v8::Object> buffer_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(buffer_obj);
        if (type == NativeType::WebGLBuffer) {
            auto buffer = WebGLBuffer::GetPointer(buffer_obj);
            if (buffer ==
                nullptr) { return; }
            canvas_native_webgl_bind_buffer(
                    target,
                    buffer->GetBuffer(),
                    ptr->GetState()
            );
        }
    }

    static void
    FastBindBufferNull(v8::Local<v8::Object> receiver_obj, uint32_t target, int32_t buffer_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_bind_buffer(
                target,
                0,
                ptr->GetState()
        );
    }

    static void BindFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastBindFramebuffer(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                    v8::Local<v8::Object> buffer_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(buffer_obj);
        if (type == NativeType::WebGLFramebuffer) {
            auto framebuffer = WebGLFramebuffer::GetPointer(buffer_obj);
            canvas_native_webgl_bind_frame_buffer(
                    target,
                    framebuffer->GetFrameBuffer(),
                    ptr->GetState()
            );
        }
    }

    static void FastBindFramebufferNull(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                        int32_t buffer_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        // null value
        // unbind
        canvas_native_webgl_bind_frame_buffer(
                target,
                0,
                ptr->GetState()
        );
    }

    static void BindRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void FastBindRenderbuffer(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                     v8::Local<v8::Object> buffer_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto type = GetNativeType(buffer_obj);
        if (type == NativeType::WebGLRenderbuffer) {
            auto renderbuffer = WebGLRenderbuffer::GetPointer(buffer_obj);

            if (renderbuffer ==
                nullptr) { return; }
            canvas_native_webgl_bind_render_buffer(
                    target,
                    renderbuffer->GetRenderBuffer(),
                    ptr->GetState()
            );
        }
    }

    static void FastBindRenderbufferNull(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                         int32_t buffer_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_bind_render_buffer(
                target,
                0,
                ptr->GetState()
        );
    }

    static void BindTexture(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastBindTexture(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                v8::Local<v8::Object> texture_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(texture_obj);
        if (type == NativeType::WebGLTexture) {
            auto texture = WebGLTexture::GetPointer(texture_obj);
            canvas_native_webgl_bind_texture(
                    target,
                    texture->GetTexture(),
                    ptr->GetState()
            );
        }
    }

    static void FastBindTextureNull(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                    int32_t texture_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_bind_texture(
                target,
                0,
                ptr->GetState()
        );
    }

    static void BlendColor(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastBlendColor(v8::Local<v8::Object> receiver_obj, double red, double green, double blue,
                   double alpha) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_blend_color(
                static_cast<float>(red),
                static_cast<float>(green),
                static_cast<float>(blue),
                static_cast<float>(alpha),
                ptr->GetState()
        );
    }

    static void BlendEquationSeparate(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastBlendEquationSeparate(v8::Local<v8::Object> receiver_obj, uint32_t modeRGB,
                                          uint32_t modeAlpha) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_blend_equation_separate(
                modeRGB,
                modeAlpha,
                ptr->GetState()
        );
    }

    static void BlendEquation(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastBlendEquation(v8::Local<v8::Object> receiver_obj, uint32_t mode) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }
        canvas_native_webgl_blend_equation(
                mode,
                ptr->GetState()
        );
    }

    static void BlendFuncSeparate(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastBlendFuncSeparate(v8::Local<v8::Object> receiver_obj, uint32_t srcRGB, uint32_t dstRGB,
                          uint32_t srcAlpha, uint32_t dstAlpha) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_blend_func_separate(
                srcRGB,
                dstRGB,
                srcAlpha,
                dstAlpha,
                ptr->GetState()
        );
    }

    static void BlendFunc(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastBlendFunc(v8::Local<v8::Object> receiver_obj, uint32_t sfactor, uint32_t dfactor) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_blend_func(
                sfactor,
                dfactor,
                ptr->GetState()
        );
    }

    static void BufferData(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastBufferDataTargetUsage(v8::Local<v8::Object> receiver_obj, uint32_t target, uint32_t usage) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_buffer_data_none(
                target,
                0,
                usage,
                ptr->GetState()
        );
    }

    /*
     *
   static void FastBufferDataU8(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                const v8::FastApiArrayBufferView &srcData, uint32_t usage) {
       WebGLRenderingContext *ptr = GetPointer(receiver_obj);
       if (ptr == nullptr) {
           return;
       }


       canvas_native_webgl_buffer_data(
               target,
               (uint8_t *) srcData.data, srcData.byte_length,
               usage,
               ptr->GetState()
       );
   }


   static void FastBufferDataI8(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                 const v8::FastApiArrayBufferView &srcData, uint32_t usage) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_buffer_data_i8(
                target,
                (int8_t *)srcData.data, srcData.byte_length,
                usage,
                ptr->GetState()
        );
    }

    static void FastBufferDataU16(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                  const v8::FastApiArrayBufferView &srcData, uint32_t usage) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_buffer_data_u16(
                target,
                (uint16_t *)srcData.data, srcData.byte_length / sizeof(uint16_t),
                usage,
                ptr->GetState()
        );
    }

    static void FastBufferDataI16(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                  const v8::FastApiArrayBufferView &srcData, uint32_t usage) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        canvas_native_webgl_buffer_data_i16(
                target,
                (int16_t *)srcData.data, srcData.byte_length / sizeof(int16_t),
                usage,
                ptr->GetState()
        );
    }

    */

    static void FastBufferDataF32(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                  const v8::FastApiTypedArray<float> &srcData, uint32_t usage) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto size = srcData.length();
        float *data;
        srcData.getStorageIfAligned(&data);


        canvas_native_webgl_buffer_data_f32(
                target,
                data, size,
                usage,
                ptr->GetState()
        );
    }

    static void FastBufferDataF64(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                  const v8::FastApiTypedArray<double> &srcData, uint32_t usage) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto size = srcData.length();
        double *data;
        srcData.getStorageIfAligned(&data);


        canvas_native_webgl_buffer_data_f64(
                target,
                data, size,
                usage,
                ptr->GetState()
        );
    }

    static void FastBufferDataU32(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                  const v8::FastApiTypedArray<uint32_t> &srcData, uint32_t usage) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto size = srcData.length();
        uint32_t *data;
        srcData.getStorageIfAligned(&data);


        canvas_native_webgl_buffer_data_u32(
                target,
                data, size,
                usage,
                ptr->GetState()
        );
    }

    static void FastBufferDataI32(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                  const v8::FastApiTypedArray<int32_t> &srcData, uint32_t usage) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto size = srcData.length();
        int32_t *data;
        srcData.getStorageIfAligned(&data);


        canvas_native_webgl_buffer_data_i32(
                target,
                data, size,
                usage,
                ptr->GetState()
        );
    }

    static void FastBufferDataArrayBuffer(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                          const v8::FastApiArrayBuffer &srcData, uint32_t usage) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        canvas_native_webgl_buffer_data(
                target,
                (uint8_t *) srcData.data, srcData.byte_length,
                usage,
                ptr->GetState()
        );
    }

    static void
    FastBufferDataOS(v8::Local<v8::Object> receiver_obj, uint32_t target, uint32_t offset,
                     uint32_t usage) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        canvas_native_webgl_buffer_data_none(
                target,
                static_cast<ssize_t>(offset),
                usage,
                ptr->GetState()
        );
    }

    static void BufferSubData(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastBufferSubDataTargetOffset(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                              int32_t offset) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_buffer_sub_data_none(
                target,
                static_cast<ssize_t>(offset),
                ptr->GetState()
        );
    }

    /*
   static void
   FastBufferSubDataU8(v8::Local<v8::Object> receiver_obj, uint32_t target, int32_t offset,
                       const v8::FastApiArrayBufferView &srcData) {
       WebGLRenderingContext *ptr = GetPointer(receiver_obj);
       if (ptr == nullptr) {
           return;
       }


       canvas_native_webgl_buffer_sub_data(
               target,
               static_cast<ssize_t>(offset),
               (uint8_t *) srcData.data, srcData.byte_length,
               ptr->GetState()
       );
   }


   static void
   FastBufferSubDataI8(v8::Local<v8::Object> receiver_obj, uint32_t target, int32_t offset,
                     const v8::FastApiArrayBufferView &srcData) {
       WebGLRenderingContext *ptr = GetPointer(receiver_obj);
       if (ptr == nullptr) {
           return;
       }

       auto size = srcData.length();
       int8_t *data;
       srcData.getStorageIfAligned(&data);


       canvas_native_webgl_buffer_sub_data_i8(
               target,
               static_cast<ssize_t>(offset),
               data, size,
               ptr->GetState()
       );
   }


   static void
   FastBufferSubDataU16(v8::Local<v8::Object> receiver_obj, uint32_t target, int32_t offset,
                       const v8::FastApiArrayBufferView &srcData) {
       WebGLRenderingContext *ptr = GetPointer(receiver_obj);
       if (ptr == nullptr) {
           return;
       }

       auto size = srcData.length();
       uint16_t *data;
       srcData.getStorageIfAligned(&data);


       canvas_native_webgl_buffer_sub_data_u16(
               target,
               static_cast<ssize_t>(offset),
               data, size,
               ptr->GetState()
       );
   }


   static void
   FastBufferSubDataI16(v8::Local<v8::Object> receiver_obj, uint32_t target, int32_t offset,
                        const v8::FastApiArrayBufferView&srcData) {
       WebGLRenderingContext *ptr = GetPointer(receiver_obj);
       if (ptr == nullptr) {
           return;
       }

       auto size = srcData.length();
       int16_t *data;
       srcData.getStorageIfAligned(&data);


       canvas_native_webgl_buffer_sub_data_i16(
               target,
               static_cast<ssize_t>(offset),
               data, size,
               ptr->GetState()
       );
   }


   */
    static void
    FastBufferSubDataU32(v8::Local<v8::Object> receiver_obj, uint32_t target, int32_t offset,
                         const v8::FastApiTypedArray<uint32_t> &srcData) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto size = srcData.length();
        uint32_t *data;
        srcData.getStorageIfAligned(&data);


        canvas_native_webgl_buffer_sub_data_u32(
                target,
                static_cast<ssize_t>(offset),
                data, size,
                ptr->GetState()
        );
    }


    static void
    FastBufferSubDataI32(v8::Local<v8::Object> receiver_obj, uint32_t target, int32_t offset,
                         const v8::FastApiTypedArray<int32_t> &srcData) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto size = srcData.length();
        int32_t *data;
        srcData.getStorageIfAligned(&data);


        canvas_native_webgl_buffer_sub_data_i32(
                target,
                static_cast<ssize_t>(offset),
                data, size,
                ptr->GetState()
        );
    }


    static void
    FastBufferSubDataF32(v8::Local<v8::Object> receiver_obj, uint32_t target, int32_t offset,
                         const v8::FastApiTypedArray<float> &srcData) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto size = srcData.length();
        float *data;
        srcData.getStorageIfAligned(&data);


        canvas_native_webgl_buffer_sub_data_f32(
                target,
                static_cast<ssize_t>(offset),
                data, size,
                ptr->GetState()
        );
    }


    static void
    FastBufferSubDataF64(v8::Local<v8::Object> receiver_obj, uint32_t target, int32_t offset,
                         const v8::FastApiTypedArray<double> &srcData) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto size = srcData.length();
        double *data;
        srcData.getStorageIfAligned(&data);


        canvas_native_webgl_buffer_sub_data_f64(
                target,
                static_cast<ssize_t>(offset),
                data, size,
                ptr->GetState()
        );
    }


    static void FastBufferSubDataArrayBuffer(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                             int32_t offset,
                                             const v8::FastApiArrayBuffer &srcData) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_buffer_sub_data(
                target,
                static_cast<intptr_t>(offset),
                (uint8_t *) srcData.data, srcData.byte_length,
                ptr->GetState()
        );
    }

    static void CheckFramebufferStatus(const v8::FunctionCallbackInfo<v8::Value> &args);

    static uint32_t
    FastCheckFramebufferStatus(v8::Local<v8::Object> receiver_obj, uint32_t target) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return 0;
        }

        return canvas_native_webgl_check_frame_buffer_status(
                target,
                ptr->GetState()
        );
    }

    static void ClearColor(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    ClearColorImpl(WebGLRenderingContext *ptr, float red, float green, float blue, float alpha) {
        canvas_native_webgl_clear_color(
                red, green, blue, alpha,
                ptr->GetState()
        );
    }

    static void
    FastClearColor(v8::Local<v8::Object> receiver_obj, double red, double green, double blue,
                   double alpha) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }
        ClearColorImpl(
                ptr,
                static_cast<float>(red),
                static_cast<float>(green),
                static_cast<float>(blue),
                static_cast<float>(alpha)
        );
    }

    static void ClearDepth(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastClearDepth(v8::Local<v8::Object> receiver_obj, double depth) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_clear_depth(
                static_cast<float>(depth),
                ptr->GetState()
        );
    }

    static void ClearStencil(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastClearStencil(v8::Local<v8::Object> receiver_obj, int32_t stencil) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_clear_stencil(
                stencil,
                ptr->GetState()
        );
    }

    static void Clear(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ClearImpl(WebGLRenderingContext *ptr, uint32_t mask) {

        canvas_native_webgl_clear(
                mask,
                ptr->GetState()
        );

        ptr->UpdateInvalidateState();
    }

    static void FastClear(v8::Local<v8::Object> receiver_obj, uint32_t mask) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        ClearImpl(
                ptr,
                mask
        );
    }

    static void ColorMask(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastColorMask(v8::Local<v8::Object> receiver_obj, bool red, bool green, bool blue, bool alpha) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_color_mask(
                red,
                green,
                blue,
                alpha,
                ptr->GetState()
        );
    }

    static void Commit(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CompileShader(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastCompileShader(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> shader_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        if (GetNativeType(shader_obj) == NativeType::WebGLShader) {
            auto shader = WebGLShader::GetPointer(shader_obj);
            if (shader != nullptr) {
                canvas_native_webgl_compile_shader(
                        shader->GetShader(),
                        ptr->GetState()
                );
            }
        }

    }

    static void CompressedTexImage2D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CompressedTexSubImage2D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CopyTexImage2D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastCopyTexImage2D(v8::Local<v8::Object> receiver_obj, uint32_t target, int32_t level,
                       uint32_t internalformat, int32_t x, int32_t y, int32_t width, int32_t height,
                       int32_t border) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_copy_tex_image2d(
                target,
                level,
                internalformat,
                x,
                y,
                width,
                height,
                border,
                ptr->GetState()
        );

    }

    static void CopyTexSubImage2D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastCopyTexSubImage2D(v8::Local<v8::Object> receiver_obj, uint32_t target, int32_t level,
                          int32_t xoffset, int32_t yoffset, int32_t x, int32_t y, int32_t width,
                          int32_t height) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        canvas_native_webgl_copy_tex_sub_image2d(
                target,
                level,
                xoffset,
                yoffset,
                x,
                y,
                width,
                height,
                ptr->GetState()
        );

    }

    static void CreateBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateProgram(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateShader(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateTexture(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CullFace(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastCullFace(v8::Local<v8::Object> receiver_obj, uint32_t mode) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        canvas_native_webgl_cull_face(
                mode,
                ptr->GetState()
        );

    }

    static void DeleteBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastDeleteBuffer(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto type = GetNativeType(value);
        if (type == NativeType::WebGLBuffer) {
            auto buffer = WebGLBuffer::GetPointer(value);
            if (buffer != nullptr) {
                canvas_native_webgl_delete_buffer(
                        buffer->GetBuffer(),
                        ptr->GetState()
                );
            }
        }

    }

    static void DeleteFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastDeleteFramebuffer(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(value);
        if (type == NativeType::WebGLFramebuffer) {
            auto buffer = WebGLFramebuffer::GetPointer(value);
            if (buffer != nullptr) {
                canvas_native_webgl_delete_framebuffer(
                        buffer->GetFrameBuffer(),
                        ptr->GetState()
                );
            }
        }

    }

    static void DeleteProgram(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastDeleteProgram(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(value);

        if (type == NativeType::WebGLProgram) {
            auto program = WebGLProgram::GetPointer(value);
            if (program != nullptr) {
                canvas_native_webgl_delete_framebuffer(
                        program->GetProgram(),
                        ptr->GetState()
                );
            }
        }


    }

    static void DeleteRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastDeleteRenderbuffer(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto type = GetNativeType(value);

        if (type == NativeType::WebGLRenderbuffer) {
            auto buffer = WebGLRenderbuffer::GetPointer(value);
            if (buffer != nullptr) {
                canvas_native_webgl_delete_renderbuffer(
                        buffer->GetRenderBuffer(),
                        ptr->GetState()
                );
            }
        }


    }

    static void DeleteShader(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastDeleteShader(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto type = GetNativeType(value);

        if (type == NativeType::WebGLShader) {
            auto shader = WebGLShader::GetPointer(value);
            if (shader != nullptr) {
                canvas_native_webgl_delete_shader(
                        shader->GetShader(),
                        ptr->GetState()
                );
            }
        }
    }

    static void DeleteTexture(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastDeleteTexture(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(value);

        if (type == NativeType::WebGLTexture) {
            auto texture = WebGLTexture::GetPointer(value);
            if (texture != nullptr) {
                canvas_native_webgl_delete_texture(
                        texture->GetTexture(),
                        ptr->GetState()
                );
            }
        }
    }

    static void DepthFunc(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastDepthFunc(v8::Local<v8::Object> receiver_obj, uint32_t func) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_depth_func(
                func,
                ptr->GetState()
        );
    }

    static void DepthMask(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastDepthMask(v8::Local<v8::Object> receiver_obj, bool mask) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }
        canvas_native_webgl_depth_mask(
                mask,
                ptr->GetState()
        );
    }

    static void DepthRange(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastDepthRange(v8::Local<v8::Object> receiver_obj, double zNear, double zFar) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_depth_range(
                static_cast<float>(zNear),
                static_cast<float>(zFar),
                ptr->GetState()
        );
    }

    static void DetachShader(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastDetachShader(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> program_obj,
                     v8::Local<v8::Object> shader_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto programType = GetNativeType(program_obj);
        auto shaderType = GetNativeType(shader_obj);
        WebGLProgram *program = nullptr;
        WebGLShader *shader = nullptr;
        if (programType == NativeType::WebGLProgram) {
            program = WebGLProgram::GetPointer(program_obj);
        }
        if (shaderType == NativeType::WebGLShader) {
            shader = WebGLShader::GetPointer(shader_obj);
        }

        if (program != nullptr &&
            shader != nullptr) {
            canvas_native_webgl_detach_shader(
                    program->GetProgram(),
                    shader->GetShader(),
                    ptr->GetState()
            );
        }
    }

    static void DisableVertexAttribArray(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastDisableVertexAttribArray(v8::Local<v8::Object> receiver_obj, uint32_t index) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        canvas_native_webgl_disable_vertex_attrib_array(
                index,
                ptr->GetState()
        );
    }

    static void Disable(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastDisable(v8::Local<v8::Object> receiver_obj, uint32_t cap) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_disable(
                cap,
                ptr->GetState()
        );
    }

    static void DrawArrays(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    DrawArraysImpl(WebGLRenderingContext *ptr, uint32_t mode, int32_t first, int32_t count) {
        canvas_native_webgl_draw_arrays(
                mode,
                first,
                count,
                ptr->GetState()
        );
        ptr->UpdateInvalidateState();
    }

    static void FastDrawArrays(v8::Local<v8::Object> receiver_obj, uint32_t mode, int32_t first,
                               int32_t count) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_draw_arrays(
                mode,
                first,
                count,
                ptr->GetState()
        );
        ptr->UpdateInvalidateState();
    }

    static void DrawElements(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastDrawElements(v8::Local<v8::Object> receiver_obj, uint32_t mode, int32_t count, int32_t type,
                     int32_t offset) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        canvas_native_webgl_draw_elements(
                mode,
                count,
                type,
                static_cast<ssize_t>(offset),
                ptr->GetState()
        );
        ptr->UpdateInvalidateState();
    }

    static void EnableVertexAttribArray(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastEnableVertexAttribArray(v8::Local<v8::Object> receiver_obj, uint32_t index) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        canvas_native_webgl_enable_vertex_attrib_array(
                index,
                ptr->GetState()
        );
    }

    static void Enable(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    EnableImpl(WebGLRenderingContext *ptr, uint32_t cap) {
        canvas_native_webgl_enable(cap, ptr->GetState()
        );
    }

    static void FastEnable(v8::Local<v8::Object> receiver_obj, uint32_t cap) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        EnableImpl(
                ptr,
                cap
        );
    }

    static void Finish(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastFinish(v8::Local<v8::Object> receiver_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_finish(
                ptr->GetState()
        );
    }

    static void FastFlush(v8::Local<v8::Object> receiver_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_flush(
                ptr->GetState()
        );
    }

    static void Flush(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FramebufferRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastFramebufferRenderbuffer(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                            uint32_t attachment,
                                            uint32_t renderbuffertarget,
                                            v8::Local<v8::Object> renderbuffer_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto type = GetNativeType(renderbuffer_obj);
        if (type == NativeType::WebGLRenderbuffer) {
            auto renderbuffer = WebGLRenderbuffer::GetPointer(renderbuffer_obj);
            if (renderbuffer != nullptr) {
                canvas_native_webgl_framebuffer_renderbuffer(
                        target,
                        attachment,
                        renderbuffertarget,
                        renderbuffer->GetRenderBuffer(),
                        ptr->GetState()
                );
            }
        }
    }

    static void FramebufferTexture2D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastFramebufferTexture2D(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                         uint32_t attachment, uint32_t textarget, int32_t level,
                                         v8::Local<v8::Object> texture_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(texture_obj);
        if (type == NativeType::WebGLTexture) {
            auto texture = WebGLTexture::GetPointer(texture_obj);
            if (texture != nullptr) {
                canvas_native_webgl_framebuffer_texture2d(
                        target,
                        attachment,
                        textarget,
                        texture->GetTexture(),
                        level,
                        ptr->GetState()
                );
            }
        }
    }

    static void FrontFace(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastFrontFace(v8::Local<v8::Object> receiver_obj, uint32_t mode) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_front_face(
                mode,
                ptr->GetState()
        );
    }

    static void GenerateMipmap(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastGenerateMipmap(v8::Local<v8::Object> receiver_obj, uint32_t target) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_generate_mipmap(
                target,
                ptr->GetState()
        );
    }

    static void GetActiveAttrib(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetActiveUniform(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetAttachedShaders(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetAttribLocation(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetBufferParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

    static uint32_t
    FastGetBufferParameter(v8::Local<v8::Object> receiver_obj, uint32_t target, uint32_t pname) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return -1;
        }


        return canvas_native_webgl_get_buffer_parameter(
                target,
                pname,
                ptr->GetState()
        );

    }

    static void GetContextAttributes(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetError(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetExtension(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetFramebufferAttachmentParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetProgramInfoLog(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetProgramParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetRenderbufferParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetShaderInfoLog(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetShaderParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetShaderPrecisionFormat(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetShaderSource(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetSupportedExtensions(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetTexParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetUniformLocation(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetUniform(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetVertexAttribOffset(const v8::FunctionCallbackInfo<v8::Value> &args);

    static int32_t
    FastGetVertexAttribOffset(v8::Local<v8::Object> receiver_obj, uint32_t index, uint32_t pname) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return 0;
        }


        return canvas_native_webgl_get_vertex_attrib_offset(
                index,
                pname,
                ptr->GetState());
    }

    static void GetVertexAttrib(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Hint(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastHint(v8::Local<v8::Object> receiver_obj, uint32_t target, uint32_t mode) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_hint(target,
                                 mode,
                                 ptr->GetState());
    }

    static void IsBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static bool FastIsBuffer(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return false;
        }

        auto type = GetNativeType(value);
        if (type == NativeType::WebGLBuffer) {
            auto buffer = WebGLBuffer::GetPointer(value);
            if (buffer != nullptr) {
                auto ret = canvas_native_webgl_is_buffer(
                        buffer->GetBuffer(),
                        ptr->GetState());
                return ret;
            }
        }

        return false;
    }

    static void IsContextLost(const v8::FunctionCallbackInfo<v8::Value> &args);

    static bool FastIsContextLost(v8::Local<v8::Object> receiver_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return false;
        }


        auto ret = canvas_native_webgl_get_is_context_lost(
                ptr->GetState());

        return ret;
    }

    static void IsEnabled(const v8::FunctionCallbackInfo<v8::Value> &args);

    static bool FastIsEnabled(v8::Local<v8::Object> receiver_obj, uint32_t cap) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return false;
        }

        return canvas_native_webgl_is_enabled(
                cap, ptr->GetState());
    }

    static void IsFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static bool FastIsFramebuffer(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return false;
        }

        auto type = GetNativeType(value);
        if (type == NativeType::WebGLFramebuffer) {
            auto framebuffer = WebGLFramebuffer::GetPointer(value);
            if (framebuffer != nullptr) {
                auto ret = canvas_native_webgl_is_framebuffer(
                        framebuffer->GetFrameBuffer(),
                        ptr->GetState());
                return ret;
            }
        }

        return false;
    }

    static void IsProgram(const v8::FunctionCallbackInfo<v8::Value> &args);

    static bool FastIsProgram(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return false;
        }

        auto type = GetNativeType(value);
        if (type == NativeType::WebGLProgram) {
            auto program = WebGLProgram::GetPointer(value);
            if (program != nullptr) {
                auto ret = canvas_native_webgl_is_program(
                        program->GetProgram(),
                        ptr->GetState());

                return ret;
            }
        }

        return false;
    }

    static void IsRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static bool
    FastIsRenderbuffer(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return false;
        }


        auto type = GetNativeType(value);
        if (type == NativeType::WebGLRenderbuffer) {
            auto renderbuffer = WebGLRenderbuffer::GetPointer(value);
            if (renderbuffer != nullptr) {
                auto ret = canvas_native_webgl_is_renderbuffer(
                        renderbuffer->GetRenderBuffer(),
                        ptr->GetState());

                return ret;
            }
        }

        return false;
    }

    static void IsShader(const v8::FunctionCallbackInfo<v8::Value> &args);

    static bool FastIsShader(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return false;
        }

        auto type = GetNativeType(value);
        if (type == NativeType::WebGLShader) {
            auto shader = WebGLShader::GetPointer(value);
            if (shader != nullptr) {
                auto ret = canvas_native_webgl_is_shader(
                        shader->GetShader(),
                        ptr->GetState());

                return ret;
            }
        }

        return false;
    }

    static void IsTexture(const v8::FunctionCallbackInfo<v8::Value> &args);

    static bool FastIsTexture(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return false;
        }


        auto type = GetNativeType(value);
        if (type == NativeType::WebGLTexture) {
            auto texture = WebGLTexture::GetPointer(value);
            if (texture != nullptr) {
                auto ret = canvas_native_webgl_is_texture(
                        texture->GetTexture(),
                        ptr->GetState());
                return ret;
            }
        }

        return false;
    }

    static void LineWidth(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastLineWidth(v8::Local<v8::Object> receiver_obj, double width) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_line_width(
                static_cast<float>(width),
                ptr->GetState());

    }

    static void LinkProgram(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastLinkProgram(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(value);
        if (type == NativeType::WebGLProgram) {
            auto program = WebGLProgram::GetPointer(value);
            if (program != nullptr) {
                canvas_native_webgl_link_program(
                        program->GetProgram(),
                        ptr->GetState());
            }
        }
    }

    static void PixelStorei(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastPixelStoreiBool(v8::Local<v8::Object> receiver_obj, uint32_t pname, bool param) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_pixel_storei(
                pname,
                param ? 1 : 0,
                ptr->GetState()
        );
    }

    static void FastPixelStorei(v8::Local<v8::Object> receiver_obj, uint32_t pname, int32_t param) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_pixel_storei(
                pname,
                param,
                ptr->GetState()
        );
    }

    static void PolygonOffset(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastPolygonOffset(v8::Local<v8::Object> receiver_obj, double factor, double units) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_polygon_offset(
                static_cast<float>(factor),
                static_cast<float>(units),
                ptr->GetState()
        );

    }

    static void ReadPixels(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RenderbufferStorage(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastRenderbufferStorage(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                        uint32_t internalFormat, int32_t width, int32_t height) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_renderbuffer_storage(
                target,
                internalFormat,
                width,
                height,
                ptr->GetState()
        );

    }

    static void SampleCoverage(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastSampleCoverage(v8::Local<v8::Object> receiver_obj, double value, bool invert) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_sample_coverage(
                static_cast<float>(value),
                invert,
                ptr->GetState()
        );

    }

    static void Scissor(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastScissor(v8::Local<v8::Object> receiver_obj, int32_t x, int32_t y, int32_t width,
                            int32_t height) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_scissor(
                x,
                y,
                width,
                height,
                ptr->GetState()
        );

    }

    static void ShaderSource(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void StencilFuncSeparate(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastStencilFuncSeparate(v8::Local<v8::Object> receiver_obj, uint32_t face, uint32_t func,
                            int32_t ref, uint32_t mask) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_stencil_func_separate(
                face,
                func,
                ref,
                mask,
                ptr->GetState()
        );

    }

    static void StencilFunc(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastStencilFunc(v8::Local<v8::Object> receiver_obj, uint32_t func, int32_t ref, uint32_t mask) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_stencil_func(
                func,
                ref,
                mask,
                ptr->GetState()
        );

    }

    static void StencilMaskSeparate(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastStencilMaskSeparate(v8::Local<v8::Object> receiver_obj, uint32_t face, uint32_t mask) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_stencil_mask_separate(
                face,
                mask,
                ptr->GetState()
        );

    }

    static void StencilMask(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastStencilMask(v8::Local<v8::Object> receiver_obj, uint32_t mask) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        canvas_native_webgl_stencil_mask(
                mask,
                ptr->GetState()
        );

    }

    static void StencilOpSeparate(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastStencilOpSeparate(v8::Local<v8::Object> receiver_obj, uint32_t face, uint32_t fail,
                          uint32_t zfail, uint32_t zpass) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_stencil_op_separate(
                face,
                fail,
                zfail,
                zpass,
                ptr->GetState()
        );

    }

    static void StencilOp(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastStencilOp(v8::Local<v8::Object> receiver_obj, uint32_t fail, uint32_t zfail,
                              uint32_t zpass) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_stencil_op(
                fail,
                zfail,
                zpass,
                ptr->GetState()
        );

    }

    static void TexImage2D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void TexParameterf(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastTexParameterf(v8::Local<v8::Object> receiver_obj, uint32_t target, uint32_t pname,
                      double param) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_tex_parameterf(
                target,
                pname,
                static_cast<float>(param),
                ptr->GetState()
        );

    }

    static void TexParameteri(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastTexParameteri(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                  uint32_t pname, int32_t
                                  param) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_tex_parameteri(
                target,
                pname,
                param,
                ptr->GetState()
        );

    }

    static void TexSubImage2D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform1f(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform1fImpl(WebGLState *receiver_obj, int32_t location, double v0) {
        canvas_native_webgl_uniform1f(
                location,
                static_cast<float>(v0),
                receiver_obj
        );
    }

    static void
    FastUniform1f(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                  double v0) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            Uniform1fImpl(
                    ptr->GetState(),
                    location->GetUniformLocation(),
                    static_cast<float>(v0)
            );
        }
    }

    static void Uniform1iv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastUniform1iv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                   const v8::FastApiTypedArray<int32_t> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto size = value.length();
            int32_t *data;
            value.getStorageIfAligned(&data);

            canvas_native_webgl_uniform1iv(
                    location->GetUniformLocation(),
                    data, size,
                    ptr->GetState());
        }
    }

    static void
    FastUniform1ivArray(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                        v8::Local<v8::Array> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto len = value->Length();
            std::vector<int32_t> buf;
            buf.reserve(len);

            auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<int32_t>::Build().GetId(), int32_t>(
                    value, buf.data(), len);

            if (copied) {
                canvas_native_webgl_uniform1iv(
                        location->GetUniformLocation(),
                        buf.data(),
                        buf.size(),
                        ptr->GetState());
            }


        }
    }


    static void Uniform1fv(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastUniform1fv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                   const v8::FastApiTypedArray<float> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto size = value.length();
            float *data;
            value.getStorageIfAligned(&data);

            canvas_native_webgl_uniform1fv(
                    location->GetUniformLocation(),
                    data, size,
                    ptr->GetState());
        }
    }

    static void
    FastUniform1fvArray(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                        v8::Local<v8::Array> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto len = value->Length();
            std::vector<float> buf;
            buf.reserve(len);

            auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                    value, buf.data(), len);

            if (copied) {
                canvas_native_webgl_uniform1fv(
                        location->GetUniformLocation(),
                        buf.data(),
                        buf.size(),
                        ptr->GetState());
            }


        }
    }


    static void Uniform1i(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void Uniform1iImpl(WebGLState *receiver_obj, int32_t location, int32_t v0) {
        canvas_native_webgl_uniform1i(
                location,
                v0,
                receiver_obj
        );
    }

    static void
    FastUniform1i(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                  int32_t v0) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            Uniform1iImpl(
                    ptr->GetState(),
                    location->GetUniformLocation(),
                    v0
            );
        }
    }

    static void Uniform2f(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastUniform2f(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                  double v0, double v1) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            canvas_native_webgl_uniform2f(
                    location->GetUniformLocation(),
                    static_cast<float>(v0),
                    static_cast<float>(v1),
                    ptr->GetState()
            );
        }
    }


    static void Uniform2iv(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastUniform2iv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                   const v8::FastApiTypedArray<int32_t> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto size = value.length();
            int32_t *data;
            value.getStorageIfAligned(&data);

            canvas_native_webgl_uniform2iv(
                    location->GetUniformLocation(),
                    data, size,
                    ptr->GetState());
        }
    }

    static void
    FastUniform2ivArray(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                        v8::Local<v8::Array> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto len = value->Length();
            std::vector<int32_t> buf;
            buf.reserve(len);

            auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<int32_t>::Build().GetId(), int32_t>(
                    value, buf.data(), len);

            if (copied) {
                canvas_native_webgl_uniform2iv(
                        location->GetUniformLocation(),
                        buf.data(),
                        buf.size(),
                        ptr->GetState());
            }

        }
    }


    static void Uniform2fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastUniform2fv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                   const v8::FastApiTypedArray<float> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto size = value.length();
            float *data;
            value.getStorageIfAligned(&data);

            canvas_native_webgl_uniform2fv(
                    location->GetUniformLocation(),
                    data, size,
                    ptr->GetState());
        }
    }

    static void
    FastUniform2fvArray(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                        v8::Local<v8::Array> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto len = value->Length();
            std::vector<float> buf;
            buf.reserve(len);

            auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                    value, buf.data(), len);

            if (copied) {
                canvas_native_webgl_uniform2fv(
                        location->GetUniformLocation(),
                        buf.data(),
                        buf.size(),
                        ptr->GetState());
            }

        }
    }


    static void Uniform2i(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastUniform2i(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                  int32_t v0, int32_t v1) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            canvas_native_webgl_uniform2i(
                    location->GetUniformLocation(),
                    v0,
                    v1,
                    ptr->GetState()
            );
        }
    }


    static void Uniform3f(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastUniform3f(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                  double v0, double v1, double v2) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            canvas_native_webgl_uniform3f(
                    location->GetUniformLocation(),
                    static_cast<float>(v0),
                    static_cast<float>(v1),
                    static_cast<float>(v2),
                    ptr->GetState()
            );
        }
    }


    static void Uniform3iv(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastUniform3fv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                   const v8::FastApiTypedArray<float> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto size = value.length();
            float *data;
            value.getStorageIfAligned(&data);

            canvas_native_webgl_uniform3fv(
                    location->GetUniformLocation(),
                    data, size,
                    ptr->GetState());
        }
    }

    static void
    FastUniform3fvArray(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                        v8::Local<v8::Array> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto len = value->Length();
            std::vector<float> buf;
            buf.reserve(len);

            auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                    value, buf.data(), len);

            if (copied) {
                canvas_native_webgl_uniform3fv(
                        location->GetUniformLocation(),
                        buf.data(),
                        buf.size(),
                        ptr->GetState());
            }

        }
    }


    static void Uniform3fv(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastUniform3iv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                   const v8::FastApiTypedArray<int32_t> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto size = value.length();
            int32_t *data;
            value.getStorageIfAligned(&data);

            canvas_native_webgl_uniform3iv(
                    location->GetUniformLocation(),
                    data, size,
                    ptr->GetState());
        }
    }

    static void
    FastUniform3ivArray(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                        v8::Local<v8::Array> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto len = value->Length();
            std::vector<int32_t> buf;
            buf.reserve(len);

            auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<int32_t>::Build().GetId(), int32_t>(
                    value, buf.data(), len);

            if (copied) {
                canvas_native_webgl_uniform3iv(
                        location->GetUniformLocation(),
                        buf.data(),
                        buf.size(),
                        ptr->GetState());
            }

        }
    }


    static void Uniform3i(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastUniform3i(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                  int32_t v0, int32_t v1, int32_t v2) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            canvas_native_webgl_uniform3i(
                    location->GetUniformLocation(),
                    v0,
                    v1,
                    v2,
                    ptr->GetState()
            );
        }
    }

    static void Uniform4f(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastUniform4f(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                  double v0, double v1, double v2, double v3) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            canvas_native_webgl_uniform4f(
                    location->GetUniformLocation(),
                    static_cast<float>(v0),
                    static_cast<float>(v1),
                    static_cast<float>(v2),
                    static_cast<float>(v3),
                    ptr->GetState()
            );
        }
    }


    static void Uniform4iv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastUniform4iv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                   const v8::FastApiTypedArray<int32_t> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto size = value.length();
            int32_t *data;
            value.getStorageIfAligned(&data);

            canvas_native_webgl_uniform4iv(
                    location->GetUniformLocation(),
                    data, size,
                    ptr->GetState());
        }
    }

    static void
    FastUniform4ivArray(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                        v8::Local<v8::Array> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto len = value->Length();
            std::vector<int32_t> buf;
            buf.reserve(len);

            auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<int32_t>::Build().GetId(), int32_t>(
                    value, buf.data(), len);

            if (copied) {
                canvas_native_webgl_uniform4iv(
                        location->GetUniformLocation(),
                        buf.data(),
                        buf.size(),
                        ptr->GetState());
            }


        }
    }

    static void Uniform4fv(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastUniform4fv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                   const v8::FastApiTypedArray<float> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto size = value.length();
            float *data;
            value.getStorageIfAligned(&data);

            canvas_native_webgl_uniform4fv(
                    location->GetUniformLocation(),
                    data, size,
                    ptr->GetState());
        }
    }

    static void
    FastUniform4fvArray(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                        v8::Local<v8::Array> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto len = value->Length();
            std::vector<float> buf;
            buf.reserve(len);

            auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                    value, buf.data(), len);

            if (copied) {
                canvas_native_webgl_uniform4fv(
                        location->GetUniformLocation(),
                        buf.data(),
                        buf.size(),
                        ptr->GetState());
            }


        }
    }


    static void Uniform4i(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastUniform4i(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                  int32_t v0, int32_t v1, int32_t v2, int32_t v3) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            canvas_native_webgl_uniform4i(
                    location->GetUniformLocation(),
                    v0,
                    v1,
                    v2,
                    v3,
                    ptr->GetState()
            );
        }
    }


    static void UniformMatrix2fv(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastUniformMatrix2fv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                         bool transpose, const v8::FastApiTypedArray<float> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {

            float *data;

            value.getStorageIfAligned(&data);
            auto size = value.length();


            canvas_native_webgl_uniform_matrix2fv(
                    location->GetUniformLocation(),
                    transpose, data, size,
                    ptr->GetState());

        }

    }

    static void FastUniformMatrix2fvArray(v8::Local<v8::Object> receiver_obj,
                                          v8::Local<v8::Object> location_obj, bool transpose,
                                          v8::Local<v8::Array> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto len = value->Length();
            std::vector<float> buf;
            buf.reserve(len);

            auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                    value, buf.data(), len);

            if (copied) {
                canvas_native_webgl_uniform_matrix2fv(
                        location->GetUniformLocation(),
                        transpose, buf.data(),
                        buf.size(),
                        ptr->GetState());
            }

        }
    }


    static void UniformMatrix3fv(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastUniformMatrix3fv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                         bool transpose, const v8::FastApiTypedArray<float> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {

            float *data;

            value.getStorageIfAligned(&data);
            auto size = value.length();


            canvas_native_webgl_uniform_matrix3fv(
                    location->GetUniformLocation(),
                    transpose, data, size,
                    ptr->GetState());

        }

    }

    static void FastUniformMatrix3fvArray(v8::Local<v8::Object> receiver_obj,
                                          v8::Local<v8::Object> location_obj, bool transpose,
                                          v8::Local<v8::Array> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto len = value->Length();
            std::vector<float> buf;
            buf.reserve(len);

            auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                    value, buf.data(), len);

            if (copied) {
                canvas_native_webgl_uniform_matrix3fv(
                        location->GetUniformLocation(),
                        transpose, buf.data(),
                        buf.size(),
                        ptr->GetState());
            }

        }
    }


    static void UniformMatrix4fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastUniformMatrix4fv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                         bool transpose, const v8::FastApiTypedArray<float> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {

            float *data;
            value.getStorageIfAligned(&data);
            auto size = value.length();


            canvas_native_webgl_uniform_matrix4fv(
                    location->GetUniformLocation(),
                    transpose, data, size,
                    ptr->GetState());

        }

    }

    static void FastUniformMatrix4fvArray(v8::Local<v8::Object> receiver_obj,
                                          v8::Local<v8::Object> location_obj, bool transpose,
                                          v8::Local<v8::Array> value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto len = value->Length();
            std::vector<float> buf;
            buf.reserve(len);

            auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                    value, buf.data(), len);

            if (copied) {
                canvas_native_webgl_uniform_matrix4fv(
                        location->GetUniformLocation(),
                        transpose, buf.data(),
                        buf.size(),
                        ptr->GetState());
            }

        }
    }

    static void UseProgram(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    UseProgramImpl(WebGLRenderingContext *ptr, uint32_t program) {
        canvas_native_webgl_use_program(
                program,
                ptr->GetState()
        );

    }


    static void FastUseProgramNull(v8::Local<v8::Object> receiver_obj, uint32_t program) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        UseProgramImpl(
                ptr,
                program
        );
    }

    static void
    FastUseProgram(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> program_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        if (GetNativeType(program_obj) == NativeType::WebGLProgram) {
            WebGLProgram *program = WebGLProgram::GetPointer(program_obj);
            UseProgramImpl(
                    ptr,
                    program->GetProgram()
            );
        }


    }

    static void ValidateProgram(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastValidateProgram(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> program_obj) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto type = GetNativeType(program_obj);
        if (type == NativeType::WebGLProgram) {
            auto program = WebGLProgram::GetPointer(program_obj);
            if (program != nullptr) {
                canvas_native_webgl_validate_program(
                        program->GetProgram(),
                        ptr->GetState()
                );
            }
        }

    }

    static void VertexAttrib1f(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastVertexAttrib1f(v8::Local<v8::Object> receiver_obj, uint32_t index, float v0) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_vertex_attrib1f(
                index, v0, ptr->GetState());

    }

    static void VertexAttrib1fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastVertexAttrib1fv(v8::Local<v8::Object> receiver_obj, uint32_t index,
                                    const v8::FastApiTypedArray<float> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto size = value.length();
        float *data;
        value.getStorageIfAligned(&data);


        canvas_native_webgl_vertex_attrib1fv(
                index, data, size,
                ptr->GetState());
    }

    static void VertexAttrib2f(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastVertexAttrib2f(v8::Local<v8::Object> receiver_obj, uint32_t index, float v0, float v1) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_vertex_attrib2f(
                index, v0, v1, ptr->GetState());

    }

    static void VertexAttrib2fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastVertexAttrib2fv(v8::Local<v8::Object> receiver_obj, uint32_t index,
                                    const v8::FastApiTypedArray<float> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto size = value.length();
        float *data;
        value.getStorageIfAligned(&data);


        canvas_native_webgl_vertex_attrib2fv(
                index, data, size,
                ptr->GetState());
    }

    static void VertexAttrib3f(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastVertexAttrib3f(v8::Local<v8::Object> receiver_obj, uint32_t index, float v0, float v1,
                       float v2) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_vertex_attrib3f(
                index, v0, v1, v2, ptr->GetState());

    }

    static void VertexAttrib3fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastVertexAttrib3fv(v8::Local<v8::Object> receiver_obj, uint32_t index,
                                    const v8::FastApiTypedArray<float> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto size = value.length();
        float *data;
        value.getStorageIfAligned(&data);


        canvas_native_webgl_vertex_attrib3fv(
                index, data, size,
                ptr->GetState());
    }

    static void VertexAttrib4f(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastVertexAttrib4f(v8::Local<v8::Object> receiver_obj, uint32_t index, float v0, float v1,
                       float v2, float v3) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_vertex_attrib4f(
                index, v0, v1, v2, v3, ptr->GetState());

    }

    static void VertexAttrib4fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastVertexAttrib4fv(v8::Local<v8::Object> receiver_obj, uint32_t index,
                                    const v8::FastApiTypedArray<float> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto size = value.length();
        float *data;
        value.getStorageIfAligned(&data);


        canvas_native_webgl_vertex_attrib4fv(
                index, data, size,
                ptr->GetState());
    }

    static void VertexAttribPointer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastVertexAttribPointer(v8::Local<v8::Object> receiver_obj, uint32_t index, int32_t size,
                            uint32_t type, bool normalized, int32_t stride, int32_t offset) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto os = static_cast<ssize_t>(offset);
        canvas_native_webgl_vertex_attrib_pointer(
                index, size, type, normalized,
                stride, os,
                ptr->GetState());
    }


    static void Viewport(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    ViewportImpl(WebGLRenderingContext *ptr, double x, double y, double width, double height) {
        canvas_native_webgl_viewport(
                static_cast<float>(x), static_cast<float>(y), static_cast<float>(width),
                static_cast<float>(height),
                ptr->GetState()
        );

    }

    static void FastViewport(v8::Local<v8::Object> receiver_obj, double x, double y, double width,
                             double height) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        ViewportImpl(
                ptr,
                x, y, width, height
        );
    }

    static void __ToDataURL(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void __GetSupportedExtensions(const v8::FunctionCallbackInfo<v8::Value> &args);

};
