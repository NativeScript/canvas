//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "Common.h"
#include "rust/cxx.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

class Helpers {
public:
    static void ThrowIllegalConstructor(v8::Isolate *isolate);

    static v8::Local<v8::String> ConvertToV8String(v8::Isolate *isolate, const std::string &string);

    static rust::String ConvertFromV8String(v8::Isolate *isolate, const v8::Local<v8::String> &value);

    static bool IsInstanceOf(v8::Isolate *isolate, v8::Local<v8::Object> value, std::string clazz);

    static void SetInternalClassName(v8::Isolate *isolate, v8::Local<v8::Object> value, std::string clazz);
};
