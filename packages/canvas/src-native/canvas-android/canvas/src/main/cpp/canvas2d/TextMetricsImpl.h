//
// Created by Osei Fortune on 18/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include <vector>
#include "Helpers.h"

using namespace org::nativescript::canvas;

class TextMetricsImpl {
public:
    TextMetricsImpl(rust::Box<TextMetrics> metrics);

    TextMetrics &GetTextMetrics();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static TextMetricsImpl *GetPointer(v8::Local<v8::Object> object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void GetWidth(v8::Local<v8::String> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetActualBoundingBoxLeft(v8::Local<v8::String> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);


    static void GetActualBoundingBoxRight(v8::Local<v8::String> name,
                                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetActualBoundingBoxAscent(v8::Local<v8::String> name,
                                          const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetActualBoundingBoxDescent(v8::Local<v8::String> name,
                                           const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetFontBoundingBoxAscent(v8::Local<v8::String> name,
                                           const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetFontBoundingBoxDescent(v8::Local<v8::String> name,
                                            const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetEmHeightAscent(v8::Local<v8::String> name,
                                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetEmHeightDescent(v8::Local<v8::String> name,
                                          const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetHangingBaseline(v8::Local<v8::String> name,
                                   const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetAlphabeticBaseline(v8::Local<v8::String> name,
                                   const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetIdeographicBaseline(v8::Local<v8::String> name,
                                   const v8::PropertyCallbackInfo<v8::Value> &info);

private:
    rust::Box<TextMetrics> metrics_;
};

