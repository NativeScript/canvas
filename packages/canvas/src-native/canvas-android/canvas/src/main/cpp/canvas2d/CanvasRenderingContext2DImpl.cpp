//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasRenderingContext2DImpl.h"
#include "../webgl/WebGLRenderingContextBase.h"
#include "../OneByteStringResource.h"
#include "RafImpl.h"


CanvasRenderingContext2DImpl::CanvasRenderingContext2DImpl(
        rust::Box<CanvasRenderingContext2D> context) : context_(
        std::move(context)) {
    this->buf_ = new char[1024];
}


std::vector<PropNameID> CanvasRenderingContext2DImpl::getPropertyNames(Runtime &rt) {
    std::vector<facebook::jsi::PropNameID> result;
    result.reserve(32);
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("__resize")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("font")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("globalAlpha")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("imageSmoothingEnabled")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("imageSmoothingQuality")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("lineDashOffset")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("lineJoin")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("lineCap")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("miterLimit")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("shadowColor")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("shadowBlur")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("shadowOffsetX")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("shadowOffsetY")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("textAlign")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("globalCompositeOperation")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("fillStyle")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("strokeStyle")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("lineWidth")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("lineDash")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("addHitRegion")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("arc")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("arcTo")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("beginPath")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("bezierCurveTo")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("clearHitRegions")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("clearRect")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("clip")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("closePath")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("createImageData")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("createLinearGradient")));
    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("createPattern")));



    result.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("__toDataURL")));


    return result;
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
                                                [this](Runtime &runtime, const Value &thisValue,
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
                                                [this](Runtime &runtime, const Value &thisValue,
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
                                                        return jsi::Object::createFromHostObject(runtime,
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
                                                        if (arguments[0].isNull() || arguments[0].isUndefined()){return Value::undefined();}

                                                        auto object = arguments[0].asObject(runtime);

                                                        auto image_asset = object.asHostObject<ImageAssetImpl>(runtime);

                                                        if (image_asset != nullptr) {
                                                            auto rep = arguments[1].asString(runtime).utf8(runtime);
                                                            rust::Box<PaintStyle> pattern = canvas_native_context_create_pattern_asset(
                                                                    this->GetContext(),
                                                                    image_asset->GetImageAsset(),
                                                                    rust::Str(rep.c_str(),
                                                                              rep.size()));
                                                            auto type = canvas_native_context_get_style_type(*pattern);
                                                            if (type == PaintStyleType::None) {
                                                                return Value::undefined();
                                                            } else {
                                                                auto ret = std::make_shared<CanvasPattern>(std::move(pattern));
                                                                return Object::createFromHostObject(runtime, ret);
                                                            }
                                                        } else if (type == ObjectType::CanvasRenderingContext2D) {
                                                            auto source = CanvasRenderingContext2DImpl::GetPointer(image);
                                                            auto rep = Helpers::ConvertFromV8String(isolate, args[1]);
                                                            rust::Box<PaintStyle> pattern = canvas_native_context_create_pattern_canvas2d(
                                                                    *source->context_,
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
                                                        } else if (type == ObjectType::WebGLRenderingContext ||
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
                                                    }


                                                    return Value::undefined();
                                                }
        );
    }





    if (methodName == "__toDataURL") {

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
    delete this->buf_;
    auto isolate = v8::Isolate::GetCurrent();
    if (isolate != nullptr) {
        isolate->AdjustAmountOfExternalAllocatedMemory(sizeof(this));
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


void CanvasRenderingContext2DImpl::CreatePattern(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2) {
        auto image = args[0]->ToObject(context).ToLocalChecked();
        // TODO handle other cases
        auto type = Helpers::GetInstanceType(isolate, image);
        if (type == ObjectType::ImageAsset) {
            auto asset = ImageAssetImpl::GetPointer(image);
            auto rep = Helpers::ConvertFromV8String(isolate, args[1]);
            rust::Box<PaintStyle> pattern = canvas_native_context_create_pattern_asset(
                    ptr->GetContext(),
                    asset->GetImageAsset(),
                    rust::Str(rep.c_str(),
                              rep.size()));
            auto type = canvas_native_context_get_style_type(*pattern);
            if (type == PaintStyleType::None) {
                args.GetReturnValue().SetUndefined();
            } else {
                args.GetReturnValue().Set(CanvasPattern::NewInstance(isolate, std::move(pattern)));
            }
            return;
        } else if (type == ObjectType::CanvasRenderingContext2D) {
            auto source = CanvasRenderingContext2DImpl::GetPointer(image);
            auto rep = Helpers::ConvertFromV8String(isolate, args[1]);
            rust::Box<PaintStyle> pattern = canvas_native_context_create_pattern_canvas2d(
                    *source->context_,
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
        } else if (type == ObjectType::WebGLRenderingContext ||
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

        args.GetReturnValue().SetUndefined();
    } else {
        args.GetReturnValue().SetUndefined();
    }
}

void CanvasRenderingContext2DImpl::CreatePatternAndroid(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2 && Helpers::IsString(args[0])) {
        auto bytes = Helpers::GetString(isolate, args[0]);
        char *endptr;
        auto bytes_addr = std::strtoll(bytes.c_str(), &endptr, 10);
        if (bytes_addr == 0) {
            args.GetReturnValue().SetUndefined();
            return;
        }

        auto rep = Helpers::ConvertFromV8String(isolate, args[1]);
        rust::Box<PaintStyle> pattern = canvas_native_context_create_pattern_bytes(
                ptr->GetContext(),
                bytes_addr,
                rust::Str(rep.c_str(),
                          rep.size()));
        auto type = canvas_native_context_get_style_type(*pattern);
        if (type == PaintStyleType::None) {
            args.GetReturnValue().SetUndefined();
        } else {
            args.GetReturnValue().Set(CanvasPattern::NewInstance(isolate, std::move(pattern)));
        }

        args.GetReturnValue().SetUndefined();
    } else {
        args.GetReturnValue().SetUndefined();
    }
}

void CanvasRenderingContext2DImpl::CreateRadialGradient(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 6) {
        auto x0 = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y0 = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto r0 = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto x1 = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto y1 = static_cast<float>(args[4]->NumberValue(context).ToChecked());
        auto r1 = static_cast<float>(args[5]->NumberValue(context).ToChecked());

        auto gradient = canvas_native_context_create_radial_gradient(ptr->GetContext(), x0, y0, r0,
                                                                     x1, y1, r1);
        args.GetReturnValue().Set(CanvasGradient::NewInstance(isolate, std::move(gradient)));
    }
}

void
CanvasRenderingContext2DImpl::DrawFocusIfNeeded(const v8::FunctionCallbackInfo<v8::Value> &args) {
    // NOOP
}


void CanvasRenderingContext2DImpl::DrawImage(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    // TODO handle other cases
    if (args.Length() == 3) {
        auto image = args[0].As<v8::Object>();
        auto dx = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto dy = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto type = Helpers::GetInstanceType(isolate, image);
        if (type == ObjectType::ImageAsset) {
            auto asset = ImageAssetImpl::GetPointer(image);
            canvas_native_context_draw_image_dx_dy_asset(ptr->GetContext(), asset->GetImageAsset(),
                                                         dx, dy);
            ptr->UpdateInvalidateState();
        } else if (type == ObjectType::ImageBitmap) {
            auto asset = ImageAssetImpl::GetPointer(image);
            canvas_native_context_draw_image_dx_dy_asset(ptr->GetContext(), asset->GetImageAsset(),
                                                         dx, dy);
            ptr->UpdateInvalidateState();
        }
    } else if (args.Length() == 5) {
        auto image = args[0]->ToObject(context).ToLocalChecked();
        auto dx = args[1]->NumberValue(context).ToChecked();
        auto dy = args[2]->NumberValue(context).ToChecked();
        auto dWidth = args[3]->NumberValue(context).ToChecked();
        auto dHeight = args[4]->NumberValue(context).ToChecked();

        auto type = Helpers::GetInstanceType(isolate, image);
        if (type == ObjectType::ImageAsset) {
            auto asset = ImageAssetImpl::GetPointer(image);
            canvas_native_context_draw_image_dx_dy_dw_dh_asset(ptr->GetContext(),
                                                               asset->GetImageAsset(), dx, dy,
                                                               dWidth,
                                                               dHeight);
            ptr->UpdateInvalidateState();
        } else if (type == ObjectType::ImageBitmap) {
            auto asset = ImageAssetImpl::GetPointer(image);
            canvas_native_context_draw_image_dx_dy_dw_dh_asset(ptr->GetContext(),
                                                               asset->GetImageAsset(), dx, dy,
                                                               dWidth,
                                                               dHeight);
            ptr->UpdateInvalidateState();
        }
    } else if (args.Length() == 9) {
        auto image = args[0]->ToObject(context).ToLocalChecked();
        auto sx = args[1]->NumberValue(context).ToChecked();
        auto sy = args[2]->NumberValue(context).ToChecked();
        auto sWidth = args[3]->NumberValue(context).ToChecked();
        auto sHeight = args[4]->NumberValue(context).ToChecked();
        auto dx = args[5]->NumberValue(context).ToChecked();
        auto dy = args[6]->NumberValue(context).ToChecked();
        auto dWidth = args[7]->NumberValue(context).ToChecked();
        auto dHeight = args[8]->NumberValue(context).ToChecked();

        auto type = Helpers::GetInstanceType(isolate, image);

        if (type == ObjectType::ImageAsset) {
            auto asset = ImageAssetImpl::GetPointer(image);
            canvas_native_context_draw_image_asset(ptr->GetContext(), asset->GetImageAsset(), sx,
                                                   sy, sWidth, sHeight,
                                                   dx,
                                                   dy, dWidth, dHeight);
            ptr->UpdateInvalidateState();
        } else if (type == ObjectType::ImageBitmap) {
            auto asset = ImageAssetImpl::GetPointer(image);
            canvas_native_context_draw_image_asset(ptr->GetContext(), asset->GetImageAsset(), sx,
                                                   sy, sWidth, sHeight,
                                                   dx,
                                                   dy, dWidth, dHeight);
            ptr->UpdateInvalidateState();
        }
    }
}


void CanvasRenderingContext2DImpl::Ellipse(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 8) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto radiusX = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto radiusY = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto rotation = static_cast<float>(args[4]->NumberValue(context).ToChecked());
        auto startAngle = static_cast<float>(args[5]->NumberValue(context).ToChecked());
        auto endAngle = static_cast<float>(args[6]->NumberValue(context).ToChecked());
        auto anticlockwise = false;
        if (args[7]->IsBoolean()) {
            anticlockwise = args[7]->BooleanValue(isolate);
        }

        canvas_native_context_ellipse(ptr->GetContext(), x, y, radiusX, radiusY, rotation,
                                      startAngle, endAngle,
                                      anticlockwise);
    }
}


void CanvasRenderingContext2DImpl::Fill(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2) {
        auto object = args[0]->ToObject(context).ToLocalChecked();
        auto rule = args[1];
        if (Helpers::GetInstanceType(isolate, object) == ObjectType::Path2D) {
            auto path = Path2D::GetPointer(object);
            auto value = Helpers::ConvertFromV8String(isolate, rule);
            canvas_native_context_fill_with_path(ptr->GetContext(), path->GetPath(),
                                                 rust::Str(value.c_str(), value.size()));
            ptr->UpdateInvalidateState();
        }
    } else if (args.Length() == 1) {
        if (args[0]->IsString()) {
            auto value = Helpers::ConvertFromV8String(isolate, args[0]);
            canvas_native_context_fill(ptr->GetContext(), rust::Str(value.c_str(), value.size()));
            ptr->UpdateInvalidateState();
        } else if (args[0]->IsObject()) {
            auto object = args[0]->ToObject(context).ToLocalChecked();
            if (Helpers::GetInstanceType(isolate, object) == ObjectType::Path2D) {
                auto path = Path2D::GetPointer(object);
                std::string rule("nonzero");
                canvas_native_context_fill_with_path(ptr->GetContext(), path->GetPath(),
                                                     rust::Str(rule.c_str(), rule.size()));
                ptr->UpdateInvalidateState();
            }
        }
    } else {
        std::string rule("nonzero");
        canvas_native_context_fill(ptr->GetContext(), rust::Str(rule.c_str(), rule.size()));
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::FillRect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto width = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto height = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        canvas_native_context_fill_rect(ptr->GetContext(), x, y, width, height);
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::FillText(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() >= 3) {
        auto text = Helpers::ConvertFromV8String(isolate, args[0]);
        auto x = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        float width = -1;
        if (args[3]->IsNumber()) {
            width = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        }
        canvas_native_context_fill_text(ptr->GetContext(), rust::Str(text.c_str(), text.size()), x,
                                        y, width);
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::GetImageData(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 4) {
        auto sx = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto sy = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto sw = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto sh = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto data = canvas_native_context_get_image_data(ptr->GetContext(), sx, sy, sw, sh);
        auto *imageData = new ImageDataImpl(std::move(data));
        auto ret = ImageDataImpl::NewInstance(isolate, imageData);

        args.GetReturnValue().Set(ret);

    } else {
        args.GetReturnValue().Set(v8::Undefined(isolate));
    }
}

void CanvasRenderingContext2DImpl::GetLineDash(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto dash = canvas_native_context_get_line_dash(ptr->GetContext());
    auto size = dash.size();
    auto array = v8::Array::New(isolate, size);
    for (int i = 0; i < size; ++i) {
        array->Set(context, i, v8::Number::New(isolate, (double) dash[i]));
    }
    args.GetReturnValue().Set(array);
}

void
CanvasRenderingContext2DImpl::GetLineDashBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto dash = canvas_native_context_get_line_dash(ptr->GetContext());
    auto size = dash.size();
    auto store = v8::ArrayBuffer::New(isolate, dash.data(), size * 4,
                                      v8::ArrayBufferCreationMode::kInternalized);
    auto array = v8::Float32Array::New(store, 0, size);
    args.GetReturnValue().Set(array);
}

void CanvasRenderingContext2DImpl::IsPointInPath(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        std::string rule("nonzero");
        auto ret = canvas_native_context_is_point_in_path(ptr->GetContext(), x, y,
                                                          rust::Str(rule.c_str(), rule.size()));
        args.GetReturnValue().Set(ret);
    } else if (args.Length() == 3 && args[2]->IsString()) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto rule = Helpers::ConvertFromV8String(isolate, args[2]);
        auto ret = canvas_native_context_is_point_in_path(ptr->GetContext(), x, y,
                                                          rust::Str(rule.c_str(), rule.size()));
        args.GetReturnValue().Set(ret);
    } else if (args.Length() == 4 && args[0]->IsObject() && args[3]->IsString()) {
        auto path = args[0]->ToObject(context).ToLocalChecked();
        auto x = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto rule = Helpers::ConvertFromV8String(isolate, args[3]);
        if (Helpers::GetInstanceType(isolate, path) == ObjectType::Path2D) {
            auto path_ptr = Path2D::GetPointer(path);
            auto ret = canvas_native_context_is_point_in_path_with_path(ptr->GetContext(),
                                                                        path_ptr->GetPath(), x, y,
                                                                        rust::Str(rule.c_str(),
                                                                                  rule.size()));
            args.GetReturnValue().Set(ret);
        } else {
            args.GetReturnValue().Set(false);
        }
    } else {
        // TODO other checks ?
        args.GetReturnValue().Set(false);
    }
}

void
CanvasRenderingContext2DImpl::IsPointInStroke(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto ret = canvas_native_context_is_point_in_stroke(ptr->GetContext(), x, y);
        args.GetReturnValue().Set(ret);
    } else if (args.Length() == 3 && args[0]->IsObject()) {
        auto path = args[0]->ToObject(context).ToLocalChecked();
        auto x = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        if (Helpers::GetInstanceType(isolate, path) == ObjectType::Path2D) {
            auto path_ptr = Path2D::GetPointer(path);
            auto ret = canvas_native_context_is_point_in_stroke_with_path(ptr->GetContext(),
                                                                          path_ptr->GetPath(), x,
                                                                          y);
            args.GetReturnValue().Set(ret);
        } else {
            args.GetReturnValue().Set(false);
        }
    } else {
        // TODO other checks ?
        args.GetReturnValue().Set(false);
    }
}


void CanvasRenderingContext2DImpl::LineTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        canvas_native_context_line_to(ptr->GetContext(), x, y);
    }
}

void CanvasRenderingContext2DImpl::MeasureText(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto text = Helpers::ConvertFromV8String(isolate, args[0]);
    auto metrics = canvas_native_context_measure_text(ptr->GetContext(),
                                                      rust::Str(text.c_str(), text.size()));
    args.GetReturnValue().Set(TextMetricsImpl::NewInstance(isolate, std::move(metrics)));
}


void CanvasRenderingContext2DImpl::MoveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        canvas_native_context_move_to(ptr->GetContext(), x, y);
    }
}

void CanvasRenderingContext2DImpl::PutImageData(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto imageData = args[0]->ToObject(context).ToLocalChecked();
    if (args.Length() == 3) {
        auto imageDataPtr = ImageDataImpl::GetPointer(imageData);
        auto dx = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto dy = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        float dirtyX = 0;
        float dirtyY = 0;
        auto dirtyWidth = imageDataPtr->GetWidth();
        auto dirtyHeight = imageDataPtr->GetHeight();
        canvas_native_context_put_image_data(ptr->GetContext(), imageDataPtr->GetImageData(), dx,
                                             dy, dirtyX, dirtyY,
                                             dirtyWidth, dirtyHeight);
        ptr->UpdateInvalidateState();
    } else if (args.Length() == 7) {
        auto imageDataPtr = ImageDataImpl::GetPointer(imageData);
        auto dx = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto dy = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        float dirtyX = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        float dirtyY = static_cast<float>(args[4]->NumberValue(context).ToChecked());;
        auto dirtyWidth = static_cast<float>(args[5]->NumberValue(context).ToChecked());
        auto dirtyHeight = static_cast<float>(args[6]->NumberValue(context).ToChecked());
        canvas_native_context_put_image_data(ptr->GetContext(), imageDataPtr->GetImageData(), dx,
                                             dy, dirtyX, dirtyY,
                                             dirtyWidth, dirtyHeight);
        ptr->UpdateInvalidateState();
    }
}

void
CanvasRenderingContext2DImpl::QuadraticCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 4) {
        auto cpx = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto cpy = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto x = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        canvas_native_context_quadratic_curve_to(ptr->GetContext(), cpx, cpy, x, y);
    }
}

void CanvasRenderingContext2DImpl::Rect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 4) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto width = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto height = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        canvas_native_context_rect(ptr->GetContext(), x, y, width, height);
    }
}


void
CanvasRenderingContext2DImpl::RemoveHitRegion(const v8::FunctionCallbackInfo<v8::Value> &args) {
    // NOOP
}


void CanvasRenderingContext2DImpl::ResetTransform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    canvas_native_context_reset_transform(ptr->GetContext());
}


void CanvasRenderingContext2DImpl::Restore(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.This());
    canvas_native_context_restore(ptr->GetContext());
}


void CanvasRenderingContext2DImpl::Rotate(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 1 && args[0]->IsNumber()) {
        canvas_native_context_rotate(ptr->GetContext(), args[0]->NumberValue(context).ToChecked());
    }
}

void CanvasRenderingContext2DImpl::Save(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.This());
    canvas_native_context_save(ptr->GetContext());
}

void CanvasRenderingContext2DImpl::Scale(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        canvas_native_context_scale(ptr->GetContext(), x, y);
    }
}

void
CanvasRenderingContext2DImpl::ScrollPathIntoView(const v8::FunctionCallbackInfo<v8::Value> &args) {
// NOOP
}

void CanvasRenderingContext2DImpl::SetLineDash(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 1) {
        auto vec = args[0];
        if (vec->IsArray()) {
            auto segments = vec.As<v8::Array>();
            auto len = segments->Length();
            std::vector<float> data;
            auto context = isolate->GetCurrentContext();
            Helpers::LogToConsole("SetLineDash");
            for (int i = 0; i < len; ++i) {
                auto item = segments->Get(context, i);
                Helpers::LogToConsole("Get");
                data.push_back(static_cast<float>(item.ToLocalChecked()->NumberValue(
                        context).ToChecked()));
            }
            Helpers::LogToConsole("Get Loop");
            rust::Slice<const float> slice{data.data(), data.size()};
            Helpers::LogToConsole("slice");
            canvas_native_context_set_line_dash(ptr->GetContext(), slice);
            Helpers::LogToConsole("canvas_native_context_set_line_dash");
        }
    }
}

void
CanvasRenderingContext2DImpl::SetLineDashBuffer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 1) {
        auto vec = args[0];
        if (vec->IsFloat32Array()) {
            auto segments = Helpers::GetTypedArrayData<const float>(vec.As<v8::Float32Array>());
            canvas_native_context_set_line_dash(ptr->GetContext(), segments);
        }
    }
}

void CanvasRenderingContext2DImpl::SetTransform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 1 && args[0]->IsObject()) {
        auto matrix = args[0]->ToObject(context).ToLocalChecked();
        if (Helpers::GetInstanceType(isolate, matrix) == ObjectType::Matrix) {
            auto matrix_ptr = MatrixImpl::GetPointer(matrix);
            canvas_native_context_set_transform_matrix(ptr->GetContext(), matrix_ptr->GetMatrix());
        }
    } else if (args.Length() == 6) {
        auto a = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto b = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto c = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto d = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto e = static_cast<float>(args[4]->NumberValue(context).ToChecked());
        auto f = static_cast<float>(args[5]->NumberValue(context).ToChecked());
        canvas_native_context_set_transform(ptr->GetContext(), a, b, c, d, e, f);
    }
}

void CanvasRenderingContext2DImpl::Stroke(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 1 && args[0]->IsObject()) {
        auto path = args[0]->ToObject(context).ToLocalChecked();
        if (Helpers::GetInstanceType(isolate, path) == ObjectType::Path2D) {
            auto path_ptr = Path2D::GetPointer(path);
            canvas_native_context_stroke_with_path(ptr->GetContext(), path_ptr->GetPath());
            ptr->UpdateInvalidateState();
        }
    } else {
        canvas_native_context_stroke(ptr->GetContext());
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::StrokeRect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 4) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto width = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto height = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        canvas_native_context_stroke_rect(ptr->GetContext(), x, y, width, height);
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::StrokeText(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() >= 3) {
        auto text = Helpers::ConvertFromV8String(isolate, args[0]);
        auto x = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        float maxWidth = static_cast<float>(args[3]->NumberValue(context).FromMaybe(-1));
        canvas_native_context_stroke_text(ptr->GetContext(), rust::Str(text.c_str(), text.size()),
                                          x, y, maxWidth);
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::Transform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 6) {
        auto a = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto b = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto c = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto d = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto e = static_cast<float>(args[4]->NumberValue(context).ToChecked());
        auto f = static_cast<float>(args[5]->NumberValue(context).ToChecked());
        canvas_native_context_transform(ptr->GetContext(), a, b, c, d, e, f);
    }
}

void CanvasRenderingContext2DImpl::Translate(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        canvas_native_context_translate(ptr->GetContext(), x, y);
    }
}

void CanvasRenderingContext2DImpl::ToDataURL(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    std::string type("image/png");
    int quality = 92;
    auto typeVal = args[0];
    auto qualityVal = args[1];

    if (typeVal->IsStringObject()) {
        type = Helpers::ConvertFromV8String(isolate, typeVal.As<v8::StringObject>()->ValueOf());
    }

    if (typeVal->IsString()) {
        type = Helpers::ConvertFromV8String(isolate, typeVal);
    }

    if (qualityVal->IsNumberObject()) {
        quality = static_cast<int32_t>(qualityVal.As<v8::NumberObject>()->ValueOf());
    }

    if (qualityVal->IsNumber()) {
        quality = qualityVal->Int32Value(context).FromMaybe(quality);
    }

    auto data = canvas_native_to_data_url(ptr->GetContext(), rust::Str(type.c_str(), type.size()),
                                          quality);
    args.GetReturnValue().Set(
            Helpers::ConvertToV8String(isolate, std::string(data.c_str(), data.size())));
}

v8::Local<v8::FunctionTemplate> CanvasRenderingContext2DImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto tmpl = cache->CanvasRenderingContext2DTmpl.get();
    if (tmpl != nullptr) {
        return tmpl->Get(isolate);
    }

    auto context = isolate->GetCurrentContext();
    auto canvasRenderingContextFunc = v8::FunctionTemplate::New(isolate, &Create);
    canvasRenderingContextFunc->SetClassName(
            Helpers::ConvertToV8String(isolate, "CanvasRenderingContext2D"));
    canvasRenderingContextFunc->InstanceTemplate()->SetInternalFieldCount(1);

    auto canvasRenderingContextTpl = canvasRenderingContextFunc->PrototypeTemplate();


    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "__resize"),
            v8::FunctionTemplate::New(isolate, &Resize)
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "globalAlpha"),
            &GetGlobalAlpha,
            &SetGlobalAlpha
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "font"),
            &GetFont,
            &SetFont
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "imageSmoothingEnabled"),
            &GetImageSmoothingEnabled,
            &SetImageSmoothingEnabled
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "imageSmoothingQuality"),
            &GetImageSmoothingQuality,
            &SetImageSmoothingQuality
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "lineDashOffset"),
            &GetLineDashOffset,
            &SetLineDashOffset
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "lineJoin"),
            &GetLineJoin,
            &SetLineJoin
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "lineCap"),
            &GetLineCap,
            &SetLineCap
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "miterLimit"),
            &GetMiterLimit,
            &SetMiterLimit
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "shadowColor"),
            &GetShadowColor,
            &SetShadowColor
    );


    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "shadowBlur"),
            &GetShadowBlur,
            &SetShadowBlur
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "shadowOffsetX"),
            &GetShadowOffsetX,
            &SetShadowOffsetX
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "shadowOffsetY"),
            &GetShadowOffsetY,
            &SetShadowOffsetY
    );


    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "textAlign"),
            &GetTextAlign,
            &SetTextAlign
    );


    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "globalCompositeOperation"),
            &GetGlobalCompositeOperation,
            &SetGlobalCompositeOperation
    );


    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "fillStyle"),
            &GetFillStyle,
            &SetFillStyle
    );


    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "strokeStyle"),
            &GetStrokeStyle,
            &SetStrokeStyle
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "lineWidth"),
            &GetLineWidth,
            &SetLineWidth
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "addHitRegion"),
            v8::FunctionTemplate::New(isolate, &AddHitRegion)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "arc"),
            v8::FunctionTemplate::New(isolate, &Arc)
    );


    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "arcTo"),
            v8::FunctionTemplate::New(isolate, &ArcTo)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "beginPath"),
            v8::FunctionTemplate::New(isolate, &BeginPath)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "bezierCurveTo"),
            v8::FunctionTemplate::New(isolate, &BezierCurveTo)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "clearHitRegions"),
            v8::FunctionTemplate::New(isolate, &ClearHitRegions)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "clearRect"),
            v8::FunctionTemplate::New(isolate, &ClearRect)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "clip"),
            v8::FunctionTemplate::New(isolate, &Clip)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "closePath"),
            v8::FunctionTemplate::New(isolate, &ClosePath)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createImageData"),
            v8::FunctionTemplate::New(isolate, &CreateImageData)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createLinearGradient"),
            v8::FunctionTemplate::New(isolate, &CreateLinearGradient)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createPattern"),
            v8::FunctionTemplate::New(isolate, &CreatePattern)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "__createPatternWithBitmap"),
            v8::FunctionTemplate::New(isolate, &CreatePatternAndroid)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createRadialGradient"),
            v8::FunctionTemplate::New(isolate, &CreateRadialGradient)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "drawFocusIfNeeded"),
            v8::FunctionTemplate::New(isolate, &DrawFocusIfNeeded)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "drawImage"),
            v8::FunctionTemplate::New(isolate, &DrawImage)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "ellipse"),
            v8::FunctionTemplate::New(isolate, &Ellipse)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "fill"),
            v8::FunctionTemplate::New(isolate, &Fill)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "fillRect"),
            v8::FunctionTemplate::New(isolate, &FillRect)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "fillText"),
            v8::FunctionTemplate::New(isolate, &FillText)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getImageData"),
            v8::FunctionTemplate::New(isolate, &GetImageData)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getLineDash"),
            v8::FunctionTemplate::New(isolate, &GetLineDash)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "__getLineDashBuffer"),
            v8::FunctionTemplate::New(isolate, &GetLineDashBuffer)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "isPointInPath"),
            v8::FunctionTemplate::New(isolate, &IsPointInPath)
    );


    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "isPointInStroke"),
            v8::FunctionTemplate::New(isolate, &IsPointInStroke)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "lineTo"),
            v8::FunctionTemplate::New(isolate, &LineTo)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "measureText"),
            v8::FunctionTemplate::New(isolate, &MeasureText)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "moveTo"),
            v8::FunctionTemplate::New(isolate, &MoveTo)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "putImageData"),
            v8::FunctionTemplate::New(isolate, &PutImageData)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "quadraticCurveTo"),
            v8::FunctionTemplate::New(isolate, &QuadraticCurveTo)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "rect"),
            v8::FunctionTemplate::New(isolate, &Rect)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "removeHitRegion"),
            v8::FunctionTemplate::New(isolate, &RemoveHitRegion)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "resetTransform"),
            v8::FunctionTemplate::New(isolate, &ResetTransform)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "restore"),
            v8::FunctionTemplate::New(isolate, &Restore)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "rotate"),
            v8::FunctionTemplate::New(isolate, &Rotate)
    );


    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "save"),
            v8::FunctionTemplate::New(isolate, &Save)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "scale"),
            v8::FunctionTemplate::New(isolate, &Scale)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "scrollPathIntoView"),
            v8::FunctionTemplate::New(isolate, &ScrollPathIntoView)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "setLineDash"),
            v8::FunctionTemplate::New(isolate, &SetLineDash)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "__setLineDashBuffer"),
            v8::FunctionTemplate::New(isolate, &SetLineDashBuffer)
    );


    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "setTransform"),
            v8::FunctionTemplate::New(isolate, &SetTransform)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "stroke"),
            v8::FunctionTemplate::New(isolate, &Stroke)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "strokeRect"),
            v8::FunctionTemplate::New(isolate, &StrokeRect)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "strokeText"),
            v8::FunctionTemplate::New(isolate, &StrokeText)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "transform"),
            v8::FunctionTemplate::New(isolate, &Transform)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "translate"),
            v8::FunctionTemplate::New(isolate, &Translate)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "__toDataURL"),
            v8::FunctionTemplate::New(isolate, &ToDataURL)
    );

    cache->CanvasRenderingContext2DTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(
            isolate,
            canvasRenderingContextFunc);
    return canvasRenderingContextFunc;
}

CanvasRenderingContext2D &CanvasRenderingContext2DImpl::GetContext() {
    return *this->context_;
}
