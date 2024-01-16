//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "gl.h"
#include <vector>
#include "Helpers.h"
#include "Common.h"
#include "Caches.h"
#include "ObjectWrapperImpl.h"
class WEBGL_lose_contextImpl: ObjectWrapperImpl {
public:
    WEBGL_lose_contextImpl(WEBGL_lose_context* context);
    ~WEBGL_lose_contextImpl() {
        canvas_native_webgl_WEBGL_lose_context_destroy(this->GetContext());
        this->context_ = nullptr;
    }

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WEBGL_lose_contextTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WEBGL_lose_context"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);

        tmpl->Set(ConvertToV8String(isolate, "loseContext"),
                  v8::FunctionTemplate::New(isolate, &LoseContext));
        tmpl->Set(ConvertToV8String(isolate, "restoreContext"),
                  v8::FunctionTemplate::New(isolate, &RestoreContext));

        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "WEBGL_lose_context"));

        cache->WEBGL_lose_contextTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, WEBGL_lose_contextImpl *element) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WEBGL_lose_contextImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( object, NativeType::WEBGL_lose_context);
        object->SetAlignedPointerInInternalField(0, element);
        element->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WEBGL_lose_contextImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WEBGL_lose_contextImpl *>(ptr);
    }

    static void LoseContext(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RestoreContext(const v8::FunctionCallbackInfo<v8::Value> &args);

    WEBGL_lose_context* GetContext() {
        return this->context_;
    }


private:
    WEBGL_lose_context* context_;
};
