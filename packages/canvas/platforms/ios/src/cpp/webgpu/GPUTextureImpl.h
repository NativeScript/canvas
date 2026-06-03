//
// Created by Osei Fortune on 21/06/2024.
//

#ifndef CANVAS_ANDROID_GPUTEXTUREIMPL_H
#define CANVAS_ANDROID_GPUTEXTUREIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"
#include "ArcHandle.h"

class GPUTextureImpl : ObjectWrapperImpl {
public:
    explicit GPUTextureImpl(const CanvasGPUTexture *texture);

    ~GPUTextureImpl() = default;

    const CanvasGPUTexture *GetTexture();

    // Drops our Arc reference to the texture (canvas_native_webgpu_texture_release
    // via the ArcHandle deleter). This is NOT destroy(): it does not free the GPU
    // texture, it only releases the wrapper's handle. Used to deterministically free
    // the per-frame swapchain GPUTexture returned by getCurrentTexture() at present,
    // whose only other free path is the starved GC finalizer. See ReleaseHandle.
    void Release() { texture_.reset(); }

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUTextureImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPUTextureImpl *texture) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUTextureImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(texture, NativeType::GPUTexture);
        object->SetAlignedPointerInInternalField(0, texture);
        texture->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void GetLabel(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);


    static void GetWidth(v8::Local<v8::Name> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetHeight(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);


    static void GetDepthOrArrayLayers(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetDimension(v8::Local<v8::Name> name,
                                      const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetFormat(v8::Local<v8::Name> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetMipLevelCount(v8::Local<v8::Name> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetSampleCount(v8::Local<v8::Name> name,
                                 const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetUsage(v8::Local<v8::Name> name,
                               const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Destroy(const v8::FunctionCallbackInfo<v8::Value> &args);

    // JS-exposed as __releaseHandle: drops the wrapper's Arc handle (see Release).
    // Distinct from destroy() so the swapchain texture's GPU memory is never freed.
    static void ReleaseHandle(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateView(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    ArcHandle<CanvasGPUTexture, canvas_native_webgpu_texture_release> texture_;
};


#endif //CANVAS_ANDROID_GPUTEXTUREIMPL_H
