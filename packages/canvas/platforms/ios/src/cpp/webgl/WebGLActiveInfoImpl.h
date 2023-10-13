//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include "../rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include "Common.h"
#include "Helpers.h"
#include <vector>

using namespace org::nativescript::canvas;

class WebGLActiveInfoImpl {
public:
    WebGLActiveInfoImpl(rust::Box<WebGLActiveInfo> info);

    WebGLActiveInfo &GetWebGLActiveInfo();

    static WebGLActiveInfoImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, WebGLActiveInfoImpl *info) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WebGLActiveInfoImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::WebGLActiveInfo);
        auto ext = v8::External::New(isolate, info);
        object->SetInternalField(0, ext);
        return scope.Escape(object);
    }

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void
    GetName(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void
    GetSize(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void
    GetType(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

private:
    rust::Box<WebGLActiveInfo> info_;
};

