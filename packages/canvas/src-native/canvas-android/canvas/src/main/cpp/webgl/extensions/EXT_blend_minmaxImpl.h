//
// Created by Osei Fortune on 28/04/2022.
//

#pragma once

#include "v8runtime/V8Runtime.h"
#include <vector>

using namespace facebook;

class JSI_EXPORT EXT_blend_minmaxImpl : public jsi::HostObject {
    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;
};
