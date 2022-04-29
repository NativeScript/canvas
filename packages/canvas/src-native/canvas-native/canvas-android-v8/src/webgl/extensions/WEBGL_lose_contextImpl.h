//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "../../Common.h"
#include "../../Caches.h"
#include "../../Helpers.h"


class WEBGL_lose_contextImpl {
public:
    WEBGL_lose_contextImpl(rust::Box <WEBGL_lose_context> context);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, rust::Box <WEBGL_lose_context> context);

    static v8::Local<v8::Function> GetCtor(v8::Isolate *isolate);

    static void LoseContext(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RestoreContext(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    rust::Box <WEBGL_lose_context> context_;

    static WEBGL_lose_contextImpl *GetPointer(v8::Local<v8::Object> object);
};
