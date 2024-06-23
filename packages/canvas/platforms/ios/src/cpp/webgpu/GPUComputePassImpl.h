//
// Created by Osei Fortune on 23/06/2024.
//

#ifndef CANVAS_ANDROID_GPUCOMPUTEPASSIMPL_H
#define CANVAS_ANDROID_GPUCOMPUTEPASSIMPL_H

#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPUComputePassImpl : ObjectWrapperImpl {
public:
    GPUComputePassImpl(CanvasGPUComputePass *computePass);

    ~GPUComputePassImpl() {
    }

    CanvasGPUComputePass *GetComputePass();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUComputePassImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPUComputePassImpl *pass) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUComputePassImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPUComputePass);
        object->SetAlignedPointerInInternalField(0, pass);
        pass->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }


private:
    CanvasGPUComputePass *computePass_;
};


#endif //CANVAS_ANDROID_GPUCOMPUTEPASSIMPL_H
