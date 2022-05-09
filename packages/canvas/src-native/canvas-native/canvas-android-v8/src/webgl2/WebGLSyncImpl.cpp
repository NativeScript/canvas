//
// Created by Osei Fortune on 30/04/2022.
//

#include "WebGLSyncImpl.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

WebGLSyncImpl::WebGLSyncImpl(rust::Box<WebGLSync> sync) : sync_(std::move(sync)) {
}

void WebGLSyncImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "WebGLSync"),
                ctor->GetFunction(context).ToLocalChecked());
}


WebGLSyncImpl *WebGLSyncImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<WebGLSyncImpl *>(ptr);
}

void WebGLSyncImpl::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Helpers::ThrowIllegalConstructor(args.GetIsolate());
}

v8::Local<v8::Object> WebGLSyncImpl::NewInstance(v8::Isolate *isolate, rust::Box<WebGLSync> sync) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto ctorFunc = GetCtor(isolate);
    WebGLSyncImpl *syncImpl = new WebGLSyncImpl(std::move(sync));
    auto result = ctorFunc->InstanceTemplate()->NewInstance(isolate->GetCurrentContext()).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result, "WebGLSync");
    auto ext = v8::External::New(isolate, syncImpl);
    result->SetInternalField(0, ext);
    return handle_scope.Escape(result);
}


v8::Local<v8::FunctionTemplate> WebGLSyncImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->WebGLSyncTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, &Create);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "WebGLSync"));
    cache->WebGLSyncTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

WebGLSync &WebGLSyncImpl::GetSync() {
    return *this->sync_;
}
