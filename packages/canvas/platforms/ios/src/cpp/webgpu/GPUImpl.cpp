//
// Created by Osei Fortune on 17/06/2024.
//

#include "GPUImpl.h"
#include "Caches.h"

GPUImpl::GPUImpl(const CanvasWebGPUInstance *instance) : instance_(instance) {}

const CanvasWebGPUInstance *GPUImpl::GetGPUInstance() {
    return this->instance_;
}

void GPUImpl::Init(const v8::Local<v8::Object> &canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPU"), func).FromJust();
}


GPUImpl *GPUImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, Ctor);

    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPU"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->Set(ConvertToV8String(isolate, "__getPointer"),
              v8::FunctionTemplate::New(isolate, __GetPointer));

    tmpl->Set(ConvertToV8String(isolate, "requestAdapter"),
              v8::FunctionTemplate::New(isolate, &RequestAdapter));

    cache->GPUTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void GPUImpl::Ctor(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();

    auto ret = args.This();

    auto instance = canvas_native_webgpu_instance_create();

    auto object = new GPUImpl(instance);

    ret->SetAlignedPointerInInternalField(0, object);

    SetNativeType(object, NativeType::GPUInstance);

    object->BindFinalizer(isolate, ret);

    args.GetReturnValue().Set(ret);
}


void GPUImpl::RequestAdapter(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    GPUImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args[1].As<v8::Function>()->Call(context, context->Global(), 0, nullptr);
        return;
    }


    CanvasGPURequestAdapterOptions opts{};
    opts.force_fallback_adapter = false;
    opts.power_preference = CanvasGPUPowerPreference::CanvasGPUPowerPreferenceNone;
    auto optionsValue = args[0];

    if (optionsValue->IsObject()) {
        auto options = optionsValue.As<v8::Object>();
        v8::Local<v8::Value> ppValue;
        options->Get(context, ConvertToV8String(isolate, "powerPreference")).ToLocal(&ppValue);
        if (ppValue->IsNumber()) {
            switch (ppValue->Int32Value(context).ToChecked()) {
                case 1:
                    opts.power_preference = CanvasGPUPowerPreference::CanvasGPUPowerPreferenceLowPower;
                    break;
                case 2:
                    opts.power_preference = CanvasGPUPowerPreference::CanvasGPUPowerPreferenceHighPerformance;
                    break;
                default:
                    break;
            }

        }

        v8::Local<v8::Value> forceFallbackValue;

        options->Get(context, ConvertToV8String(isolate, "forceFallbackAdapter")).ToLocal(
                &forceFallbackValue);


        if (!forceFallbackValue.IsEmpty()) {
            opts.force_fallback_adapter = forceFallbackValue->BooleanValue(isolate);
        }
    }


    auto callback = new AsyncCallback{
            isolate,
            args[1].As<v8::Function>(),
            [](bool done, void *data) {
                if (data != nullptr) {
                    auto async_data = static_cast<AsyncCallback *>(data);
                    auto func = async_data->inner_.get();
                    if (func != nullptr && func->isolate_ != nullptr) {
                        v8::Isolate *isolate = func->isolate_;
                        v8::Locker locker(isolate);
                        v8::Isolate::Scope isolate_scope(isolate);
                        v8::HandleScope handle_scope(isolate);
                        v8::Local<v8::Function> callback = func->callback_.Get(
                                isolate);
                        v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                        v8::Context::Scope context_scope(context);

                        if (func->data != nullptr) {
                            auto impl = new GPUAdapterImpl(
                                    static_cast<const CanvasGPUAdapter *>(func->data));
                            auto ret = GPUAdapterImpl::NewInstance(
                                    isolate, impl);


                            v8::Local<v8::Value> args[2] = {
                                    v8::Null(isolate), ret};


                            callback->Call(context, context->Global(),
                                           2,
                                           args);  // ignore JS return value
                            
                            delete static_cast<AsyncCallback *>(data);

                        } else {
                            v8::Local<v8::Value> args[1] = {
                                    v8::Null(isolate)};

                            callback->Call(context, context->Global(),
                                           1,
                                           args);  // ignore JS return value
                            delete static_cast<AsyncCallback *>(data);
                        }
                    }
                }
            }
    };
    callback->prepare();
    canvas_native_webgpu_request_adapter(ptr->GetGPUInstance(), &opts,
                                         [](const CanvasGPUAdapter *adapter, void *data) {
                                             if (data != nullptr) {
                                                 auto async_data = static_cast<AsyncCallback *>(data);
                                                 auto inner = async_data->inner_.get();
                                                 if (inner != nullptr) {
                                                     inner->data = (void *) adapter;
                                                     async_data->execute(true);
                                                 }
                                             }
                                         }, callback);
}


void GPUImpl::__GetPointer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetEmptyString();
        return;
    }

    auto isolate = args.GetIsolate();

    auto pointer = canvas_native_webgpu_get_pointer_addr(ptr->GetGPUInstance());
    auto ret = std::to_string(pointer);
    args.GetReturnValue().Set(ConvertToV8String(isolate, ret));
}
