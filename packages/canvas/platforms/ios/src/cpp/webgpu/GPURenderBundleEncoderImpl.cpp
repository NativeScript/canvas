//
// Created by Osei Fortune on 17/07/2024.
//

#include "GPURenderBundleEncoderImpl.h"
#include "Caches.h"
#include "GPURenderPipelineImpl.h"
#include "GPUBufferImpl.h"
#include "GPURenderBundleImpl.h"
#include "GPUBindGroupImpl.h"

GPURenderBundleEncoderImpl::GPURenderBundleEncoderImpl(const CanvasGPURenderBundleEncoder *encoder)
        : encoder_(
        encoder) {}

const CanvasGPURenderBundleEncoder *GPURenderBundleEncoderImpl::GetEncoder() {
    return this->encoder_;
}


void GPURenderBundleEncoderImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPURenderBundleEncoder"),
                      func).FromJust();
}

GPURenderBundleEncoderImpl *
GPURenderBundleEncoderImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPURenderBundleEncoderImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPURenderBundleEncoderImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPURenderBundleEncoderTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPURenderBundleEncoder"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "label"),
            GetLabel
    );

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
            ConvertToV8String(isolate, "finish"),
            v8::FunctionTemplate::New(isolate, &Finish));

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
            ConvertToV8String(isolate, "setIndexBuffer"),
            v8::FunctionTemplate::New(isolate, &SetIndexBuffer));

    tmpl->Set(
            ConvertToV8String(isolate, "setPipeline"),
            v8::FunctionTemplate::New(isolate, &SetPipeline));

    tmpl->Set(
            ConvertToV8String(isolate, "setVertexBuffer"),
            v8::FunctionTemplate::New(isolate, &SetVertexBuffer));


    cache->GPURenderBundleEncoderTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}


void
GPURenderBundleEncoderImpl::GetLabel(v8::Local<v8::Name> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto label = canvas_native_webgpu_render_bundle_encoder_get_label(ptr->encoder_);
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


void GPURenderBundleEncoderImpl::Draw(const v8::FunctionCallbackInfo<v8::Value> &args) {
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

        canvas_native_webgpu_render_bundle_encoder_draw(ptr->GetEncoder(), vertexCount,
                                                        instanceCount,
                                                        firstVertex, firstInstance);

    }
}

void GPURenderBundleEncoderImpl::DrawIndexed(const v8::FunctionCallbackInfo<v8::Value> &args) {
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

        canvas_native_webgpu_render_bundle_encoder_draw_indexed(ptr->GetEncoder(),
                                                                indexCountVal->Uint32Value(
                                                                        context).FromJust(),
                                                                instanceCount, firstIndex,
                                                                baseVertex,
                                                                firstInstance);
    }


}

void
GPURenderBundleEncoderImpl::DrawIndexedIndirect(const v8::FunctionCallbackInfo<v8::Value> &args) {
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
        canvas_native_webgpu_render_bundle_encoder_draw_indexed_indirect(ptr->GetEncoder(),
                                                                         indirectBuffer->GetGPUBuffer(),
                                                                         offset);
    }


}

void
GPURenderBundleEncoderImpl::DrawIndirect(const v8::FunctionCallbackInfo<v8::Value> &args) {
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
        canvas_native_webgpu_render_bundle_encoder_draw_indirect(ptr->GetEncoder(),
                                                                 indirectBuffer->GetGPUBuffer(),
                                                                 offset);
    }


}

void GPURenderBundleEncoderImpl::Finish(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    std::string label;
    v8::Local<v8::Value> labelVal;

    auto optionsVal = args[0];

    if (optionsVal->IsObject()) {
        auto options = optionsVal.As<v8::Object>();
        options->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);

        label = ConvertFromV8String(isolate, labelVal);
    }


    auto bundle = canvas_native_webgpu_render_bundle_encoder_finish(ptr->GetEncoder(), label.c_str());

    if (bundle != nullptr) {
        auto ret = GPURenderBundleImpl::NewInstance(isolate, new GPURenderBundleImpl(bundle));
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetUndefined();
}

void
GPURenderBundleEncoderImpl::InsertDebugMarker(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

    auto markerLabelVal = args[0];
    if (markerLabelVal->IsString()) {
        auto markerLabel = ConvertFromV8String(isolate, markerLabelVal);
        canvas_native_webgpu_render_bundle_encoder_insert_debug_marker(ptr->GetEncoder(),
                                                                       markerLabel.c_str());
    }
}

void GPURenderBundleEncoderImpl::PopDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_webgpu_render_bundle_encoder_pop_debug_group(ptr->GetEncoder());
}

void GPURenderBundleEncoderImpl::PushDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

    auto groupLabelVal = args[0];
    if (groupLabelVal->IsString()) {
        auto groupLabel = ConvertFromV8String(isolate, groupLabelVal);
        canvas_native_webgpu_render_bundle_encoder_push_debug_group(ptr->GetEncoder(),
                                                                    groupLabel.c_str());
    }
}

void GPURenderBundleEncoderImpl::SetBindGroup(const v8::FunctionCallbackInfo<v8::Value> &args) {
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
        auto bindGroup = GPUBindGroupImpl::GetPointer(bindGroupVal.As<v8::Object>());

        if (dynamicOffsets->IsUint32Array()) {
            auto buf = dynamicOffsets.As<v8::Uint32Array>();
            auto buffer = buf->Buffer();
            auto store = buffer->GetBackingStore();
            auto offset = buf->ByteOffset();
            auto data = static_cast<uint8_t *>(buffer->GetBackingStore()->Data()) + offset;
            auto size = buf->Length();
            auto start = (size_t) dynamicOffsetsStart->NumberValue(context).FromJust();
            auto offset_length = (size_t) dynamicOffsetsLength->NumberValue(context).FromJust();
            canvas_native_webgpu_render_bundle_encoder_set_bind_group(ptr->GetEncoder(), index,
                                                                      bindGroup->GetBindGroup(),
                                                                      static_cast<const uint32_t *>(static_cast<void *>(data)),
                                                                      size, start, offset_length);
        } else {
            canvas_native_webgpu_render_bundle_encoder_set_bind_group(ptr->GetEncoder(), index,
                                                                      bindGroup->GetBindGroup(),
                                                                      nullptr, 0, 0, 0);
        }
    }

}

void GPURenderBundleEncoderImpl::SetIndexBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
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

    if (type == NativeType::GPUBuffer) {
        auto buffer = GPUBufferImpl::GetPointer(bufferVal.As<v8::Object>());
        auto indexFormat = ConvertFromV8String(isolate, indexFormatVal);
        if (offsetVal->IsNumber()) {
            offset = (int64_t) offsetVal.As<v8::Number>()->Value();
        }

        if (sizeVal->IsNumber()) {
            size = (int64_t) sizeVal.As<v8::Number>()->Value();
        }

        if (indexFormat == "uint16") {
            canvas_native_webgpu_render_bundle_encoder_set_index_buffer(ptr->GetEncoder(),
                                                                        buffer->GetGPUBuffer(),
                                                                        CanvasIndexFormatUint16,
                                                                        offset,
                                                                        size);
        } else if (indexFormat == "uint32") {
            canvas_native_webgpu_render_bundle_encoder_set_index_buffer(ptr->GetEncoder(),
                                                                        buffer->GetGPUBuffer(),
                                                                        CanvasIndexFormatUint32,
                                                                        offset,
                                                                        size);
        }


    }
}

void GPURenderBundleEncoderImpl::SetPipeline(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto pipelineVal = args[0];
    if(GetNativeType(pipelineVal) == NativeType::GPURenderPipeline){
        auto pipeline = GPURenderPipelineImpl::GetPointer(pipelineVal.As<v8::Object>());
        if (pipeline != nullptr) {
            canvas_native_webgpu_render_bundle_encoder_set_pipeline(ptr->GetEncoder(),
                                                                    pipeline->GetGPUPipeline());
        }
    }
}

void GPURenderBundleEncoderImpl::SetVertexBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
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

        canvas_native_webgpu_render_bundle_encoder_set_vertex_buffer(ptr->GetEncoder(), slot,
                                                                     buffer->GetGPUBuffer(), offset,
                                                                     size);

    }
}


