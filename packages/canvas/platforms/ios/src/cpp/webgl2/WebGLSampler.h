//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once

#include "Caches.h"
#include "Common.h"
#include "ObjectWrapperImpl.h"

class WebGLSampler: ObjectWrapperImpl {
public:
    WebGLSampler(uint32_t sampler) : sampler_(sampler) {}

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WebGLSamplerTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGLSampler"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);

        cache->WebGLSamplerTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, WebGLSampler *sampler) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WebGLSampler::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( sampler, NativeType::WebGLSampler);
        object->SetAlignedPointerInInternalField(0, sampler);
        sampler->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WebGLSampler *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WebGLSampler *>(ptr);
    }

    uint32_t GetSampler() {
        return this->sampler_;
    }

private:
    uint32_t sampler_;
};

