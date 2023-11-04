//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "Caches.h"
#include "Common.h"

class EXT_shader_texture_lodImpl {
public:
    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->EXT_shader_texture_lodTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "EXT_shader_texture_lod"));

        auto tmpl = ctorTmpl->InstanceTemplate();

        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "EXT_shader_texture_lod"));

        cache->EXT_shader_texture_lodTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, EXT_shader_texture_lodImpl *texture) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = EXT_shader_texture_lodImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::EXT_shader_texture_lod);
        auto ext = v8::External::New(isolate, texture);
        object->SetInternalField(0, ext);
        return scope.Escape(object);
    }

    static EXT_shader_texture_lodImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<EXT_shader_texture_lodImpl *>(ptr);
    }
};
