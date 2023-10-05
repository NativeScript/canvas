//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include "v8runtime/V8Runtime.h"
#include "MatrixImpl.h"
#include <vector>

using namespace org::nativescript::canvas;

class CanvasPattern{
public:
    CanvasPattern(rust::Box<PaintStyle> style);

    PaintStyle &GetPaintStyle();

    static void Init(const v8::Local<v8::Object>& canvasModule, v8::Isolate *isolate);

    static CanvasPattern *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void SetTransform(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    rust::Box<PaintStyle> style_;
};

