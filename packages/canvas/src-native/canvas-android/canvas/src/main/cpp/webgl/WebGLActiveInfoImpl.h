//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include "Common.h"
#include <vector>

using namespace org::nativescript::canvas;

class WebGLActiveInfoImpl {
public:
    WebGLActiveInfoImpl(rust::Box<WebGLActiveInfo> info);

    WebGLActiveInfo &GetWebGLActiveInfo();

    static WebGLActiveInfoImpl *GetPointer(const v8::Local<v8::Object>& object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void GetName(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetSize(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetType(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

private:
    rust::Box<WebGLActiveInfo> info_;
};

