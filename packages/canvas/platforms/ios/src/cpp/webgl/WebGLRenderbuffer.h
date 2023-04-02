//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#import "NativeScript/JSIRuntime.h"

using namespace facebook;

class JSI_EXPORT WebGLRenderbuffer : public jsi::HostObject {
public:
    WebGLRenderbuffer(uint32_t renderbuffer) : renderbuffer_(renderbuffer) {}

    uint32_t GetRenderBuffer() {
        return this->renderbuffer_;
    }

private:
    uint32_t renderbuffer_;
};

