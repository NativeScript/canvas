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
class WEBGL_compressed_texture_s3tc_srgbImpl: ObjectWrapperImpl {
public:
    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WEBGL_compressed_texture_s3tc_srgbTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WEBGL_compressed_texture_s3tc_srgb"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_SRGB_S3TC_DXT1_EXT"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COMPRESSED_SRGB_S3TC_DXT1_EXT));

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT));

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT));

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT));

        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "WEBGL_compressed_texture_s3tc_srgb"));

        cache->WEBGL_compressed_texture_s3tc_srgbTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate,
                WEBGL_compressed_texture_s3tc_srgbImpl *compressedTextureS3TcSrgb) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WEBGL_compressed_texture_s3tc_srgbImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( compressedTextureS3TcSrgb, NativeType::WEBGL_compressed_texture_s3tc_srgb);
        object->SetAlignedPointerInInternalField(0, compressedTextureS3TcSrgb);
        object->Set(context, ConvertToV8String(isolate, "ext_name"),
                    ConvertToV8String(isolate, "WEBGL_compressed_texture_s3tc_srgb"));
        compressedTextureS3TcSrgb->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WEBGL_compressed_texture_s3tc_srgbImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WEBGL_compressed_texture_s3tc_srgbImpl *>(ptr);
    }
};

