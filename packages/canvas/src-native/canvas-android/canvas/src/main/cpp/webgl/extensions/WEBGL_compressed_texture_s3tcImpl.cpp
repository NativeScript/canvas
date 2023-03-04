//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_compressed_texture_s3tcImpl.h"

jsi::Value
WEBGL_compressed_texture_s3tcImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "COMPRESSED_RGB_S3TC_DXT1_EXT") {
        return {GL_COMPRESSED_RGB_S3TC_DXT1_EXT};
    } else if (methodName == "COMPRESSED_RGBA_S3TC_DXT1_EXT") {
        return {GL_COMPRESSED_RGBA_S3TC_DXT1_EXT};
    } else if (methodName == "COMPRESSED_RGBA_S3TC_DXT3_EXT") {
        return {GL_COMPRESSED_RGBA_S3TC_DXT3_EXT};
    } else if (methodName == "COMPRESSED_RGBA_S3TC_DXT5_EXT") {
        return {GL_COMPRESSED_RGBA_S3TC_DXT5_EXT};
    }

    return jsi::Value::undefined();
}

std::vector<jsi::PropNameID> WEBGL_compressed_texture_s3tcImpl::getPropertyNames(jsi::Runtime &rt) {
    return {
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_RGB_S3TC_DXT1_EXT")),
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_RGBA_S3TC_DXT1_EXT")),
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_RGBA_S3TC_DXT3_EXT")),
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_RGBA_S3TC_DXT5_EXT"))
    };
}
