//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once
#pragma process_pending_includes
#include "rust/cxx.h"
#include "canvas-cxx/src/webgl.rs.h"
#include "v8runtime/V8Runtime.h"


#include <cmath>
#include "../Helpers.h"

#include "../ImageAssetImpl.h"
#include "../ImageBitmapImpl.h"
#include "../RafImpl.h"

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

#include "WebGLRenderingContextBase.h"


class WebGLRenderingContext : WebGLRenderingContextBase {

public:
    static void Init(v8::Isolate *isolate);

    WebGLRenderingContext(rust::Box<WebGLState> state);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    ~WebGLRenderingContext();

    static void ToDataURL(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Resized(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetDrawingBufferWidth(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetDrawingBufferHeight(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetSuppressLogs(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetSuppressLogs(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                 const v8::PropertyCallbackInfo<void> &info);

    static void GetFlipY(v8::Local<v8::String> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Create(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void InstanceFromPointer(const v8::FunctionCallbackInfo<v8::Value> &args);

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

    static void ClearDepth(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ClearStencil(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Clear(const v8::FunctionCallbackInfo<v8::Value> &args);

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

    static void DrawElements(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void EnableVertexAttribArray(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Enable(const v8::FunctionCallbackInfo<v8::Value> &args);

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

    static void GetParameterInternal(const v8::FunctionCallbackInfo<v8::Value> &args, uint32_t pnameValue,
                                     rust::Box<WebGLResult> result);

    static void GetProgramInfoLog(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetProgramParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetRenderbufferParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetShaderInfoLog(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetShaderParameter(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetShaderPrecisionFormat(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetShaderSource(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetSupportedExtensions(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetSupportedExtensionsString(const v8::FunctionCallbackInfo<v8::Value> &args);

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

    static void Uniform1iv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform1fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform1i(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform2f(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform2iv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform2fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform2i(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform3f(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform3iv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform3fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform3i(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform4f(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform4iv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform4fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Uniform4i(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void UniformMatrix2fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void UniformMatrix3fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void UniformMatrix4fv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void UseProgram(const v8::FunctionCallbackInfo<v8::Value> &args);

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

    static void SetConstants(v8::Isolate *isolate, const v8::Local<v8::ObjectTemplate> &tmpl);

    static void SetProps(v8::Isolate *isolate, const v8::Local<v8::ObjectTemplate> &tmpl);

    static void SetMethods(v8::Isolate *isolate, const v8::Local<v8::ObjectTemplate> &tmpl);

    template<typename T>
    static void AddWeakListener(v8::Isolate *isolate, const v8::Local<v8::Object> &object, T *data){
        auto ext = v8::External::New(isolate, data);
        object->SetInternalField(0, ext);
        auto persistent = new v8::Persistent<v8::Object>(isolate, object);
        auto entry = new ObjectCacheEntry(static_cast<void *>(data), persistent);
        auto callback = [](const v8::WeakCallbackInfo<ObjectCacheEntry> &cacheEntry) {
            auto value = cacheEntry.GetParameter();
            auto ptr = static_cast<T *>(value->data);
            if (ptr != nullptr) {
                delete ptr;
            }
            auto persistent_ptr = value->object;
            if (persistent_ptr != nullptr) {
                if (!persistent_ptr->IsEmpty()) {
                    persistent_ptr->Reset();
                }
            }
            delete value;
        };
        persistent->SetWeak(entry, callback, v8::WeakCallbackType::kFinalizer);
    }

private:
    static WebGLRenderingContext *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    bool supressLogs_ = true;

};
