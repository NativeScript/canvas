//
// Created by Osei Fortune on 28/03/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include "v8runtime/V8Runtime.h"
#include "Helpers.h"
#include "v8-fast-api-calls.h"
#include <vector>

using namespace org::nativescript::canvas;

class Path2D {
public:
    Path2D(rust::Box<Path> path);

    Path &GetPath();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static Path2D *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void Ctor(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void AddPath(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Arc(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ArcTo(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BezierCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ClosePath(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Ellipse(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void LineTo(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void MoveTo(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void QuadraticCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Rect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RoundRect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void __toSVG(const v8::FunctionCallbackInfo<v8::Value> &args);


private:
    rust::Box<Path> path_;
};
