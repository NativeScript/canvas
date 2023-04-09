//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#import <NativeScript/JSIRuntime.h>

using namespace facebook;

class JSI_EXPORT WebGLShader : public jsi::HostObject {
public:
    WebGLShader(uint32_t shader) : shader_(shader) {}

    uint32_t GetShader() {
        return this->shader_;
    }

private:
    uint32_t shader_;
};

