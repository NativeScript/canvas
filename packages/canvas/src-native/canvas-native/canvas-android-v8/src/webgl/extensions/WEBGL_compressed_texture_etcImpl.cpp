//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_compressed_texture_etcImpl.h"

v8::Local<v8::FunctionTemplate> WEBGL_compressed_texture_etcImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->WEBGL_compressed_texture_etcImplTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "WEBGL_compressed_texture_etc"));
    cache->WEBGL_compressed_texture_etcImplTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate,
                                                                                                         ctorTmpl);
    return ctorTmpl;
}

v8::Local<v8::Object> WEBGL_compressed_texture_etcImpl::NewInstance(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->InstanceTemplate()->NewInstance(context).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result, "WEBGL_compressed_texture_etc");

    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_R11_EAC"),
                v8::Int32::New(isolate, GL_COMPRESSED_R11_EAC));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_SIGNED_R11_EAC"),
                v8::Int32::New(isolate, GL_COMPRESSED_SIGNED_R11_EAC));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_RG11_EAC"),
                v8::Int32::New(isolate, GL_COMPRESSED_RG11_EAC));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_SIGNED_RG11_EAC"),
                v8::Int32::New(isolate, GL_COMPRESSED_SIGNED_RG11_EAC));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_RGB8_ETC2"),
                v8::Int32::New(isolate, GL_COMPRESSED_RGB8_ETC2));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_RGBA8_ETC2_EAC"),
                v8::Int32::New(isolate, GL_COMPRESSED_RGBA8_ETC2_EAC));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_SRGB8_ETC2"),
                v8::Int32::New(isolate, GL_COMPRESSED_SRGB8_ETC2));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_SRGB8_ALPHA8_ETC2_EAC"),
                v8::Int32::New(isolate, GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2"),
                v8::Int32::New(isolate, GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2"),
                v8::Int32::New(isolate, GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2));

    return handle_scope.Escape(result);
}