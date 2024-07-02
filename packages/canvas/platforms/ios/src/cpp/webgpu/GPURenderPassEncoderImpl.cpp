//
// Created by Osei Fortune on 01/07/2024.
//

#include "GPURenderPassEncoderImpl.h"
#include "Caches.h"
#include "GPURenderPipelineImpl.h"
#include "GPUBufferImpl.h"

GPURenderPassEncoderImpl::GPURenderPassEncoderImpl(CanvasGPURenderPass *pass) : pass_(pass) {}

CanvasGPURenderPass *GPURenderPassEncoderImpl::GetPass() {
    return this->pass_;
}


void GPURenderPassEncoderImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPURenderPassEncoder"), func);
}

GPURenderPassEncoderImpl *
GPURenderPassEncoderImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPURenderPassEncoderImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPURenderPassEncoderImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPURenderPassEncoderTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPURenderPassEncoder"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->Set(
            ConvertToV8String(isolate, "setPipeline"),
            v8::FunctionTemplate::New(isolate, &SetPipeline));

    tmpl->Set(
            ConvertToV8String(isolate, "setVertexBuffer"),
            v8::FunctionTemplate::New(isolate, &SetVertexBuffer));

    tmpl->Set(
            ConvertToV8String(isolate, "draw"),
            v8::FunctionTemplate::New(isolate, &Draw));

    tmpl->Set(
            ConvertToV8String(isolate, "end"),
            v8::FunctionTemplate::New(isolate, &End));

    cache->GPURenderPassEncoderTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void GPURenderPassEncoderImpl::SetPipeline(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto pipelineVal = args[0];
    if (pipelineVal->IsObject()) {
        auto pipeline = GPURenderPipelineImpl::GetPointer(pipelineVal.As<v8::Object>());
        if (pipeline != nullptr) {
            canvas_native_webgpu_render_pass_set_pipeline(ptr->GetPass(),
                                                          pipeline->GetGPUPipeline());
        }
    }
}

void GPURenderPassEncoderImpl::SetVertexBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto slotVal = args[0];
    auto bufferVal = args[1];
    int64_t offset = -1;
    int64_t size = -1;
    auto offsetVal = args[2];
    auto sizeVal = args[3];

    if (slotVal->IsUint32() && bufferVal->IsObject()) {
        auto slot = slotVal.As<v8::Uint32>()->Value();
        auto buffer = GPUBufferImpl::GetPointer(bufferVal.As<v8::Object>());
        if (buffer == nullptr) {
            // todo throw ??
            return;
        }

        if (offsetVal->IsNumber()) {
            offset = (int64_t) offsetVal.As<v8::Number>()->Value();
        }

        if (sizeVal->IsNumber()) {
            size = (int64_t) sizeVal.As<v8::Number>()->Value();
        }

        canvas_native_webgpu_render_pass_set_vertex_buffer(ptr->GetPass(), slot,
                                                           buffer->GetGPUBuffer(), offset, size);

    }
}

void GPURenderPassEncoderImpl::Draw(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto vertexCountVal = args[0];
    auto instanceCountVal = args[1];
    uint32_t instanceCount = 1;
    uint32_t firstVertex = 0;
    uint32_t firstInstance = 0;
    auto firstVertexVal = args[2];
    auto firstInstanceVal = args[3];

    if (vertexCountVal->IsUint32()) {
        auto vertexCount = vertexCountVal.As<v8::Uint32>()->Value();


        if (instanceCountVal->IsUint32()) {
            instanceCount = instanceCountVal.As<v8::Uint32>()->Value();
        }

        if (firstVertexVal->IsUint32()) {
            firstVertex = firstVertexVal.As<v8::Uint32>()->Value();
        }

        if (firstInstanceVal->IsUint32()) {
            firstInstance = firstInstanceVal.As<v8::Uint32>()->Value();
        }

        canvas_native_webgpu_render_pass_encoder_draw(ptr->GetPass(), vertexCount, instanceCount,
                                                      firstVertex, firstInstance);

    }
}

void GPURenderPassEncoderImpl::End(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_webgpu_render_pass_end(ptr->GetPass());
}
