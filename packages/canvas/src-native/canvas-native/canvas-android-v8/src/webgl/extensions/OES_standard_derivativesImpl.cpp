//
// Created by Osei Fortune on 29/04/2022.
//

#include "OES_standard_derivativesImpl.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

v8::Local<v8::FunctionTemplate> OES_standard_derivativesImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->OES_standard_derivativesImplTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "OES_standard_derivatives"));
    cache->OES_standard_derivativesImplTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

v8::Local<v8::Object> OES_standard_derivativesImpl::NewInstance(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->InstanceTemplate()->NewInstance(context).ToLocalChecked();
    Helpers::SetInstanceType(isolate, result, ObjectType::OES_standard_derivatives);
    result->Set(context, Helpers::ConvertToV8String(isolate, "FRAGMENT_SHADER_DERIVATIVE_HINT_OES"),
                v8::Int32::New(isolate, GL_FRAGMENT_SHADER_DERIVATIVE_HINT_OES));
    return handle_scope.Escape(result);
}