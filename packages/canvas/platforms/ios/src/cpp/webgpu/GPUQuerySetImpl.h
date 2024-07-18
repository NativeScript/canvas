//
// Created by Osei Fortune on 23/06/2024.
//

#ifndef CANVAS_ANDROID_GPUQUERYSETIMPL_H
#define CANVAS_ANDROID_GPUQUERYSETIMPL_H

#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPUQuerySetImpl : ObjectWrapperImpl {
public:
    GPUQuerySetImpl(const CanvasGPUQuerySet *querySet);

    ~GPUQuerySetImpl() {
        canvas_native_webgpu_query_set_release(this->GetQuerySet());
    }

    const CanvasGPUQuerySet *GetQuerySet();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPUQuerySetImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, GPUQuerySetImpl *querySet) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPUQuerySetImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::GPUQuerySet);
        object->SetAlignedPointerInInternalField(0, querySet);
        querySet->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void GetType(v8::Local<v8::Name> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetCount(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetLabel(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Destroy(const v8::FunctionCallbackInfo<v8::Value> &args);


private:
    const CanvasGPUQuerySet *querySet_;
};


#endif //CANVAS_ANDROID_GPUQUERYSETIMPL_H
