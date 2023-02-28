//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasRenderingContext2DImpl.h"
#include "../webgl/WebGLRenderingContextBase.h"
#include "RafImpl.h"


CanvasRenderingContext2DImpl::CanvasRenderingContext2DImpl(
        rust::Box<CanvasRenderingContext2D> context) : context_(
        std::move(context)) {
}


std::vector<PropNameID> CanvasRenderingContext2DImpl::getPropertyNames(Runtime &rt) {
    return {
        jsi::PropNameID::forUtf8(rt, std::string("__resize")),
    jsi::PropNameID::forUtf8(rt, std::string("font")),
    jsi::PropNameID::forUtf8(rt, std::string("globalAlpha")),
    jsi::PropNameID::forUtf8(rt, std::string("imageSmoothingEnabled")),
    jsi::PropNameID::forUtf8(rt, std::string("imageSmoothingQuality")),
    jsi::PropNameID::forUtf8(rt, std::string("lineDashOffset")),
    jsi::PropNameID::forUtf8(rt, std::string("lineJoin")),
    jsi::PropNameID::forUtf8(rt, std::string("lineCap")),
    jsi::PropNameID::forUtf8(rt, std::string("miterLimit")),
    jsi::PropNameID::forUtf8(rt, std::string("shadowColor")),
    jsi::PropNameID::forUtf8(rt, std::string("shadowBlur")),
    jsi::PropNameID::forUtf8(rt, std::string("shadowOffsetX")),
    jsi::PropNameID::forUtf8(rt, std::string("shadowOffsetY")),
    jsi::PropNameID::forUtf8(rt, std::string("textAlign")),
    jsi::PropNameID::forUtf8(rt, std::string("globalCompositeOperation")),
    jsi::PropNameID::forUtf8(rt, std::string("fillStyle")),
    jsi::PropNameID::forUtf8(rt, std::string("strokeStyle")),
    jsi::PropNameID::forUtf8(rt, std::string("lineWidth")),
    jsi::PropNameID::forUtf8(rt, std::string("lineDash")),
    jsi::PropNameID::forUtf8(rt, std::string("addHitRegion")),
    jsi::PropNameID::forUtf8(rt, std::string("arc")),
    jsi::PropNameID::forUtf8(rt, std::string("arcTo")),
    jsi::PropNameID::forUtf8(rt, std::string("beginPath")),
    jsi::PropNameID::forUtf8(rt, std::string("bezierCurveTo")),
    jsi::PropNameID::forUtf8(rt, std::string("clearHitRegions")),
    jsi::PropNameID::forUtf8(rt, std::string("clearRect")),
    jsi::PropNameID::forUtf8(rt, std::string("clip")),
    jsi::PropNameID::forUtf8(rt, std::string("closePath")),
    jsi::PropNameID::forUtf8(rt, std::string("createImageData")),
    jsi::PropNameID::forUtf8(rt, std::string("createLinearGradient")),
    jsi::PropNameID::forUtf8(rt, std::string("createPattern")),
    jsi::PropNameID::forUtf8(rt, std::string("drawImage")),
    jsi::PropNameID::forUtf8(rt, std::string("ellipse")),
    jsi::PropNameID::forUtf8(rt, std::string("fill")),
    jsi::PropNameID::forUtf8(rt, std::string("fillRect")),
    jsi::PropNameID::forUtf8(rt, std::string("fillText")),
    jsi::PropNameID::forUtf8(rt, std::string("getImageData")),
    jsi::PropNameID::forUtf8(rt, std::string("getLineDash")),
    jsi::PropNameID::forUtf8(rt, std::string("isPointInPath")),
    jsi::PropNameID::forUtf8(rt, std::string("isPointInStroke")),
    jsi::PropNameID::forUtf8(rt, std::string("lineTo")),
    jsi::PropNameID::forUtf8(rt, std::string("measureText")),
    jsi::PropNameID::forUtf8(rt, std::string("moveTo")),
    jsi::PropNameID::forUtf8(rt, std::string("putImageData")),
    jsi::PropNameID::forUtf8(rt, std::string("quadraticCurveTo")),
    jsi::PropNameID::forUtf8(rt, std::string("rect")),
    jsi::PropNameID::forUtf8(rt, std::string("removeHitRegion")),
    jsi::PropNameID::forUtf8(rt, std::string("resetTransform")),
    jsi::PropNameID::forUtf8(rt, std::string("restore")),
    jsi::PropNameID::forUtf8(rt, std::string("rotate")),
    jsi::PropNameID::forUtf8(rt, std::string("save")),
    jsi::PropNameID::forUtf8(rt, std::string("scale")),
    jsi::PropNameID::forUtf8(rt, std::string("scrollPathIntoView")),
    jsi::PropNameID::forUtf8(rt, std::string("setLineDash")),
    jsi::PropNameID::forUtf8(rt, std::string("setTransform")),
    jsi::PropNameID::forUtf8(rt, std::string("stroke")),
    jsi::PropNameID::forUtf8(rt, std::string("strokeRect")),
    jsi::PropNameID::forUtf8(rt, std::string("strokeText")),
    jsi::PropNameID::forUtf8(rt, std::string("transform")),
    jsi::PropNameID::forUtf8(rt, std::string("translate")),
    jsi::PropNameID::forUtf8(rt, std::string("__toDataURL"))
    };
}


void
CanvasRenderingContext2DImpl::set(Runtime &runtime, const PropNameID &name, const Value &value) {
    auto methodName = name.utf8(runtime);
    if (methodName == "font") {
        auto val = value.asString(runtime).utf8(runtime);
        canvas_native_context_set_font(this->GetContext(), rust::Str(val.c_str(), val.size()));
    } else if (methodName == "globalAlpha") {
        auto alpha = (float) value.asNumber();
        canvas_native_context_set_global_alpha(this->GetContext(), alpha);
    } else if (methodName == "imageSmoothingEnabled") {
        auto imageSmoothingEnabled = value.asBool();
        canvas_native_context_set_image_smoothing_enabled(this->GetContext(),
                                                          imageSmoothingEnabled);
    } else if (methodName == "imageSmoothingQuality") {
        auto imageSmoothingQuality = value.asString(runtime).utf8(runtime);
        canvas_native_context_set_image_smoothing_quality(this->GetContext(),
                                                          rust::Str(imageSmoothingQuality));
    } else if (methodName == "lineDashOffset") {
        auto lineDashOffset = (float) value.asNumber();
        canvas_native_context_set_line_dash_offset(this->GetContext(), lineDashOffset);
    } else if (methodName == "lineJoin") {
        auto join = value.asString(runtime).utf8(runtime);
        canvas_native_context_set_line_join(this->GetContext(),
                                            rust::Str(join.c_str(), join.size()));
    } else if (methodName == "lineCap") {
        auto cap = value.asString(runtime).utf8(runtime);
        canvas_native_context_set_line_cap(this->GetContext(),
                                           rust::Str(cap.c_str(), cap.size()));
    } else if (methodName == "miterLimit") {
        auto miterLimit = (float) value.asNumber();
        canvas_native_context_set_miter_limit(this->GetContext(), miterLimit);
    } else if (methodName == "shadowColor") {
        auto shadowColor = value.asString(runtime).utf8(runtime);
        canvas_native_context_set_shadow_color(this->GetContext(),
                                               rust::Str(shadowColor.c_str(), shadowColor.size()));
    } else if (methodName == "shadowBlur") {
        auto shadowBlur = (float) value.asNumber();
        canvas_native_context_set_shadow_blur(this->GetContext(), shadowBlur);
    } else if (methodName == "shadowOffsetX") {
        auto shadowOffsetX = (float) value.asNumber();
        canvas_native_context_set_shadow_offset_x(this->GetContext(), shadowOffsetX);
    } else if (methodName == "shadowOffsetY") {
        auto shadowOffsetY = (float) value.asNumber();
        canvas_native_context_set_shadow_offset_y(this->GetContext(), shadowOffsetY);
    } else if (methodName == "textAlign") {
        auto textAlign = value.asString(runtime).utf8(runtime);
        canvas_native_context_set_text_align(this->GetContext(),
                                             rust::Str(textAlign.c_str(), textAlign.size()));
    } else if (methodName == "globalCompositeOperation") {
        auto globalCompositeOperation = value.asString(runtime).utf8(runtime);
        canvas_native_context_set_global_composition(this->GetContext(),
                                                     rust::Str(globalCompositeOperation.c_str(),
                                                               globalCompositeOperation.size()));
    } else if (methodName == "fillStyle") {
        if (value.isString()) {
            auto style = value.asString(runtime).utf8(runtime);
            canvas_native_paint_style_set_fill_color_with_string(this->GetContext(),
                                                                 rust::Str(style.c_str(),
                                                                           style.size()));
        } else if (!value.isNull() && !value.isUndefined() && value.isObject()) {
            auto gradient = value.asObject(runtime).asHostObject<CanvasGradient>(runtime);
            if (gradient != nullptr) {
                canvas_native_context_set_fill_style(this->GetContext(), gradient->GetPaintStyle());
                return;
            }

            auto pattern = value.asObject(runtime).asHostObject<CanvasPattern>(runtime);

            if (pattern != nullptr) {
                canvas_native_context_set_fill_style(this->GetContext(), pattern->GetPaintStyle());
                return;
            }
        }
    } else if (methodName == "strokeStyle") {
        if (value.isString()) {
            auto style = value.asString(runtime).utf8(runtime);
            canvas_native_paint_style_set_stroke_color_with_string(this->GetContext(),
                                                                   rust::Str(style.c_str(),
                                                                             style.size()));
        } else if (!value.isNull() && !value.isUndefined() && value.isObject()) {
            auto gradient = value.asObject(runtime).asHostObject<CanvasGradient>(runtime);
            if (gradient != nullptr) {
                canvas_native_context_set_stroke_style(this->GetContext(),
                                                       gradient->GetPaintStyle());
                return;
            }

            auto pattern = value.asObject(runtime).asHostObject<CanvasPattern>(runtime);

            if (pattern != nullptr) {
                canvas_native_context_set_stroke_style(this->GetContext(),
                                                       pattern->GetPaintStyle());
                return;
            }
        }
    } else if (methodName == "lineWidth") {
        auto lineWidth = (float) value.asNumber();
        canvas_native_context_set_line_width(this->GetContext(), lineWidth);
    } else if (methodName == "lineDash") {
        if (!value.isNull() && !value.isUndefined() && value.isObject()) {
            auto lineDashObject = value.asObject(runtime);
            if (lineDashObject.isArray(runtime)) {
                auto lineDash = lineDashObject.asArray(runtime);
                auto size = lineDash.size(runtime);
                std::vector<float> dash;
                dash.reserve(size);
                for (size_t i = 0; i < size; ++i) {
                    auto val = lineDash.getValueAtIndex(runtime, i);
                    dash[i] = (float) value.asNumber();
                }
                canvas_native_context_set_line_dash(this->GetContext(),
                                                    rust::Slice<const float>(dash.data(),
                                                                             dash.size()));
            }
        }
    }


}


Value CanvasRenderingContext2DImpl::get(Runtime &runtime, const PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "__resize") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    auto width = arguments[0].asNumber();
                                                    auto height = arguments[1].asNumber();
                                                    canvas_native_context_resize(this->GetContext(),
                                                                                 static_cast<float>(width),
                                                                                 static_cast<float>(height));
                                                }
        );
    } else if (methodName == "font") {
        auto font = canvas_native_context_get_font(this->GetContext());
        return jsi::String::createFromAscii(runtime, font.data(), font.length());
    } else if (methodName == "globalAlpha") {
        return {(double) canvas_native_context_get_global_alpha(this->GetContext())};
    } else if (methodName == "imageSmoothingEnabled") {
        return {canvas_native_context_get_image_smoothing_enabled(this->GetContext())};
    } else if (methodName == "imageSmoothingQuality") {
        auto quality = canvas_native_context_get_image_smoothing_quality(this->GetContext());
        return jsi::String::createFromAscii(runtime, quality.data(), quality.length());
    } else if (methodName == "lineDashOffset") {
        auto lineDashOffset = canvas_native_context_get_line_dash_offset(this->GetContext());
        return {(double) lineDashOffset};
    } else if (methodName == "lineJoin") {
        auto join = canvas_native_context_get_line_join(this->GetContext());
        return jsi::String::createFromAscii(runtime, join.data(), join.size());
    } else if (methodName == "lineCap") {
        auto cap = canvas_native_context_get_line_cap(this->GetContext());
        return jsi::String::createFromAscii(runtime, cap.data(), cap.size());
    } else if (methodName == "miterLimit") {
        auto miterLimit = canvas_native_context_get_miter_limit(this->GetContext());
        return {(double) miterLimit};
    } else if (methodName == "shadowColor") {
        auto shadowColor = canvas_native_context_get_shadow_color(this->GetContext());
        return jsi::String::createFromAscii(runtime, shadowColor.data(), shadowColor.size());
    } else if (methodName == "shadowBlur") {
        return {(double) canvas_native_context_get_shadow_blur(this->GetContext())};
    } else if (methodName == "shadowOffsetX") {
        return {(double) canvas_native_context_get_shadow_offset_x(this->GetContext())};
    } else if (methodName == "shadowOffsetY") {
        return {(double) canvas_native_context_get_shadow_offset_y(this->GetContext())};
    } else if (methodName == "textAlign") {
        auto textAlign = canvas_native_context_get_text_align(this->GetContext());
        return jsi::String::createFromAscii(runtime, textAlign.data(), textAlign.size());
    } else if (methodName == "globalCompositeOperation") {
        auto globalCompositeOperation = canvas_native_context_get_global_composition(
                this->GetContext());
        return jsi::String::createFromAscii(runtime, globalCompositeOperation.data(),
                                            globalCompositeOperation.size());
    } else if (methodName == "fillStyle") {
        PaintStyleType type = canvas_native_context_get_current_fill_style_type(this->GetContext());
        switch (type) {
            case PaintStyleType::Color: {
                auto color = canvas_native_paint_style_get_current_fill_color_string(
                        this->GetContext());
                return jsi::String::createFromAscii(runtime, color.data(), color.size());
            }
            case PaintStyleType::Gradient: {
                auto style = std::make_shared<CanvasGradient>(
                        canvas_native_context_get_fill_style(this->GetContext()));
                return jsi::Object::createFromHostObject(runtime, style);
            }
            case PaintStyleType::Pattern: {
                auto style = std::make_shared<CanvasPattern>(
                        canvas_native_context_get_fill_style(this->GetContext()));
                return jsi::Object::createFromHostObject(runtime, style);
            }
            case PaintStyleType::None: {
                return Value::undefined();
            }
        }
    } else if (methodName == "strokeStyle") {
        PaintStyleType type = canvas_native_context_get_current_stroke_style_type(
                this->GetContext());
        switch (type) {
            case PaintStyleType::Color: {
                auto color = canvas_native_paint_style_get_current_stroke_color_string(
                        this->GetContext());
                return jsi::String::createFromAscii(runtime, color.data(), color.size());
            }
            case PaintStyleType::Gradient: {
                auto style = std::make_shared<CanvasGradient>(
                        canvas_native_context_get_stroke_style(this->GetContext()));
                return jsi::Object::createFromHostObject(runtime, style);
            }
            case PaintStyleType::Pattern: {
                auto style = std::make_shared<CanvasPattern>(
                        canvas_native_context_get_stroke_style(this->GetContext()));
                return jsi::Object::createFromHostObject(runtime, style);
            }
            case PaintStyleType::None: {
                return Value::undefined();
            }
        }
    } else if (methodName == "lineWidth") {
        auto lineWidth = canvas_native_context_get_line_width(this->GetContext());
        return {(double) lineWidth};
    } else if (methodName == "lineDash") {
        auto lineDash = canvas_native_context_get_line_dash(this->GetContext());
        auto size = lineDash.size();
        auto ret = jsi::Array(runtime, size);
        for (size_t i = 0; i < size; i++) {
            auto item = lineDash[i];
            ret.setValueAtIndex(runtime, i, Value((double) item));
        }
        return ret;
    } else if (methodName == "addHitRegion") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 0,
                                                [](Runtime &runtime, const Value &thisValue,
                                                   const Value *arguments,
                                                   size_t count) -> Value {
                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "arc") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 6,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    auto anti_clockwise = false;
                                                    if (count == 6) {
                                                        anti_clockwise = arguments[5].asBool();
                                                    }
                                                    canvas_native_context_arc(
                                                            this->GetContext(),
                                                            static_cast<float>(arguments[0].asNumber()),
                                                            static_cast<float>(arguments[1].asNumber()),
                                                            static_cast<float>(arguments[2].asNumber()),
                                                            static_cast<float>(arguments[3].asNumber()),
                                                            static_cast<float>(arguments[4].asNumber()),
                                                            anti_clockwise
                                                    );

                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "arcTo") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 5,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    auto x1 = static_cast<float>(arguments[0].asNumber());
                                                    auto y1 = static_cast<float>(arguments[1].asNumber());
                                                    auto x2 = static_cast<float>(arguments[2].asNumber());
                                                    auto y2 = static_cast<float>(arguments[3].asNumber());
                                                    auto radius = static_cast<float>(arguments[4].asNumber());
                                                    canvas_native_context_arc_to(
                                                            this->GetContext(),
                                                            x1,
                                                            y1,
                                                            x2,
                                                            y2,
                                                            radius
                                                    );

                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "beginPath") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 0,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    canvas_native_context_begin_path(
                                                            this->GetContext());
                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "bezierCurveTo") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 6,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    canvas_native_context_bezier_curve_to(
                                                            this->GetContext(),
                                                            static_cast<float>(arguments[0].asNumber()),
                                                            static_cast<float>(arguments[1].asNumber()),
                                                            static_cast<float>(arguments[2].asNumber()),
                                                            static_cast<float>(arguments[3].asNumber()),
                                                            static_cast<float>(arguments[4].asNumber()),
                                                            static_cast<float>(arguments[5].asNumber())
                                                    );
                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "clearHitRegions") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 0,
                                                [](Runtime &runtime, const Value &thisValue,
                                                   const Value *arguments,
                                                   size_t count) -> Value {
                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "clearRect") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 4,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    canvas_native_context_clear_rect(
                                                            this->GetContext(),
                                                            static_cast<float>(arguments[0].asNumber()),
                                                            static_cast<float>(arguments[1].asNumber()),
                                                            static_cast<float>(arguments[2].asNumber()),
                                                            static_cast<float>(arguments[3].asNumber())
                                                    );
                                                    this->UpdateInvalidateState();

                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "clip") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 1,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count == 0) {
                                                        std::string rule("nonzero");
                                                        canvas_native_context_clip_rule(
                                                                this->GetContext(),
                                                                rust::Str(rule.c_str(),
                                                                          rule.size()));
                                                    } else if (arguments[0].isString()) {
                                                        auto val = arguments[0].asString(
                                                                runtime).utf8(runtime);
                                                        canvas_native_context_clip_rule(
                                                                this->GetContext(),
                                                                rust::Str(val.c_str(), val.size()));
                                                    }

                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "closePath") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 0,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    canvas_native_context_close_path(
                                                            this->GetContext());
                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "createImageData") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [](Runtime &runtime, const Value &thisValue,
                                                   const Value *arguments,
                                                   size_t count) -> Value {


                                                    if (count == 1 && arguments[0].isObject()) {

                                                        auto object = arguments[0].asObject(
                                                                runtime).asHostObject<ImageDataImpl>(
                                                                runtime);

                                                        if (object != nullptr) {
                                                            auto width = canvas_native_image_data_get_width(
                                                                    object->GetImageData());
                                                            auto height = canvas_native_image_data_get_height(
                                                                    object->GetImageData());
                                                            auto data = std::make_shared<ImageDataImpl>(
                                                                    canvas_native_image_data_create(
                                                                            width, height));
                                                            return jsi::Object::createFromHostObject(
                                                                    runtime, data);
                                                        }

                                                    } else if (count > 1) {

                                                        auto width = (int) arguments[0].asNumber();
                                                        auto height = (int) arguments[1].asNumber();
                                                        auto data = std::make_shared<ImageDataImpl>(
                                                                canvas_native_image_data_create(
                                                                        width, height));
                                                        return jsi::Object::createFromHostObject(
                                                                runtime, data);
                                                    }

                                                    // todo throw ?
                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "createLinearGradient") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 4,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    if (count == 4) {
                                                        auto x0 = static_cast<float>(arguments[0].asNumber());
                                                        auto y0 = static_cast<float>(arguments[1].asNumber());
                                                        auto x1 = static_cast<float>(arguments[2].asNumber());
                                                        auto y1 = static_cast<float>(arguments[3].asNumber());

                                                        auto gradient = canvas_native_context_create_linear_gradient(
                                                                this->GetContext(), x0, y0, x1,
                                                                y1);
                                                        auto ret = std::make_shared<CanvasGradient>(
                                                                std::move(gradient));
                                                        return jsi::Object::createFromHostObject(
                                                                runtime,
                                                                ret);
                                                    }

                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "createPattern") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 0,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count > 1) {
                                                        if (arguments[0].isNull() ||
                                                            arguments[0].isUndefined()) { return Value::undefined(); }

                                                        auto object = arguments[0].asObject(
                                                                runtime);

                                                        auto image_asset = object.asHostObject<ImageAssetImpl>(
                                                                runtime);

                                                        if (image_asset != nullptr) {
                                                            auto rep = arguments[1].asString(
                                                                    runtime).utf8(runtime);
                                                            rust::Box<PaintStyle> pattern = canvas_native_context_create_pattern_asset(
                                                                    this->GetContext(),
                                                                    image_asset->GetImageAsset(),
                                                                    rust::Str(rep.c_str(),
                                                                              rep.size()));
                                                            auto type = canvas_native_context_get_style_type(
                                                                    *pattern);
                                                            if (type == PaintStyleType::None) {
                                                                return Value::undefined();
                                                            } else {
                                                                auto ret = std::make_shared<CanvasPattern>(
                                                                        std::move(pattern));
                                                                return Object::createFromHostObject(
                                                                        runtime, ret);
                                                            }
                                                        }


                                                        auto canvas_2d = object.asHostObject<CanvasRenderingContext2DImpl>(
                                                                runtime);
                                                        if (canvas_2d != nullptr) {
                                                            auto rep = arguments[1].asString(
                                                                    runtime).utf8(runtime);
                                                            rust::Box<PaintStyle> pattern = canvas_native_context_create_pattern_canvas2d(
                                                                    this->GetContext(),
                                                                    canvas_2d->GetContext(),
                                                                    rust::Str(rep.c_str(),
                                                                              rep.size()));
                                                            auto type = canvas_native_context_get_style_type(
                                                                    *pattern);
                                                            if (type == PaintStyleType::None) {
                                                                return Value::undefined();
                                                            } else {
                                                                auto ret = std::make_shared<CanvasPattern>(
                                                                        std::move(pattern));
                                                                return jsi::Object::createFromHostObject(
                                                                        runtime, ret);
                                                            }
                                                        }

                                                        /*
                                                        auto webgl = object.asHostObject<W>(runtime);

                                                        if (type == ObjectType::WebGLRenderingContext ||
                                                                  type == ObjectType::WebGL2RenderingContext) {
                                                           auto source = WebGLRenderingContextBase::GetPointerBase(image);
                                                           auto rep = Helpers::ConvertFromV8String(isolate, args[1]);
                                                           rust::Box<PaintStyle> pattern = canvas_native_context_create_pattern_webgl(
                                                                   source->GetState(),
                                                                   ptr->GetContext(),
                                                                   rust::Str(rep.c_str(),
                                                                             rep.size()));
                                                           auto type = canvas_native_context_get_style_type(*pattern);
                                                           if (type == PaintStyleType::None) {
                                                               args.GetReturnValue().SetUndefined();
                                                           } else {
                                                               args.GetReturnValue().Set(CanvasPattern::NewInstance(isolate, std::move(pattern)));
                                                           }
                                                           return;
                                                       }


                                                        */


                                                    }


                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "createRadialGradient") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 6,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    if (count == 6) {
                                                        auto x0 = static_cast<float>(arguments[0].asNumber());
                                                        auto y0 = static_cast<float>(arguments[1].asNumber());
                                                        auto r0 = static_cast<float>(arguments[2].asNumber());
                                                        auto x1 = static_cast<float>(arguments[3].asNumber());
                                                        auto y1 = static_cast<float>(arguments[4].asNumber());
                                                        auto r1 = static_cast<float>(arguments[5].asNumber());

                                                        auto gradient = canvas_native_context_create_radial_gradient(
                                                                this->GetContext(), x0, y0, r0,
                                                                x1, y1, r1);
                                                        auto ret = std::make_shared<CanvasGradient>(
                                                                std::move(gradient));
                                                        return jsi::Object::createFromHostObject(
                                                                runtime, ret);
                                                    }

                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "drawFocusIfNeeded") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 0,
                                                [](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "drawImage") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 9,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    if (count == 3) {
                                                        auto image = arguments[0].asObject(runtime);
                                                        auto dx = static_cast<float>(arguments[1].asNumber());
                                                        auto dy = static_cast<float>(arguments[2].asNumber());


                                                        auto image_asset = image.asHostObject<ImageAssetImpl>(
                                                                runtime);

                                                        if (image_asset != nullptr) {
                                                            canvas_native_context_draw_image_dx_dy_asset(
                                                                    this->GetContext(),
                                                                    image_asset->GetImageAsset(),
                                                                    dx, dy);
                                                            this->UpdateInvalidateState();
                                                            return Value::undefined();
                                                        }

                                                        auto image_bitmap = image.asHostObject<ImageBitmapImpl>(
                                                                runtime);
                                                        if (image_bitmap != nullptr) {
                                                            canvas_native_context_draw_image_dx_dy_asset(
                                                                    this->GetContext(),
                                                                    image_bitmap->GetImageAsset(),
                                                                    dx, dy);
                                                            this->UpdateInvalidateState();
                                                        }
                                                    } else if (count == 5) {
                                                        auto image = arguments[0].asObject(runtime);
                                                        auto dx = (float) arguments[1].asNumber();
                                                        auto dy = (float) arguments[2].asNumber();
                                                        auto dWidth = (float) arguments[3].asNumber();
                                                        auto dHeight = (float) arguments[4].asNumber();

                                                        auto image_asset = image.asHostObject<ImageAssetImpl>(
                                                                runtime);
                                                        if (image_asset != nullptr) {
                                                            canvas_native_context_draw_image_dx_dy_dw_dh_asset(
                                                                    this->GetContext(),
                                                                    image_asset->GetImageAsset(),
                                                                    dx, dy,
                                                                    dWidth,
                                                                    dHeight);
                                                            this->UpdateInvalidateState();
                                                            return Value::undefined();
                                                        }

                                                        auto image_bitmap = image.asHostObject<ImageBitmapImpl>(
                                                                runtime);
                                                        if (image_bitmap != nullptr) {
                                                            canvas_native_context_draw_image_dx_dy_dw_dh_asset(
                                                                    this->GetContext(),
                                                                    image_bitmap->GetImageAsset(),
                                                                    dx, dy,
                                                                    dWidth,
                                                                    dHeight);
                                                            this->UpdateInvalidateState();
                                                            return Value::undefined();
                                                        }
                                                    } else if (count == 9) {
                                                        auto image = arguments[0].asObject(runtime);
                                                        auto sx = (float) arguments[1].asNumber();
                                                        auto sy = (float) arguments[2].asNumber();
                                                        auto sWidth = (float) arguments[3].asNumber();
                                                        auto sHeight = (float) arguments[4].asNumber();
                                                        auto dx = (float) arguments[5].asNumber();
                                                        auto dy = (float) arguments[6].asNumber();
                                                        auto dWidth = (float) arguments[7].asNumber();
                                                        auto dHeight = (float) arguments[8].asNumber();


                                                        auto image_asset = image.asHostObject<ImageAssetImpl>(
                                                                runtime);
                                                        if (image_asset != nullptr) {
                                                            canvas_native_context_draw_image_asset(
                                                                    this->GetContext(),
                                                                    image_asset->GetImageAsset(),
                                                                    sx,
                                                                    sy, sWidth, sHeight,
                                                                    dx,
                                                                    dy, dWidth, dHeight);
                                                            this->UpdateInvalidateState();
                                                            return Value::undefined();
                                                        }

                                                        auto image_bitmap = image.asHostObject<ImageBitmapImpl>(
                                                                runtime);
                                                        if (image_bitmap != nullptr) {
                                                            canvas_native_context_draw_image_asset(
                                                                    this->GetContext(),
                                                                    image_bitmap->GetImageAsset(),
                                                                    sx,
                                                                    sy, sWidth, sHeight,
                                                                    dx,
                                                                    dy, dWidth, dHeight);
                                                            this->UpdateInvalidateState();
                                                            return Value::undefined();
                                                        }
                                                    }

                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "ellipse") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 8,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    if (count == 8) {
                                                        auto x = static_cast<float>(arguments[0].asNumber());
                                                        auto y = static_cast<float>(arguments[1].asNumber());
                                                        auto radiusX = static_cast<float>(arguments[2].asNumber());
                                                        auto radiusY = static_cast<float>(arguments[3].asNumber());
                                                        auto rotation = static_cast<float>(arguments[4].asNumber());
                                                        auto startAngle = static_cast<float>(arguments[5].asNumber());
                                                        auto endAngle = static_cast<float>(arguments[6].asNumber());
                                                        auto anticlockwise = false;
                                                        if (arguments[7].isBool()) {
                                                            anticlockwise = arguments[7].asBool();
                                                        }
                                                        canvas_native_context_ellipse(
                                                                this->GetContext(), x, y, radiusX,
                                                                radiusY, rotation,
                                                                startAngle, endAngle,
                                                                anticlockwise);
                                                    }

                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "fill") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    if (count == 2) {
                                                        auto object = arguments[0].asObject(
                                                                runtime).asHostObject<Path2D>(
                                                                runtime);
                                                        auto rule = &arguments[1];
                                                        if (object != nullptr) {
                                                            auto value = rule->asString(
                                                                    runtime).utf8(runtime);
                                                            canvas_native_context_fill_with_path(
                                                                    this->GetContext(),
                                                                    object->GetPath(),
                                                                    rust::Str(value.c_str(),
                                                                              value.size()));
                                                            this->UpdateInvalidateState();
                                                        }
                                                    } else if (count == 1) {
                                                        if (arguments[0].isString()) {
                                                            auto value = arguments[0].asString(
                                                                    runtime).utf8(runtime);
                                                            canvas_native_context_fill(
                                                                    this->GetContext(),
                                                                    rust::Str(value.c_str(),
                                                                              value.size()));
                                                            this->UpdateInvalidateState();
                                                        } else if (arguments[0].isObject()) {
                                                            auto object = arguments[0].asObject(
                                                                    runtime).asHostObject<Path2D>(
                                                                    runtime);
                                                            if (object != nullptr) {
                                                                std::string rule("nonzero");
                                                                canvas_native_context_fill_with_path(
                                                                        this->GetContext(),
                                                                        object->GetPath(),
                                                                        rust::Str(rule.c_str(),
                                                                                  rule.size()));
                                                                this->UpdateInvalidateState();
                                                            }
                                                        }
                                                    } else {
                                                        std::string rule("nonzero");
                                                        canvas_native_context_fill(
                                                                this->GetContext(),
                                                                rust::Str(rule.c_str(),
                                                                          rule.size()));
                                                        this->UpdateInvalidateState();
                                                    }
                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "fillRect") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 4,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    auto x = static_cast<float>(arguments[0].asNumber());
                                                    auto y = static_cast<float>(arguments[1].asNumber());
                                                    auto width = static_cast<float>(arguments[2].asNumber());
                                                    auto height = static_cast<float>(arguments[3].asNumber());
                                                    canvas_native_context_fill_rect(this->GetContext(), x, y, width, height);
                                                    this->UpdateInvalidateState();

                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "fillText") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 4,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    auto text = arguments[0].asString(runtime).utf8(runtime);
                                                    auto x = static_cast<float>(arguments[1].asNumber());
                                                    auto y = static_cast<float>(arguments[2].asNumber());
                                                    float width = -1;
                                                    if (arguments[3].isNumber()) {
                                                        width = static_cast<float>(arguments[3].asNumber());
                                                    }
                                                    canvas_native_context_fill_text(this->GetContext(), rust::Str(text.data(), text.size()), x,
                                                                                    y, width);
                                                    this->UpdateInvalidateState();

                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "getImageData") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 4,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if(count == 4){
                                                        auto sx = static_cast<float>(arguments[0].asNumber());
                                                        auto sy = static_cast<float>(arguments[1].asNumber());
                                                        auto sw = static_cast<float>(arguments[2].asNumber());
                                                        auto sh = static_cast<float>(arguments[3].asNumber());
                                                        auto data = canvas_native_context_get_image_data(this->GetContext(), sx, sy, sw, sh);
                                                        auto object = std::make_shared<ImageDataImpl>(std::move(data));
                                                        return jsi::Object::createFromHostObject(runtime, object);
                                                    }

                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "getLineDash") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 4,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    auto dash = canvas_native_context_get_line_dash(this->GetContext());
                                                    auto size = dash.size();
                                                    auto array = jsi::Array(runtime, size);
                                                    for (int i = 0; i < size; ++i) {
                                                        array.setValueAtIndex(runtime, i, Value((double) dash[i]));
                                                    }
                                                    return array;
                                                }
        );
    } else if (methodName == "isPointInPath") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 4,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count == 2) {
                                                        auto x = static_cast<float>(arguments[0].asNumber());
                                                        auto y = static_cast<float>(arguments[1].asNumber());
                                                        std::string rule("nonzero");
                                                        auto ret = canvas_native_context_is_point_in_path(this->GetContext(), x, y,
                                                                                                          rust::Str(rule.data(), rule.size()));
                                                        return {ret};
                                                    } else if (count == 3 && arguments[2].isString()) {
                                                        auto x = static_cast<float>(arguments[0].asNumber());
                                                        auto y = static_cast<float>(arguments[1].asNumber());
                                                        auto rule = arguments[2].asString(runtime).utf8(runtime);
                                                        auto ret = canvas_native_context_is_point_in_path(this->GetContext(), x, y,
                                                                                                          rust::Str(rule.data(), rule.size()));
                                                        return {ret};
                                                    } else if (count == 4 && arguments[0].isObject() && arguments[3].isString()) {
                                                        auto path = arguments[0].asObject(runtime).asHostObject<Path2D>(runtime);
                                                        auto x = static_cast<float>(arguments[1].asNumber());
                                                        auto y = static_cast<float>(arguments[2].asNumber());
                                                        auto rule = arguments[3].asString(runtime).utf8(runtime);


                                                        if (path != nullptr) {
                                                            auto ret = canvas_native_context_is_point_in_path_with_path(this->GetContext(),
                                                                                                                        path->GetPath(), x, y,
                                                                                                                        rust::Str(rule.data(),
                                                                                                                                  rule.size()));
                                                            return {ret};
                                                        }
                                                    }

                                                    return {false};

                                                }
        );
    } else if (methodName == "isPointInStroke") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 3,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count == 2) {
                                                        auto x = static_cast<float>(arguments[0].asNumber());
                                                        auto y = static_cast<float>(arguments[1].asNumber());
                                                        auto ret = canvas_native_context_is_point_in_stroke(this->GetContext(), x, y);
                                                        return {ret};
                                                    } else if (count == 3 && arguments[0].isObject()) {
                                                        auto path = arguments[0].asObject(runtime).asHostObject<Path2D>(runtime);
                                                        auto x = static_cast<float>(arguments[1].asNumber());
                                                        auto y = static_cast<float>(arguments[2].asNumber());
                                                        if (path != nullptr) {
                                                            auto ret = canvas_native_context_is_point_in_stroke_with_path(this->GetContext(),
                                                                                                                          path->GetPath(), x,
                                                                                                                          y);
                                                            return  {ret};
                                                        }
                                                    }

                                                    return {false};

                                                }
        );
    } else if (methodName == "lineTo") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count > 1) {
                                                        auto x = static_cast<float>(arguments[0].asNumber());
                                                        auto y = static_cast<float>(arguments[1].asNumber());
                                                        canvas_native_context_line_to(this->GetContext(), x, y);
                                                    }

                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "measureText") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    auto text = arguments[0].asString(runtime).utf8(runtime);
                                                    auto metrics = canvas_native_context_measure_text(this->GetContext(),
                                                                                                      rust::Str(text.c_str(), text.size()));

                                                    auto object = std::make_shared<TextMetricsImpl>(std::move(metrics));

                                                    return jsi::Object::createFromHostObject(runtime, object);
                                                }
        );
    } else if (methodName == "moveTo") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count > 1) {
                                                        auto x = static_cast<float>(arguments[0].asNumber());
                                                        auto y = static_cast<float>(arguments[1].asNumber());
                                                        canvas_native_context_move_to(
                                                                this->GetContext(), x, y);
                                                    }

                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "putImageData") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 7,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    auto imageData = arguments[0].asObject(runtime).asHostObject<ImageDataImpl>(runtime);
                                                    if (count == 3) {
                                                        auto dx = static_cast<float>(arguments[1].asNumber());
                                                        auto dy = static_cast<float>(arguments[2].asNumber());
                                                        float dirtyX = 0;
                                                        float dirtyY = 0;
                                                        auto dirtyWidth = (float)canvas_native_image_data_get_width(imageData->GetImageData());
                                                        auto dirtyHeight = (float)canvas_native_image_data_get_height(imageData->GetImageData());
                                                        canvas_native_context_put_image_data(this->GetContext(), imageData->GetImageData(), dx,
                                                                                             dy, dirtyX, dirtyY,
                                                                                             dirtyWidth, dirtyHeight);
                                                        this->UpdateInvalidateState();
                                                    } else if (count == 7) {
                                                        auto dx = static_cast<float>(arguments[1].asNumber());
                                                        auto dy = static_cast<float>(arguments[2].asNumber());
                                                        auto dirtyX = static_cast<float>(arguments[3].asNumber());
                                                        auto dirtyY = static_cast<float>(arguments[4].asNumber());;
                                                        auto dirtyWidth = static_cast<float>(arguments[5].asNumber());
                                                        auto dirtyHeight = static_cast<float>(arguments[6].asNumber());
                                                        canvas_native_context_put_image_data(this->GetContext(), imageData->GetImageData(), dx,
                                                                                             dy, dirtyX, dirtyY,
                                                                                             dirtyWidth, dirtyHeight);
                                                        this->UpdateInvalidateState();
                                                    }

                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "quadraticCurveTo") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 4,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count == 4) {
                                                        auto cpx = static_cast<float>(arguments[0].asNumber());
                                                        auto cpy = static_cast<float>(arguments[1].asNumber());
                                                        auto x = static_cast<float>(arguments[2].asNumber());
                                                        auto y = static_cast<float>(arguments[3].asNumber());
                                                        canvas_native_context_quadratic_curve_to(this->GetContext(), cpx, cpy, x, y);
                                                    }

                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "rect") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 4,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count == 4) {
                                                        auto x = static_cast<float>(arguments[0].asNumber());
                                                        auto y = static_cast<float>(arguments[1].asNumber());
                                                        auto width = static_cast<float>(arguments[2].asNumber());
                                                        auto height = static_cast<float>(arguments[3].asNumber());
                                                        canvas_native_context_rect(this->GetContext(), x, y, width, height);
                                                    }

                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "removeHitRegion") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 0,
                                                [](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "resetTransform") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 0,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    canvas_native_context_reset_transform(this->GetContext());
                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "restore") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 0,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    canvas_native_context_restore(this->GetContext());
                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "rotate") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 1,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count == 1 && arguments[0].isNumber()) {
                                                        canvas_native_context_rotate(this->GetContext(), (float)arguments[0].asNumber());
                                                    }

                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "save") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 0,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    canvas_native_context_save(this->GetContext());
                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "scale") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count == 2) {
                                                        auto x = static_cast<float>(arguments[0].asNumber());
                                                        auto y = static_cast<float>(arguments[1].asNumber());
                                                        canvas_native_context_scale(this->GetContext(), x, y);
                                                    }

                                                    return Value::undefined();
                                                }
        );
    } else if (methodName == "scrollPathIntoView") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 0,
                                                [](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "setLineDash") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 1,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count == 1) {
                                                        auto vec = &arguments[0];
                                                        if (vec->isObject() && vec->asObject(runtime).isArray(runtime)) {
                                                            auto segments = vec->asObject(runtime).asArray(runtime);
                                                            auto len = segments.size(runtime);
                                                            std::vector<float> data;
                                                            for (int i = 0; i < len; ++i) {
                                                                auto item = segments.getValueAtIndex(runtime, i);
                                                                data.push_back(static_cast<float>(item.asNumber()));
                                                            }
                                                            rust::Slice<const float> slice{data.data(), data.size()};
                                                            canvas_native_context_set_line_dash(this->GetContext(), slice);
                                                        }
                                                    }
                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "setTransform") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 6,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {


                                                    if (count == 1 && arguments[0].isObject()) {
                                                        auto matrix = arguments[0].asObject(runtime).asHostObject<MatrixImpl>(runtime);
                                                        if (matrix != nullptr) {
                                                            canvas_native_context_set_transform_matrix(this->GetContext(), matrix->GetMatrix());
                                                        }
                                                    } else if (count == 6) {
                                                        auto a = static_cast<float>(arguments[0].asNumber());
                                                        auto b = static_cast<float>(arguments[1].asNumber());
                                                        auto c = static_cast<float>(arguments[2].asNumber());
                                                        auto d = static_cast<float>(arguments[3].asNumber());
                                                        auto e = static_cast<float>(arguments[4].asNumber());
                                                        auto f = static_cast<float>(arguments[5].asNumber());
                                                        canvas_native_context_set_transform(this->GetContext(), a, b, c, d, e, f);
                                                    }

                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "stroke") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 1,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count == 1 && arguments[0].isObject()) {
                                                        auto path = arguments[0].asObject(runtime).asHostObject<Path2D>(runtime);
                                                        if (path != nullptr) {
                                                            canvas_native_context_stroke_with_path(this->GetContext(), path->GetPath());
                                                            this->UpdateInvalidateState();
                                                        }
                                                    } else {
                                                        canvas_native_context_stroke(this->GetContext());
                                                        this->UpdateInvalidateState();
                                                    }

                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "strokeRect") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 4,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count == 4) {
                                                        auto x = static_cast<float>(arguments[0].asNumber());
                                                        auto y = static_cast<float>(arguments[1].asNumber());
                                                        auto width = static_cast<float>(arguments[2].asNumber());
                                                        auto height = static_cast<float>(arguments[3].asNumber());
                                                        canvas_native_context_stroke_rect(this->GetContext(), x, y, width, height);
                                                        this->UpdateInvalidateState();
                                                    }

                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "strokeText") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 4,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count >= 3) {
                                                        auto text = arguments[0].asString(runtime).utf8(runtime);
                                                        auto x = static_cast<float>(arguments[1].asNumber());
                                                        auto y = static_cast<float>(arguments[2].asNumber());
                                                        float maxWidth = -1;

                                                        if(count > 3){
                                                            maxWidth = static_cast<float>(arguments[3].asNumber());
                                                        }

                                                        canvas_native_context_stroke_text(this->GetContext(), rust::Str(text.c_str(), text.size()),
                                                                                          x, y, maxWidth);
                                                        this->UpdateInvalidateState();
                                                    }
                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "transform") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 6,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count == 6) {
                                                        auto a = static_cast<float>(arguments[0].asNumber());
                                                        auto b = static_cast<float>(arguments[1].asNumber());
                                                        auto c = static_cast<float>(arguments[2].asNumber());
                                                        auto d = static_cast<float>(arguments[3].asNumber());
                                                        auto e = static_cast<float>(arguments[4].asNumber());
                                                        auto f = static_cast<float>(arguments[5].asNumber());
                                                        canvas_native_context_transform(this->GetContext(), a, b, c, d, e, f);
                                                    }

                                                    return Value::undefined();

                                                }
        );
    } else if (methodName == "translate") {
        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {

                                                    if (count == 2) {
                                                        auto x = static_cast<float>(arguments[0].asNumber());
                                                        auto y = static_cast<float>(arguments[1].asNumber());
                                                        canvas_native_context_translate(this->GetContext(), x, y);
                                                    }

                                                    return Value::undefined();

                                                }
        );
    }else if (methodName == "__toDataURL") {

        return Function::createFromHostFunction(runtime,
                                                jsi::PropNameID::forAscii(runtime, methodName), 2,
                                                [this](Runtime &runtime, const Value &thisValue,
                                                       const Value *arguments,
                                                       size_t count) -> Value {
                                                    std::string type("image/png");
                                                    int quality = 92;
                                                    if (arguments[0].isString()) {
                                                        type = arguments[0].asString(runtime).utf8(
                                                                runtime);
                                                    }


                                                    if (arguments[1].isNumber()) {
                                                        quality = (int) arguments[1].asNumber();
                                                    }


                                                    auto data = canvas_native_to_data_url(
                                                            this->GetContext(),
                                                            rust::Str(type.c_str(), type.size()),
                                                            quality);
                                                    return jsi::String::createFromAscii(runtime,
                                                                                        data.data(),
                                                                                        data.size());
                                                }
        );
    }


    return Value::undefined();
}


CanvasRenderingContext2DImpl::~CanvasRenderingContext2DImpl() {
    auto raf = this->raf_.get();
    if (raf != nullptr) {
        canvas_native_raf_stop(raf->GetRaf());
    }
}

void CanvasRenderingContext2DImpl::UpdateInvalidateState() {
    auto raf = this->raf_.get();
    if (raf != nullptr) {
        if (!canvas_native_raf_get_started(raf->GetRaf())) {
            canvas_native_raf_start(raf->GetRaf());
        }
    }

    this->invalidateState_ = InvalidateState::PENDING;
}

InvalidateState CanvasRenderingContext2DImpl::GetInvalidateState() const {
    return this->invalidateState_;
}

void CanvasRenderingContext2DImpl::SetInvalidateState(InvalidateState state) {
    this->invalidateState_ = state;
}

void CanvasRenderingContext2DImpl::Flush() {
    if (this->GetInvalidateState() == InvalidateState::PENDING) {
        canvas_native_context_flush(*this->context_);
        this->SetInvalidateState(InvalidateState::INVALIDATING);
        auto current = canvas_native_context_gl_make_current(*this->context_);
        auto swapped = canvas_native_context_gl_swap_buffers(*this->context_);
        this->SetInvalidateState(InvalidateState::NONE);
    }
}

void CanvasRenderingContext2DImpl::Flush(intptr_t context) {
    auto ctx = reinterpret_cast<CanvasRenderingContext2DImpl *>(reinterpret_cast<intptr_t *>(context));
    if (ctx != nullptr) {
        ctx->Flush();
    }
}

void CanvasRenderingContext2DImpl::Init(v8::Isolate *isolate) {
    auto ctorFunc = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "CanvasRenderingContext2D"),
                ctorFunc->GetFunction(context).ToLocalChecked());
    auto funcTpl = v8::FunctionTemplate::New(isolate, &InstanceFromPointer);
    global->Set(context, Helpers::ConvertToV8String(isolate, "__getCanvasRenderingContext2DImpl"),
                funcTpl->GetFunction(context).ToLocalChecked());
}

void
CanvasRenderingContext2DImpl::InstanceFromPointer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (args.Length() > 0) {
        auto width = args[0];
        auto height = args[1];
        auto density = args[2];
        auto fontColor = args[3];
        auto ppi = args[4];
        auto direction = args[5];
        auto alpha = args[6];
        auto offscreenValue = args[7];
        bool offscreen = false;
        if (args.Length() < 6) {
            args.GetReturnValue().SetUndefined();
            return;
        } else {
            auto cache = Caches::Get(isolate);

            if (offscreenValue->IsBoolean() || offscreenValue->IsBooleanObject()) {
                offscreen = offscreenValue->BooleanValue(isolate);
            }

            if (offscreen) {
                auto ctx = canvas_native_context_create_gl_no_window(
                        static_cast<float>(width->NumberValue(context).ToChecked()),
                        static_cast<float>(height->NumberValue(context).ToChecked()),
                        static_cast<float>(density->NumberValue(context).ToChecked()),
                        fontColor->Int32Value(context).ToChecked(),
                        static_cast<float>(ppi->NumberValue(context).ToChecked()),
                        direction->Uint32Value(context).ToChecked(),
                        alpha->BooleanValue(isolate)
                );
                auto ret = NewInstance(isolate, std::move(ctx));
                args.GetReturnValue().Set(ret);
                return;
            } else {
                auto ctx = canvas_native_context_create_with_current(
                        static_cast<float>(width->NumberValue(context).ToChecked()),
                        static_cast<float>(height->NumberValue(context).ToChecked()),
                        static_cast<float>(density->NumberValue(context).ToChecked()),
                        fontColor->Int32Value(context).ToChecked(),
                        static_cast<float>(ppi->NumberValue(context).ToChecked()),
                        direction->Uint32Value(context).ToChecked(),
                        alpha->BooleanValue(isolate)
                );
                auto ret = NewInstance(isolate, std::move(ctx));
                args.GetReturnValue().Set(ret);
                return;
            }

        }
    }
    args.GetReturnValue().SetUndefined();
}

v8::Local<v8::Object>
CanvasRenderingContext2DImpl::NewInstance(v8::Isolate *isolate,
                                          rust::Box<CanvasRenderingContext2D> ctx) {
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto *renderingContext = new CanvasRenderingContext2DImpl(std::move(ctx));


    auto ctx_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(renderingContext));
    auto raf_callback = new OnRafCallback(ctx_ptr, 0);
    auto raf_callback_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(raf_callback));
    auto raf = canvas_native_raf_create(raf_callback_ptr);
    renderingContext->raf_ = std::make_shared<RafImpl>(raf_callback, raf_callback_ptr,
                                                       std::move(raf));


    auto _raf = renderingContext->raf_.get();
    canvas_native_raf_start(_raf->GetRaf());


    auto ret = GetCtor(isolate)->InstanceTemplate()->NewInstance(context).ToLocalChecked();
    Helpers::SetInstanceType(isolate, ret, ObjectType::CanvasRenderingContext2D);
    AddWeakListener(isolate, ret, renderingContext);
    return handle_scope.Escape(ret);
}


CanvasRenderingContext2D &CanvasRenderingContext2DImpl::GetContext() {
    return *this->context_;
}
