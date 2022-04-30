//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "../../Common.h"
#include "../../Caches.h"
#include "../../Helpers.h"

class ANGLE_instanced_arraysImpl {
public:
    ANGLE_instanced_arraysImpl(rust::Box<ANGLE_instanced_arrays> arrays);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, rust::Box<ANGLE_instanced_arrays> arrays);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void DrawArraysInstancedANGLE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawElementsInstancedANGLE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void VertexAttribDivisorANGLE(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    rust::Box<ANGLE_instanced_arrays> arrays_;

    static ANGLE_instanced_arraysImpl *GetPointer(v8::Local<v8::Object> object);
};

