//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include "v8runtime/V8Runtime.h"

using namespace facebook;

class JSI_EXPORT WebGLBuffer : public jsi::HostObject {
public:
    WebGLBuffer(uint32_t buffer) : buffer_(buffer) {}

    uint32_t GetBuffer() {
        return this->buffer_;
    }

private:
    uint32_t buffer_;
};

