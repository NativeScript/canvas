//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "gl.h"
#include <vector>
#include "Helpers.h"
#include "Common.h"
#include "Caches.h"

class EXT_texture_filter_anisotropicImpl {
public:
    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->EXT_texture_filter_anisotropicTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "EXT_texture_filter_anisotropic"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(1);
        tmpl->Set(ConvertToV8String(isolate, "MAX_TEXTURE_MAX_ANISOTROPY_EXT"),
                  v8::Number::New(isolate, (double) GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT));
        tmpl->Set(ConvertToV8String(isolate, "TEXTURE_MAX_ANISOTROPY_EXT"),
                  v8::Number::New(isolate, (double) GL_TEXTURE_MAX_ANISOTROPY_EXT));
        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "EXT_texture_filter_anisotropic"));

        cache->EXT_texture_filter_anisotropicTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, EXT_texture_filter_anisotropicImpl *filterAnisotropic) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = EXT_texture_filter_anisotropicImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::EXT_texture_filter_anisotropic);
        auto ext = v8::External::New(isolate, filterAnisotropic);
        object->SetInternalField(0, ext);
        return scope.Escape(object);
    }

    static EXT_texture_filter_anisotropicImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<EXT_texture_filter_anisotropicImpl *>(ptr);
    }
};
