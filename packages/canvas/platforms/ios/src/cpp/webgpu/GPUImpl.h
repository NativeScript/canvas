//
// Created by Osei Fortune on 17/06/2024.
//

#ifndef CANVAS_ANDROID_GPUIMPL_H
#define CANVAS_ANDROID_GPUIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "GPUAdapterImpl.h"
#include "ObjectWrapperImpl.h"

class GPUImpl : ObjectWrapperImpl {
public:
    GPUImpl(const CanvasWebGPUInstance *instance);

    ~GPUImpl() {
        if (this->instance_ != nullptr) {
            canvas_native_webgpu_instance_release(this->instance_);
            this->instance_ = nullptr;
        }
    }

    const CanvasWebGPUInstance *GetGPUInstance();

    static void Init(const v8::Local<v8::Object> &canvasModule, v8::Isolate *isolate);

    static GPUImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void Ctor(const v8::FunctionCallbackInfo<v8::Value> &args);


    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPUImpl *instance) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPUInstance);
        object->SetAlignedPointerInInternalField(0, instance);
        instance->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void __GetPointer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RequestAdapter(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    const CanvasWebGPUInstance *instance_;
};


#endif //CANVAS_ANDROID_GPUIMPL_H
