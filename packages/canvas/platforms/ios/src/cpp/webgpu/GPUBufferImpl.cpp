//
// Created by Osei Fortune on 18/06/2024.
//

#include "GPUBufferImpl.h"
#include "Caches.h"

GPUBufferImpl::GPUBufferImpl(CanvasGPUBuffer *buffer) : buffer_(buffer) {}

CanvasGPUBuffer *GPUBufferImpl::GetGPUBuffer() {
    return this->buffer_;
}


void GPUBufferImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUBuffer"), func);
}

GPUBufferImpl *GPUBufferImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUBufferImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUBufferImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUBufferTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUBuffer"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "usage"),
            GetUsage
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "size"),
            GetSize
    );


    tmpl->Set(
            ConvertToV8String(isolate, "destroy"),
            v8::FunctionTemplate::New(isolate, &Destroy));


    cache->GPUBufferTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}


void
GPUBufferImpl::GetUsage(v8::Local<v8::Name> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto usage = canvas_native_webgpu_buffer_usage(ptr->GetGPUBuffer());
        info.GetReturnValue().Set(
                usage
        );
        return;
    }
    info.GetReturnValue().Set(0);
}


void
GPUBufferImpl::GetSize(v8::Local<v8::Name> name,
                       const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto size = canvas_native_webgpu_buffer_size(ptr->GetGPUBuffer());
        info.GetReturnValue().Set((double) size);
        return;
    }
    info.GetReturnValue().Set(0);
}


void GPUBufferImpl::Destroy(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUBufferImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    canvas_native_webgpu_buffer_destroy(ptr->GetGPUBuffer());
}

