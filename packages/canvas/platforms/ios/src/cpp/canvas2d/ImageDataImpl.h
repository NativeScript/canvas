//
// Created by Osei Fortune on 27/03/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include <vector>
#include "Common.h"

using namespace org::nativescript::canvas;

class ImageDataImpl {
public:
    ImageDataImpl(rust::Box<ImageData> imageData);

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static ImageDataImpl *GetPointer(const v8::Local<v8::Object>& object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void Ctor(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetWidth(v8::Local<v8::String> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetHeight(v8::Local<v8::String> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetData(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info);

    ImageData &GetImageData() {
        return *this->imageData_;
    }

private:
    rust::Box<ImageData> imageData_;
};
