//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_compressed_texture_pvrtcImpl.h"

jsi::Value
WEBGL_compressed_texture_pvrtcImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
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

std::vector<jsi::PropNameID>
WEBGL_compressed_texture_pvrtcImpl::getPropertyNames(jsi::Runtime &rt) {
    return {
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_RGB_PVRTC_4BPPV1_IMG")),
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_RGBA_PVRTC_4BPPV1_IMG")),
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_RGB_PVRTC_2BPPV1_IMG")),
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_RGBA_PVRTC_2BPPV1_IMG"))
    };
}
