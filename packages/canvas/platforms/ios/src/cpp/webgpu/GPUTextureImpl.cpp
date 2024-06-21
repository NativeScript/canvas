//
// Created by Osei Fortune on 21/06/2024.
//

#include "GPUTextureImpl.h"

#include "Caches.h"

GPUTextureImpl::GPUTextureImpl(CanvasGPUTexture *texture) : texture_(texture) {}

CanvasGPUTexture *GPUTextureImpl::GetTexture() {
    return this->texture_;
}


void GPUTextureImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUTexture"), func);
}

GPUTextureImpl *GPUTextureImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUTextureImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUTextureImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUTextureTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUTexture"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);


    tmpl->Set(
            ConvertToV8String(isolate, "destroy"),
            v8::FunctionTemplate::New(isolate, &Destroy));


    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "width"),
            GetWidth
    );


    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "height"),
            GetHeight
    );


    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "usage"),
            GetUsage
    );


    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "depthOrArrayLayers"),
            GetDepthOrArrayLayers
    );


    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "dimension"),
            GetDimension
    );


    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "sampleCount"),
            GetSampleCount
    );


    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "mipLevelCount"),
            GetMipLevelCount
    );

    cache->GPUTextureTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void
GPUTextureImpl::GetDimension(v8::Local<v8::Name> name,
                             const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto width = canvas_native_webgpu_texture_get_dimension(ptr->GetTexture());
        auto isolate = info.GetIsolate();
        switch (width) {
            case D1:
                info.GetReturnValue().Set(ConvertToV8String(isolate, "d1"));
                break;
            case D2:
                info.GetReturnValue().Set(ConvertToV8String(isolate, "d2"));
                break;
            case D3:
                info.GetReturnValue().Set(ConvertToV8String(isolate, "d3"));
                break;
        }

        return;
    }
}


void
GPUTextureImpl::GetWidth(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto width = canvas_native_webgpu_texture_get_width(ptr->GetTexture());
        info.GetReturnValue().Set(width);
        return;
    }
    info.GetReturnValue().Set(0);
}


void
GPUTextureImpl::GetHeight(v8::Local<v8::Name> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto value = canvas_native_webgpu_texture_get_height(ptr->GetTexture());
        info.GetReturnValue().Set(value);
        return;
    }
    info.GetReturnValue().Set(0);
}

void
GPUTextureImpl::GetDepthOrArrayLayers(v8::Local<v8::Name> name,
                                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto value = canvas_native_webgpu_texture_get_depth_or_array_layers(ptr->GetTexture());
        info.GetReturnValue().Set(value);
        return;
    }
    info.GetReturnValue().Set(0);
}


void
GPUTextureImpl::GetUsage(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto value = canvas_native_webgpu_texture_get_usage(ptr->GetTexture());
        info.GetReturnValue().Set(value);
        return;
    }
    info.GetReturnValue().Set(0);
}


void
GPUTextureImpl::GetFormat(v8::Local<v8::Name> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto isolate = info.GetIsolate();
        auto value = canvas_native_webgpu_texture_get_format(ptr->GetTexture());
        auto str = canvas_native_webgpu_enum_gpu_texture_to_string(value);
        info.GetReturnValue().Set(ConvertToV8String(isolate, str));
        return;
    }
    info.GetReturnValue().Set(0);
}

void
GPUTextureImpl::GetSampleCount(v8::Local<v8::Name> name,
                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto value = canvas_native_webgpu_texture_get_sample_count(ptr->GetTexture());
        info.GetReturnValue().Set(value);
        return;
    }
    info.GetReturnValue().Set(0);
}

void
GPUTextureImpl::GetMipLevelCount(v8::Local<v8::Name> name,
                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto value = canvas_native_webgpu_texture_get_mip_level_count(ptr->GetTexture());
        info.GetReturnValue().Set(value);
        return;
    }
    info.GetReturnValue().Set(0);
}


void GPUTextureImpl::Destroy(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUTextureImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    canvas_native_webgpu_texture_destroy(ptr->GetTexture());
}
