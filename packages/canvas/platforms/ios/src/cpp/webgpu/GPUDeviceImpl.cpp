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

GPUDeviceImpl::GPUDeviceImpl(CanvasGPUDevice *device) : device_(device) {}

CanvasGPUDevice *GPUDeviceImpl::GetGPUDevice() {
    return this->device_;
}


void GPUDeviceImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUDevice"), func);
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
            ConvertToV8String(isolate, "destroy"),
            v8::FunctionTemplate::New(isolate, &Destroy));

    tmpl->Set(
            ConvertToV8String(isolate, "createBuffer"),
            v8::FunctionTemplate::New(isolate, &CreateBuffer));

    tmpl->Set(
            ConvertToV8String(isolate, "createCommandEncoder"),
            v8::FunctionTemplate::New(isolate, &CreateCommandEncoder));

    tmpl->Set(
            ConvertToV8String(isolate, "createRenderPipeline"),
            v8::FunctionTemplate::New(isolate, &CreateRenderPipeline));

    tmpl->Set(
            ConvertToV8String(isolate, "createTexture"),
            v8::FunctionTemplate::New(isolate, &CreateTexture));

    tmpl->Set(
            ConvertToV8String(isolate, "createShaderModule"),
            v8::FunctionTemplate::New(isolate, &CreateShaderModule));


    cache->GPUDeviceTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
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


void GPUDeviceImpl::Destroy(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    canvas_native_webgpu_device_destroy(ptr->GetGPUDevice());
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

    char *error = nullptr;


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
                                                            mappedAtCreation, error);

    if (buffer != nullptr) {
        auto bufImpl = new GPUBufferImpl(buffer);
        auto ret = GPUBufferImpl::NewInstance(isolate, bufImpl);
        args.GetReturnValue().Set(ret);

    } else {
        if (error != nullptr) {
            isolate->ThrowError(ConvertToV8String(isolate, error));
        }

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

void GPUDeviceImpl::CreateTexture(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    CanvasCreateTextureDescriptor descriptor{};
    descriptor.dimension = CanvasTextureDimension::CanvasTextureDimensionD1;
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
        options->Get(context, ConvertToV8String(isolate, "depthOrArrayLayers")).ToLocal(
                &depthOrArrayLayersVal);

        if (depthOrArrayLayersVal->IsUint32()) {
            descriptor.depthOrArrayLayers = depthOrArrayLayersVal.As<v8::Uint32>()->Value();
        }


        v8::Local<v8::Value> widthVal;
        options->Get(context, ConvertToV8String(isolate, "width")).ToLocal(
                &widthVal);

        if (widthVal->IsUint32()) {
            descriptor.width = widthVal.As<v8::Uint32>()->Value();
        }


        v8::Local<v8::Value> heightVal;
        options->Get(context, ConvertToV8String(isolate, "height")).ToLocal(
                &heightVal);

        if (heightVal->IsUint32()) {
            descriptor.height = heightVal.As<v8::Uint32>()->Value();
        }


        v8::Local<v8::Value> usageVal;
        options->Get(context, ConvertToV8String(isolate, "usage")).ToLocal(
                &usageVal);

        if (usageVal->IsUint32()) {
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
            if (strcmp(dimension.c_str(), "d1") == 0) {
                descriptor.dimension = CanvasTextureDimension::CanvasTextureDimensionD1;
            } else if (strcmp(dimension.c_str(), "d2") == 0) {
                descriptor.dimension = CanvasTextureDimension::CanvasTextureDimensionD2;
            } else if (strcmp(dimension.c_str(), "d3") == 0) {
                descriptor.dimension = CanvasTextureDimension::CanvasTextureDimensionD3;
            }

        }


        v8::Local<v8::Value> formatVal;
        options->Get(context, ConvertToV8String(isolate, "format")).ToLocal(
                &formatVal);

        if (formatVal->IsString()) {
            auto format = ConvertFromV8String(isolate, formatVal);

            // todo use enum
            CanvasGPUTextureFormat fmt{};
            if (strcmp(format.c_str(), "bgra8unorm") == 0) {
                fmt.tag = CanvasGPUTextureFormat_Tag::CanvasGPUTextureFormatBgra8Unorm;
                descriptor.format = fmt;
            } else if (strcmp(format.c_str(), "rgba8unorm") == 0) {
                fmt.tag = CanvasGPUTextureFormat_Tag::CanvasGPUTextureFormatRgba8Unorm;
                descriptor.format = fmt;
            }

        }

    }

    auto texture = canvas_native_webgpu_device_create_texture(ptr->GetGPUDevice(), &descriptor,
                                                              error);

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

void GPUDeviceImpl::CreateRenderPipeline(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    CanvasCreateRenderPipelineDescriptor descriptor{};
    descriptor.label = nullptr;

    char *error = nullptr;


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

    if (stencilValue->IsObject()) {
        auto stencilObj = stencilValue.As<v8::Object>();
        stencil = new CanvasDepthStencilState{};
        stencil->depth_bias = 0;
        stencil->depth_bias_clamp = 0;
        stencil->depth_bias_slope_scale = 0;
        stencil->stencil_read_mask = 0xFFFFFFFF;
        stencil->stencil_write_mask = 0xFFFFFFFF;
        // todo throw if failed
        v8::Local<v8::Value> formatValue;

        stencilObj->Get(context, ConvertToV8String(isolate, "format")).ToLocal(&formatValue);
        if (!formatValue.IsEmpty() && formatValue->IsString()) {
            auto val = *v8::String::Utf8Value(isolate, formatValue);
            auto format = canvas_native_webgpu_string_to_gpu_texture_enum(
                    val);
            if (format.tag ==
                CanvasOptionalGPUTextureFormat_Tag::CanvasOptionalGPUTextureFormatSome) {
                stencil->format = format.some;
            }
        } else {
            // todo throw
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
            auto formatResult = canvas_native_webgpu_string_to_gpu_texture_enum(formatStr.c_str());


            if (formatResult.tag == CanvasOptionalGPUTextureFormatNone) {
                // todo throw
                args.GetReturnValue().SetUndefined();
                return;
            } else {

            }

            auto format = CanvasGPUTextureFormat{
                    formatResult.some.tag
            };


            uint32_t writeMask = 0xF;

            v8::Local<v8::Value> writeMaskVal;

            state->Get(context, ConvertToV8String(isolate, "writeMask")).ToLocal(&writeMaskVal);


            if (!writeMaskVal.IsEmpty() && writeMaskVal->IsUint32()) {
                writeMask = writeMaskVal->Uint32Value(context).FromJust();
            }

            CanvasOptionalBlendState blend = CanvasOptionalBlendState{
                    CanvasOptionalBlendStateNone
            };


            v8::Local<v8::Value> blendVal;

            state->Get(context, ConvertToV8String(isolate, "blend")).ToLocal(&blendVal);


            if (!blendVal.IsEmpty() && blendVal->IsObject()) {
                auto blendObj = blendVal.As<v8::Object>();
                auto alpha = blendObj->Get(context, ConvertToV8String(isolate,
                                                                      "alpha")).ToLocalChecked().As<v8::Object>();
                auto alpha_src_factor = (CanvasBlendFactor) alpha->Get(context,
                                                                       ConvertToV8String(isolate,
                                                                                         "src_factor")).ToLocalChecked()->Uint32Value(
                        context).FromJust();
                auto alpha_dst_factor = (CanvasBlendFactor) alpha->Get(context,
                                                                       ConvertToV8String(isolate,
                                                                                         "dst_factor")).ToLocalChecked()->Uint32Value(
                        context).FromJust();
                auto alpha_operation = (CanvasBlendOperation) alpha->Get(context,
                                                                         ConvertToV8String(isolate,
                                                                                           "operation")).ToLocalChecked()->Uint32Value(
                        context).FromJust();

                auto alpha_val = CanvasBlendComponent{alpha_src_factor, alpha_dst_factor,
                                                      alpha_operation};

                auto color = blendObj->Get(context, ConvertToV8String(isolate,
                                                                      "color")).ToLocalChecked().As<v8::Object>();
                auto color_src_factor = (CanvasBlendFactor) color->Get(context,
                                                                       ConvertToV8String(isolate,
                                                                                         "src_factor")).ToLocalChecked()->Uint32Value(
                        context).FromJust();
                auto color_dst_factor = (CanvasBlendFactor) color->Get(context,
                                                                       ConvertToV8String(isolate,
                                                                                         "dst_factor")).ToLocalChecked()->Uint32Value(
                        context).FromJust();
                auto color_operation = (CanvasBlendOperation) color->Get(context,
                                                                         ConvertToV8String(isolate,
                                                                                           "operation")).ToLocalChecked()->Uint32Value(
                        context).FromJust();


                auto color_val = CanvasBlendComponent{color_src_factor, color_dst_factor,
                                                      color_operation};


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

        if (targets.size() > 0) {
            fragment->targets = targets.data();
            fragment->targets_size = targets.size();
        }


        v8::Local<v8::Value> constantsVal;
        fragmentValueObj->Get(context, ConvertToV8String(isolate, "constants")).ToLocal(
                &constantsVal);

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
    } else {
        auto pipeline = GPUPipelineLayoutImpl::GetPointer(layoutVal.As<v8::Object>());
        layout = CanvasGPUPipelineLayoutOrGPUAutoLayoutMode{
                CanvasGPUPipelineLayoutOrGPUAutoLayoutModeLayout,
                layout.layout = pipeline->GetPipeline()
        };
    }

    descriptor.layout = layout;


    v8::Local<v8::Value> multisampleValue;
    options->Get(context, ConvertToV8String(isolate, "multisample")).ToLocal(
            &stencilValue);


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
        primitiveObj->Get(context, ConvertToV8String(isolate, "cullMode")).ToLocal(
                &cullModeValue);

        if (!cullModeValue.IsEmpty() && cullModeValue->IsUint32()) {
            auto cullMode = cullModeValue.As<v8::Uint32>()->Value();

            switch (cullMode) {
                case 0:
                    primitive->cull_mode = CanvasCullMode::CanvasCullModeNone;
                    break;
                case 1:
                    primitive->cull_mode = CanvasCullMode::CanvasCullModeFront;
                    break;
                case 2:

                    break;
            }
        }


        v8::Local<v8::Value> frontFaceValue;
        primitiveObj->Get(context, ConvertToV8String(isolate, "frontFace")).ToLocal(
                &frontFaceValue);

        if (!frontFaceValue.IsEmpty() && frontFaceValue->IsUint32()) {
            auto cullMode = cullModeValue.As<v8::Uint32>()->Value();

            switch (cullMode) {
                case 0:
                    primitive->front_face = CanvasFrontFace::CanvasFrontFaceCcw;
                    break;
                case 1:
                    primitive->front_face = CanvasFrontFace::CanvasFrontFaceCw;
                    break;
                default:
                    break;
            }
        }


        v8::Local<v8::Value> stripIndexFormatValue;
        primitiveObj->Get(context, ConvertToV8String(isolate, "stripIndexFormat")).ToLocal(
                &stripIndexFormatValue);

        if (!stripIndexFormatValue.IsEmpty() && stripIndexFormatValue->IsUint32()) {
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
        }


        v8::Local<v8::Value> topologyValue;
        primitiveObj->Get(context, ConvertToV8String(isolate, "topology")).ToLocal(
                &topologyValue);

        if (!topologyValue.IsEmpty() && topologyValue->IsUint32()) {
            auto topology = stripIndexFormatValue.As<v8::Uint32>()->Value();
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
        }


        v8::Local<v8::Value> unclippedDepthValue;
        primitiveObj->Get(context, ConvertToV8String(isolate, "unclippedDepth")).ToLocal(
                &unclippedDepthValue);

        if (!unclippedDepthValue.IsEmpty()) {
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


                CanvasVertexStepMode stepMode = CanvasVertexStepMode::CanvasVertexStepModeVertex;

                v8::Local<v8::Value> stepModeVal;

                buffer->Get(context, ConvertToV8String(isolate, "stepMode")).ToLocal(
                        &stepModeVal);


                if (!stepModeVal.IsEmpty() && stepModeVal->IsUint32()) {
                    switch (stepModeVal.As<v8::Uint32>()->Value()) {
                        case 0:
                            stepMode = CanvasVertexStepMode::CanvasVertexStepModeVertex;
                            break;
                        case 1:
                            stepMode = CanvasVertexStepMode::CanvasVertexStepModeInstance;
                            break;
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
        desc->Get(context, ConvertToV8String(isolate, "code")).ToLocal(&codeVal);

        std::string code;
        if (!codeVal.IsEmpty() && codeVal->IsString()) {
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
