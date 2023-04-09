//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "rust/cxx.h"
#import <NativeScript/JSIRuntime.h>
#include "gl.h"
#include <vector>

using namespace facebook;

class JSI_EXPORT OES_texture_half_floatImpl : public jsi::HostObject {
    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;
};
