//
// Created by Osei Fortune on 28/04/2022.
//

#include "EXT_blend_minmaxImpl.h"

std::vector<jsi::PropNameID> EXT_blend_minmaxImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(2);
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("MIN_EXT")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("MAX_EXT")));
    return ret;
}

jsi::Value EXT_blend_minmaxImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "MIN_EXT") {
        return {0x8007};
    } else if (methodName == "MAX_EXT") {
        return {0x8008};
    }
    return jsi::Value::undefined();
}
