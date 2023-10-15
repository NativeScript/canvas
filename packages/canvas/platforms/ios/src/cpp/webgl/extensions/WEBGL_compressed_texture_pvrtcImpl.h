//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "gl.h"
#include <vector>
#include "Helpers.h"
#include "Common.h"
#include "Caches.h"


class WEBGL_compressed_texture_pvrtcImpl {
public:
    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WEBGL_compressed_texture_pvrtcTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WEBGL_compressed_texture_pvrtc"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(1);

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_RGB_PVRTC_4BPPV1_IMG"),
                  v8::Number::New(isolate, (double) GL_COMPRESSED_RGB_PVRTC_4BPPV1_IMG));

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_RGBA_PVRTC_4BPPV1_IMG"),
                  v8::Number::New(isolate, (double) GL_COMPRESSED_RGBA_PVRTC_4BPPV1_IMG));

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_RGB_PVRTC_2BPPV1_IMG"),
                  v8::Number::New(isolate, (double) GL_COMPRESSED_RGB_PVRTC_2BPPV1_IMG));

        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_RGBA_PVRTC_2BPPV1_IMG"),
                  v8::Number::New(isolate, (double) GL_COMPRESSED_RGBA_PVRTC_2BPPV1_IMG));

        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "WEBGL_compressed_texture_pvrtc"));

        cache->WEBGL_compressed_texture_pvrtcTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, WEBGL_compressed_texture_pvrtcImpl *compressedTexturePvrtc) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WEBGL_compressed_texture_pvrtcImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::WEBGL_compressed_texture_pvrtc);
        auto ext = v8::External::New(isolate, compressedTexturePvrtc);
        object->SetInternalField(0, ext);
        return scope.Escape(object);
    }

    static WEBGL_compressed_texture_pvrtcImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WEBGL_compressed_texture_pvrtcImpl *>(ptr);
    }
};