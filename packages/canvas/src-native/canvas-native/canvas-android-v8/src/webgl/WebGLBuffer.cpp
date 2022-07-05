//
// Created by Osei Fortune on 27/04/2022.
//

#include "WebGLBuffer.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

void WebGLBuffer::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "WebGLBuffer"),
                ctor->GetFunction(context).ToLocalChecked());
}

void WebGLBuffer::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Helpers::ThrowIllegalConstructor(args.GetIsolate());
}

v8::Local<v8::FunctionTemplate> WebGLBuffer::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->WebGLBufferTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, &Create);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "WebGLBuffer"));
    cache->WebGLBufferTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

v8::Local<v8::Object> WebGLBuffer::NewInstance(v8::Isolate *isolate, uint32_t buffer) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->InstanceTemplate()->NewInstance(isolate->GetCurrentContext()).ToLocalChecked();
    Helpers::SetPrivate(isolate, result, "instance", v8::Uint32::New(isolate, buffer));
    Helpers::SetInstanceType(isolate, result, ObjectType::WebGLBuffer);
    return handle_scope.Escape(result);
}
