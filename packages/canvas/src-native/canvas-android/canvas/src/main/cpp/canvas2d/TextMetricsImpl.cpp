//
// Created by Osei Fortune on 18/04/2022.
//

#include "TextMetricsImpl.h"


TextMetricsImpl::TextMetricsImpl(rust::Box<TextMetrics> metrics) : metrics_(std::move(metrics)) {}

std::vector<jsi::PropNameID> TextMetricsImpl::getPropertyNames(jsi::Runtime &rt) {
    return {
            jsi::PropNameID::forUtf8(rt, "width"),
            jsi::PropNameID::forUtf8(rt, "actualBoundingBoxLeft"),
            jsi::PropNameID::forUtf8(rt, "actualBoundingBoxRight"),
            jsi::PropNameID::forUtf8(rt, "actualBoundingBoxAscent"),
            jsi::PropNameID::forUtf8(rt, "actualBoundingBoxDescent"),
            jsi::PropNameID::forUtf8(rt, "fontBoundingBoxAscent"),
            jsi::PropNameID::forUtf8(rt, "fontBoundingBoxDescent"),
            jsi::PropNameID::forUtf8(rt, "emHeightAscent"),
            jsi::PropNameID::forUtf8(rt, "emHeightDescent"),
            jsi::PropNameID::forUtf8(rt, "hangingBaseline"),
            jsi::PropNameID::forUtf8(rt, "alphabeticBaseline"),
            jsi::PropNameID::forUtf8(rt, "ideographicBaseline"),
    };
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
