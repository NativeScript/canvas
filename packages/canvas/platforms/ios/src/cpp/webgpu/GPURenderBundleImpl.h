//
// Created by Osei Fortune on 18/07/2024.
//

#ifndef CANVAS_ANDROID_GPURENDERBUNDLEIMPL_H
#define CANVAS_ANDROID_GPURENDERBUNDLEIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPURenderBundleImpl : ObjectWrapperImpl {
public:
    explicit GPURenderBundleImpl(const CanvasGPURenderBundle *bundle);

    ~GPURenderBundleImpl() {
        canvas_native_webgpu_render_bundle_release(this->bundle_);
    }

    const CanvasGPURenderBundle *GetBundle();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPURenderBundleImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPURenderBundleImpl *bundle) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPURenderBundleImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPURenderBundle);
        object->SetAlignedPointerInInternalField(0, bundle);
        bundle->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }


private:
    const CanvasGPURenderBundle *bundle_;
};


#endif //CANVAS_ANDROID_GPURENDERBUNDLEIMPL_H
