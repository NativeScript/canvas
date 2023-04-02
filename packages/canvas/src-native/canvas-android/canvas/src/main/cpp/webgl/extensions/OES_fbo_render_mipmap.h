//
// Created by Osei Fortune on 02/05/2022.
//

#pragma once

#include "rust/cxx.h"
#include "v8runtime/V8Runtime.h"

using namespace facebook;

class JSI_EXPORT OES_fbo_render_mipmapImpl : public jsi::HostObject {
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
            return jsi::String::createFromAscii(runtime, "OES_fbo_render_mipmap");
        }

        return jsi::Value::undefined();
    }
};
