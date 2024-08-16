//
// Created by Osei Fortune on 29/06/2024.
//

#include "GPUBindGroupLayoutImpl.h"
#include "Caches.h"

GPUBindGroupLayoutImpl::GPUBindGroupLayoutImpl(const CanvasGPUBindGroupLayout *groupLayout)
        : groupLayout_(groupLayout) {}

const CanvasGPUBindGroupLayout *GPUBindGroupLayoutImpl::GetBindGroupLayout() {
    return this->groupLayout_;
}


void GPUBindGroupLayoutImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUBindGroupLayout"), func).FromJust();
}

GPUBindGroupLayoutImpl *GPUBindGroupLayoutImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUBindGroupLayoutImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUBindGroupLayoutImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUBindGroupLayoutTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUBindGroupLayout"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "label"),
            GetLabel
    );

    cache->GPUBindGroupLayoutTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}


void
GPUBindGroupLayoutImpl::GetLabel(v8::Local<v8::Name> name,
                           const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto label = canvas_native_webgpu_bind_group_layout_get_label(ptr->groupLayout_);
        if (label == nullptr) {
            info.GetReturnValue().SetEmptyString();
            return;
        }
        info.GetReturnValue().Set(
                ConvertToV8String(info.GetIsolate(), label)
        );
        canvas_native_string_destroy(label);
        return;
    }

    info.GetReturnValue().SetEmptyString();
}
