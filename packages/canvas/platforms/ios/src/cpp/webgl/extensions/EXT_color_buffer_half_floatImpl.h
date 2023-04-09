//
// Created by Osei Fortune on 28/04/2022.
//

#pragma once

#import <NativeScript/JSIRuntime.h>
#include <vector>

using namespace facebook;

class JSI_EXPORT EXT_color_buffer_half_floatImpl : public jsi::HostObject {
    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;
};

