//
// Created by Osei Fortune on 29/04/2022.
//

#include "EXT_texture_filter_anisotropicImpl.h"


std::vector<jsi::PropNameID>
EXT_texture_filter_anisotropicImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(3);
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("MAX_TEXTURE_MAX_ANISOTROPY_EXT")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("TEXTURE_MAX_ANISOTROPY_EXT")));

    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("ext_name")));
    return ret;
}


jsi::Value
EXT_texture_filter_anisotropicImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);

    if (methodName == "ext_name") {
        return jsi::String::createFromAscii(runtime, "EXT_texture_filter_anisotropic");
    }

    if (methodName == "MAX_TEXTURE_MAX_ANISOTROPY_EXT") {
        return {GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT};
    } else if (methodName == "TEXTURE_MAX_ANISOTROPY_EXT") {
        return {GL_TEXTURE_MAX_ANISOTROPY_EXT};
    }


    return jsi::Value::undefined();
}
