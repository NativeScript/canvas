//
// Created by Osei Fortune on 27/03/2022.
//

#include "ImageDataImpl.h"


struct ImageDataBuffer : jsi::MutableBuffer {
public:
    explicit ImageDataBuffer(rust::Box<ImageData> imageData) : imageData_(std::move(imageData)) {
        auto slice = canvas_native_image_data_get_data(*this->imageData_);
        this->buf_ = slice.data();
        this->size_ = slice.size();
    }

    size_t size() const override {
        return this->size_;
    }

    uint8_t *data() override {
        return this->buf_;
    }

    ~ImageDataBuffer() override {
        this->buf_ = nullptr;
    }

private:
    uint8_t *buf_;
    size_t size_;
    rust::Box<ImageData> imageData_;
};

ImageDataImpl::ImageDataImpl(rust::Box<ImageData> imageData) : imageData_(std::move(imageData)) {}

std::vector<jsi::PropNameID> ImageDataImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(3);
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("width")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("height")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("data")));
    return ret;
}

jsi::Value ImageDataImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "width") {
        return {canvas_native_image_data_get_width(this->GetImageData())};
    } else if (methodName == "height") {
        return {canvas_native_image_data_get_height(this->GetImageData())};
    } else if (methodName == "data") {
        auto buf = std::make_shared<ImageDataBuffer>(
                canvas_native_image_data_get_shared_instance(this->GetImageData()));

        auto ab = jsi::ArrayBuffer(runtime, buf);


        auto Uint8ClampedArray = runtime.global()
                .getProperty(runtime,
                             "Uint8ClampedArray")
                .asObject(runtime)
                .asFunction(runtime);

        return Uint8ClampedArray.callAsConstructor(
                runtime, ab);
    }
    return jsi::Value::undefined();
}