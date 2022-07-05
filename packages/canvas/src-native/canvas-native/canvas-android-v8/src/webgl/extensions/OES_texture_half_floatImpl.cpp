//
// Created by Osei Fortune on 29/04/2022.
//

#include "OES_texture_half_floatImpl.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

v8::Local<v8::FunctionTemplate> OES_texture_half_floatImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->OES_texture_half_floatImplTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "OES_texture_half_float"));
    cache->OES_texture_half_floatImplTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

v8::Local<v8::Object> OES_texture_half_floatImpl::NewInstance(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->InstanceTemplate()->NewInstance(context).ToLocalChecked();
    Helpers::SetInstanceType(isolate, result, ObjectType::OES_texture_half_float);
    result->Set(context, Helpers::ConvertToV8String(isolate, "HALF_FLOAT_OES"),
                v8::Int32::New(isolate, GL_HALF_FLOAT_OES));
    return handle_scope.Escape(result);
}