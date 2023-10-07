//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once

#include "Caches.h"
#include "Common.h"

class WebGLVertexArrayObject {
public:
    WebGLVertexArrayObject(uint32_t vertexArrayObject) : vertexArrayObject_(vertexArrayObject) {}

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WebGLVertexArrayObjectTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGLVertexArrayObject"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(1);

        cache->WebGLVertexArrayObjectTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, WebGLVertexArrayObject *vertexArrayObject) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WebGLVertexArrayObject::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::WebGLVertexArrayObject);
        auto ext = v8::External::New(isolate, vertexArrayObject);
        object->SetInternalField(0, ext);
        return scope.Escape(object);
    }

    static WebGLVertexArrayObject *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WebGLVertexArrayObject *>(ptr);
    }

    uint32_t GetVertexArrayObject() {
        return this->vertexArrayObject_;
    }

private:
    uint32_t vertexArrayObject_;
};
