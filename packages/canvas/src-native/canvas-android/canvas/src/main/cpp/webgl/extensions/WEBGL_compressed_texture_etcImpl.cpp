//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_compressed_texture_etcImpl.h"

jsi::Value
WEBGL_compressed_texture_etcImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "COMPRESSED_R11_EAC") {
        return {GL_COMPRESSED_R11_EAC};
    } else if (methodName == "COMPRESSED_SIGNED_R11_EAC") {
        return {GL_COMPRESSED_SIGNED_R11_EAC};
    } else if (methodName == "COMPRESSED_RG11_EAC") {
        return {GL_COMPRESSED_RG11_EAC};
    } else if (methodName == "COMPRESSED_SIGNED_RG11_EAC") {
        return {GL_COMPRESSED_SIGNED_RG11_EAC};
    } else if (methodName == "COMPRESSED_RGB8_ETC2") {
        return {GL_COMPRESSED_RGB8_ETC2};
    } else if (methodName == "COMPRESSED_RGBA8_ETC2_EAC") {
        return {GL_COMPRESSED_RGBA8_ETC2_EAC};
    } else if (methodName == "COMPRESSED_SRGB8_ETC2") {
        return {GL_COMPRESSED_SRGB8_ETC2};
    } else if (methodName == "COMPRESSED_SRGB8_ALPHA8_ETC2_EAC") {
        return {GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC};
    } else if (methodName == "COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2") {
        return {GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2};
    } else if (methodName == "COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2") {
        return {GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2};
    }

    return jsi::Value::undefined();
}

std::vector<jsi::PropNameID> WEBGL_compressed_texture_etcImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(10);
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_R11_EAC")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_SIGNED_R11_EAC")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_RG11_EAC")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_SIGNED_RG11_EAC")));

    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_RGB8_ETC2")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_RGBA8_ETC2_EAC")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_SRGB8_ETC2")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_SRGB8_ALPHA8_ETC2_EAC")));

    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2")));
    return ret;
}
