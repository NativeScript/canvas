//
// Created by Osei Fortune on 18/06/2024.
//

#include "GPUDeviceImpl.h"
#include "Caches.h"
#include "GPUAdapterImpl.h"
#include "GPUQueueImpl.h"

GPUDeviceImpl::GPUDeviceImpl(CanvasGPUDevice *device) : device_(device) {}

CanvasGPUDevice *GPUDeviceImpl::GetGPUDevice() {
    return this->device_;
}


void GPUDeviceImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUDevice"), func);
}

GPUDeviceImpl *GPUDeviceImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUDeviceImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUDeviceImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUDeviceTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUDevice"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "features"),
            GetFeatures
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "queue"),
            GetQueue
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "limits"),
            GetLimits
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "lost"),
            GetLost
    );

    tmpl->Set(
            ConvertToV8String(isolate, "destroy"),
            v8::FunctionTemplate::New(isolate, &Destroy));

    tmpl->Set(
            ConvertToV8String(isolate, "createBuffer"),
            v8::FunctionTemplate::New(isolate, &CreateBuffer));

    tmpl->Set(
            ConvertToV8String(isolate, "createTexture"),
            v8::FunctionTemplate::New(isolate, &CreateTexture));


    cache->GPUDeviceTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}


void
GPUDeviceImpl::GetFeatures(v8::Local<v8::Name> name,
                           const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto isolate = info.GetIsolate();
    if (ptr != nullptr) {
        auto context = isolate->GetCurrentContext();

        auto features = canvas_native_webgpu_device_get_features(ptr->GetGPUDevice());

        auto len = canvas_native_string_buffer_get_length(features);

        auto map = v8::Map::New(isolate);
        for (int i = 0; i < len; ++i) {
            auto item = canvas_native_string_buffer_get_value_at(features, i);
            if (item != nullptr) {
                auto keyValue = ConvertToV8OneByteString(isolate, (char *) item);
                map->Set(context, keyValue, keyValue);
            }

        }
        canvas_native_string_buffer_destroy(features);

        info.GetReturnValue().Set(map);

        return;
    }

    info.GetReturnValue().Set(v8::Map::New(info.GetIsolate()));
}


void
GPUDeviceImpl::GetLimits(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto limits = canvas_native_webgpu_device_get_limits(ptr->GetGPUDevice());

        auto ret = GPUSupportedLimitsImpl::NewInstance(info.GetIsolate(),
                                                       new GPUSupportedLimitsImpl(limits));
        info.GetReturnValue().Set(ret);
        return;
    }
    info.GetReturnValue().SetUndefined();
}


void
GPUDeviceImpl::GetQueue(v8::Local<v8::Name> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto queue = canvas_native_webgpu_device_get_queue(ptr->GetGPUDevice());
        auto ret = GPUQueueImpl::NewInstance(info.GetIsolate(),
                                             new GPUQueueImpl(queue));
        info.GetReturnValue().Set(ret);
        return;
    }
    info.GetReturnValue().SetUndefined();
}


void
GPUDeviceImpl::GetLost(v8::Local<v8::Name> name,
                       const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto isolate = info.GetIsolate();
    auto resolver = v8::Promise::Resolver::New(isolate->GetCurrentContext()).ToLocalChecked();
    info.GetReturnValue().Set(resolver->GetPromise());
    if (ptr != nullptr) {
        v8::Global<v8::Promise::Resolver> promise(isolate, resolver);
        auto callback = new PromiseCallback{
                isolate,
                std::move(promise)
        };

        canvas_native_webgpu_device_set_lost_callback(ptr->GetGPUDevice(),
                                                      [](int32_t reason, char *message,
                                                         void *data) {
                                                          if (data != nullptr) {
                                                              auto func = static_cast<PromiseCallback *>(data);
                                                              if (func->isolate != nullptr) {
                                                                  v8::Isolate *isolate = func->isolate;
                                                                  v8::Locker locker(isolate);
                                                                  v8::Isolate::Scope isolate_scope(
                                                                          isolate);
                                                                  v8::HandleScope handle_scope(
                                                                          isolate);
                                                                  v8::Local<v8::Promise::Resolver> callback = func->callback.Get(
                                                                          isolate);
                                                                  v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                                                                  v8::Context::Scope context_scope(
                                                                          context);

                                                                  auto ret = v8::Object::New(
                                                                          isolate);
                                                                  if (message == nullptr) {
                                                                      ret->Set(context,
                                                                               ConvertToV8String(
                                                                                       isolate,
                                                                                       "message"),
                                                                               v8::String::Empty(
                                                                                       isolate));
                                                                  } else {
                                                                      ret->Set(context,
                                                                               ConvertToV8String(
                                                                                       isolate,
                                                                                       "message"),
                                                                               ConvertToV8String(
                                                                                       isolate,
                                                                                       message));
                                                                      canvas_native_string_destroy(
                                                                              message);
                                                                  }

                                                                  ret->Set(context,
                                                                           ConvertToV8String(
                                                                                   isolate,
                                                                                   "reason"),
                                                                           v8::Int32::New(isolate,
                                                                                          reason)).IsJust();


                                                                  callback->Resolve(context, ret);
                                                                  delete static_cast<PromiseCallback *>(data);
                                                              }
                                                          }

                                                      }, callback);
    }
}


void GPUDeviceImpl::Destroy(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    canvas_native_webgpu_device_destroy(ptr->GetGPUDevice());
}


void GPUDeviceImpl::CreateBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    char *label = nullptr;
    bool mappedAtCreation = false;
    uint64_t size = 0;
    uint32_t usage = 0;

    char *error = nullptr;


    auto optionsVal = args[0];

    if (optionsVal->IsObject()) {
        auto options = optionsVal.As<v8::Object>();
        v8::Local<v8::Value> labelVal;
        options->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);
        label = *v8::String::Utf8Value(isolate, labelVal);

        v8::Local<v8::Value> mappedAtCreationVal;

        options->Get(context, ConvertToV8String(isolate, "mappedAtCreation")).ToLocal(
                &mappedAtCreationVal);

        if (!mappedAtCreationVal.IsEmpty()) {
            mappedAtCreation = mappedAtCreationVal->BooleanValue(isolate);
        }

        v8::Local<v8::Value> sizeVal;
        options->Get(context, ConvertToV8String(isolate, "size")).ToLocal(&sizeVal);

        if (sizeVal->IsNumber()) {
            size = (uint64_t) sizeVal->NumberValue(context).ToChecked();
        }


        v8::Local<v8::Value> usageVal;
        options->Get(context, ConvertToV8String(isolate, "usage")).ToLocal(&usageVal);

        if (usageVal->IsNumber()) {
            usage = usageVal->Uint32Value(context).ToChecked();
        }
    }

    auto buffer = canvas_native_webgpu_device_create_buffer(ptr->GetGPUDevice(), label, size, usage,
                                                            mappedAtCreation, error);

    if (buffer != nullptr) {
        auto bufImpl = new GPUBufferImpl(buffer);
        auto ret = GPUBufferImpl::NewInstance(isolate, bufImpl);
        args.GetReturnValue().Set(ret);

    } else {
        if (error != nullptr) {
            isolate->ThrowError(ConvertToV8String(isolate, error));
        }

        args.GetReturnValue().SetUndefined();
    }
}


void GPUDeviceImpl::CreateTexture(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUDeviceImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    CanvasCreateTextureDescriptor descriptor{};
    descriptor.dimension = CanvasTextureDimension::D2;
    descriptor.depthOrArrayLayers = 1;
    descriptor.sampleCount = 1;
    descriptor.mipLevelCount = 1;
    descriptor.view_formats = nullptr;
    descriptor.view_formats_size = 0;


    char *error = nullptr;


    auto optionsVal = args[0];


    if (optionsVal->IsObject()) {
        auto options = optionsVal.As<v8::Object>();
        v8::Local<v8::Value> depthOrArrayLayersVal;
        options->Get(context, ConvertToV8String(isolate, "depthOrArrayLayers")).ToLocal(
                &depthOrArrayLayersVal);

        if (depthOrArrayLayersVal->IsUint32()) {
            descriptor.depthOrArrayLayers = depthOrArrayLayersVal.As<v8::Uint32>()->Value();
        }


        v8::Local<v8::Value> widthVal;
        options->Get(context, ConvertToV8String(isolate, "width")).ToLocal(
                &widthVal);

        if (widthVal->IsUint32()) {
            descriptor.width = widthVal.As<v8::Uint32>()->Value();
        }


        v8::Local<v8::Value> heightVal;
        options->Get(context, ConvertToV8String(isolate, "height")).ToLocal(
                &heightVal);

        if (heightVal->IsUint32()) {
            descriptor.height = heightVal.As<v8::Uint32>()->Value();
        }


        v8::Local<v8::Value> usageVal;
        options->Get(context, ConvertToV8String(isolate, "usage")).ToLocal(
                &usageVal);

        if (usageVal->IsUint32()) {
            descriptor.usage = usageVal.As<v8::Uint32>()->Value();
        }


        v8::Local<v8::Value> sampleCountVal;
        options->Get(context, ConvertToV8String(isolate, "sampleCount")).ToLocal(
                &sampleCountVal);

        if (sampleCountVal->IsUint32()) {
            descriptor.sampleCount = sampleCountVal.As<v8::Uint32>()->Value();
        }


        v8::Local<v8::Value> mipLevelCountVal;
        options->Get(context, ConvertToV8String(isolate, "mipLevelCount")).ToLocal(
                &mipLevelCountVal);

        if (mipLevelCountVal->IsUint32()) {
            descriptor.mipLevelCount = mipLevelCountVal.As<v8::Uint32>()->Value();
        }


        v8::Local<v8::Value> dimensionVal;
        options->Get(context, ConvertToV8String(isolate, "dimension")).ToLocal(
                &dimensionVal);

        if (dimensionVal->IsString()) {
            auto dimension = ConvertFromV8String(isolate, dimensionVal);

            // todo use enum
            if (strcmp(dimension.c_str(), "d1") == 0) {
                descriptor.dimension = CanvasTextureDimension::D1;
            } else if (strcmp(dimension.c_str(), "d2") == 0) {
                descriptor.dimension = CanvasTextureDimension::D2;
            } else if (strcmp(dimension.c_str(), "d3") == 0) {
                descriptor.dimension = CanvasTextureDimension::D3;
            }

        }


        v8::Local<v8::Value> formatVal;
        options->Get(context, ConvertToV8String(isolate, "format")).ToLocal(
                &formatVal);

        if (formatVal->IsString()) {
            auto format = ConvertFromV8String(isolate, formatVal);

            // todo use enum
            CanvasGPUTextureFormat fmt{};
            if (strcmp(format.c_str(), "bgra8unorm") == 0) {
                fmt.tag = CanvasGPUTextureFormat_Tag::Bgra8Unorm;
                descriptor.format = fmt;
            } else if (strcmp(format.c_str(), "rgba8unorm") == 0) {
                fmt.tag = CanvasGPUTextureFormat_Tag::Rgba8Unorm;
                descriptor.format = fmt;
            }

        }

    }

    auto texture = canvas_native_webgpu_device_create_texture(ptr->GetGPUDevice(), &descriptor,
                                                              error);

    if (texture != nullptr) {
        auto textureImpl = new GPUTextureImpl(texture);
        auto ret = GPUTextureImpl::NewInstance(isolate, textureImpl);
        args.GetReturnValue().Set(ret);

    } else {
        if (error != nullptr) {
            isolate->ThrowError(ConvertToV8String(isolate, error));
        }

        args.GetReturnValue().SetUndefined();
    }
}
