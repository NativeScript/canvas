//
// Created by Osei Fortune on 24/09/2026.
//

#include "GPUExternalTextureImpl.h"
#include "Caches.h"

GPUExternalTextureImpl::GPUExternalTextureImpl(const CanvasGPUExternalTexture *texture)
        : texture_(texture) {}

const CanvasGPUExternalTexture *GPUExternalTextureImpl::GetExternalTexture() {
    return this->texture_.get();
}

GPUExternalTextureImpl *GPUExternalTextureImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0, ObjectWrapperImpl::kInternalFieldTag);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<GPUExternalTextureImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> GPUExternalTextureImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->GPUExternalTextureTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "GPUExternalTexture"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "label"),
            GetLabel
    );

    cache->GPUExternalTextureTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void
GPUExternalTextureImpl::GetLabel(v8::Local<v8::Name> name,
                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    if (ptr != nullptr) {
        auto label = canvas_native_webgpu_external_texture_get_label(ptr->texture_.get());
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
