//
// Created by Osei Fortune on 14/06/2025.
//

#include "GPUCompilationInfoImpl.h"
#include "Caches.h"
#include "GPUCompilationMessageImpl.h"

GPUCompilationInfoImpl::GPUCompilationInfoImpl(CanvasGPUCompilationInfo *info) : info_(
        info) {}

CanvasGPUCompilationInfo *GPUCompilationInfoImpl::GetCompilationInfo() {
    return this->info_;
}


void GPUCompilationInfoImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUCompilationInfo"), func).FromJust();
}

GPUCompilationInfoImpl *GPUCompilationInfoImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUCompilationInfoImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUCompilationInfoImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUCompilationInfoTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUCompilationInfo"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "messages"),
            GetMessages
    );


    cache->GPUCompilationInfoTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}


void
GPUCompilationInfoImpl::GetMessages(v8::Local<v8::Name> name,
                                    const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto count = canvas_native_webgpu_compilation_info_get_messages_count(
                ptr->GetCompilationInfo());
        auto array = v8::Array::New(isolate, (int) count);
        for (int i = 0; i < count; i++) {
            auto msg = canvas_native_webgpu_compilation_info_get_message_at(
                    ptr->GetCompilationInfo(), i);
            if (msg != nullptr) {
                auto ret = new GPUCompilationMessageImpl(msg);
                array->Set(context, i, GPUCompilationMessageImpl::NewInstance(isolate, ret));
            }
        }

        info.GetReturnValue().Set(array);

        return;
    }

    info.GetReturnValue().Set(v8::Array::New(isolate));
}
