//
// Created by Osei Fortune on 18/06/2024.
//

#include "GPUQueueImpl.h"
#include "Caches.h"
#include "GPUBufferImpl.h"
#include "GPUCommandBufferImpl.h"

GPUQueueImpl::GPUQueueImpl(const CanvasGPUQueue *queue) : queue_(queue) {}

const CanvasGPUQueue *GPUQueueImpl::GetGPUQueue() {
    return this->queue_;
}


void GPUQueueImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUQueue"), func);
}

GPUQueueImpl *GPUQueueImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUQueueImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUQueueImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUQueueTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUQueue"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);


    tmpl->Set(
            ConvertToV8String(isolate, "writeBuffer"),
            v8::FunctionTemplate::New(isolate, &WriteBuffer));

    tmpl->Set(
            ConvertToV8String(isolate, "submit"),
            v8::FunctionTemplate::New(isolate, &Submit));

    cache->GPUQueueTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void GPUQueueImpl::WriteBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto bufferValue = args[0];

    if (bufferValue->IsObject()) {
        auto buffer = GPUBufferImpl::GetPointer(bufferValue.As<v8::Object>());


        auto bufferOffset = (uint64_t) args[1].As<v8::Number>()->Value();

        auto dataValue = args[2].As<v8::TypedArray>();

        auto offset = dataValue->ByteOffset();

        auto store = dataValue->Buffer()->GetBackingStore();

        auto data = static_cast<uint8_t *>(store->Data()) + offset;

        auto data_size = store->ByteLength();

        auto dataOffset = (uint64_t) args[3].As<v8::Number>()->Value();

        int64_t size = -1;
        auto sizeValue = args[4];

        if (sizeValue->IsNumber()) {
            size = (int64_t) sizeValue->ToNumber(context).ToLocalChecked()->Value();
        }


        if (buffer != nullptr) {
            canvas_native_webgpu_queue_write_buffer(ptr->GetGPUQueue(), buffer->GetGPUBuffer(),
                                                    bufferOffset, data, data_size, dataOffset,
                                                    size);
        }
    }


}


void GPUQueueImpl::Submit(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    std::vector<const CanvasGPUCommandBuffer *> commandBuffers;
    auto commandBuffersVal = args[0];

    if (commandBuffersVal->IsArray()) {
        auto commandBuffersArray = commandBuffersVal.As<v8::Array>();
        auto len = commandBuffersArray->Length();

        for (int i = 0; i < len; i++) {
            v8::Local<v8::Value> item;
            commandBuffersArray->Get(context, i).ToLocal(&item);
            if (!item.IsEmpty() && item->IsObject()) {
                auto buffer = GPUCommandBufferImpl::GetPointer(item.As<v8::Object>());
                if (buffer != nullptr) {
                    commandBuffers.push_back(buffer->GetGPUCommandBuffer());
                }
            }
        }

        canvas_native_webgpu_queue_submit(ptr->GetGPUQueue(), commandBuffers.data(),
                                          commandBuffers.size());

    }


}

