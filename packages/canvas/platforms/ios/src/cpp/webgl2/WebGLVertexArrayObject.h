//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once

#import <NativeScript/JSIRuntime.h>

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
