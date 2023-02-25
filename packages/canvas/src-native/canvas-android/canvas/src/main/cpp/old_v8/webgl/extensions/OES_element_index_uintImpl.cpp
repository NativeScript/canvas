//
// Created by Osei Fortune on 29/04/2022.
//

#include "OES_element_index_uintImpl.h"
#include "canvas-android/src/lib.rs.h"

v8::Local<v8::FunctionTemplate> OES_element_index_uintImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->OES_element_index_uintImplTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "OES_element_index_uint"));

    cache->OES_element_index_uintImplTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

v8::Local<v8::Object> OES_element_index_uintImpl::NewInstance(v8::Isolate *isolate) {
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->InstanceTemplate()->NewInstance(context).ToLocalChecked();
    Helpers::SetInstanceType(isolate, result, ObjectType::OES_element_index_uint);
    result->Set(context, Helpers::ConvertToV8String(isolate, "UNSIGNED_INT"), v8::Int32::New(isolate, GL_UNSIGNED_INT));
    return handle_scope.Escape(result);
}
