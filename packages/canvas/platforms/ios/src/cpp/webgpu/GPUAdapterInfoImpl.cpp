//
// Created by Osei Fortune on 17/06/2024.
//

#include "GPUAdapterInfoImpl.h"
#include "Caches.h"

GPUAdapterInfoImpl::GPUAdapterInfoImpl(CanvasGPUAdapterInfo *info) : info_(info) {}

CanvasGPUAdapterInfo *GPUAdapterInfoImpl::GetInfo() {
    return this->info_;
}


void GPUAdapterInfoImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUAdapterInfo"), func);
}

GPUAdapterInfoImpl *GPUAdapterInfoImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUAdapterInfoImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUAdapterInfoImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUAdapterInfoTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUAdapterInfo"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "architecture"),
            GetArchitecture
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "description"),
            GetDescription
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "device"),
            GetDevice
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "vendor"),
            GetVendor
    );


    cache->GPUAdapterInfoTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}


void
GPUAdapterInfoImpl::GetArchitecture(v8::Local<v8::Name> name,
                                    const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());

    if (ptr == nullptr) {
        info.GetReturnValue().SetEmptyString();
        return;
    }

    auto architecture = canvas_native_webgpu_adapter_info_architecture(ptr->GetInfo());

    if (architecture == nullptr) {
        info.GetReturnValue().SetEmptyString();
        return;
    }

    auto ret = ConvertToV8String(info.GetIsolate(), architecture);

    canvas_native_string_destroy(architecture);

    info.GetReturnValue().Set(ret);
}


void
GPUAdapterInfoImpl::GetDescription(v8::Local<v8::Name> name,
                                   const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());

    if (ptr == nullptr) {
        info.GetReturnValue().SetEmptyString();
        return;
    }

    auto description = canvas_native_webgpu_adapter_info_description(ptr->GetInfo());

    if (description == nullptr) {
        info.GetReturnValue().SetEmptyString();
        return;
    }

    auto ret = ConvertToV8String(info.GetIsolate(), description);

    canvas_native_string_destroy(description);

    info.GetReturnValue().Set(ret);
}


void
GPUAdapterInfoImpl::GetDevice(v8::Local<v8::Name> name,
                              const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());

    if (ptr == nullptr) {
        info.GetReturnValue().SetEmptyString();
        return;
    }

    auto device = canvas_native_webgpu_adapter_info_device(ptr->GetInfo());

    if (device == nullptr) {
        info.GetReturnValue().SetEmptyString();
        return;
    }

    auto ret = ConvertToV8String(info.GetIsolate(), device);

    canvas_native_string_destroy(device);

    info.GetReturnValue().Set(ret);
}


void
GPUAdapterInfoImpl::GetVendor(v8::Local<v8::Name> name,
                              const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());

    if (ptr == nullptr) {
        info.GetReturnValue().SetEmptyString();
        return;
    }

    auto vendor = canvas_native_webgpu_adapter_info_vendor(ptr->GetInfo());

    if (vendor == nullptr) {
        info.GetReturnValue().SetEmptyString();
        return;
    }

    auto ret = ConvertToV8String(info.GetIsolate(), vendor);

    canvas_native_string_destroy(vendor);

    info.GetReturnValue().Set(ret);
}

