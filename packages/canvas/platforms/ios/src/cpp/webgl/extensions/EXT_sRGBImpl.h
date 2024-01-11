//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "gl.h"
#include <vector>
#include "Helpers.h"
#include "Caches.h"
#include "ObjectWrapperImpl.h"
class EXT_sRGBImpl: ObjectWrapperImpl {
public:
    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->EXT_sRGBTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "EXT_sRGB"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(1);
        tmpl->Set(ConvertToV8String(isolate, "SRGB_EXT"),
                  v8::Integer::NewFromUnsigned(isolate, GL_SRGB_EXT));
        tmpl->Set(ConvertToV8String(isolate, "SRGB_ALPHA_EXT"),
                  v8::Integer::NewFromUnsigned(isolate, GL_SRGB_ALPHA_EXT));
        tmpl->Set(ConvertToV8String(isolate, "SRGB8_ALPHA8_EXT"),
                  v8::Integer::NewFromUnsigned(isolate, GL_SRGB8_ALPHA8_EXT));
        tmpl->Set(ConvertToV8String(isolate, "FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT"),
                  v8::Integer::NewFromUnsigned(isolate, GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT));
        tmpl->Set(ConvertToV8String(isolate, "ext_name"), ConvertToV8String(isolate, "EXT_sRGB"));

        cache->EXT_sRGBTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, EXT_sRGBImpl *extSrgb) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = EXT_sRGBImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::EXT_sRGB);
        auto ext = v8::External::New(isolate, extSrgb);
        object->SetInternalField(0, ext);
        extSrgb->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static EXT_sRGBImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<EXT_sRGBImpl *>(ptr);
    }
};
