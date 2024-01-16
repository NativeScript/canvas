//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include "Helpers.h"
#include "ObjectWrapperImpl.h"
class WebGLRenderbuffer: ObjectWrapperImpl {
public:
    WebGLRenderbuffer(uint32_t renderbuffer) : renderbuffer_(renderbuffer) {}

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WebGLRenderbufferTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGLRenderbuffer"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);

        cache->WebGLRenderbufferTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, WebGLRenderbuffer *buffer) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WebGLRenderbuffer::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( object, NativeType::WebGLRenderbuffer);
        object->SetAlignedPointerInInternalField(0, buffer);
        buffer->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WebGLRenderbuffer *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WebGLRenderbuffer *>(ptr);
    }

    uint32_t GetRenderBuffer() {
        return this->renderbuffer_;
    }

private:
    uint32_t renderbuffer_;
};

