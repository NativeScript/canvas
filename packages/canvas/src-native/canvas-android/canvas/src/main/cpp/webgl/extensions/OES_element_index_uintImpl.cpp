//
// Created by Osei Fortune on 29/04/2022.
//

#include "OES_element_index_uintImpl.h"

std::vector<jsi::PropNameID> OES_element_index_uintImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("UNSIGNED_INT")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("ext_name")));
    return ret;
}


jsi::Value OES_element_index_uintImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);

    if (methodName == "ext_name") {
        return jsi::String::createFromAscii(runtime, "OES_element_index_uint");
    }

    if (methodName == "UNSIGNED_INT") {
        return {GL_UNSIGNED_INT};
    }

    return jsi::Value::undefined();
}
