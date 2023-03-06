//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "canvas-cxx/src/webgl.rs.h"

#include "rust/cxx.h"
#include "v8runtime/V8Runtime.h"
#include "gl.h"
#include <vector>

using namespace facebook;
using namespace org::nativescript::canvas;

class JSI_EXPORT WEBGL_draw_buffersImpl : public jsi::HostObject {
public:
    WEBGL_draw_buffersImpl(rust::Box<WEBGL_draw_buffers> buffers);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    WEBGL_draw_buffers &GetDrawBuffers() {
        return *this->buffers_;
    }

private:
    rust::Box<WEBGL_draw_buffers> buffers_;
};
