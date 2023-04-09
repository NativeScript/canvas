//
// Created by Osei Fortune on 10/03/2023.
//

#pragma once
#include <NativeScript/JSIRuntime.h>

using namespace facebook;

struct JSICallback {
    std::shared_ptr<jsi::Value> value_;
    std::shared_ptr<jsi::HostObject> data_;

    JSICallback(std::shared_ptr<jsi::Value> value) : value_(std::move(value)) {}
};
