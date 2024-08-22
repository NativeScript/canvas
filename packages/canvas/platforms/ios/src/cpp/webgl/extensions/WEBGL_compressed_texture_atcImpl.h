//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "gl.h"
#include <vector>
#include "Helpers.h"
#include "Common.h"
#include "Caches.h"
#include "ObjectWrapperImpl.h"
class WEBGL_compressed_texture_atcImpl: ObjectWrapperImpl {
public:
    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WEBGL_compressed_texture_atcTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WEBGL_compressed_texture_atc"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);
        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "WEBGL_compressed_texture_atc"));
        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_RGB_ATC_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_ATC_RGB_AMD));
        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate,  GL_ATC_RGBA_EXPLICIT_ALPHA_AMD));
        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_ATC_RGBA_INTERPOLATED_ALPHA_AMD));

        cache->WEBGL_compressed_texture_atcTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, WEBGL_compressed_texture_atcImpl *compressedTextureAtc) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WEBGL_compressed_texture_atcImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( compressedTextureAtc, NativeType::WEBGL_compressed_texture_atc);
        object->SetAlignedPointerInInternalField(0, compressedTextureAtc);
        compressedTextureAtc->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WEBGL_compressed_texture_atcImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WEBGL_compressed_texture_atcImpl *>(ptr);
    }
};
