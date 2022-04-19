//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once
#include "../Common.h"
#include "../Helpers.h"
#include "../Caches.h"

class CanvasGradient {
public:
    CanvasGradient(rust::Box<PaintStyle> style);

    static void Init(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate* isolate, rust::Box<PaintStyle> style);

    static void CreateCallback(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void AddColorStop(const v8::FunctionCallbackInfo<v8::Value> &args);

    static CanvasGradient *GetPointer(v8::Local<v8::Object> object);

    PaintStyle& GetPaintStyle();

private:
    rust::Box<PaintStyle> style_;
    static v8::Local<v8::Function> GetCtorFunc(v8::Isolate *isolate);
};

