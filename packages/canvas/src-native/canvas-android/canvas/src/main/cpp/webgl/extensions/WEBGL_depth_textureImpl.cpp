//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_depth_textureImpl.h"

std::vector<jsi::PropNameID> WEBGL_depth_textureImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("UNSIGNED_INT_24_8_WEBGL")));

    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("ext_name")));

    return ret;
}

jsi::Value WEBGL_depth_textureImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);

    if (methodName == "ext_name") {
        return jsi::String::createFromAscii(runtime, "WEBGL_depth_texture");
    }

    if (methodName == "UNSIGNED_INT_24_8_WEBGL") {
        return {0x84FA};
    }

    return jsi::Value::undefined();
}
