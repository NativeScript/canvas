//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once
#include "../Common.h"
#include "../Caches.h"
#include "../Helpers.h"

class WebGLShaderPrecisionFormatImpl {
public:
    WebGLShaderPrecisionFormatImpl(rust::Box<WebGLShaderPrecisionFormat> shader);

    static void Init(v8::Isolate *isolate);

    static void Create(const v8::FunctionCallbackInfo <v8::Value> &args);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, rust::Box<WebGLShaderPrecisionFormat> shader);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void GetRangeMin(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetRangeMax(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetPrecision(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    template<typename T>
    static void AddWeakListener(v8::Isolate *isolate, const v8::Local<v8::Object> &object, T *data){
        auto ext = v8::External::New(isolate, data);
        object->SetInternalField(0, ext);
        auto persistent = new v8::Persistent<v8::Object>(isolate, object);
        auto entry = new ObjectCacheEntry(static_cast<void *>(data), persistent);
        auto callback = [](const v8::WeakCallbackInfo<ObjectCacheEntry> &cacheEntry) {
            auto value = cacheEntry.GetParameter();
            auto ptr = static_cast<T *>(value->data);
            if (ptr != nullptr) {
                delete ptr;
            }
            auto persistent_ptr = value->object;
            if (persistent_ptr != nullptr) {
                if (!persistent_ptr->IsEmpty()) {
                    persistent_ptr->Reset();
                }
            }
            delete value;
        };
        persistent->SetWeak(entry, callback, v8::WeakCallbackType::kFinalizer);
    }

private:
    rust::Box<WebGLShaderPrecisionFormat> shader_;

    static WebGLShaderPrecisionFormatImpl *GetPointer(const v8::Local<v8::Object>& object);
};

