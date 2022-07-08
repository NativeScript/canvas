//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_lose_contextImpl.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

WEBGL_lose_contextImpl::WEBGL_lose_contextImpl(rust::Box<WEBGL_lose_context> context) : context_(
        std::move(context)) {

}

WEBGL_lose_contextImpl *WEBGL_lose_contextImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<WEBGL_lose_contextImpl *>(ptr);
}

v8::Local<v8::Object>
WEBGL_lose_contextImpl::NewInstance(v8::Isolate *isolate, rust::Box<WEBGL_lose_context> context) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto ctorFunc = GetCtor(isolate);
    WEBGL_lose_contextImpl *contextImpl = new WEBGL_lose_contextImpl(std::move(context));
    auto result = ctorFunc->InstanceTemplate()->NewInstance(isolate->GetCurrentContext()).ToLocalChecked();
    Helpers::SetInstanceType(isolate, result, ObjectType::WEBGL_lose_context);
    AddWeakListener(isolate, result, contextImpl);
    return handle_scope.Escape(result);
}

v8::Local<v8::FunctionTemplate> WEBGL_lose_contextImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->ANGLE_instanced_arraysImplTmpl.get();

    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "ANGLE_instanced_arrays"));

    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);

    auto tmpl = ctorTmpl->PrototypeTemplate();

    tmpl->Set(Helpers::ConvertToV8String(isolate, "loseContext"),
              v8::FunctionTemplate::New(isolate, &LoseContext));

    tmpl->Set(Helpers::ConvertToV8String(isolate, "restore"),
              v8::FunctionTemplate::New(isolate, &RestoreContext));

    cache->ANGLE_instanced_arraysImplTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void WEBGL_lose_contextImpl::LoseContext(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());

    canvas_native_webgl_lose_context_lose_context(*ptr->context_);
}

void WEBGL_lose_contextImpl::RestoreContext(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    canvas_native_webgl_lose_context_restore_context(*ptr->context_);
}
