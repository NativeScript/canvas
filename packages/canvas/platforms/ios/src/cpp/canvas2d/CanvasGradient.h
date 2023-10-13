//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include <vector>
#include "Common.h"
#include "Helpers.h"

using namespace ::org::nativescript::canvas;

class CanvasGradient {
public:
    CanvasGradient(rust::Box<PaintStyle> style);

    PaintStyle &GetPaintStyle();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static CanvasGradient *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, CanvasGradient *gradient) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = CanvasGradient::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::CanvasGradient);
        auto ext = v8::External::New(isolate, gradient);
        object->SetInternalField(0, ext);
        return scope.Escape(object);
    }

    static void AddColorStop(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    rust::Box<PaintStyle> style_;
};

