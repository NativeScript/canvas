//
// Created by Osei Fortune on 17/06/2024.
//

#ifndef CANVAS_ANDROID_GPUADAPTERINFOIMPL_H
#define CANVAS_ANDROID_GPUADAPTERINFOIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPUAdapterInfoImpl : ObjectWrapperImpl {
public:
    GPUAdapterInfoImpl(CanvasGPUAdapterInfo *info);

    ~GPUAdapterInfoImpl() {
        if (this->info_ != nullptr) {
            canvas_native_webgpu_adapter_info_destroy(this->info_);
            this->info_ = nullptr;
        }
    }

    CanvasGPUAdapterInfo *GetInfo();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUAdapterInfoImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPUAdapterInfoImpl *info) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUAdapterInfoImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPUAdapterInfo);
        object->SetAlignedPointerInInternalField(0, info);
        info->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void GetArchitecture(v8::Local<v8::Name> name,
                                const v8::PropertyCallbackInfo<v8::Value> &info);


    static void GetDescription(v8::Local<v8::Name> name,
                               const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetDevice(v8::Local<v8::Name> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetVendor(v8::Local<v8::Name> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info);


private:
    CanvasGPUAdapterInfo *info_;
};


#endif //CANVAS_ANDROID_GPUADAPTERINFOIMPL_H
