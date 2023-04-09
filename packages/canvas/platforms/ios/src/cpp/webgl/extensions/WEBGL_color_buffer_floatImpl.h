//
// Created by Osei Fortune on 29/04/2022.
//

#define once

#include "rust/cxx.h"
#import <NativeScript/JSIRuntime.h>
#include "gl.h"
#include <vector>

using namespace facebook;

class JSI_EXPORT WEBGL_color_buffer_floatImpl : public jsi::HostObject {
public:
    std::string ext_name_ = "WEBGL_color_buffer_float";
    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;
};
