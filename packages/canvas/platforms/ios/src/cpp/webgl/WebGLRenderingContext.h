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

    static v8::CFunction fast_vertex_attrib_pointer;



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

    static void __StartRaf(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void __StopRaf(const v8::FunctionCallbackInfo<v8::Value> &args);

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

    static void BlendEquationSeparate(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BlendEquation(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BlendFuncSeparate(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BlendFunc(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BufferData(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BufferSubData(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CheckFramebufferStatus(const v8::FunctionCallbackInfo<v8::Value> &args);

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

    static void ClearStencil(const v8::FunctionCallbackInfo<v8::Value> &args);

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

    static void Commit(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CompileShader(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CompressedTexImage2D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CompressedTexSubImage2D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CopyTexImage2D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CopyTexSubImage2D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateProgram(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateShader(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateTexture(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CullFace(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DeleteBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DeleteFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DeleteProgram(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DeleteRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DeleteShader(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DeleteTexture(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DepthFunc(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DepthMask(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DepthRange(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DetachShader(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DisableVertexAttribArray(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Disable(const v8::FunctionCallbackInfo<v8::Value> &args);

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


        DrawArraysImpl(
                ptr,
                mode,
                first,
                count
        );
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

    static void Flush(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FramebufferRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FramebufferTexture2D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FrontFace(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GenerateMipmap(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetActiveAttrib(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetActiveUniform(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetAttachedShaders(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetAttribLocation(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetBufferParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

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

    static void GetVertexAttrib(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Hint(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsContextLost(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsEnabled(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsProgram(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsShader(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsTexture(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void LineWidth(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void LinkProgram(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void PixelStorei(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void PolygonOffset(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadPixels(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RenderbufferStorage(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SampleCoverage(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Scissor(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ShaderSource(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void StencilFuncSeparate(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void StencilFunc(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void StencilMaskSeparate(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void StencilMask(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void StencilOpSeparate(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void StencilOp(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void TexImage2D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void TexParameterf(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void TexParameteri(const v8::FunctionCallbackInfo<v8::Value> &args);

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

    static void FastVertexAttrib1fv(v8::Local<v8::Object> receiver_obj, uint32_t index, const v8::FastApiTypedArray<float> &value) {
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

    static void FastVertexAttrib2fv(v8::Local<v8::Object> receiver_obj, uint32_t index, const v8::FastApiTypedArray<float> &value) {
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

    static void FastVertexAttrib3fv(v8::Local<v8::Object> receiver_obj, uint32_t index, const v8::FastApiTypedArray<float> &value) {
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

    static void FastVertexAttrib4fv(v8::Local<v8::Object> receiver_obj, uint32_t index, const v8::FastApiTypedArray<float> &value) {
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
