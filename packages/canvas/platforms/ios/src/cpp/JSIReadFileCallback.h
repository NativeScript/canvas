//
// Created by Osei Fortune on 17/03/2023.
//

#pragma once
#include "NativeScript/JSIRuntime.h"

using namespace facebook;

struct JSIReadFileCallback {
    std::shared_ptr<jsi::Value> value_;
    std::shared_ptr<jsi::Value> data_;

    JSIReadFileCallback(std::shared_ptr<jsi::Value> value) : value_(std::move(value)) {}
};

