//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include <vector>
#include "Helpers.h"
#include "Caches.h"
#include "Common.h"


class WebGLShaderPrecisionFormatImpl {
public:
    WebGLShaderPrecisionFormatImpl(WebGLShaderPrecisionFormat* shader);
    
    ~WebGLShaderPrecisionFormatImpl() {
        canvas_native_webgl_shader_precision_format_destroy(this->GetShaderPrecisionFormat());
        this->shader_ = nullptr;
    }

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WebGLShaderPrecisionFormatTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGLShaderPrecisionFormat"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(1);

        tmpl->SetAccessor(ConvertToV8String(isolate, "rangeMin"), &GetRangeMin);
        tmpl->SetAccessor(ConvertToV8String(isolate, "rangeMax"), &GetRangeMax);
        tmpl->SetAccessor(ConvertToV8String(isolate, "precision"), &GetPrecision);

        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "WebGLShaderPrecisionFormat"));

        cache->WebGLShaderPrecisionFormatTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, WebGLShaderPrecisionFormatImpl *shaderPrecisionFormat) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WebGLShaderPrecisionFormatImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::WebGLShaderPrecisionFormat);
        auto ext = v8::External::New(isolate, shaderPrecisionFormat);
        object->SetInternalField(0, ext);
        return scope.Escape(object);
    }

    static WebGLShaderPrecisionFormatImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WebGLShaderPrecisionFormatImpl *>(ptr);
    }


    static void GetRangeMin(v8::Local<v8::String> property,
                            const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetRangeMax(v8::Local<v8::String> property,
                            const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetPrecision(v8::Local<v8::String> property,
                             const v8::PropertyCallbackInfo<v8::Value> &info);

    WebGLShaderPrecisionFormat* GetShaderPrecisionFormat();

private:
    WebGLShaderPrecisionFormat* shader_;
};

