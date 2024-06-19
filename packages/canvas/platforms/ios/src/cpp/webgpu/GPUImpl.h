//
// Created by Osei Fortune on 17/06/2024.
//

#ifndef CANVAS_ANDROID_GPUIMPL_H
#define CANVAS_ANDROID_GPUIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "GPUAdapterImpl.h"

class GPUImpl {
public:
    static void Init(const v8::Local<v8::Object> &canvasModule, v8::Isolate *isolate);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void RequestAdapter(const v8::FunctionCallbackInfo<v8::Value> &args);
};


#endif //CANVAS_ANDROID_GPUIMPL_H
