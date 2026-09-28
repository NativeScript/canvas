//
// Created by Osei Fortune on 01/07/2024.
//

#ifndef CANVAS_ANDROID_GPURENDERPASSENCODERIMPL_H
#define CANVAS_ANDROID_GPURENDERPASSENCODERIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"
#include "GPUUtils.h"
#include "ArcHandle.h"

class GPURenderPassEncoderImpl : ObjectWrapperImpl {
public:
    static v8::CFunction fast_draw_;
    static v8::CFunction fast_draw_indexed_;
    static v8::CFunction fast_set_pipeline_;
    static v8::CFunction fast_set_vertex_buffer_;
    static v8::CFunction fast_set_bind_group_[2];
    static v8::CFunction fast_set_scissor_rect_;
    static v8::CFunction fast_set_viewport_;
    static v8::CFunction fast_set_stencil_reference_;
    static v8::CFunction fast_begin_occlusion_query_;
    static v8::CFunction fast_end_occlusion_query_;
    static v8::CFunction fast_draw_indirect_;
    static v8::CFunction fast_draw_indexed_indirect_;
    static v8::CFunction fast_set_index_buffer_;

    // indexFormat: 0 = uint16, 1 = uint32.
    static void FastSetIndexBuffer(v8::Local<v8::Object> receiver_obj,
                                   v8::Local<v8::Object> buffer_obj, uint32_t indexFormat,
                                   double offset, double size);

    static void FastSetScissorRect(v8::Local<v8::Object> receiver_obj, uint32_t x, uint32_t y,
                                   uint32_t width, uint32_t height);

    static void FastSetViewport(v8::Local<v8::Object> receiver_obj, double x, double y,
                                double width, double height, double minDepth, double maxDepth);

    static void FastSetStencilReference(v8::Local<v8::Object> receiver_obj, uint32_t reference);

    static void FastBeginOcclusionQuery(v8::Local<v8::Object> receiver_obj, uint32_t queryIndex);

    static void FastEndOcclusionQuery(v8::Local<v8::Object> receiver_obj);

    static void FastDrawIndirect(v8::Local<v8::Object> receiver_obj,
                                 v8::Local<v8::Object> buffer_obj, double offset);

    static void FastDrawIndexedIndirect(v8::Local<v8::Object> receiver_obj,
                                        v8::Local<v8::Object> buffer_obj, double offset);

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

    explicit GPURenderPassEncoderImpl(const CanvasGPURenderPassEncoder *pass);

    // Deterministic dispose, called from Destroy() at pass.end(). See ArcHandle.h.
    void Release() { pass_.reset(); }

    const CanvasGPURenderPassEncoder *GetPass();

    static void Destroy(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPURenderPassEncoderImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, GPURenderPassEncoderImpl *encoder) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPURenderPassEncoderImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(encoder, NativeType::GPURenderPassEncoder);
        object->SetAlignedPointerInInternalField(0, encoder, ObjectWrapperImpl::kInternalFieldTag);
        encoder->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void GetLabel(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void BeginOcclusionQuery(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Draw(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawIndexed(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawIndexedIndirect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawIndirect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void MultiDrawIndexedIndirect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void MultiDrawIndirect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void End(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void EndOcclusionQuery(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ExecuteBundles(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void InsertDebugMarker(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void PopDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void PushDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetBindGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetBlendConstant(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetIndexBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetPipeline(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetScissorRect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetStencilReference(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetVertexBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetViewport(const v8::FunctionCallbackInfo<v8::Value> &args);


private:
    ArcHandle<CanvasGPURenderPassEncoder, canvas_native_webgpu_render_pass_encoder_release> pass_;
};


#endif //CANVAS_ANDROID_GPURENDERPASSENCODERIMPL_H
