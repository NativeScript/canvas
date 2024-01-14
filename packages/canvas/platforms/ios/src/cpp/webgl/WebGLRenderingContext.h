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

    static v8::CFunction fast_uniform1f_;

    static v8::CFunction fast_uniform1i_;

    static v8::CFunction fast_clear_;

    static v8::CFunction fast_clear_color_;

    static v8::CFunction fast_viewport_;

    static v8::CFunction fast_enable_;

    static v8::CFunction fast_use_program_;

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


    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, WebGLRenderingContext *renderingContext) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WebGLRenderingContext::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::WebGLRenderingContextBase);
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

    static void AttachShader(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BindAttribLocation(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BindBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BindFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BindRenderbuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BindTexture(const v8::FunctionCallbackInfo<v8::Value> &args);

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

    static void EnableVertexAttribArray(const v8::FunctionCallbackInfo<v8::Value> &args);

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
    FastUniform1fImpl(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
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
                    value, nullptr, len);

            canvas_native_webgl_uniform1fv(
                    location->GetUniformLocation(),
                    buf.data(),
                    buf.size(),
                    ptr->GetState());

        }
    }


    static void Uniform1i(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void Uniform1iImpl(WebGLState *receiver_obj, int32_t location, int32_t v0) {
        canvas_native_webgl_uniform1i(
                location,
                static_cast<float>(v0),
                receiver_obj
        );
    }

    static void
    FastUniform1iImpl(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
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

    static void Uniform2iv(const v8::FunctionCallbackInfo<v8::Value> &args);

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
                    value, nullptr, len);

            canvas_native_webgl_uniform2fv(
                    location->GetUniformLocation(),
                    buf.data(),
                    buf.size(),
                    ptr->GetState());

        }
    }


    static void Uniform2i(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform3f(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform3iv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform3fv(const v8::FunctionCallbackInfo<v8::Value> &args);


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
                    value, nullptr, len);

            canvas_native_webgl_uniform3fv(
                    location->GetUniformLocation(),
                    buf.data(),
                    buf.size(),
                    ptr->GetState());

        }
    }


    static void Uniform3i(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform4f(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform4iv(const v8::FunctionCallbackInfo<v8::Value> &args);

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
                    value, nullptr, len);

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
                    value, nullptr, len);

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
                    value, nullptr, len);

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
                    value, nullptr, len);

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

    static void FastUseProgram(v8::Local<v8::Object> receiver_obj, uint32_t program) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        UseProgramImpl(
                ptr,
                program
        );
    }

    static void ValidateProgram(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void VertexAttrib1f(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void VertexAttrib1fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void VertexAttrib2f(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void VertexAttrib2fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void VertexAttrib3f(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void VertexAttrib3fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void VertexAttrib4f(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void VertexAttrib4fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void VertexAttribPointer(const v8::FunctionCallbackInfo<v8::Value> &args);

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
