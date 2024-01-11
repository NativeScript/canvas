//
// Created by Osei Fortune on 27/04/2022.
//

#include "WebGLActiveInfoImpl.h"
#include "Caches.h"
#include "Helpers.h"

WebGLActiveInfoImpl::WebGLActiveInfoImpl(WebGLActiveInfo* info) : info_(info) {}

WebGLActiveInfoImpl *WebGLActiveInfoImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<WebGLActiveInfoImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> WebGLActiveInfoImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->WebGLActiveInfoTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGLActiveInfo"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);
    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "name"),
            GetName);
    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "size"),
            GetSize);
    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "type"),
            GetType);

    cache->WebGLActiveInfoTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void
WebGLActiveInfoImpl::GetName(v8::Local<v8::Name> name,
                             const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto isolate = info.GetIsolate();
    if (ptr != nullptr) {
        auto info_name = canvas_native_webgl_active_info_get_name(ptr->GetWebGLActiveInfo());
        auto returnValue = new OneByteStringResource((char *)info_name);
        auto ret = v8::String::NewExternalOneByte(isolate, returnValue);
        info.GetReturnValue().Set(ret.ToLocalChecked());
        return;
    }
    info.GetReturnValue().SetEmptyString();
}

void
WebGLActiveInfoImpl::GetSize(v8::Local<v8::Name> name,
                             const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto size = canvas_native_webgl_active_info_get_size(ptr->GetWebGLActiveInfo());
        info.GetReturnValue().Set(size);
        return;
    }
    info.GetReturnValue().Set(0);
}


void
WebGLActiveInfoImpl::GetType(v8::Local<v8::Name> name,
                             const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto type = canvas_native_webgl_active_info_get_type(ptr->GetWebGLActiveInfo());
        info.GetReturnValue().Set(type);
        return;
    }
    info.GetReturnValue().Set(0);
}


WebGLActiveInfo* WebGLActiveInfoImpl::GetWebGLActiveInfo() {
    return this->info_;
}

