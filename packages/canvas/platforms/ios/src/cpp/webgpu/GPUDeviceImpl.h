//
// Created by Osei Fortune on 18/06/2024.
//

#ifndef CANVAS_ANDROID_GPUDEVICEIMPL_H
#define CANVAS_ANDROID_GPUDEVICEIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"
#include "GPUBufferImpl.h"

class GPUDeviceImpl : ObjectWrapperImpl {

public:
    GPUDeviceImpl(CanvasGPUDevice *device);

    ~GPUDeviceImpl() {
        if(this->device_ != nullptr){
            canvas_native_webgpu_device_destroy_destory(this->device_);
            this->device_ = nullptr;
        }
    }

    CanvasGPUDevice *GetGPUDevice();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUDeviceImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPUDeviceImpl *device) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUDeviceImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPUDevice);
        object->SetAlignedPointerInInternalField(0, device);
        device->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }


    static void GetFeatures(v8::Local<v8::Name> name,
                               const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetLimits(v8::Local<v8::Name> name,
                            const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetQueue(v8::Local<v8::Name> name,
                            const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetLost(v8::Local<v8::Name> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Destroy(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    CanvasGPUDevice *device_;
};


#endif //CANVAS_ANDROID_GPUDEVICEIMPL_H
