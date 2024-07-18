//
// Created by Osei Fortune on 17/07/2024.
//

#ifndef CANVAS_ANDROID_GPUCOMPUTEPIPELINEIMPL_H
#define CANVAS_ANDROID_GPUCOMPUTEPIPELINEIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPUComputePipelineImpl : ObjectWrapperImpl {
public:
    explicit GPUComputePipelineImpl(const CanvasGPUComputePipeline *pipeline);

    ~GPUComputePipelineImpl() {
        canvas_native_webgpu_compute_pipeline_release(this->pipeline_);
    }

    const CanvasGPUComputePipeline *GetGPUPipeline();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUComputePipelineImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, GPUComputePipelineImpl *pipeline) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUComputePipelineImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPUComputePipeline);
        object->SetAlignedPointerInInternalField(0, pipeline);
        pipeline->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }


    static void GetBindGroupLayout(const v8::FunctionCallbackInfo<v8::Value> &args);


private:
    const CanvasGPUComputePipeline *pipeline_;
};


#endif //CANVAS_ANDROID_GPUCOMPUTEPIPELINEIMPL_H
