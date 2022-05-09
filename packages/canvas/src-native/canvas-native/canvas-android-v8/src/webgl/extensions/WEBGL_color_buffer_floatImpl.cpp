//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_color_buffer_floatImpl.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

v8::Local<v8::FunctionTemplate> WEBGL_color_buffer_floatImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->WEBGL_color_buffer_floatImplTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "WEBGL_color_buffer_float"));
    cache->WEBGL_color_buffer_floatImplTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

v8::Local<v8::Object> WEBGL_color_buffer_floatImpl::NewInstance(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->InstanceTemplate()->NewInstance(context).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result, "WEBGL_color_buffer_float");

    result->Set(context, Helpers::ConvertToV8String(isolate, "RGBA32F_EXT"), v8::Int32::New(isolate, GL_RGBA32F_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "RGB32F_EXT"), v8::Int32::New(isolate, GL_RGB32F_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT"),
                v8::Int32::New(isolate, GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "UNSIGNED_NORMALIZED_EXT"),
                v8::Int32::New(isolate, GL_UNSIGNED_NORMALIZED_EXT));
    return handle_scope.Escape(result);
}