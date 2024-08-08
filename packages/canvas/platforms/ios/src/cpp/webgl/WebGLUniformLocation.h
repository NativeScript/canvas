//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include "Helpers.h"
#include "Common.h"
#include "Caches.h"
#include "ObjectWrapperImpl.h"

class WebGLUniformLocation : ObjectWrapperImpl {
public:
    WebGLUniformLocation(int32_t
                         uniformLocation) :
            uniformLocation_(uniformLocation) {}


    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WebGLUniformLocationTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGLUniformLocation"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);

        cache->WebGLUniformLocationTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, WebGLUniformLocation *uniformLocation) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WebGLUniformLocation::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(uniformLocation, NativeType::WebGLUniformLocation);
        object->SetAlignedPointerInInternalField(0, uniformLocation);
        uniformLocation->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WebGLUniformLocation *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WebGLUniformLocation *>(ptr);
    }

    int32_t GetUniformLocation() {
        return this->uniformLocation_;
    }

private:
    int32_t uniformLocation_;
};
