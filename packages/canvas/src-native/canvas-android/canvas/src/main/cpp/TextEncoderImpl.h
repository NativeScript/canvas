//
// Created by Osei Fortune on 19/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include "VecMutableBuffer.h"
#include <vector>

using namespace org::nativescript::canvas;

class TextEncoderImpl {

public:
    TextEncoderImpl(rust::Box<TextEncoder> encoder);

    TextEncoder &GetTextEncoder();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static TextEncoderImpl *GetPointer(v8::Local<v8::Object> object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void Ctor(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Encode(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Encoding(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

private:
    rust::Box<TextEncoder> encoder_;
};
