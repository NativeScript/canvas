//
// Created by Osei Fortune on 29/04/2022.
//

#include "EXT_texture_filter_anisotropicImpl.h"

jsi::Value
EXT_texture_filter_anisotropicImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "MAX_TEXTURE_MAX_ANISOTROPY_EXT") {
        return {GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT};
    } else if (methodName == "TEXTURE_MAX_ANISOTROPY_EXT") {
        return {GL_TEXTURE_MAX_ANISOTROPY_EXT};
    }

    return jsi::Value::undefined();
}

std::vector<jsi::PropNameID>
EXT_texture_filter_anisotropicImpl::getPropertyNames(jsi::Runtime &rt) {
    return {
            jsi::PropNameID::forUtf8(rt, std::string("MAX_TEXTURE_MAX_ANISOTROPY_EXT")),
            jsi::PropNameID::forUtf8(rt, std::string("TEXTURE_MAX_ANISOTROPY_EXT"))
    };
}
