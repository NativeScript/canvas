//
// Created by Osei Fortune on 17/06/2024.
//

#ifndef CANVAS_ANDROID_GPUADAPTERIMPL_H
#define CANVAS_ANDROID_GPUADAPTERIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"
#include "GPUSupportedLimitsImpl.h"
#include "GPUDeviceImpl.h"

class GPUAdapterImpl : ObjectWrapperImpl {
public:
    GPUAdapterImpl(CanvasGPUAdapter *adapter);

    ~GPUAdapterImpl() {
    }

    CanvasGPUAdapter *GetGPUAdapter();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUAdapterImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPUAdapterImpl *adapter) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUAdapterImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPUAdapter);
        object->SetAlignedPointerInInternalField(0, adapter);
        adapter->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void GetFeatures(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);


    static void GetIsFallbackAdapter(v8::Local<v8::Name> name,
                            const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetLimits(v8::Local<v8::Name> name,
                            const v8::PropertyCallbackInfo<v8::Value> &info);


    static void RequestAdapterInfo(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RequestDevice(const v8::FunctionCallbackInfo<v8::Value> &args);


private:
    CanvasGPUAdapter *adapter_;
};


#endif //CANVAS_ANDROID_GPUADAPTERIMPL_H
