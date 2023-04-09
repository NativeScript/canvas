//
// Created by Osei Fortune on 25/02/2023.
//

#pragma once

#include <memory>
#include "rust/cxx.h"
#import <NativeScript/JSIRuntime.h>

using namespace facebook;

inline static int64_t getPointerValue(jsi::Runtime &runtime, const facebook::jsi::Value &value) {
    return value.asBigInt(runtime).Int64Value(runtime);
}

template<typename T>
inline static rust::Slice<T>
GetArrayBufferData(jsi::Runtime &runtime, facebook::jsi::ArrayBuffer &array) {
    auto buf = array.data(runtime);
    auto size = array.size(runtime);

    rust::Slice<T> slice(reinterpret_cast<T *>(buf), size);
    return std::move(slice);
}

template<typename T>
inline static rust::Slice<T>
GetTypedArrayData(jsi::Runtime &runtime, facebook::jsi::TypedArray &array) {
    auto buf = array.data(runtime);
    auto offset = array.offset(runtime);
    auto size = array.size(runtime);

    auto data = buf + offset;

    rust::Slice<T> slice(reinterpret_cast<T *>(data), (size / sizeof(T)));
    return std::move(slice);
}

template<typename T>
inline static std::shared_ptr<T>
getHostObject(jsi::Runtime &runtime, const facebook::jsi::Value &value) {
    if (value.isObject()) {
        auto valueObject = value.asObject(runtime);
        try {
            if (valueObject.template isHostObject(runtime)) {
                return valueObject.template asHostObject<T>(runtime);
            }
        } catch (...) {}
    }

    return nullptr;
}

template<typename T>
inline static std::shared_ptr<T>
getHostObject(jsi::Runtime &runtime, const facebook::jsi::Object &value) {
    if (value.template isHostObject(runtime)) {
        return value.template asHostObject<T>(runtime);
    }
    return nullptr;
}

