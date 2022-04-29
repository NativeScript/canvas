//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include "../Common.h"
#include "../Caches.h"
#include "../Helpers.h"

class WebGLActiveInfoImpl {
public:
    WebGLActiveInfoImpl(rust::Box<WebGLActiveInfo> info);

    static void Init(v8::Isolate *isolate);

    static void Create(const v8::FunctionCallbackInfo <v8::Value> &args);

    static v8::Local<v8::Object> NewInstance(v8::Isolate* isolate, rust::Box<WebGLActiveInfo> info);

    static v8::Local<v8::Function> GetCtor(v8::Isolate *isolate);

    static void GetName(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetSize(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetType(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);
private:
    rust::Box<WebGLActiveInfo> info_;
    static WebGLActiveInfoImpl *GetPointer(v8::Local<v8::Object> object);
};

