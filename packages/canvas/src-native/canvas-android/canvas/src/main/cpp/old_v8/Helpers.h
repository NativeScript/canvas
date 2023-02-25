//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include <sstream>
#include <iostream>
#include <iomanip>

#include "Common.h"
#include "rust/cxx.h"
#include "v8.h"
#include <cassert>
#include <dlfcn.h>
#include <stdint.h>
#include <GLES2/gl2.h>
#include <GLES2/gl2ext.h>
#include <GLES3/gl3.h>
#include <android/log.h>
#include "ObjectCacheEntry.h"

enum class ObjectType : uint8_t {
    Unknown,
    ImageAsset,
    ImageBitmap,
    TextDecoder,
    TextEncoder,
    CanvasGradient,
    CanvasPattern,
    CanvasRenderingContext2D,
    ImageData,
    Matrix,
    Path2D,
    TextMetrics,
    WebGLShader,
    WebGLTexture,
    WebGLRenderbuffer,
    WebGLUniformLocation,
    WebGLRenderingContext,
    WebGLBuffer,
    WebGLProgram,
    WebGLFramebuffer,
    WebGLActiveInfo,
    WebGLShaderPrecisionFormat,
    ANGLE_instanced_arrays,
    EXT_blend_minmax,
    EXT_color_buffer_half_float,
    EXT_disjoint_timer_query,
    EXT_shader_texture_lod,
    EXT_sRGB,
    EXT_texture_filter_anisotropic,
    OES_element_index_uint,
    OES_standard_derivatives,
    OES_texture_float_linear,
    OES_texture_float,
    OES_texture_half_float_linear,
    OES_texture_half_float,
    OES_vertex_array_object,
    WEBGL_color_buffer_float,
    WEBGL_compressed_texture_atc,
    WEBGL_compressed_texture_etc1,
    WEBGL_compressed_texture_etc,
    WEBGL_compressed_texture_pvrtc,
    WEBGL_compressed_texture_s3tc_srgb,
    WEBGL_compressed_texture_s3tc,
    WEBGL_depth_texture,
    WEBGL_draw_buffers,
    WEBGL_lose_context,
    WebGL2RenderingContext,
    WebGLQuery,
    WebGLSampler,
    WebGLVertexArrayObject,
    WebGLTransformFeedback,
    WebGLSync,
    HTMLCanvas,
    HTMLImage,
    HTMLVideo
};

class Helpers {
public:
    static const char *LOG_TAG;

    static int m_maxLogcatObjectSize;

    static std::string RGBAToHex(uint8_t r, uint8_t g, uint8_t b, uint8_t a);

    static void sendToADBLogcat(const std::string &message, android_LogPriority logPriority);

    static void LogToConsole(const std::string &message);

    static void LogWarningToConsole(const std::string &message);

    static void ThrowIllegalConstructor(v8::Isolate *isolate);

    static v8::Local<v8::String> ConvertToV8String(v8::Isolate *isolate, const std::string &string);

    static std::string ConvertFromV8String(v8::Isolate *isolate, const v8::Local<v8::Value> &value);

    static v8::Local<v8::String> ConvertToV8String(v8::Isolate *isolate, const char *string);

    static rust::Str ConvertFromV8StringToRust(v8::Isolate *isolate, const v8::Local<v8::Value> &value);

    static bool
    IsInstanceOf(v8::Isolate *isolate, const v8::Local<v8::Value> &value, const v8::Local<v8::Object> &instance);

    static bool IsInstanceOf(v8::Isolate *isolate, const v8::Local<v8::Value> &value, const std::string &clazz);

    static void
    SetPrivate(v8::Isolate *isolate, const v8::Local<v8::Object> &object, const std::string &property,
               const v8::Local<v8::Value> &value);

    static v8::Local<v8::Value>
    GetPrivate(v8::Isolate *isolate, const v8::Local<v8::Object> &object, const std::string &property);


    static void
    SetInstanceType(v8::Isolate *isolate, const v8::Local<v8::Object> &object, ObjectType type);

    static ObjectType
    GetInstanceType(v8::Isolate *isolate, const v8::Local<v8::Value> &object);


    static v8::Local<v8::ArrayBuffer>
    CreateArrayBuffer(v8::Isolate *isolate, void *data, size_t size, v8::BackingStore::DeleterCallback callback,
                      void *deleter_data);

    template<typename T>
    static rust::Slice<T> GetTypedArrayData(const v8::Local<v8::TypedArray> &array) {
        auto buffer = array->Buffer();
        auto store = buffer->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + array->ByteOffset();
        rust::Slice<T> slice(reinterpret_cast<T *>(data), (array->ByteLength() / sizeof(T)));
        return std::move(slice);
    }

    static rust::Slice<uint8_t> GetArrayBufferData(const v8::Local<v8::ArrayBuffer> &array) {
        auto store = array->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data());
        rust::Slice<uint8_t> slice(reinterpret_cast<uint8_t *>(data), array->ByteLength());
        return std::move(slice);
    }

    inline static bool IsNumber(const v8::Local<v8::Value> &value) {
        return value->IsNumber() || value->IsNumberObject();
    }

    inline static bool IsString(const v8::Local<v8::Value> &value) {
        return value->IsString() || value->IsStringObject();
    }

    inline static bool IsBoolean(const v8::Local<v8::Value> &value) {
        return value->IsBoolean() || value->IsBooleanObject();
    }

    inline static bool IsObject(const v8::Local<v8::Value> &value) {
        return !value->IsNullOrUndefined() && value->IsObject();
    }

    inline static double GetNumberValue(v8::Isolate *isolate, const v8::Local<v8::Value> &value) {
        if (value->IsNumberObject()) {
            return value.As<v8::NumberObject>()->ValueOf();
        }
        return value->NumberValue(isolate->GetCurrentContext()).ToChecked();
    }

    inline static std::string GetString(v8::Isolate *isolate, const v8::Local<v8::Value> &value) {
        if (value->IsStringObject()) {
            return ConvertFromV8String(isolate, value.As<v8::StringObject>()->ValueOf());
        }
        return ConvertFromV8String(isolate, value->ToString(isolate->GetCurrentContext()).ToLocalChecked());
    }

    inline static bool GetBoolean(v8::Isolate *isolate, const v8::Local<v8::Value> &value) {
        if (value->IsBooleanObject()) {
            return value.As<v8::BooleanObject>()->ValueOf();
        }
        return value->BooleanValue(isolate);
    }


};
