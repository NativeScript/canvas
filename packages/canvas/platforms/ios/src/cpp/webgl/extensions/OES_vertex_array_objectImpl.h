//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "canvas-cxx/src/lib.rs.h"
#include "rust/cxx.h"
#import "NativeScript/JSIRuntime.h"
#include "gl.h"
#include "../../webgl2/WebGLVertexArrayObject.h"
#include "Helpers.h"
#include <vector>

using namespace facebook;
using namespace org::nativescript::canvas;

class JSI_EXPORT OES_vertex_array_objectImpl : public jsi::HostObject {
public:
    OES_vertex_array_objectImpl(rust::Box<OES_vertex_array_object> object);

    OES_vertex_array_object &GetVertexArrayObject() {
        return *this->object_;
    }

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

private:
    rust::Box<OES_vertex_array_object> object_;
};
