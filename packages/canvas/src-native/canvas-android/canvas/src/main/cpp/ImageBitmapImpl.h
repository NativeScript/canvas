//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include "Helpers.h"
#import "v8runtime/V8Runtime.h"
#include <vector>
using namespace org::nativescript::canvas;

struct Options {
    bool flipY = false;
    ImageBitmapPremultiplyAlpha premultiplyAlpha = ImageBitmapPremultiplyAlpha::Default;
    ImageBitmapColorSpaceConversion colorSpaceConversion = ImageBitmapColorSpaceConversion::Default;
    ImageBitmapResizeQuality resizeQuality = ImageBitmapResizeQuality::Low;
    float resizeWidth = 0;
    float resizeHeight = 0;
};


class ImageBitmapImpl {
public:
    ImageBitmapImpl(rust::Box<ImageAsset> asset);

    static Options HandleOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &options);

    ImageAsset &GetImageAsset();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static ImageBitmapImpl *GetPointer(v8::Local<v8::Object> object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void GetWidth(v8::Local<v8::String> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetHeight(v8::Local<v8::String> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Close(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    rust::Box<ImageAsset> bitmap_;
    bool closed_ = false;
};
