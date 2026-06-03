//
// Created by Osei Fortune on 14/06/2025.
//

#ifndef CANVAS_ANDROID_GPUCOMPILATIONMESSAGEIMPL_H
#define CANVAS_ANDROID_GPUCOMPILATIONMESSAGEIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"
#include "ArcHandle.h"

class GPUCompilationMessageImpl : ObjectWrapperImpl {
public:
    explicit GPUCompilationMessageImpl(CanvasGPUCompilationMessage *message);

    ~GPUCompilationMessageImpl() = default;

    CanvasGPUCompilationMessage *GetMessage();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUCompilationMessageImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, GPUCompilationMessageImpl *message) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUCompilationMessageImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(message, NativeType::GPUCompilationMessage);
        object->SetAlignedPointerInInternalField(0, message);
        message->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void GetLength(v8::Local<v8::Name> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetLineNum(v8::Local<v8::Name> name,
                           const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetLinePos(v8::Local<v8::Name> name,
                           const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetMessage(v8::Local<v8::Name> name,
                           const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetOffset(v8::Local<v8::Name> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetType(v8::Local<v8::Name> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info);


private:
    MutArcHandle<CanvasGPUCompilationMessage, canvas_native_webgpu_compilation_message_release> message_;
};


#endif //CANVAS_ANDROID_GPUCOMPILATIONMESSAGEIMPL_H
