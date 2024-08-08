//
// Created by Osei Fortune on 01/07/2024.
//

#include "GPURenderPassEncoderImpl.h"
#include "Caches.h"
#include "GPURenderPipelineImpl.h"
#include "GPUBufferImpl.h"
#include "GPURenderBundleImpl.h"
#include "GPUQuerySetImpl.h"
#include "GPUBindGroupImpl.h"

GPURenderPassEncoderImpl::GPURenderPassEncoderImpl(const CanvasGPURenderPassEncoder *pass) : pass_(
        pass) {}

const CanvasGPURenderPassEncoder *GPURenderPassEncoderImpl::GetPass() {
    return this->pass_;
}


void GPURenderPassEncoderImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPURenderPassEncoder"),
                      func).FromJust();
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
            ConvertToV8String(isolate, "beginOcclusionQuery"),
            v8::FunctionTemplate::New(isolate, &BeginOcclusionQuery));

    tmpl->Set(
            ConvertToV8String(isolate, "draw"),
            v8::FunctionTemplate::New(isolate, &Draw));

    tmpl->Set(
            ConvertToV8String(isolate, "drawIndexed"),
            v8::FunctionTemplate::New(isolate, &DrawIndexed));

    tmpl->Set(
            ConvertToV8String(isolate, "drawIndexedIndirect"),
            v8::FunctionTemplate::New(isolate, &DrawIndexedIndirect));

    tmpl->Set(
            ConvertToV8String(isolate, "drawIndirect"),
            v8::FunctionTemplate::New(isolate, &DrawIndirect));

    tmpl->Set(
            ConvertToV8String(isolate, "end"),
            v8::FunctionTemplate::New(isolate, &End));

    tmpl->Set(
            ConvertToV8String(isolate, "endOcclusionQuery"),
            v8::FunctionTemplate::New(isolate, &EndOcclusionQuery));

    tmpl->Set(
            ConvertToV8String(isolate, "executeBundles"),
            v8::FunctionTemplate::New(isolate, &ExecuteBundles));

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
            ConvertToV8String(isolate, "setBlendConstant"),
            v8::FunctionTemplate::New(isolate, &SetBlendConstant));

    tmpl->Set(
            ConvertToV8String(isolate, "setIndexBuffer"),
            v8::FunctionTemplate::New(isolate, &SetIndexBuffer));

    tmpl->Set(
            ConvertToV8String(isolate, "setPipeline"),
            v8::FunctionTemplate::New(isolate, &SetPipeline));

    tmpl->Set(
            ConvertToV8String(isolate, "setScissorRect"),
            v8::FunctionTemplate::New(isolate, &SetScissorRect));

    tmpl->Set(
            ConvertToV8String(isolate, "setStencilReference"),
            v8::FunctionTemplate::New(isolate, &SetStencilReference));

    tmpl->Set(
            ConvertToV8String(isolate, "setVertexBuffer"),
            v8::FunctionTemplate::New(isolate, &SetVertexBuffer));

    tmpl->Set(
            ConvertToV8String(isolate, "setViewport"),
            v8::FunctionTemplate::New(isolate, &SetViewport));

    cache->GPURenderPassEncoderTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}


void
GPURenderPassEncoderImpl::BeginOcclusionQuery(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto queryIndexVal = args[0];
    if (queryIndexVal->IsUint32()) {
        auto queryIndex = queryIndexVal->Uint32Value(context).FromJust();
        canvas_native_webgpu_render_pass_encoder_begin_occlusion_query(ptr->GetPass(), queryIndex);
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

void GPURenderPassEncoderImpl::DrawIndexed(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    uint32_t instanceCount = 1;
    uint32_t firstIndex = 0;
    int32_t baseVertex = 0;
    uint32_t firstInstance = 0;


    auto indexCountVal = args[0];
    auto instanceCountVal = args[1];
    auto firstIndexVal = args[2];
    auto baseVertexVal = args[3];
    auto firstInstanceVal = args[4];

    if (indexCountVal->IsUint32()) {

        if (instanceCountVal->IsUint32()) {
            instanceCount = instanceCountVal.As<v8::Uint32>()->Value();
        }

        if (firstIndexVal->IsUint32()) {
            firstIndex = firstIndexVal.As<v8::Uint32>()->Value();
        }

        if (baseVertexVal->IsInt32()) {
            baseVertex = baseVertexVal.As<v8::Int32>()->Value();
        }

        if (firstInstanceVal->IsUint32()) {
            firstInstance = firstInstanceVal.As<v8::Uint32>()->Value();
        }

        auto indexCount = indexCountVal->Uint32Value(
                                                     context).FromJust();
        canvas_native_webgpu_render_pass_encoder_draw_indexed(ptr->GetPass(),
                                                              indexCount,
                                                              instanceCount, firstIndex,
                                                              baseVertex,
                                                              firstInstance);
    }


}

void
GPURenderPassEncoderImpl::DrawIndexedIndirect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto indirectBufferVal = args[0];
    auto indirectOffsetVal = args[1];


    auto indirectBufferType = GetNativeType(indirectBufferVal);

    if (indirectBufferType == NativeType::GPUBuffer) {
        auto indirectBuffer = GPUBufferImpl::GetPointer(indirectBufferVal.As<v8::Object>());
        uint64_t offset = (uint64_t) indirectOffsetVal->NumberValue(context).FromJust();
        canvas_native_webgpu_render_pass_encoder_draw_indexed_indirect(ptr->GetPass(),
                                                                       indirectBuffer->GetGPUBuffer(),
                                                                       offset);
    }


}

void
GPURenderPassEncoderImpl::DrawIndirect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto indirectBufferVal = args[0];
    auto indirectOffsetVal = args[1];


    auto indirectBufferType = GetNativeType(indirectBufferVal);

    if (indirectBufferType == NativeType::GPUBuffer) {
        auto indirectBuffer = GPUBufferImpl::GetPointer(indirectBufferVal.As<v8::Object>());
        uint64_t offset = (uint64_t) indirectOffsetVal->NumberValue(context).FromJust();
        canvas_native_webgpu_render_pass_encoder_draw_indirect(ptr->GetPass(),
                                                               indirectBuffer->GetGPUBuffer(),
                                                               offset);
    }


}

void GPURenderPassEncoderImpl::End(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_webgpu_render_pass_encoder_end(ptr->GetPass());
}

void GPURenderPassEncoderImpl::EndOcclusionQuery(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_webgpu_render_pass_encoder_end_occlusion_query(ptr->GetPass());
}

void GPURenderPassEncoderImpl::ExecuteBundles(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto bundlesVal = args[0];
    if (bundlesVal->IsArray()) {
        auto bundlesArray = bundlesVal.As<v8::Array>();
        auto len = bundlesArray->Length();
        std::vector<const CanvasGPURenderBundle *> bundles;
        for (int i = 0; i < len; i++) {
            v8::Local<v8::Value> bundleVal;
            bundlesArray->Get(context, i).ToLocal(&bundlesVal);
            auto type = GetNativeType(bundleVal);
            if (type == NativeType::GPURenderBundle) {
                auto bundle = GPURenderBundleImpl::GetPointer(bundleVal.As<v8::Object>());
                bundles.emplace_back(bundle->GetBundle());
            }
        }

        if (!bundles.empty()) {
            canvas_native_webgpu_render_pass_encoder_execute_bundles(ptr->GetPass(), bundles.data(),
                                                                     bundles.size());
        }


    }
}

void
GPURenderPassEncoderImpl::InsertDebugMarker(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

    auto markerLabelVal = args[0];
    if (markerLabelVal->IsString()) {
        auto markerLabel = ConvertFromV8String(isolate, markerLabelVal);
        canvas_native_webgpu_render_pass_encoder_insert_debug_marker(ptr->GetPass(),
                                                                     markerLabel.c_str());
    }
}

void GPURenderPassEncoderImpl::PopDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_webgpu_render_pass_encoder_pop_debug_group(ptr->GetPass());
}

void GPURenderPassEncoderImpl::PushDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

    auto groupLabelVal = args[0];
    if (groupLabelVal->IsString()) {
        auto groupLabel = ConvertFromV8String(isolate, groupLabelVal);
        canvas_native_webgpu_render_pass_encoder_push_debug_group(ptr->GetPass(),
                                                                  groupLabel.c_str());
    }
}

void GPURenderPassEncoderImpl::SetBindGroup(const v8::FunctionCallbackInfo<v8::Value> &args) {
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
            auto offset_length = (size_t) dynamicOffsetsLength->NumberValue(context).FromJust();
            canvas_native_webgpu_render_pass_encoder_set_bind_group(ptr->GetPass(), index,
                                                                    bindgroup->GetBindGroup(),
                                                                    static_cast<const uint32_t *>(static_cast<void *>(data)),
                                                                    size, start, offset_length);
        } else {
            canvas_native_webgpu_render_pass_encoder_set_bind_group(ptr->GetPass(), index,
                                                                    bindgroup->GetBindGroup(),
                                                                    nullptr, 0, 0, 0);
        }
    }
}

void GPURenderPassEncoderImpl::SetIndexBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

    auto bufferVal = args[0];
    auto indexFormatVal = args[1];
    int64_t offset = -1;
    int64_t size = -1;
    auto offsetVal = args[2];
    auto sizeVal = args[3];

    auto type = GetNativeType(bufferVal);

    if (type == NativeType::GPURenderBundleEncoder) {
        auto buffer = GPUBufferImpl::GetPointer(bufferVal.As<v8::Object>());
        auto indexFormat = ConvertFromV8String(isolate, indexFormatVal);
        if (offsetVal->IsNumber()) {
            offset = (int64_t) offsetVal.As<v8::Number>()->Value();
        }

        if (sizeVal->IsNumber()) {
            size = (int64_t) sizeVal.As<v8::Number>()->Value();
        }

        if (indexFormat == "uint16") {
            canvas_native_webgpu_render_pass_encoder_set_index_buffer(ptr->GetPass(),
                                                                      buffer->GetGPUBuffer(),
                                                                      CanvasIndexFormatUint16,
                                                                      offset,
                                                                      size);
        } else if (indexFormat == "uint32") {
            canvas_native_webgpu_render_pass_encoder_set_index_buffer(ptr->GetPass(),
                                                                      buffer->GetGPUBuffer(),
                                                                      CanvasIndexFormatUint32,
                                                                      offset,
                                                                      size);
        }


    }
}

void GPURenderPassEncoderImpl::SetBlendConstant(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

    auto colorVal = args[0];

    if (colorVal->IsObject() || colorVal->IsArray()) {

        auto color = ParseColor(isolate, colorVal);


        canvas_native_webgpu_render_pass_encoder_set_blend_constant(ptr->GetPass(), &color);
    }

}

void GPURenderPassEncoderImpl::SetPipeline(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto pipelineVal = args[0];
    if(GetNativeType(pipelineVal) == NativeType::GPURenderPipeline){
        auto pipeline = GPURenderPipelineImpl::GetPointer(pipelineVal.As<v8::Object>());
        canvas_native_webgpu_render_pass_encoder_set_pipeline(ptr->GetPass(),
                                                              pipeline->GetGPUPipeline());
    }
}

void GPURenderPassEncoderImpl::SetScissorRect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto x = args[0];
    auto y = args[1];
    auto width = args[2];
    auto height = args[3];

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    canvas_native_webgpu_render_pass_encoder_set_scissor_rect(ptr->GetPass(),
                                                              x->Uint32Value(context).FromJust(),
                                                              y->Uint32Value(context).FromJust(),
                                                              width->Uint32Value(
                                                                      context).FromJust(),
                                                              height->Uint32Value(
                                                                      context).FromJust());
}

void
GPURenderPassEncoderImpl::SetStencilReference(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto reference = args[0];

    if (reference->IsUint32()) {
        canvas_native_webgpu_render_pass_encoder_set_stencil_reference(ptr->GetPass(),
                                                                       reference->Uint32Value(
                                                                               context).FromJust());
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
        if (GetNativeType(bufferVal) != NativeType::GPUBuffer) {
            // todo throw ??
            return;
        }

        auto buffer = GPUBufferImpl::GetPointer(bufferVal.As<v8::Object>());

        if (offsetVal->IsNumber()) {
            offset = (int64_t) offsetVal.As<v8::Number>()->Value();
        }

        if (sizeVal->IsNumber()) {
            size = (int64_t) sizeVal.As<v8::Number>()->Value();
        }

        canvas_native_webgpu_render_pass_encoder_set_vertex_buffer(ptr->GetPass(), slot,
                                                                   buffer->GetGPUBuffer(), offset,
                                                                   size);

    }
}

void GPURenderPassEncoderImpl::SetViewport(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto x = args[0];
    auto y = args[1];
    auto width = args[2];
    auto height = args[3];

    auto minDepth = args[4];
    auto maxDepth = args[5];

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    canvas_native_webgpu_render_pass_encoder_set_viewport(ptr->GetPass(),
                                                          (float) x->NumberValue(
                                                                  context).FromJust(),
                                                          (float) y->NumberValue(
                                                                  context).FromJust(),
                                                          (float) width->NumberValue(
                                                                  context).FromJust(),
                                                          (float) height->NumberValue(
                                                                  context).FromJust(),
                                                          (float) minDepth->NumberValue(
                                                                  context).FromJust(),
                                                          (float) maxDepth->NumberValue(
                                                                  context).FromJust());

}
