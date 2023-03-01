//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/webgl.rs.h"
#include "v8runtime/V8Runtime.h"

using namespace facebook;

class JSI_EXPORT WebGLRenderbuffer : jsi::HostObject {
public:
    WebGLRenderbuffer(uint32_t renderbuffer) : renderbuffer_(renderbuffer) {}

    uint32_t GetRenderBuffer() {
        return this->renderbuffer_;
    }

private:
    uint32_t renderbuffer_;
};

