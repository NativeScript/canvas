//
// Created by Osei Fortune on 29/04/2022.
//

#include "OES_texture_half_floatImpl.h"

jsi::Value OES_texture_half_floatImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "HALF_FLOAT_OES") {
        return {GL_HALF_FLOAT_OES};
    }
    return jsi::Value::undefined();
}

std::vector<jsi::PropNameID> OES_texture_half_floatImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("HALF_FLOAT_OES")));
    return ret;
}
