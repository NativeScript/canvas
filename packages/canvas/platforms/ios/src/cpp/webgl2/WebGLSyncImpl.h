//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#import <NativeScript/JSIRuntime.h>

using namespace facebook;
using namespace org::nativescript::canvas;

class WebGLSyncImpl : public jsi::HostObject {
public:
    WebGLSyncImpl(rust::Box<WebGLSync> sync) : sync_(std::move(sync)) {}

    WebGLSync &GetSync() {
        return *this->sync_;
    }

private:
    rust::Box<WebGLSync> sync_;
};
