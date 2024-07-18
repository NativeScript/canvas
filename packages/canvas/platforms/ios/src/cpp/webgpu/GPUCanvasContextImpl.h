//
// Created by Osei Fortune on 20/06/2024.
//

#ifndef CANVAS_ANDROID_GPUCANVASCONTEXTIMPL_H
#define CANVAS_ANDROID_GPUCANVASCONTEXTIMPL_H

#include "Helpers.h"
#include "ObjectWrapperImpl.h"
#include "GPUDeviceImpl.h"
#include "GPUTextureImpl.h"

class GPUCanvasContextImpl : ObjectWrapperImpl {
public:
    explicit GPUCanvasContextImpl(const CanvasGPUCanvasContext *context);

    ~GPUCanvasContextImpl() {
        canvas_native_webgpu_context_release(this->context_);
    }

    const CanvasGPUCanvasContext *GetContext();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUCanvasContextImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPUCanvasContextImpl *ctx) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUCanvasContextImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPUCanvasContext);
        object->SetAlignedPointerInInternalField(0, ctx);
        ctx->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void Configure(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void UnConfigure(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetCurrentTexture(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void PresentSurface(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetCapabilities(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    const CanvasGPUCanvasContext *context_;
};


#endif //CANVAS_ANDROID_GPUCANVASCONTEXTIMPL_H
