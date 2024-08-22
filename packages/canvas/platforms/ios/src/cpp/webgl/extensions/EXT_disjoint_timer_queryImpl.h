//
// Created by Osei Fortune on 28/04/2022.
//

#pragma once

#include "Helpers.h"
#include "Common.h"
#include "WebGLQuery.h"
#include <vector>
#include "ObjectWrapperImpl.h"
class EXT_disjoint_timer_queryImpl: ObjectWrapperImpl {
public:
    EXT_disjoint_timer_queryImpl(EXT_disjoint_timer_query *query);

    ~EXT_disjoint_timer_queryImpl() {
        canvas_native_webgl_EXT_disjoint_timer_query_destroy(this->GetQuery());
        this->query_ = nullptr;
    }

    EXT_disjoint_timer_query *GetQuery();

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, EXT_disjoint_timer_queryImpl *query) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = EXT_disjoint_timer_queryImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( query, NativeType::EXT_disjoint_timer_query);
        object->SetAlignedPointerInInternalField(0, query);
        object->Set(context, ConvertToV8String(isolate, "ext_name"),
                    ConvertToV8String(isolate, "EXT_disjoint_timer_query"));
        query->BindFinalizer(isolate, object);
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
    EXT_disjoint_timer_query *query_;
};
