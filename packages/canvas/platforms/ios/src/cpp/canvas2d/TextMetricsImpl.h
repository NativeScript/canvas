//
// Created by Osei Fortune on 18/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#import "NativeScript/JSIRuntime.h"
#include <vector>

using namespace facebook;
using namespace org::nativescript::canvas;

class JSI_EXPORT TextMetricsImpl : public jsi::HostObject {
public:
    TextMetricsImpl(rust::Box<TextMetrics> metrics);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    TextMetrics &GetTextMetrics();


private:
    rust::Box<TextMetrics> metrics_;
};
