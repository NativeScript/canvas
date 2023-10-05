//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once

#include "Common.h"
#include "Caches.h"

class WebGLTransformFeedback {
public:
    WebGLTransformFeedback(uint32_t feedback) : feedback_(feedback) {}

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WebGLTransformFeedbackTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGLTransformFeedback"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(1);

        cache->WebGLTransformFeedbackTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static WebGLTransformFeedback *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WebGLTransformFeedback *>(ptr);
    }

    uint32_t GetFeedback() {
        return this->feedback_;
    }

private:
    uint32_t feedback_;
};
