//
// Created by Osei Fortune on 23/06/2024.
//

#ifndef CANVAS_ANDROID_GPUCOMPUTEPASSENCODERIMPL_H
#define CANVAS_ANDROID_GPUCOMPUTEPASSENCODERIMPL_H

#include "Helpers.h"
#include "ObjectWrapperImpl.h"
#include "ArcHandle.h"

class GPUComputePassEncoderImpl : ObjectWrapperImpl {
public:
    static v8::CFunction fast_dispatch_workgroups_;
    static v8::CFunction fast_dispatch_workgroups_indirect_;
    static v8::CFunction fast_set_pipeline_;
    static v8::CFunction fast_set_bind_group_[2];

    static void FastDispatchWorkgroups(v8::Local<v8::Object> receiver_obj, uint32_t x, uint32_t y,
                                       uint32_t z);

    static void FastDispatchWorkgroupsIndirect(v8::Local<v8::Object> receiver_obj,
                                               v8::Local<v8::Object> buffer_obj, double offset);

    static void FastSetPipeline(v8::Local<v8::Object> receiver_obj,
                                v8::Local<v8::Object> pipeline_obj);

    static void FastSetBindGroupNoOffsets(v8::Local<v8::Object> receiver_obj, uint32_t index,
                                          v8::Local<v8::Object> bind_group_obj);

    static void FastSetBindGroup(v8::Local<v8::Object> receiver_obj, uint32_t index,
                                 v8::Local<v8::Object> bind_group_obj,
                                 v8::Local<v8::Value> dynamic_offsets, double start,
                                 double length);

    explicit GPUComputePassEncoderImpl(const CanvasGPUComputePassEncoder *computePass);

    // Deterministic dispose, called from Destroy() at pass.end(). See ArcHandle.h.
    void Release() { computePass_.reset(); }

    const CanvasGPUComputePassEncoder *GetComputePass();

    static void Destroy(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUComputePassEncoderImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, GPUComputePassEncoderImpl *pass) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUComputePassEncoderImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(pass, NativeType::GPUComputePass);
        object->SetAlignedPointerInInternalField(0, pass, ObjectWrapperImpl::kInternalFieldTag);
        pass->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void GetLabel(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void DispatchWorkgroups(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DispatchWorkgroupsIndirect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void End(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void InsertDebugMarker(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void PopDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void PushDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetBindGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetPipeline(const v8::FunctionCallbackInfo<v8::Value> &args);



private:
    ArcHandle<CanvasGPUComputePassEncoder, canvas_native_webgpu_compute_pass_encoder_release> computePass_;
};


#endif //CANVAS_ANDROID_GPUCOMPUTEPASSENCODERIMPL_H
