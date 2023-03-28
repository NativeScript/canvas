//
// Created by Osei Fortune on 19/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/lib.rs.h"
#include "NativeScript/JSIRuntime.h"
#include <vector>
#include "Helpers.h"

using namespace facebook;
using namespace org::nativescript::canvas;

class JSI_EXPORT TextDecoderImpl : public jsi::HostObject {
public:
    TextDecoderImpl(rust::Box<TextDecoder> decoder);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    TextDecoder &GetTextDecoder();

private:
    rust::Box<TextDecoder> decoder_;
};
