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
class WEBGL_compressed_texture_etcImpl: ObjectWrapperImpl {
public:
    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WEBGL_compressed_texture_etcTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WEBGL_compressed_texture_etc"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(1);

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_R11_EAC"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COMPRESSED_R11_EAC));

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_SIGNED_R11_EAC"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COMPRESSED_SIGNED_R11_EAC));

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_RG11_EAC"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COMPRESSED_RG11_EAC));

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_SIGNED_RG11_EAC"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COMPRESSED_SIGNED_RG11_EAC));

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_RGB8_ETC2"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COMPRESSED_RGB8_ETC2));

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_RGBA8_ETC2_EAC"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COMPRESSED_RGBA8_ETC2_EAC));

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_SRGB8_ETC2"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COMPRESSED_SRGB8_ETC2));

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_SRGB8_ALPHA8_ETC2_EAC"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC));

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2));

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2));

        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "WEBGL_compressed_texture_etc"));

        cache->WEBGL_compressed_texture_etcTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, WEBGL_compressed_texture_etcImpl *compressedTextureEtc) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WEBGL_compressed_texture_etcImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::WEBGL_compressed_texture_etc);
        auto ext = v8::External::New(isolate, compressedTextureEtc);
        object->SetInternalField(0, ext);
        object->Set(context, ConvertToV8String(isolate, "ext_name"),
                    ConvertToV8String(isolate, "WEBGL_compressed_texture_etc"));
        compressedTextureEtc->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WEBGL_compressed_texture_etcImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WEBGL_compressed_texture_etcImpl *>(ptr);
    }
};
