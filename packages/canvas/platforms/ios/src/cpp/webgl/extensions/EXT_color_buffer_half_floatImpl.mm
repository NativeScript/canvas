//
// Created by Osei Fortune on 28/04/2022.
//

#include "EXT_color_buffer_half_floatImpl.h"


std::vector<jsi::PropNameID> EXT_color_buffer_half_floatImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(5);
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("RGBA16F_EXT")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("RGB16F_EXT")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("UNSIGNED_NORMALIZED_EXT")));

    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("ext_name")));

    return ret;
}

jsi::Value
EXT_color_buffer_half_floatImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);

    if (methodName == "ext_name") {
        return jsi::String::createFromAscii(runtime,"EXT_color_buffer_half_float");
    }

    if (methodName == "RGBA16F_EXT") {
        return {0x881A};
    } else if (methodName == "RGB16F_EXT") {
        return {0x881B};
    } else if (methodName == "FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT") {
        return {0x8211};
    } else if (methodName == "UNSIGNED_NORMALIZED_EXT") {
        return {0x8C17};
    }

    return jsi::Value::undefined();
}
