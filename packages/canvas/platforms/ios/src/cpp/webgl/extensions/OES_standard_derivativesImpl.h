//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "gl.h"
#include <vector>
#include "Caches.h"
#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"
class OES_standard_derivativesImpl: ObjectWrapperImpl {
public:
    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->OES_standard_derivativesTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "OES_standard_derivatives"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);
        tmpl->Set(ConvertToV8String(isolate, "FRAGMENT_SHADER_DERIVATIVE_HINT_OES"),
                  v8::Integer::NewFromUnsigned(isolate, GL_FRAGMENT_SHADER_DERIVATIVE_HINT_OES));
        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "OES_standard_derivatives"));
        cache->OES_standard_derivativesTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, OES_standard_derivativesImpl *derivatives) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = OES_standard_derivativesImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( derivatives, NativeType::OES_standard_derivatives);
        object->SetAlignedPointerInInternalField(0, derivatives);
        derivatives->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static OES_standard_derivativesImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<OES_standard_derivativesImpl *>(ptr);
    }
};
