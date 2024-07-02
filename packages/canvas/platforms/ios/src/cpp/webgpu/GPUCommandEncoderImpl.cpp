//
// Created by Osei Fortune on 23/06/2024.
//

#include "GPUCommandEncoderImpl.h"
#include "Caches.h"
#include "GPUComputePassImpl.h"
#include "GPUQuerySetImpl.h"
#include "GPUBufferImpl.h"
#include "GPUTextureViewImpl.h"
#include "GPURenderPassEncoderImpl.h"
#include "GPUCommandBufferImpl.h"

GPUCommandEncoderImpl::GPUCommandEncoderImpl(CanvasGPUCommandEncoder *encoder) : encoder_(
        encoder) {}

CanvasGPUCommandEncoder *GPUCommandEncoderImpl::GetEncoder() {
    return this->encoder_;
}


void GPUCommandEncoderImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUCommandEncoder"), func);
}

GPUCommandEncoderImpl *GPUCommandEncoderImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUCommandEncoderImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUCommandEncoderImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUCommandEncoderTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUCommandEncoder"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);


    tmpl->Set(
            ConvertToV8String(isolate, "beginComputePass"),
            v8::FunctionTemplate::New(isolate, &BeginComputePass));

    tmpl->Set(
            ConvertToV8String(isolate, "beginRenderPass"),
            v8::FunctionTemplate::New(isolate, &BeginRenderPass));

    tmpl->Set(
            ConvertToV8String(isolate, "clearBuffer"),
            v8::FunctionTemplate::New(isolate, &ClearBuffer));

    tmpl->Set(
            ConvertToV8String(isolate, "finish"),
            v8::FunctionTemplate::New(isolate, &Finish));

    cache->GPUCommandEncoderTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}


void GPUCommandEncoderImpl::BeginComputePass(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto descVal = args[0];

    CanvasGPUComputePass *pass;

    if (!descVal->IsNullOrUndefined() && descVal->IsObject()) {
        auto desc = descVal.As<v8::Object>();


        v8::Local<v8::Value> labelVal;
        desc->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);

        char *label = nullptr;

        if (!labelVal.IsEmpty() && labelVal->IsString()) {
            label = *v8::String::Utf8Value(isolate, labelVal);
        }

        CanvasGPUQuerySet *querySet = nullptr;
        int32_t beginningOfPassWriteIndex = -1;

        int32_t endOfPassWriteIndex = -1;


        v8::Local<v8::Value> timestampWritesVal;
        desc->Get(context, ConvertToV8String(isolate, "timestampWrites")).ToLocal(
                &timestampWritesVal);


        if (!timestampWritesVal.IsEmpty() && timestampWritesVal->IsObject()) {
            auto timestampWrites = timestampWritesVal.As<v8::Object>();

            v8::Local<v8::Value> querySetVal;
            timestampWrites->Get(context, ConvertToV8String(isolate, "querySet")).ToLocal(
                    &querySetVal);


            if (!querySetVal.IsEmpty() && querySetVal->IsObject()) {
                auto queryPtr = GPUQuerySetImpl::GetPointer(querySetVal.As<v8::Object>());
                if (queryPtr != nullptr) {
                    querySet = queryPtr->GetQuerySet();
                }
            }

            v8::Local<v8::Value> beginningOfPassWriteIndexVal;


            v8::Local<v8::Value> endOfPassWriteIndexVal;


            timestampWrites->Get(context,
                                 ConvertToV8String(isolate, "beginningOfPassWriteIndex")).ToLocal(
                    &beginningOfPassWriteIndexVal);

            timestampWrites->Get(context,
                                 ConvertToV8String(isolate, "endOfPassWriteIndex")).ToLocal(
                    &endOfPassWriteIndexVal);


            if (beginningOfPassWriteIndexVal->IsInt32()) {
                beginningOfPassWriteIndex = beginningOfPassWriteIndexVal.As<v8::Int32>()->Value();
            }

            if (endOfPassWriteIndexVal->IsInt32()) {
                endOfPassWriteIndex = endOfPassWriteIndexVal.As<v8::Int32>()->Value();
            }
        }


        pass = canvas_native_webgpu_command_encoder_begin_compute_pass(ptr->GetEncoder(),
                                                                       querySet, label,
                                                                       beginningOfPassWriteIndex,
                                                                       endOfPassWriteIndex);


    } else {
        pass = canvas_native_webgpu_command_encoder_begin_compute_pass(ptr->GetEncoder(),
                                                                       nullptr, nullptr, -1,
                                                                       -1);

    }


    if (pass != nullptr) {
        auto value = new GPUComputePassImpl(pass);
        auto ret = GPUComputePassImpl::NewInstance(isolate, value);
        args.GetReturnValue().Set(ret);
    } else {
        args.GetReturnValue().SetUndefined();
    }

}

void GPUCommandEncoderImpl::BeginRenderPass(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto descVal = args[0];

    CanvasGPURenderPass *pass = nullptr;

    if (!descVal->IsNullOrUndefined() && descVal->IsObject()) {
        auto desc = descVal.As<v8::Object>();


        v8::Local<v8::Value> labelVal;
        desc->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);

        char *label = nullptr;

        if (!labelVal.IsEmpty() && labelVal->IsString()) {
            label = *v8::String::Utf8Value(isolate, labelVal);
        }

        std::vector<CanvasRenderPassColorAttachment> colorAttachments_;
        v8::Local<v8::Value> colorAttachmentsVal;
        desc->Get(context, ConvertToV8String(isolate, "colorAttachments")).ToLocal(&colorAttachmentsVal);

        auto colorAttachments = colorAttachmentsVal.As<v8::Array>();
        auto colorAttachmentsLength = colorAttachments->Length();

        for (int i = 0; i < colorAttachmentsLength; i++) {
            auto colorAttachment = colorAttachments->Get(context,
                                                         i).ToLocalChecked().As<v8::Object>();

            v8::Local<v8::Value> clearValueVal;
            colorAttachment->Get(context, ConvertToV8String(isolate, "clearValue")).ToLocal(
                    &clearValueVal);
            auto clearValue = CanvasColor{0, 0, 0, 0};
            if (!clearValueVal.IsEmpty() && clearValueVal->IsObject()) {
                auto clearValueObj = clearValueVal.As<v8::Object>();

                v8::Local<v8::Value> r;
                v8::Local<v8::Value> g;
                v8::Local<v8::Value> b;
                v8::Local<v8::Value> a;

                clearValueObj->Get(context, ConvertToV8String(isolate, "r")).ToLocal(&r);
                clearValueObj->Get(context, ConvertToV8String(isolate, "g")).ToLocal(&g);
                clearValueObj->Get(context, ConvertToV8String(isolate, "b")).ToLocal(&b);
                clearValueObj->Get(context, ConvertToV8String(isolate, "a")).ToLocal(&a);

                if (!r.IsEmpty() && r->IsNumber()) {
                    clearValue.r = r.As<v8::Number>()->Value();
                }

                if (!g.IsEmpty() && g->IsNumber()) {
                    clearValue.g = g.As<v8::Number>()->Value();
                }


                if (!b.IsEmpty() && b->IsNumber()) {
                    clearValue.b = b.As<v8::Number>()->Value();
                }

                if (!a.IsEmpty() && a->IsNumber()) {
                    clearValue.a = a.As<v8::Number>()->Value();
                }
            }


            auto viewVal = colorAttachment->Get(context, ConvertToV8String(isolate,
                                                                           "view")).ToLocalChecked();

            auto view = GPUTextureViewImpl::GetPointer(viewVal.As<v8::Object>());


            CanvasGPUTextureView *resolve_target = nullptr;

            v8::Local<v8::Value> resolve_target_val;

            colorAttachment->Get(context, ConvertToV8String(isolate, "resolveTarget")).ToLocal(
                    &resolve_target_val);

            if (!resolve_target_val.IsEmpty() && resolve_target_val->IsObject()) {
                auto res = GPUTextureViewImpl::GetPointer(resolve_target_val.As<v8::Object>());
                resolve_target = res->GetTextureView();
            }

            auto loadVal = colorAttachment->Get(context, ConvertToV8String(isolate,
                                                                           "loadOp")).ToLocalChecked()->Uint32Value(
                    context).ToChecked();
            auto storeVal = colorAttachment->Get(context, ConvertToV8String(isolate,
                                                                            "storeOp")).ToLocalChecked()->Uint32Value(
                    context).ToChecked();

            CanvasPassChannelColor channel{
                    (CanvasLoadOp) loadVal,
                    (CanvasStoreOp) storeVal
            };

            auto attachment = CanvasRenderPassColorAttachment{
                    view->GetTextureView(),
                    resolve_target,
                    channel
            };

            colorAttachments_.push_back(attachment);

        }


        v8::Local<v8::Value> maxDrawCountVal;

        desc->Get(context, ConvertToV8String(isolate, "maxDrawCount")).ToLocal(&maxDrawCountVal);


        CanvasRenderPassDepthStencilAttachment *depthStencilAttachment = nullptr;
        v8::Local<v8::Value> depthStencilAttachmentVal;

        desc->Get(context, ConvertToV8String(isolate, "depthStencilAttachment")).ToLocal(
                &depthStencilAttachmentVal);


        if (!depthStencilAttachmentVal.IsEmpty() && depthStencilAttachmentVal->IsObject()) {
            auto depthStencilAttachmentObj = depthStencilAttachmentVal.As<v8::Object>();
        }


        CanvasGPUQuerySet *occlusion_query_set = nullptr;
        v8::Local<v8::Value> occlusionQuerySetVal;


        desc->Get(context, ConvertToV8String(isolate, "occlusionQuerySet")).ToLocal(
                &occlusionQuerySetVal);


        if (!occlusionQuerySetVal.IsEmpty() && occlusionQuerySetVal->IsObject()) {
            auto occlusionQuerySet = GPUQuerySetImpl::GetPointer(
                    occlusionQuerySetVal.As<v8::Object>());
            occlusion_query_set = occlusionQuerySet->GetQuerySet();
        }


        v8::Local<v8::Value> timestampWritesVal;
        desc->Get(context, ConvertToV8String(isolate, "timestampWrites")).ToLocal(
                &timestampWritesVal);


        CanvasGPUQuerySet *querySet = nullptr;
        int32_t beginningOfPassWriteIndex = -1;
        int32_t endOfPassWriteIndex = -1;

        if (!timestampWritesVal.IsEmpty() && timestampWritesVal->IsObject()) {
            auto timestampWrites = timestampWritesVal.As<v8::Object>();
            v8::Local<v8::Value> querySetVal;
            timestampWrites->Get(context, ConvertToV8String(isolate, "querySet")).ToLocal(
                    &querySetVal);


            if (!querySetVal.IsEmpty() && querySetVal->IsObject()) {
                auto queryPtr = GPUQuerySetImpl::GetPointer(querySetVal.As<v8::Object>());
                if (queryPtr != nullptr) {
                    querySet = queryPtr->GetQuerySet();
                }
            }

            v8::Local<v8::Value> beginningOfPassWriteIndexVal;

            v8::Local<v8::Value> endOfPassWriteIndexVal;


            timestampWrites->Get(context,
                                 ConvertToV8String(isolate, "beginningOfPassWriteIndex")).ToLocal(
                    &beginningOfPassWriteIndexVal);

            timestampWrites->Get(context,
                                 ConvertToV8String(isolate, "endOfPassWriteIndex")).ToLocal(
                    &endOfPassWriteIndexVal);


            if (beginningOfPassWriteIndexVal->IsInt32()) {
                beginningOfPassWriteIndex = beginningOfPassWriteIndexVal.As<v8::Int32>()->Value();
            }

            if (endOfPassWriteIndexVal->IsInt32()) {
                endOfPassWriteIndex = endOfPassWriteIndexVal.As<v8::Int32>()->Value();
            }

        }


        pass = canvas_native_webgpu_command_encoder_begin_render_pass(
                ptr->GetEncoder(), label, colorAttachments_.data(), colorAttachments_.size(),
                depthStencilAttachment, occlusion_query_set,
                querySet, beginningOfPassWriteIndex, endOfPassWriteIndex
        );


    }


    if (pass != nullptr) {
        auto value = new GPURenderPassEncoderImpl(pass);
        auto ret = GPURenderPassEncoderImpl::NewInstance(isolate, value);
        args.GetReturnValue().Set(ret);
    } else {
        args.GetReturnValue().SetUndefined();
    }

}

void GPUCommandEncoderImpl::ClearBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

    auto bufferVal = args[0];
    CanvasGPUBuffer *buffer = nullptr;

    if (bufferVal->IsObject()) {
        auto bufferPtr = GPUBufferImpl::GetPointer(bufferVal.As<v8::Object>());
        if (bufferPtr != nullptr) {
            buffer = bufferPtr->GetGPUBuffer();
        }
    }

    if (buffer == nullptr) {
        return;
    }

    int64_t offset = -1;
    auto offsetVal = args[1];

    if (offsetVal->IsNumber()) {
        offset = (int64_t) offsetVal.As<v8::Number>()->Value();
    }

    int64_t size = -1;
    auto sizeVal = args[2];

    if (sizeVal->IsNumber()) {
        size = (int64_t) sizeVal.As<v8::Number>()->Value();
    }


    canvas_native_webgpu_command_encoder_clear_buffer(ptr->GetEncoder(), buffer, offset, size);


}

void GPUCommandEncoderImpl::Finish(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto descVal = args[0];
    std::string label;
    if (descVal->IsObject()) {
        auto desc = descVal.As<v8::Object>();
        v8::Local<v8::Value> labelVal;
        desc->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);
        if (!labelVal.IsEmpty() && labelVal->IsString()) {
            label = ConvertFromV8String(isolate, labelVal);
        }
    }

    auto value = canvas_native_webgpu_command_encoder_finish(ptr->GetEncoder(), nullptr);

    if (value != nullptr) {
        auto ret = GPUCommandBufferImpl::NewInstance(isolate, new GPUCommandBufferImpl(value));
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetUndefined();
}
