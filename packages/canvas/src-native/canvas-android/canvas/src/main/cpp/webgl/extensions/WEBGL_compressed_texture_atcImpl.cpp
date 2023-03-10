//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_compressed_texture_atcImpl.h"

std::vector<jsi::PropNameID> WEBGL_compressed_texture_atcImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(4);
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_RGB_ATC_WEBGL")));

    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("COMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt,
                                     std::string("COMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL")));

    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("ext_name")));

    return ret;
}


jsi::Value
WEBGL_compressed_texture_atcImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);

    if (methodName == "ext_name") {
        return jsi::String::createFromAscii(runtime, "WEBGL_compressed_texture_atc");
    }


    if (methodName == "COMPRESSED_RGB_ATC_WEBGL") {
        return {GL_ATC_RGB_AMD};
    } else if (methodName == "COMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL") {
        return {GL_ATC_RGBA_EXPLICIT_ALPHA_AMD};
    } else if (methodName == "COMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL") {
        return {GL_ATC_RGBA_INTERPOLATED_ALPHA_AMD};
    }

    return jsi::Value::undefined();
}

