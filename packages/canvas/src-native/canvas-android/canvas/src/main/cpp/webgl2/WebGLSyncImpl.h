//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include "Common.h"
#include "Caches.h"

using namespace org::nativescript::canvas;

class WebGLSyncImpl {
public:
    WebGLSyncImpl(rust::Box<WebGLSync> sync) : sync_(std::move(sync)) {}

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WebGLSyncTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGLSync"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(1);

        cache->WebGLSyncTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static WebGLSyncImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WebGLSyncImpl *>(ptr);
    }

    WebGLSync &GetSync() {
        return *this->sync_;
    }

private:
    rust::Box<WebGLSync> sync_;
};
