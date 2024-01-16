//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include "Helpers.h"
#include "ObjectWrapperImpl.h"
class WebGLTexture: ObjectWrapperImpl {
public:
    WebGLTexture(uint32_t texture) : texture_(texture) {}

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WebGLTextureTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGLTexture"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);

        cache->WebGLTextureTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, WebGLTexture *texture) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WebGLTexture::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( object, NativeType::WebGLTexture);
        object->SetAlignedPointerInInternalField(0, texture);
        texture->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WebGLTexture *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WebGLTexture *>(ptr);
    }

    uint32_t GetTexture() {
        return this->texture_;
    }

private:
    uint32_t texture_;
};
