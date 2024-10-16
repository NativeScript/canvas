//
// Created by Osei Fortune on 18/06/2024.
//

#ifndef CANVAS_ANDROID_GPUQUEUEIMPL_H
#define CANVAS_ANDROID_GPUQUEUEIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"
#include "GPUUtils.h"

class GPUQueueImpl : ObjectWrapperImpl {
public:
    explicit GPUQueueImpl(const CanvasGPUQueue *queue);

    ~GPUQueueImpl() {
        canvas_native_webgpu_queue_release(this->GetGPUQueue());
    }

    const CanvasGPUQueue *GetGPUQueue();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUQueueImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPUQueueImpl *queue) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUQueueImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(queue, NativeType::GPUQueue);
        object->SetAlignedPointerInInternalField(0, queue);
        queue->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void GetLabel(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void CopyExternalImageToTexture(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Submit(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SubmitWorkDone(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteTexture(const v8::FunctionCallbackInfo<v8::Value> &args);


private:
    const CanvasGPUQueue *queue_;
};


#endif //CANVAS_ANDROID_GPUQUEUEIMPL_H
