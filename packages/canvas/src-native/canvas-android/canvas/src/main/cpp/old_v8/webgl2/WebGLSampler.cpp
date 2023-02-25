//
// Created by Osei Fortune on 30/04/2022.
//

#include "WebGLSampler.h"


void WebGLSampler::Init(v8::Isolate *isolate) {
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "WebGLSampler"),
                ctor->GetFunction(context).ToLocalChecked());
}

void WebGLSampler::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Helpers::ThrowIllegalConstructor(args.GetIsolate());
}

v8::Local<v8::FunctionTemplate> WebGLSampler::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->WebGLSamplerTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, &Create);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "WebGLSampler"));
    cache->WebGLSamplerTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

v8::Local<v8::Object> WebGLSampler::NewInstance(v8::Isolate *isolate, uint32_t sampler) {
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->InstanceTemplate()->NewInstance(isolate->GetCurrentContext()).ToLocalChecked();
    Helpers::SetPrivate(isolate, result, "instance", v8::Uint32::New(isolate, sampler));
    Helpers::SetInstanceType(isolate, result, ObjectType::WebGLSampler);
    return handle_scope.Escape(result);
}