//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "gl.h"
#include <vector>
#include "Helpers.h"
#include "Common.h"
#include "Caches.h"

class WEBGL_compressed_texture_s3tcImpl {
public:
    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WEBGL_compressed_texture_s3tcTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WEBGL_compressed_texture_s3tc"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(1);

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_RGB_S3TC_DXT1_EXT"),
                  v8::Number::New(isolate, (double) GL_COMPRESSED_RGB_S3TC_DXT1_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_RGBA_S3TC_DXT1_EXT"),
                  v8::Number::New(isolate, (double) GL_COMPRESSED_RGBA_S3TC_DXT1_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_RGBA_S3TC_DXT3_EXT"),
                  v8::Number::New(isolate, (double) GL_COMPRESSED_RGBA_S3TC_DXT3_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_RGBA_S3TC_DXT5_EXT"),
                  v8::Number::New(isolate, (double) GL_COMPRESSED_RGBA_S3TC_DXT5_EXT));

        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "WEBGL_compressed_texture_s3tc"));

        cache->WEBGL_compressed_texture_s3tcTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, WEBGL_compressed_texture_s3tcImpl *textureS3Tc) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WEBGL_compressed_texture_s3tcImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::WEBGL_compressed_texture_s3tc);
        auto ext = v8::External::New(isolate, textureS3Tc);
        object->SetInternalField(0, ext);
        return scope.Escape(object);
    }

    static WEBGL_compressed_texture_s3tcImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WEBGL_compressed_texture_s3tcImpl *>(ptr);
    }
};