//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_compressed_texture_pvrtcImpl.h"

std::vector<jsi::PropNameID>
WEBGL_compressed_texture_pvrtcImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(5);
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_RGB_PVRTC_4BPPV1_IMG")));

    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_RGBA_PVRTC_4BPPV1_IMG")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt,
                                     std::string("COMPRESSED_RGB_PVRTC_2BPPV1_IMG")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt,
                                     std::string("COMPRESSED_RGBA_PVRTC_2BPPV1_IMG")));

    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("ext_name")));
    return ret;
}

jsi::Value
WEBGL_compressed_texture_pvrtcImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);

    if (methodName == "ext_name") {
        return jsi::String::createFromAscii(runtime, "WEBGL_compressed_texture_pvrtc");
    }

    if (methodName == "COMPRESSED_RGB_PVRTC_4BPPV1_IMG") {
        return {GL_COMPRESSED_RGB_PVRTC_4BPPV1_IMG};
    } else if (methodName == "COMPRESSED_RGBA_PVRTC_4BPPV1_IMG") {
        return {GL_COMPRESSED_RGBA_PVRTC_4BPPV1_IMG};
    } else if (methodName == "COMPRESSED_RGB_PVRTC_2BPPV1_IMG") {
        return {GL_COMPRESSED_RGB_PVRTC_2BPPV1_IMG};
    } else if (methodName == "COMPRESSED_RGBA_PVRTC_2BPPV1_IMG") {
        return {GL_COMPRESSED_RGBA_PVRTC_2BPPV1_IMG};
    }

    return jsi::Value::undefined();
}
