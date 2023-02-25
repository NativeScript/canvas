//
// Created by Osei Fortune on 18/04/2022.
//

#pragma once

#include "../Common.h"
#include "../Helpers.h"
#include "../Caches.h"

class TextMetricsImpl {
public:
    TextMetricsImpl(rust::Box <TextMetrics> metrics);

    static void Init(v8::Isolate *isolate);

    static void Create(const v8::FunctionCallbackInfo<v8::Value> &args);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, rust::Box <TextMetrics> metrics);

    static void GetWidth(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void ActualBoundingBoxLeft(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void ActualBoundingBoxRight(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void ActualBoundingBoxAscent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void ActualBoundingBoxDescent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void FontBoundingBoxAscent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void FontBoundingBoxDescent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void EmHeightAscent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void EmHeightDescent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void HangingBaseline(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void AlphabeticBaseline(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void IdeographicBaseline(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

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
    rust::Box <TextMetrics> metrics_;

    static TextMetricsImpl *GetPointer(const v8::Local<v8::Object>& object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);
};

