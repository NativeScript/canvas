//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once
#include "v8runtime/V8Runtime.h"

using namespace facebook;

class WebGLSampler : public jsi::HostObject {
public:
    WebGLSampler(uint32_t sampler) : sampler_(sampler) {}

    uint32_t GetSampler() {
        return this->sampler_;
    }

private:
    uint32_t sampler_;
};

