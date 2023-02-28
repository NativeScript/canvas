//
// Created by Osei Fortune on 28/03/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/canvas2d.rs.h"
#include "v8runtime/V8Runtime.h"

using namespace facebook;

class JSI_EXPORT Path2D : public jsi::HostObject {
public:
    Path2D(rust::Box<Path> path);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

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

    static void ToSVG(const v8::FunctionCallbackInfo<v8::Value> &args);


    Path &GetPath();

private:
    rust::Box<Path> path_;
};
