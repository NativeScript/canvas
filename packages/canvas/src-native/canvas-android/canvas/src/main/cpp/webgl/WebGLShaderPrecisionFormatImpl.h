//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/webgl.rs.h"
#include "v8runtime/V8Runtime.h"
#include <vector>

using namespace facebook;
using namespace org::nativescript::canvas;

class WebGLShaderPrecisionFormatImpl : public jsi::HostObject {
public:
    WebGLShaderPrecisionFormatImpl(rust::Box<WebGLShaderPrecisionFormat> shader);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    WebGLShaderPrecisionFormat &GetShaderPrecisionFormat();

private:
    rust::Box<WebGLShaderPrecisionFormat> shader_;
};

