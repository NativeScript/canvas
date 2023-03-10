//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "v8runtime/V8Runtime.h"

using namespace facebook;

class JSI_EXPORT EXT_shader_texture_lodImpl : public jsi::HostObject {
public:
    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override {
        std::vector<jsi::PropNameID> ret;
        ret.emplace_back(
                jsi::PropNameID::forUtf8(rt, std::string("ext_name")));
        return ret;
    }

    jsi::Value get(jsi::Runtime &runtime, const jsi::PropNameID &name) override {
        auto methodName = name.utf8(runtime);

        if (methodName == "ext_name") {
            return jsi::String::createFromAscii(runtime, "EXT_shader_texture_lod");
        }

        return jsi::Value::undefined();
    }
};
