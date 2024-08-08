//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include "Common.h"
#include "Caches.h"
#include "ObjectWrapperImpl.h"
class WebGLBuffer: ObjectWrapperImpl {
public:
    WebGLBuffer(uint32_t buffer) : buffer_(buffer) {}

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WebGLBufferTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGLBuffer"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);

        cache->WebGLBufferTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, WebGLBuffer *buffer) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WebGLBuffer::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( buffer, NativeType::WebGLBuffer);
        object->SetAlignedPointerInInternalField(0, buffer);
        buffer->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WebGLBuffer *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WebGLBuffer *>(ptr);
    }

    uint32_t GetBuffer() {
        return this->buffer_;
    }

private:
    uint32_t buffer_;
};

