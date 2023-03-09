//
// Created by Osei Fortune on 27/04/2022.
//

#include "WebGLShaderPrecisionFormatImpl.h"

WebGLShaderPrecisionFormatImpl::WebGLShaderPrecisionFormatImpl(
        rust::Box<WebGLShaderPrecisionFormat> shader) : shader_(
        std::move(shader)) {}

std::vector<jsi::PropNameID> WebGLShaderPrecisionFormatImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(3);
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("rangeMin")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("rangeMax")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("precision")));
    return ret;
}

jsi::Value WebGLShaderPrecisionFormatImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);

    if (methodName == "rangeMin") {
        return {canvas_native_webgl_shader_precision_format_get_range_min(
                this->GetShaderPrecisionFormat())};
    } else if (methodName == "rangeMax") {
        return {canvas_native_webgl_shader_precision_format_get_range_max(
                this->GetShaderPrecisionFormat())};
    } else if (methodName == "precision") {
        return {canvas_native_webgl_shader_precision_format_get_precision(
                this->GetShaderPrecisionFormat())};
    }

    return jsi::Value::undefined();
}


WebGLShaderPrecisionFormat &WebGLShaderPrecisionFormatImpl::GetShaderPrecisionFormat() {
    return *this->shader_;
}
