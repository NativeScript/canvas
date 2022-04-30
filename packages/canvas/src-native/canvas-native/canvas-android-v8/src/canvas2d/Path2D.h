//
// Created by Osei Fortune on 28/03/2022.
//

#pragma once

#include "../Common.h"
#include "../Caches.h"
#include "../Helpers.h"
#include "../ObjectCacheEntry.h"

class Path2D {
public:
    Path2D(rust::Box <Path> path);

    ~Path2D();

    static void Init(v8::Isolate *isolate);

    static void Create(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void AddWeakListener(v8::Isolate *isolate, v8::Local<v8::Object> object, Path2D* path);

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

    static Path2D *GetPointer(v8::Local<v8::Object> object);

    Path& GetPath();

private:
    rust::Box <Path> path_;

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);
};
