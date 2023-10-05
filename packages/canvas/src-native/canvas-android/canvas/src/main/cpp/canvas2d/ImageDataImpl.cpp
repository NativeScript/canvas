//
// Created by Osei Fortune on 27/03/2022.
//

#include "ImageDataImpl.h"
#include "Helpers.h"
#include "Caches.h"

struct ImageDataBuffer {
public:
    explicit ImageDataBuffer(rust::Box<ImageData> imageData) : imageData_(std::move(imageData)) {
        auto slice = canvas_native_image_data_get_data(*this->imageData_);
        this->buf_ = slice.data();
        this->size_ = slice.size();
    }

    size_t size() const {
        return this->size_;
    }

    uint8_t *data() {
        return this->buf_;
    }

    ~ImageDataBuffer() {
        this->buf_ = nullptr;
    }

private:
    uint8_t *buf_;
    size_t size_;
    rust::Box<ImageData> imageData_;
};

ImageDataImpl::ImageDataImpl(rust::Box<ImageData> imageData) : imageData_(std::move(imageData)) {}

void ImageDataImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "ImageData"), func);
}


ImageDataImpl *ImageDataImpl::GetPointer(const v8::Local<v8::Object>& object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<ImageDataImpl *>(ptr);
}


v8::Local<v8::FunctionTemplate> ImageDataImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->ImageDataTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, Ctor);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "ImageData"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);
    tmpl->SetAccessor(
            ConvertToV8String(isolate, "width"),
            GetWidth);
    tmpl->SetAccessor(
            ConvertToV8String(isolate, "height"),
            GetHeight);
    tmpl->SetAccessor(
            ConvertToV8String(isolate, "data"),
            GetData);
    cache->ImageDataTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}


void ImageDataImpl::Ctor(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto count = args.Length();
    auto value = args[0];
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (count == 1 && !value->IsString()) {
        auto label = value->ToString(isolate->GetCurrentContext()).ToLocalChecked();
        auto arg = ConvertFromV8String(isolate, label);
        auto error = "Failed to construct 'TextEncoder': The encoding label provided (" +
                     arg + "') is invalid";
        isolate->ThrowError(ConvertToV8String(isolate, error));
        return;
    }

    auto ret = args.This();

    if (value->IsNumber()) {
        auto image_data = canvas_native_context_create_image_data(
                (int32_t) value->NumberValue(context).ToChecked(),
                (int32_t) args[1]->NumberValue(context).ToChecked());
        auto object = new ImageDataImpl(std::move(image_data));

        auto ext = v8::External::New(isolate, object);

        ret->SetInternalField(0, ext);

        args.GetReturnValue().Set(ret);
        return;

    } else if (value->IsUint8ClampedArray()) {
        auto arrayObject = value.As<v8::Uint8ClampedArray>();
        auto array = arrayObject->Buffer();
        auto store = array->GetBackingStore();
        auto buffer_data = static_cast<u_int8_t *>(store->Data()) + arrayObject->ByteOffset();
        auto len = arrayObject->Length();

        rust::Slice<const uint8_t> buf(buffer_data, len);

        auto image_data = canvas_native_context_create_image_data_with_data(
                (int32_t) args[1]->NumberValue(context).ToChecked(),
                (int32_t) args[2]->NumberValue(context).ToChecked(), buf);

        auto object = new ImageDataImpl(std::move(image_data));

        auto ext = v8::External::New(isolate, object);

        ret->SetInternalField(0, ext);

        args.GetReturnValue().Set(ret);
        return;
    }

    // TODO throw ?
    args.GetReturnValue().SetUndefined();
}


void
ImageDataImpl::GetWidth(v8::Local<v8::String> name,
                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto ret = canvas_native_image_data_get_width(ptr->GetImageData());
        info.GetReturnValue().Set(ret);
        return;
    }
    info.GetReturnValue().Set(0);
}

void
ImageDataImpl::GetHeight(v8::Local<v8::String> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto ret = canvas_native_image_data_get_height(ptr->GetImageData());
        info.GetReturnValue().Set(ret);
        return;
    }
    info.GetReturnValue().Set(0);
}


void
ImageDataImpl::GetData(v8::Local<v8::String> name,
                       const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto isolate = info.GetIsolate();

        auto data = new ImageDataBuffer(
                canvas_native_image_data_get_shared_instance(ptr->GetImageData()));

        auto length = data->size();
        auto store = v8::ArrayBuffer::NewBackingStore(data->data(), length,
                                                      [](void *data, size_t length,
                                                         void *deleter_data) {
                                                          if (deleter_data != nullptr) {
                                                              delete (ImageDataBuffer *) deleter_data;
                                                          }
                                                      },
                                                      data);

        auto buf = v8::ArrayBuffer::New(isolate, std::move(store));

        auto ret = v8::Uint8ClampedArray::New(buf, 0, length);


        info.GetReturnValue().Set(ret);

        return;
    }
    info.GetReturnValue().SetUndefined();
}
