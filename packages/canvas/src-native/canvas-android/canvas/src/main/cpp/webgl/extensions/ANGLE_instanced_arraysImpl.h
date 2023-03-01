//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/webgl.rs.h"
#include "v8runtime/V8Runtime.h"

using namespace facebook;

class JSI_EXPORT ANGLE_instanced_arraysImpl : jsi::HostObject {
public:
    ANGLE_instanced_arraysImpl(rust::Box<ANGLE_instanced_arrays> arrays);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    ANGLE_instanced_arrays &GetArrays();

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, rust::Box<ANGLE_instanced_arrays> arrays);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void DrawArraysInstancedANGLE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawElementsInstancedANGLE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void VertexAttribDivisorANGLE(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    rust::Box<ANGLE_instanced_arrays> arrays_;
};

