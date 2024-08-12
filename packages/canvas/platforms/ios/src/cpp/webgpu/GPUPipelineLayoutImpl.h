//
// Created by Osei Fortune on 26/06/2024.
//

#ifndef CANVAS_ANDROID_GPUPIPELINELAYOUTIMPL_H
#define CANVAS_ANDROID_GPUPIPELINELAYOUTIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"


class GPUPipelineLayoutImpl : ObjectWrapperImpl {
public:
    explicit GPUPipelineLayoutImpl(const CanvasGPUPipelineLayout *pipeline);

    ~GPUPipelineLayoutImpl() {
        canvas_native_webgpu_pipeline_layout_release(this->GetPipeline());
    }

    const CanvasGPUPipelineLayout *GetPipeline();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUPipelineLayoutImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, GPUPipelineLayoutImpl *pipeline) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUPipelineLayoutImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(pipeline, NativeType::GPUPipelineLayout);
        object->SetAlignedPointerInInternalField(0, pipeline);
        pipeline->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }


private:
    const CanvasGPUPipelineLayout *pipeline_;
};


#endif //CANVAS_ANDROID_GPUPIPELINELAYOUTIMPL_H
