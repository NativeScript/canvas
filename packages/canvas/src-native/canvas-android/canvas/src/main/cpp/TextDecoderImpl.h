//
// Created by Osei Fortune on 19/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include <vector>
#include "Common.h"

using namespace org::nativescript::canvas;

class TextDecoderImpl {
public:
    TextDecoderImpl(rust::Box<TextDecoder> decoder);

    TextDecoder &GetTextDecoder();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static TextDecoderImpl *GetPointer(v8::Local<v8::Object> object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void Ctor(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Decode(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Encoding(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

private:
    rust::Box<TextDecoder> decoder_;
};
