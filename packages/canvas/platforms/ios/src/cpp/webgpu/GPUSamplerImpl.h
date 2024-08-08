//
// Created by Osei Fortune on 17/07/2024.
//

#ifndef CANVAS_ANDROID_GPUSAMPLERIMPL_H
#define CANVAS_ANDROID_GPUSAMPLERIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPUSamplerImpl: ObjectWrapperImpl {
public:
    explicit GPUSamplerImpl(const CanvasGPUSampler *sampler);

    ~GPUSamplerImpl() {
        canvas_native_webgpu_sampler_release(this->sampler_);
    }

    const CanvasGPUSampler *GetSampler();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUSamplerImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPUSamplerImpl *sampler) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUSamplerImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(sampler, NativeType::GPUSampler);
        object->SetAlignedPointerInInternalField(0, sampler);
        sampler->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }


private:
    const CanvasGPUSampler *sampler_;
};


#endif //CANVAS_ANDROID_GPUSAMPLERIMPL_H
