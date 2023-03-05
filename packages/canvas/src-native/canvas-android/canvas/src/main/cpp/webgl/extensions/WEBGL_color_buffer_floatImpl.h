//
// Created by Osei Fortune on 29/04/2022.
//

#define once

#include "rust/cxx.h"
#include "v8runtime/V8Runtime.h"
#include "gl.h"
#include <vector>

using namespace facebook;

class JSI_EXPORT WEBGL_color_buffer_floatImpl : public jsi::HostObject {
    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;
};
