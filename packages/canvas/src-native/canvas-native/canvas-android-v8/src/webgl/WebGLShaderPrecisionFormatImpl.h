//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once
#include "../Common.h"
#include "../Caches.h"
#include "../Helpers.h"

class WebGLShaderPrecisionFormatImpl {
public:
    WebGLShaderPrecisionFormatImpl(rust::Box<WebGLShaderPrecisionFormat> shader);

    static void Init(v8::Isolate *isolate);

    static void Create(const v8::FunctionCallbackInfo <v8::Value> &args);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, rust::Box<WebGLShaderPrecisionFormat> shader);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void GetRangeMin(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetRangeMax(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetPrecision(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

private:
    rust::Box<WebGLShaderPrecisionFormat> shader_;

    static WebGLShaderPrecisionFormatImpl *GetPointer(const v8::Local<v8::Object>& object);
};

