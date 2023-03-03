//
// Created by Osei Fortune on 27/04/2022.
//

#include "WebGLActiveInfoImpl.h"

WebGLActiveInfoImpl::WebGLActiveInfoImpl(rust::Box<WebGLActiveInfo> info) : info_(
        std::move(info)) {}

std::vector<jsi::PropNameID> WebGLActiveInfoImpl::getPropertyNames(jsi::Runtime &rt) {
    return {
            jsi::PropNameID::forUtf8(rt, std::string("name")),
            jsi::PropNameID::forUtf8(rt, std::string("size")),
            jsi::PropNameID::forUtf8(rt, std::string("type"))
    };
}

jsi::Value WebGLActiveInfoImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "name") {
        auto info_name = canvas_native_webgl_active_info_get_name(this->GetWebGLActiveInfo());
        return jsi::String::createFromAscii(runtime, info_name.data(), info_name.size());
    } else if (methodName == "size") {
        auto size = canvas_native_webgl_active_info_get_size(this->GetWebGLActiveInfo());
        return {size};
    } else if (methodName == "type") {
        auto type = canvas_native_webgl_active_info_get_type(this->GetWebGLActiveInfo());
        return {type};
    }

    return jsi::Value::undefined();
}

WebGLActiveInfo &WebGLActiveInfoImpl::GetWebGLActiveInfo() {
    return *this->info_;
}
