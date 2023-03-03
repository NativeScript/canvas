//
// Created by Osei Fortune on 28/04/2022.
//

#include "EXT_color_buffer_half_floatImpl.h"


std::vector<jsi::PropNameID> EXT_color_buffer_half_floatImpl::getPropertyNames(jsi::Runtime &rt) {
    return {
            jsi::PropNameID::forUtf8(rt, std::string("RGBA16F_EXT")),
            jsi::PropNameID::forUtf8(rt, std::string("RGB16F_EXT")),
            jsi::PropNameID::forUtf8(rt, std::string("FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT")),
            jsi::PropNameID::forUtf8(rt, std::string("UNSIGNED_NORMALIZED_EXT")),
    };
}

jsi::Value
EXT_color_buffer_half_floatImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
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