//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include "../Common.h"
#include "../Caches.h"
#include "../Helpers.h"


class WebGLUniformLocation {
public:
    static void Init(v8::Isolate *isolate);

    static void Create(const v8::FunctionCallbackInfo<v8::Value> &args);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, int32_t location);
};
