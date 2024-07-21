//
// Created by Osei Fortune on 18/06/2024.
//

#include "GPUDeviceImpl.h"
#include "Caches.h"
#include "GPUAdapterImpl.h"
#include "GPUQueueImpl.h"
#include "GPUCommandEncoderImpl.h"
#include "GPUShaderModuleImpl.h"
#include "GPUPipelineLayoutImpl.h"
#include "GPURenderPipelineImpl.h"
#include "JSICallback.h"
#include "GPUBindGroupImpl.h"
#include "GPUBindGroupLayoutImpl.h"
#include "GPUSamplerImpl.h"
#include "GPUTextureViewImpl.h"
#include "GPUBufferImpl.h"
#include "GPUTextureImpl.h"
#include "GPUComputePipelineImpl.h"
#include "GPUQuerySetImpl.h"
#include "GPURenderBundleEncoderImpl.h"
#include "GPUUtils.h"

GPUDeviceImpl::GPUDeviceImpl(const CanvasGPUDevice *device) : device_(device) {}

const CanvasGPUDevice *GPUDeviceImpl::GetGPUDevice() {
    return this->device_;
}


void GPUDeviceImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUDevice"), func).FromJust();;
}

GPUDeviceImpl *GPUDeviceImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUDeviceImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUDeviceImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUDeviceTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUDevice"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "features"),
            GetFeatures
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "queue"),
            GetQueue
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "limits"),
            GetLimits
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "lost"),
            GetLost
    );

    tmpl->Set(
            ConvertToV8String(isolate, "createBindGroup"),
            v8::FunctionTemplate::New(isolate, &CreateBindGroup));

    tmpl->Set(
            ConvertToV8String(isolate, "createBindGroupLayout"),
            v8::FunctionTemplate::New(isolate, &CreateBindGroupLayout));

    tmpl->Set(
            ConvertToV8String(isolate, "createBuffer"),
            v8::FunctionTemplate::New(isolate, &CreateBuffer));

    tmpl->Set(
            ConvertToV8String(isolate, "createCommandEncoder"),
            v8::FunctionTemplate::New(isolate, &CreateCommandEncoder));

    tmpl->Set(
            ConvertToV8String(isolate, "createComputePipeline"),
            v8::FunctionTemplate::New(isolate, &CreateComputePipeline));

    tmpl->Set(
            ConvertToV8String(isolate, "createPipelineLayout"),
            v8::FunctionTemplate::New(isolate, &CreatePipelineLayout));

    tmpl->Set(
            ConvertToV8String(isolate, "createQuerySet"),
            v8::FunctionTemplate::New(isolate, &CreateQuerySet));

    tmpl->Set(
            ConvertToV8String(isolate, "createRenderBundleEncoder"),
            v8::FunctionTemplate::New(isolate, &CreateRenderBundleEncoder));

    tmpl->Set(
            ConvertToV8String(isolate, "createRenderPipeline"),
            v8::FunctionTemplate::New(isolate, &CreateRenderPipeline));

    tmpl->Set(
            ConvertToV8String(isolate, "createSampler"),
            v8::FunctionTemplate::New(isolate, &CreateSampler));

    tmpl->Set(
            ConvertToV8String(isolate, "createShaderModule"),
            v8::FunctionTemplate::New(isolate, &CreateShaderModule));

    tmpl->Set(
            ConvertToV8String(isolate, "createTexture"),
            v8::FunctionTemplate::New(isolate, &CreateTexture));

    tmpl->Set(
            ConvertToV8String(isolate, "destroy"),
            v8::FunctionTemplate::New(isolate, &Destroy));

    tmpl->Set(
            ConvertToV8String(isolate, "popErrorScope"),
            v8::FunctionTemplate::New(isolate, &PopErrorScope));

    tmpl->Set(
            ConvertToV8String(isolate, "pushErrorScope"),
            v8::FunctionTemplate::New(isolate, &PushErrorScope));


    tmpl->Set(
            ConvertToV8String(isolate, "setuncapturederror"),
            v8::FunctionTemplate::New(isolate, &SetUncapturedError));


    cache->GPUDeviceTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void
GPUDeviceImpl::SetUncapturedError(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto cb = args[0];
    auto callback = new JSICallback(isolate, cb.As<v8::Function>());

    canvas_native_webgpu_device_set_uncaptured_error_callback(ptr->GetGPUDevice(),
                                                              [](CanvasGPUErrorType type, char *msg,
                                                                 void *data) {

                                                                  auto cb = static_cast<JSICallback *>(data);

                                                                  v8::Isolate *isolate = cb->isolate_;
                                                                  v8::Locker locker(isolate);
                                                                  v8::Isolate::Scope isolate_scope(
                                                                          isolate);
                                                                  v8::HandleScope handle_scope(
                                                                          isolate);
                                                                  v8::Local<v8::Function> callback = cb->callback_->Get(
                                                                          isolate);
                                                                  v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                                                                  v8::Context::Scope context_scope(
                                                                          context);
                                                                  v8::Local<v8::Value> typ;
                                                                  v8::Local<v8::Value> message;
                                                                  switch (type) {
                                                                      case CanvasGPUErrorType::CanvasGPUErrorTypeNone:
                                                                          typ = v8::Number::New(
                                                                                  isolate, 0);
                                                                          message = v8::Null(
                                                                                  isolate);
                                                                          break;
                                                                      case CanvasGPUErrorType::CanvasGPUErrorTypeLost:
                                                                          typ = v8::Number::New(
                                                                                  isolate, 1);
                                                                          message = v8::Null(
                                                                                  isolate);
                                                                          break;
                                                                      case CanvasGPUErrorType::CanvasGPUErrorTypeOutOfMemory:
                                                                          typ = v8::Number::New(
                                                                                  isolate, 2);
                                                                          message = v8::Null(
                                                                                  isolate);
                                                                          break;
                                                                      case CanvasGPUErrorType::CanvasGPUErrorTypeValidation:
                                                                          typ = v8::Number::New(
                                                                                  isolate, 3);
                                                                          message = ConvertToV8String(
                                                                                  isolate, msg);
                                                                          break;
                                                                      case CanvasGPUErrorType::CanvasGPUErrorTypeInternal:
                                                                          typ = v8::Number::New(
                                                                                  isolate, 4);
                                                                          message = v8::Null(
                                                                                  isolate);
                                                                          break;
                                                                  }
                                                                  v8::Local<v8::Value> args[2] = {
                                                                          typ, message
                                                                  };

                                                                  callback->Call(context,
                                                                                 context->Global(),
                                                                                 2, args);

                                                                  // todo clean up
//                                                                  delete static_cast<JSICallback *>(data);

                                                              }, callback);


}


void
GPUDeviceImpl::GetFeatures(v8::Local<v8::Name> name,
                           const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto isolate = info.GetIsolate();
    if (ptr != nullptr) {
        auto context = isolate->GetCurrentContext();

        auto features = canvas_native_webgpu_device_get_features(ptr->GetGPUDevice());

        auto len = canvas_native_string_buffer_get_length(features);

        auto map = v8::Map::New(isolate);
        for (int i = 0; i < len; ++i) {
            auto item = canvas_native_string_buffer_get_value_at(features, i);
            if (item != nullptr) {
                auto keyValue = ConvertToV8OneByteString(isolate, (char *) item);
                map->Set(context, keyValue, keyValue);
                canvas_native_string_destroy(item);
            }

        }
        canvas_native_string_buffer_destroy(features);

        info.GetReturnValue().Set(map);

        return;
    }

    info.GetReturnValue().Set(v8::Map::New(info.GetIsolate()));
}


void
GPUDeviceImpl::GetLimits(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto limits = canvas_native_webgpu_device_get_limits(ptr->GetGPUDevice());

        auto ret = GPUSupportedLimitsImpl::NewInstance(info.GetIsolate(),
                                                       new GPUSupportedLimitsImpl(limits));
        info.GetReturnValue().Set(ret);
        return;
    }
    info.GetReturnValue().SetUndefined();
}


void
GPUDeviceImpl::GetQueue(v8::Local<v8::Name> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto queue = canvas_native_webgpu_device_get_queue(ptr->GetGPUDevice());
        auto ret = GPUQueueImpl::NewInstance(info.GetIsolate(),
                                             new GPUQueueImpl(queue));
        info.GetReturnValue().Set(ret);
        return;
    }
    info.GetReturnValue().SetUndefined();
}


void
GPUDeviceImpl::GetLost(v8::Local<v8::Name> name,
                       const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto isolate = info.GetIsolate();
    auto resolver = v8::Promise::Resolver::New(isolate->GetCurrentContext()).ToLocalChecked();
    info.GetReturnValue().Set(resolver->GetPromise());
    if (ptr != nullptr) {
        v8::Global<v8::Promise::Resolver> promise(isolate, resolver);
        auto callback = new PromiseCallback{
                isolate,
                std::move(promise)
        };

        canvas_native_webgpu_device_set_lost_callback(ptr->GetGPUDevice(),
                                                      [](int32_t reason, char *message,
                                                         void *data) {
                                                          if (data != nullptr) {
                                                              auto func = static_cast<PromiseCallback *>(data);
                                                              if (func->isolate != nullptr) {
                                                                  v8::Isolate *isolate = func->isolate;
                                                                  v8::Locker locker(isolate);
                                                                  v8::Isolate::Scope isolate_scope(
                                                                          isolate);
                                                                  v8::HandleScope handle_scope(
                                                                          isolate);
                                                                  v8::Local<v8::Promise::Resolver> callback = func->callback.Get(
                                                                          isolate);
                                                                  v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                                                                  v8::Context::Scope context_scope(
                                                                          context);

                                                                  auto ret = v8::Object::New(
                                                                          isolate);
                                                                  if (message == nullptr) {
                                                                      ret->Set(context,
                                                                               ConvertToV8String(
                                                                                       isolate,
                                                                                       "message"),
                                                                               v8::String::Empty(
                                                                                       isolate));
                                                                  } else {
                                                                      ret->Set(context,
                                                                               ConvertToV8String(
                                                                                       isolate,
                                                                                       "message"),
                                                                               ConvertToV8String(
                                                                                       isolate,
                                                                                       message));
                                                                      canvas_native_string_destroy(
                                                                              message);
                                                                  }

                                                                  ret->Set(context,
                                                                           ConvertToV8String(
                                                                                   isolate,
                                                                                   "reason"),
                                                                           v8::Int32::New(isolate,
                                                                                          reason)).IsJust();


                                                                  callback->Resolve(context, ret);
                                                                  delete static_cast<PromiseCallback *>(data);
                                                              }
                                                          }

                                                      }, callback);
    }
}


void GPUDeviceImpl::CreateBindGroup(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    char *label = nullptr;

    auto optionsVal = args[0];

    if (optionsVal->IsObject()) {
        auto options = optionsVal.As<v8::Object>();
        v8::Local<v8::Value> labelVal;
        options->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);

        if (!labelVal.IsEmpty() && labelVal->IsString()) {
            label = *v8::String::Utf8Value(isolate, labelVal);
        }

        const CanvasGPUBindGroupLayout *layout = nullptr;


        v8::Local<v8::Value> layoutVal;
        options->Get(context, ConvertToV8String(isolate, "layout")).ToLocal(&layoutVal);

        if (!layoutVal.IsEmpty() && layoutVal->IsObject()) {
            auto layoutObj = layoutVal.As<v8::Object>();
            auto layoutImpl = GPUBindGroupLayoutImpl::GetPointer(layoutObj);
            if (layoutImpl != nullptr) {
                layout = layoutImpl->GetBindGroupLayout();
            }
        }

        std::vector<CanvasBindGroupEntry> entries;

        v8::Local<v8::Value> entriesVal;
        options->Get(context, ConvertToV8String(isolate, "entries")).ToLocal(&entriesVal);

        if (!entriesVal.IsEmpty() && entriesVal->IsArray()) {
            auto entriesArray = entriesVal.As<v8::Array>();
            auto len = entriesArray->Length();
            for (int i = 0; i < len; i++) {
                v8::Local<v8::Value> entryVal;
                entriesArray->Get(context, i).ToLocal(&entryVal);
                if (!entryVal.IsEmpty() && entryVal->IsObject()) {
                    auto entryObj = entryVal.As<v8::Object>();
                    auto binding = entryObj->Get(context, ConvertToV8String(isolate,
                                                                            "binding")).ToLocalChecked()->Uint32Value(
                            context).FromJust();
                    v8::Local<v8::Value> resourceVal;
                    entryObj->Get(context, ConvertToV8String(isolate, "resource")).ToLocal(
                            &resourceVal);

                    auto resourceType = GetNativeType(resourceVal);
                    switch (resourceType) {
                        case NativeType::GPUSampler: {
                            auto sampler = GPUSamplerImpl::GetPointer(resourceVal.As<v8::Object>());
                            if (sampler != nullptr) {
                                auto resource = CanvasBindGroupEntryResource{
                                        CanvasBindGroupEntryResourceSampler,
                                };
                                resource.sampler = sampler->GetSampler();
                                CanvasBindGroupEntry entry{binding, resource};
                                entries.push_back(entry);
                            }

                        }
                            break;
                        case NativeType::GPUTextureView: {
                            auto textureView = GPUTextureViewImpl::GetPointer(
                                    resourceVal.As<v8::Object>());
                            if (textureView != nullptr) {
                                auto resource = CanvasBindGroupEntryResource{
                                        CanvasBindGroupEntryResourceTextureView,
                                };
                                resource.texture_view = textureView->GetTextureView();
                                CanvasBindGroupEntry entry{binding, resource};
                                entries.push_back(entry);
                            }
                        }
                            break;
                        default: {
                            if (!resourceVal.IsEmpty() && resourceVal->IsObject()) {
                                auto resourceObj = resourceVal.As<v8::Object>();
                                v8::Local<v8::Value> bufferVal;
                                resourceObj->Get(context,
                                                 ConvertToV8String(isolate, "buffer")).ToLocal(
                                        &bufferVal);
                                if (GetNativeType(bufferVal) == NativeType::GPUBuffer) {
                                    auto bufferObj = bufferVal.As<v8::Object>();
                                    auto buffer = GPUBufferImpl::GetPointer(bufferObj);
                                    if (buffer != nullptr) {
                                        auto resource = CanvasBindGroupEntryResource{
                                                CanvasBindGroupEntryResourceBuffer,
                                        };
                                        int64_t offset = -1;

                                        v8::Local<v8::Value> offsetVal;
                                        bufferObj->Get(context,
                                                       ConvertToV8String(isolate,
                                                                         "offset")).ToLocal(
                                                &offsetVal);
                                        if (!offsetVal.IsEmpty() && offsetVal->IsNumber()) {
                                            offset = (int64_t) offsetVal->NumberValue(
                                                    context).ToChecked();
                                        }

                                        int64_t size = -1;

                                        v8::Local<v8::Value> sizeVal;
                                        bufferObj->Get(context,
                                                       ConvertToV8String(isolate, "size")).ToLocal(
                                                &offsetVal);
                                        if (!sizeVal.IsEmpty() && sizeVal->IsNumber()) {
                                            size = (int64_t) sizeVal->NumberValue(
                                                    context).ToChecked();
                                        }

                                        resource.buffer = CanvasBufferBinding{
                                                buffer->GetGPUBuffer(), offset, size
                                        };

                                        CanvasBindGroupEntry entry{binding, resource};
                                        entries.push_back(entry);
                                    }
                                }
                            }

                        }
                            break;
                    }

                }
            }
        }


        auto bind_group = canvas_native_webgpu_device_create_bind_group(ptr->GetGPUDevice(), label,
                                                                        layout, entries.data(),
                                                                        entries.size());

        if (bind_group != nullptr) {
            auto ret = GPUBindGroupImpl::NewInstance(isolate, new GPUBindGroupImpl(bind_group));
            args.GetReturnValue().Set(ret);
            return;
        }
    }

    args.GetReturnValue().SetUndefined();
}

void GPUDeviceImpl::CreateBindGroupLayout(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    char *label = nullptr;

    auto optionsVal = args[0];

    if (optionsVal->IsObject()) {
        auto options = optionsVal.As<v8::Object>();
        v8::Local<v8::Value> labelVal;
        options->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);

        if (!labelVal.IsEmpty() && labelVal->IsString()) {
            label = *v8::String::Utf8Value(isolate, labelVal);
        }


        std::vector<CanvasBindGroupLayoutEntry> entries;

        v8::Local<v8::Value> entriesVal;
        options->Get(context, ConvertToV8String(isolate, "entries")).ToLocal(&entriesVal);

        if (!entriesVal.IsEmpty() && entriesVal->IsArray()) {
            auto entriesArray = entriesVal.As<v8::Array>();
            auto len = entriesArray->Length();
            for (int i = 0; i < len; i++) {
                v8::Local<v8::Value> entryVal;
                entriesArray->Get(context, i).ToLocal(&entryVal);
                if (!entryVal.IsEmpty() && entryVal->IsObject()) {
                    auto entryObj = entryVal.As<v8::Object>();
                    auto binding = entryObj->Get(context, ConvertToV8String(isolate,
                                                                            "binding")).ToLocalChecked()->Uint32Value(
                            context).FromJust();
                    v8::Local<v8::Value> resourceVal;
                    entryObj->Get(context, ConvertToV8String(isolate, "resource")).ToLocal(
                            &resourceVal);


                    auto visibility = entryObj->Get(context, ConvertToV8String(isolate,
                                                                               "visibility")).ToLocalChecked()->Uint32Value(
                            context).FromJust();


                    v8::Local<v8::Value> bufferVal;

                    entryObj->Get(context, ConvertToV8String(isolate, "buffer")).ToLocal(
                            &bufferVal);


                    if (!bufferVal.IsEmpty() && bufferVal->IsObject()) {
                        auto bufferObj = bufferVal.As<v8::Object>();
                        v8::Local<v8::Value> bufferType;
                        bufferObj->Get(context, ConvertToV8String(isolate, "type")).ToLocal(
                                &bufferType);

                        CanvasBufferBindingType type = CanvasBufferBindingTypeUniform;

                        if (!bufferType.IsEmpty() && bufferType->IsString()) {
                            auto typeStr = ConvertFromV8String(isolate, bufferType);
                            if (typeStr == "read-only-storage") {
                                type = CanvasBufferBindingTypeReadOnlyStorage;
                            } else if (typeStr == "storage") {
                                type = CanvasBufferBindingTypeStorage;
                            } else if (typeStr == "uniform") {
                                type = CanvasBufferBindingTypeUniform;
                            }
                        }

                        bool has_dynamic_offset = false;

                        v8::Local<v8::Value> hasDynamicOffsetVal;
                        bufferObj->Get(context,
                                       ConvertToV8String(isolate, "hasDynamicOffset ")).ToLocal(
                                &hasDynamicOffsetVal);


                        if (!hasDynamicOffsetVal.IsEmpty() && hasDynamicOffsetVal->IsBoolean()) {
                            has_dynamic_offset = hasDynamicOffsetVal->BooleanValue(isolate);
                        }

                        int64_t min_binding_size = -1;

                        v8::Local<v8::Value> minBindingSizeVal;
                        bufferObj->Get(context,
                                       ConvertToV8String(isolate, "minBindingSize ")).ToLocal(
                                &minBindingSizeVal);


                        if (!minBindingSizeVal.IsEmpty() && minBindingSizeVal->IsNumber()) {
                            min_binding_size = (int64_t) minBindingSizeVal->NumberValue(
                                    context).ToChecked();
                        }

                        CanvasBindingType buffer{
                                CanvasBindingTypeBuffer
                        };

                        buffer.buffer = CanvasBufferBindingLayout{
                                type, has_dynamic_offset, min_binding_size
                        };


                        CanvasBindGroupLayoutEntry entry{
                                binding,
                                visibility,
                                buffer
                        };

                        entries.push_back(entry);

                        continue;
                    }


                    v8::Local<v8::Value> externalTextureVal;

                    entryObj->Get(context, ConvertToV8String(isolate, "externalTexture")).ToLocal(
                            &externalTextureVal);

                    if (!externalTextureVal.IsEmpty() && externalTextureVal->IsObject()) {
                        // todo
//                        CanvasBindingType buffer{
//                                CanvasBindingTypeTexture
//                        };
//
//                        buffer.buffer = CanvasBufferBindingLayout{
//                                type, has_dynamic_offset, min_binding_size
//                        };
//
//
//                        CanvasBindGroupLayoutEntry entry{
//                                binding,
//                                visibility,
//                                buffer
//                        };

                        continue;
                    }


                    v8::Local<v8::Value> samplerVal;

                    entryObj->Get(context, ConvertToV8String(isolate, "sampler")).ToLocal(
                            &samplerVal);

                    if (!samplerVal.IsEmpty() && samplerVal->IsObject()) {
                        auto samplerObj = samplerVal.As<v8::Object>();
                        v8::Local<v8::Value> samplerType;
                        samplerObj->Get(context, ConvertToV8String(isolate, "type")).ToLocal(
                                &samplerType);

                        CanvasSamplerBindingType type = CanvasSamplerBindingTypeFiltering;

                        if (!samplerType.IsEmpty() && samplerType->IsString()) {
                            auto typeStr = ConvertFromV8String(isolate, samplerType);
                            if (typeStr == "comparison") {
                                type = CanvasSamplerBindingTypeComparison;
                            } else if (typeStr == "non-filtering") {
                                type = CanvasSamplerBindingTypeNonFiltering;
                            } else if (typeStr == "filtering") {
                                type = CanvasSamplerBindingTypeFiltering;
                            }
                        }


                        CanvasBindingType sampler{
                                CanvasBindingTypeSampler
                        };

                        sampler.sampler = CanvasSamplerBindingLayout{
                                type
                        };


                        CanvasBindGroupLayoutEntry entry{
                                binding,
                                visibility,
                                sampler
                        };

                        entries.push_back(entry);

                        continue;

                    }


                    v8::Local<v8::Value> storageTextureVal;

                    entryObj->Get(context, ConvertToV8String(isolate, "storageTexture")).ToLocal(
                            &storageTextureVal);

                    if (!storageTextureVal.IsEmpty() && storageTextureVal->IsObject()) {
                        auto storageTextureObj = storageTextureVal.As<v8::Object>();

                        CanvasBindingType storage{
                                CanvasBindingTypeStorageTexture
                        };

                        CanvasStorageTextureAccess access = CanvasStorageTextureAccessWriteOnly;

                        v8::Local<v8::Value> accessVal;

                        storageTextureObj->Get(context,
                                               ConvertToV8String(isolate, "access")).ToLocal(
                                &accessVal);


                        if (!accessVal.IsEmpty() && accessVal->IsString()) {
                            auto accessStr = ConvertFromV8String(isolate, accessVal);

                            if (accessStr == "write-only") {
                                access = CanvasStorageTextureAccessWriteOnly;
                            } else if (accessStr == "read-only") {
                                access = CanvasStorageTextureAccessReadOnly;
                            } else if (accessStr == "read-write") {
                                access = CanvasStorageTextureAccessReadWrite;
                            }
                        }


                        CanvasTextureViewDimension view_dimension = CanvasTextureViewDimensionD2;


                        v8::Local<v8::Value> viewDimensionVal;

                        storageTextureObj->Get(context,
                                               ConvertToV8String(isolate, "viewDimension")).ToLocal(
                                &viewDimensionVal);


                        if (!viewDimensionVal.IsEmpty() && viewDimensionVal->IsString()) {
                            auto viewDimensionStr = ConvertFromV8String(isolate, viewDimensionVal);
                            if (viewDimensionStr == "1d") {
                                view_dimension = CanvasTextureViewDimensionD1;
                            } else if (viewDimensionStr == "2d") {
                                view_dimension = CanvasTextureViewDimensionD2;
                            } else if (viewDimensionStr == "2d-array") {
                                view_dimension = CanvasTextureViewDimensionD2Array;
                            } else if (viewDimensionStr == "cube") {
                                view_dimension = CanvasTextureViewDimensionCube;
                            } else if (viewDimensionStr == "cube-array") {
                                view_dimension = CanvasTextureViewDimensionCubeArray;
                            } else if (viewDimensionStr == "3d") {
                                view_dimension = CanvasTextureViewDimensionD3;
                            }
                        }


                        v8::Local<v8::Value> formatVal;

                        storageTextureObj->Get(context,
                                               ConvertToV8String(isolate, "format")).ToLocal(
                                &formatVal);


                        if (!formatVal.IsEmpty() && formatVal->IsString()) {
                            auto formatStr = ConvertFromV8String(isolate, formatVal);
                            auto textureFormat = canvas_native_webgpu_enum_string_to_gpu_texture(
                                    formatStr.c_str());
                            if (textureFormat.tag == CanvasOptionalGPUTextureFormatSome) {
                                storage.storage_texture = CanvasStorageTextureBindingLayout{
                                        access,
                                        textureFormat.some,
                                        view_dimension
                                };

                                CanvasBindGroupLayoutEntry entry{
                                        binding,
                                        visibility,
                                        storage
                                };

                                entries.push_back(entry);

                                continue;

                            } else {
                                // todo throw ??
                                continue;
                            }
                        }


                    }


                    v8::Local<v8::Value> textureVal;

                    entryObj->Get(context, ConvertToV8String(isolate, "texture")).ToLocal(
                            &textureVal);

                    if (!textureVal.IsEmpty() && textureVal->IsObject()) {
                        auto textureObj = textureVal.As<v8::Object>();
                        bool multisampled = false;

                        v8::Local<v8::Value> multisampledVal;

                        textureObj->Get(context,
                                        ConvertToV8String(isolate, "multisampled")).ToLocal(
                                &multisampledVal);

                        if (!multisampledVal.IsEmpty() && multisampledVal->IsBoolean()) {
                            multisampled = multisampledVal->BooleanValue(isolate);
                        }


                        v8::Local<v8::Value> sampleTypeVal;
                        textureObj->Get(context, ConvertToV8String(isolate, "sampleType")).ToLocal(
                                &sampleTypeVal);

                        CanvasTextureSampleType type = CanvasTextureSampleTypeFloat;

                        if (!sampleTypeVal.IsEmpty() && sampleTypeVal->IsString()) {
                            auto typeStr = ConvertFromV8String(isolate, sampleTypeVal);
                            if (typeStr == "depth") {
                                type = CanvasTextureSampleTypeDepth;
                            } else if (typeStr == "float") {
                                type = CanvasTextureSampleTypeFloat;
                            } else if (typeStr == "sint") {
                                type = CanvasTextureSampleTypeSint;
                            } else if (typeStr == "uint") {
                                type = CanvasTextureSampleTypeUint;
                            } else if (typeStr == "unfilterable-float") {
                                type = CanvasTextureSampleTypeUnfilterableFloat;
                            }
                        }


                        CanvasTextureViewDimension view_dimension = CanvasTextureViewDimensionD2;


                        v8::Local<v8::Value> viewDimensionVal;

                        textureObj->Get(context,
                                        ConvertToV8String(isolate, "viewDimension")).ToLocal(
                                &viewDimensionVal);


                        if (!viewDimensionVal.IsEmpty() && viewDimensionVal->IsString()) {
                            auto viewDimensionStr = ConvertFromV8String(isolate, viewDimensionVal);
                            if (viewDimensionStr == "1d") {
                                view_dimension = CanvasTextureViewDimensionD1;
                            } else if (viewDimensionStr == "2d") {
                                view_dimension = CanvasTextureViewDimensionD2;
                            } else if (viewDimensionStr == "2d-array") {
                                view_dimension = CanvasTextureViewDimensionD2Array;
                            } else if (viewDimensionStr == "cube") {
                                view_dimension = CanvasTextureViewDimensionCube;
                            } else if (viewDimensionStr == "cube-array") {
                                view_dimension = CanvasTextureViewDimensionCubeArray;
                            } else if (viewDimensionStr == "3d") {
                                view_dimension = CanvasTextureViewDimensionD3;
                            }
                        }


                        CanvasBindingType texture{
                                CanvasBindingTypeTexture
                        };

                        texture.texture = CanvasTextureBindingLayout{
                                type,
                                view_dimension,
                                multisampled
                        };


                        CanvasBindGroupLayoutEntry entry{
                                binding,
                                visibility,
                                texture
                        };

                        entries.push_back(entry);

                        continue;

                    }

                }
            }
        }


        auto bind_group = canvas_native_webgpu_device_create_bind_group_layout(ptr->GetGPUDevice(),
                                                                               label,
                                                                               entries.data(),
                                                                               entries.size());

        if (bind_group != nullptr) {
            auto ret = GPUBindGroupLayoutImpl::NewInstance(isolate,
                                                           new GPUBindGroupLayoutImpl(bind_group));
            args.GetReturnValue().Set(ret);
            return;
        }
    }

    args.GetReturnValue().SetUndefined();
}

void GPUDeviceImpl::CreateBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    char *label = nullptr;
    bool mappedAtCreation = false;
    uint64_t size = 0;
    uint32_t usage = 0;

    auto optionsVal = args[0];

    if (optionsVal->IsObject()) {
        auto options = optionsVal.As<v8::Object>();
        v8::Local<v8::Value> labelVal;
        options->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);

        if (!labelVal.IsEmpty() && labelVal->IsString()) {
            label = *v8::String::Utf8Value(isolate, labelVal);
        }

        v8::Local<v8::Value> mappedAtCreationVal;

        options->Get(context, ConvertToV8String(isolate, "mappedAtCreation")).ToLocal(
                &mappedAtCreationVal);

        if (!mappedAtCreationVal.IsEmpty()) {
            mappedAtCreation = mappedAtCreationVal->BooleanValue(isolate);
        }

        v8::Local<v8::Value> sizeVal;
        options->Get(context, ConvertToV8String(isolate, "size")).ToLocal(&sizeVal);

        if (sizeVal->IsNumber()) {
            size = (uint64_t) sizeVal->NumberValue(context).ToChecked();
        }


        v8::Local<v8::Value> usageVal;
        options->Get(context, ConvertToV8String(isolate, "usage")).ToLocal(&usageVal);

        if (usageVal->IsNumber()) {
            usage = usageVal->Uint32Value(context).ToChecked();
        }
    }

    auto buffer = canvas_native_webgpu_device_create_buffer(ptr->GetGPUDevice(), label, size, usage,
                                                            mappedAtCreation);

    if (buffer != nullptr) {
        auto bufImpl = new GPUBufferImpl(buffer);
        auto ret = GPUBufferImpl::NewInstance(isolate, bufImpl);
        args.GetReturnValue().Set(ret);

    } else {
        // todo return invalid buffer
        args.GetReturnValue().SetUndefined();
    }
}

void GPUDeviceImpl::CreateCommandEncoder(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();

    char *label = nullptr;

    auto labelVal = args[0];

    if (!labelVal.IsEmpty() && labelVal->IsString()) {
        label = *v8::String::Utf8Value(isolate, labelVal);
    }

    auto encoder = canvas_native_webgpu_device_create_command_encoder(ptr->GetGPUDevice(), label);

    if (encoder != nullptr) {
        auto instance = new GPUCommandEncoderImpl(encoder);
        auto ret = GPUCommandEncoderImpl::NewInstance(isolate, instance);
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetUndefined();

}

void GPUDeviceImpl::CreateComputePipeline(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto optionsVal = args[0];

    if (!optionsVal->IsObject()) {
        // should error at this point
        return;
    }
    auto options = optionsVal.As<v8::Object>();

    v8::Local<v8::Value> labelVal;
    options->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);

    char *label = nullptr;

    if (!labelVal.IsEmpty() && labelVal->IsString()) {
        label = *v8::String::Utf8Value(isolate, labelVal);
    }

    CanvasGPUPipelineLayoutOrGPUAutoLayoutMode layout{
            CanvasGPUPipelineLayoutOrGPUAutoLayoutModeAuto
    };

    v8::Local<v8::Value> layoutVal;
    options->Get(context, ConvertToV8String(isolate, "layout")).ToLocal(&layoutVal);

    if (!layoutVal.IsEmpty() && layoutVal->IsObject()) {
        auto layoutImpl = GPUPipelineLayoutImpl::GetPointer(layoutVal.As<v8::Object>());
        if (layoutImpl != nullptr) {
            layout.tag = CanvasGPUPipelineLayoutOrGPUAutoLayoutModeLayout;
            layout.layout = layoutImpl->GetPipeline();
        }
    }


    v8::Local<v8::Value> computeVal;
    options->Get(context, ConvertToV8String(isolate, "compute")).ToLocal(&computeVal);

    if (!computeVal.IsEmpty() && computeVal->IsObject()) {

        auto computeObj = computeVal.As<v8::Object>();

        v8::Local<v8::Value> constantsVal;
        computeObj->Get(context, ConvertToV8String(isolate, "constants")).ToLocal(
                &constantsVal);

        CanvasConstants *store = nullptr;

        if (!constantsVal.IsEmpty() && constantsVal->IsMap()) {
            auto constants = constantsVal.As<v8::Map>();
            auto keyValues = constants->AsArray();
            auto length = keyValues->Length();
            if (length > 0) {
                store = canvas_native_webgpu_constants_create();
                for (int i = 0; i < length; i += 2) {
                    auto k = i;
                    auto v = k + 1;

                    v8::Local<v8::Value> keyVal;
                    keyValues->Get(context, k).ToLocal(&keyVal);
                    v8::Local<v8::Value> valueVal;
                    keyValues->Get(context, v).ToLocal(&valueVal);


                    if (!keyVal.IsEmpty() && keyVal->IsString() && !valueVal.IsEmpty() &&
                        valueVal->IsNumber()) {
                        canvas_native_webgpu_constants_insert(
                                store,
                                *v8::String::Utf8Value(isolate, keyVal),
                                valueVal.As<v8::Number>()->Value()
                        );
                    }

                }
            }
        }


        v8::Local<v8::Value> entryPoint;
        computeObj->Get(context, ConvertToV8String(isolate, "entryPoint")).ToLocal(
                &entryPoint);

        char *entry_point = nullptr;

        if (!entryPoint.IsEmpty() && entryPoint->IsString()) {
            auto ep = v8::String::Utf8Value(isolate, entryPoint);
            entry_point = (char *) malloc(ep.length());
            std::strcpy(entry_point, *ep);
        }

        v8::Local<v8::Value> moduleVal;
        computeObj->Get(context, ConvertToV8String(isolate, "module")).ToLocal(&moduleVal);

        auto module = GPUShaderModuleImpl::GetPointer(moduleVal.As<v8::Object>());

        CanvasProgrammableStage stage{
                module->GetShaderModule(),
                entry_point,
                store
        };

        auto pipeline = canvas_native_webgpu_device_create_compute_pipeline(ptr->GetGPUDevice(),
                                                                            label, layout, &stage);

        if (entry_point != nullptr) {
            free(entry_point);
        }

        if (store != nullptr) {
            canvas_native_webgpu_constants_destroy(store);
        }

        if (pipeline != nullptr) {
            auto ret = GPUComputePipelineImpl::NewInstance(isolate,
                                                           new GPUComputePipelineImpl(pipeline));
            args.GetReturnValue().Set(ret);
            return;
        }


    }

    args.GetReturnValue().SetUndefined();
}

void GPUDeviceImpl::CreatePipelineLayout(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto optionsVal = args[0];

    if (!optionsVal->IsObject()) {
        // should error at this point
        return;
    }
    auto options = optionsVal.As<v8::Object>();

    v8::Local<v8::Value> labelVal;
    options->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);

    char *label = nullptr;

    if (!labelVal.IsEmpty() && labelVal->IsString()) {
        label = *v8::String::Utf8Value(isolate, labelVal);
    }


    std::vector<const CanvasGPUBindGroupLayout *> group_layouts;

    v8::Local<v8::Value> groupLayoutsVal;
    options->Get(context, ConvertToV8String(isolate, "bindGroupLayouts")).ToLocal(&groupLayoutsVal);

    if (!groupLayoutsVal.IsEmpty() && groupLayoutsVal->IsArray()) {
        auto groupLayoutsArray = groupLayoutsVal.As<v8::Array>();
        auto len = groupLayoutsArray->Length();
        for (int i = 0; i < len; i++) {
            v8::Local<v8::Value> groupVal;
            groupLayoutsArray->Get(context, i).ToLocal(&groupVal);
            if (GetNativeType(groupVal) == NativeType::GPUBindGroupLayout) {
                auto layout = GPUBindGroupLayoutImpl::GetPointer(groupVal.As<v8::Object>());
                if (layout != nullptr) {
                    group_layouts.push_back(layout->GetBindGroupLayout());
                }
            }
        }

        auto layout = canvas_native_webgpu_device_create_pipeline_layout(ptr->GetGPUDevice(), label,
                                                                         group_layouts.data(),
                                                                         group_layouts.size());

        if (layout != nullptr) {
            auto ret = GPUPipelineLayoutImpl::NewInstance(isolate,
                                                          new GPUPipelineLayoutImpl(layout));
            args.GetReturnValue().Set(ret);
            return;
        }
    }


    args.GetReturnValue().SetUndefined();
}

void GPUDeviceImpl::CreateQuerySet(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto optionsVal = args[0];

    if (!optionsVal->IsObject()) {
        // should error at this point
        return;
    }
    auto options = optionsVal.As<v8::Object>();

    v8::Local<v8::Value> labelVal;
    options->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);

    char *label = nullptr;

    if (!labelVal.IsEmpty() && labelVal->IsString()) {
        label = *v8::String::Utf8Value(isolate, labelVal);
    }


    v8::Local<v8::Value> typeVal;
    options->Get(context, ConvertToV8String(isolate, "type")).ToLocal(&labelVal);


    v8::Local<v8::Value> countVal;
    options->Get(context, ConvertToV8String(isolate, "count")).ToLocal(&labelVal);

    auto typeStr = ConvertFromV8String(isolate, typeVal);

    const CanvasGPUQuerySet *query_set = nullptr;
    if (typeStr == "occlusion") {
        query_set = canvas_native_webgpu_device_create_query_set(ptr->GetGPUDevice(), label,
                                                                 CanvasQueryTypeOcclusion,
                                                                 countVal->Uint32Value(
                                                                         context).FromJust());
    } else if (typeStr == "timestamp") {
        query_set = canvas_native_webgpu_device_create_query_set(ptr->GetGPUDevice(), label,
                                                                 CanvasQueryTypeTimestamp,
                                                                 countVal->Uint32Value(
                                                                         context).FromJust());
    } else {
        // todo throw
    }


    if (query_set != nullptr) {
        auto ret = GPUQuerySetImpl::NewInstance(isolate, new GPUQuerySetImpl(query_set));
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetUndefined();

}

void GPUDeviceImpl::CreateRenderBundleEncoder(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto optionsVal = args[0];

    if (!optionsVal->IsObject()) {
        // should error at this point
        return;
    }
    auto options = optionsVal.As<v8::Object>();

    v8::Local<v8::Value> labelVal;
    options->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);

    char *label = nullptr;

    if (!labelVal.IsEmpty() && labelVal->IsString()) {
        label = *v8::String::Utf8Value(isolate, labelVal);
    }


    std::vector<CanvasGPUTextureFormat> colorFormats;

    v8::Local<v8::Value> colorFormatsVal;
    options->Get(context, ConvertToV8String(isolate, "colorFormats")).ToLocal(&colorFormatsVal);

    if (!colorFormatsVal.IsEmpty() && colorFormatsVal->IsArray()) {
        auto colorFormatsArray = colorFormatsVal.As<v8::Array>();
        auto len = colorFormatsArray->Length();
        for (int i = 0; i < len; i++) {
            v8::Local<v8::Value> formatVal;
            colorFormatsArray->Get(context, i).ToLocal(&formatVal);
            if (!formatVal.IsEmpty() && formatVal->IsString()) {
                auto formatStr = ConvertFromV8String(isolate, formatVal);
                auto format = canvas_native_webgpu_enum_string_to_gpu_texture(formatStr.c_str());
                if (format.tag == CanvasOptionalGPUTextureFormatSome) {
                    colorFormats.push_back(format.some);
                }
            }

        }
    }


    auto depthStencilFormat = CanvasOptionalGPUTextureFormat{
            CanvasOptionalGPUTextureFormatNone
    };

    v8::Local<v8::Value> depthStencilFormatVal;
    options->Get(context, ConvertToV8String(isolate, "depthStencilFormat")).ToLocal(
            &depthStencilFormatVal);

    if (!depthStencilFormatVal.IsEmpty() && depthStencilFormatVal->IsString()) {
        auto depthStencilFormatStr = ConvertFromV8String(isolate, depthStencilFormatVal);
        depthStencilFormat = canvas_native_webgpu_enum_string_to_gpu_texture(
                depthStencilFormatStr.c_str());
    }

    uint32_t sampleCount = 1;

    bool depthReadOnly = false;

    bool stencilReadOnly = false;

    CanvasCreateRenderBundleEncoderDescriptor descriptor{
            label,
            colorFormats.data(),
            colorFormats.size(),
            depthStencilFormat,
            sampleCount,
            depthReadOnly,
            stencilReadOnly
    };


    auto encoder = canvas_native_webgpu_device_create_render_bundle_encoder(ptr->GetGPUDevice(),
                                                                            &descriptor);

    if (encoder != nullptr) {
        auto ret = GPURenderBundleEncoderImpl::NewInstance(isolate,
                                                           new GPURenderBundleEncoderImpl(encoder));
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetUndefined();
}

void GPUDeviceImpl::CreateRenderPipeline(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    CanvasCreateRenderPipelineDescriptor descriptor{};
    descriptor.label = nullptr;

    auto optionsVal = args[0];

    if (!optionsVal->IsObject()) {
        // should error at this point
        return;
    }
    auto options = optionsVal.As<v8::Object>();


    v8::Local<v8::Value> stencilValue;
    options->Get(context, ConvertToV8String(isolate, "depthStencil")).ToLocal(
            &stencilValue);

    CanvasDepthStencilState *stencil = nullptr;

    if (!stencilValue.IsEmpty() && stencilValue->IsObject()) {
        auto stencilObj = stencilValue.As<v8::Object>();
        stencil = new CanvasDepthStencilState{};
        stencil->depth_bias = 0;
        stencil->depth_bias_clamp = 0;
        stencil->depth_bias_slope_scale = 0;
        stencil->stencil_read_mask = 0xFFFFFFFF;
        stencil->stencil_write_mask = 0xFFFFFFFF;
        stencil->stencil_front = CanvasStencilFaceState{
                CanvasCompareFunctionAlways,
                CanvasStencilOperationKeep,
                CanvasStencilOperationKeep,
                CanvasStencilOperationKeep
        };

        stencil->stencil_back = CanvasStencilFaceState{
                CanvasCompareFunctionAlways,
                CanvasStencilOperationKeep,
                CanvasStencilOperationKeep,
                CanvasStencilOperationKeep
        };
        // todo throw if failed
        v8::Local<v8::Value> formatValue;

        stencilObj->Get(context, ConvertToV8String(isolate, "format")).ToLocal(&formatValue);
        if (!formatValue.IsEmpty() && formatValue->IsString()) {
            auto val = *v8::String::Utf8Value(isolate, formatValue);
            auto format = canvas_native_webgpu_enum_string_to_gpu_texture(
                    val);
            if (format.tag ==
                CanvasOptionalGPUTextureFormat_Tag::CanvasOptionalGPUTextureFormatSome) {
                stencil->format = format.some;
            }
        } else {
            // todo throw
        }

        v8::Local<v8::Value> depthBiasVal;
        stencilObj->Get(context, ConvertToV8String(isolate, "depthBias")).ToLocal(&depthBiasVal);

        if (!depthBiasVal.IsEmpty() && depthBiasVal->IsInt32()) {
            stencil->depth_bias = depthBiasVal->Int32Value(context).FromJust();
        }

        v8::Local<v8::Value> depthBiasClampVal;
        stencilObj->Get(context, ConvertToV8String(isolate, "depthBiasClamp")).ToLocal(
                &depthBiasClampVal);

        if (!depthBiasClampVal.IsEmpty() && depthBiasClampVal->IsInt32()) {
            stencil->depth_bias_clamp = depthBiasClampVal->Int32Value(context).FromJust();
        }


        v8::Local<v8::Value> depthBiasSlopeScaleVal;
        stencilObj->Get(context, ConvertToV8String(isolate, "depthBiasSlopeScale")).ToLocal(
                &depthBiasSlopeScaleVal);

        if (!depthBiasSlopeScaleVal.IsEmpty() && depthBiasSlopeScaleVal->IsInt32()) {
            stencil->depth_bias_slope_scale = depthBiasSlopeScaleVal->Int32Value(
                    context).FromJust();
        }

        v8::Local<v8::Value> depthCompareVal;
        stencilObj->Get(context, ConvertToV8String(isolate, "depthCompare")).ToLocal(
                &depthCompareVal);

        auto depthCompareStr = ConvertFromV8String(isolate, depthCompareVal);

        if (depthCompareStr == "never") {
            stencil->depth_compare = CanvasCompareFunctionNever;
        } else if (depthCompareStr == "less") {
            stencil->depth_compare = CanvasCompareFunctionLess;
        } else if (depthCompareStr == "equal") {
            stencil->depth_compare = CanvasCompareFunctionEqual;
        } else if (depthCompareStr == "less-equal") {
            stencil->depth_compare = CanvasCompareFunctionLessEqual;
        } else if (depthCompareStr == "greater") {
            stencil->depth_compare = CanvasCompareFunctionGreater;
        } else if (depthCompareStr == "not-equal") {
            stencil->depth_compare = CanvasCompareFunctionNotEqual;
        } else if (depthCompareStr == "greater-equal") {
            stencil->depth_compare = CanvasCompareFunctionGreaterEqual;
        } else if (depthCompareStr == "always") {
            stencil->depth_compare = CanvasCompareFunctionAlways;
        }

        stencil->depth_write_enabled = stencilObj->Get(context, ConvertToV8String(isolate,
                                                                                  "depthWriteEnabled")).ToLocalChecked()->BooleanValue(
                isolate);


        v8::Local<v8::Value> stencilBackVal;
        stencilObj->Get(context, ConvertToV8String(isolate, "stencilBack")).ToLocal(
                &stencilBackVal);

        if (!stencilBackVal.IsEmpty() && stencilBackVal->IsObject()) {
            auto stencilBackObj = stencilBackVal.As<v8::Object>();

            v8::Local<v8::Value> compareVal;
            stencilBackObj->Get(context, ConvertToV8String(isolate, "compare")).ToLocal(
                    &compareVal);

            stencil->stencil_back.compare = ParseCompareFunction(isolate, compareVal,
                                                                 stencil->stencil_back.compare);

            v8::Local<v8::Value> depthFailOpVal;
            stencilBackObj->Get(context, ConvertToV8String(isolate, "depthFailOp")).ToLocal(
                    &depthFailOpVal);

            stencil->stencil_back.depth_fail_op = ParseStencilOperation(isolate, depthFailOpVal,
                                                                        stencil->stencil_back.depth_fail_op);


            v8::Local<v8::Value> failOpVal;
            stencilBackObj->Get(context, ConvertToV8String(isolate, "failOp")).ToLocal(
                    &failOpVal);

            stencil->stencil_back.fail_op = ParseStencilOperation(isolate, failOpVal,
                                                                  stencil->stencil_back.fail_op);

            v8::Local<v8::Value> passOpVal;
            stencilBackObj->Get(context, ConvertToV8String(isolate, "passOp")).ToLocal(
                    &passOpVal);

            stencil->stencil_back.pass_op = ParseStencilOperation(isolate, passOpVal,
                                                                  stencil->stencil_back.pass_op);

        }


        v8::Local<v8::Value> stencilFrontVal;
        stencilObj->Get(context, ConvertToV8String(isolate, "stencilFront")).ToLocal(
                &stencilFrontVal);

        if (!stencilFrontVal.IsEmpty() && stencilFrontVal->IsObject()) {
            auto stencilFrontObj = stencilFrontVal.As<v8::Object>();

            v8::Local<v8::Value> compareVal;
            stencilFrontObj->Get(context, ConvertToV8String(isolate, "compare")).ToLocal(
                    &compareVal);

            stencil->stencil_front.compare = ParseCompareFunction(isolate, compareVal,
                                                                  stencil->stencil_front.compare);

            v8::Local<v8::Value> depthFailOpVal;
            stencilFrontObj->Get(context, ConvertToV8String(isolate, "depthFailOp")).ToLocal(
                    &depthFailOpVal);

            stencil->stencil_front.depth_fail_op = ParseStencilOperation(isolate, depthFailOpVal,
                                                                         stencil->stencil_front.depth_fail_op);


            v8::Local<v8::Value> failOpVal;
            stencilFrontObj->Get(context, ConvertToV8String(isolate, "failOp")).ToLocal(
                    &failOpVal);

            stencil->stencil_front.fail_op = ParseStencilOperation(isolate, failOpVal,
                                                                   stencil->stencil_front.fail_op);

            v8::Local<v8::Value> passOpVal;
            stencilFrontObj->Get(context, ConvertToV8String(isolate, "passOp")).ToLocal(
                    &passOpVal);

            stencil->stencil_front.pass_op = ParseStencilOperation(isolate, passOpVal,
                                                                   stencil->stencil_front.pass_op);

        }

        v8::Local<v8::Value> stencilReadMaskVal;
        stencilObj->Get(context, ConvertToV8String(isolate, "stencilReadMask")).ToLocal(
                &stencilReadMaskVal);

        if (!stencilReadMaskVal.IsEmpty() && stencilReadMaskVal->IsUint32()) {
            stencil->stencil_read_mask = stencilReadMaskVal->Uint32Value(context).FromJust();
        }


        v8::Local<v8::Value> stencilWriteMaskVal;
        stencilObj->Get(context, ConvertToV8String(isolate, "stencilWriteMask")).ToLocal(
                &stencilWriteMaskVal);

        if (!stencilWriteMaskVal.IsEmpty() && stencilWriteMaskVal->IsUint32()) {
            stencil->stencil_write_mask = stencilWriteMaskVal->Uint32Value(context).FromJust();
        }

    }


    v8::Local<v8::Value> fragmentValue;
    options->Get(context, ConvertToV8String(isolate, "fragment")).ToLocal(
            &fragmentValue);

    CanvasFragmentState *fragment = nullptr;

    std::vector<CanvasColorTargetState> targets;

    if (!fragmentValue.IsEmpty() && fragmentValue->IsObject()) {
        auto fragmentValueObj = fragmentValue.As<v8::Object>();
        fragment = new CanvasFragmentState{};

        v8::Local<v8::Value> targetsVal;
        fragmentValueObj->Get(context, ConvertToV8String(isolate, "targets")).ToLocal(&targetsVal);


        auto targetsArray = targetsVal.As<v8::Array>();
        auto len = targetsArray->Length();

        for (int i = 0; i < len; i++) {
            auto state = targetsArray->Get(context, i).ToLocalChecked().As<v8::Object>();

            auto formatVal = state->Get(context,
                                        ConvertToV8String(isolate, "format")).ToLocalChecked();
            auto formatStr = ConvertFromV8String(isolate, formatVal);
            auto formatResult = canvas_native_webgpu_enum_string_to_gpu_texture(formatStr.c_str());


            if (formatResult.tag == CanvasOptionalGPUTextureFormatNone) {
                // todo throw
                args.GetReturnValue().SetUndefined();
                return;
            } else {}

            auto format = CanvasGPUTextureFormat{
                    formatResult.some.tag
            };

            uint32_t writeMask = 0xF;

            v8::Local<v8::Value> writeMaskVal;

            state->Get(context, ConvertToV8String(isolate, "writeMask")).ToLocal(&writeMaskVal);

            if (!writeMaskVal.IsEmpty() && writeMaskVal->IsUint32()) {
                writeMask = writeMaskVal->Uint32Value(context).FromJust();
            }

            CanvasOptionalBlendState blend{
                    CanvasOptionalBlendStateNone
            };

            v8::Local<v8::Value> blendVal;

            state->Get(context, ConvertToV8String(isolate, "blend")).ToLocal(&blendVal);

            if (!blendVal.IsEmpty() && blendVal->IsObject()) {
                auto blendObj = blendVal.As<v8::Object>();
                auto alpha = blendObj->Get(context, ConvertToV8String(isolate,
                                                                      "alpha")).ToLocalChecked().As<v8::Object>();

                v8::Local<v8::Value> alphaSrcFactorVal;

                alpha->Get(context,
                           ConvertToV8String(isolate,
                                             "srcFactor")).ToLocal(&alphaSrcFactorVal);

                auto alphaSrcFactor = ParseBlendFactor(isolate, alphaSrcFactorVal,
                                                       CanvasBlendFactorZero);

                v8::Local<v8::Value> alphaDstFactorVal;
                alpha->Get(context,
                           ConvertToV8String(isolate,
                                             "dstFactor")).ToLocal(&alphaDstFactorVal);

                auto alphaDstFactor = ParseBlendFactor(isolate, alphaDstFactorVal,
                                                       CanvasBlendFactorZero);

                v8::Local<v8::Value> alphaOperationVal;

                alpha->Get(context,
                           ConvertToV8String(isolate,
                                             "operation")).ToLocal(&alphaOperationVal);

                auto alphaOperation = ParseBlendOperation(isolate, alphaOperationVal,
                                                          CanvasBlendOperationAdd);

                auto alpha_val = CanvasBlendComponent{alphaSrcFactor, alphaDstFactor,
                                                      alphaOperation};

                auto color = blendObj->Get(context, ConvertToV8String(isolate,
                                                                      "color")).ToLocalChecked().As<v8::Object>();


                v8::Local<v8::Value> colorSrcFactorVal;

                color->Get(context,
                           ConvertToV8String(isolate,
                                             "srcFactor")).ToLocal(&colorSrcFactorVal);

                auto colorSrcFactor = ParseBlendFactor(isolate, colorSrcFactorVal,
                                                       CanvasBlendFactorZero);

                v8::Local<v8::Value> colorDstFactorVal;
                color->Get(context,
                           ConvertToV8String(isolate,
                                             "dstFactor")).ToLocal(&colorDstFactorVal);

                auto colorDstFactor = ParseBlendFactor(isolate, colorDstFactorVal,
                                                       CanvasBlendFactorZero);

                v8::Local<v8::Value> colorOperationVal;

                color->Get(context,
                           ConvertToV8String(isolate,
                                             "operation")).ToLocal(&colorOperationVal);

                auto colorOperation = ParseBlendOperation(isolate, colorOperationVal,
                                                          CanvasBlendOperationAdd);


                auto color_val = CanvasBlendComponent{colorSrcFactor, colorDstFactor,
                                                      colorOperation};


                blend = CanvasOptionalBlendState{
                        CanvasOptionalBlendStateSome,
                        CanvasBlendState{
                                color_val, alpha_val
                        }
                };
            }

            auto targetState = CanvasColorTargetState{
                    format,
                    blend,
                    writeMask
            };

            targets.push_back(targetState);
        }

        if (!targets.empty()) {
            fragment->targets = targets.data();
            fragment->targets_size = targets.size();
        }

        v8::Local<v8::Value> constantsVal;
        fragmentValueObj->Get(context, ConvertToV8String(isolate, "constants")).ToLocal(
                &constantsVal);

        if (!constantsVal.IsEmpty() && constantsVal->IsMap()) {
            auto constants = constantsVal.As<v8::Map>();
            auto keyValues = constants->AsArray();
            auto length = keyValues->Length();
            CanvasConstants *store = nullptr;

            if (length > 0) {
                store = canvas_native_webgpu_constants_create();
                for (int i = 0; i < length; i += 2) {
                    auto k = i;
                    auto v = k + 1;

                    v8::Local<v8::Value> keyVal;
                    keyValues->Get(context, k).ToLocal(&keyVal);
                    v8::Local<v8::Value> valueVal;
                    keyValues->Get(context, v).ToLocal(&valueVal);


                    if (!keyVal.IsEmpty() && keyVal->IsString() && !valueVal.IsEmpty() &&
                        valueVal->IsNumber()) {
                        canvas_native_webgpu_constants_insert(
                                store,
                                *v8::String::Utf8Value(isolate, keyVal),
                                valueVal.As<v8::Number>()->Value()
                        );
                    }

                }
            }
            fragment->constants = store;
        }


        v8::Local<v8::Value> entryPoint;
        fragmentValueObj->Get(context, ConvertToV8String(isolate, "entryPoint")).ToLocal(
                &entryPoint);


        if (!entryPoint.IsEmpty() && entryPoint->IsString()) {
            auto ep = v8::String::Utf8Value(isolate, entryPoint);
            char *entry_point = (char *) malloc(ep.length());
            std::strcpy(entry_point, *ep);

            fragment->entry_point = entry_point;
        }


        v8::Local<v8::Value> moduleVal;
        fragmentValueObj->Get(context, ConvertToV8String(isolate, "module")).ToLocal(&moduleVal);

        auto module = GPUShaderModuleImpl::GetPointer(moduleVal.As<v8::Object>());

        fragment->module = module->GetShaderModule();

        descriptor.fragment = fragment;

    }


    v8::Local<v8::Value> labelVal;
    options->Get(context, ConvertToV8String(isolate, "label")).ToLocal(
            &labelVal);


    if (!labelVal.IsEmpty() && labelVal->IsString()) {
        descriptor.label = *v8::String::Utf8Value(isolate, labelVal);
    }


    v8::Local<v8::Value> layoutVal;
    options->Get(context, ConvertToV8String(isolate, "layout")).ToLocal(
            &layoutVal);

    CanvasGPUPipelineLayoutOrGPUAutoLayoutMode layout;

    if (layoutVal->IsString()) {
        layout = CanvasGPUPipelineLayoutOrGPUAutoLayoutMode{
                CanvasGPUPipelineLayoutOrGPUAutoLayoutModeAuto
        };
    } else if (!layoutVal->IsNullOrUndefined() && layoutVal->IsObject()) {
        auto pipeline = GPUPipelineLayoutImpl::GetPointer(layoutVal.As<v8::Object>());
        layout = CanvasGPUPipelineLayoutOrGPUAutoLayoutMode{
                CanvasGPUPipelineLayoutOrGPUAutoLayoutModeLayout,
                layout.layout = pipeline->GetPipeline()
        };
    } else {
        // todo throw ?
        layout = CanvasGPUPipelineLayoutOrGPUAutoLayoutMode{
                CanvasGPUPipelineLayoutOrGPUAutoLayoutModeAuto
        };
    }

    descriptor.layout = layout;


    v8::Local<v8::Value> multisampleValue;
    options->Get(context, ConvertToV8String(isolate, "multisample")).ToLocal(
            &multisampleValue);


    CanvasMultisampleState *multisample = nullptr;

    if (!multisampleValue.IsEmpty() && multisampleValue->IsObject()) {
        auto multisampleObj = multisampleValue.As<v8::Object>();
        multisample = new CanvasMultisampleState{};
        multisample->alpha_to_coverage_enabled = true;
        multisample->count = 1;
        multisample->mask = 0xFFFFFFFF;

        v8::Local<v8::Value> alphaToCoverageEnabled;
        v8::Local<v8::Value> count;
        v8::Local<v8::Value> mask;

        multisampleObj->Get(context, ConvertToV8String(isolate, "alphaToCoverageEnabled")).
                ToLocal(&alphaToCoverageEnabled);

        if (!alphaToCoverageEnabled.IsEmpty() && alphaToCoverageEnabled->IsBoolean()) {
            multisample->alpha_to_coverage_enabled = alphaToCoverageEnabled->BooleanValue(
                    isolate);
        }

        multisampleObj->Get(context, ConvertToV8String(isolate, "count")).
                ToLocal(&count);

        if (!count.IsEmpty() && count->IsUint32()) {
            multisample->count = count.As<v8::Uint32>()->Value();
        }

        multisampleObj->Get(context, ConvertToV8String(isolate, "mask")).
                ToLocal(&mask);

        if (!mask.IsEmpty() && mask->IsNumber()) {
            // todo verify mask
            auto maskValue = mask.As<v8::Number>()->Value();
            multisample->mask = (uint64_t) maskValue;
        }


        descriptor.multisample = multisample;

    }


    v8::Local<v8::Value> primitiveValue;
    options->Get(context, ConvertToV8String(isolate, "primitive")).ToLocal(
            &primitiveValue);


    CanvasPrimitiveState *primitive = nullptr;

    if (!primitiveValue.IsEmpty() && primitiveValue->IsObject()) {
        auto primitiveObj = primitiveValue.As<v8::Object>();
        primitive = new CanvasPrimitiveState{};

        primitive->cull_mode = CanvasCullMode::CanvasCullModeNone;
        primitive->front_face = CanvasFrontFaceCcw;
        primitive->strip_index_format = CanvasOptionalIndexFormat{
                CanvasOptionalIndexFormatNone
        };

        primitive->topology = CanvasPrimitiveTopologyTriangleList;

        primitive->unclipped_depth = false;


        v8::Local<v8::Value> cullModeValue;

        if (primitiveObj->Get(context, ConvertToV8String(isolate, "cullMode")).ToLocal(
                                                                                       &cullModeValue)) {
            if (cullModeValue->IsUint32()) {
                auto cullMode = cullModeValue.As<v8::Uint32>()->Value();

                switch (cullMode) {
                    case 0:
                        primitive->cull_mode = CanvasCullMode::CanvasCullModeNone;
                        break;
                    case 1:
                        primitive->cull_mode = CanvasCullMode::CanvasCullModeFront;
                        break;
                    case 2:
                        primitive->cull_mode = CanvasCullMode::CanvasCullModeBack;
                        break;
                }
            } else if (cullModeValue->IsString()) {

                auto cullMode = ConvertFromV8String(isolate, cullModeValue);

                if (cullMode == "none") {
                    primitive->cull_mode = CanvasCullMode::CanvasCullModeNone;
                } else if (cullMode == "front") {
                    primitive->cull_mode = CanvasCullMode::CanvasCullModeFront;
                } else if (cullMode == "back") {
                    primitive->cull_mode = CanvasCullMode::CanvasCullModeBack;
                }
            }

        }

        v8::Local<v8::Value> frontFaceValue;

        if (primitiveObj->Get(context, ConvertToV8String(isolate, "frontFace")).ToLocal(
                                                                                        &frontFaceValue)) {
            if (frontFaceValue->IsUint32()) {
                auto frontFace = frontFaceValue.As<v8::Uint32>()->Value();
                switch (frontFace) {
                    case 0:
                        primitive->front_face = CanvasFrontFace::CanvasFrontFaceCcw;
                        break;
                    case 1:
                        primitive->front_face = CanvasFrontFace::CanvasFrontFaceCw;
                        break;
                    default:
                        break;
                }
            } else if (frontFaceValue->IsString()) {
                auto frontFace = ConvertFromV8String(isolate, frontFaceValue);
                if (frontFace == "ccw") {
                    primitive->front_face = CanvasFrontFace::CanvasFrontFaceCcw;
                } else if (frontFace == "cw") {
                    primitive->front_face = CanvasFrontFace::CanvasFrontFaceCw;
                }
            }
        }


        v8::Local<v8::Value> stripIndexFormatValue;

        if (primitiveObj->Get(context, ConvertToV8String(isolate, "stripIndexFormat")).ToLocal(
                                                                                               &stripIndexFormatValue)) {
            if (stripIndexFormatValue->IsUint32()) {
                auto stripIndexFormat = stripIndexFormatValue.As<v8::Uint32>()->Value();
                switch (stripIndexFormat) {
                    case 0:
                        primitive->strip_index_format = CanvasOptionalIndexFormat{
                                CanvasOptionalIndexFormatSome,
                                CanvasIndexFormat::CanvasIndexFormatUint16
                        };
                        break;
                    case 1:
                        primitive->strip_index_format = CanvasOptionalIndexFormat{
                                CanvasOptionalIndexFormatSome,
                                CanvasIndexFormat::CanvasIndexFormatUint32
                        };
                        break;
                    default:
                        break;
                }
            } else if (stripIndexFormatValue->IsString()) {
                auto stripIndexFormat = ConvertFromV8String(isolate, stripIndexFormatValue);


                if (stripIndexFormat == "uint16") {
                    primitive->strip_index_format = CanvasOptionalIndexFormat{
                            CanvasOptionalIndexFormatSome,
                            CanvasIndexFormat::CanvasIndexFormatUint16
                    };
                } else if (stripIndexFormat == "uint32") {
                    primitive->strip_index_format = CanvasOptionalIndexFormat{
                            CanvasOptionalIndexFormatSome,
                            CanvasIndexFormat::CanvasIndexFormatUint32
                    };
                }
            }
        }


        v8::Local<v8::Value> topologyValue;

        if (primitiveObj->Get(context, ConvertToV8String(isolate, "topology")).ToLocal(
                                                                                       &topologyValue)) {

            if (topologyValue->IsUint32()) {
                auto topology = topologyValue.As<v8::Uint32>()->Value();
                switch (topology) {
                    case 0:
                        primitive->topology = CanvasPrimitiveTopology::CanvasPrimitiveTopologyPointList;
                        break;
                    case 1:
                        primitive->topology = CanvasPrimitiveTopology::CanvasPrimitiveTopologyLineList;
                        break;
                    case 2:
                        primitive->topology = CanvasPrimitiveTopology::CanvasPrimitiveTopologyLineStrip;
                        break;
                    case 3:
                        primitive->topology = CanvasPrimitiveTopology::CanvasPrimitiveTopologyTriangleList;
                        break;
                    case 4:
                        primitive->topology = CanvasPrimitiveTopology::CanvasPrimitiveTopologyTriangleStrip;
                        break;
                    default:
                        break;
                }
            } else if (topologyValue->IsString()) {
                auto topology = ConvertFromV8String(isolate, topologyValue);
                if (topology == "line-list") {
                    primitive->topology = CanvasPrimitiveTopology::CanvasPrimitiveTopologyLineList;
                } else if (topology == "line-strip") {
                    primitive->topology = CanvasPrimitiveTopology::CanvasPrimitiveTopologyLineStrip;
                } else if (topology == "point-list") {
                    primitive->topology = CanvasPrimitiveTopology::CanvasPrimitiveTopologyPointList;
                } else if (topology == "triangle-list") {
                    primitive->topology = CanvasPrimitiveTopology::CanvasPrimitiveTopologyTriangleList;
                } else if (topology == "triangle-strip") {
                    primitive->topology = CanvasPrimitiveTopology::CanvasPrimitiveTopologyTriangleStrip;
                }
            }

        }


        v8::Local<v8::Value> unclippedDepthValue;
        primitiveObj->Get(context, ConvertToV8String(isolate, "unclippedDepth")).ToLocal(
                &unclippedDepthValue);

        if (!unclippedDepthValue.IsEmpty() && unclippedDepthValue->IsBoolean()) {
            primitive->unclipped_depth = unclippedDepthValue->BooleanValue(isolate);
        }

        descriptor.primitive = primitive;

    }


    v8::Local<v8::Value> vertexValue;
    options->Get(context, ConvertToV8String(isolate, "vertex")).ToLocal(
            &vertexValue);


    CanvasVertexState *vertex = nullptr;

    std::vector<CanvasVertexBufferLayout> bufferLayout;

    std::vector<std::vector<CanvasVertexAttribute>> attributes;


    if (!vertexValue.IsEmpty() && vertexValue->IsObject()) {
        auto vertexObj = vertexValue.As<v8::Object>();
        vertex = new CanvasVertexState{};

        v8::Local<v8::Value> moduleVal;
        vertexObj->Get(context, ConvertToV8String(isolate, "module")).ToLocal(&moduleVal);

        auto module = GPUShaderModuleImpl::GetPointer(moduleVal.As<v8::Object>());

        vertex->module = module->GetShaderModule();

        v8::Local<v8::Value> constantsVal;
        vertexObj->Get(context, ConvertToV8String(isolate, "constants")).ToLocal(&constantsVal);

        if (!constantsVal.IsEmpty() && constantsVal->IsMap()) {
            auto constants = constantsVal.As<v8::Map>();
            auto keyValues = constants->AsArray();
            auto len = keyValues->Length();
            CanvasConstants *store = nullptr;

            if (len > 0) {
                store = canvas_native_webgpu_constants_create();
                for (int i = 0; i < len; i += 2) {
                    auto k = i;
                    auto v = k + 1;

                    v8::Local<v8::Value> keyVal;
                    keyValues->Get(context, k).ToLocal(&keyVal);
                    v8::Local<v8::Value> valueVal;
                    keyValues->Get(context, v).ToLocal(&valueVal);


                    if (!keyVal.IsEmpty() && keyVal->IsString() && !valueVal.IsEmpty() &&
                        valueVal->IsNumber()) {
                        canvas_native_webgpu_constants_insert(
                                store,
                                *v8::String::Utf8Value(isolate, keyVal),
                                valueVal.As<v8::Number>()->Value()
                        );
                    }

                }
            }

            vertex->constants = store;

        }

        v8::Local<v8::Value> buffersVal;
        vertexObj->Get(context, ConvertToV8String(isolate, "buffers")).ToLocal(&buffersVal);

        uint64_t stride = 0;
        if (!buffersVal.IsEmpty() && buffersVal->IsArray()) {
            auto buffers = buffersVal.As<v8::Array>();
            auto len = buffers->Length();

            for (int i = 0; i < len; i++) {
                auto buffer = buffers->Get(context, i).ToLocalChecked().As<v8::Object>();

                v8::Local<v8::Value> arrayStride;

                buffer->Get(context, ConvertToV8String(isolate, "arrayStride")).ToLocal(
                        &arrayStride);

                if (!arrayStride.IsEmpty() && arrayStride->IsNumber()) {
                    stride = (uint64_t) arrayStride.As<v8::Number>()->Value();
                }

                std::vector<CanvasVertexAttribute> attributes_;

                v8::Local<v8::Value> attributesValue;

                buffer->Get(context, ConvertToV8String(isolate, "attributes")).ToLocal(
                        &attributesValue);

                if (!attributesValue.IsEmpty() && attributesValue->IsArray()) {
                    auto attributes_array = attributesValue.As<v8::Array>();
                    auto attributes_len = attributes_array->Length();

                    for (int j = 0; j < attributes_len; j++) {
                        auto attr = attributes_array->Get(context,
                                                          j).ToLocalChecked().As<v8::Object>();
                        auto format = attr->Get(context, ConvertToV8String(isolate,
                                                                           "format")).ToLocalChecked()->Uint32Value(
                                context).ToChecked();

                        auto offset = (uint64_t) attr->Get(context, ConvertToV8String(isolate,
                                                                                      "offset")).ToLocalChecked()->NumberValue(
                                context).ToChecked();
                        auto shaderLocation = attr->Get(context, ConvertToV8String(isolate,
                                                                                   "shaderLocation")).ToLocalChecked()->Uint32Value(
                                context).ToChecked();

                        auto attribute = CanvasVertexAttribute{
                                (CanvasVertexFormat) format,
                                offset,
                                shaderLocation
                        };

                        attributes_.push_back(attribute);
                    }

                    attributes.push_back(attributes_);
                }


                CanvasVertexStepMode stepMode = CanvasVertexStepModeVertex;

                v8::Local<v8::Value> stepModeVal;

                buffer->Get(context, ConvertToV8String(isolate, "stepMode")).ToLocal(
                        &stepModeVal);


                if (!stepModeVal.IsEmpty()) {
                    if (stepModeVal->IsUint32()) {
                        switch (stepModeVal.As<v8::Uint32>()->Value()) {
                            case 0:
                                stepMode = CanvasVertexStepMode::CanvasVertexStepModeVertex;
                                break;
                            case 1:
                                stepMode = CanvasVertexStepMode::CanvasVertexStepModeInstance;
                                break;
                        }
                    } else if (stepModeVal->IsString()) {
                        auto stepModeStr = ConvertFromV8String(isolate, stepModeVal);
                        if (stepModeStr == "vertex") {
                            stepMode = CanvasVertexStepMode::CanvasVertexStepModeVertex;
                        } else if (stepModeStr == "instance") {
                            stepMode = CanvasVertexStepMode::CanvasVertexStepModeInstance;
                        }
                    }
                }

                auto vertexBufferLayout = CanvasVertexBufferLayout{
                        stride,
                        stepMode,
                        attributes[i].data(),
                        attributes[i].size()
                };

                bufferLayout.push_back(vertexBufferLayout);
            }

            vertex->buffers = bufferLayout.data();
            vertex->buffers_size = bufferLayout.size();

        }


        v8::Local<v8::Value> entryPoint;
        vertexObj->Get(context, ConvertToV8String(isolate, "entryPoint")).ToLocal(&entryPoint);


        if (!entryPoint.IsEmpty() && entryPoint->IsString()) {
            auto ep = v8::String::Utf8Value(isolate, entryPoint);
            char *entry_point = (char *) malloc(ep.length());
            std::strcpy(entry_point, *ep);
            vertex->entry_point = entry_point;
        }

        descriptor.vertex = vertex;

    }


    auto pipeline = canvas_native_webgpu_device_create_render_pipeline(ptr->GetGPUDevice(),
                                                                       &descriptor);


    if (descriptor.fragment != nullptr) {
        if (descriptor.fragment->entry_point != nullptr) {
            free((void *) descriptor.fragment->entry_point);
        }
    }

    if (descriptor.primitive != nullptr) {
        delete descriptor.primitive;
    }

    if (descriptor.multisample != nullptr) {
        delete descriptor.multisample;
    }


    if (descriptor.vertex != nullptr) {
        if (descriptor.vertex->constants != nullptr) {
            canvas_native_webgpu_constants_destroy(
                    (CanvasConstants *) descriptor.vertex->constants);
        }

        if (descriptor.vertex->entry_point != nullptr) {
            free((void *) descriptor.vertex->entry_point);
        }

    }


    if (pipeline != nullptr) {
        auto ret = GPURenderPipelineImpl::NewInstance(isolate, new GPURenderPipelineImpl(pipeline));
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetUndefined();
}

void GPUDeviceImpl::CreateSampler(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto optionsVal = args[0];

    if (!optionsVal->IsNullOrUndefined() && optionsVal->IsObject()) {
        auto options = optionsVal.As<v8::Object>();

        v8::Local<v8::Value> labelVal;
        options->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);

        char *label = nullptr;

        if (!labelVal.IsEmpty() && labelVal->IsString()) {
            label = *v8::String::Utf8Value(isolate, labelVal);
        }

        auto addressModeU = CanvasAddressModeClampToEdge;

        v8::Local<v8::Value> addressModeUVal;
        options->Get(context, ConvertToV8String(isolate, "addressModeU")).ToLocal(&addressModeUVal);
        auto addressModeUStr = ConvertFromV8String(isolate, addressModeUVal);
        if (addressModeUStr == "repeat") {
            addressModeU = CanvasAddressModeRepeat;
        } else if (addressModeUStr == "mirror-repeat") {
            addressModeU = CanvasAddressModeMirrorRepeat;
        }


        auto addressModeV = CanvasAddressModeClampToEdge;

        v8::Local<v8::Value> addressModeVVal;
        options->Get(context, ConvertToV8String(isolate, "addressModeV")).ToLocal(&addressModeVVal);
        auto addressModeVStr = ConvertFromV8String(isolate, addressModeVVal);
        if (addressModeVStr == "repeat") {
            addressModeV = CanvasAddressModeRepeat;
        } else if (addressModeVStr == "mirror-repeat") {
            addressModeV = CanvasAddressModeMirrorRepeat;
        }


        auto addressModeW = CanvasAddressModeClampToEdge;

        v8::Local<v8::Value> addressModeWVal;
        options->Get(context, ConvertToV8String(isolate, "addressModeW")).ToLocal(&addressModeWVal);
        auto addressModeWStr = ConvertFromV8String(isolate, addressModeWVal);
        if (addressModeWStr == "repeat") {
            addressModeW = CanvasAddressModeRepeat;
        } else if (addressModeWStr == "mirror-repeat") {
            addressModeW = CanvasAddressModeMirrorRepeat;
        }


        auto magFilter = CanvasFilterModeNearest;

        v8::Local<v8::Value> magFilterVal;
        options->Get(context, ConvertToV8String(isolate, "magFilter")).ToLocal(&magFilterVal);
        auto magFilterStr = ConvertFromV8String(isolate, magFilterVal);
        if (magFilterStr == "linear") {
            magFilter = CanvasFilterModeLinear;
        }

        auto minFilter = CanvasFilterModeNearest;

        v8::Local<v8::Value> minFilterVal;
        options->Get(context, ConvertToV8String(isolate, "minFilter")).ToLocal(&minFilterVal);
        auto minFilterStr = ConvertFromV8String(isolate, minFilterVal);
        if (minFilterStr == "linear") {
            minFilter = CanvasFilterModeLinear;
        }


        auto mipmapFilter = CanvasFilterModeNearest;

        v8::Local<v8::Value> mipmapFilterVal;
        options->Get(context, ConvertToV8String(isolate, "mipmapFilter")).ToLocal(&mipmapFilterVal);
        auto mipmapFilterStr = ConvertFromV8String(isolate, mipmapFilterVal);
        if (mipmapFilterStr == "linear") {
            mipmapFilter = CanvasFilterModeLinear;
        }


        float lodMinClamp = 0;

        v8::Local<v8::Value> lodMinClampVal;
        options->Get(context, ConvertToV8String(isolate, "lodMinClamp")).ToLocal(&lodMinClampVal);

        if (!lodMinClampVal.IsEmpty() && lodMinClampVal->IsNumber()) {
            lodMinClamp = (float) lodMinClampVal->NumberValue(context).FromJust();
        }


        float lodMaxClamp = 32;

        v8::Local<v8::Value> lodMaxClampVal;
        options->Get(context, ConvertToV8String(isolate, "lodMaxClamp")).ToLocal(&lodMaxClampVal);

        if (!lodMaxClampVal.IsEmpty() && lodMaxClampVal->IsNumber()) {
            lodMaxClamp = (float) lodMaxClampVal->NumberValue(context).FromJust();
        }


        CanvasOptionalCompareFunction compare{
                CanvasOptionalCompareFunctionNone
        };


        v8::Local<v8::Value> compareVal;
        options->Get(context, ConvertToV8String(isolate, "compare")).ToLocal(&compareVal);

        auto compareStr = ConvertFromV8String(isolate, compareVal);

        if (compareStr == "never") {
            compare.tag = CanvasOptionalCompareFunctionSome;
            compare.some = CanvasCompareFunctionNever;
        } else if (compareStr == "less") {
            compare.tag = CanvasOptionalCompareFunctionSome;
            compare.some = CanvasCompareFunctionLess;
        } else if (compareStr == "equal") {
            compare.tag = CanvasOptionalCompareFunctionSome;
            compare.some = CanvasCompareFunctionEqual;
        } else if (compareStr == "less-equal") {
            compare.tag = CanvasOptionalCompareFunctionSome;
            compare.some = CanvasCompareFunctionLessEqual;
        } else if (compareStr == "greater") {
            compare.tag = CanvasOptionalCompareFunctionSome;
            compare.some = CanvasCompareFunctionGreater;
        } else if (compareStr == "not-equal") {
            compare.tag = CanvasOptionalCompareFunctionSome;
            compare.some = CanvasCompareFunctionNotEqual;
        } else if (compareStr == "greater-equal") {
            compare.tag = CanvasOptionalCompareFunctionSome;
            compare.some = CanvasCompareFunctionGreaterEqual;
        } else if (compareStr == "always") {
            compare.tag = CanvasOptionalCompareFunctionSome;
            compare.some = CanvasCompareFunctionAlways;
        }

        uint16_t maxAnisotropy = 1;

        v8::Local<v8::Value> maxAnisotropyVal;
        options->Get(context, ConvertToV8String(isolate, "maxAnisotropy")).ToLocal(
                &maxAnisotropyVal);

        if (!maxAnisotropyVal.IsEmpty() && maxAnisotropyVal->IsNumber()) {
            maxAnisotropy = (u_short) maxAnisotropyVal->NumberValue(context).FromJust();
        }


        CanvasCreateSamplerDescriptor descriptor{
                label,
                addressModeU,
                addressModeV,
                addressModeW,
                magFilter,
                minFilter,
                mipmapFilter,
                lodMinClamp,
                lodMaxClamp,
                compare,
                maxAnisotropy
        };

        auto sampler = canvas_native_webgpu_device_create_sampler(ptr->GetGPUDevice(), &descriptor);
        if (sampler != nullptr) {
            auto ret = GPUSamplerImpl::NewInstance(isolate, new GPUSamplerImpl(sampler));
            args.GetReturnValue().Set(ret);
            return;
        }

    } else {
        auto sampler = canvas_native_webgpu_device_create_sampler(ptr->GetGPUDevice(), nullptr);
        if (sampler != nullptr) {
            auto ret = GPUSamplerImpl::NewInstance(isolate, new GPUSamplerImpl(sampler));
            args.GetReturnValue().Set(ret);
            return;
        }
    }

    args.GetReturnValue().SetUndefined();

}

void GPUDeviceImpl::CreateShaderModule(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto descVal = args[0];


    if (!descVal->IsNullOrUndefined() && descVal->IsObject()) {
        auto desc = descVal.As<v8::Object>();


        v8::Local<v8::Value> labelVal;
        desc->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);

        char *label = nullptr;

        if (!labelVal.IsEmpty() && labelVal->IsString()) {
            label = *v8::String::Utf8Value(isolate, labelVal);
        }

        v8::Local<v8::Value> codeVal;

        std::string code;
        if (desc->Get(context, ConvertToV8String(isolate, "code")).ToLocal(&codeVal) && codeVal->IsString()) {
            code = ConvertFromV8String(isolate, codeVal);
        }

        auto module = canvas_native_webgpu_device_create_shader_module(ptr->GetGPUDevice(), label,
                                                                       code.c_str());

        if (module != nullptr) {
            auto instance = new GPUShaderModuleImpl(module);
            auto ret = GPUShaderModuleImpl::NewInstance(isolate, instance);
            args.GetReturnValue().Set(ret);
            return;
        }

    }

    args.GetReturnValue().SetUndefined();


}

void GPUDeviceImpl::CreateTexture(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    CanvasCreateTextureDescriptor descriptor{};
    descriptor.dimension = CanvasTextureDimension::CanvasTextureDimensionD2;
    descriptor.depthOrArrayLayers = 1;
    descriptor.sampleCount = 1;
    descriptor.mipLevelCount = 1;
    descriptor.view_formats = nullptr;
    descriptor.view_formats_size = 0;


    char *error = nullptr;


    auto optionsVal = args[0];


    if (optionsVal->IsObject()) {
        auto options = optionsVal.As<v8::Object>();
        v8::Local<v8::Value> depthOrArrayLayersVal;

        if (options->Get(context, ConvertToV8String(isolate, "depthOrArrayLayers")).ToLocal(
                                                                                            &depthOrArrayLayersVal) && depthOrArrayLayersVal->IsUint32()) {
            descriptor.depthOrArrayLayers = depthOrArrayLayersVal.As<v8::Uint32>()->Value();
        }


        v8::Local<v8::Value> widthVal;

        if ( options->Get(context, ConvertToV8String(isolate, "width")).ToLocal(
                                                                                &widthVal) &&widthVal->IsUint32()) {
            descriptor.width = widthVal.As<v8::Uint32>()->Value();
        }


        v8::Local<v8::Value> heightVal;

        if (options->Get(context, ConvertToV8String(isolate, "height")).ToLocal(
                                                                                &heightVal) && heightVal->IsUint32()) {
            descriptor.height = heightVal.As<v8::Uint32>()->Value();
        }


        v8::Local<v8::Value> usageVal;

        if (options->Get(context, ConvertToV8String(isolate, "usage")).ToLocal(
                                                                               &usageVal) && usageVal->IsUint32()) {
            descriptor.usage = usageVal.As<v8::Uint32>()->Value();
        }


        v8::Local<v8::Value> sampleCountVal;
        options->Get(context, ConvertToV8String(isolate, "sampleCount")).ToLocal(
                &sampleCountVal);

        if (sampleCountVal->IsUint32()) {
            descriptor.sampleCount = sampleCountVal.As<v8::Uint32>()->Value();
        }


        v8::Local<v8::Value> mipLevelCountVal;
        options->Get(context, ConvertToV8String(isolate, "mipLevelCount")).ToLocal(
                &mipLevelCountVal);

        if (mipLevelCountVal->IsUint32()) {
            descriptor.mipLevelCount = mipLevelCountVal.As<v8::Uint32>()->Value();
        }


        v8::Local<v8::Value> dimensionVal;
        options->Get(context, ConvertToV8String(isolate, "dimension")).ToLocal(
                &dimensionVal);

        if (dimensionVal->IsString()) {
            auto dimension = ConvertFromV8String(isolate, dimensionVal);

            // todo use enum
            if (dimension == "1d") {
                descriptor.dimension = CanvasTextureDimension::CanvasTextureDimensionD1;
            } else if (dimension == "2d") {
                descriptor.dimension = CanvasTextureDimension::CanvasTextureDimensionD2;
            } else if (dimension == "3d") {
                descriptor.dimension = CanvasTextureDimension::CanvasTextureDimensionD3;
            }

        }


        v8::Local<v8::Value> formatVal;
        options->Get(context, ConvertToV8String(isolate, "format")).ToLocal(
                &formatVal);

        if (formatVal->IsString()) {
            auto format = ConvertFromV8String(isolate, formatVal);

            // todo use enum
            //CanvasGPUTextureFormat fmt{};

            auto fmtOpt = canvas_native_webgpu_enum_string_to_gpu_texture(format.c_str());
            if (fmtOpt.tag == CanvasOptionalGPUTextureFormatSome) {
                descriptor.format = fmtOpt.some;
            }
//            if (strcmp(format.c_str(), "bgra8unorm") == 0) {
//                fmt.tag = CanvasGPUTextureFormat_Tag::CanvasGPUTextureFormatBgra8Unorm;
//                descriptor.format = fmt;
//            } else if (strcmp(format.c_str(), "rgba8unorm") == 0) {
//                fmt.tag = CanvasGPUTextureFormat_Tag::CanvasGPUTextureFormatRgba8Unorm;
//                descriptor.format = fmt;
//            }

        }

    }

    auto texture = canvas_native_webgpu_device_create_texture(ptr->GetGPUDevice(), &descriptor);

    if (texture != nullptr) {
        auto textureImpl = new GPUTextureImpl(texture);
        auto ret = GPUTextureImpl::NewInstance(isolate, textureImpl);
        args.GetReturnValue().Set(ret);

    } else {
        if (error != nullptr) {
            isolate->ThrowError(ConvertToV8String(isolate, error));
        }

        args.GetReturnValue().SetUndefined();
    }
}

void GPUDeviceImpl::Destroy(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    canvas_native_webgpu_device_destroy(ptr->GetGPUDevice());
}

void GPUDeviceImpl::PopErrorScope(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto cb = args[0];
    auto callback = new JSICallback(isolate, cb.As<v8::Function>());

    canvas_native_webgpu_device_pop_error_scope(ptr->GetGPUDevice(), [](CanvasGPUErrorType type,
                                                                        char *msg,
                                                                        void *data) {
        auto cb = static_cast<JSICallback *>(data);

        v8::Isolate *isolate = cb->isolate_;
        v8::Locker locker(isolate);
        v8::Isolate::Scope isolate_scope(
                isolate);
        v8::HandleScope handle_scope(
                isolate);
        v8::Local<v8::Function> callback = cb->callback_->Get(
                isolate);
        v8::Local<v8::Context> context = callback->GetCreationContextChecked();
        v8::Context::Scope context_scope(
                context);
        v8::Local<v8::Value> typ;
        v8::Local<v8::Value> message;
        switch (type) {
            case CanvasGPUErrorType::CanvasGPUErrorTypeNone:
                typ = v8::Number::New(
                        isolate, 0);
                message = v8::Null(
                        isolate);
                break;
            case CanvasGPUErrorType::CanvasGPUErrorTypeLost:
                typ = v8::Number::New(
                        isolate, 1);
                message = v8::Null(
                        isolate);
                break;
            case CanvasGPUErrorType::CanvasGPUErrorTypeOutOfMemory:
                typ = v8::Number::New(
                        isolate, 2);
                message = v8::Null(
                        isolate);
                break;
            case CanvasGPUErrorType::CanvasGPUErrorTypeValidation:
                typ = v8::Number::New(
                        isolate, 3);
                message = ConvertToV8String(
                        isolate, msg);
                break;
            case CanvasGPUErrorType::CanvasGPUErrorTypeInternal:
                typ = v8::Number::New(
                        isolate, 4);
                message = v8::Null(
                        isolate);
                break;
        }
        v8::Local<v8::Value> args[2] = {
                typ, message
        };

        callback->Call(context,
                       context->Global(),
                       2, args);

        delete static_cast<JSICallback *>(data);
    }, callback);
}

void GPUDeviceImpl::PushErrorScope(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto scope = ConvertFromV8String(args.GetIsolate(), args[0]);

    if (scope == "internal") {
        canvas_native_webgpu_device_push_error_scope(ptr->GetGPUDevice(),
                                                     CanvasGPUErrorFilterInternal);
    } else if (scope == "out-of-memory") {
        canvas_native_webgpu_device_push_error_scope(ptr->GetGPUDevice(),
                                                     CanvasGPUErrorFilterOutOfMemory);
    } else if (scope == "validation") {
        canvas_native_webgpu_device_push_error_scope(ptr->GetGPUDevice(),
                                                     CanvasGPUErrorFilterValidation);
    }


}
