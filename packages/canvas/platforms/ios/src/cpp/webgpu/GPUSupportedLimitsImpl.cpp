//
// Created by Osei Fortune on 17/06/2024.
//

#include "GPUSupportedLimitsImpl.h"
#include "Caches.h"

GPUSupportedLimitsImpl::GPUSupportedLimitsImpl(CanvasGPUSupportedLimits *limits) : limits_(
        limits) {}

CanvasGPUSupportedLimits *GPUSupportedLimitsImpl::GetLimits() {
    return this->limits_;
}


void GPUSupportedLimitsImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUSupportedLimits"), func).FromJust();;
}

GPUSupportedLimitsImpl *GPUSupportedLimitsImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUSupportedLimitsImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUSupportedLimitsImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUSupportedLimitsTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, Ctor);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUSupportedLimits"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxTextureDimension1D"),
            GetMaxTextureDimension1D,
            SetMaxTextureDimension1D
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxTextureDimension2D"),
            GetMaxTextureDimension2D,
            SetMaxTextureDimension2D
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxTextureDimension3D"),
            GetMaxTextureDimension3D,
            SetMaxTextureDimension3D
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxTextureArrayLayers"),
            GetMaxTextureArrayLayers,
            SetMaxTextureArrayLayers
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxBindGroups"),
            GetMaxBindGroups,
            SetMaxBindGroups
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxBindingsPerBindGroup"),
            GetMaxBindingsPerBindGroup,
            SetMaxBindingsPerBindGroup
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxDynamicUniformBuffersPerPipelineLayout"),
            GetMaxDynamicUniformBuffersPerPipelineLayout,
            SetMaxDynamicUniformBuffersPerPipelineLayout
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxDynamicStorageBuffersPerPipelineLayout"),
            GetMaxDynamicStorageBuffersPerPipelineLayout,
            SetMaxDynamicStorageBuffersPerPipelineLayout
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxSampledTexturesPerShaderStage"),
            GetMaxSampledTexturesPerShaderStage,
            SetMaxSampledTexturesPerShaderStage
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxSamplersPerShaderStage"),
            GetMaxSamplersPerShaderStage,
            SetMaxSamplersPerShaderStage
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxStorageBuffersPerShaderStage"),
            GetMaxStorageBuffersPerShaderStage,
            SetMaxStorageBuffersPerShaderStage
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxStorageTexturesPerShaderStage"),
            GetMaxStorageTexturesPerShaderStage,
            SetMaxStorageTexturesPerShaderStage
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxUniformBuffersPerShaderStage"),
            GetMaxUniformBuffersPerShaderStage,
            SetMaxUniformBuffersPerShaderStage
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxUniformBufferBindingSize"),
            GetMaxUniformBufferBindingSize,
            SetMaxUniformBufferBindingSize
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxStorageBufferBindingSize"),
            GetMaxStorageBufferBindingSize,
            SetMaxStorageBufferBindingSize
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxVertexBuffers"),
            GetMaxVertexBuffers,
            SetMaxVertexBuffers
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxBufferSize"),
            GetMaxBufferSize,
            SetMaxBufferSize
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxVertexAttributes"),
            GetMaxVertexAttributes,
            SetMaxVertexAttributes
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxVertexBufferArrayStride"),
            GetMaxVertexBufferArrayStride,
            SetMaxVertexBufferArrayStride
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "minUniformBufferOffsetAlignment"),
            GetMinUniformBufferOffsetAlignment,
            SetMinUniformBufferOffsetAlignment
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "minStorageBufferOffsetAlignment"),
            GetMinStorageBufferOffsetAlignment,
            SetMinStorageBufferOffsetAlignment
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxInterStageShaderComponents"),
            GetMaxInterStageShaderComponents,
            SetMaxInterStageShaderComponents
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxColorAttachments"),
            GetMaxColorAttachments,
            SetMaxColorAttachments
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxColorAttachmentBytesPerSample"),
            GetMaxColorAttachmentBytesPerSample,
            SetMaxColorAttachmentBytesPerSample
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxComputeWorkgroupStorageSize"),
            GetMaxComputeWorkgroupStorageSize,
            SetMaxComputeWorkgroupStorageSize
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxComputeInvocationsPerWorkgroup"),
            GetMaxComputeInvocationsPerWorkgroup,
            SetMaxComputeInvocationsPerWorkgroup
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxComputeWorkgroupSizeX"),
            GetMaxComputeWorkgroupSizeX,
            SetMaxComputeWorkgroupSizeX
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxComputeWorkgroupSizeY"),
            GetMaxComputeWorkgroupSizeY,
            SetMaxComputeWorkgroupSizeY
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxComputeWorkgroupSizeZ"),
            GetMaxComputeWorkgroupSizeZ,
            SetMaxComputeWorkgroupSizeZ
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxComputeWorkgroupsPerDimension"),
            GetMaxComputeWorkgroupsPerDimension,
            SetMaxComputeWorkgroupsPerDimension
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "minSubgroupSize"),
            GetMinSubgroupSize,
            SetMinSubgroupSize
    );

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxSubgroupSize"),
            GetMaxSubgroupSize,
            SetMaxSubgroupSize
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxPushConstantSize"),
            GetMaxPushConstantSize,
            SetMaxPushConstantSize
    );


    tmpl->SetAccessor(
            ConvertToV8String(isolate, "maxNonSamplerBindings"),
            GetMaxNonSamplerBindings,
            SetMaxNonSamplerBindings
    );


    cache->GPUSupportedLimitsTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void GPUSupportedLimitsImpl::Ctor(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();

    auto ret = args.This();

    auto limits = canvas_native_webgpu_create_limits();
    auto object = new GPUSupportedLimitsImpl(limits);

    ret->SetAlignedPointerInInternalField(0, object);

    SetNativeType(ret, NativeType::GPUSupportedLimits);

    object->BindFinalizer(isolate, ret);

    args.GetReturnValue().Set(ret);
    return;
}

void GPUSupportedLimitsImpl::GetMaxTextureDimension1D(v8::Local<v8::String> property,
                                                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(8192);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_texture_dimension_1d);
}


void GPUSupportedLimitsImpl::SetMaxTextureDimension1D(v8::Local<v8::String> property,
                                                      v8::Local<v8::Value> value,
                                                      const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_texture_dimension_1d = value->Int32Value(context).ToChecked();
    }
}

void GPUSupportedLimitsImpl::GetMaxTextureDimension2D(v8::Local<v8::String> property,
                                                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(8192);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_texture_dimension_2d);
}


void GPUSupportedLimitsImpl::SetMaxTextureDimension2D(v8::Local<v8::String> property,
                                                      v8::Local<v8::Value> value,
                                                      const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_texture_dimension_2d = value->Int32Value(context).ToChecked();
    }
}

void GPUSupportedLimitsImpl::GetMaxTextureDimension3D(v8::Local<v8::String> property,
                                                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(2048);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_texture_dimension_3d);
}

void GPUSupportedLimitsImpl::SetMaxTextureDimension3D(v8::Local<v8::String> property,
                                                      v8::Local<v8::Value> value,
                                                      const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_texture_dimension_3d = value->Int32Value(context).ToChecked();
    }
}

void GPUSupportedLimitsImpl::GetMaxTextureArrayLayers(v8::Local<v8::String> property,
                                                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(256);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_texture_array_layers);
}


void GPUSupportedLimitsImpl::SetMaxTextureArrayLayers(v8::Local<v8::String> property,
                                                      v8::Local<v8::Value> value,
                                                      const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_texture_array_layers = value->Int32Value(context).ToChecked();
    }
}

void GPUSupportedLimitsImpl::GetMaxBindGroups(v8::Local<v8::String> property,
                                              const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(4);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_bind_groups);
}


void GPUSupportedLimitsImpl::SetMaxBindGroups(v8::Local<v8::String> property,
                                              v8::Local<v8::Value> value,
                                              const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_bind_groups = value->Int32Value(context).ToChecked();
    }
}

void GPUSupportedLimitsImpl::GetMaxBindingsPerBindGroup(v8::Local<v8::String> property,
                                                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(1000);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_bindings_per_bind_group);

}


void GPUSupportedLimitsImpl::SetMaxBindingsPerBindGroup(v8::Local<v8::String> property,
                                                        v8::Local<v8::Value> value,
                                                        const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_bindings_per_bind_group = value->Int32Value(context).ToChecked();
    }
}

void
GPUSupportedLimitsImpl::GetMaxDynamicUniformBuffersPerPipelineLayout(v8::Local<v8::String> property,
                                                                     const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(8);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_dynamic_uniform_buffers_per_pipeline_layout);
}

void
GPUSupportedLimitsImpl::SetMaxDynamicUniformBuffersPerPipelineLayout(v8::Local<v8::String> property,
                                                                     v8::Local<v8::Value> value,
                                                                     const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_dynamic_uniform_buffers_per_pipeline_layout = value->Int32Value(
                context).ToChecked();
    }
}

void
GPUSupportedLimitsImpl::GetMaxDynamicStorageBuffersPerPipelineLayout(v8::Local<v8::String> property,
                                                                     const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(4);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_dynamic_storage_buffers_per_pipeline_layout);
}

void
GPUSupportedLimitsImpl::SetMaxDynamicStorageBuffersPerPipelineLayout(v8::Local<v8::String> property,
                                                                     v8::Local<v8::Value> value,
                                                                     const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_dynamic_storage_buffers_per_pipeline_layout = value->Int32Value(
                context).ToChecked();
    }
}

void GPUSupportedLimitsImpl::GetMaxSampledTexturesPerShaderStage(v8::Local<v8::String> property,
                                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(16);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_sampled_textures_per_shader_stage);
}

void GPUSupportedLimitsImpl::SetMaxSampledTexturesPerShaderStage(v8::Local<v8::String> property,
                                                                 v8::Local<v8::Value> value,
                                                                 const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_sampled_textures_per_shader_stage = value->Int32Value(
                context).ToChecked();
    }
}


void GPUSupportedLimitsImpl::GetMaxSamplersPerShaderStage(v8::Local<v8::String> property,
                                                          const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(16);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_samplers_per_shader_stage);

}

void GPUSupportedLimitsImpl::SetMaxSamplersPerShaderStage(v8::Local<v8::String> property,
                                                          v8::Local<v8::Value> value,
                                                          const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_samplers_per_shader_stage = value->Int32Value(context).ToChecked();
    }
}


void GPUSupportedLimitsImpl::GetMaxStorageBuffersPerShaderStage(v8::Local<v8::String> property,
                                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(8);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_storage_buffers_per_shader_stage);
}

void GPUSupportedLimitsImpl::SetMaxStorageBuffersPerShaderStage(v8::Local<v8::String> property,
                                                                v8::Local<v8::Value> value,
                                                                const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_storage_buffers_per_shader_stage = value->Int32Value(context).ToChecked();
    }
}


void GPUSupportedLimitsImpl::GetMaxStorageTexturesPerShaderStage(v8::Local<v8::String> property,
                                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(4);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_storage_textures_per_shader_stage);
}

void GPUSupportedLimitsImpl::SetMaxStorageTexturesPerShaderStage(v8::Local<v8::String> property,
                                                                 v8::Local<v8::Value> value,
                                                                 const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_storage_textures_per_shader_stage = value->Int32Value(
                context).ToChecked();
    }
}


void GPUSupportedLimitsImpl::GetMaxUniformBuffersPerShaderStage(v8::Local<v8::String> property,
                                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(12);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_uniform_buffers_per_shader_stage);
}


void GPUSupportedLimitsImpl::SetMaxUniformBuffersPerShaderStage(v8::Local<v8::String> property,
                                                                v8::Local<v8::Value> value,
                                                                const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_uniform_buffers_per_shader_stage = value->Int32Value(context).ToChecked();
    }
}

void GPUSupportedLimitsImpl::GetMaxUniformBufferBindingSize(v8::Local<v8::String> property,
                                                            const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(64);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_uniform_buffer_binding_size);
}


void GPUSupportedLimitsImpl::SetMaxUniformBufferBindingSize(v8::Local<v8::String> property,
                                                            v8::Local<v8::Value> value,
                                                            const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_uniform_buffer_binding_size = value->Int32Value(context).ToChecked();
    }
}

void GPUSupportedLimitsImpl::GetMaxStorageBufferBindingSize(v8::Local<v8::String> property,
                                                            const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(128);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_storage_buffer_binding_size);
}


void GPUSupportedLimitsImpl::SetMaxStorageBufferBindingSize(v8::Local<v8::String> property,
                                                            v8::Local<v8::Value> value,
                                                            const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_storage_buffer_binding_size = value->Int32Value(context).ToChecked();
    }
}

void GPUSupportedLimitsImpl::GetMaxVertexBuffers(v8::Local<v8::String> property,
                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(8);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_vertex_buffers);
}


void GPUSupportedLimitsImpl::SetMaxVertexBuffers(v8::Local<v8::String> property,
                                                 v8::Local<v8::Value> value,
                                                 const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_vertex_buffers = value->Int32Value(context).ToChecked();
    }
}

void GPUSupportedLimitsImpl::GetMaxBufferSize(v8::Local<v8::String> property,
                                              const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(256);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(v8::Number::New(info.GetIsolate(), (double) limits->max_buffer_size));

}


void GPUSupportedLimitsImpl::SetMaxBufferSize(v8::Local<v8::String> property,
                                              v8::Local<v8::Value> value,
                                              const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsNumber()) {
        ptr->limits_->max_buffer_size = (uint64_t) value->NumberValue(context).ToChecked();
    }
}


void GPUSupportedLimitsImpl::GetMaxVertexAttributes(v8::Local<v8::String> property,
                                                    const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(16);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_vertex_attributes);
}


void GPUSupportedLimitsImpl::SetMaxVertexAttributes(v8::Local<v8::String> property,
                                                    v8::Local<v8::Value> value,
                                                    const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_vertex_attributes = value->Int32Value(context).ToChecked();
    }
}


void GPUSupportedLimitsImpl::GetMaxVertexBufferArrayStride(v8::Local<v8::String> property,
                                                           const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(2048);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_vertex_buffer_array_stride);

}


void GPUSupportedLimitsImpl::SetMaxVertexBufferArrayStride(v8::Local<v8::String> property,
                                                           v8::Local<v8::Value> value,
                                                           const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_vertex_buffer_array_stride = value->Int32Value(context).ToChecked();
    }
}


void GPUSupportedLimitsImpl::GetMinUniformBufferOffsetAlignment(v8::Local<v8::String> property,
                                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(256);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->min_uniform_buffer_offset_alignment);

}


void GPUSupportedLimitsImpl::SetMinUniformBufferOffsetAlignment(v8::Local<v8::String> property,
                                                                v8::Local<v8::Value> value,
                                                                const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->min_uniform_buffer_offset_alignment = value->Int32Value(context).ToChecked();
    }
}


void GPUSupportedLimitsImpl::GetMinStorageBufferOffsetAlignment(v8::Local<v8::String> property,
                                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(256);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->min_storage_buffer_offset_alignment);

}


void GPUSupportedLimitsImpl::SetMinStorageBufferOffsetAlignment(v8::Local<v8::String> property,
                                                                v8::Local<v8::Value> value,
                                                                const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->min_storage_buffer_offset_alignment = value->Int32Value(context).ToChecked();
    }
}


void GPUSupportedLimitsImpl::GetMaxInterStageShaderComponents(v8::Local<v8::String> property,
                                                              const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(60);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_inter_stage_shader_components);

}


void GPUSupportedLimitsImpl::SetMaxInterStageShaderComponents(v8::Local<v8::String> property,
                                                              v8::Local<v8::Value> value,
                                                              const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_inter_stage_shader_components = value->Int32Value(context).ToChecked();
    }
}


void GPUSupportedLimitsImpl::GetMaxColorAttachments(v8::Local<v8::String> property,
                                                    const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(8);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_color_attachments);

}


void GPUSupportedLimitsImpl::SetMaxColorAttachments(v8::Local<v8::String> property,
                                                    v8::Local<v8::Value> value,
                                                    const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_color_attachments = value->Int32Value(context).ToChecked();
    }
}


void GPUSupportedLimitsImpl::GetMaxColorAttachmentBytesPerSample(v8::Local<v8::String> property,
                                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(32);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_color_attachment_bytes_per_sample);

}

void GPUSupportedLimitsImpl::SetMaxColorAttachmentBytesPerSample(v8::Local<v8::String> property,
                                                                 v8::Local<v8::Value> value,
                                                                 const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_color_attachment_bytes_per_sample = value->Int32Value(
                context).ToChecked();
    }
}


void GPUSupportedLimitsImpl::GetMaxComputeWorkgroupStorageSize(v8::Local<v8::String> property,
                                                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(16384);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_compute_workgroup_storage_size);
}


void GPUSupportedLimitsImpl::SetMaxComputeWorkgroupStorageSize(v8::Local<v8::String> property,
                                                               v8::Local<v8::Value> value,
                                                               const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_compute_workgroup_storage_size = value->Int32Value(context).ToChecked();
    }
}


void GPUSupportedLimitsImpl::GetMaxComputeInvocationsPerWorkgroup(v8::Local<v8::String> property,
                                                                  const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(256);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_compute_invocations_per_workgroup);
}


void GPUSupportedLimitsImpl::SetMaxComputeInvocationsPerWorkgroup(v8::Local<v8::String> property,
                                                                  v8::Local<v8::Value> value,
                                                                  const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_compute_invocations_per_workgroup = value->Int32Value(
                context).ToChecked();
    }
}

void GPUSupportedLimitsImpl::GetMaxComputeWorkgroupSizeX(v8::Local<v8::String> property,
                                                         const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(256);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_compute_workgroup_size_x);
}


void GPUSupportedLimitsImpl::SetMaxComputeWorkgroupSizeX(v8::Local<v8::String> property,
                                                         v8::Local<v8::Value> value,
                                                         const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_compute_workgroup_size_x = value->Int32Value(context).ToChecked();
    }
}


void GPUSupportedLimitsImpl::GetMaxComputeWorkgroupSizeY(v8::Local<v8::String> property,
                                                         const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(256);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_compute_workgroup_size_y);
}


void GPUSupportedLimitsImpl::SetMaxComputeWorkgroupSizeY(v8::Local<v8::String> property,
                                                         v8::Local<v8::Value> value,
                                                         const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_compute_workgroup_size_y = value->Int32Value(context).ToChecked();
    }
}


void GPUSupportedLimitsImpl::GetMaxComputeWorkgroupSizeZ(v8::Local<v8::String> property,
                                                         const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(64);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_compute_workgroup_size_z);
}


void GPUSupportedLimitsImpl::SetMaxComputeWorkgroupSizeZ(v8::Local<v8::String> property,
                                                         v8::Local<v8::Value> value,
                                                         const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_compute_workgroup_size_z = value->Int32Value(context).ToChecked();
    }
}

void GPUSupportedLimitsImpl::GetMaxComputeWorkgroupsPerDimension(v8::Local<v8::String> property,
                                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(65535);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_compute_workgroups_per_dimension);
}


void GPUSupportedLimitsImpl::SetMaxComputeWorkgroupsPerDimension(v8::Local<v8::String> property,
                                                                 v8::Local<v8::Value> value,
                                                                 const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_compute_workgroups_per_dimension = value->Int32Value(context).ToChecked();
    }
}

void GPUSupportedLimitsImpl::GetMinSubgroupSize(v8::Local<v8::String> property,
                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(0);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->min_subgroup_size);
}


void GPUSupportedLimitsImpl::SetMinSubgroupSize(v8::Local<v8::String> property,
                                                v8::Local<v8::Value> value,
                                                const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->min_subgroup_size = value->Int32Value(context).ToChecked();
    }
}

void GPUSupportedLimitsImpl::GetMaxSubgroupSize(v8::Local<v8::String> property,
                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(0);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_subgroup_size);
}


void GPUSupportedLimitsImpl::SetMaxSubgroupSize(v8::Local<v8::String> property,
                                                v8::Local<v8::Value> value,
                                                const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_subgroup_size = value->Int32Value(context).ToChecked();
    }
}


void GPUSupportedLimitsImpl::GetMaxPushConstantSize(v8::Local<v8::String> property,
                                                    const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(0);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_push_constant_size);
}


void GPUSupportedLimitsImpl::SetMaxPushConstantSize(v8::Local<v8::String> property,
                                                    v8::Local<v8::Value> value,
                                                    const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_push_constant_size = value->Int32Value(context).ToChecked();
    }
}

void GPUSupportedLimitsImpl::GetMaxNonSamplerBindings(v8::Local<v8::String> property,
                                                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        // return default ??
        info.GetReturnValue().Set(1000000);
        return;
    }

    auto limits = ptr->GetLimits();

    info.GetReturnValue().Set(limits->max_non_sampler_bindings);

}

void GPUSupportedLimitsImpl::SetMaxNonSamplerBindings(v8::Local<v8::String> property,
                                                      v8::Local<v8::Value> value,
                                                      const v8::PropertyCallbackInfo<void> &info) {
    GPUSupportedLimitsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (value->IsInt32()) {
        ptr->limits_->max_non_sampler_bindings = value->Int32Value(context).ToChecked();
    }
}

