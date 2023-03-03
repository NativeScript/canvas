//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once

#include "canvas-cxx/src/webgl2.rs.h"
#include "rust/cxx.h"
#include "v8runtime/V8Runtime.h"

using namespace facebook;

class WebGLSyncImpl : public jsi::HostObject {
public:
    WebGLSyncImpl(rust::Box<WebGLSync> sync) : sync_(std::move(sync)) {}

    WebGLSync &GetSync() {
        return *this->sync_;
    }

private:
    rust::Box<WebGLSync> sync_;
};
