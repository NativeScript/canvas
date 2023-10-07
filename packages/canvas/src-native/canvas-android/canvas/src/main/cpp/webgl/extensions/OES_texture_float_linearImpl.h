//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "Helpers.h"
#include "Common.h"

class  OES_texture_float_linearImpl {
public:

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->OES_texture_float_linearTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "OES_texture_float_linear"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(1);
        tmpl->Set(ConvertToV8String(isolate, "ext_name"), ConvertToV8String(isolate, "OES_texture_float_linear"));

        cache->OES_texture_float_linearTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, OES_texture_float_linearImpl *texture) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = OES_texture_float_linearImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::OES_texture_float_linear);
        auto ext = v8::External::New(isolate, texture);
        object->SetInternalField(0, ext);
        return scope.Escape(object);
    }

    static OES_texture_float_linearImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<OES_texture_float_linearImpl *>(ptr);
    }

};
