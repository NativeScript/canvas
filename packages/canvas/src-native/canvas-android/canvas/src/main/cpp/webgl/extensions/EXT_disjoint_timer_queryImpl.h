//
// Created by Osei Fortune on 28/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include "Helpers.h"
#include "Common.h"
#include "webgl2/WebGLQuery.h"
#include <vector>

using namespace org::nativescript::canvas;

class EXT_disjoint_timer_queryImpl {
public:
    EXT_disjoint_timer_queryImpl(rust::Box<EXT_disjoint_timer_query> query);

    EXT_disjoint_timer_query &GetQuery();

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, EXT_disjoint_timer_queryImpl *query) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = EXT_disjoint_timer_queryImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::EXT_disjoint_timer_query);
        auto ext = v8::External::New(isolate, query);
        object->SetInternalField(0, ext);
        object->Set(context, ConvertToV8String(isolate, "ext_name"),
                    ConvertToV8String(isolate, "EXT_disjoint_timer_query"));
        return scope.Escape(object);
    }

    static EXT_disjoint_timer_queryImpl *GetPointer(const v8::Local<v8::Object> &object);

    static void CreateQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DeleteQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BeginQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void EndQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void QueryCounterExt(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetQueryExt(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetQueryObjectExt(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetQueryParameterExt(const v8::FunctionCallbackInfo<v8::Value> &args);


private:
    rust::Box<EXT_disjoint_timer_query> query_;
};
