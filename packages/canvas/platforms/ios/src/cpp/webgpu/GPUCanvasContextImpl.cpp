//
// Created by Osei Fortune on 20/06/2024.
//

#include "GPUCanvasContextImpl.h"
#include "Caches.h"
#include "GPUAdapterImpl.h"
#include "GPUUtils.h"

GPUCanvasContextImpl::GPUCanvasContextImpl(const CanvasGPUCanvasContext *context) : context_(
        context) {}

const CanvasGPUCanvasContext *GPUCanvasContextImpl::GetContext() {
    return this->context_;
}


void GPUCanvasContextImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUCanvasContext"), func).FromJust();
}

GPUCanvasContextImpl *GPUCanvasContextImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUCanvasContextImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUCanvasContextImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUCanvasContextTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUCanvasContext"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);


    tmpl->Set(
            ConvertToV8String(isolate, "configure"),
            v8::FunctionTemplate::New(isolate, &Configure));

    tmpl->Set(
            ConvertToV8String(isolate, "unconfigure"),
            v8::FunctionTemplate::New(isolate, &UnConfigure));

    tmpl->Set(
            ConvertToV8String(isolate, "getCurrentTexture"),
            v8::FunctionTemplate::New(isolate, &GetCurrentTexture));

    tmpl->Set(
            ConvertToV8String(isolate, "presentSurface"),
            v8::FunctionTemplate::New(isolate, &PresentSurface));

    tmpl->Set(
            ConvertToV8String(isolate, "getCapabilities"),
            v8::FunctionTemplate::New(isolate, &GetCapabilities));


    cache->GPUCanvasContextTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void GPUCanvasContextImpl::Configure(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUCanvasContextImpl *ptr = GetPointer(args.This());
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto optionsValue = args[0];
    if (ptr == nullptr) {
        return;
    }

    if (optionsValue->IsNullOrUndefined() || !optionsValue->IsObject()) {
        return;
    }

    auto options = optionsValue.As<v8::Object>();

    v8::Local<v8::Value> deviceValue;

    options->Get(context, ConvertToV8String(isolate, "device")).ToLocal(
            &deviceValue);


    if (deviceValue->IsNullOrUndefined() || !deviceValue->IsObject()) {
        return;
    }

    auto device = GPUDeviceImpl::GetPointer(deviceValue.As<v8::Object>());

    if (device == nullptr) { return; }

    CanvasGPUSurfaceConfiguration config{};
    config.alphaMode = CanvasGPUSurfaceAlphaMode::CanvasGPUSurfaceAlphaModeOpaque;
    config.presentMode = CanvasGPUPresentMode::CanvasGPUPresentModeFifo;
    config.view_formats = nullptr;
    config.view_formats_size = 0;
    config.usage = 0x10;
    config.format = CanvasOptionalGPUTextureFormat{
            CanvasOptionalGPUTextureFormatNone
    };
    config.size = nullptr;


    v8::Local<v8::Value> formatValue;

    if (options->Get(context, ConvertToV8String(isolate, "format")).ToLocal(
            &formatValue)) {
        auto format = ConvertFromV8String(isolate, formatValue);
        config.format = canvas_native_webgpu_enum_string_to_gpu_texture(format.c_str());
    }


    v8::Local<v8::Value> usageValue;
    options->Get(context, ConvertToV8String(isolate, "usage")).ToLocal(
            &usageValue);

    if (!usageValue.IsEmpty() && usageValue->IsUint32()) {
        config.usage = usageValue->Uint32Value(context).ToChecked();
    }


    v8::Local<v8::Value> presentModeValue;

    if (options->Get(context, ConvertToV8String(isolate, "presentMode")).ToLocal(
            &presentModeValue)) {
        if (presentModeValue->IsString()) {
            auto presentMode = ConvertFromV8String(isolate, presentModeValue);
            if (strcmp(presentMode.c_str(), "autoVsync") == 0) {
                config.presentMode = CanvasGPUPresentMode::CanvasGPUPresentModeAutoVsync;
            } else if (strcmp(presentMode.c_str(), "autoNoVsync") == 0) {
                config.presentMode = CanvasGPUPresentMode::CanvasGPUPresentModeAutoNoVsync;
            } else if (strcmp(presentMode.c_str(), "fifo") == 0) {
                config.presentMode = CanvasGPUPresentMode::CanvasGPUPresentModeFifo;
            } else if (strcmp(presentMode.c_str(), "fifoRelaxed") == 0) {
                config.presentMode = CanvasGPUPresentMode::CanvasGPUPresentModeFifoRelaxed;
            } else if (strcmp(presentMode.c_str(), "immediate") == 0) {
                config.presentMode = CanvasGPUPresentMode::CanvasGPUPresentModeImmediate;
            } else if (strcmp(presentMode.c_str(), "mailbox") == 0) {
                config.presentMode = CanvasGPUPresentMode::CanvasGPUPresentModeMailbox;
            }

        } else if (presentModeValue->IsInt32()) {
            config.presentMode = (CanvasGPUPresentMode) presentModeValue->Int32Value(
                    context).ToChecked();
        }
    }


    v8::Local<v8::Value> alphaModeValue;

    if (options->Get(context, ConvertToV8String(isolate, "alphaMode")).ToLocal(
            &alphaModeValue)) {
        if (alphaModeValue->IsString()) {
            auto alphaMode = ConvertFromV8String(isolate, alphaModeValue);
            if (strcmp(alphaMode.c_str(), "premultiplied") == 0) {
                config.alphaMode = CanvasGPUSurfaceAlphaMode::CanvasGPUSurfaceAlphaModePreMultiplied;
            } else if (strcmp(alphaMode.c_str(), "opaque") == 0) {
                config.alphaMode = CanvasGPUSurfaceAlphaMode::CanvasGPUSurfaceAlphaModeOpaque;
            } else if (strcmp(alphaMode.c_str(), "postmultiplied") == 0) {
                config.alphaMode = CanvasGPUSurfaceAlphaMode::CanvasGPUSurfaceAlphaModePostMultiplied;
            } else if (strcmp(alphaMode.c_str(), "inherit") == 0) {
                config.alphaMode = CanvasGPUSurfaceAlphaMode::CanvasGPUSurfaceAlphaModeInherit;
            } else if (strcmp(alphaMode.c_str(), "auto") == 0) {
                config.alphaMode = CanvasGPUSurfaceAlphaMode::CanvasGPUSurfaceAlphaModeAuto;
            }
        } else if (alphaModeValue->IsInt32()) {
            config.alphaMode = (CanvasGPUSurfaceAlphaMode) alphaModeValue->Int32Value(
                    context).ToChecked();
        }

    }


    v8::Local<v8::Value> sizeValue;
    options->Get(context, ConvertToV8String(isolate, "size")).ToLocal(
            &usageValue);

    CanvasExtent3d size = ParseExtent3d(isolate, sizeValue);

    if (size.width > 0) {
        config.size = &size;
    }

    canvas_native_webgpu_context_configure(ptr->GetContext(), device->GetGPUDevice(), &config);

}

void GPUCanvasContextImpl::UnConfigure(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUCanvasContextImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    //  canvas_native_webgpu_context_unconfigure(ptr->GetContext());

}

void GPUCanvasContextImpl::GetCurrentTexture(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUCanvasContextImpl *ptr = GetPointer(args.This());
    auto isolate = args.GetIsolate();
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto ctx = ptr->GetContext();

    auto texture = canvas_native_webgpu_context_get_current_texture(ctx);

    if (texture != nullptr) {
        auto textureImpl = new GPUTextureImpl(texture);
        auto ret = GPUTextureImpl::NewInstance(isolate, textureImpl);
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetUndefined();
}

void GPUCanvasContextImpl::PresentSurface(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUCanvasContextImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto textureVal = args[0];
    if (!textureVal.IsEmpty() && textureVal->IsObject()) {
        auto texture = GPUTextureImpl::GetPointer(textureVal.As<v8::Object>());
        if (texture != nullptr) {
            auto ctx = ptr->GetContext();

            canvas_native_webgpu_context_present_surface(ctx, texture->GetTexture());
        }
    }

}

void GPUCanvasContextImpl::GetCapabilities(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUCanvasContextImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto adapterVal = args[0];
    auto ret = v8::Object::New(isolate);

    if (!adapterVal.IsEmpty() && adapterVal->IsObject()) {
        auto adapter = GPUAdapterImpl::GetPointer(adapterVal.As<v8::Object>());
        auto ctx = ptr->GetContext();

        if (adapter != nullptr) {
            auto cap = canvas_native_webgpu_context_get_capabilities(ctx, adapter->GetGPUAdapter());
            auto formats_len = canvas_native_string_buffer_get_length(cap->formats);
            auto formats = v8::Array::New(isolate, (int) formats_len);
            for (int i = 0; i < formats_len; i++) {
                auto format = canvas_native_string_buffer_get_value_at(cap->formats, i);
                formats->Set(context, i, ConvertToV8String(isolate, format));
                canvas_native_string_destroy(format);
            }

            auto present_modes_len = canvas_native_string_buffer_get_length(cap->present_modes);
            auto present_modes = v8::Array::New(isolate, (int) present_modes_len);


            for (int i = 0; i < present_modes_len; i++) {
                auto mode = canvas_native_string_buffer_get_value_at(cap->present_modes, i);
                present_modes->Set(context, i, ConvertToV8String(isolate, mode));
                canvas_native_string_destroy(mode);
            }

            auto alpha_modes_len = canvas_native_string_buffer_get_length(cap->alpha_modes);
            auto alpha_modes = v8::Array::New(isolate, (int) alpha_modes_len);


            for (int i = 0; i < alpha_modes_len; i++) {
                auto mode = canvas_native_string_buffer_get_value_at(cap->alpha_modes, i);
                alpha_modes->Set(context, i, ConvertToV8String(isolate, mode));
                canvas_native_string_destroy(mode);
            }

            ret->Set(context, ConvertToV8String(isolate, "format"), formats);
            ret->Set(context, ConvertToV8String(isolate, "presentModes"), present_modes);
            ret->Set(context, ConvertToV8String(isolate, "alphaModes"), alpha_modes);
            ret->Set(context, ConvertToV8String(isolate, "usages"),
                     v8::Uint32::NewFromUnsigned(isolate, cap->usages));

            canvas_native_webgpu_struct_surface_capabilities_release(cap);

            args.GetReturnValue().Set(ret);
            return;
        }


    }

    ret->Set(context, ConvertToV8String(isolate, "format"), v8::Array::New(isolate));
    ret->Set(context, ConvertToV8String(isolate, "presentModes"), v8::Array::New(isolate));
    ret->Set(context, ConvertToV8String(isolate, "alphaModes"), v8::Array::New(isolate));
    ret->Set(context, ConvertToV8String(isolate, "usages"), v8::Uint32::New(isolate, 0));
    args.GetReturnValue().Set(ret);
}
