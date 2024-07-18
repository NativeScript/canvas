//
// Created by Osei Fortune on 17/07/2024.
//

#ifndef CANVAS_ANDROID_GPUBINDGROUPIMPL_H
#define CANVAS_ANDROID_GPUBINDGROUPIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPUBindGroupImpl : ObjectWrapperImpl {
public:
    GPUBindGroupImpl(const CanvasGPUBindGroup *group);

    ~GPUBindGroupImpl() {
        canvas_native_webgpu_bind_group_release(this->GetBindGroup());
    }

    const CanvasGPUBindGroup *GetBindGroup();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUBindGroupImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, GPUBindGroupImpl *groupLayout) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUBindGroupImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPUBindGroup);
        object->SetAlignedPointerInInternalField(0, groupLayout);
        groupLayout->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

private:
    const CanvasGPUBindGroup *group_;
};


#endif //CANVAS_ANDROID_GPUBINDGROUPIMPL_H
