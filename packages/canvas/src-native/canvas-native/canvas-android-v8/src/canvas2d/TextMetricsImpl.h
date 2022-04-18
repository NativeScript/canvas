//
// Created by Osei Fortune on 18/04/2022.
//

#ifndef CANVAS_NATIVE_TEXTMETRICSIMPL_H
#define CANVAS_NATIVE_TEXTMETRICSIMPL_H

#include "../Common.h"
#include "../Helpers.h"
#include "../Caches.h"
#include "rust/cxx.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

class TextMetricsImpl {
public:
    TextMetricsImpl(rust::Box <TextMetrics> metrics);

    static void Init(v8::Isolate *isolate);

    static void Create(const v8::FunctionCallbackInfo<v8::Value> &args);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, rust::Box <TextMetrics> metrics);

    static void GetWidth(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void ActualBoundingBoxLeft(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void ActualBoundingBoxRight(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void ActualBoundingBoxAscent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void ActualBoundingBoxDescent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void FontBoundingBoxAscent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void FontBoundingBoxDescent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void EmHeightAscent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void EmHeightDescent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void HangingBaseline(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void AlphabeticBaseline(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void IdeographicBaseline(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

private:
    rust::Box <TextMetrics> metrics_;

    static TextMetricsImpl *GetPointer(v8::Local<v8::Object> object);

    static v8::Local<v8::Object> GetCtor(v8::Isolate *isolate);
};


#endif //CANVAS_NATIVE_TEXTMETRICSIMPL_H
