//
// Created by Osei Fortune on 25/02/2023.
//

#ifndef CANVAS_ANDROID_HELPERS_H
#define CANVAS_ANDROID_HELPERS_H

#include "v8runtime/V8Runtime.h"

using namespace facebook::jsi;

inline static int64_t getPointerValue(const facebook::jsi::Value &value, Runtime &runtime) {
    return value.asBigInt(runtime).Int64Value(runtime);
}

#endif //CANVAS_ANDROID_HELPERS_H
