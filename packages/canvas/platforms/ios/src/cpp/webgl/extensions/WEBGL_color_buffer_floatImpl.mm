//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_color_buffer_floatImpl.h"


std::vector<jsi::PropNameID> WEBGL_color_buffer_floatImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(5);
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("RGBA32F_EXT")));

    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("RGB32F_EXT")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("UNSIGNED_NORMALIZED_EXT")));

    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("ext_name")));

    return ret;
}

jsi::Value WEBGL_color_buffer_floatImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);

    if (methodName == "ext_name") {
        return jsi::String::createFromAscii(runtime, ext_name_);
    }

    if (methodName == "RGBA32F_EXT") {
        return {GL_RGBA32F_EXT};
    } else if (methodName == "RGB32F_EXT") {
        return {GL_RGB32F_EXT};
    } else if (methodName == "FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT") {
        return {GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT};
    } else if (methodName == "UNSIGNED_NORMALIZED_EXT") {
        return {GL_UNSIGNED_NORMALIZED_EXT};
    }

    return jsi::Value::undefined();
}
