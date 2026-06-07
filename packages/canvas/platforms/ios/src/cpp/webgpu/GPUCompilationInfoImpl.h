//
// Created by Osei Fortune on 14/06/2025.
//

#ifndef CANVAS_ANDROID_GPUCOMPILATIONINFOIMPL_H
#define CANVAS_ANDROID_GPUCOMPILATIONINFOIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"
#include "ArcHandle.h"

class GPUCompilationInfoImpl : ObjectWrapperImpl {
public:
    explicit GPUCompilationInfoImpl(CanvasGPUCompilationInfo *info);

    ~GPUCompilationInfoImpl() = default;

    CanvasGPUCompilationInfo* GetCompilationInfo();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUCompilationInfoImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, GPUCompilationInfoImpl *info) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUCompilationInfoImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(info, NativeType::GPUCompilationInfo);
        object->SetAlignedPointerInInternalField(0, info);
        info->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void GetMessages(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);


private:
    MutArcHandle<CanvasGPUCompilationInfo, canvas_native_webgpu_compilation_info_release> info_;
};


#endif //CANVAS_ANDROID_GPUCOMPILATIONINFOIMPL_H
