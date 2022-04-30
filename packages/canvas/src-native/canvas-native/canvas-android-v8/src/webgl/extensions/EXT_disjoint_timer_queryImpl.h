//
// Created by Osei Fortune on 28/04/2022.
//

#pragma once

#include "../../Common.h"
#include "../../Caches.h"
#include "../../Helpers.h"

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
private:
    rust::Box<EXT_disjoint_timer_query> query_;
    static EXT_disjoint_timer_queryImpl *GetPointer(v8::Local<v8::Object> object);
};
