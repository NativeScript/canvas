//
// Created by Osei Fortune on 28/02/2023.
//

#ifndef CANVAS_ANDROID_VECMUTABLEBUFFER_H
#define CANVAS_ANDROID_VECMUTABLEBUFFER_H

#include "rust/cxx.h"
#include "canvas-cxx/src/canvas2d.rs.h"
#include "v8runtime/V8Runtime.h"

using namespace facebook;

template<typename T>
struct VecMutableBuffer : jsi::MutableBuffer {
public:
    VecMutableBuffer(rust::Vec<T> buffer) : vec_(std::move(buffer)) {
        this->buf_ = vec_.data();
        this->size_ = vec_.size();
    }

    size_t size() const override {
        return this->size_;
    }

    T *data() override {
        return this->buf_;
    }

    ~VecMutableBuffer() override {
        this->buf_ = nullptr;
    }

private:
    uint8_t *buf_;
    size_t size_;
    rust::Vec<T> vec_;
};

#endif //CANVAS_ANDROID_VECMUTABLEBUFFER_H
