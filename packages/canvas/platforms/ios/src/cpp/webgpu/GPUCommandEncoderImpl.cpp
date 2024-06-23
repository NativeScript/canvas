//
// Created by Osei Fortune on 23/06/2024.
//

#include "GPUCommandEncoderImpl.h"
#include "Caches.h"
#include "GPUComputePassImpl.h"
#include "GPUQuerySetImpl.h"
#include "GPUBufferImpl.h"

GPUCommandEncoderImpl::GPUCommandEncoderImpl(CanvasGPUCommandEncoder *encoder) : encoder_(
        encoder) {}

CanvasGPUCommandEncoder *GPUCommandEncoderImpl::GetEncoder() {
    return this->encoder_;
}


void GPUCommandEncoderImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUDevice"), func);
}

GPUCommandEncoderImpl *GPUCommandEncoderImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUCommandEncoderImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUCommandEncoderImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUCommandEncoderTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUCommandEncoder"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);


    tmpl->Set(
            ConvertToV8String(isolate, "beginComputePass"),
            v8::FunctionTemplate::New(isolate, &BeginComputePass));


    tmpl->Set(
            ConvertToV8String(isolate, "clearBuffer"),
            v8::FunctionTemplate::New(isolate, &ClearBuffer));


    cache->GPUCommandEncoderTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}


void GPUCommandEncoderImpl::BeginComputePass(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto descVal = args[0];

    CanvasGPUComputePass *pass;

    if (!descVal->IsNullOrUndefined() && descVal->IsObject()) {
        auto desc = descVal.As<v8::Object>();


        v8::Local<v8::Value> labelVal;
        desc->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);

        char *label = nullptr;

        if (!labelVal.IsEmpty() && labelVal->IsString()) {
            label = *v8::String::Utf8Value(isolate, labelVal);
        }

        v8::Local<v8::Value> querySetVal;
        desc->Get(context, ConvertToV8String(isolate, "querySet")).ToLocal(&querySetVal);

        CanvasGPUQuerySet *querySet = nullptr;
        if (!querySetVal.IsEmpty() && querySetVal->IsObject()) {
            auto queryPtr = GPUQuerySetImpl::GetPointer(querySetVal.As<v8::Object>());
            if (queryPtr != nullptr) {
                querySet = queryPtr->GetQuerySet();
            }
        }

        v8::Local<v8::Value> beginningOfPassWriteIndexVal;
        int32_t beginningOfPassWriteIndex = -1;

        v8::Local<v8::Value> endOfPassWriteIndexVal;
        int32_t endOfPassWriteIndex = -1;


        desc->Get(context, ConvertToV8String(isolate, "beginningOfPassWriteIndex")).ToLocal(
                &beginningOfPassWriteIndexVal);

        desc->Get(context, ConvertToV8String(isolate, "endOfPassWriteIndex")).ToLocal(
                &endOfPassWriteIndexVal);


        if (beginningOfPassWriteIndexVal->IsInt32()) {
            beginningOfPassWriteIndex = beginningOfPassWriteIndexVal.As<v8::Int32>()->Value();
        }

        if (endOfPassWriteIndexVal->IsInt32()) {
            endOfPassWriteIndex = endOfPassWriteIndexVal.As<v8::Int32>()->Value();
        }

        pass = canvas_native_webgpu_command_encoder_begin_compute_pass(ptr->GetEncoder(),
                                                                       querySet, label,
                                                                       beginningOfPassWriteIndex,
                                                                       endOfPassWriteIndex);


    } else {
        pass = canvas_native_webgpu_command_encoder_begin_compute_pass(ptr->GetEncoder(),
                                                                       nullptr, nullptr, -1,
                                                                       -1);

    }


    if (pass != nullptr) {
        auto value = new GPUComputePassImpl(pass);
        auto ret = GPUComputePassImpl::NewInstance(isolate, value);
        args.GetReturnValue().Set(ret);
    } else {
        args.GetReturnValue().SetUndefined();
    }

}


void GPUCommandEncoderImpl::ClearBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

    auto bufferVal = args[0];
    CanvasGPUBuffer *buffer = nullptr;

    if (bufferVal->IsObject()) {
        auto bufferPtr = GPUBufferImpl::GetPointer(bufferVal.As<v8::Object>());
        if (bufferPtr != nullptr) {
            buffer = bufferPtr->GetGPUBuffer();
        }
    }

    if (buffer == nullptr) {
        return;
    }

    int64_t offset = -1;
    auto offsetVal = args[1];

    if (offsetVal->IsNumber()) {
        offset = (int64_t) offsetVal.As<v8::Number>()->Value();
    }

    int64_t size = -1;
    auto sizeVal = args[2];

    if (sizeVal->IsNumber()) {
        size = (int64_t) sizeVal.As<v8::Number>()->Value();
    }


    canvas_native_webgpu_command_encoder_clear_buffer(ptr->GetEncoder(), buffer, offset, size);


}
