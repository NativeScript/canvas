//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/canvas2d.rs.h"
#include "Helpers.h"
#import "v8runtime/V8Runtime.h"

struct Options {
    bool flipY = false;
    ImageBitmapPremultiplyAlpha premultiplyAlpha = ImageBitmapPremultiplyAlpha::Default;
    ImageBitmapColorSpaceConversion colorSpaceConversion = ImageBitmapColorSpaceConversion::Default;
    ImageBitmapResizeQuality resizeQuality = ImageBitmapResizeQuality::Low;
    float resizeWidth = 0;
    float resizeHeight = 0;
};


using namespace facebook;

class JSI_EXPORT ImageBitmapImpl : public jsi::HostObject {
public:
    ImageBitmapImpl(rust::Box<ImageAsset> asset);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    static Options HandleOptions(jsi::Runtime &runtime, const jsi::Value &options);

    ImageAsset &GetImageAsset();

private:
    rust::Box<ImageAsset> bitmap_;
    bool closed_ = false;
};
