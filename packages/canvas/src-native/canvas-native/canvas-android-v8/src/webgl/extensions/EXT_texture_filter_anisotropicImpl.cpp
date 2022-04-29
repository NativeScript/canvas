//
// Created by Osei Fortune on 29/04/2022.
//

#include "EXT_texture_filter_anisotropicImpl.h"

v8::Local <v8::Function> EXT_texture_filter_anisotropicImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->EXT_texture_filter_anisotropicImplCtor.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "EXT_texture_filter_anisotropicImpl"));

    auto func = ctorTmpl->GetFunction(context).ToLocalChecked();

    cache->EXT_texture_filter_anisotropicImplCtor = std::make_unique<v8::Persistent<v8::Function>>(isolate, func);
    return func;
}

v8::Local<v8::Object> EXT_texture_filter_anisotropicImpl::NewInstance(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->NewInstance(context).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result,"EXT_texture_filter_anisotropic");

    result->Set(context, Helpers::ConvertToV8String(isolate, "MAX_TEXTURE_MAX_ANISOTROPY_EXT"), v8::Int32::New(isolate, GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "TEXTURE_MAX_ANISOTROPY_EXT"), v8::Int32::New(isolate, GL_TEXTURE_MAX_ANISOTROPY_EXT));
    return handle_scope.Escape(result);
}
