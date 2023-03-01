//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/webgl.rs.h"
#include "v8runtime/V8Runtime.h"

using namespace facebook;

class JSI_EXPORT WebGLFramebuffer: public jsi::HostObject {
public:
WebGLFramebuffer(uint32_t framebuffer) : framebuffer_(framebuffer) {}

uint32_t GetFrameBuffer() {
    return this->framebuffer_;
}

private:
uint32_t framebuffer_;
};

