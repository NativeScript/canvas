//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_compressed_texture_pvrtcImpl.h"

v8::Local <v8::Function> WEBGL_compressed_texture_pvrtcImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->WEBGL_compressed_texture_pvrtcImplCtor.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "WEBGL_compressed_texture_pvrtc"));

    auto func = ctorTmpl->GetFunction(context).ToLocalChecked();

    cache->WEBGL_compressed_texture_pvrtcImplCtor = std::make_unique<v8::Persistent<v8::Function>>(isolate, func);
    return func;
}

v8::Local<v8::Object> WEBGL_compressed_texture_pvrtcImpl::NewInstance(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->NewInstance(context).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result,"WEBGL_compressed_texture_pvrtc");

    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_RGB_PVRTC_4BPPV1_IMG"), v8::Int32::New(isolate, GL_COMPRESSED_RGB_PVRTC_4BPPV1_IMG));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_RGBA_PVRTC_4BPPV1_IMG"), v8::Int32::New(isolate, GL_COMPRESSED_RGBA_PVRTC_4BPPV1_IMG));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_RGB_PVRTC_2BPPV1_IMG"), v8::Int32::New(isolate, GL_COMPRESSED_RGB_PVRTC_2BPPV1_IMG));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_RGBA_PVRTC_2BPPV1_IMG"), v8::Int32::New(isolate, GL_COMPRESSED_RGBA_PVRTC_2BPPV1_IMG));
    return handle_scope.Escape(result);
}