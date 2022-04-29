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

    static v8::Local<v8::Function> GetCtor(v8::Isolate *isolate);
};
