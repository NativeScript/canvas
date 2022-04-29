//
// Created by Osei Fortune on 27/04/2022.
//

#include "WebGLFramebuffer.h"

void WebGLFramebuffer::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "WebGLFramebuffer"), ctor);
}

void WebGLFramebuffer::Create(const v8::FunctionCallbackInfo <v8::Value> &args) {
    Helpers::ThrowIllegalConstructor(args.GetIsolate());
}

v8::Local<v8::Function> WebGLFramebuffer::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->WebGLFramebufferCtor.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, &Create);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "WebGLFramebuffer"));

    auto func = ctorTmpl->GetFunction(context).ToLocalChecked();

    cache->WebGLFramebufferCtor = std::make_unique<v8::Persistent<v8::Function>>(isolate, func);
    return func;
}