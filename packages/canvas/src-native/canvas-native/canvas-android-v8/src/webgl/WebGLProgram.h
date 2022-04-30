//
// Created by Osei Fortune on 27/04/2022.
//

#pragma

#include "../Common.h"
#include "../Caches.h"
#include "../Helpers.h"

class WebGLProgram {
public:
    static void Init(v8::Isolate *isolate);

    static void Create(const v8::FunctionCallbackInfo<v8::Value> &args);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);
};

