//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_compressed_texture_s3tc_srgbImpl.h"


std::vector<jsi::PropNameID>
WEBGL_compressed_texture_s3tc_srgbImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(5);
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_SRGB_S3TC_DXT1_EXT")));

    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt,
                                     std::string("COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt,
                                     std::string("COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT")));

    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("ext_name")));
    return ret;
}


jsi::Value
WEBGL_compressed_texture_s3tc_srgbImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);

    if (methodName == "ext_name") {
        return jsi::String::createFromAscii(runtime, "WEBGL_compressed_texture_s3tc_srgb");
    }

    if (methodName == "COMPRESSED_SRGB_S3TC_DXT1_EXT") {
        return {GL_COMPRESSED_SRGB_S3TC_DXT1_EXT};
    } else if (methodName == "COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT") {
        return {GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT};
    } else if (methodName == "COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT") {
        return {GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT};
    } else if (methodName == "COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT") {
        return {GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT};
    }

    return jsi::Value::undefined();
}
