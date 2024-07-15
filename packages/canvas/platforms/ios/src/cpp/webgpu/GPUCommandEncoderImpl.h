//
// Created by Osei Fortune on 23/06/2024.
//

#ifndef CANVAS_ANDROID_GPUCOMMANDENCODERIMPL_H
#define CANVAS_ANDROID_GPUCOMMANDENCODERIMPL_H

#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPUCommandEncoderImpl : ObjectWrapperImpl {
public:
    GPUCommandEncoderImpl(const CanvasGPUCommandEncoder *encoder);

    ~GPUCommandEncoderImpl() {
        canvas_native_webgpu_command_encoder_release(this->GetEncoder());
    }

    const CanvasGPUCommandEncoder *GetEncoder();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUCommandEncoderImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPUCommandEncoderImpl *encoder) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUCommandEncoderImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPUCommandEncoder);
        object->SetAlignedPointerInInternalField(0, encoder);
        encoder->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void BeginComputePass(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BeginRenderPass(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ClearBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Finish(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    const CanvasGPUCommandEncoder *encoder_;
};


#endif //CANVAS_ANDROID_GPUCOMMANDENCODERIMPL_H
