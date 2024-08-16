//
// Created by Osei Fortune on 23/06/2024.
//

#ifndef CANVAS_ANDROID_GPUCOMPUTEPASSENCODERIMPL_H
#define CANVAS_ANDROID_GPUCOMPUTEPASSENCODERIMPL_H

#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPUComputePassEncoderImpl : ObjectWrapperImpl {
public:
    explicit GPUComputePassEncoderImpl(const CanvasGPUComputePassEncoder *computePass);

    ~GPUComputePassEncoderImpl() {
        canvas_native_webgpu_compute_pass_encoder_release(this->computePass_);
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
        SetNativeType(pass, NativeType::GPUComputePass);
        object->SetAlignedPointerInInternalField(0, pass);
        pass->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void GetLabel(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void DispatchWorkgroups(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DispatchWorkgroupsIndirect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void End(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void InsertDebugMarker(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void PopDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void PushDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetBindGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetPipeline(const v8::FunctionCallbackInfo<v8::Value> &args);



private:
    const CanvasGPUComputePassEncoder *computePass_;
};


#endif //CANVAS_ANDROID_GPUCOMPUTEPASSENCODERIMPL_H
