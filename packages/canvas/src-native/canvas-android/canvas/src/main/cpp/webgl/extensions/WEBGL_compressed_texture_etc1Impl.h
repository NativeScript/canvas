//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "gl.h"
#include <vector>
#include "Common.h"
#include "Caches.h"
#include "Helpers.h"

class WEBGL_compressed_texture_etc1Impl {
public:
    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WEBGL_compressed_texture_etc1Tmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WEBGL_compressed_texture_etc1"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(1);


        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "WEBGL_compressed_texture_etc1"));
        tmpl->Set(ConvertToV8String(isolate, "COMPRESSED_RGB_ETC1_WEBGL"),
                  v8::Number::New(isolate, GL_ETC1_RGB8_OES));

        cache->WEBGL_compressed_texture_etc1Tmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, WEBGL_compressed_texture_etc1Impl *compressedTextureEtc1) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WEBGL_compressed_texture_etc1Impl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::WEBGL_compressed_texture_etc1);
        auto ext = v8::External::New(isolate, compressedTextureEtc1);
        object->SetInternalField(0, ext);
        return scope.Escape(object);
    }

    static WEBGL_compressed_texture_etc1Impl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WEBGL_compressed_texture_etc1Impl *>(ptr);
    }
};
