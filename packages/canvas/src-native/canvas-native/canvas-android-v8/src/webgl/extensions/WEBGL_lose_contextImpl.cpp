//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_lose_contextImpl.h"


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
    auto result = ctorFunc->NewInstance(isolate->GetCurrentContext()).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result, "WEBGL_lose_context");
    auto ext = v8::External::New(isolate, contextImpl);
    result->SetInternalField(0, ext);

    return handle_scope.Escape(result);
}

v8::Local<v8::Function> WEBGL_lose_contextImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->ANGLE_instanced_arraysImplCtor.get();

    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "ANGLE_instanced_arrays"));

    auto func = ctorTmpl->GetFunction(context).ToLocalChecked();
    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);

    tmpl->Set(Helpers::ConvertToV8String(isolate, "loseContext"),
                v8::FunctionTemplate::New(isolate, &LoseContext));

    tmpl->Set(Helpers::ConvertToV8String(isolate, "restore"),
                v8::FunctionTemplate::New(isolate, &RestoreContext));

    cache->ANGLE_instanced_arraysImplCtor = std::make_unique<v8::Persistent<v8::Function>>(isolate, func);
    return func;
}

void WEBGL_lose_contextImpl::LoseContext(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());

    canvas_native_webgl_lose_context_lose_context(*ptr->context_);
}

void WEBGL_lose_contextImpl::RestoreContext(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    canvas_native_webgl_lose_context_restore_context(*ptr->context_);
}
