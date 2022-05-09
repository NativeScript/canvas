//
// Created by Osei Fortune on 27/04/2022.
//

#include "WebGLActiveInfoImpl.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

WebGLActiveInfoImpl::WebGLActiveInfoImpl(rust::Box<WebGLActiveInfo> info) : info_(std::move(info)) {

}

void WebGLActiveInfoImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "WebGLActiveInfo"),
                ctor->GetFunction(context).ToLocalChecked());
}

WebGLActiveInfoImpl *WebGLActiveInfoImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<WebGLActiveInfoImpl *>(ptr);
}

void WebGLActiveInfoImpl::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Helpers::ThrowIllegalConstructor(args.GetIsolate());
}

v8::Local<v8::Object> WebGLActiveInfoImpl::NewInstance(v8::Isolate *isolate, rust::Box<WebGLActiveInfo> info) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto ctorFunc = GetCtor(isolate);
    WebGLActiveInfoImpl *activeInfo = new WebGLActiveInfoImpl(std::move(info));
    auto result = ctorFunc->InstanceTemplate()->NewInstance(isolate->GetCurrentContext()).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result, "WebGLActiveInfo");
    auto ext = v8::External::New(isolate, activeInfo);
    result->SetInternalField(0, ext);
    return handle_scope.Escape(result);
}

void WebGLActiveInfoImpl::GetName(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(info.Holder());
    std::string info_name;
    canvas_native_webgl_active_info_get_name(*ptr->info_, info_name);
    info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, info_name));
}

void WebGLActiveInfoImpl::GetSize(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    info.GetReturnValue().Set(
            canvas_native_webgl_active_info_get_size(*ptr->info_));
}

void WebGLActiveInfoImpl::GetType(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(info.Holder());
    info.GetReturnValue().Set(
            canvas_native_webgl_active_info_get_type(*ptr->info_));
}

v8::Local<v8::FunctionTemplate> WebGLActiveInfoImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->WebGLActiveInfoTmpl.get();

    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, &Create);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "WebGLActiveInfo"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);

    tmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "name"), &GetName);
    tmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "size"), &GetSize);
    tmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "type"), &GetType);


    cache->WebGLActiveInfoTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}
