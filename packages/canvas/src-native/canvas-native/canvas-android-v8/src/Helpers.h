//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "Common.h"

class Helpers {
public:
    static void ThrowIllegalConstructor(v8::Isolate *isolate);

    static v8::Local<v8::String> ConvertToV8String(v8::Isolate *isolate, const std::string &string);

    static rust::String ConvertFromV8String(v8::Isolate *isolate, const v8::Local<v8::String> &value);

    static std::string ConvertFromV8StringToString(v8::Isolate *isolate, const v8::Local<v8::String> &value);

    static bool IsInstanceOf(v8::Isolate *isolate, v8::Local<v8::Value> value, std::string clazz);

    static void SetInternalClassName(v8::Isolate *isolate, v8::Local<v8::Object> value, std::string clazz);

    static void
    SetPrivate(v8::Isolate *isolate, v8::Local<v8::Object> object, std::string property, v8::Local<v8::Value> value);

    static v8::Local<v8::Value> GetPrivate(v8::Isolate *isolate, v8::Local<v8::Object> object, std::string property);

    static v8::Local<v8::Value> ArrayGet(v8::Local<v8::Context> context, v8::Local<v8::Array> array, uint32_t i);

    static void
    ArraySet(v8::Local<v8::Context> context, v8::Local<v8::Array> array, uint32_t i, v8::Local<v8::Value> value);

    template<typename T>
    static rust::Slice<T> GetTypedArrayData(v8::Local<v8::TypedArray> array) {
        auto buffer = array->Buffer();
        auto store = buffer->GetBackingStore();
        auto data = static_cast<uint8_t *>(store->Data()) + array->ByteOffset();
        rust::Slice<T> slice(reinterpret_cast<T *>(data), array->Length());
        return std::move(slice);
    }
};
