//
// Created by Osei Fortune on 22/03/2022.
//

#include "ImageBitmapImpl.h"
#include "ImageAssetImpl.h"
#include "rust/cxx.h"

ImageBitmapImpl::ImageBitmapImpl(rust::Box<ImageAsset> asset)
        : bitmap_(std::move(asset)) {}

jsi::Value ImageBitmapImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "width") {
        if (this->closed_) {
            return {0};
        }
        return {canvas_native_image_asset_width(this->GetImageAsset())};
    } else if (methodName == "height") {
        if (this->closed_) {
            return {0};
        }
        return {canvas_native_image_asset_height(this->GetImageAsset())};
    } else if (methodName == "close") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         this->closed_ = true;
                                                         return jsi::Value::undefined();
                                                     }
        );
    }
    return jsi::Value::undefined();
}


std::vector<jsi::PropNameID> ImageBitmapImpl::getPropertyNames(jsi::Runtime &rt) {
    return {
            jsi::PropNameID::forUtf8(rt, std::string("width")),
            jsi::PropNameID::forUtf8(rt, std::string("height")),
            jsi::PropNameID::forUtf8(rt, std::string("close"))
    };
}


Options
ImageBitmapImpl::HandleOptions(jsi::Runtime &runtime, const jsi::Value &options) {
    Options ret;

    if (options.isObject()) {
        auto config = options.asObject(runtime);


        auto flipYValue = config.getProperty(runtime, "flipY");
        if (flipYValue.isBool()) {
            ret.flipY = flipYValue.asBool();
        }

        auto premultiplyAlphaValue = config.getProperty(runtime, "premultiplyAlpha");

        if (premultiplyAlphaValue.isString()) {
            auto premultiplyAlpha = premultiplyAlphaValue.asString(runtime).utf8(runtime);

            if (premultiplyAlpha == "premultiply") {
                ret.premultiplyAlpha = ImageBitmapPremultiplyAlpha::Premultiply;
            }

            if (premultiplyAlpha == "none") {
                ret.premultiplyAlpha = ImageBitmapPremultiplyAlpha::None;
            }
        }

        auto colorSpaceConversionValue = config.getProperty(runtime, "colorSpaceConversion");

        if (colorSpaceConversionValue.isString()) {
            auto colorSpaceConversion = colorSpaceConversionValue.asString(runtime).utf8(runtime);

            if (colorSpaceConversion == "none") {
                ret.colorSpaceConversion = ImageBitmapColorSpaceConversion::None;
            }
        }

        auto resizeQualityValue = config.getProperty(runtime, "resizeQuality");

        if (resizeQualityValue.isString()) {
            auto resizeQuality = resizeQualityValue.asString(runtime).utf8(runtime);

            if (resizeQuality == "medium") {
                ret.resizeQuality = ImageBitmapResizeQuality::Medium;
            }

            if (resizeQuality == "high") {
                ret.resizeQuality = ImageBitmapResizeQuality::High;
            }

            if (resizeQuality == "pixelated") {
                ret.resizeQuality = ImageBitmapResizeQuality::Pixelated;
            }
        }

        auto resizeWidthValue = config.getProperty(runtime, "resizeWidth");

        if (resizeWidthValue.isNumber()) {
            auto val = resizeWidthValue.asNumber();
            ret.resizeWidth = (float) val;
        }

        auto resizeHeightValue = config.getProperty(runtime, "resizeHeight");

        if (resizeHeightValue.isNumber()) {
            auto val = resizeHeightValue.asNumber();
            ret.resizeHeight = (float) val;
        }
    }

    return ret;
}

ImageAsset &ImageBitmapImpl::GetImageAsset() {
    return *this->bitmap_;
}
