//
// Created by Osei Fortune on 01/07/2024.
//

#ifndef CANVAS_ANDROID_GPURENDERPASSENCODERIMPL_H
#define CANVAS_ANDROID_GPURENDERPASSENCODERIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPURenderPassEncoderImpl : ObjectWrapperImpl {
public:
    GPURenderPassEncoderImpl(CanvasGPURenderPass *pass);

    ~GPURenderPassEncoderImpl() {
    }

    CanvasGPURenderPass *GetPass();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPURenderPassEncoderImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, GPURenderPassEncoderImpl *encoder) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPURenderPassEncoderImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPURenderPassEncoder);
        object->SetAlignedPointerInInternalField(0, encoder);
        encoder->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }


    static void SetPipeline(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetVertexBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Draw(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void End(const v8::FunctionCallbackInfo<v8::Value> &args);


private:
    CanvasGPURenderPass *pass_;
};


#endif //CANVAS_ANDROID_GPURENDERPASSENCODERIMPL_H