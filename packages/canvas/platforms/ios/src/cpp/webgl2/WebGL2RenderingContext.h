//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once
#pragma process_pending_includes

#include <vector>

#include "CanvasRenderingContext2DImpl.h"
#include "RafImpl.h"
#include <cmath>
#include "Helpers.h"
#include "WebGLRenderingContextBase.h"
#include "WebGLRenderingContext.h"
#include "WebGLQuery.h"
#include "WebGLSampler.h"
#include "WebGLSyncImpl.h"
#include "WebGLTransformFeedback.h"
#include "WebGLVertexArrayObject.h"
#include "gl.h"


class WebGL2RenderingContext : public WebGLRenderingContext {
public:

    WebGL2RenderingContext(WebGLState *state);

    WebGL2RenderingContext(WebGLState *state, WebGLRenderingVersion version);

    static v8::CFunction fast_begin_query_;
    static v8::CFunction fast_begin_transform_feedback_;
    static v8::CFunction fast_bind_vertex_array_;
    static v8::CFunction fast_bind_buffer_base_;
    static v8::CFunction fast_bind_buffer_range_;
    static v8::CFunction fast_bind_sampler_;
    static v8::CFunction fast_bind_transform_feedback_;
    static v8::CFunction fast_blit_framebuffer_;
    static v8::CFunction fast_clear_buffer_fi_;
    static v8::CFunction fast_clear_buffer_fv_;
    static v8::CFunction fast_clear_buffer_fv_array_;
    static v8::CFunction fast_clear_buffer_iv_;
    static v8::CFunction fast_clear_buffer_iv_array_;
    static v8::CFunction fast_clear_buffer_uiv_;
    static v8::CFunction fast_clear_buffer_uiv_array_;
    static v8::CFunction fast_draw_arrays_instanced_;
    static v8::CFunction fast_draw_buffers_;
    static v8::CFunction fast_draw_elements_instanced_;
    static v8::CFunction fast_draw_range_elements_;
    static v8::CFunction fast_resume_transform_feedback_;
    static v8::CFunction fast_sampler_parameterf_;
    static v8::CFunction fast_sampler_parameteri_;
    static v8::CFunction fast_uniform_1ui_;
    static v8::CFunction fast_uniform_1uiv_;
    static v8::CFunction fast_uniform_2ui_;
    static v8::CFunction fast_uniform_2uiv_;
    static v8::CFunction fast_uniform_3ui_;
    static v8::CFunction fast_uniform_3uiv_;
    static v8::CFunction fast_uniform_4ui_;
    static v8::CFunction fast_uniform_4uiv_;
    static v8::CFunction fast_uniform_matrix_2x3fv_;
    static v8::CFunction fast_uniform_matrix_2x3fv_array_;
    static v8::CFunction fast_uniform_matrix_2x4fv_;
    static v8::CFunction fast_uniform_matrix_2x4fv_array_;
    static v8::CFunction fast_uniform_matrix_3x2fv_;
    static v8::CFunction fast_uniform_matrix_3x2fv_array_;
    static v8::CFunction fast_uniform_matrix_3x4fv_;
    static v8::CFunction fast_uniform_matrix_3x4fv_array_;
    static v8::CFunction fast_uniform_matrix_4x2fv_;
    static v8::CFunction fast_uniform_matrix_4x2fv_array_;
    static v8::CFunction fast_uniform_matrix_4x3fv_;
    static v8::CFunction fast_uniform_matrix_4x3fv_array_;
    static v8::CFunction fast_vertex_attrib_divisor_;
    static v8::CFunction fast_vertex_attrib_i_4i_;
    static v8::CFunction fast_vertex_attrib_i_4iv_;
    static v8::CFunction fast_vertex_attrib_i_4iv_array_;
    static v8::CFunction fast_vertex_attrib_i_4ui_;
    static v8::CFunction fast_vertex_attrib_i_4uiv_;
    static v8::CFunction fast_vertex_attrib_i_4uiv_array_;
    static v8::CFunction fast_uniform_block_binding_;

    static v8::CFunction fast_invalidate_framebuffer_;
    static v8::CFunction fast_invalidate_sub_framebuffer_;


    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, WebGL2RenderingContext *renderingContext) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WebGL2RenderingContext::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::WebGLRenderingContextBase);
        object->SetAlignedPointerInInternalField(0, renderingContext);
        renderingContext->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WebGL2RenderingContext *GetPointer(const v8::Local<v8::Object> &object);

    static void SetConstants(v8::Isolate *isolate, const v8::Local<v8::ObjectTemplate> &tmpl);

    static void
    SetProps(v8::Isolate *isolate, const v8::Local<v8::ObjectTemplate> &webgl2RenderingContextTpl);

    static void SetMethods(v8::Isolate *isolate,
                           const v8::Local<v8::ObjectTemplate> &tmpl);

    static void BeginQuery(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastBeginQuery(v8::Local<v8::Object> receiver_obj, uint32_t target,
                               v8::Local<v8::Object> query_obj) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(query_obj);
        if (type == NativeType::WebGLQuery) {
            auto query = WebGLQuery::GetPointer(query_obj);

            if (query != nullptr) {
                canvas_native_webgl2_begin_query(
                        target,
                        query->GetQuery(),
                        ptr->GetState()
                );
            }
        }
    }

    static void BeginTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastBeginTransformFeedback(v8::Local<v8::Object> receiver_obj, uint32_t value) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl2_begin_transform_feedback(
                value,
                ptr->GetState()
        );
    }

    static void BindBufferBase(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastBindBufferBase(v8::Local<v8::Object> receiver_obj, uint32_t target, uint32_t index,
                       v8::Local<v8::Object> buffer_obj) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(buffer_obj);

        if (type == NativeType::WebGLBuffer) {
            auto buffer = WebGLBuffer::GetPointer(buffer_obj);

            canvas_native_webgl2_bind_buffer_base(
                    target,
                    index,
                    buffer->GetBuffer(),
                    ptr->GetState()
            );
        }
    }

    static void BindBufferRange(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastBindBufferRange(v8::Local<v8::Object> receiver_obj, uint32_t target, uint32_t index,
                        v8::Local<v8::Object> buffer_obj, int32_t offset, int32_t size) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto type = GetNativeType(buffer_obj);
        if (type == NativeType::WebGLBuffer) {
            auto buffer = WebGLBuffer::GetPointer(buffer_obj);
            canvas_native_webgl2_bind_buffer_range(
                    target,
                    index,
                    buffer->GetBuffer(),
                    static_cast<ssize_t>(offset),
                    static_cast<ssize_t>(size),
                    ptr->GetState()
            );
        }
    }

    static void BindSampler(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastBindSampler(v8::Local<v8::Object> receiver_obj, uint32_t unit,
                                v8::Local<v8::Object> sampler_obj) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(sampler_obj);
        if (type == NativeType::WebGLSampler) {
            auto sampler = WebGLSampler::GetPointer(sampler_obj);

            canvas_native_webgl2_bind_sampler(
                    unit,
                    sampler->GetSampler(),
                    ptr->GetState()
            );
        }
    }

    static void BindTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastBindTransformFeedback(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                          v8::Local<v8::Object> transformer_obj) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto type = GetNativeType(transformer_obj);

        if (type == NativeType::WebGLTransformFeedback) {
            auto transformFeedback = WebGLTransformFeedback::GetPointer(transformer_obj);

            canvas_native_webgl2_bind_transform_feedback(
                    target,
                    transformFeedback->GetFeedback(),
                    ptr->GetState()
            );
        }
    }

    static void BindVertexArray(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    BindVertexArrayImpl(WebGL2RenderingContext *ptr, uint32_t vertex_array) {
        canvas_native_webgl2_bind_vertex_array(
                vertex_array,
                ptr->GetState()
        );

    }

    static void FastBindVertexArray(v8::Local<v8::Object> receiver_obj, uint32_t vertex_array) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        BindVertexArrayImpl(
                ptr,
                vertex_array
        );
    }

    static void BlitFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastBlitFramebuffer(v8::Local<v8::Object> receiver_obj, int32_t srcX0, int32_t srcY0,
                        int32_t srcX1, int32_t srcY1, int32_t dstX0, int32_t dstY0, int32_t dstX1,
                        int32_t dstY1, uint32_t mask, uint32_t filter) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        canvas_native_webgl2_blit_framebuffer(
                srcX0,
                srcY0,
                srcX1,
                srcY1,
                dstX0,
                dstY0,
                dstX1,
                dstY1,
                mask,
                filter,
                ptr->GetState()
        );
    }

    static void ClearBufferfi(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastClearBufferfi(v8::Local<v8::Object> receiver_obj, uint32_t buffer, int32_t drawbuffer,
                      double depth, int32_t stencil) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        canvas_native_webgl2_clear_bufferfi(
                buffer,
                drawbuffer,
                static_cast<float>(depth),
                stencil,
                ptr->GetState()
        );
    }

    static void ClearBufferfv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastClearBufferfv(v8::Local<v8::Object> receiver_obj, uint32_t buffer, int32_t drawbuffer,
                      const v8::FastApiTypedArray<float> &values) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto size = values.length();
        float *data;
        values.getStorageIfAligned(&data);


        canvas_native_webgl2_clear_bufferfv(
                buffer,
                drawbuffer,
                data,
                size,
                ptr->GetState()
        );
    }

    static void
    FastClearBufferfvArray(v8::Local<v8::Object> receiver_obj, uint32_t buffer, int32_t drawbuffer,
                           v8::Local<v8::Array> values) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto len = values->Length();
        std::vector<float> buf;
        buf.reserve(len);

        auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                values, buf.data(), len);


        if (copied) {
            canvas_native_webgl2_clear_bufferfv(
                    buffer,
                    drawbuffer,
                    buf.data(),
                    buf.size(),
                    ptr->GetState()
            );
        }
    }

    static void ClearBufferiv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastClearBufferiv(v8::Local<v8::Object> receiver_obj, uint32_t buffer, int32_t drawbuffer,
                      const v8::FastApiTypedArray<int32_t> &values) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto size = values.length();
        int32_t *data;
        values.getStorageIfAligned(&data);


        canvas_native_webgl2_clear_bufferiv(
                buffer,
                drawbuffer,
                data,
                size,
                ptr->GetState()
        );
    }

    static void
    FastClearBufferivArray(v8::Local<v8::Object> receiver_obj, uint32_t buffer, int32_t drawbuffer,
                           v8::Local<v8::Array> values) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto len = values->Length();
        std::vector<int32_t> buf;
        buf.reserve(len);

        auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<int32_t>::Build().GetId(), int32_t>(
                values, buf.data(), len);


        if (copied) {
            canvas_native_webgl2_clear_bufferiv(
                    buffer,
                    drawbuffer,
                    buf.data(),
                    buf.size(),
                    ptr->GetState()
            );
        }
    }


    static void ClearBufferuiv(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastClearBufferuiv(v8::Local<v8::Object> receiver_obj, uint32_t buffer, int32_t drawbuffer,
                       const v8::FastApiTypedArray<uint32_t> &values) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto size = values.length();
        uint32_t *data;
        values.getStorageIfAligned(&data);


        canvas_native_webgl2_clear_bufferuiv(
                buffer,
                drawbuffer,
                data,
                size,
                ptr->GetState()
        );
    }

    static void
    FastClearBufferuivArray(v8::Local<v8::Object> receiver_obj, uint32_t buffer, int32_t drawbuffer,
                            v8::Local<v8::Array> values) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto len = values->Length();
        std::vector<uint32_t> buf;
        buf.reserve(len);

        auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<uint32_t>::Build().GetId(), uint32_t>(
                values, buf.data(), len);


        if (copied) {
            canvas_native_webgl2_clear_bufferuiv(
                    buffer,
                    drawbuffer,
                    buf.data(),
                    buf.size(),
                    ptr->GetState()
            );
        }
    }

    static void ClientWaitSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CompressedTexSubImage3D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CopyBufferSubData(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CopyTexSubImage3D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateQuery(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateSampler(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateVertexArray(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DeleteQuery(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DeleteSampler(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DeleteSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DeleteTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DeleteVertexArray(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawArraysInstanced(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastDrawArraysInstanced(v8::Local<v8::Object> receiver_obj, uint32_t mode, int32_t first,
                            int32_t count, int32_t instanceCount) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        canvas_native_webgl2_draw_arrays_instanced(
                mode,
                first,
                count,
                instanceCount,
                ptr->GetState()
        );

        ptr->UpdateInvalidateState();
    }

    static void DrawBuffers(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastDrawBuffers(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Array> buffers) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto len = buffers->Length();
        std::vector<uint32_t> buf;
        buf.reserve(len);

        auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<uint32_t>::Build().GetId(), uint32_t>(
                buffers, buf.data(), len);


        if (copied) {
            canvas_native_webgl2_draw_buffers(
                    buf.data(), buf.size(),
                    ptr->GetState());
        }

    }

    static void DrawElementsInstanced(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastDrawElementsInstanced(v8::Local<v8::Object> receiver_obj, uint32_t mode, int32_t count,
                              uint32_t type, int32_t offset, int32_t instanceCount) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        canvas_native_webgl2_draw_elements_instanced(
                mode,
                count,
                type,
                static_cast<ssize_t>(offset),
                instanceCount,
                ptr->GetState()
        );

        ptr->UpdateInvalidateState();
    }

    static void DrawRangeElements(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastDrawRangeElements(v8::Local<v8::Object> receiver_obj, uint32_t mode, uint32_t start,
                          uint32_t end, int32_t count, uint32_t type, int32_t offset) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        canvas_native_webgl2_draw_range_elements(
                mode,
                start,
                end,
                count,
                type,
                static_cast<ssize_t>(offset),
                ptr->GetState()
        );

        ptr->UpdateInvalidateState();
    }

    static void EndQuery(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void EndTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FenceSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FramebufferTextureLayer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetActiveUniformBlockName(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetActiveUniformBlockParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetActiveUniforms(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetBufferSubData(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetFragDataLocation(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetIndexedParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetInternalformatParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetQueryParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetQuery(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetSamplerParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetSyncParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetTransformFeedbackVarying(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetUniformBlockIndex(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetUniformIndices(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void InvalidateFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastInvalidateFramebuffer(v8::Local<v8::Object> receiver_obj, uint32_t target,
                              v8::Local<v8::Array> attachments) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto len = attachments->Length();
        std::vector<uint32_t> buf;
        buf.reserve(len);

        auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<uint32_t>::Build().GetId(), uint32_t>(
                attachments, buf.data(), len);

        if (copied) {
            canvas_native_webgl2_invalidate_framebuffer(
                    target, buf.data(), buf.size(),
                    ptr->GetState());
        }
    }

    static void InvalidateSubFramebuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastInvalidateSubFramebuffer(v8::Local<v8::Object> receiver_obj, uint32_t target,
                                             v8::Local<v8::Array> attachments, int32_t x, int32_t y,
                                             int32_t width, int32_t height) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto len = attachments->Length();
        std::vector<uint32_t> buf;
        buf.reserve(len);

        auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<uint32_t>::Build().GetId(), uint32_t>(
                attachments, buf.data(), len);

        if (copied) {

            canvas_native_webgl2_invalidate_sub_framebuffer(
                    target,
                    buf.data(), buf.size(),
                    x,
                    y,
                    width,
                    height,
                    ptr->GetState());
        }
    }

    static void IsQuery(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsSampler(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsVertexArray(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void PauseTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RenderbufferStorageMultisample(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ResumeTransformFeedback(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastResumeTransformFeedback(v8::Local<v8::Object> receiver_obj) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl2_resume_transform_feedback(
                ptr->GetState());
    }

    static void SamplerParameterf(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastSamplerParameterf(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> sampler_obj,
                          uint32_t pname, double param) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(sampler_obj);
        if (type == NativeType::WebGLSampler) {
            auto sampler = WebGLSampler::GetPointer(sampler_obj);
            if (sampler != nullptr) {
                canvas_native_webgl2_sampler_parameterf(
                        sampler->GetSampler(),
                        pname,
                        static_cast<float>(param),
                        ptr->GetState());
            }
        }

    }

    static void SamplerParameteri(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastSamplerParameteri(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> sampler_obj,
                          uint32_t pname, int32_t param) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(sampler_obj);
        if (type == NativeType::WebGLSampler) {
            auto sampler = WebGLSampler::GetPointer(sampler_obj);
            if (sampler != nullptr) {
                canvas_native_webgl2_sampler_parameteri(
                        sampler->GetSampler(),
                        pname,
                        param,
                        ptr->GetState());
            }
        }

    }

    static void TexImage3D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void TexStorage2D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void TexStorage3D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void TexSubImage3D(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void TransformFeedbackVaryings(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform1ui(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastUniform1ui(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                   uint32_t v0) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto type = GetNativeType(location_obj);

        if (type == NativeType::WebGLUniformLocation) {

            auto location = WebGLUniformLocation::GetPointer(location_obj);

            if (location != nullptr) {
                canvas_native_webgl2_uniform1ui(
                        location->GetUniformLocation(),
                        v0,
                        ptr->GetState()
                );
            }
        }

    }

    static void Uniform1uiv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastUniform1uiv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                    const v8::FastApiTypedArray<uint32_t> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto size = value.length();
            uint32_t *data;
            value.getStorageIfAligned(&data);

            canvas_native_webgl2_uniform1uiv(
                    location->GetUniformLocation(),
                    data, size,
                    ptr->GetState());
        }
    }

    static void Uniform2ui(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastUniform2ui(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                   uint32_t v0, uint32_t v1) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto type = GetNativeType(location_obj);

        if (type == NativeType::WebGLUniformLocation) {

            auto location = WebGLUniformLocation::GetPointer(location_obj);

            if (location != nullptr) {
                canvas_native_webgl2_uniform2ui(
                        location->GetUniformLocation(),
                        v0, v1,
                        ptr->GetState()
                );
            }
        }

    }

    static void Uniform2uiv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastUniform2uiv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                    const v8::FastApiTypedArray<uint32_t> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto size = value.length();
            uint32_t *data;
            value.getStorageIfAligned(&data);

            canvas_native_webgl2_uniform2uiv(
                    location->GetUniformLocation(),
                    data, size,
                    ptr->GetState());
        }
    }

    static void Uniform3ui(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastUniform3ui(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                   uint32_t v0, uint32_t v1, uint32_t v2) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto type = GetNativeType(location_obj);

        if (type == NativeType::WebGLUniformLocation) {

            auto location = WebGLUniformLocation::GetPointer(location_obj);

            if (location != nullptr) {
                canvas_native_webgl2_uniform3ui(
                        location->GetUniformLocation(),
                        v0, v1, v2,
                        ptr->GetState()
                );
            }
        }

    }

    static void Uniform3uiv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastUniform3uiv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                    const v8::FastApiTypedArray<uint32_t> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto size = value.length();
            uint32_t *data;
            value.getStorageIfAligned(&data);

            canvas_native_webgl2_uniform3uiv(
                    location->GetUniformLocation(),
                    data, size,
                    ptr->GetState());
        }
    }

    static void Uniform4ui(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastUniform4ui(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                   uint32_t v0, uint32_t v1, uint32_t v2, uint32_t v3) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto type = GetNativeType(location_obj);

        if (type == NativeType::WebGLUniformLocation) {

            auto location = WebGLUniformLocation::GetPointer(location_obj);

            if (location != nullptr) {
                canvas_native_webgl2_uniform4ui(
                        location->GetUniformLocation(),
                        v0, v1, v2, v3,
                        ptr->GetState()
                );
            }
        }

    }

    static void Uniform4uiv(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastUniform4uiv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                    const v8::FastApiTypedArray<uint32_t> &value) {
        WebGLRenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto location = WebGLUniformLocation::GetPointer(location_obj);

        if (location != nullptr) {
            auto size = value.length();
            uint32_t *data;
            value.getStorageIfAligned(&data);

            canvas_native_webgl2_uniform4uiv(
                    location->GetUniformLocation(),
                    data, size,
                    ptr->GetState());
        }
    }

    static void UniformBlockBinding(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastUniformBlockBinding(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> program_obj,
                            uint32_t uniformBlockIndex, uint32_t uniformBlockBinding) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto type = GetNativeType(program_obj);
        if (type == NativeType::WebGLProgram) {

            auto program = WebGLProgram::GetPointer(program_obj);

            if (program != nullptr) {
                canvas_native_webgl2_uniform_block_binding(
                        program->GetProgram(),
                        uniformBlockIndex,
                        uniformBlockBinding,
                        ptr->GetState()
                );
            }

        }

    }

    static void UniformMatrix2x3fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastUniformMatrix2x3fv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                           bool transpose, const v8::FastApiTypedArray<float> &data_value) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(location_obj);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(location_obj);
            auto size = data_value.length();
            float *data;
            data_value.getStorageIfAligned(&data);

            canvas_native_webgl2_uniform_matrix2x3fv(
                    location->GetUniformLocation(),
                    transpose,
                    data, size,
                    ptr->GetState()
            );
        }

    }

    static void
    FastUniformMatrix2x3fvArray(v8::Local<v8::Object> receiver_obj,
                                v8::Local<v8::Object> location_obj,
                                bool transpose, v8::Local<v8::Array> data_value) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(location_obj);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(location_obj);

            auto len = data_value->Length();
            std::vector<float> buf;
            buf.reserve(len);

            auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                    data_value, buf.data(), len);

            if (copied) {
                canvas_native_webgl2_uniform_matrix2x3fv(
                        location->GetUniformLocation(),
                        transpose,
                        buf.data(), buf.size(),
                        ptr->GetState()
                );
            }
        }

    }

    static void UniformMatrix2x4fv(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastUniformMatrix2x4fv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                           bool transpose, const v8::FastApiTypedArray<float> &data_value) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(location_obj);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(location_obj);
            auto size = data_value.length();
            float *data;
            data_value.getStorageIfAligned(&data);

            canvas_native_webgl2_uniform_matrix2x4fv(
                    location->GetUniformLocation(),
                    transpose,
                    data, size,
                    ptr->GetState()
            );
        }

    }

    static void
    FastUniformMatrix2x4fvArray(v8::Local<v8::Object> receiver_obj,
                                v8::Local<v8::Object> location_obj,
                                bool transpose, v8::Local<v8::Array> data_value) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(location_obj);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(location_obj);

            auto len = data_value->Length();
            std::vector<float> buf;
            buf.reserve(len);

            auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                    data_value, buf.data(), len);

            if (copied) {
                canvas_native_webgl2_uniform_matrix2x4fv(
                        location->GetUniformLocation(),
                        transpose,
                        buf.data(), buf.size(),
                        ptr->GetState()
                );
            }
        }

    }

    static void UniformMatrix3x2fv(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastUniformMatrix3x2fv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                           bool transpose, const v8::FastApiTypedArray<float> &data_value) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(location_obj);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(location_obj);
            auto size = data_value.length();
            float *data;
            data_value.getStorageIfAligned(&data);

            canvas_native_webgl2_uniform_matrix3x2fv(
                    location->GetUniformLocation(),
                    transpose,
                    data, size,
                    ptr->GetState()
            );
        }

    }

    static void
    FastUniformMatrix3x2fvArray(v8::Local<v8::Object> receiver_obj,
                                v8::Local<v8::Object> location_obj,
                                bool transpose, v8::Local<v8::Array> data_value) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(location_obj);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(location_obj);

            auto len = data_value->Length();
            std::vector<float> buf;
            buf.reserve(len);

            auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                    data_value, buf.data(), len);

            if (copied) {
                canvas_native_webgl2_uniform_matrix3x2fv(
                        location->GetUniformLocation(),
                        transpose,
                        buf.data(), buf.size(),
                        ptr->GetState()
                );
            }
        }

    }

    static void UniformMatrix3x4fv(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastUniformMatrix3x4fv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                           bool transpose, const v8::FastApiTypedArray<float> &data_value) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(location_obj);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(location_obj);
            auto size = data_value.length();
            float *data;
            data_value.getStorageIfAligned(&data);

            canvas_native_webgl2_uniform_matrix3x4fv(
                    location->GetUniformLocation(),
                    transpose,
                    data, size,
                    ptr->GetState()
            );
        }

    }

    static void
    FastUniformMatrix3x4fvArray(v8::Local<v8::Object> receiver_obj,
                                v8::Local<v8::Object> location_obj,
                                bool transpose, v8::Local<v8::Array> data_value) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(location_obj);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(location_obj);

            auto len = data_value->Length();
            std::vector<float> buf;
            buf.reserve(len);

            auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                    data_value, buf.data(), len);

            if (copied) {
                canvas_native_webgl2_uniform_matrix3x4fv(
                        location->GetUniformLocation(),
                        transpose,
                        buf.data(), buf.size(),
                        ptr->GetState()
                );
            }
        }

    }


    static void UniformMatrix4x2fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastUniformMatrix4x2fv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                           bool transpose, const v8::FastApiTypedArray<float> &data_value) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(location_obj);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(location_obj);
            auto size = data_value.length();
            float *data;
            data_value.getStorageIfAligned(&data);

            canvas_native_webgl2_uniform_matrix4x2fv(
                    location->GetUniformLocation(),
                    transpose,
                    data, size,
                    ptr->GetState()
            );
        }

    }

    static void
    FastUniformMatrix4x2fvArray(v8::Local<v8::Object> receiver_obj,
                                v8::Local<v8::Object> location_obj,
                                bool transpose, v8::Local<v8::Array> data_value) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(location_obj);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(location_obj);

            auto len = data_value->Length();
            std::vector<float> buf;
            buf.reserve(len);

            auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                    data_value, buf.data(), len);

            if (copied) {
                canvas_native_webgl2_uniform_matrix4x2fv(
                        location->GetUniformLocation(),
                        transpose,
                        buf.data(), buf.size(),
                        ptr->GetState()
                );
            }
        }

    }

    static void UniformMatrix4x3fv(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastUniformMatrix4x3fv(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> location_obj,
                           bool transpose, const v8::FastApiTypedArray<float> &data_value) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(location_obj);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(location_obj);
            auto size = data_value.length();
            float *data;
            data_value.getStorageIfAligned(&data);

            canvas_native_webgl2_uniform_matrix4x3fv(
                    location->GetUniformLocation(),
                    transpose,
                    data, size,
                    ptr->GetState()
            );
        }

    }

    static void
    FastUniformMatrix4x3fvArray(v8::Local<v8::Object> receiver_obj,
                                v8::Local<v8::Object> location_obj,
                                bool transpose, v8::Local<v8::Array> data_value) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(location_obj);
        if (type == NativeType::WebGLUniformLocation) {
            auto location = WebGLUniformLocation::GetPointer(location_obj);

            auto len = data_value->Length();
            std::vector<float> buf;
            buf.reserve(len);

            auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                    data_value, buf.data(), len);

            if (copied) {
                canvas_native_webgl2_uniform_matrix4x3fv(
                        location->GetUniformLocation(),
                        transpose,
                        buf.data(), buf.size(),
                        ptr->GetState()
                );
            }
        }

    }

    static void VertexAttribDivisor(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastVertexAttribDivisor(v8::Local<v8::Object> receiver_obj, uint32_t index, uint32_t divisor) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl2_vertex_attrib_divisor(
                index,
                divisor,
                ptr->GetState()
        );

    }

    static void VertexAttribI4i(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastVertexAttribI4i(v8::Local<v8::Object> receiver_obj, int32_t index, int32_t v0, int32_t v1,
                        int32_t v2, int32_t v3) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        canvas_native_webgl2_vertex_attrib_i4i(
                index,
                v0,
                v1,
                v2,
                v3,
                ptr->GetState()
        );

    }

    static void VertexAttribI4iv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastVertexAttribI4iv(v8::Local<v8::Object> receiver_obj, uint32_t index,
                                     const v8::FastApiTypedArray<int32_t> &data_value) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto size = data_value.length();
        int32_t *data;
        data_value.getStorageIfAligned(&data);

        canvas_native_webgl2_vertex_attrib_i4iv(
                index,
                data, size,
                ptr->GetState()
        );

    }

    static void FastVertexAttribI4ivArray(v8::Local<v8::Object> receiver_obj, uint32_t index,
                                          v8::Local<v8::Array> data_value) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto len = data_value->Length();
        std::vector<int32_t> buf;
        buf.reserve(len);

        auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<int32_t>::Build().GetId(), int32_t>(
                data_value, buf.data(), len);


        if (copied) {
            canvas_native_webgl2_vertex_attrib_i4iv(
                    index,
                    buf.data(), buf.size(),
                    ptr->GetState()
            );
        }

    }

    static void VertexAttribI4ui(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastVertexAttribI4ui(v8::Local<v8::Object> receiver_obj, uint32_t index, uint32_t v0,
                         uint32_t v1,
                         uint32_t v2, uint32_t v3) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        canvas_native_webgl2_vertex_attrib_i4ui(
                index,
                v0,
                v1,
                v2,
                v3,
                ptr->GetState()
        );

    }

    static void VertexAttribI4uiv(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void FastVertexAttribI4uiv(v8::Local<v8::Object> receiver_obj, uint32_t index,
                                      const v8::FastApiTypedArray<uint32_t> &data_value) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto size = data_value.length();
        uint32_t *data;
        data_value.getStorageIfAligned(&data);

        canvas_native_webgl2_vertex_attrib_i4uiv(
                index,
                data, size,
                ptr->GetState()
        );

    }

    static void FastVertexAttribI4uivArray(v8::Local<v8::Object> receiver_obj, uint32_t index,
                                           v8::Local<v8::Array> data_value) {
        WebGL2RenderingContext *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto len = data_value->Length();
        std::vector<uint32_t> buf;
        buf.reserve(len);

        auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<uint32_t>::Build().GetId(), uint32_t>(
                data_value, buf.data(), len);


        if (copied) {
            canvas_native_webgl2_vertex_attrib_i4uiv(
                    index,
                    buf.data(), buf.size(),
                    ptr->GetState()
            );
        }

    }

};
