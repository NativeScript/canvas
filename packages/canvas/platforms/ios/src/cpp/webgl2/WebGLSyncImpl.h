//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once
#include "Common.h"
#include "Caches.h"
#include "ObjectWrapperImpl.h"

class WebGLSyncImpl: ObjectWrapperImpl {
public:
    WebGLSyncImpl(WebGLSync* sync) : sync_(sync) {}

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WebGLSyncTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGLSync"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);

        cache->WebGLSyncTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, WebGLSyncImpl *sync) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WebGLSyncImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( object, NativeType::WebGLSync);
        object->SetAlignedPointerInInternalField(0, sync);
        sync->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WebGLSyncImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WebGLSyncImpl *>(ptr);
    }

    WebGLSync * GetSync() {
        return this->sync_;
    }

private:
    WebGLSync* sync_;
};
