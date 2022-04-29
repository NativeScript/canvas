//
// Created by Osei Fortune on 27/04/2022.
//

#include "WebGLShaderPrecisionFormatImpl.h"

WebGLShaderPrecisionFormatImpl::WebGLShaderPrecisionFormatImpl(rust::Box <WebGLShaderPrecisionFormat> shader) : shader_(
        std::move(shader)) {

}


void WebGLShaderPrecisionFormatImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "WebGLShaderPrecisionFormat"), ctor);
}

WebGLShaderPrecisionFormatImpl *WebGLShaderPrecisionFormatImpl::GetPointer(v8::Local<v8::Object> object) {
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
WebGLShaderPrecisionFormatImpl::NewInstance(v8::Isolate *isolate, rust::Box <WebGLShaderPrecisionFormat> shader) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto ctorFunc = GetCtor(isolate);
    WebGLShaderPrecisionFormatImpl *format = new WebGLShaderPrecisionFormatImpl(std::move(shader));
    auto result = ctorFunc->NewInstance(isolate->GetCurrentContext()).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result, "WebGLShaderPrecisionFormat");
    auto ext = v8::External::New(isolate, format);
    result->SetInternalField(0, ext);
    return handle_scope.Escape(result);
}

void WebGLShaderPrecisionFormatImpl::GetRangeMin(v8::Local<v8::String> name,
                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(info.Holder());
    info.GetReturnValue().Set(
            canvas_native_webgl_shader_precision_format_get_range_min(*ptr->shader_)
    );
}

void WebGLShaderPrecisionFormatImpl::GetRangeMax(v8::Local<v8::String> name,
                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    info.GetReturnValue().Set(
            canvas_native_webgl_shader_precision_format_get_range_max(*ptr->shader_));
}

void WebGLShaderPrecisionFormatImpl::GetPrecision(v8::Local<v8::String> name,
                                                  const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(info.Holder());
    info.GetReturnValue().Set(
            canvas_native_webgl_shader_precision_format_get_precision(*ptr->shader_));
}

v8::Local<v8::Function> WebGLShaderPrecisionFormatImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->WebGLShaderPrecisionFormatCtor.get();

    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, &Create);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "WebGLShaderPrecisionFormat"));

    auto func = ctorTmpl->GetFunction(context).ToLocalChecked();
    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);

    tmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "rangeMin"), &GetRangeMin);
    tmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "rangeMax"), &GetRangeMax);
    tmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "precision"), &GetPrecision);


    cache->WebGLShaderPrecisionFormatCtor = std::make_unique<v8::Persistent<v8::Function>>(isolate, func);
    return func;
}