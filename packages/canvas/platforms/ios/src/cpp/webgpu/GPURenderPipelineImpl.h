//
// Created by Osei Fortune on 29/06/2024.
//

#ifndef CANVAS_ANDROID_GPURENDERPIPELINEIMPL_H
#define CANVAS_ANDROID_GPURENDERPIPELINEIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPURenderPipelineImpl : ObjectWrapperImpl {
public:
    explicit GPURenderPipelineImpl(const CanvasGPURenderPipeline *pipeline);

    ~GPURenderPipelineImpl() {
        canvas_native_webgpu_render_pipeline_release(this->pipeline_);
    }

    const CanvasGPURenderPipeline *GetGPUPipeline();

    static void Init(v8::Local <v8::Object> canvasModule, v8::Isolate *isolate);

    static GPURenderPipelineImpl *GetPointer(const v8::Local <v8::Object> &object);

    static v8::Local <v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local <v8::Object>
    NewInstance(v8::Isolate *isolate, GPURenderPipelineImpl *pipeline) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPURenderPipelineImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(pipeline, NativeType::GPURenderPipeline);
        object->SetAlignedPointerInInternalField(0, pipeline);
        pipeline->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }


    static void GetBindGroupLayout(const v8::FunctionCallbackInfo <v8::Value> &args);


private:
    const CanvasGPURenderPipeline *pipeline_;
};


#endif //CANVAS_ANDROID_GPURENDERPIPELINEIMPL_H
