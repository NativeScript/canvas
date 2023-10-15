//
// Created by Osei Fortune on 27/04/2022.
//

#include "WebGLShaderPrecisionFormatImpl.h"

WebGLShaderPrecisionFormatImpl::WebGLShaderPrecisionFormatImpl(
                                                               WebGLShaderPrecisionFormat* shader) : shader_(shader) {}


void WebGLShaderPrecisionFormatImpl::GetRangeMin(v8::Local<v8::String> property,
                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    WebGLShaderPrecisionFormatImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_webgl_shader_precision_format_get_range_min(
            ptr->GetShaderPrecisionFormat()));
}

void WebGLShaderPrecisionFormatImpl::GetRangeMax(v8::Local<v8::String> property,
                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    WebGLShaderPrecisionFormatImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_webgl_shader_precision_format_get_range_max(
            ptr->GetShaderPrecisionFormat()));
}


void WebGLShaderPrecisionFormatImpl::GetPrecision(v8::Local<v8::String> property,
                                                  const v8::PropertyCallbackInfo<v8::Value> &info) {
    WebGLShaderPrecisionFormatImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_webgl_shader_precision_format_get_precision(
            ptr->GetShaderPrecisionFormat()));
}

WebGLShaderPrecisionFormat &WebGLShaderPrecisionFormatImpl::GetShaderPrecisionFormat() {
    return *this->shader_;
}
