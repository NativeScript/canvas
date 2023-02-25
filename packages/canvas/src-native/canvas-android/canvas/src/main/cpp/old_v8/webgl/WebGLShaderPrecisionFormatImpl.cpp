//
// Created by Osei Fortune on 27/04/2022.
//

#include "WebGLShaderPrecisionFormatImpl.h"
#include "canvas-android/src/lib.rs.h"

WebGLShaderPrecisionFormatImpl::WebGLShaderPrecisionFormatImpl(rust::Box<WebGLShaderPrecisionFormat> shader) : shader_(
        std::move(shader)) {

}


void WebGLShaderPrecisionFormatImpl::Init(v8::Isolate *isolate) {
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "WebGLShaderPrecisionFormat"),
                ctor->GetFunction(context).ToLocalChecked());
}

WebGLShaderPrecisionFormatImpl *WebGLShaderPrecisionFormatImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<WebGLShaderPrecisionFormatImpl *>(ptr);
}

void WebGLShaderPrecisionFormatImpl::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Helpers::ThrowIllegalConstructor(args.GetIsolate());
}

v8::Local<v8::Object>
WebGLShaderPrecisionFormatImpl::NewInstance(v8::Isolate *isolate, rust::Box<WebGLShaderPrecisionFormat> shader) {
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto ctorFunc = GetCtor(isolate);
    WebGLShaderPrecisionFormatImpl *format = new WebGLShaderPrecisionFormatImpl(std::move(shader));
    auto result = ctorFunc->InstanceTemplate()->NewInstance(isolate->GetCurrentContext()).ToLocalChecked();
    Helpers::SetInstanceType(isolate, result, ObjectType::WebGLShaderPrecisionFormat);
    AddWeakListener(isolate, result, format);
    return handle_scope.Escape(result);
}

void WebGLShaderPrecisionFormatImpl::GetRangeMin(v8::Local<v8::String> name,
                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    info.GetReturnValue().Set(
            canvas_native_webgl_shader_precision_format_get_range_min(*ptr->shader_)
    );
}

void WebGLShaderPrecisionFormatImpl::GetRangeMax(v8::Local<v8::String> name,
                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    info.GetReturnValue().Set(
            canvas_native_webgl_shader_precision_format_get_range_max(*ptr->shader_));
}

void WebGLShaderPrecisionFormatImpl::GetPrecision(v8::Local<v8::String> name,
                                                  const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    info.GetReturnValue().Set(
            canvas_native_webgl_shader_precision_format_get_precision(*ptr->shader_));
}

v8::Local<v8::FunctionTemplate> WebGLShaderPrecisionFormatImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->WebGLShaderPrecisionFormatTmpl.get();

    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, &Create);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "WebGLShaderPrecisionFormat"));
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    auto tmpl = ctorTmpl->PrototypeTemplate();

    tmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "rangeMin"), &GetRangeMin);
    tmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "rangeMax"), &GetRangeMax);
    tmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "precision"), &GetPrecision);


    cache->WebGLShaderPrecisionFormatTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}
