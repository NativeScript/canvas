//
// Created by Osei Fortune on 19/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/canvas2d.rs.h"
#import "v8runtime/V8Runtime.h"

using namespace facebook;

class JSI_EXPORT TextDecoderImpl : public jsi::HostObject {
public:
    TextDecoderImpl(rust::Box<TextDecoder> decoder);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    TextDecoder &GetTextDecoder();

    static void
    GetEncoding(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Decode(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    rust::Box<TextDecoder> decoder_;
};
