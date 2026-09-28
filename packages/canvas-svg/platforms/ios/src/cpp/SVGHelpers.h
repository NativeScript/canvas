#pragma once

#include "SVGCommon.h"
#include <string>

inline static v8::Local<v8::String>
ConvertToV8String(v8::Isolate *isolate, const std::string &string) {
    return v8::String::NewFromUtf8(isolate, string.c_str()).ToLocalChecked();
}

inline static std::string
ConvertFromV8String(v8::Isolate *isolate, const v8::Local<v8::Value> &value) {
    if (value.IsEmpty() || !value->IsString()) {
        return {};
    }
    v8::String::Utf8Value utf8(isolate, value);
    if (*utf8 == nullptr) {
        return {};
    }
    return std::string(*utf8, utf8.length());
}
