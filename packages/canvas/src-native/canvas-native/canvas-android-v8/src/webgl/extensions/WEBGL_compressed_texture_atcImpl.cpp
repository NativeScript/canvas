//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_compressed_texture_atcImpl.h"

v8::Local<v8::FunctionTemplate> WEBGL_compressed_texture_atcImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->WEBGL_compressed_texture_atcImplTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "WEBGL_compressed_texture_atc"));
    cache->WEBGL_compressed_texture_atcImplTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

v8::Local<v8::Object> WEBGL_compressed_texture_atcImpl::NewInstance(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->InstanceTemplate()->NewInstance(context).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result, "WEBGL_compressed_texture_atc");

    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_RGB_ATC_WEBGL"),
                v8::Int32::New(isolate, GL_ATC_RGB_AMD));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL"),
                v8::Int32::New(isolate, GL_ATC_RGBA_EXPLICIT_ALPHA_AMD));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL"),
                v8::Int32::New(isolate, GL_ATC_RGBA_INTERPOLATED_ALPHA_AMD));
    return handle_scope.Escape(result);
}