//
// Created by Osei Fortune on 27/03/2022.
//

#include "ImageDataImpl.h"

using namespace v8;

ImageDataImpl::ImageDataImpl(rust::Box <ImageData> imageData) : imageData_(std::move(imageData)) {

}

ImageDataImpl::~ImageDataImpl() {
    if (this->buffer_.get() != nullptr) {
        this->buffer_->ClearWeak();
        this->buffer_->Reset();
    }
}

ImageDataImpl *ImageDataImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<ImageDataImpl *>(ptr);
}


void ImageDataImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ctorFunc = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "ImageData"), ctorFunc).ToChecked();
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
        auto ret = args.This();
        ret->SetPrivate(context, v8::Private::New(isolate, Helpers::ConvertToV8String(isolate, "class_name")),
                        Helpers::ConvertToV8String(isolate, "ImageData"));
        ImageDataImpl *imageData = new ImageDataImpl(std::move(canvas_native_image_data_create(
                args[0]->Int32Value(context).ToChecked(),
                args[1]->Int32Value(context).ToChecked()
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
//    ImageDataImpl *cptr = new ImageDataImpl(canvas_native_image_data_get_shared_instance(*ptr->imageData_));
    auto data = canvas_native_image_data(*ptr->imageData_);
    auto len = static_cast<size_t>(data.size());
    auto store_data = reinterpret_cast<void *>(data.data());

    auto callback = [](const v8::WeakCallbackInfo<ImageDataImpl> &data) {
        auto value = data.GetParameter();
        delete value;
    };

    auto buffer = v8::ArrayBuffer::New(isolate, store_data, len);
    auto array = v8::Uint8ClampedArray::New(buffer, 0, len);
    ptr->buffer_ = std::make_shared<v8::Persistent<v8::Object>>(isolate, array);
    ptr->buffer_->SetWeak(ptr, callback, v8::WeakCallbackType::kFinalizer);
    info.GetReturnValue().Set(array);
}


v8::Local<v8::Function> ImageDataImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto tmpl = cache->ImageDataCtor.get();
    if (tmpl != nullptr) {
        return tmpl->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();
    auto image_data_tmpl = v8::FunctionTemplate::New(isolate, &Create);
    image_data_tmpl->InstanceTemplate()->SetInternalFieldCount(1);
    image_data_tmpl->SetClassName(Helpers::ConvertToV8String(isolate, "ImageData"));

    auto func = image_data_tmpl->GetFunction(context).ToLocalChecked();

    auto instanceTmpl = image_data_tmpl->InstanceTemplate();

    instanceTmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "width"),
            &GetWidth
    );

    instanceTmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "height"),
            &GetHeight
    );

    instanceTmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "data"),
            &GetData
    );

    cache->ImageDataCtor = std::make_unique<v8::Persistent<v8::Function>>(isolate, func);
    return func;
}

v8::Local<v8::Object> ImageDataImpl::NewInstance(v8::Isolate *isolate, ImageDataImpl *imageData) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctor = GetCtor(isolate);
    auto ret = ctor->NewInstance(context).ToLocalChecked();

    ret->SetPrivate(context, v8::Private::New(isolate, Helpers::ConvertToV8String(isolate, "class_name")),
                    Helpers::ConvertToV8String(isolate, "ImageData"));

    auto ext = v8::External::New(isolate, imageData);

    if (ret->InternalFieldCount() > 0) {
        ret->SetInternalField(0, ext);
    } else {
        ret->SetPrivate(context, v8::Private::New(isolate, Helpers::ConvertToV8String(isolate, "rustPtr")),
                        ext);
    }

    return handle_scope.Escape(ret);
}

const ImageData& ImageDataImpl::GetImageData() {
    return *this->imageData_;
}

ImageData& ImageDataImpl::GetImageDataMut() {
    return *this->imageData_;
}
