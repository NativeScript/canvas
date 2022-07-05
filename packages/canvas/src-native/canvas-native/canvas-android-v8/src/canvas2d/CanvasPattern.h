//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once
#include "../Common.h"
#include "../Caches.h"
#include "../Helpers.h"
#include "MatrixImpl.h"

class CanvasPattern {
public:
    CanvasPattern(rust::Box<PaintStyle> style);
    static void Init(v8::Isolate *isolate);

    static void CreateCallback(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetTransform(const v8::FunctionCallbackInfo<v8::Value> &args);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, rust::Box<PaintStyle> style);

    static CanvasPattern *GetPointer(const v8::Local<v8::Object>& object);

    PaintStyle& GetPaintStyle();
private:
    rust::Box<PaintStyle> style_;
    static v8::Local<v8::FunctionTemplate> GetCtorFunc(v8::Isolate *isolate);
};

