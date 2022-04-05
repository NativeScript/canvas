//
// Created by Osei Fortune on 27/03/2022.
//

#include "ImageDataImpl.h"

ImageDataImpl::ImageDataImpl(rust::Box <ImageData> imageData) : imageData_(std::move(imageData)) {

}

void ImageDataImpl::Init(v8::Isolate *isolate) {
    auto ctorFunc = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, v8::String::NewFromUtf8(isolate, "ImageData").ToLocalChecked(), ctorFunc);
}

void ImageDataImpl::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();

    if (!args.IsConstructCall()) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(
                        isolate,
                        "Please use the 'new' operator, this object constructor cannot be called as a function."
                )
        );
        isolate->ThrowException(err);
        return;
    } else {
        if (args.Length() < 2) {
            auto err = v8::Exception::TypeError(
                    Helpers::ConvertToV8String(
                            isolate,
                            "Uncaught TypeError: Failed to construct 'ImageData': 2 arguments required, but only " +
                            std::to_string(args.Length()) + " present."
                    )
            );
            isolate->ThrowException(err);
            return;
        }
        v8::Local<v8::Object> ret = args.This();
        ret->SetPrivate(context, v8::Private::New(isolate, Helpers::ConvertToV8String(isolate, "class_name")),
                        Helpers::ConvertToV8String(isolate, "ImageData"));

        ImageDataImpl *imageData = new ImageDataImpl(std::move(canvas_native_image_data_create(
                static_cast<int32_t>(args[0]->IntegerValue(context).ToChecked()),
                static_cast<int32_t>(args[1]->IntegerValue(context).ToChecked())
        )));

        auto ext = v8::External::New(isolate, imageData);

        if (ret->InternalFieldCount() > 0) {
            ret->SetInternalField(0, ext);
        } else {
            ret->SetPrivate(context, v8::Private::New(isolate, Helpers::ConvertToV8String(isolate, "rustPtr")),
                            ext);
        }

        args.GetReturnValue().Set(ret);
    }
}

ImageDataImpl *ImageDataImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<ImageDataImpl *>(ptr);
}

void ImageDataImpl::GetWidth(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto width = canvas_native_image_data_width(*ptr->imageData_);
    info.GetReturnValue().Set(
            v8::Integer::New(isolate, static_cast<int32_t>(width))
    );
}

void ImageDataImpl::GetHeight(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto width = canvas_native_image_data_height(*ptr->imageData_);
    info.GetReturnValue().Set(
            v8::Integer::New(isolate, static_cast<int32_t>(width))
    );
}

void ImageDataImpl::GetData(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    if (ptr->buffer_.get() != nullptr) {
        info.GetReturnValue().Set(ptr->buffer_.get()->Get(isolate));
        return;
    }
    ImageDataImpl *cptr = new ImageDataImpl(canvas_native_image_data_get_shared_instance(*ptr->imageData_));
    auto data = canvas_native_image_data(*ptr->imageData_);
    auto len = data.size();
    auto store = v8::ArrayBuffer::NewBackingStore(
            static_cast<void *>(data.data()), len, &DisposeBuffer, (void *) cptr
    );
    auto buffer = v8::ArrayBuffer::New(isolate, std::move(store));
    auto array = v8::Uint8ClampedArray::New(buffer, 0, len);
    ptr->buffer_ = std::make_shared<v8::Persistent<v8::Object>>(isolate, array);
    info.GetReturnValue().Set(array);
}

void ImageDataImpl::DisposeBuffer(void *data, size_t length,
                                  void *deleter_data) {
    console_log("DisposeBuffer");
    delete static_cast<ImageDataImpl *>(deleter_data);
}

v8::Local<v8::Function> ImageDataImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto tmpl = cache->ImageDataCtor.get();
    if (tmpl != nullptr) {
        return tmpl->Get(isolate);
    }

    auto context = isolate->GetCurrentContext();
    auto image_data_tmpl = v8::FunctionTemplate::New(isolate);
    auto instance_tmpl = image_data_tmpl->InstanceTemplate();
    instance_tmpl->SetInternalFieldCount(1);
    instance_tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "width"),
            &GetWidth
    );

    instance_tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "height"),
            &GetHeight
    );

    instance_tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "data"),
            &GetData
    );
    auto func = image_data_tmpl->GetFunction(context).ToLocalChecked();
    cache->ImageDataCtor = std::make_unique<v8::Persistent<v8::Function>>(isolate, func);
    return func;
}
