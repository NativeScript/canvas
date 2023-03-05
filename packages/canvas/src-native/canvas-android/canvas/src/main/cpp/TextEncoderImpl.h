//
// Created by Osei Fortune on 19/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/canvas2d.rs.h"
#import "v8runtime/V8Runtime.h"
#include <vector>

using namespace facebook;

class JSI_EXPORT TextEncoderImpl : public jsi::HostObject {

public:
    TextEncoderImpl(rust::Box<TextEncoder> encoder);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    TextEncoder &GetTextEncoder();

private:
    rust::Box<TextEncoder> encoder_;
};
