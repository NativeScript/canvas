//
// Created by Osei Fortune on 29/06/2024.
//

#include "GPURenderPipelineImpl.h"
#include "Caches.h"
#include "GPUBindGroupLayoutImpl.h"

GPURenderPipelineImpl::GPURenderPipelineImpl(const CanvasGPURenderPipeline *pipeline) : pipeline_(
        pipeline) {}

const CanvasGPURenderPipeline *GPURenderPipelineImpl::GetGPUPipeline() {
    return this->pipeline_;
}


void GPURenderPipelineImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPURenderPipeline"), func).FromJust();
}

GPURenderPipelineImpl *GPURenderPipelineImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPURenderPipelineImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPURenderPipelineImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPURenderPipelineTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPURenderPipeline"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "label"),
            GetLabel
    );

    tmpl->Set(
            ConvertToV8String(isolate, "getBindGroupLayout"),
            v8::FunctionTemplate::New(isolate, &GetBindGroupLayout));

    cache->GPURenderPipelineTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void GPURenderPipelineImpl::GetBindGroupLayout(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto index = args[0]->Uint32Value(context).ToChecked();


    auto group_layout = canvas_native_webgpu_render_pipeline_get_bind_group_layout(
            ptr->GetGPUPipeline(), index);

    if (group_layout != nullptr) {
        auto ret = GPUBindGroupLayoutImpl::NewInstance(isolate,
                                                       new GPUBindGroupLayoutImpl(group_layout));
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetUndefined();
}


void
GPURenderPipelineImpl::GetLabel(v8::Local<v8::Name> name,
                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto label = canvas_native_webgpu_render_pipeline_get_label(ptr->pipeline_);
        if (label == nullptr) {
            info.GetReturnValue().SetEmptyString();
            return;
        }
        info.GetReturnValue().Set(
                ConvertToV8String(info.GetIsolate(), label)
        );
        canvas_native_string_destroy(label);
        return;
    }

    info.GetReturnValue().SetEmptyString();
}

