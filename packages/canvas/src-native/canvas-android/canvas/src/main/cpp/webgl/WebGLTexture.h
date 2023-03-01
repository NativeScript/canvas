//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/webgl.rs.h"
#include "v8runtime/V8Runtime.h"

using namespace facebook;

class JSI_EXPORT WebGLTexture : jsi::HostObject {
public:
    WebGLTexture(uint32_t texture) : texture_(texture) {}

    uint32_t GetTexture() {
        return this->texture_;
    }

private:
    uint32_t texture_;
};
