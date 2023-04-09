//
// Created by Osei Fortune on 28/02/2023.
//

#pragma once
#ifndef CANVAS_ANDROID_VECMUTABLEBUFFER_H
#define CANVAS_ANDROID_VECMUTABLEBUFFER_H

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include <NativeScript/JSIRuntime.h>

using namespace facebook;

template<typename T>
struct VecMutableBuffer : jsi::MutableBuffer {
public:
    VecMutableBuffer(rust::Vec<T> buffer) : vec_(std::move(buffer)) {}

    T *buffer_data() {
        return this->vec_.data();
    }

    size_t buffer_size() const {
        return this->vec_.size();
    }
    
    size_t buffer_size() {
        return this->vec_.size();
    }


    uint8_t *data() override {
        return (uint8_t *) this->vec_.data();
    }

    size_t size() const override {
        return this->vec_.size() * sizeof(T);
    }


private:
    rust::Vec<T> vec_;
};

#endif //CANVAS_ANDROID_VECMUTABLEBUFFER_H
