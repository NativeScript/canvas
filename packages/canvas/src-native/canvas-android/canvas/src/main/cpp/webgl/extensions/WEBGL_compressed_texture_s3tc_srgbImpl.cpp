//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_compressed_texture_s3tc_srgbImpl.h"

jsi::Value
WEBGL_compressed_texture_s3tc_srgbImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
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

std::vector<jsi::PropNameID>
WEBGL_compressed_texture_s3tc_srgbImpl::getPropertyNames(jsi::Runtime &rt) {
    return {
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_SRGB_S3TC_DXT1_EXT")),
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT")),
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT")),
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT"))
    };
}
