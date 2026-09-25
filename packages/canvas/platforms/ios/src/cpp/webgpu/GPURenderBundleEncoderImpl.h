//
// Created by Osei Fortune on 17/07/2024.
//

#ifndef CANVAS_ANDROID_GPURENDERBUNDLEENCODERIMPL_H
#define CANVAS_ANDROID_GPURENDERBUNDLEENCODERIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"
#include "ArcHandle.h"

class GPURenderBundleEncoderImpl : ObjectWrapperImpl {

public:
    static v8::CFunction fast_draw_;
    static v8::CFunction fast_draw_indexed_;
    static v8::CFunction fast_set_pipeline_;
    static v8::CFunction fast_set_vertex_buffer_;
    static v8::CFunction fast_set_bind_group_[2];
    static v8::CFunction fast_set_index_buffer_;

    // indexFormat: 0 = uint16, 1 = uint32.
    static void FastSetIndexBuffer(v8::Local<v8::Object> receiver_obj,
                                   v8::Local<v8::Object> buffer_obj, uint32_t indexFormat,
                                   double offset, double size);

    static void FastDraw(v8::Local<v8::Object> receiver_obj, uint32_t vertexCount,
                         uint32_t instanceCount, uint32_t firstVertex, uint32_t firstInstance);

    static void FastDrawIndexed(v8::Local<v8::Object> receiver_obj, uint32_t indexCount,
                                uint32_t instanceCount, uint32_t firstIndex, int32_t baseVertex,
                                uint32_t firstInstance);

    static void FastSetPipeline(v8::Local<v8::Object> receiver_obj,
                                v8::Local<v8::Object> pipeline_obj);

    static void FastSetVertexBuffer(v8::Local<v8::Object> receiver_obj, uint32_t slot,
                                    v8::Local<v8::Object> buffer_obj, double offset, double size);

    static void FastSetBindGroupNoOffsets(v8::Local<v8::Object> receiver_obj, uint32_t index,
                                          v8::Local<v8::Object> bind_group_obj);

    static void FastSetBindGroup(v8::Local<v8::Object> receiver_obj, uint32_t index,
                                 v8::Local<v8::Object> bind_group_obj,
                                 v8::Local<v8::Value> dynamic_offsets, double start,
                                 double length);

    explicit GPURenderBundleEncoderImpl(const CanvasGPURenderBundleEncoder *encoder);

    ~GPURenderBundleEncoderImpl() = default;

    const CanvasGPURenderBundleEncoder *GetEncoder();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPURenderBundleEncoderImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, GPURenderBundleEncoderImpl *encoder) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPURenderBundleEncoderImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(encoder, NativeType::GPURenderBundleEncoder);
        object->SetAlignedPointerInInternalField(0, encoder, ObjectWrapperImpl::kInternalFieldTag);
        encoder->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void GetLabel(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Draw(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawIndexed(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawIndexedIndirect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawIndirect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Finish(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void InsertDebugMarker(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void PopDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void PushDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetBindGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetIndexBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetPipeline(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetVertexBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);


private:
    ArcHandle<CanvasGPURenderBundleEncoder, canvas_native_webgpu_render_bundle_encoder_release> encoder_;
};


#endif //CANVAS_ANDROID_GPURENDERBUNDLEENCODERIMPL_H
