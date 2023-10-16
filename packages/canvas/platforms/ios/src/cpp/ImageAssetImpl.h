//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include <unistd.h>
#include <thread>
#include <vector>
#include "Helpers.h"
#include "Common.h"
#include "NSOperationQueueWrapper.h"

class ImageAssetImpl{
public:
    ImageAssetImpl(ImageAsset* asset);
    
    ~ImageAssetImpl();

    ImageAsset* GetImageAsset();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static ImageAssetImpl *GetPointer(const v8::Local<v8::Object>& object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void Ctor(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetWidth(v8::Local<v8::String> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetHeight(v8::Local<v8::String> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetError(v8::Local<v8::String> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Scale(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FromUrlSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FromUrlCb(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FromFileSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FromFileCb(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FromBytesSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FromBytesCb(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SaveSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SaveCb(const v8::FunctionCallbackInfo<v8::Value> &args);


private:
    ImageAsset* asset_;
};
