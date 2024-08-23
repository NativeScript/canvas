//
// Created by Osei Fortune on 22/03/2022.
//

#include "ImageBitmapImpl.h"
#include "Caches.h"

ImageBitmapImpl::ImageBitmapImpl(const ImageAsset *asset)
        : bitmap_(asset) {}

ImageBitmapImpl::~ImageBitmapImpl() {
    canvas_native_image_asset_release(bitmap_);
    this->bitmap_ = nullptr;
}


void ImageBitmapImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "ImageBitmap"), func);
}


void ImageBitmapImpl::FromAsset(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto asset = args[0];
    auto type = GetNativeType(asset);
    auto isolate = args.GetIsolate();
    if (type == NativeType::ImageAsset){
        auto ptr = ImageAssetImpl::GetPointer(asset.As<v8::Object>());
        auto ret = canvas_native_image_asset_reference(ptr->GetImageAsset());
        auto bitmap = new ImageBitmapImpl(ret);
        auto data = v8::External::New(isolate, bitmap);
        auto object = ImageBitmapImpl::NewInstance(isolate, data);
        args.GetReturnValue().Set(object);
        return;
    }
    args.GetReturnValue().SetNull();
}

ImageBitmapImpl *ImageBitmapImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<ImageBitmapImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> ImageBitmapImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->ImageBitmapTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "ImageBitmap"));

    ctorTmpl->Set(ConvertToV8String(isolate, "fromAsset"), v8::FunctionTemplate::New(isolate, FromAsset));

    auto tmpl = ctorTmpl->InstanceTemplate();

    tmpl->SetInternalFieldCount(2);
    tmpl->SetAccessor(
            ConvertToV8String(isolate, "width"), GetWidth);

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "height"), GetHeight);

    tmpl->Set(
            ConvertToV8String(isolate, "close"), v8::FunctionTemplate::New(isolate, Close));

    tmpl->SetAccessor(
            ConvertToV8String(isolate, "__addr"),
            GetAddr);

    tmpl->Set(
            ConvertToV8String(isolate, "__getRef"),
            v8::FunctionTemplate::New(isolate, GetReference));

    cache->ImageBitmapTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void
ImageBitmapImpl::GetWidth(v8::Local<v8::String> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr && !ptr->closed_) {
        auto ret = canvas_native_image_asset_width(ptr->GetImageAsset());
        info.GetReturnValue().Set(ret);
        return;
    }
    info.GetReturnValue().Set(0);
}

void
ImageBitmapImpl::GetHeight(v8::Local<v8::String> name,
                           const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr && !ptr->closed_) {
        auto ret = canvas_native_image_asset_height(ptr->GetImageAsset());
        info.GetReturnValue().Set(ret);
        return;
    }
    info.GetReturnValue().Set(0);
}


void ImageBitmapImpl::Close(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_image_asset_close(ptr->GetImageAsset());

    ptr->closed_ = true;
}

Options
ImageBitmapImpl::HandleOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &options) {
    Options ret;

    if (options->IsObject()) {
        auto context = isolate->GetCurrentContext();
        auto config = options.As<v8::Object>();


        v8::Local<v8::Value> flipYValue;

        config->Get(context, ConvertToV8String(isolate, "flipY")).ToLocal(&flipYValue);

        if (flipYValue->IsBoolean()) {
            ret.flipY = flipYValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> premultiplyAlphaValue;
        config->Get(context, ConvertToV8String(isolate, "premultiplyAlpha")).ToLocal(
                &premultiplyAlphaValue);

        if (premultiplyAlphaValue->IsString()) {
            auto premultiplyAlpha = ConvertFromV8String(isolate, premultiplyAlphaValue);

            if (premultiplyAlpha == "premultiply") {
                ret.premultiplyAlpha = ImageBitmapPremultiplyAlpha::ImageBitmapPremultiplyAlphaPremultiply;
            }

            if (premultiplyAlpha == "none") {
                ret.premultiplyAlpha = ImageBitmapPremultiplyAlpha::ImageBitmapPremultiplyAlphaAlphaNone;
            }
        }

        v8::Local<v8::Value> colorSpaceConversionValue;
        config->Get(context, ConvertToV8String(isolate, "colorSpaceConversion")).ToLocal(
                &colorSpaceConversionValue);

        if (colorSpaceConversionValue->IsString()) {
            auto colorSpaceConversion = ConvertFromV8String(isolate, colorSpaceConversionValue);

            if (colorSpaceConversion == "none") {
                ret.colorSpaceConversion = ImageBitmapColorSpaceConversionNone;
            }
        }

        v8::Local<v8::Value> resizeQualityValue;

        config->Get(context, ConvertToV8String(isolate, "resizeQuality")).ToLocal(
                &resizeQualityValue);

        if (resizeQualityValue->IsString()) {
            auto resizeQuality = ConvertFromV8String(isolate, resizeQualityValue);

            if (resizeQuality == "medium") {
                ret.resizeQuality = ImageBitmapResizeQualityMedium;
            }

            if (resizeQuality == "high") {
                ret.resizeQuality = ImageBitmapResizeQualityHigh;
            }

            if (resizeQuality == "pixelated") {
                ret.resizeQuality = ImageBitmapResizeQualityPixelated;
            }
        }

        v8::Local<v8::Value> resizeWidthValue;
        config->Get(context, ConvertToV8String(isolate, "resizeWidth")).ToLocal(&resizeWidthValue);

        if (resizeWidthValue->IsNumber()) {
            auto val = resizeWidthValue->NumberValue(context).ToChecked();
            ret.resizeWidth = (float) val;
        }

        v8::Local<v8::Value> resizeHeightValue;

        config->Get(context, ConvertToV8String(isolate, "resizeHeight")).ToLocal(
                &resizeHeightValue);

        if (resizeHeightValue->IsNumber()) {
            auto val = resizeHeightValue->NumberValue(context).ToChecked();
            ret.resizeHeight = (float) val;
        }
    }

    return ret;
}


void
ImageBitmapImpl::GetAddr(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto isolate = info.GetIsolate();
        auto ret = std::to_string(canvas_native_image_asset_get_addr(ptr->GetImageAsset()));
        info.GetReturnValue().Set(ConvertToV8String(isolate, ret));
        return;
    }
    info.GetReturnValue().SetEmptyString();
}

void
ImageBitmapImpl::GetReference(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto isolate = args.GetIsolate();
        auto reference = canvas_native_image_asset_reference(ptr->GetImageAsset());
        auto ret = std::to_string(canvas_native_image_asset_get_addr(reference));
        args.GetReturnValue().Set(ConvertToV8String(isolate, ret));
        return;
    }

    args.GetReturnValue().SetEmptyString();
}

const ImageAsset *ImageBitmapImpl::GetImageAsset() {
    return this->bitmap_;
}
