//
// Created by Osei Fortune on 29/04/2022.
//

#include "OES_standard_derivativesImpl.h"

v8::Local <v8::Function> OES_standard_derivativesImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->OES_standard_derivativesImplCtor.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "OES_standard_derivatives"));

    auto func = ctorTmpl->GetFunction(context).ToLocalChecked();

    cache->OES_standard_derivativesImplCtor = std::make_unique<v8::Persistent<v8::Function>>(isolate, func);
    return func;
}

v8::Local<v8::Object> OES_standard_derivativesImpl::NewInstance(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->NewInstance(context).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result,"OES_standard_derivatives");
    result->Set(context, Helpers::ConvertToV8String(isolate, "FRAGMENT_SHADER_DERIVATIVE_HINT_OES"), v8::Int32::New(isolate, GL_FRAGMENT_SHADER_DERIVATIVE_HINT_OES));
    return handle_scope.Escape(result);
}