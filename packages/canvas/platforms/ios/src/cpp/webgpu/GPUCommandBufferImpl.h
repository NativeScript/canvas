//
// Created by Osei Fortune on 01/07/2024.
//

#ifndef CANVAS_ANDROID_GPUCOMMANDBUFFERIMPL_H
#define CANVAS_ANDROID_GPUCOMMANDBUFFERIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"
#include "ArcHandle.h"

class GPUCommandBufferImpl : ObjectWrapperImpl {
public:
    explicit GPUCommandBufferImpl(const CanvasGPUCommandBuffer *commandBuffer);

    // Deterministic dispose, called from Destroy() at queue.submit(). See ArcHandle.h.
    void Release() { commandBuffer_.reset(); }

    const CanvasGPUCommandBuffer *GetGPUCommandBuffer();

    static void Destroy(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUCommandBufferImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, GPUCommandBufferImpl *commandBuffer) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUCommandBufferImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(commandBuffer, NativeType::GPUCommandBuffer);
        object->SetAlignedPointerInInternalField(0, commandBuffer);
        commandBuffer->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void GetLabel(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

private:
    ArcHandle<CanvasGPUCommandBuffer, canvas_native_webgpu_command_buffer_release> commandBuffer_;
};


#endif //CANVAS_ANDROID_GPUCOMMANDBUFFERIMPL_H
