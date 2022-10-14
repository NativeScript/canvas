//
// Created by Osei Fortune on 28/04/2022.
//

#include "EXT_color_buffer_half_floatImpl.h"
#include "canvas-android/src/lib.rs.h"

v8::Local<v8::FunctionTemplate> EXT_color_buffer_half_floatImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->EXT_color_buffer_half_floatImplTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "EXT_color_buffer_half_float"));

    cache->EXT_color_buffer_half_floatImplTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

v8::Local<v8::Object> EXT_color_buffer_half_floatImpl::NewInstance(v8::Isolate *isolate) {
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->InstanceTemplate()->NewInstance(context).ToLocalChecked();
    Helpers::SetInstanceType(isolate, result, ObjectType::EXT_color_buffer_half_float);
    result->Set(context, Helpers::ConvertToV8String(isolate, "RGBA16F_EXT"), v8::Int32::New(isolate, GL_RGBA16F_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "RGB16F_EXT"), v8::Int32::New(isolate, GL_RGB16F_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT"),
                v8::Int32::New(isolate, GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "UNSIGNED_NORMALIZED_EXT"),
                v8::Int32::New(isolate, GL_UNSIGNED_NORMALIZED_EXT));

    return handle_scope.Escape(result);
}
