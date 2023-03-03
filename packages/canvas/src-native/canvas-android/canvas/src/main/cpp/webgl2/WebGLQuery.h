//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once

#include "v8runtime/V8Runtime.h"

using namespace facebook;

class JSI_EXPORT WebGLQuery : public jsi::HostObject {
public:
    WebGLQuery(uint32_t query) : query_(query) {}

    uint32_t GetQuery() {
        return this->query_;
    }

private:
    uint32_t query_;
};
