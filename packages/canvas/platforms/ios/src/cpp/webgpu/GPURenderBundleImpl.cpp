//
// Created by Osei Fortune on 18/07/2024.
//

#include "GPURenderBundleImpl.h"
#include "Caches.h"

GPURenderBundleImpl::GPURenderBundleImpl(const CanvasGPURenderBundle *bundle) : bundle_(
        bundle) {}

const CanvasGPURenderBundle *GPURenderBundleImpl::GetBundle() {
    return this->bundle_;
}


void GPURenderBundleImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPURenderBundle"), func).FromJust();;
}

GPURenderBundleImpl *GPURenderBundleImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPURenderBundleImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPURenderBundleImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPURenderBundleTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPURenderBundle"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    cache->GPURenderBundleTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}
