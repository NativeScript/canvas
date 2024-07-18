//
// Created by Osei Fortune on 21/06/2024.
//

#ifndef CANVAS_ANDROID_GPUTEXTUREIMPL_H
#define CANVAS_ANDROID_GPUTEXTUREIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPUTextureImpl : ObjectWrapperImpl {
public:
    explicit GPUTextureImpl(const CanvasGPUTexture *texture);

    ~GPUTextureImpl() {
        canvas_native_webgpu_texture_release(this->GetTexture());
    }

    const CanvasGPUTexture *GetTexture();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUTextureImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPUTextureImpl *queue) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUTextureImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPUTexture);
        object->SetAlignedPointerInInternalField(0, queue);
        queue->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }



    static void GetWidth(v8::Local<v8::Name> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetHeight(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);


    static void GetDepthOrArrayLayers(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetDimension(v8::Local<v8::Name> name,
                                      const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetFormat(v8::Local<v8::Name> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetMipLevelCount(v8::Local<v8::Name> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetSampleCount(v8::Local<v8::Name> name,
                                 const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetUsage(v8::Local<v8::Name> name,
                               const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Destroy(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateView(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    const CanvasGPUTexture *texture_;
};


#endif //CANVAS_ANDROID_GPUTEXTUREIMPL_H
