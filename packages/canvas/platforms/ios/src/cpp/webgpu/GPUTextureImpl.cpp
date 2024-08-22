//
// Created by Osei Fortune on 21/06/2024.
//

#include "GPUTextureImpl.h"
#include "Caches.h"
#include "GPUTextureViewImpl.h"
#include "GPULabel.h"

GPUTextureImpl::GPUTextureImpl(const CanvasGPUTexture *texture) : texture_(texture) {}

const CanvasGPUTexture *GPUTextureImpl::GetTexture() {
    return this->texture_;
}


void GPUTextureImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "GPUTexture"), func).FromJust();
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

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "label"),
            GetLabel
    );

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
            ConvertToV8String(isolate, "format"),
            GetFormat
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

    tmpl->Set(
            ConvertToV8String(isolate, "createView"),
            v8::FunctionTemplate::New(isolate, &CreateView));

    cache->GPUTextureTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}


void
GPUTextureImpl::GetLabel(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto label = canvas_native_webgpu_texture_get_label(ptr->texture_);
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

void
GPUTextureImpl::GetDimension(v8::Local<v8::Name> name,
                             const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto width = canvas_native_webgpu_texture_get_dimension(ptr->GetTexture());
        auto isolate = info.GetIsolate();
        switch (width) {
            case CanvasTextureDimensionD1:
                info.GetReturnValue().Set(ConvertToV8String(isolate, "1d"));
                break;
            case CanvasTextureDimensionD2:
                info.GetReturnValue().Set(ConvertToV8String(isolate, "2d"));
                break;
            case CanvasTextureDimensionD3:
                info.GetReturnValue().Set(ConvertToV8String(isolate, "3d"));
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


void GPUTextureImpl::CreateView(const v8::FunctionCallbackInfo<v8::Value> &args) {
    GPUTextureImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto descVal = args[0];

    CanvasCreateTextureViewDescriptor *descriptor = nullptr;

    CanvasTextureAspect aspect = CanvasTextureAspectAll;

    CanvasImageSubresourceRange range = {
            aspect, 0, -1, 0, -1
    };

    GPULabel label;

    if (descVal->IsObject()) {
        descriptor = new CanvasCreateTextureViewDescriptor{};
        descriptor->label = nullptr;
        auto descObj = descVal.As<v8::Object>();

        v8::Local<v8::Value> aspectVal;

        if (descObj->Get(context, ConvertToV8String(isolate, "aspect")).ToLocal(&aspectVal)) {
            auto aspectStr = ConvertFromV8String(isolate, aspectVal);
            if (aspectStr == "all") {
                aspect = CanvasTextureAspectAll;
            } else if (aspectStr == "stencil-only") {
                aspect = CanvasTextureAspectStencilOnly;
            } else if (aspectStr == "depth-only") {
                aspect = CanvasTextureAspectDepthOnly;
            }
        }


        v8::Local<v8::Value> arrayLayerCountVal;
        if (descObj->Get(context, ConvertToV8String(isolate, "arrayLayerCount")).ToLocal(
                &arrayLayerCountVal) && arrayLayerCountVal->IsInt32()) {
            range.array_layer_count = arrayLayerCountVal->Int32Value(context).FromJust();
        }

        v8::Local<v8::Value> mipLevelCountVal;
        if (descObj->Get(context, ConvertToV8String(isolate, "mipLevelCount")).ToLocal(
                &mipLevelCountVal) && mipLevelCountVal->IsInt32()) {
            range.mip_level_count = mipLevelCountVal->Int32Value(context).FromJust();
        }

        v8::Local<v8::Value> baseArrayLayerVal;
        if (descObj->Get(context, ConvertToV8String(isolate, "baseArrayLayer")).ToLocal(
                &baseArrayLayerVal) && baseArrayLayerVal->IsUint32()) {
            range.base_array_layer = baseArrayLayerVal->Uint32Value(context).FromJust();
        }

        v8::Local<v8::Value> baseMipLevelVal;
        if (descObj->Get(context, ConvertToV8String(isolate, "baseMipLevel")).ToLocal(
                &baseMipLevelVal) && baseMipLevelVal->IsUint32()) {
            range.base_mip_level = baseMipLevelVal->Uint32Value(context).FromJust();
        }

        descriptor->range = &range;


        v8::Local<v8::Value> formatVal;
        descObj->Get(context, ConvertToV8String(isolate, "format")).ToLocal(&formatVal);
        auto formatStr = ConvertFromV8String(isolate, formatVal);

        descriptor->format = CanvasOptionalGPUTextureFormat{
                CanvasOptionalGPUTextureFormatNone
        };

        if (!formatStr.empty()) {
            auto format = canvas_native_webgpu_enum_string_to_gpu_texture(formatStr.c_str());
            descriptor->format = format;
        }

        auto dimension = CanvasOptionalTextureViewDimensionNone;


        v8::Local<v8::Value> labelVal;
        descObj->Get(context, ConvertToV8String(isolate, "label")).ToLocal(&labelVal);

        label = GPULabel(isolate, labelVal);

        descriptor->label = *label;

        v8::Local<v8::Value> dimensionVal;
        if (descObj->Get(context, ConvertToV8String(isolate, "dimension")).ToLocal(&dimensionVal)) {
            auto dimensionStr = ConvertFromV8String(isolate, dimensionVal);
            if (dimensionStr == "1d") {
                dimension = CanvasOptionalTextureViewDimensionD1;
            } else if (dimensionStr == "2d") {
                dimension = CanvasOptionalTextureViewDimensionD2;
            } else if (dimensionStr == "2d-array") {
                dimension = CanvasOptionalTextureViewDimensionD2Array;
            } else if (dimensionStr == "cube") {
                dimension = CanvasOptionalTextureViewDimensionCube;
            } else if (dimensionStr == "cube-array") {
                dimension = CanvasOptionalTextureViewDimensionCubeArray;
            } else if (dimensionStr == "3d") {
                dimension = CanvasOptionalTextureViewDimensionD3;
            }
        }

        descriptor->dimension = dimension;


    }
    auto view = canvas_native_webgpu_texture_create_texture_view(ptr->GetTexture(), descriptor);

    if (descriptor != nullptr) {
        delete descriptor;
        descriptor = nullptr;
    }

    if (view != nullptr) {
        auto ret = GPUTextureViewImpl::NewInstance(isolate, new GPUTextureViewImpl(view));
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetUndefined();

}
