//
// Created by Osei Fortune on 27/03/2022.
//

#ifndef CANVAS_NATIVE_IMAGEDATAIMPL_H
#define CANVAS_NATIVE_IMAGEDATAIMPL_H

#include "../Common.h"
#include "../Caches.h"
#include "../Helpers.h"
#include "rust/cxx.h"
#include "canvas-android-v8/src/bridges/image_data.rs.h"

class ImageDataImpl {
public:
    ImageDataImpl(rust::Box<ImageData> imageData);

    static void Init(v8::Isolate *isolate);

    static void Create(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetWidth(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetHeight(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetData(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

private:
    rust::Box<ImageData> imageData_;
    std::shared_ptr<v8::Persistent<v8::Object>> buffer_;
    static v8::Local<v8::Function> GetCtor(v8::Isolate *isolate);
    static ImageDataImpl *GetPointer(v8::Local<v8::Object> object);

    static void DisposeBuffer(void *data, size_t length,
                        void *deleter_data);
};
#endif //CANVAS_NATIVE_IMAGEDATAIMPL_H
