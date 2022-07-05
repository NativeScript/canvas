//
// Created by Osei Fortune on 29/04/2022.
//

#include "EXT_texture_filter_anisotropicImpl.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

v8::Local<v8::FunctionTemplate> EXT_texture_filter_anisotropicImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->EXT_texture_filter_anisotropicImplTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "EXT_texture_filter_anisotropic"));

    cache->EXT_texture_filter_anisotropicImplTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate,
                                                                                                           ctorTmpl);
    return ctorTmpl;
}

v8::Local<v8::Object> EXT_texture_filter_anisotropicImpl::NewInstance(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->InstanceTemplate()->NewInstance(context).ToLocalChecked();

    Helpers::SetInstanceType(isolate, result, ObjectType::EXT_texture_filter_anisotropic);

    result->Set(context, Helpers::ConvertToV8String(isolate, "MAX_TEXTURE_MAX_ANISOTROPY_EXT"),
                v8::Int32::New(isolate, GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "TEXTURE_MAX_ANISOTROPY_EXT"),
                v8::Int32::New(isolate, GL_TEXTURE_MAX_ANISOTROPY_EXT));
    return handle_scope.Escape(result);
}
