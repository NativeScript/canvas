//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "rust/cxx.h"
#include "canvas-cxx/src/webgl.rs.h"
#include "v8runtime/V8Runtime.h"
#include <vector>

using namespace facebook;

class JSI_EXPORT ANGLE_instanced_arraysImpl : public jsi::HostObject {
public:
    ANGLE_instanced_arraysImpl(rust::Box<ANGLE_instanced_arrays> arrays);

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    ANGLE_instanced_arrays &GetArrays();

private:
    rust::Box<ANGLE_instanced_arrays> arrays_;
};

