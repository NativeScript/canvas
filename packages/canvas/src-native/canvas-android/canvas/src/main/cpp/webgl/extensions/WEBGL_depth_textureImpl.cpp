//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_depth_textureImpl.h"

jsi::Value WEBGL_depth_textureImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "UNSIGNED_INT_24_8_WEBGL") {
        return {0x84FA};
    }

    return jsi::Value::undefined();
}

std::vector<jsi::PropNameID> WEBGL_depth_textureImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("UNSIGNED_INT_24_8_WEBGL")));
    return ret;
}
