//
// Created by Osei Fortune on 25/02/2023.
//

#pragma once

#include <memory>
#include "rust/cxx.h"
#include "Common.h"

enum class NativeType {
    None,
    CanvasGradient,
    CanvasPattern,
    ImageData,
    ImageAsset,
    CanvasRenderingContext2D,
    WebGLRenderingContextBase,
    Path2D,
    Matrix,
    ImageBitmap,
    TextMetrics,

    WebGLQuery,
    WebGLProgram,
    WebGLShader,
    WebGLBuffer,
    WebGLFramebuffer,
    WebGLRenderbuffer,
    WebGLTexture
};


template<typename T>
struct VecBuffer {
public:
    VecBuffer(rust::Vec<T> buffer) : vec_(std::move(buffer)) {
        this->buf_ = vec_.data();
        this->buffer_size_ = vec_.size();
        this->size_ = vec_.size() * sizeof(T);
    }

    T *buffer_data() {
        return this->buf_;
    }

    size_t buffer_size() const {
        return this->buffer_size_;
    }

    uint8_t *data() {
        return (uint8_t *) this->buf_;
    }

    size_t size() const {
        return this->size_;
    }

    ~VecBuffer() {
        this->buf_ = nullptr;
    }

private:
    T *buf_;
    size_t size_;
    size_t buffer_size_;
    rust::Vec<T> vec_;
};


inline static v8::Local<v8::String>
ConvertToV8String(v8::Isolate *isolate, const std::string &string) {
    return v8::String::NewFromUtf8(isolate, string.c_str()).ToLocalChecked();
}

inline static std::string
ConvertFromV8String(v8::Isolate *isolate, const v8::Local<v8::Value> &value) {
    if (value.IsEmpty()) {
        return {};
    }

    if (value->IsStringObject()) {
        v8::Local<v8::String> obj = value.As<v8::StringObject>()->ValueOf();
        return ConvertFromV8String(isolate, obj);
    }

    v8::String::Utf8Value result(isolate, value);

    const char *val = *result;

    if (val == nullptr) {
        return {};
    }

    return {*result};
}


static void SetPrivateValue(v8::Isolate *isolate, const v8::Local<v8::Object> &obj,
                            const v8::Local<v8::String> &propName,
                            const v8::Local<v8::Value> &value) {
    v8::Local<v8::Context> context;
    obj->GetCreationContext().ToLocal(&context);
    v8::Local<v8::Private> privateKey = v8::Private::ForApi(isolate, propName);

    obj->SetPrivate(context, privateKey, value);
}

static v8::Local<v8::Value>
GetPrivateValue(v8::Isolate *isolate, const v8::Local<v8::Object> &obj,
                const v8::Local<v8::String> &propName) {
    v8::Local<v8::Context> context;
    obj->GetCreationContext().ToLocal(&context);
    v8::Local<v8::Private> privateKey = v8::Private::ForApi(isolate, propName);

    v8::Maybe<bool> hasPrivate = obj->HasPrivate(context, privateKey);

    if (!hasPrivate.FromMaybe(false)) {
        return v8::Local<v8::Value>();
    }

    v8::Local<v8::Value> result;

    obj->GetPrivate(context, privateKey).ToLocal(&result);

    return result;
}

static void SetNativeType(v8::Isolate *isolate, const v8::Local<v8::Object> &obj, NativeType type) {
    v8::Local<v8::String> name = ConvertToV8String(isolate, "__type");
    v8::Local<v8::Value> typeValue = v8::Number::New(isolate, (double) type).As<v8::Value>();
    SetPrivateValue(isolate, obj, name, typeValue);
}

inline static NativeType GetNativeType(v8::Isolate *isolate, const v8::Local<v8::Object> &obj) {
    v8::Local<v8::String> name = ConvertToV8String(isolate, "__type");
    auto ret = GetPrivateValue(isolate, obj, name);

    auto context = isolate->GetCurrentContext();
    if (ret->IsNumber()) {
        auto value = (int) ret->NumberValue(context).ToChecked();
        if (value >= (int) NativeType::CanvasGradient && value <= (int) NativeType::TextMetrics) {
            return (NativeType) value;
        }
    }

    return NativeType::None;
}


template<typename T>
inline static rust::Slice<T>
GetArrayBufferData(v8::Local<v8::ArrayBuffer> &array) {
    auto buf = array->GetBackingStore()->Data();
    auto size = array->ByteLength();

    rust::Slice<T> slice(reinterpret_cast<T *>(buf), size);
    return std::move(slice);
}

template<typename T>
inline static rust::Slice<T>
GetTypedArrayData(v8::Local<v8::TypedArray> &array) {
    auto buf = array->Buffer();
    auto offset = array->ByteOffset();
    auto size = buf->ByteLength();

    rust::Slice<T> slice(reinterpret_cast<T *>(buf->GetBackingStore()->Data()) + offset,
                         (size / sizeof(T)));
    return std::move(slice);
}
