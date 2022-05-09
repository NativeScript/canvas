//
// Created by Osei Fortune on 29/04/2022.
//

#include "EXT_sRGBImpl.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

v8::Local <v8::FunctionTemplate> EXT_sRGBImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->EXT_sRGBImplTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "EXT_sRGB"));

    cache->EXT_sRGBImplTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

v8::Local<v8::Object> EXT_sRGBImpl::NewInstance(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->InstanceTemplate()->NewInstance(context).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result,"EXT_sRGB");

    result->Set(context, Helpers::ConvertToV8String(isolate, "SRGB_EXT"), v8::Int32::New(isolate, GL_SRGB_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "SRGB_ALPHA_EXT"), v8::Int32::New(isolate, GL_SRGB_ALPHA_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "SRGB8_ALPHA8_EXT"), v8::Int32::New(isolate, GL_SRGB8_ALPHA8_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT"), v8::Int32::New(isolate, GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT));
    return handle_scope.Escape(result);
}
