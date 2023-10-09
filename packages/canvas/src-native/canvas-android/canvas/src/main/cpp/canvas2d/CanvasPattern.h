//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include "MatrixImpl.h"
#include <vector>

using namespace org::nativescript::canvas;

class CanvasPattern {
public:
    CanvasPattern(rust::Box<PaintStyle> style);

    PaintStyle &GetPaintStyle();

    static void Init(const v8::Local<v8::Object> &canvasModule, v8::Isolate *isolate);

    static CanvasPattern *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, CanvasPattern *pattern) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = CanvasPattern::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::CanvasPattern);
        auto ext = v8::External::New(isolate, pattern);
        object->SetInternalField(0, ext);
        return scope.Escape(object);
    }

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void SetTransform(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    rust::Box<PaintStyle> style_;
};

