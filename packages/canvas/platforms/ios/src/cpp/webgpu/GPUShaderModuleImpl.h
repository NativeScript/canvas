//
// Created by Osei Fortune on 23/06/2024.
//

#ifndef CANVAS_ANDROID_GPUSHADERMODULEIMPL_H
#define CANVAS_ANDROID_GPUSHADERMODULEIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPUShaderModuleImpl : ObjectWrapperImpl {
public:
    explicit GPUShaderModuleImpl(const CanvasGPUShaderModule *shaderModule);

    ~GPUShaderModuleImpl() {
        canvas_native_webgpu_shader_module_release(this->GetShaderModule());
    }

    const CanvasGPUShaderModule *GetShaderModule();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUShaderModuleImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, GPUShaderModuleImpl *shaderModule) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUShaderModuleImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(shaderModule, NativeType::GPUShaderModule);
        object->SetAlignedPointerInInternalField(0, shaderModule);
        shaderModule->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

private:
    const CanvasGPUShaderModule *shaderModule_;
};


#endif //CANVAS_ANDROID_GPUSHADERMODULEIMPL_H
