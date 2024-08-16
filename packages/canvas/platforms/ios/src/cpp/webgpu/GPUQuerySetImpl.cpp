//
// Created by Osei Fortune on 23/06/2024.
//

#include "GPUQuerySetImpl.h"
#include "Caches.h"

GPUQuerySetImpl::GPUQuerySetImpl(const CanvasGPUQuerySet *querySet) : querySet_(querySet) {}

const CanvasGPUQuerySet *GPUQuerySetImpl::GetQuerySet() {
    return this->querySet_;
}


void GPUQuerySetImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUQuerySet"), func).FromJust();
}

GPUQuerySetImpl *GPUQuerySetImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUQuerySetImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUQuerySetImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUQuerySetTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUQuerySet"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "count"),
            GetCount
    );


    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "type"),
            GetType
    );


    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "label"),
            GetLabel
    );


    tmpl->Set(
            ConvertToV8String(isolate, "destroy"),
            v8::FunctionTemplate::New(isolate, &Destroy));


    cache->GPUQuerySetTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void
GPUQuerySetImpl::GetCount(v8::Local<v8::Name> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        info.GetReturnValue().Set(canvas_native_webgpu_query_set_get_count(ptr->GetQuerySet()));
        return;
    }

    info.GetReturnValue().Set(0);
}


void
GPUQuerySetImpl::GetType(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto isolate = info.GetIsolate();
    if (ptr != nullptr) {
        auto type = canvas_native_webgpu_query_set_get_type(ptr->GetQuerySet());
        switch (type) {
            case CanvasQueryTypeOcclusion:
                info.GetReturnValue().Set(ConvertToV8String(isolate, "occlusion"));
                break;
            case CanvasQueryTypeTimestamp:
                info.GetReturnValue().Set(ConvertToV8String(isolate, "timestamp"));
                break;
        }
        return;
    }

    info.GetReturnValue().SetEmptyString();
}

void
GPUQuerySetImpl::GetLabel(v8::Local<v8::Name> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto isolate = info.GetIsolate();
    if (ptr != nullptr) {
        auto label = canvas_native_webgpu_query_set_get_label(ptr->GetQuerySet());
        if (label != nullptr) {
            info.GetReturnValue().Set(ConvertToV8String(isolate, label));
            canvas_native_string_destroy(label);
            return;
        }
    }

    info.GetReturnValue().SetEmptyString();
}


void GPUQuerySetImpl::Destroy(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    canvas_native_webgpu_query_set_destroy(ptr->querySet_);

}
