//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "Common.h"
#include "Caches.h"
#include "Helpers.h"
#include "rust/cxx.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

class ImageAssetImpl {
public:
    ImageAssetImpl(rust::Box <ImageAsset> asset);

    static void Init(v8::Isolate *isolate);

    static void Create(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetWidth(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetHeight(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetError(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void FlipX(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FlipY(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Scale(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FromUrl(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FromUrlAsync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FromFile(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FromFileAsync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FromBytes(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FromBytesAsync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Save(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SaveAsync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static ImageAssetImpl *GetPointer(v8::Local<v8::Object> object);

    ImageAsset& GetImageAsset();

private:
    rust::Box <ImageAsset> asset_;

    static v8::Local<v8::Function> GetCtor(v8::Isolate *isolate);
};
