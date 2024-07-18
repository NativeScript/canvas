//
// Created by Osei Fortune on 29/06/2024.
//

#ifndef CANVAS_ANDROID_GPUBINDGROUPLAYOUTIMPL_H
#define CANVAS_ANDROID_GPUBINDGROUPLAYOUTIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPUBindGroupLayoutImpl : ObjectWrapperImpl {
public:
    explicit GPUBindGroupLayoutImpl(const CanvasGPUBindGroupLayout *groupLayout);

    ~GPUBindGroupLayoutImpl() {
        canvas_native_webgpu_bind_group_layout_release(this->GetBindGroupLayout());
    }

    const CanvasGPUBindGroupLayout *GetBindGroupLayout();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUBindGroupLayoutImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, GPUBindGroupLayoutImpl *groupLayout) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUBindGroupLayoutImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPUBindGroupLayout);
        object->SetAlignedPointerInInternalField(0, groupLayout);
        groupLayout->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

private:
    const CanvasGPUBindGroupLayout *groupLayout_;
};


#endif //CANVAS_ANDROID_GPUBINDGROUPLAYOUTIMPL_H
