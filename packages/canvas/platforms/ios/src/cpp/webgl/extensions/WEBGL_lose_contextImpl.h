//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "rust/cxx.h"
#import "NativeScript/JSIRuntime.h"
#include "gl.h"
#include "canvas-cxx/src/lib.rs.h"
#include <vector>

using namespace facebook;
using namespace org::nativescript::canvas;

class JSI_EXPORT WEBGL_lose_contextImpl : public jsi::HostObject {
public:
    WEBGL_lose_contextImpl(rust::Box<WEBGL_lose_context> context);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;


    WEBGL_lose_context &GetContext() {
        return *this->context_;
    }

private:
    rust::Box<WEBGL_lose_context> context_;
};
