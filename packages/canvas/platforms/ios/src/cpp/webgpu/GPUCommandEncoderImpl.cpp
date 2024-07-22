//
// Created by Osei Fortune on 23/06/2024.
//

#include "GPUCommandEncoderImpl.h"
#include "Caches.h"
#include "GPUComputePassEncoderImpl.h"
#include "GPUQuerySetImpl.h"
#include "GPUBufferImpl.h"
#include "GPUTextureViewImpl.h"
#include "GPURenderPassEncoderImpl.h"
#include "GPUCommandBufferImpl.h"
#include "GPUTextureImpl.h"

GPUCommandEncoderImpl::GPUCommandEncoderImpl(const CanvasGPUCommandEncoder *encoder) : encoder_(
        encoder) {}

const CanvasGPUCommandEncoder *GPUCommandEncoderImpl::GetEncoder() {
    return this->encoder_;
}

void GPUCommandEncoderImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUCommandEncoder"), func).FromJust();;
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
            ConvertToV8String(isolate, "copyBufferToBuffer"),
            v8::FunctionTemplate::New(isolate, &CopyBufferToBuffer));

    tmpl->Set(
            ConvertToV8String(isolate, "copyBufferToTexture"),
            v8::FunctionTemplate::New(isolate, &CopyBufferToTexture));

    tmpl->Set(
            ConvertToV8String(isolate, "copyTextureToBuffer"),
            v8::FunctionTemplate::New(isolate, &CopyTextureToBuffer));

    tmpl->Set(
            ConvertToV8String(isolate, "copyTextureToTexture"),
            v8::FunctionTemplate::New(isolate, &CopyTextureToTexture));

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
            ConvertToV8String(isolate, "resolveQuerySet"),
            v8::FunctionTemplate::New(isolate, &ResolveQuerySet));

    tmpl->Set(
            ConvertToV8String(isolate, "writeTimestamp"),
            v8::FunctionTemplate::New(isolate, &WriteTimestamp));


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

    const CanvasGPUComputePassEncoder *pass;

    if (!descVal->IsNullOrUndefined() && descVal->IsObject()) {
        auto desc = descVal.As<v8::Object>();


        v8::Local<v8::Value> labelVal;
        desc->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);

        char *label = nullptr;

        if (!labelVal.IsEmpty() && labelVal->IsString()) {
            label = *v8::String::Utf8Value(isolate, labelVal);
        }

        const CanvasGPUQuerySet *querySet = nullptr;
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
        auto value = new GPUComputePassEncoderImpl(pass);
        auto ret = GPUComputePassEncoderImpl::NewInstance(isolate, value);
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

    const CanvasGPURenderPassEncoder *pass = nullptr;

    std::vector<CanvasRenderPassColorAttachment> colorAttachments_;

    if (!descVal->IsNullOrUndefined() && descVal->IsObject()) {
        auto desc = descVal.As<v8::Object>();


        v8::Local<v8::Value> labelVal;
        desc->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);

        char *label = nullptr;

        if (!labelVal.IsEmpty() && labelVal->IsString()) {
            label = *v8::String::Utf8Value(isolate, labelVal);
        }


        v8::Local<v8::Value> colorAttachmentsVal;
        desc->Get(context, ConvertToV8String(isolate, "colorAttachments")).ToLocal(
                &colorAttachmentsVal);

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


            const CanvasGPUTextureView *resolve_target = nullptr;

            v8::Local<v8::Value> resolve_target_val;

            colorAttachment->Get(context, ConvertToV8String(isolate, "resolveTarget")).ToLocal(
                    &resolve_target_val);

            auto resolve_target_type = GetNativeType(resolve_target_val);

            if (resolve_target_type == NativeType::GPUTextureView) {
                auto res = GPUTextureViewImpl::GetPointer(resolve_target_val.As<v8::Object>());
                resolve_target = res->GetTextureView();
            }

            // default
            CanvasLoadOp load = CanvasLoadOp::CanvasLoadOpClear;
            CanvasStoreOp store = CanvasStoreOp::CanvasStoreOpStore;
            auto loadVal = colorAttachment->Get(context, ConvertToV8String(isolate,
                                                                           "loadOp")).ToLocalChecked();

            if (loadVal->IsUint32()) {
                load = (CanvasLoadOp) loadVal->Uint32Value(
                        context).ToChecked();
            } else if (loadVal->IsString()) {
                auto val = ConvertFromV8String(isolate, loadVal);
                if (val == "clear") {
                    load = CanvasLoadOp::CanvasLoadOpClear;
                } else if (val == "load") {
                    load = CanvasLoadOp::CanvasLoadOpLoad;
                }
            }


            auto storeVal = colorAttachment->Get(context, ConvertToV8String(isolate,
                                                                            "storeOp")).ToLocalChecked();

            if (!storeVal.IsEmpty() && storeVal->IsUint32()) {
                store = (CanvasStoreOp) storeVal->Uint32Value(
                        context).ToChecked();
            } else if (storeVal->IsString()) {
                auto val = ConvertFromV8String(isolate, storeVal);
                if (val == "discard") {
                    store = CanvasStoreOp::CanvasStoreOpDiscard;
                } else if (val == "store") {
                    store = CanvasStoreOp::CanvasStoreOpStore;
                }
            }

            CanvasPassChannelColor channel{
                    load,
                    store,
                    clearValue,
                    false
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


        const CanvasGPUQuerySet *occlusion_query_set = nullptr;
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


        const CanvasGPUQuerySet *querySet = nullptr;
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


            auto beginningOfPassWriteIndexValSuccess = timestampWrites->Get(context,
                                                                            ConvertToV8String(
                                                                                    isolate,
                                                                                    "beginningOfPassWriteIndex")).ToLocal(
                    &beginningOfPassWriteIndexVal);

            auto endOfPassWriteIndexValSuccess = timestampWrites->Get(context,
                                                                      ConvertToV8String(isolate,
                                                                                        "endOfPassWriteIndex")).ToLocal(
                    &endOfPassWriteIndexVal);


            if (beginningOfPassWriteIndexValSuccess && beginningOfPassWriteIndexVal->IsInt32()) {
                beginningOfPassWriteIndex = beginningOfPassWriteIndexVal.As<v8::Int32>()->Value();
            }

            if (endOfPassWriteIndexValSuccess && endOfPassWriteIndexVal->IsInt32()) {
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

    auto bufferVal = args[0];
    const CanvasGPUBuffer *buffer = nullptr;

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

void GPUCommandEncoderImpl::CopyBufferToBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto source = args[0];
    auto sourceType = GetNativeType(source);
    auto sourceOffset = args[1];
    auto destination = args[2];
    auto destinationType = GetNativeType(destination);
    auto destinationOffset = args[3];
    auto size = args[4];

    if (sourceType == NativeType::GPUBuffer && destinationType == NativeType::GPUBuffer) {
        auto src = GPUBufferImpl::GetPointer(source.As<v8::Object>());
        auto dst = GPUBufferImpl::GetPointer(destination.As<v8::Object>());
        canvas_native_webgpu_command_encoder_copy_buffer_to_buffer(ptr->GetEncoder(),
                                                                   src->GetGPUBuffer(),
                                                                   (int64_t) sourceOffset->NumberValue(
                                                                           context).FromJust(),
                                                                   dst->GetGPUBuffer(),
                                                                   (int64_t) destinationOffset->NumberValue(
                                                                           context).FromJust(),
                                                                   (uint64_t) size->NumberValue(
                                                                           context).FromJust()
        );
    }

}

void GPUCommandEncoderImpl::CopyBufferToTexture(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }


    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto source = args[0];
    auto destination = args[1];
    auto copySize = args[2];

    if (source->IsObject() && destination->IsObject() && copySize->IsObject()) {
        const CanvasGPUBuffer *buffer = nullptr;
        auto src = source.As<v8::Object>();
        v8::Local<v8::Value> bufferVal;
        src->Get(context, ConvertToV8String(isolate, "buffer")).ToLocal(&bufferVal);
        if (GetNativeType(bufferVal) == NativeType::GPUBuffer) {
            buffer = GPUBufferImpl::GetPointer(bufferVal.As<v8::Object>())->GetGPUBuffer();
        }
        uint64_t offset = 0;
        int32_t rowsPerImage = -1;

        v8::Local<v8::Value> rowsPerImageVal;

        src->Get(context, ConvertToV8String(isolate, "rowsPerImage")).ToLocal(&rowsPerImageVal);

        if (!rowsPerImageVal.IsEmpty() && rowsPerImageVal->IsInt32()) {
            rowsPerImage = rowsPerImageVal->Int32Value(context).FromJust();
        }


        v8::Local<v8::Value> offsetVal;

        src->Get(context, ConvertToV8String(isolate, "offset")).ToLocal(&offsetVal);

        if (!offsetVal.IsEmpty() && offsetVal->IsNumber()) {
            offset = (int64_t) offsetVal->NumberValue(context).FromJust();
        }

        v8::Local<v8::Value> bytesPerRowVal;

        src->Get(context, ConvertToV8String(isolate, "bytesPerRow")).ToLocal(&bytesPerRowVal);

        CanvasImageCopyBuffer copy{
                buffer,
                offset,
                bytesPerRowVal->Int32Value(context).FromJust(),
                rowsPerImage
        };


        uint32_t mipLevel = 0;
        CanvasOrigin3d origin{0, 0, 0};
        CanvasTextureAspect aspect = CanvasTextureAspectAll;

        const CanvasGPUTexture *texture = nullptr;
        auto dst = destination.As<v8::Object>();


        v8::Local<v8::Value> textureVal;
        dst->Get(context, ConvertToV8String(isolate, "texture")).ToLocal(&textureVal);
        if (GetNativeType(textureVal) == NativeType::GPUTexture) {
            texture = GPUTextureImpl::GetPointer(textureVal.As<v8::Object>())->GetTexture();
        }


        v8::Local<v8::Value> mipLevelVal;
        dst->Get(context, ConvertToV8String(isolate, "mipLevel")).ToLocal(&mipLevelVal);

        if (!mipLevelVal.IsEmpty() && mipLevelVal->IsUint32()) {
            mipLevel = mipLevelVal->Uint32Value(context).FromJust();
        }

        v8::Local<v8::Value> originVal;
        dst->Get(context, ConvertToV8String(isolate, "origin")).ToLocal(&originVal);


        if (!originVal.IsEmpty() && originVal->IsObject()) {
            auto originObj = originVal.As<v8::Object>();


            v8::Local<v8::Value> xVal;
            v8::Local<v8::Value> yVal;
            v8::Local<v8::Value> zVal;
            originObj->Get(context, ConvertToV8String(isolate, "x")).ToLocal(&xVal);
            originObj->Get(context, ConvertToV8String(isolate, "y")).ToLocal(&yVal);
            originObj->Get(context, ConvertToV8String(isolate, "z")).ToLocal(&zVal);

            if (xVal->IsUint32()) {
                origin.x = xVal->Uint32Value(context).FromJust();
            }
            if (yVal->IsUint32()) {
                origin.y = yVal->Uint32Value(context).FromJust();
            }
            if (zVal->IsUint32()) {
                origin.z = zVal->Uint32Value(context).FromJust();
            }

        }


        v8::Local<v8::Value> aspectVal;
        dst->Get(context, ConvertToV8String(isolate, "aspect")).ToLocal(&aspectVal);


        auto aspectStr = ConvertFromV8String(isolate, aspectVal);

        if (aspectStr == "stencil-only") {
            aspect = CanvasTextureAspectStencilOnly;
        } else if (aspectStr == "depth-only") {
            aspect = CanvasTextureAspectDepthOnly;
        }

        CanvasImageCopyTexture ct{
                texture, mipLevel, origin, aspect
        };

        CanvasExtent3d cz = ParseExtent3d(isolate, copySize);
        canvas_native_webgpu_command_encoder_copy_buffer_to_texture(
                ptr->GetEncoder(),
                &copy,
                &ct,
                &cz
        );
    }


}

void GPUCommandEncoderImpl::CopyTextureToBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }


    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    // copying texture to buffer swapped the real source and dst
    auto source = args[1];
    auto destination = args[0];
    auto copySize = args[2];

    if (source->IsObject() && destination->IsObject() && copySize->IsObject()) {
        const CanvasGPUBuffer *buffer = nullptr;
        auto src = source.As<v8::Object>();

        v8::Local<v8::Value> bufferVal;
        src->Get(context, ConvertToV8String(isolate, "buffer")).ToLocal(&bufferVal);
        if (GetNativeType(bufferVal) == NativeType::GPUBuffer) {
            buffer = GPUBufferImpl::GetPointer(bufferVal.As<v8::Object>())->GetGPUBuffer();
        }
        uint64_t offset = 0;
        int32_t rowsPerImage = -1;

        v8::Local<v8::Value> rowsPerImageVal;

        src->Get(context, ConvertToV8String(isolate, "rowsPerImage")).ToLocal(&rowsPerImageVal);

        if (!rowsPerImageVal.IsEmpty() && rowsPerImageVal->IsInt32()) {
            rowsPerImage = rowsPerImageVal->Int32Value(context).FromJust();
        }


        v8::Local<v8::Value> offsetVal;

        src->Get(context, ConvertToV8String(isolate, "offset")).ToLocal(&offsetVal);

        if (!offsetVal.IsEmpty() && offsetVal->IsNumber()) {
            offset = (int64_t) offsetVal->NumberValue(context).FromJust();
        }

        v8::Local<v8::Value> bytesPerRowVal;

        src->Get(context, ConvertToV8String(isolate, "bytesPerRow")).ToLocal(&bytesPerRowVal);

        CanvasImageCopyBuffer copy{
                buffer,
                offset,
                bytesPerRowVal->Int32Value(context).FromJust(),
                rowsPerImage
        };


        uint32_t mipLevel = 0;
        CanvasOrigin3d origin{0, 0, 0};
        CanvasTextureAspect aspect = CanvasTextureAspectAll;

        const CanvasGPUTexture *texture = nullptr;
        auto dst = destination.As<v8::Object>();


        v8::Local<v8::Value> textureVal;
        dst->Get(context, ConvertToV8String(isolate, "texture")).ToLocal(&textureVal);
        if (GetNativeType(textureVal) == NativeType::GPUTexture) {
            texture = GPUTextureImpl::GetPointer(textureVal.As<v8::Object>())->GetTexture();
        }


        v8::Local<v8::Value> mipLevelVal;
        dst->Get(context, ConvertToV8String(isolate, "mipLevel")).ToLocal(&mipLevelVal);

        if (!mipLevelVal.IsEmpty() && mipLevelVal->IsUint32()) {
            mipLevel = mipLevelVal->Uint32Value(context).FromJust();
        }

        v8::Local<v8::Value> originVal;
        dst->Get(context, ConvertToV8String(isolate, "origin")).ToLocal(&originVal);


        if (!originVal.IsEmpty() && originVal->IsObject()) {
            auto originObj = originVal.As<v8::Object>();


            v8::Local<v8::Value> xVal;
            v8::Local<v8::Value> yVal;
            v8::Local<v8::Value> zVal;
            originObj->Get(context, ConvertToV8String(isolate, "x")).ToLocal(&xVal);
            originObj->Get(context, ConvertToV8String(isolate, "y")).ToLocal(&yVal);
            originObj->Get(context, ConvertToV8String(isolate, "z")).ToLocal(&zVal);

            if (xVal->IsUint32()) {
                origin.x = xVal->Uint32Value(context).FromJust();
            }
            if (yVal->IsUint32()) {
                origin.y = yVal->Uint32Value(context).FromJust();
            }
            if (zVal->IsUint32()) {
                origin.z = zVal->Uint32Value(context).FromJust();
            }

        }


        v8::Local<v8::Value> aspectVal;
        dst->Get(context, ConvertToV8String(isolate, "aspect")).ToLocal(&aspectVal);


        auto aspectStr = ConvertFromV8String(isolate, aspectVal);

        if (aspectStr == "stencil-only") {
            aspect = CanvasTextureAspectStencilOnly;
        } else if (aspectStr == "depth-only") {
            aspect = CanvasTextureAspectDepthOnly;
        }

        CanvasImageCopyTexture ct{
                texture, mipLevel, origin, aspect
        };

        CanvasExtent3d cz = ParseExtent3d(isolate, copySize);
        canvas_native_webgpu_command_encoder_copy_texture_to_buffer(
                ptr->GetEncoder(),
                &ct,
                &copy,
                &cz
        );
    }


}

void GPUCommandEncoderImpl::CopyTextureToTexture(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    // copying texture to buffer swapped the real source and dst
    auto source = args[0];
    auto destination = args[1];
    auto copySize = args[2];

    if (source->IsObject() && destination->IsObject() && copySize->IsObject()) {
        auto src = source.As<v8::Object>();

        uint32_t mipLevelA = 0;
        CanvasOrigin3d originA{0, 0, 0};
        CanvasTextureAspect aspectA = CanvasTextureAspectAll;

        const CanvasGPUTexture *textureA = nullptr;

        v8::Local<v8::Value> srcTextureVal;
        src->Get(context, ConvertToV8String(isolate, "texture")).ToLocal(&srcTextureVal);
        if (GetNativeType(srcTextureVal) == NativeType::GPUTexture) {
            textureA = GPUTextureImpl::GetPointer(srcTextureVal.As<v8::Object>())->GetTexture();
        }

        v8::Local<v8::Value> mipLevelAVal;
        src->Get(context, ConvertToV8String(isolate, "mipLevel")).ToLocal(&mipLevelAVal);

        if (!mipLevelAVal.IsEmpty() && mipLevelAVal->IsUint32()) {
            mipLevelA = mipLevelAVal->Uint32Value(context).FromJust();
        }

        v8::Local<v8::Value> originAVal;
        src->Get(context, ConvertToV8String(isolate, "origin")).ToLocal(&originAVal);


        if (!originAVal.IsEmpty() && originAVal->IsObject()) {
            auto originObj = originAVal.As<v8::Object>();

            v8::Local<v8::Value> xVal;
            v8::Local<v8::Value> yVal;
            v8::Local<v8::Value> zVal;
            originObj->Get(context, ConvertToV8String(isolate, "x")).ToLocal(&xVal);
            originObj->Get(context, ConvertToV8String(isolate, "y")).ToLocal(&yVal);
            originObj->Get(context, ConvertToV8String(isolate, "z")).ToLocal(&zVal);

            if (xVal->IsUint32()) {
                originA.x = xVal->Uint32Value(context).FromJust();
            }
            if (yVal->IsUint32()) {
                originA.y = yVal->Uint32Value(context).FromJust();
            }
            if (zVal->IsUint32()) {
                originA.z = zVal->Uint32Value(context).FromJust();
            }

        }

        v8::Local<v8::Value> aspectAVal;
        src->Get(context, ConvertToV8String(isolate, "aspect")).ToLocal(&aspectAVal);

        auto aspectAStr = ConvertFromV8String(isolate, aspectAVal);

        if (aspectAStr == "stencil-only") {
            aspectA = CanvasTextureAspectStencilOnly;
        } else if (aspectAStr == "depth-only") {
            aspectA = CanvasTextureAspectDepthOnly;
        }
        CanvasImageCopyTexture copy{
                textureA, mipLevelA, originA, aspectA
        };

        uint32_t mipLevel = 0;
        CanvasOrigin3d origin{0, 0, 0};
        CanvasTextureAspect aspect = CanvasTextureAspectAll;

        const CanvasGPUTexture *texture = nullptr;
        auto dst = destination.As<v8::Object>();


        v8::Local<v8::Value> textureVal;
        dst->Get(context, ConvertToV8String(isolate, "texture")).ToLocal(&textureVal);
        if (GetNativeType(textureVal) == NativeType::GPUTexture) {
            texture = GPUTextureImpl::GetPointer(textureVal.As<v8::Object>())->GetTexture();
        }


        v8::Local<v8::Value> mipLevelVal;
        dst->Get(context, ConvertToV8String(isolate, "mipLevel")).ToLocal(&mipLevelVal);

        if (!mipLevelVal.IsEmpty() && mipLevelVal->IsUint32()) {
            mipLevel = mipLevelVal->Uint32Value(context).FromJust();
        }

        v8::Local<v8::Value> originVal;
        dst->Get(context, ConvertToV8String(isolate, "origin")).ToLocal(&originVal);

        if (!originVal.IsEmpty() && originVal->IsObject()) {
            auto originObj = originVal.As<v8::Object>();

            v8::Local<v8::Value> xVal;
            v8::Local<v8::Value> yVal;
            v8::Local<v8::Value> zVal;
            originObj->Get(context, ConvertToV8String(isolate, "x")).ToLocal(&xVal);
            originObj->Get(context, ConvertToV8String(isolate, "y")).ToLocal(&yVal);
            originObj->Get(context, ConvertToV8String(isolate, "z")).ToLocal(&zVal);

            if (xVal->IsUint32()) {
                origin.x = xVal->Uint32Value(context).FromJust();
            }
            if (yVal->IsUint32()) {
                origin.y = yVal->Uint32Value(context).FromJust();
            }
            if (zVal->IsUint32()) {
                origin.z = zVal->Uint32Value(context).FromJust();
            }

        }


        v8::Local<v8::Value> aspectVal;
        dst->Get(context, ConvertToV8String(isolate, "aspect")).ToLocal(&aspectVal);

        auto aspectStr = ConvertFromV8String(isolate, aspectVal);

        if (aspectStr == "stencil-only") {
            aspect = CanvasTextureAspectStencilOnly;
        } else if (aspectStr == "depth-only") {
            aspect = CanvasTextureAspectDepthOnly;
        }

        CanvasImageCopyTexture ct{
                texture, mipLevel, origin, aspect
        };

        CanvasExtent3d cz = ParseExtent3d(isolate, copySize);

        canvas_native_webgpu_command_encoder_copy_texture_to_texture(
                ptr->GetEncoder(),
                &copy,
                &ct,
                &cz
        );
    }


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
    bool didSet = false;
    if (descVal->IsObject()) {
        auto desc = descVal.As<v8::Object>();
        v8::Local<v8::Value> labelVal;
        desc->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);
        if (!labelVal.IsEmpty() && labelVal->IsString()) {
            label = ConvertFromV8String(isolate, labelVal);
            didSet = true;
        }
    }

    auto value = canvas_native_webgpu_command_encoder_finish(ptr->GetEncoder(),
                                                             didSet ? label.c_str() : nullptr);

    if (value != nullptr) {
        auto ret = GPUCommandBufferImpl::NewInstance(isolate, new GPUCommandBufferImpl(value));
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetUndefined();
}

void
GPUCommandEncoderImpl::InsertDebugMarker(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

    auto markerLabelVal = args[0];
    if (markerLabelVal->IsString()) {
        auto markerLabel = ConvertFromV8String(isolate, markerLabelVal);
        canvas_native_webgpu_command_encoder_insert_debug_marker(ptr->GetEncoder(),
                                                                 markerLabel.c_str());
    }
}

void GPUCommandEncoderImpl::PopDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_webgpu_command_encoder_pop_debug_group(ptr->GetEncoder());
}

void GPUCommandEncoderImpl::PushDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

    auto groupLabelVal = args[0];
    if (groupLabelVal->IsString()) {
        auto groupLabel = ConvertFromV8String(isolate, groupLabelVal);
        canvas_native_webgpu_command_encoder_push_debug_group(ptr->GetEncoder(),
                                                              groupLabel.c_str());
    }
}

void GPUCommandEncoderImpl::ResolveQuerySet(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto querySet = args[0];
    auto queryType = GetNativeType(querySet);
    auto firstQuery = args[1];
    auto queryCount = args[2];
    auto destination = args[3];
    auto destinationType = GetNativeType(destination);
    auto destinationOffset = args[4];


    if (queryType == NativeType::GPUQuerySet && destinationType == NativeType::GPUBuffer) {
        auto qs = GPUQuerySetImpl::GetPointer(querySet.As<v8::Object>());
        auto dest = GPUBufferImpl::GetPointer(destination.As<v8::Object>());
        canvas_native_webgpu_command_encoder_resolve_query_set(ptr->GetEncoder(), qs->GetQuerySet(),
                                                               firstQuery->Uint32Value(
                                                                       context).FromJust(),
                                                               queryCount->Uint32Value(
                                                                       context).FromJust(),
                                                               dest->GetGPUBuffer(),
                                                               (uint64_t) destinationOffset->NumberValue(
                                                                       context).FromJust());
    }
}

void GPUCommandEncoderImpl::WriteTimestamp(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto querySet = args[0];
    auto queryType = GetNativeType(querySet);
    auto queryIndex = args[1];

    if (queryType == NativeType::GPUQuerySet) {
        auto qs = GPUQuerySetImpl::GetPointer(querySet.As<v8::Object>());
        canvas_native_webgpu_command_encoder_write_timestamp(ptr->GetEncoder(), qs->GetQuerySet(),
                                                             queryIndex->Uint32Value(
                                                                     context).FromJust());
    }


}
