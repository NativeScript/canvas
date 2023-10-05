//
// Created by Osei Fortune on 28/02/2023.
//

#ifndef CANVAS_ANDROID_VECMUTABLEBUFFER_H
#define CANVAS_ANDROID_VECMUTABLEBUFFER_H

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"


template<typename T>
struct VecMutableBuffer{
public:
    VecMutableBuffer(rust::Vec<T> buffer) : vec_(std::move(buffer)) {
        this->buf_ = vec_.data();
        this->buffer_size_ = vec_.size();
        this->size_ = vec_.size() * sizeof(T);
    }

    T *buffer_data() {
        return this->buf_;
    }

    size_t buffer_size() const {
        return this->buffer_size_;
    }

    uint8_t *data() {
        return (uint8_t *) this->buf_;
    }

    size_t size() const {
        return this->size_;
    }

    ~VecMutableBuffer() {
        this->buf_ = nullptr;
    }

private:
    T *buf_;
    size_t size_;
    size_t buffer_size_;
    rust::Vec<T> vec_;
};

#endif //CANVAS_ANDROID_VECMUTABLEBUFFER_H
