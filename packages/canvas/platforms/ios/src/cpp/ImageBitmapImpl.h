//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "ImageAssetImpl.h"
#include "Helpers.h"
#include <vector>
#include "ObjectWrapperImpl.h"

struct Options {
    bool flipY = false;
    ImageBitmapPremultiplyAlpha premultiplyAlpha = ImageBitmapPremultiplyAlpha::ImageBitmapPremultiplyAlphaDefault;
    ImageBitmapColorSpaceConversion colorSpaceConversion = ImageBitmapColorSpaceConversion::ImageBitmapColorSpaceConversionDefault;
    ImageBitmapResizeQuality resizeQuality = ImageBitmapResizeQuality::ImageBitmapResizeQualityLow;
    float resizeWidth = 0;
    float resizeHeight = 0;
};


class ImageBitmapImpl : public ObjectWrapperImpl {
public:
    explicit ImageBitmapImpl(const ImageAsset *asset);

    ~ImageBitmapImpl();

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, v8::Local<v8::External> asset) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = ImageBitmapImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();

        auto ptr = asset->Value();
        auto impl = static_cast<ObjectWrapperImpl *>(ptr);

        SetNativeType(impl, NativeType::ImageBitmap);


        object->SetAlignedPointerInInternalField(0, ptr);

        impl->BindFinalizer(isolate, object);

        return scope.Escape(object);
    }

    static Options HandleOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &options);

    const ImageAsset *GetImageAsset();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static ImageBitmapImpl *GetPointer(v8::Local<v8::Object> object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void GetWidth(v8::Local<v8::String> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetHeight(v8::Local<v8::String> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetAddr(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Close(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    const ImageAsset *bitmap_;
    bool closed_ = false;
};
