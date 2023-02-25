//
// Created by Osei Fortune on 27/03/2022.
//

#include "ImageDataImpl.h"


using namespace v8;

ImageDataImpl::ImageDataImpl(rust::Box <ImageData> imageData) : imageData_(std::move(imageData)) {

}

ImageDataImpl::~ImageDataImpl() {
    if (this->buffer_.get() != nullptr) {
        this->buffer_->Reset();
    }
}

ImageDataImpl *ImageDataImpl::GetPointer(const v8::Local<v8::Object> &object) {
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
    global->Set(context, Helpers::ConvertToV8String(isolate, "ImageData"),
                ctorFunc->GetFunction(context).ToLocalChecked()).ToChecked();
}

void ImageDataImpl::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
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
        Helpers::SetInstanceType(isolate, ret, ObjectType::ImageData);

        ImageDataImpl *imageData = new ImageDataImpl(std::move(canvas_native_image_data_create(
                args[0]->Int32Value(context).ToChecked(),
                args[1]->Int32Value(context).ToChecked()
        )));
        AddWeakListener(isolate, ret, imageData);
        args.GetReturnValue().Set(ret);
    }
}

void ImageDataImpl::GetWidth(v8::Local<v8::String> name,
                             const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto width = canvas_native_image_data_get_width(*ptr->imageData_);
    info.GetReturnValue().Set(
            v8::Integer::New(isolate, static_cast<int32_t>(width))
    );
}

void ImageDataImpl::GetHeight(v8::Local<v8::String> name,
                              const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto width = canvas_native_image_data_get_height(*ptr->imageData_);
    info.GetReturnValue().Set(
            v8::Integer::New(isolate, static_cast<int32_t>(width))
    );
}

void ImageDataImpl::GetData(v8::Local<v8::String> name,
                            const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    if (ptr->buffer_.get() != nullptr) {
        info.GetReturnValue().Set(ptr->buffer_.get()->Get(isolate));
        return;
    }
    ImageDataImpl *cptr = new ImageDataImpl(
            canvas_native_image_data_get_shared_instance(*ptr->imageData_));
    auto data = canvas_native_image_data_get_data(*ptr->imageData_);
    auto len = static_cast<size_t>(data.size());
    auto store_data = reinterpret_cast<void *>(data.data());


    auto callback = [](void *data, size_t length,
                       void *deleter_data) {
        delete reinterpret_cast<ImageDataImpl *>(deleter_data);
    };

    auto store = v8::ArrayBuffer::NewBackingStore(store_data, len, callback, cptr);

    auto buffer = v8::ArrayBuffer::New(isolate, std::move(store));

    auto array = v8::Uint8ClampedArray::New(buffer, 0, len);

    info.GetReturnValue().Set(array);
}


v8::Local<v8::FunctionTemplate> ImageDataImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto tmpl = cache->ImageDataTmpl.get();
    if (tmpl != nullptr) {
        return tmpl->Get(isolate);
    }
    auto image_data_tmpl = v8::FunctionTemplate::New(isolate, &Create);
    image_data_tmpl->InstanceTemplate()->SetInternalFieldCount(1);
    image_data_tmpl->SetClassName(Helpers::ConvertToV8String(isolate, "ImageData"));

    auto instanceTmpl = image_data_tmpl->PrototypeTemplate();

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

    cache->ImageDataTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate,
                                                                                  image_data_tmpl);
    return image_data_tmpl;
}

v8::Local<v8::Object> ImageDataImpl::NewInstance(v8::Isolate *isolate, ImageDataImpl *imageData) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctor = GetCtor(isolate);
    auto ret = ctor->InstanceTemplate()->NewInstance(context).ToLocalChecked();
    Helpers::SetInstanceType(isolate, ret, ObjectType::ImageData);
    auto ext = v8::External::New(isolate, imageData);
    ret->SetInternalField(0, ext);
    return handle_scope.Escape(ret);
}

ImageData &ImageDataImpl::GetImageData() {
    return *this->imageData_;
}

const float ImageDataImpl::GetWidth() {
    return canvas_native_image_data_get_width(*this->imageData_);
}

const float ImageDataImpl::GetHeight() {
    return canvas_native_image_data_get_height(*this->imageData_);
}
