//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once
#include "Common.h"
#include "Helpers.h"
#include <vector>
#include "ObjectWrapperImpl.h"
class WebGLActiveInfoImpl: ObjectWrapperImpl {
public:
    WebGLActiveInfoImpl(WebGLActiveInfo* info);

    ~WebGLActiveInfoImpl(){
        canvas_native_webgl_active_info_destroy(this->info_);
        this->info_ = nullptr;
    }

    WebGLActiveInfo* GetWebGLActiveInfo();

    static WebGLActiveInfoImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, WebGLActiveInfoImpl *info) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WebGLActiveInfoImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( info, NativeType::WebGLActiveInfo);
        object->SetAlignedPointerInInternalField(0, info);
        info->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void
    GetName(v8::Local<v8::Name> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void
    GetSize(v8::Local<v8::Name> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void
    GetType(v8::Local<v8::Name> name, const v8::PropertyCallbackInfo<v8::Value> &info);

private:
    WebGLActiveInfo* info_;
};

