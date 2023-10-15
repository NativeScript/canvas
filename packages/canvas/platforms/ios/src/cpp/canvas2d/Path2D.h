//
// Created by Osei Fortune on 28/03/2022.
//

#pragma once

#include "Common.h"
#include "Helpers.h"
#include <vector>

class Path2D {
public:
    Path2D(Path* path);
    ~Path2D(){
        canvas_native_path_destroy(this->GetPath());
        this->path_ = nullptr;
    }

    Path* GetPath();

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
    Path* path_;
};
