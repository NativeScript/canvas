//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "../../Common.h"
#include "../../Caches.h"
#include "../../Helpers.h"

class ANGLE_instanced_arraysImpl {
public:
    ANGLE_instanced_arraysImpl(rust::Box<ANGLE_instanced_arrays> arrays);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, rust::Box<ANGLE_instanced_arrays> arrays);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void DrawArraysInstancedANGLE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawElementsInstancedANGLE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void VertexAttribDivisorANGLE(const v8::FunctionCallbackInfo<v8::Value> &args);

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
    rust::Box<ANGLE_instanced_arrays> arrays_;

    static ANGLE_instanced_arraysImpl *GetPointer(v8::Local<v8::Object> object);
};

