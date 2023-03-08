//
// Created by Osei Fortune on 28/03/2022.
//

#pragma once
#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include "v8runtime/V8Runtime.h"
#include "Helpers.h"
#include <vector>

using namespace facebook;
using namespace org::nativescript::canvas;

class JSI_EXPORT Path2D : public jsi::HostObject {
public:
    Path2D(rust::Box<Path> path);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    Path &GetPath();

private:
    rust::Box<Path> path_;
};
