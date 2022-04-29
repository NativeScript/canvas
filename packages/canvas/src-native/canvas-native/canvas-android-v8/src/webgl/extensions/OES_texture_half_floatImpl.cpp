//
// Created by Osei Fortune on 29/04/2022.
//

#include "OES_texture_half_floatImpl.h"

v8::Local <v8::Function> OES_texture_half_floatImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->OES_texture_half_floatImplCtor.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "OES_texture_half_float"));

    auto func = ctorTmpl->GetFunction(context).ToLocalChecked();

    cache->OES_texture_half_floatImplCtor = std::make_unique<v8::Persistent<v8::Function>>(isolate, func);
    return func;
}

v8::Local<v8::Object> OES_texture_half_floatImpl::NewInstance(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->NewInstance(context).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result,"OES_texture_half_float");
    result->Set(context, Helpers::ConvertToV8String(isolate, "HALF_FLOAT_OES"), v8::Int32::New(isolate, GL_HALF_FLOAT_OES));
    return handle_scope.Escape(result);
}