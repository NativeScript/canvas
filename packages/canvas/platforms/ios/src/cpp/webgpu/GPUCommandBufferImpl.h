//
// Created by Osei Fortune on 01/07/2024.
//

#ifndef CANVAS_ANDROID_GPUCOMMANDBUFFERIMPL_H
#define CANVAS_ANDROID_GPUCOMMANDBUFFERIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPUCommandBufferImpl : ObjectWrapperImpl {
public:
    GPUCommandBufferImpl(const CanvasGPUCommandBuffer *commandBuffer);

    ~GPUCommandBufferImpl() {
    }

    const CanvasGPUCommandBuffer *GetGPUCommandBuffer();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUCommandBufferImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, GPUCommandBufferImpl *commandBuffer) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUCommandBufferImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPUCommandBuffer);
        object->SetAlignedPointerInInternalField(0, commandBuffer);
        commandBuffer->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

private:
    const CanvasGPUCommandBuffer *commandBuffer_;
};


#endif //CANVAS_ANDROID_GPUCOMMANDBUFFERIMPL_H
