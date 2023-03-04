//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once

#include "v8runtime/V8Runtime.h"

using namespace facebook;

class JSI_EXPORT WebGLVertexArrayObject : public jsi::HostObject {
public:
    WebGLVertexArrayObject(uint32_t vertexArrayObject) : vertexArrayObject_(vertexArrayObject) {}

    uint32_t GetVertexArrayObject() {
        return this->vertexArrayObject_;
    }

private:
    uint32_t vertexArrayObject_;
};
