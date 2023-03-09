//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once

#include "v8runtime/V8Runtime.h"

using namespace facebook;

class JSI_EXPORT WebGLTransformFeedback : public jsi::HostObject {
public:
    WebGLTransformFeedback(uint32_t feedback) : feedback_(feedback) {}

    uint32_t GetFeedback() {
        return this->feedback_;
    }

private:
    uint32_t feedback_;
};
