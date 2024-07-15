//
// Created by Osei Fortune on 23/06/2024.
//

#ifndef CANVAS_ANDROID_GPUCOMPUTEPASSENCODERIMPL_H
#define CANVAS_ANDROID_GPUCOMPUTEPASSENCODERIMPL_H

#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPUComputePassEncoderImpl : ObjectWrapperImpl {
public:
    GPUComputePassEncoderImpl(const CanvasGPUComputePassEncoder *computePass);

    ~GPUComputePassEncoderImpl() {
        canvas_native_webgpu_compute_pass_release(this->GetComputePass());
    }

    const CanvasGPUComputePassEncoder *GetComputePass();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUComputePassEncoderImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, GPUComputePassEncoderImpl *pass) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUComputePassEncoderImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPUComputePass);
        object->SetAlignedPointerInInternalField(0, pass);
        pass->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }


private:
    const CanvasGPUComputePassEncoder *computePass_;
};


#endif //CANVAS_ANDROID_GPUCOMPUTEPASSENCODERIMPL_H
