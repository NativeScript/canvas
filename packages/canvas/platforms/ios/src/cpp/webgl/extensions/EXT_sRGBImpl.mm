//
// Created by Osei Fortune on 29/04/2022.
//

#include "EXT_sRGBImpl.h"


std::vector<jsi::PropNameID> EXT_sRGBImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(5);
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("SRGB_EXT")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("SRGB_ALPHA_EXT")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("SRGB8_ALPHA8_EXT")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT")));

    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("ext_name")));
    return ret;
}

jsi::Value EXT_sRGBImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);

    if (methodName == "ext_name") {
        return jsi::String::createFromAscii(runtime, "EXT_sRGB");
    }

    if (methodName == "SRGB_EXT") {
        return {GL_SRGB_EXT};
    } else if (methodName == "SRGB_ALPHA_EXT") {
        return {GL_SRGB_ALPHA_EXT};
    } else if (methodName == "SRGB8_ALPHA8_EXT") {
        return {GL_SRGB8_ALPHA8_EXT};
    } else if (methodName == "FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT") {
        return {GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT};
    }

    return jsi::Value::undefined();
}
