//
// Created by Osei Fortune on 18/04/2022.
//

#include "TextMetricsImpl.h"
#include "canvas-cxx/src/lib.rs.h"

TextMetricsImpl::TextMetricsImpl(rust::Box<TextMetrics> metrics) : metrics_(std::move(metrics)) {}

std::vector<jsi::PropNameID> TextMetricsImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(12);
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("width")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("actualBoundingBoxLeft")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("actualBoundingBoxRight")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("actualBoundingBoxAscent")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("actualBoundingBoxDescent")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("fontBoundingBoxAscent")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("fontBoundingBoxDescent")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("emHeightAscent")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("emHeightDescent")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("hangingBaseline")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("alphabeticBaseline")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("ideographicBaseline")));
    return ret;
}

jsi::Value TextMetricsImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "width") {
        return {(double) canvas_native_text_metrics_get_width(this->GetTextMetrics())};
    } else if (methodName == "actualBoundingBoxLeft") {
        return {(double) canvas_native_text_metrics_get_actual_bounding_box_left(
                this->GetTextMetrics())};
    } else if (methodName == "actualBoundingBoxRight") {
        return {(double) canvas_native_text_metrics_get_actual_bounding_box_right(
                this->GetTextMetrics())};
    } else if (methodName == "actualBoundingBoxAscent") {
        return {(double) canvas_native_text_metrics_get_actual_bounding_box_ascent(
                this->GetTextMetrics())};
    } else if (methodName == "actualBoundingBoxDescent") {
        return {(double) canvas_native_text_metrics_get_actual_bounding_box_descent(
                this->GetTextMetrics())};
    } else if (methodName == "fontBoundingBoxAscent") {
        return {(double) canvas_native_text_metrics_get_font_bounding_box_ascent(
                this->GetTextMetrics())};
    } else if (methodName == "fontBoundingBoxDescent") {
        return {(double) canvas_native_text_metrics_get_font_bounding_box_descent(
                this->GetTextMetrics())};
    } else if (methodName == "emHeightAscent") {
        return {(double) canvas_native_text_metrics_get_em_height_ascent(this->GetTextMetrics())};
    } else if (methodName == "emHeightDescent") {
        return {(double) canvas_native_text_metrics_get_em_height_descent(this->GetTextMetrics())};
    } else if (methodName == "alphabeticBaseline") {
        return {(double) canvas_native_text_metrics_get_alphabetic_baseline(
                this->GetTextMetrics())};
    } else if (methodName == "ideographicBaseline") {
        return {(double) canvas_native_text_metrics_get_ideographic_baseline(
                this->GetTextMetrics())};
    }

    return jsi::Value::undefined();
}


TextMetrics &TextMetricsImpl::GetTextMetrics() {
    return *this->metrics_;
}
