//
// Created by Osei Fortune on 10/06/2022.
//

#pragma once
#include <stdint.h>
#include <string.h>

#ifdef __APPLE__
#include <NativeScript/include/v8.h>

#ifdef __cplusplus
extern "C" {
#endif

#include <CanvasNative/canvas_native.h>
#include <CanvasNative/canvas_ios.h>

#ifdef __cplusplus
}
#endif
#endif

#ifdef __ANDROID__
#include "include/v8.h"
#include "include/canvas_android.h"
#include <android/log.h>
#include <thread>
#endif

namespace canvas {
inline void* GetAlignedPointer(v8::Local<v8::Object> object, int index) {
#if V8_MAJOR_VERSION >= 14
    return object->GetAlignedPointerFromInternalField(index, v8::kEmbedderDataTypeTagDefault);
#else
    return object->GetAlignedPointerFromInternalField(index);
#endif
}
inline void SetAlignedPointer(v8::Local<v8::Object> object, int index, void* value) {
#if V8_MAJOR_VERSION >= 14
    object->SetAlignedPointerInInternalField(index, value, v8::kEmbedderDataTypeTagDefault);
#else
    object->SetAlignedPointerInInternalField(index, value);
#endif
}
inline v8::Local<v8::External> NewExternal(v8::Isolate* isolate, void* value) {
#if V8_MAJOR_VERSION >= 14
    return v8::External::New(isolate, value, v8::kExternalPointerTypeTagDefault);
#else
    return v8::External::New(isolate, value);
#endif
}
inline void* ExternalValue(v8::Local<v8::External> value) {
#if V8_MAJOR_VERSION >= 14
    return value->Value(v8::kExternalPointerTypeTagDefault);
#else
    return value->Value();
#endif
}
}

// V8 14 removed the legacy fast typed-array callbacks. Jitless Apple builds
// use the existing FunctionCallback implementations instead.
#if V8_MAJOR_VERSION >= 14
#define CANVAS_FAST_FUNCTION(callback) (v8::CFunction{})
#else
#define CANVAS_FAST_FUNCTION(callback) v8::CFunction::Make(callback)
#endif

namespace canvas {
inline void SetAccessor(v8::Local<v8::ObjectTemplate> object, v8::Local<v8::Name> name,
    v8::AccessorNameGetterCallback getter, v8::AccessorNameSetterCallback setter = nullptr,
    v8::Local<v8::Value> data = {}, v8::PropertyAttribute attributes = v8::None) {
#if V8_MAJOR_VERSION >= 14
    object->SetNativeDataProperty(name, getter, setter, data, attributes);
#else
    object->SetAccessor(name, getter, setter, data, v8::DEFAULT, attributes);
#endif
}
template <typename T>
inline v8::Local<v8::Object> Receiver(const v8::PropertyCallbackInfo<T>& info) {
#if V8_MAJOR_VERSION >= 14
    return info.Holder();
#else
    return info.This();
#endif
}
template <typename T>
inline v8::Local<v8::Object> Receiver(const v8::FunctionCallbackInfo<T>& info) {
    return info.This();
}
}

namespace canvas {
inline size_t Utf8Length(v8::Local<v8::String> text, v8::Isolate* isolate) {
#if V8_MAJOR_VERSION >= 14
    return text->Utf8LengthV2(isolate);
#else
    return text->Utf8Length(isolate);
#endif
}
inline void WriteUtf8(v8::Local<v8::String> text, v8::Isolate* isolate, char* buffer,
    size_t capacity, bool terminate) {
#if V8_MAJOR_VERSION >= 14
    text->WriteUtf8V2(isolate, buffer, capacity,
        terminate ? v8::String::WriteFlags::kNullTerminate : v8::String::WriteFlags::kNone);
#else
    text->WriteUtf8(isolate, buffer, static_cast<int>(capacity), nullptr,
        terminate ? v8::String::PRESERVE_ONE_BYTE_NULL : v8::String::NO_NULL_TERMINATION);
#endif
}
}
