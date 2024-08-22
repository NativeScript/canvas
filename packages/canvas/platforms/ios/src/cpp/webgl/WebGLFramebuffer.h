//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class WebGLFramebuffer: ObjectWrapperImpl {
public:
    WebGLFramebuffer(uint32_t framebuffer) : framebuffer_(framebuffer) {}

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WebGLFramebufferTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGLFramebuffer"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);

        cache->WebGLFramebufferTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, WebGLFramebuffer *buffer) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WebGLFramebuffer::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( buffer, NativeType::WebGLFramebuffer);
        object->SetAlignedPointerInInternalField(0, buffer);
        buffer->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WebGLFramebuffer *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WebGLFramebuffer *>(ptr);
    }

    uint32_t GetFrameBuffer() {
        return this->framebuffer_;
    }


private:
    uint32_t framebuffer_;
};

