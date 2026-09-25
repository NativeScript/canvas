//
// Created by Osei Fortune on 24/09/2026.
//

#ifndef CANVAS_ANDROID_GPUEXTERNALTEXTUREIMPL_H
#define CANVAS_ANDROID_GPUEXTERNALTEXTUREIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"
#include "ArcHandle.h"

class GPUExternalTextureImpl : ObjectWrapperImpl {
public:
    explicit GPUExternalTextureImpl(const CanvasGPUExternalTexture *texture);

    const CanvasGPUExternalTexture *GetExternalTexture();

    static GPUExternalTextureImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPUExternalTextureImpl *texture) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUExternalTextureImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(texture, NativeType::GPUExternalTexture);
        object->SetAlignedPointerInInternalField(0, texture, ObjectWrapperImpl::kInternalFieldTag);
        texture->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void GetLabel(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

private:
    ArcHandle<CanvasGPUExternalTexture, canvas_native_webgpu_external_texture_release> texture_;
};


#endif //CANVAS_ANDROID_GPUEXTERNALTEXTUREIMPL_H
