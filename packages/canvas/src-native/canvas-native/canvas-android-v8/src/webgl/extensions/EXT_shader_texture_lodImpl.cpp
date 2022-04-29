//
// Created by Osei Fortune on 29/04/2022.
//

#include "EXT_shader_texture_lodImpl.h"

v8::Local <v8::Function> EXT_shader_texture_lodImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->EXT_shader_texture_lodImplCtor.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "EXT_shader_texture_lod"));

    auto func = ctorTmpl->GetFunction(context).ToLocalChecked();

    cache->EXT_shader_texture_lodImplCtor = std::make_unique<v8::Persistent<v8::Function>>(isolate, func);
    return func;
}

v8::Local<v8::Object> EXT_shader_texture_lodImpl::NewInstance(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->NewInstance(context).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result,"EXT_shader_texture_lod");
    return handle_scope.Escape(result);
}
