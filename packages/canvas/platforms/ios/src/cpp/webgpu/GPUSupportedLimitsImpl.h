//
// Created by Osei Fortune on 17/06/2024.
//

#ifndef CANVAS_ANDROID_GPUSUPPORTEDLIMITSIMPL_H
#define CANVAS_ANDROID_GPUSUPPORTEDLIMITSIMPL_H

#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPUSupportedLimitsImpl : public ObjectWrapperImpl {
public:
    GPUSupportedLimitsImpl(CanvasGPUSupportedLimits *limits);

    ~GPUSupportedLimitsImpl() {
        canvas_native_webgpu_limits_release(this->limits_);
    }

    CanvasGPUSupportedLimits *GetLimits();

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPUSupportedLimitsImpl *limits) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUSupportedLimitsImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPUSupportedLimits);
        object->SetAlignedPointerInInternalField(0, limits);
        limits->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUSupportedLimitsImpl *GetPointer(v8::Local<v8::Object> object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void Ctor(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetMaxTextureDimension1D(v8::Local<v8::String> property,
                                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxTextureDimension1D(v8::Local<v8::String> property,
                                         v8::Local<v8::Value> value,
                                         const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxTextureDimension2D(v8::Local<v8::String> property,
                                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxTextureDimension2D(v8::Local<v8::String> property,
                                         v8::Local<v8::Value> value,
                                         const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxTextureDimension3D(v8::Local<v8::String> property,
                                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxTextureDimension3D(v8::Local<v8::String> property,
                                         v8::Local<v8::Value> value,
                                         const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxTextureArrayLayers(v8::Local<v8::String> property,
                                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxTextureArrayLayers(v8::Local<v8::String> property,
                                         v8::Local<v8::Value> value,
                                         const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxBindGroups(v8::Local<v8::String> property,
                                 const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxBindGroups(v8::Local<v8::String> property,
                                 v8::Local<v8::Value> value,
                                 const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxBindingsPerBindGroup(v8::Local<v8::String> property,
                                           const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxBindingsPerBindGroup(v8::Local<v8::String> property,
                                           v8::Local<v8::Value> value,
                                           const v8::PropertyCallbackInfo<void> &info);


    static void GetMaxDynamicUniformBuffersPerPipelineLayout(v8::Local<v8::String> property,
                                                             const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxDynamicUniformBuffersPerPipelineLayout(v8::Local<v8::String> property,
                                                             v8::Local<v8::Value> value,
                                                             const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxDynamicStorageBuffersPerPipelineLayout(v8::Local<v8::String> property,
                                                             const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxDynamicStorageBuffersPerPipelineLayout(v8::Local<v8::String> property,
                                                             v8::Local<v8::Value> value,
                                                             const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxSampledTexturesPerShaderStage(v8::Local<v8::String> property,
                                                    const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxSampledTexturesPerShaderStage(v8::Local<v8::String> property,
                                                    v8::Local<v8::Value> value,
                                                    const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxSamplersPerShaderStage(v8::Local<v8::String> property,
                                             const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxSamplersPerShaderStage(v8::Local<v8::String> property,
                                             v8::Local<v8::Value> value,
                                             const v8::PropertyCallbackInfo<void> &info);


    static void GetMaxStorageBuffersPerShaderStage(v8::Local<v8::String> property,
                                                   const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxStorageBuffersPerShaderStage(v8::Local<v8::String> property,
                                                   v8::Local<v8::Value> value,
                                                   const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxStorageTexturesPerShaderStage(v8::Local<v8::String> property,
                                                    const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxStorageTexturesPerShaderStage(v8::Local<v8::String> property,
                                                    v8::Local<v8::Value> value,
                                                    const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxUniformBuffersPerShaderStage(v8::Local<v8::String> property,
                                                   const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxUniformBuffersPerShaderStage(v8::Local<v8::String> property,
                                                   v8::Local<v8::Value> value,
                                                   const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxUniformBufferBindingSize(v8::Local<v8::String> property,
                                               const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxUniformBufferBindingSize(v8::Local<v8::String> property,
                                               v8::Local<v8::Value> value,
                                               const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxStorageBufferBindingSize(v8::Local<v8::String> property,
                                               const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxStorageBufferBindingSize(v8::Local<v8::String> property,
                                               v8::Local<v8::Value> value,
                                               const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxVertexBuffers(v8::Local<v8::String> property,
                                    const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxVertexBuffers(v8::Local<v8::String> property,
                                    v8::Local<v8::Value> value,
                                    const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxBufferSize(v8::Local<v8::String> property,
                                 const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxBufferSize(v8::Local<v8::String> property,
                                 v8::Local<v8::Value> value,
                                 const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxVertexAttributes(v8::Local<v8::String> property,
                                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxVertexAttributes(v8::Local<v8::String> property,
                                       v8::Local<v8::Value> value,
                                       const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxVertexBufferArrayStride(v8::Local<v8::String> property,
                                              const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxVertexBufferArrayStride(v8::Local<v8::String> property,
                                              v8::Local<v8::Value> value,
                                              const v8::PropertyCallbackInfo<void> &info);

    static void GetMinUniformBufferOffsetAlignment(v8::Local<v8::String> property,
                                                   const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMinUniformBufferOffsetAlignment(v8::Local<v8::String> property,
                                                   v8::Local<v8::Value> value,
                                                   const v8::PropertyCallbackInfo<void> &info);

    static void GetMinStorageBufferOffsetAlignment(v8::Local<v8::String> property,
                                                   const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMinStorageBufferOffsetAlignment(v8::Local<v8::String> property,
                                                   v8::Local<v8::Value> value,
                                                   const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxInterStageShaderComponents(v8::Local<v8::String> property,
                                                 const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxInterStageShaderComponents(v8::Local<v8::String> property,
                                                 v8::Local<v8::Value> value,
                                                 const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxColorAttachments(v8::Local<v8::String> property,
                                       const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxColorAttachments(v8::Local<v8::String> property,
                                       v8::Local<v8::Value> value,
                                       const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxColorAttachmentBytesPerSample(v8::Local<v8::String> property,
                                                    const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxColorAttachmentBytesPerSample(v8::Local<v8::String> property,
                                                    v8::Local<v8::Value> value,
                                                    const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxComputeWorkgroupStorageSize(v8::Local<v8::String> property,
                                                  const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxComputeWorkgroupStorageSize(v8::Local<v8::String> property,
                                                  v8::Local<v8::Value> value,
                                                  const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxComputeInvocationsPerWorkgroup(v8::Local<v8::String> property,
                                                     const v8::PropertyCallbackInfo<v8::Value> &info);


    static void SetMaxComputeInvocationsPerWorkgroup(v8::Local<v8::String> property,
                                                     v8::Local<v8::Value> value,
                                                     const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxComputeWorkgroupSizeX(v8::Local<v8::String> property,
                                            const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxComputeWorkgroupSizeX(v8::Local<v8::String> property,
                                            v8::Local<v8::Value> value,
                                            const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxComputeWorkgroupSizeY(v8::Local<v8::String> property,
                                            const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxComputeWorkgroupSizeY(v8::Local<v8::String> property,
                                            v8::Local<v8::Value> value,
                                            const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxComputeWorkgroupSizeZ(v8::Local<v8::String> property,
                                            const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxComputeWorkgroupSizeZ(v8::Local<v8::String> property,
                                            v8::Local<v8::Value> value,
                                            const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxComputeWorkgroupsPerDimension(v8::Local<v8::String> property,
                                                    const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxComputeWorkgroupsPerDimension(v8::Local<v8::String> property,
                                                    v8::Local<v8::Value> value,
                                                    const v8::PropertyCallbackInfo<void> &info);

    static void GetMinSubgroupSize(v8::Local<v8::String> property,
                                   const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMinSubgroupSize(v8::Local<v8::String> property,
                                   v8::Local<v8::Value> value,
                                   const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxSubgroupSize(v8::Local<v8::String> property,
                                   const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMaxSubgroupSize(v8::Local<v8::String> property,
                                   v8::Local<v8::Value> value,
                                   const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxPushConstantSize(v8::Local<v8::String> property,
                                       const v8::PropertyCallbackInfo<v8::Value> &info);


    static void SetMaxPushConstantSize(v8::Local<v8::String> property,
                                       v8::Local<v8::Value> value,
                                       const v8::PropertyCallbackInfo<void> &info);

    static void GetMaxNonSamplerBindings(v8::Local<v8::String> property,
                                         const v8::PropertyCallbackInfo<v8::Value> &info);


    static void SetMaxNonSamplerBindings(v8::Local<v8::String> property,
                                         v8::Local<v8::Value> value,
                                         const v8::PropertyCallbackInfo<void> &info);

private:
    CanvasGPUSupportedLimits *limits_;
};


#endif //CANVAS_ANDROID_GPUSUPPORTEDLIMITSIMPL_H
