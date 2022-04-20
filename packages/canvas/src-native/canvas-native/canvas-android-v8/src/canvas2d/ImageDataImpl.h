//
// Created by Osei Fortune on 27/03/2022.
//

#pragma once
#include "../Common.h"
#include "../Caches.h"
#include "../Helpers.h"

class ImageDataImpl {
public:
    ImageDataImpl(rust::Box<ImageData> imageData);

    ~ImageDataImpl();

    static void Init(v8::Isolate *isolate);

    static void Create(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetWidth(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetHeight(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetData(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static ImageDataImpl *GetPointer(v8::Local<v8::Object> object);

    static v8::Local<v8::Object> NewInstance(v8::Isolate* isolate, ImageDataImpl* imageData);

    ImageData& GetImageData();
    const float GetWidth();
    const float GetHeight();
private:
    rust::Box<ImageData> imageData_;
    std::shared_ptr<v8::Persistent<v8::Object>> buffer_;
    static v8::Local<v8::Function> GetCtor(v8::Isolate *isolate);
};
