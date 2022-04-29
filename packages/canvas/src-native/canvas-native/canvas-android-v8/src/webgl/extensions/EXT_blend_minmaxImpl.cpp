//
// Created by Osei Fortune on 28/04/2022.
//

#include "EXT_blend_minmaxImpl.h"

v8::Local<v8::Function> EXT_blend_minmaxImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->EXT_blend_minmaxCtor.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "EXT_blend_minmax"));

    auto func = ctorTmpl->GetFunction(context).ToLocalChecked();

    cache->EXT_blend_minmaxCtor = std::make_unique<v8::Persistent<v8::Function>>(isolate, func);
    return func;
}

v8::Local<v8::Object> EXT_blend_minmaxImpl::NewInstance(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->NewInstance(context).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result, "EXT_blend_minmax");
    result->Set(context, Helpers::ConvertToV8String(isolate, "MIN_EXT"), v8::Int32::New(isolate, GL_MIN_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "MAX_EXT"), v8::Int32::New(isolate, GL_MAX_EXT));
    return handle_scope.Escape(result);
}
