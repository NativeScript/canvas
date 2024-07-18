//
// Created by Osei Fortune on 23/06/2024.
//

#include "GPUComputePassEncoderImpl.h"
#include "Caches.h"
#include "GPUBindGroupImpl.h"
#include "GPUComputePipelineImpl.h"
#include "GPUBufferImpl.h"

GPUComputePassEncoderImpl::GPUComputePassEncoderImpl(const CanvasGPUComputePassEncoder *pass)
        : computePass_(
        pass) {}

const CanvasGPUComputePassEncoder *GPUComputePassEncoderImpl::GetComputePass() {
    return this->computePass_;
}


void GPUComputePassEncoderImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUComputePassEncoder"), func);
}

GPUComputePassEncoderImpl *
GPUComputePassEncoderImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUComputePassEncoderImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUComputePassEncoderImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUComputePassEncoderTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUComputePassEncoder"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->Set(
            ConvertToV8String(isolate, "dispatchWorkgroups"),
            v8::FunctionTemplate::New(isolate, &DispatchWorkgroups));

    tmpl->Set(
            ConvertToV8String(isolate, "dispatchWorkgroupsIndirect"),
            v8::FunctionTemplate::New(isolate, &DispatchWorkgroupsIndirect));

    tmpl->Set(
            ConvertToV8String(isolate, "end"),
            v8::FunctionTemplate::New(isolate, &End));

    tmpl->Set(
            ConvertToV8String(isolate, "insertDebugMarker"),
            v8::FunctionTemplate::New(isolate, &InsertDebugMarker));

    tmpl->Set(
            ConvertToV8String(isolate, "popDebugGroup"),
            v8::FunctionTemplate::New(isolate, &PopDebugGroup));

    tmpl->Set(
            ConvertToV8String(isolate, "pushDebugGroup"),
            v8::FunctionTemplate::New(isolate, &PushDebugGroup));

    tmpl->Set(
            ConvertToV8String(isolate, "setBindGroup"),
            v8::FunctionTemplate::New(isolate, &SetBindGroup));

    tmpl->Set(
            ConvertToV8String(isolate, "setPipeline"),
            v8::FunctionTemplate::New(isolate, &SetPipeline));


    cache->GPUComputePassEncoderTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void
GPUComputePassEncoderImpl::DispatchWorkgroups(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto workgroupCountXVal = args[0];
    auto workgroupCountYVal = args[1];
    auto workgroupCountZVal = args[2];

    uint32_t workgroupCountY = 1;
    uint32_t workgroupCountZ = 1;

    if (workgroupCountYVal->IsUint32()) {
        workgroupCountY = workgroupCountYVal.As<v8::Uint32>()->Value();
    }

    if (workgroupCountZVal->IsUint32()) {
        workgroupCountZ = workgroupCountZVal.As<v8::Uint32>()->Value();
    }

    if (workgroupCountXVal->IsUint32()) {
        canvas_native_webgpu_compute_pass_encoder_dispatch_workgroups(ptr->GetComputePass(),
                                                                      workgroupCountXVal->Uint32Value(
                                                                              context).FromJust(),
                                                                      workgroupCountY,
                                                                      workgroupCountZ);
    }

}

void GPUComputePassEncoderImpl::DispatchWorkgroupsIndirect(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto indirectBuffer = args[0];
    auto indirectOffset = args[1];

    auto type = GetNativeType(indirectBuffer);


    if (type == NativeType::GPUBuffer) {
        auto buffer = GPUBufferImpl::GetPointer(indirectBuffer.As<v8::Object>());
        auto offset = (size_t) indirectOffset->NumberValue(context).FromJust();
        canvas_native_webgpu_compute_pass_encoder_dispatch_workgroups_indirect(
                ptr->GetComputePass(), buffer->GetGPUBuffer(), offset);
    }

}


void GPUComputePassEncoderImpl::End(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_webgpu_compute_pass_encoder_end(ptr->GetComputePass());
}

void
GPUComputePassEncoderImpl::InsertDebugMarker(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

    auto markerLabelVal = args[0];
    if (markerLabelVal->IsString()) {
        auto markerLabel = ConvertFromV8String(isolate, markerLabelVal);
        canvas_native_webgpu_compute_pass_encoder_insert_debug_marker(ptr->GetComputePass(),
                                                                      markerLabel.c_str());
    }
}

void GPUComputePassEncoderImpl::PopDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_webgpu_compute_pass_encoder_pop_debug_group(ptr->GetComputePass());
}

void GPUComputePassEncoderImpl::PushDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

    auto groupLabelVal = args[0];
    if (groupLabelVal->IsString()) {
        auto groupLabel = ConvertFromV8String(isolate, groupLabelVal);
        canvas_native_webgpu_compute_pass_encoder_push_debug_group(ptr->GetComputePass(),
                                                                   groupLabel.c_str());
    }
}

void GPUComputePassEncoderImpl::SetBindGroup(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto indexVal = args[0];
    auto bindGroupVal = args[1];
    auto dynamicOffsets = args[2];
    auto dynamicOffsetsStart = args[3];
    auto dynamicOffsetsLength = args[4];

    auto type = GetNativeType(bindGroupVal);

    if (type == NativeType::GPUBindGroup) {
        auto index = indexVal->Uint32Value(context).FromJust();
        auto bindgroup = GPUBindGroupImpl::GetPointer(bindGroupVal.As<v8::Object>());

        if (dynamicOffsets->IsUint8Array()) {
            auto buf = dynamicOffsets.As<v8::Uint32Array>();
            auto buffer = buf->Buffer();
            auto store = buffer->GetBackingStore();
            auto offset = buf->ByteOffset();
            auto data = static_cast<uint8_t *>(buffer->GetBackingStore()->Data()) + offset;
            auto size = buf->Length();
            auto start = (size_t) dynamicOffsetsStart->NumberValue(context).FromJust();
            auto offset_length = (size_t) dynamicOffsetsStart->NumberValue(context).FromJust();
            canvas_native_webgpu_compute_pass_encoder_set_bind_group(ptr->GetComputePass(), index,
                                                                     bindgroup->GetBindGroup(),
                                                                     static_cast<const uint32_t *>(static_cast<void *>(data)),
                                                                     size, start, offset_length);
        } else {
            canvas_native_webgpu_compute_pass_encoder_set_bind_group(ptr->GetComputePass(), index,
                                                                     bindgroup->GetBindGroup(),
                                                                     nullptr, 0, 0, 0);
        }
    }
}

void GPUComputePassEncoderImpl::SetPipeline(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto pipelineVal = args[0];
    if (pipelineVal->IsObject()) {
        auto pipeline = GPUComputePipelineImpl::GetPointer(pipelineVal.As<v8::Object>());
        if (pipeline != nullptr) {
            canvas_native_webgpu_compute_pass_encoder_set_pipeline(ptr->GetComputePass(),
                                                                   pipeline->GetGPUPipeline());
        }
    }
}
