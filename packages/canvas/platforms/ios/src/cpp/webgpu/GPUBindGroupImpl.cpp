//
// Created by Osei Fortune on 17/07/2024.
//

#include "GPUBindGroupImpl.h"
#include "Caches.h"

GPUBindGroupImpl::GPUBindGroupImpl(const CanvasGPUBindGroup *groupLayout)
        : group_(groupLayout) {}

const CanvasGPUBindGroup *GPUBindGroupImpl::GetBindGroup() {
    return this->group_;
}


void GPUBindGroupImpl::Init(v8::Local <v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUBindGroup"), func).FromJust();;
}

GPUBindGroupImpl *GPUBindGroupImpl::GetPointer(const v8::Local <v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUBindGroupImpl *>(ptr);
}

v8::Local <v8::FunctionTemplate> GPUBindGroupImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUBindGroupTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local <v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUBindGroup"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    cache->GPUBindGroupTmpl =
            std::make_unique < v8::Persistent < v8::FunctionTemplate >> (isolate, ctorTmpl);
    return ctorTmpl;
}
