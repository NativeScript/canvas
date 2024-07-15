//
// Created by Osei Fortune on 01/07/2024.
//

#ifndef CANVAS_ANDROID_GPUTEXTUREVIEWIMPL_H
#define CANVAS_ANDROID_GPUTEXTUREVIEWIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPUTextureViewImpl : ObjectWrapperImpl {
public:
    GPUTextureViewImpl(const CanvasGPUTextureView *view);

    ~GPUTextureViewImpl() {
        canvas_native_webgpu_texture_view_release(this->GetTextureView());
    }

    const CanvasGPUTextureView *GetTextureView();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUTextureViewImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPUTextureViewImpl *view) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUTextureViewImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPUTextureView);
        object->SetAlignedPointerInInternalField(0, view);
        view->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }


private:
    const CanvasGPUTextureView *view_;
};


#endif //CANVAS_ANDROID_GPUTEXTUREVIEWIMPL_H
