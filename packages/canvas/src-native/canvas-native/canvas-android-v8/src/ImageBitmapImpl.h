//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "Common.h"
#include "Caches.h"
#include "Helpers.h"

class ImageBitmapImpl {
public:
    ImageBitmapImpl(rust::Box <ImageAsset> asset);

    static void Init(v8::Isolate *isolate);

    static void Create(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetWidth(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetHeight(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Close(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateImageBitmap(const v8::FunctionCallbackInfo<v8::Value> &args);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, rust::Box<ImageAsset> bitmap);

    static ImageBitmapImpl *GetPointer(const v8::Local<v8::Object>& object);

    ImageAsset& GetImageAsset();

private:
    rust::Box <ImageAsset> bitmap_;

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);
};
