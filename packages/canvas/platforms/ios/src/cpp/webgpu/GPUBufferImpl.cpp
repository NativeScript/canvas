//
// Created by Osei Fortune on 18/06/2024.
//

#include "GPUBufferImpl.h"
#include "Caches.h"

GPUBufferImpl::GPUBufferImpl(const CanvasGPUBuffer *buffer) : buffer_(buffer) {}

const CanvasGPUBuffer *GPUBufferImpl::GetGPUBuffer() {
    return this->buffer_;
}


void GPUBufferImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUBuffer"), func).FromJust();
}

GPUBufferImpl *GPUBufferImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUBufferImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUBufferImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUBufferTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUBuffer"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "usage"),
            GetUsage
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "size"),
            GetSize
    );

    tmpl->Set(
            ConvertToV8String(isolate, "destroy"),
            v8::FunctionTemplate::New(isolate, &Destroy));


    tmpl->Set(
            ConvertToV8String(isolate, "mapAsync"),
            v8::FunctionTemplate::New(isolate, &MapAsync));


    tmpl->Set(
            ConvertToV8String(isolate, "unmap"),
            v8::FunctionTemplate::New(isolate, &UnMap));


    tmpl->Set(
            ConvertToV8String(isolate, "getMappedRange"),
            v8::FunctionTemplate::New(isolate, &GetMappedRange));

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "label"),
            GetLabel
    );


    cache->GPUBufferTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}


void
GPUBufferImpl::GetUsage(v8::Local<v8::Name> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto usage = canvas_native_webgpu_buffer_usage(ptr->GetGPUBuffer());
        info.GetReturnValue().Set(
                usage
        );
        return;
    }
    info.GetReturnValue().Set(0);
}


void
GPUBufferImpl::GetSize(v8::Local<v8::Name> name,
                       const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto size = canvas_native_webgpu_buffer_size(ptr->GetGPUBuffer());
        info.GetReturnValue().Set((double) size);
        return;
    }
    info.GetReturnValue().Set(0);
}


void GPUBufferImpl::Destroy(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUBufferImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    canvas_native_webgpu_buffer_destroy(ptr->GetGPUBuffer());
}


void GPUBufferImpl::UnMap(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUBufferImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    canvas_native_webgpu_buffer_unmap(ptr->GetGPUBuffer());
}

struct MapAsyncData {
    enum CanvasGPUErrorType type;
    char* error;
};

void GPUBufferImpl::MapAsync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUBufferImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto mode = args[0]->Int32Value(context).ToChecked();

    int64_t offset = -1;
    int64_t size = -1;

    auto offsetVal = args[1];
    if (offsetVal->IsNumber()) {
        offset = (int64_t) offsetVal.As<v8::Number>()->Value();
    }

    auto sizeVal = args[2];

    if (sizeVal->IsNumber()) {
        size = (int64_t) sizeVal.As<v8::Number>()->Value();
    }

    auto resolver = v8::Promise::Resolver::New(isolate->GetCurrentContext()).ToLocalChecked();
    args.GetReturnValue().Set(resolver->GetPromise());

    auto callback = new PromiseCallback{
            isolate,
            resolver,
            [](bool done, void *data){
                auto async_data = static_cast<PromiseCallback *>(data);
                auto func = async_data->inner_.get();
                if (func != nullptr && func->isolate_ != nullptr) {
                    v8::Isolate *isolate = func->isolate_;
                    v8::Locker locker(isolate);
                    v8::Isolate::Scope isolate_scope(
                            isolate);
                    v8::HandleScope handle_scope(
                            isolate);
                    v8::Local<v8::Promise::Resolver> callback = func->callback_.Get(
                            isolate);
                    v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                    v8::Context::Scope context_scope(
                            context);


                    if (func->data == nullptr) {
                        callback->Resolve(context, v8::Undefined(isolate));
                    } else {
                        auto errorData = static_cast<MapAsyncData*>(func->data);
                        callback->Reject(context,
                                         v8::Exception::Error(
                                                 ConvertToV8String(
                                                         isolate,
                                                         errorData->error)));
                        canvas_native_string_destroy(
                                errorData->error);
                        delete errorData;
                        func->data = nullptr;
                    }
                }
                delete static_cast<PromiseCallback *>(data);
            }
    };
    callback->prepare();


    canvas_native_webgpu_buffer_map_async(ptr->GetGPUBuffer(), mode == 1 ? GPUMapMode::GPUMapModeRead : GPUMapMode::GPUMapModeWrite, offset, size,
                                          [](enum CanvasGPUErrorType type, char *error, void *data) {
                                              if (data != nullptr) {
                                                  auto async_data = static_cast<PromiseCallback *>(data);
                                                  auto inner = async_data->inner_.get();
                                                  if (inner != nullptr) {
                                                      if(error != nullptr){
                                                          inner->data = new MapAsyncData {
                                                              type, error
                                                          };
                                                      }
                                                      async_data->execute(true);
                                                  }
                                              }
                                          }, callback);
}

void GPUBufferImpl::GetMappedRange(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUBufferImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    int64_t offset = -1;
    int64_t size = -1;

    auto offsetVal = args[0];
    if (offsetVal->IsNumber()) {
        offset = (int64_t) offsetVal.As<v8::Number>()->Value();
    }

    auto sizeVal = args[1];

    if (sizeVal->IsNumber()) {
        size = (int64_t) sizeVal.As<v8::Number>()->Value();
    }

    auto buf = canvas_native_webgpu_buffer_get_mapped_range(ptr->GetGPUBuffer(), offset, size);
    auto isolate = args.GetIsolate();
    if (buf == nullptr) {
        args.GetReturnValue().Set(v8::ArrayBuffer::New(isolate, 0));
    } else {
        auto store = v8::ArrayBuffer::NewBackingStore(buf, size, [](void *data, size_t length,
                                                                    void *deleter_data) {

                                                                 }, nullptr);
        auto ab = v8::ArrayBuffer::New(isolate, std::move(store));
        args.GetReturnValue().Set(ab);
    }
}


void
GPUBufferImpl::GetLabel(v8::Local<v8::Name> name,
                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto label = canvas_native_webgpu_buffer_get_label(ptr->buffer_);
        if (label == nullptr) {
            info.GetReturnValue().SetEmptyString();
            return;
        }
        info.GetReturnValue().Set(
                ConvertToV8String(info.GetIsolate(), label)
        );
        canvas_native_string_destroy(label);
        return;
    }

    info.GetReturnValue().SetEmptyString();
}
