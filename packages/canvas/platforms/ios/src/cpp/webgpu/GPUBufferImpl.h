//
// Created by Osei Fortune on 18/06/2024.
//

#ifndef CANVAS_ANDROID_GPUBUFFERIMPL_H
#define CANVAS_ANDROID_GPUBUFFERIMPL_H

#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPUBufferImpl : ObjectWrapperImpl {
public:
    GPUBufferImpl(const CanvasGPUBuffer *buffer);

    ~GPUBufferImpl() {
    }

    const CanvasGPUBuffer *GetGPUBuffer();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUBufferImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPUBufferImpl *adapter) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUBufferImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPUBuffer);
        object->SetAlignedPointerInInternalField(0, adapter);
        adapter->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void GetUsage(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetSize(v8::Local<v8::Name> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info);


    static void Destroy(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void UnMap(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void MapAsync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetMappedRange(const v8::FunctionCallbackInfo<v8::Value> &args);


private:
    const CanvasGPUBuffer *buffer_;
};


#endif //CANVAS_ANDROID_GPUBUFFERIMPL_H
