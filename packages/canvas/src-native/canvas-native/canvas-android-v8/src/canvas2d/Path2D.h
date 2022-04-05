//
// Created by Osei Fortune on 28/03/2022.
//

#ifndef CANVAS_NATIVE_PATH2D_H
#define CANVAS_NATIVE_PATH2D_H

#include "../Common.h"
#include "../Caches.h"
#include "../Helpers.h"
#include "rust/cxx.h"
#include "canvas-android-v8/src/bridges/path.rs.h"

class Path2D {
public:
    Path2D(rust::Box <Path> path);

    ~Path2D();

    static void Init(v8::Isolate *isolate);

    static void Create(const v8::FunctionCallbackInfo<v8::Value> &args);

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

private:
    rust::Box <Path> path_;

    static Path2D *GetPointer(v8::Local<v8::Object> object);

    static v8::Local<v8::Function> GetCtor(v8::Isolate *isolate);
};


#endif //CANVAS_NATIVE_PATH2D_H
