//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_color_buffer_floatImpl.h"

jsi::Value WEBGL_color_buffer_floatImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
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

std::vector<jsi::PropNameID> WEBGL_color_buffer_floatImpl::getPropertyNames(jsi::Runtime &rt) {
    return {
            jsi::PropNameID::forUtf8(rt, std::string("RGBA32F_EXT")),
            jsi::PropNameID::forUtf8(rt, std::string("RGB32F_EXT")),
            jsi::PropNameID::forUtf8(rt, std::string("FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT")),
            jsi::PropNameID::forUtf8(rt, std::string("UNSIGNED_NORMALIZED_EXT"))
    };
}
