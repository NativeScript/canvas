//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once

#include "../Common.h"
#include "../Caches.h"
#include "../Helpers.h"

class WebGLSyncImpl {
public:
    WebGLSyncImpl(rust::Box<WebGLSync> sync);

    static void Init(v8::Isolate *isolate);

    static void Create(const v8::FunctionCallbackInfo<v8::Value> &args);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, rust::Box<WebGLSync> sync_);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static WebGLSyncImpl *GetPointer(v8::Local<v8::Object> object);

    WebGLSync &GetSync();

private:
    rust::Box<WebGLSync> sync_;
};
