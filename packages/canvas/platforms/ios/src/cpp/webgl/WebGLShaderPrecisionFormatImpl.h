//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include <vector>
#include "Helpers.h"
#include "Caches.h"
#include "Common.h"
#include "ObjectWrapperImpl.h"

class WebGLShaderPrecisionFormatImpl : ObjectWrapperImpl {
public:
    explicit WebGLShaderPrecisionFormatImpl(WebGLShaderPrecisionFormat *shader);

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
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGLShaderPrecisionFormat"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);

        tmpl->SetLazyDataProperty(ConvertToV8String(isolate, "rangeMin"), GetRangeMin);
        tmpl->SetLazyDataProperty(ConvertToV8String(isolate, "rangeMax"), GetRangeMax);
        tmpl->SetLazyDataProperty(ConvertToV8String(isolate, "precision"), GetPrecision);

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
        SetNativeType(shaderPrecisionFormat, NativeType::WebGLShaderPrecisionFormat);
        object->SetAlignedPointerInInternalField(0, shaderPrecisionFormat);
        shaderPrecisionFormat->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WebGLShaderPrecisionFormatImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WebGLShaderPrecisionFormatImpl *>(ptr);
    }


    static void GetRangeMin(v8::Local<v8::Name> property,
                            const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetRangeMax(v8::Local<v8::Name> property,
                            const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetPrecision(v8::Local<v8::Name> property,
                             const v8::PropertyCallbackInfo<v8::Value> &info);

    WebGLShaderPrecisionFormat *GetShaderPrecisionFormat();

private:
    WebGLShaderPrecisionFormat *shader_;
};

