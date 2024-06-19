//
// Created by Osei Fortune on 17/06/2024.
//

#include "GPUAdapterImpl.h"
#include "Caches.h"

GPUAdapterImpl::GPUAdapterImpl(CanvasGPUAdapter *adapter) : adapter_(adapter) {}

CanvasGPUAdapter *GPUAdapterImpl::GetGPUAdapter() {
    return this->adapter_;
}


void GPUAdapterImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUAdapter"), func);
}

GPUAdapterImpl *GPUAdapterImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUAdapterImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUAdapterImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUAdapterTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUAdapter"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "features"),
            GetFeatures
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "isFallbackAdapter"),
            GetIsFallbackAdapter
    );

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "limits"),
            GetLimits
    );


    tmpl->Set(
            ConvertToV8String(isolate, "requestAdapterInfo"),
            v8::FunctionTemplate::New(isolate, &RequestAdapterInfo));


    tmpl->Set(
            ConvertToV8String(isolate, "requestDevice"),
            v8::FunctionTemplate::New(isolate, &RequestDevice));


    cache->GPUAdapterTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}


void
GPUAdapterImpl::GetFeatures(v8::Local<v8::Name> name,
                            const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto isolate = info.GetIsolate();
    if (ptr != nullptr) {
        auto context = isolate->GetCurrentContext();

        auto features = canvas_native_webgpu_adapter_get_features(ptr->GetGPUAdapter());

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
GPUAdapterImpl::GetIsFallbackAdapter(v8::Local<v8::Name> name,
                                     const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        info.GetReturnValue().Set(
                canvas_native_webgpu_adapter_is_fallback_adapter(ptr->GetGPUAdapter())
        );
        return;
    }
    info.GetReturnValue().Set(false);
}

void
GPUAdapterImpl::GetLimits(v8::Local<v8::Name> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto limits = canvas_native_webgpu_adapter_get_limits(ptr->GetGPUAdapter());
        auto ret = GPUSupportedLimitsImpl::NewInstance(info.GetIsolate(),
                                                       new GPUSupportedLimitsImpl(limits));
        info.GetReturnValue().Set(ret);
        return;
    }
    info.GetReturnValue().SetUndefined();
}


void GPUAdapterImpl::RequestAdapterInfo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUAdapterImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto info = canvas_native_webgpu_request_adapter_info(ptr->GetGPUAdapter());

    auto ret = v8::Object::New(isolate);


    auto architecture = canvas_native_webgpu_adapter_info_architecture(info);

    if (architecture != nullptr) {
        ret->Set(context, ConvertToV8String(isolate, "architecture"),
                 ConvertToV8String(isolate, architecture));
        canvas_native_string_destroy(architecture);
    } else {
        ret->Set(context, ConvertToV8String(isolate, "architecture"), v8::String::Empty(isolate));
    }


    auto description = canvas_native_webgpu_adapter_info_description(info);

    if (description != nullptr) {
        ret->Set(context, ConvertToV8String(isolate, "description"),
                 ConvertToV8String(isolate, description));
        canvas_native_string_destroy(description);
    } else {
        ret->Set(context, ConvertToV8String(isolate, "description"), v8::String::Empty(isolate));
    }


    auto device = canvas_native_webgpu_adapter_info_device(info);

    if (device != nullptr) {
        ret->Set(context, ConvertToV8String(isolate, "device"),
                 ConvertToV8String(isolate, device));
        canvas_native_string_destroy(device);
    } else {
        ret->Set(context, ConvertToV8String(isolate, "device"), v8::String::Empty(isolate));
    }


    auto vendor = canvas_native_webgpu_adapter_info_vendor(info);

    if (vendor != nullptr) {
        ret->Set(context, ConvertToV8String(isolate, "vendor"),
                 ConvertToV8String(isolate, vendor));
        canvas_native_string_destroy(device);
    } else {
        ret->Set(context, ConvertToV8String(isolate, "vendor"), v8::String::Empty(isolate));
    }


    args.GetReturnValue().Set(ret);

}

void GPUAdapterImpl::RequestDevice(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUAdapterImpl *ptr = GetPointer(args.This());
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto optionsValue = args[0];
    auto cb = args[1];
    if (ptr == nullptr) {
        // todo error
        cb.As<v8::Function>()->Call(context, context->Global(), 0, nullptr);
        return;
    }

    v8::Global<v8::Function> func(isolate, cb.As<v8::Function>());

    std::string label;

    std::vector<std::string> required_features_buf;

    CanvasGPUSupportedLimits *limits = nullptr;

    if (!optionsValue.IsEmpty() && optionsValue->IsObject()) {
        auto options = optionsValue.As<v8::Object>();
        v8::Local<v8::Value> labelValue;
        options->Get(context, ConvertToV8String(isolate, "label")).ToLocal(
                &labelValue);

        if (!labelValue.IsEmpty() && labelValue->IsString()) {
            label = ConvertFromV8String(isolate, labelValue);
        }

        v8::Local<v8::Value> requiredFeaturesValue;

        options->Get(context, ConvertToV8String(isolate, "requiredFeatures")).ToLocal(
                &requiredFeaturesValue);


        if (!requiredFeaturesValue.IsEmpty() && requiredFeaturesValue->IsSet()) {
            v8::Local<v8::Set> requiredFeaturesSet = requiredFeaturesValue.As<v8::Set>();
            v8::Local<v8::Array> requiredFeatures = requiredFeaturesSet->AsArray();
            int len = requiredFeatures->Length();

            for (int i = 0; i < len; i++) {
                auto item = requiredFeatures->Get(context, i);
                if (!item.IsEmpty()) {
                    auto value = item.ToLocalChecked();
                    if (value->IsString()) {
                        required_features_buf.push_back(ConvertFromV8String(isolate, value));
                    }
                }
            }
        }


        v8::Local<v8::Value> limitsValue;

        options->Get(context, ConvertToV8String(isolate, "requiredLimits")).ToLocal(
                &limitsValue);

        if (!limitsValue.IsEmpty() && limitsValue->IsObject()) {
            auto limits_ptr = GPUSupportedLimitsImpl::GetPointer(limitsValue.As<v8::Object>());
            if (limits_ptr != nullptr) {
                limits = limits_ptr->GetLimits();
            }
        }
    }

    char **required_features_data = nullptr;
    size_t required_features_data_length = required_features_buf.size();

    if (required_features_data_length > 0) {
        required_features_data = new char *[required_features_data_length + 1];

        for (size_t i = 0; i < required_features_data_length; ++i) {
            std::strcpy(required_features_data[i], required_features_buf[i].c_str());
        }
    }


    auto callback = new AsyncCallback{
            isolate,
            std::move(func),
            static_cast<void *>(required_features_data)
    };

    canvas_native_webgpu_request_device(ptr->GetGPUAdapter(),
                                        label.empty() ? nullptr : label.c_str(),
                                        required_features_data, required_features_data_length,
                                        limits,
                                        [](char *error, CanvasGPUDevice *device, void *data) {
                                            if (data != nullptr) {
                                                auto func = static_cast<AsyncCallback *>(data);
                                                if (func->isolate != nullptr) {
                                                    v8::Isolate *isolate = func->isolate;
                                                    v8::Locker locker(isolate);
                                                    v8::Isolate::Scope isolate_scope(isolate);
                                                    v8::HandleScope handle_scope(isolate);
                                                    v8::Local<v8::Function> callback = func->callback.Get(
                                                            isolate);
                                                    v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                                                    v8::Context::Scope context_scope(context);

                                                    if (func->data != nullptr) {
                                                        delete[] static_cast<char **>(func->data);
                                                        func->data = nullptr;
                                                    }

                                                    if (error != nullptr) {
                                                        v8::Local<v8::Value> args[1] = {
                                                                v8::Exception::Error(
                                                                        ConvertToV8String(isolate,
                                                                                          error))};

                                                        canvas_native_string_destroy(error);

                                                        callback->Call(context, context->Global(),
                                                                       1,
                                                                       args);  // ignore JS return value
                                                        delete static_cast<AsyncCallback *>(data);
                                                    } else {

                                                        auto impl = new GPUDeviceImpl(device);
                                                        auto ret = GPUDeviceImpl::NewInstance(
                                                                isolate, impl);


                                                        v8::Local<v8::Value> args[2] = {
                                                                v8::Null(isolate), ret};


                                                        callback->Call(context, context->Global(),
                                                                       2,
                                                                       args);  // ignore JS return value

                                                        delete static_cast<AsyncCallback *>(data);

                                                    }
                                                }
                                            }
                                        }, callback);


}
