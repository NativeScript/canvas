//
// Created by Osei Fortune on 28/04/2022.
//

#pragma once

#include "../../Common.h"
#include "../../Caches.h"
#include "../../Helpers.h"
#include "../../webgl2/WebGLQuery.h"

class EXT_disjoint_timer_queryImpl {
public:
    EXT_disjoint_timer_queryImpl(rust::Box<EXT_disjoint_timer_query> query);

    static v8::Local<v8::Object> NewInstance(v8::Isolate* isolate, rust::Box<EXT_disjoint_timer_query> query);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void CreateQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DeleteQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BeginQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void EndQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void QueryCounterExt(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetQueryObjectExt(const v8::FunctionCallbackInfo<v8::Value> &args);

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
    rust::Box<EXT_disjoint_timer_query> query_;
    static EXT_disjoint_timer_queryImpl *GetPointer(v8::Local<v8::Object> object);
};
