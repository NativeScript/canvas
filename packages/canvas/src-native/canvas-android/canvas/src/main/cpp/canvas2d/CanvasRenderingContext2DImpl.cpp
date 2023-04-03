//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasRenderingContext2DImpl.h"

CanvasRenderingContext2DImpl::CanvasRenderingContext2DImpl(
        rust::Box<CanvasRenderingContext2D> context) : context_(
        std::move(context)) {

    auto ctx_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(this));
    auto raf_callback = new OnRafCallback(ctx_ptr, 0);
    auto raf_callback_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(raf_callback));
    auto raf = canvas_native_raf_create(raf_callback_ptr);
    this->SetRaf(std::make_shared<RafImpl>(raf_callback, raf_callback_ptr, std::move(raf)));

    auto _raf = this->GetRaf();

    if (_raf != nullptr) {
        canvas_native_raf_start(_raf->GetRaf());
    }

}

void CanvasRenderingContext2DImpl::SetRaf(std::shared_ptr<RafImpl> raf) {
    this->raf_ = std::move(raf);
}

RafImpl *CanvasRenderingContext2DImpl::GetRaf() {
    return this->raf_.get();
}


std::vector<jsi::PropNameID> CanvasRenderingContext2DImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(68);

    /* Non Standard 2D */

    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("drawPoint")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("drawPoints")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("drawPaint")));

    /* Non Standard 2D */

    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("__makeDirty")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("__getPointer")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("__resize")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("filter")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("font")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("globalAlpha")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("imageSmoothingEnabled")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("imageSmoothingQuality")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("lineDashOffset")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("lineJoin")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("lineCap")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("miterLimit")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("shadowColor")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("shadowBlur")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("shadowOffsetX")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("shadowOffsetY")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("textAlign")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("globalCompositeOperation")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("fillStyle")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("strokeStyle")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("lineWidth")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("lineDash")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("addHitRegion")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("arc")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("arcTo")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("beginPath")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("bezierCurveTo")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("clearHitRegions")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("clearRect")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("clip")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("closePath")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("createImageData")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("createLinearGradient")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("createRadialGradient")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("createPattern")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("__createPatternWithNative")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("drawImage")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("ellipse")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("fill")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("fillRect")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("fillText")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getImageData")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("getLineDash")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("isPointInPath")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("isPointInStroke")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("lineTo")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("measureText")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("moveTo")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("putImageData")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("quadraticCurveTo")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("rect")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("roundRect")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("removeHitRegion")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("resetTransform")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("restore")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("rotate")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("save")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("scale")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("scrollPathIntoView")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("setLineDash")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("setTransform")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("stroke")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("strokeRect")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("strokeText")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("transform")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("translate")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("__toDataURL")));
    return ret;
}


void
CanvasRenderingContext2DImpl::set(jsi::Runtime &runtime, const jsi::PropNameID &name,
                                  const jsi::Value &value) {
    auto methodName = name.utf8(runtime);

    if (methodName == "filter") {
        auto val = value.asString(runtime).utf8(runtime);
        canvas_native_context_set_filter(this->GetContext(), rust::Str(val.c_str()));
    } else if (methodName == "font") {
        auto val = value.asString(runtime).utf8(runtime);
        canvas_native_context_set_font(this->GetContext(), rust::Str(val.c_str()));
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
                                            rust::Str(join.c_str()));
    } else if (methodName == "lineCap") {
        auto cap = value.asString(runtime).utf8(runtime);
        canvas_native_context_set_line_cap(this->GetContext(),
                                           rust::Str(cap.c_str()));
    } else if (methodName == "miterLimit") {
        auto miterLimit = (float) value.asNumber();
        canvas_native_context_set_miter_limit(this->GetContext(), miterLimit);
    } else if (methodName == "shadowColor") {
        auto shadowColor = value.asString(runtime).utf8(runtime);
        canvas_native_context_set_shadow_color(this->GetContext(),
                                               rust::Str(shadowColor.c_str()));
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
                                             rust::Str(textAlign.c_str()));
    } else if (methodName == "globalCompositeOperation") {
        auto globalCompositeOperation = value.asString(runtime).utf8(runtime);
        canvas_native_context_set_global_composition(this->GetContext(),
                                                     rust::Str(globalCompositeOperation.c_str()));
    } else if (methodName == "fillStyle") {
        if (value.isString()) {
            auto style = value.asString(runtime).utf8(runtime);
            canvas_native_paint_style_set_fill_color_with_c_string(this->GetContext(), style.c_str());
        } else if (value.isObject()) {

            try {
                auto gradient = getHostObject<CanvasGradient>(runtime, value);
                if (gradient != nullptr) {
                    canvas_native_context_set_fill_style(this->GetContext(),
                                                         gradient->GetPaintStyle());
                    return;
                }
            } catch (...) {}

            try {
                auto pattern = getHostObject<CanvasPattern>(runtime, value);

                if (pattern != nullptr) {
                    canvas_native_context_set_fill_style(this->GetContext(),
                                                         pattern->GetPaintStyle());
                    return;
                }
            } catch (...) {}
        }
    } else if (methodName == "strokeStyle") {
        if (value.isString()) {
            auto style = value.asString(runtime).utf8(runtime);
            canvas_native_paint_style_set_stroke_color_with_c_string(this->GetContext(),style.c_str());
        } else if (value.isObject()) {

            try {
                auto gradient = getHostObject<CanvasGradient>(runtime, value);
                if (gradient != nullptr) {
                    canvas_native_context_set_stroke_style(this->GetContext(),
                                                           gradient->GetPaintStyle());
                    return;
                }
            } catch (...) {}

            try {
                auto pattern = getHostObject<CanvasPattern>(runtime, value);

                if (pattern != nullptr) {
                    canvas_native_context_set_stroke_style(this->GetContext(),
                                                           pattern->GetPaintStyle());
                    return;
                }
            } catch (...) {}
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


jsi::Value CanvasRenderingContext2DImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);


    if (methodName == "drawPaint") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         auto color = arguments[0].asString(
                                                                 runtime).utf8(runtime);
                                                         canvas_native_context_draw_paint(
                                                                 this->GetContext(),
                                                                 rust::Str(color));
                                                         this->UpdateInvalidateState();
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "drawPoint") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         auto x = (float) arguments[0].asNumber();
                                                         auto y = (float) arguments[1].asNumber();
                                                         canvas_native_context_draw_point(
                                                                 this->GetContext(), x, y);
                                                         this->UpdateInvalidateState();
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "drawPoints") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         auto mode = arguments[0].asString(
                                                                 runtime).utf8(
                                                                 runtime);
                                                         auto points = arguments[1].asObject(
                                                                 runtime).getArray(
                                                                 runtime);
                                                         auto size = points.size(runtime);


                                                         if ((size % 2) == 0) {
                                                             int32_t pointMode = -1;
                                                             if (mode == "points") {
                                                                 pointMode = 0;
                                                             } else if (mode == "lines") {
                                                                 pointMode = 1;
                                                             } else if (mode == "polygon") {
                                                                 pointMode = 2;
                                                             }
                                                             if (pointMode == -1) {
                                                                 return jsi::Value::undefined();
                                                             }
                                                             rust::Vec<float> store;
                                                             store.reserve(size);
                                                             size_t next = 0;
                                                             for (size_t i = 0; i < size; i++) {

                                                                 auto object = points.getValueAtIndex(
                                                                         runtime, i).asObject(
                                                                         runtime);

                                                                 auto x = object.getProperty(
                                                                         runtime, "x").asNumber();
                                                                 auto y = object.getProperty(
                                                                         runtime, "y").asNumber();
                                                                 store[next] = (float) x;
                                                                 store[next + 1] = (float) y;

                                                                 next = i + 2;
                                                             }

                                                             rust::Slice<const float> buf(
                                                                     store.data(),
                                                                     store.size());

                                                             canvas_native_context_draw_points(
                                                                     this->GetContext(), pointMode,
                                                                     buf);
                                                             this->UpdateInvalidateState();
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    }

    if (methodName == "filter") {
        auto filter = canvas_native_context_get_filter(this->GetContext());
        return jsi::String::createFromAscii(runtime, filter.data(), filter.length());
    }

    if (methodName == "__makeDirty") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         this->UpdateInvalidateState();
                                                         return jsi::Value::undefined();
                                                     }
        );
    }

    if (methodName == "__getPointer") {
        auto ptr = (intptr_t *) &this->GetContext();
        auto ret = std::to_string((intptr_t) ptr);
        return jsi::String::createFromAscii(runtime, ret);
    }

    if (methodName == "__resize") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         auto width = arguments[0].asNumber();
                                                         auto height = arguments[1].asNumber();
                                                         canvas_native_context_resize(
                                                                 this->GetContext(),
                                                                 static_cast<float>(width),
                                                                 static_cast<float>(height));
                                                         return jsi::Value::undefined();
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
                return jsi::Value::undefined();
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
                return jsi::Value::undefined();
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
            ret.setValueAtIndex(runtime, i, jsi::Value((double) item));
        }
        return ret;
    } else if (methodName == "addHitRegion") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [](jsi::Runtime &runtime,
                                                        const jsi::Value &thisValue,
                                                        const jsi::Value *arguments,
                                                        size_t count) -> jsi::Value {
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "arc") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     6,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


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

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "arcTo") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     5,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


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

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "beginPath") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         canvas_native_context_begin_path(
                                                                 this->GetContext());
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "bezierCurveTo") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     6,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         canvas_native_context_bezier_curve_to(
                                                                 this->GetContext(),
                                                                 static_cast<float>(arguments[0].asNumber()),
                                                                 static_cast<float>(arguments[1].asNumber()),
                                                                 static_cast<float>(arguments[2].asNumber()),
                                                                 static_cast<float>(arguments[3].asNumber()),
                                                                 static_cast<float>(arguments[4].asNumber()),
                                                                 static_cast<float>(arguments[5].asNumber())
                                                         );
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "clearHitRegions") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [](jsi::Runtime &runtime,
                                                        const jsi::Value &thisValue,
                                                        const jsi::Value *arguments,
                                                        size_t count) -> jsi::Value {
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "clearRect") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         canvas_native_context_clear_rect(
                                                                 this->GetContext(),
                                                                 static_cast<float>(arguments[0].asNumber()),
                                                                 static_cast<float>(arguments[1].asNumber()),
                                                                 static_cast<float>(arguments[2].asNumber()),
                                                                 static_cast<float>(arguments[3].asNumber())
                                                         );
                                                         this->UpdateInvalidateState();

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "clip") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 0) {
                                                             std::string rule("nonzero");
                                                             canvas_native_context_clip_rule(
                                                                     this->GetContext(),
                                                                     rust::Str(rule.c_str()));
                                                         } else if (arguments[0].isString()) {
                                                             auto val = arguments[0].asString(
                                                                     runtime).utf8(runtime);
                                                             canvas_native_context_clip_rule(
                                                                     this->GetContext(),
                                                                     rust::Str(val.c_str()));
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "closePath") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         canvas_native_context_close_path(
                                                                 this->GetContext());
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "createImageData") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [](jsi::Runtime &runtime,
                                                        const jsi::Value &thisValue,
                                                        const jsi::Value *arguments,
                                                        size_t count) -> jsi::Value {


                                                         if (count == 1 &&
                                                             arguments[0].isObject()) {

                                                             auto object = getHostObject<ImageDataImpl>(
                                                                     runtime, arguments[0]);

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
                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "createLinearGradient") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


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

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "createPattern") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             if (arguments[0].isNull() ||
                                                                 arguments[0].isUndefined()) { return jsi::Value::undefined(); }

                                                             auto object = arguments[0].asObject(
                                                                     runtime);

                                                             try {
                                                                 auto image_asset = getHostObject<ImageAssetImpl>(
                                                                         runtime, object);

                                                                 if (image_asset != nullptr) {
                                                                     auto rep = arguments[1].asString(
                                                                             runtime).utf8(runtime);
                                                                     rust::Box<PaintStyle> pattern = canvas_native_context_create_pattern_asset(
                                                                             this->GetContext(),
                                                                             image_asset->GetImageAsset(),
                                                                             rust::Str(
                                                                                     rep.c_str()));
                                                                     auto type = canvas_native_context_get_style_type(
                                                                             *pattern);
                                                                     if (type ==
                                                                         PaintStyleType::None) {
                                                                         return jsi::Value::undefined();
                                                                     } else {
                                                                         auto ret = std::make_shared<CanvasPattern>(
                                                                                 std::move(
                                                                                         pattern));
                                                                         return jsi::Object::createFromHostObject(
                                                                                 runtime, ret);
                                                                     }
                                                                 }
                                                             } catch (...) {}

                                                             try {
                                                                 auto canvas_2d = getHostObject<CanvasRenderingContext2DImpl>(
                                                                         runtime, object);
                                                                 if (canvas_2d != nullptr) {
                                                                     auto rep = arguments[1].asString(
                                                                             runtime).utf8(runtime);
                                                                     rust::Box<PaintStyle> pattern = canvas_native_context_create_pattern_canvas2d(
                                                                             canvas_2d->GetContext(),
                                                                             this->GetContext(),
                                                                             rust::Str(
                                                                                     rep.c_str()));
                                                                     auto type = canvas_native_context_get_style_type(
                                                                             *pattern);
                                                                     if (type ==
                                                                         PaintStyleType::None) {
                                                                         return jsi::Value::undefined();
                                                                     } else {
                                                                         auto ret = std::make_shared<CanvasPattern>(
                                                                                 std::move(
                                                                                         pattern));
                                                                         return jsi::Object::createFromHostObject(
                                                                                 runtime, ret);
                                                                     }
                                                                 }
                                                             } catch (...) {}

                                                             try {
                                                                 auto webgl = getHostObject<WebGLRenderingContextBase>(
                                                                         runtime, object);
                                                                 if (webgl != nullptr) {
                                                                     auto rep = arguments[1].asString(
                                                                             runtime).utf8(runtime);
                                                                     auto pattern = canvas_native_context_create_pattern_webgl(
                                                                             webgl->GetState(),
                                                                             this->GetContext(),
                                                                             rust::Str(
                                                                                     rep.c_str()));
                                                                     auto type = canvas_native_context_get_style_type(
                                                                             *pattern);
                                                                     if (type ==
                                                                         PaintStyleType::None) {
                                                                         return jsi::Value::undefined();
                                                                     } else {
                                                                         auto ret = std::make_shared<CanvasPattern>(
                                                                                 std::move(
                                                                                         pattern));
                                                                         return jsi::Object::createFromHostObject(
                                                                                 runtime, ret);
                                                                     }
                                                                 }
                                                             } catch (...) {}

                                                         }


                                                         return jsi::Value::undefined();
                                                     }
        );

    } else if (methodName == "__createPatternWithNative") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [](jsi::Runtime &runtime,
                                                        const jsi::Value &thisValue,
                                                        const jsi::Value *arguments,
                                                        size_t count) -> jsi::Value {

                                                         auto ptr = getPointerValue(runtime,
                                                                                    arguments[0]);

                                                         auto pattern = canvas_native_pattern_from_ptr(
                                                                 ptr);
                                                         auto type = canvas_native_context_get_style_type(
                                                                 *pattern);
                                                         if (type == PaintStyleType::None) {
                                                             return jsi::Value::undefined();
                                                         } else {
                                                             auto ret = std::make_shared<CanvasPattern>(
                                                                     std::move(pattern));
                                                             return jsi::Object::createFromHostObject(
                                                                     runtime, ret);
                                                         }

                                                     }
        );

    } else if (methodName == "createRadialGradient") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     6,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


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

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "drawFocusIfNeeded") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [](jsi::Runtime &runtime,
                                                        const jsi::Value &thisValue,
                                                        const jsi::Value *arguments,
                                                        size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "drawImage") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     9,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count == 3) {
                                                             auto image = arguments[0].asObject(
                                                                     runtime);
                                                             auto dx = static_cast<float>(arguments[1].asNumber());
                                                             auto dy = static_cast<float>(arguments[2].asNumber());


                                                             try {
                                                                 auto image_asset = getHostObject<ImageAssetImpl>(
                                                                         runtime, image);

                                                                 if (image_asset != nullptr) {
                                                                     canvas_native_context_draw_image_dx_dy_asset(
                                                                             this->GetContext(),
                                                                             image_asset->GetImageAsset(),
                                                                             dx, dy);
                                                                     this->UpdateInvalidateState();
                                                                     return jsi::Value::undefined();
                                                                 }
                                                             } catch (...) {}


                                                             try {
                                                                 auto image_bitmap = getHostObject<ImageBitmapImpl>(
                                                                         runtime, image);
                                                                 if (image_bitmap != nullptr) {
                                                                     canvas_native_context_draw_image_dx_dy_asset(
                                                                             this->GetContext(),
                                                                             image_bitmap->GetImageAsset(),
                                                                             dx, dy);
                                                                     this->UpdateInvalidateState();
                                                                 }
                                                             } catch (...) {}


                                                             try {
                                                                 auto image_canvas = getHostObject<CanvasRenderingContext2DImpl>(
                                                                         runtime, image);
                                                                 if (image_canvas != nullptr) {
                                                                     canvas_native_context_draw_image_dx_dy_context(
                                                                             this->GetContext(),
                                                                             image_canvas->GetContext(),
                                                                             dx, dy);
                                                                     this->UpdateInvalidateState();
                                                                 }
                                                             } catch (...) {}


                                                         } else if (count == 5) {
                                                             auto image = arguments[0].asObject(
                                                                     runtime);
                                                             auto dx = (float) arguments[1].asNumber();
                                                             auto dy = (float) arguments[2].asNumber();
                                                             auto dWidth = (float) arguments[3].asNumber();
                                                             auto dHeight = (float) arguments[4].asNumber();

                                                             try {
                                                                 auto image_asset = getHostObject<ImageAssetImpl>(
                                                                         runtime, image);
                                                                 if (image_asset != nullptr) {
                                                                     canvas_native_context_draw_image_dx_dy_dw_dh_asset(
                                                                             this->GetContext(),
                                                                             image_asset->GetImageAsset(),
                                                                             dx, dy,
                                                                             dWidth,
                                                                             dHeight);
                                                                     this->UpdateInvalidateState();
                                                                     return jsi::Value::undefined();
                                                                 }
                                                             } catch (...) {}

                                                             try {
                                                                 auto image_bitmap = getHostObject<ImageBitmapImpl>(
                                                                         runtime, image);
                                                                 if (image_bitmap != nullptr) {
                                                                     canvas_native_context_draw_image_dx_dy_dw_dh_asset(
                                                                             this->GetContext(),
                                                                             image_bitmap->GetImageAsset(),
                                                                             dx, dy,
                                                                             dWidth,
                                                                             dHeight);
                                                                     this->UpdateInvalidateState();
                                                                     return jsi::Value::undefined();
                                                                 }
                                                             } catch (...) {}

                                                             try {
                                                                 auto image_canvas = getHostObject<CanvasRenderingContext2DImpl>(
                                                                         runtime, image);
                                                                 if (image_canvas != nullptr) {
                                                                     canvas_native_context_draw_image_dx_dy_dw_dh_context(
                                                                             this->GetContext(),
                                                                             image_canvas->GetContext(),
                                                                             dx, dy,
                                                                             dWidth,
                                                                             dHeight);
                                                                     this->UpdateInvalidateState();
                                                                     return jsi::Value::undefined();
                                                                 }
                                                             } catch (...) {}


                                                         } else if (count == 9) {
                                                             auto image = arguments[0].asObject(
                                                                     runtime);
                                                             auto sx = (float) arguments[1].asNumber();
                                                             auto sy = (float) arguments[2].asNumber();
                                                             auto sWidth = (float) arguments[3].asNumber();
                                                             auto sHeight = (float) arguments[4].asNumber();
                                                             auto dx = (float) arguments[5].asNumber();
                                                             auto dy = (float) arguments[6].asNumber();
                                                             auto dWidth = (float) arguments[7].asNumber();
                                                             auto dHeight = (float) arguments[8].asNumber();

                                                             try {
                                                                 auto image_asset = getHostObject<ImageAssetImpl>(
                                                                         runtime, image);
                                                                 if (image_asset != nullptr) {
                                                                     canvas_native_context_draw_image_asset(
                                                                             this->GetContext(),
                                                                             image_asset->GetImageAsset(),
                                                                             sx,
                                                                             sy, sWidth, sHeight,
                                                                             dx,
                                                                             dy, dWidth, dHeight);
                                                                     this->UpdateInvalidateState();
                                                                     return jsi::Value::undefined();
                                                                 }
                                                             } catch (...) {}

                                                             try {
                                                                 auto image_bitmap = getHostObject<ImageBitmapImpl>(
                                                                         runtime, image);
                                                                 if (image_bitmap != nullptr) {
                                                                     canvas_native_context_draw_image_asset(
                                                                             this->GetContext(),
                                                                             image_bitmap->GetImageAsset(),
                                                                             sx,
                                                                             sy, sWidth, sHeight,
                                                                             dx,
                                                                             dy, dWidth, dHeight);
                                                                     this->UpdateInvalidateState();
                                                                     return jsi::Value::undefined();
                                                                 }
                                                             } catch (...) {}

                                                             try {
                                                                 auto image_canvas = getHostObject<CanvasRenderingContext2DImpl>(
                                                                         runtime, image);
                                                                 if (image_canvas != nullptr) {
                                                                     canvas_native_context_draw_image_context(
                                                                             this->GetContext(),
                                                                             image_canvas->GetContext(),
                                                                             sx,
                                                                             sy, sWidth, sHeight,
                                                                             dx,
                                                                             dy, dWidth, dHeight);
                                                                     this->UpdateInvalidateState();
                                                                     return jsi::Value::undefined();
                                                                 }
                                                             } catch (...) {}


                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "ellipse") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     8,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
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
                                                                     this->GetContext(), x, y,
                                                                     radiusX,
                                                                     radiusY, rotation,
                                                                     startAngle, endAngle,
                                                                     anticlockwise);
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "fill") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         if (count == 2) {
                                                             auto object = getHostObject<Path2D>(
                                                                     runtime, arguments[0]);

                                                             if (object != nullptr) {
                                                                 auto value = arguments[1].asString(
                                                                         runtime).utf8(runtime);
                                                                 canvas_native_context_fill_with_path(
                                                                         this->GetContext(),
                                                                         object->GetPath(),
                                                                         rust::Str(value.c_str()));
                                                                 this->UpdateInvalidateState();
                                                             }
                                                         } else if (count == 1) {
                                                             if (arguments[0].isString()) {
                                                                 auto value = arguments[0].asString(
                                                                         runtime).utf8(runtime);
                                                                 canvas_native_context_fill(
                                                                         this->GetContext(),
                                                                         rust::Str(value.c_str()));
                                                                 this->UpdateInvalidateState();
                                                             } else if (arguments[0].isObject()) {
                                                                 auto object = getHostObject<Path2D>(
                                                                         runtime, arguments[0]);
                                                                 if (object != nullptr) {
                                                                     std::string rule("nonzero");
                                                                     canvas_native_context_fill_with_path(
                                                                             this->GetContext(),
                                                                             object->GetPath(),
                                                                             rust::Str(
                                                                                     rule.c_str()));
                                                                     this->UpdateInvalidateState();
                                                                 }
                                                             }
                                                         } else {
                                                             std::string rule("nonzero");
                                                             canvas_native_context_fill(
                                                                     this->GetContext(),
                                                                     rust::Str(rule.c_str()));
                                                             this->UpdateInvalidateState();
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "fillRect") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto x = static_cast<float>(arguments[0].asNumber());
                                                         auto y = static_cast<float>(arguments[1].asNumber());
                                                         auto width = static_cast<float>(arguments[2].asNumber());
                                                         auto height = static_cast<float>(arguments[3].asNumber());
                                                         canvas_native_context_fill_rect(
                                                                 this->GetContext(), x, y, width,
                                                                 height);
                                                         this->UpdateInvalidateState();

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "fillText") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto text = arguments[0].asString(
                                                                 runtime).utf8(
                                                                 runtime);
                                                         auto x = static_cast<float>(arguments[1].asNumber());
                                                         auto y = static_cast<float>(arguments[2].asNumber());
                                                         float width = -1;
                                                         if (arguments[3].isNumber()) {
                                                             width = static_cast<float>(arguments[3].asNumber());
                                                         }
                                                         canvas_native_context_fill_text(
                                                                 this->GetContext(),
                                                                 rust::Str(text.c_str()), x,
                                                                 y, width);
                                                         this->UpdateInvalidateState();

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "getImageData") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 4) {
                                                             auto sx = static_cast<float>(arguments[0].asNumber());
                                                             auto sy = static_cast<float>(arguments[1].asNumber());
                                                             auto sw = static_cast<float>(arguments[2].asNumber());
                                                             auto sh = static_cast<float>(arguments[3].asNumber());
                                                             auto data = canvas_native_context_get_image_data(
                                                                     this->GetContext(), sx, sy, sw,
                                                                     sh);
                                                             auto object = std::make_shared<ImageDataImpl>(
                                                                     std::move(data));
                                                             return jsi::Object::createFromHostObject(
                                                                     runtime, object);
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "getLineDash") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto dash = canvas_native_context_get_line_dash(
                                                                 this->GetContext());
                                                         auto size = dash.size();
                                                         auto array = jsi::Array(runtime, size);
                                                         for (int i = 0; i < size; ++i) {
                                                             array.setValueAtIndex(runtime, i,
                                                                                   jsi::Value(
                                                                                           (double) dash[i]));
                                                         }
                                                         return array;
                                                     }
        );
    } else if (methodName == "isPointInPath") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 2) {
                                                             auto x = static_cast<float>(arguments[0].asNumber());
                                                             auto y = static_cast<float>(arguments[1].asNumber());
                                                             std::string rule("nonzero");
                                                             auto ret = canvas_native_context_is_point_in_path(
                                                                     this->GetContext(), x, y,
                                                                     rust::Str(rule.c_str()));
                                                             return {ret};
                                                         } else if (count == 3 &&
                                                                    arguments[2].isString()) {
                                                             auto x = static_cast<float>(arguments[0].asNumber());
                                                             auto y = static_cast<float>(arguments[1].asNumber());
                                                             auto rule = arguments[2].asString(
                                                                     runtime).utf8(runtime);
                                                             auto ret = canvas_native_context_is_point_in_path(
                                                                     this->GetContext(), x, y,
                                                                     rust::Str(rule.c_str()));
                                                             return {ret};
                                                         } else if (count == 4 &&
                                                                    arguments[0].isObject() &&
                                                                    arguments[3].isString()) {
                                                             auto path = getHostObject<Path2D>(
                                                                     runtime, arguments[0]);
                                                             auto x = static_cast<float>(arguments[1].asNumber());
                                                             auto y = static_cast<float>(arguments[2].asNumber());
                                                             auto rule = arguments[3].asString(
                                                                     runtime).utf8(runtime);


                                                             if (path != nullptr) {
                                                                 auto ret = canvas_native_context_is_point_in_path_with_path(
                                                                         this->GetContext(),
                                                                         path->GetPath(), x, y,
                                                                         rust::Str(rule.c_str()));
                                                                 return {ret};
                                                             }
                                                         }

                                                         return {false};

                                                     }
        );
    } else if (methodName == "isPointInStroke") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     3,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 2) {
                                                             auto x = static_cast<float>(arguments[0].asNumber());
                                                             auto y = static_cast<float>(arguments[1].asNumber());
                                                             auto ret = canvas_native_context_is_point_in_stroke(
                                                                     this->GetContext(), x, y);
                                                             return {ret};
                                                         } else if (count == 3 &&
                                                                    arguments[0].isObject()) {
                                                             auto path = getHostObject<Path2D>(
                                                                     runtime, arguments[0]);
                                                             auto x = static_cast<float>(arguments[1].asNumber());
                                                             auto y = static_cast<float>(arguments[2].asNumber());
                                                             if (path != nullptr) {
                                                                 auto ret = canvas_native_context_is_point_in_stroke_with_path(
                                                                         this->GetContext(),
                                                                         path->GetPath(), x,
                                                                         y);
                                                                 return {ret};
                                                             }
                                                         }

                                                         return {false};

                                                     }
        );
    } else if (methodName == "lineTo") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto x = static_cast<float>(arguments[0].asNumber());
                                                             auto y = static_cast<float>(arguments[1].asNumber());
                                                             canvas_native_context_line_to(
                                                                     this->GetContext(), x, y);
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "measureText") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto text = arguments[0].asString(
                                                                 runtime).utf8(
                                                                 runtime);
                                                         auto metrics = canvas_native_context_measure_text(
                                                                 this->GetContext(),
                                                                 rust::Str(text.c_str()));

                                                         auto object = std::make_shared<TextMetricsImpl>(
                                                                 std::move(metrics));

                                                         return jsi::Object::createFromHostObject(
                                                                 runtime, object);
                                                     }
        );
    } else if (methodName == "moveTo") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count > 1) {
                                                             auto x = static_cast<float>(arguments[0].asNumber());
                                                             auto y = static_cast<float>(arguments[1].asNumber());
                                                             canvas_native_context_move_to(
                                                                     this->GetContext(), x, y);
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "putImageData") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     7,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto imageData = getHostObject<ImageDataImpl>(
                                                                 runtime, arguments[0]);
                                                         if (count == 3) {
                                                             auto dx = static_cast<float>(arguments[1].asNumber());
                                                             auto dy = static_cast<float>(arguments[2].asNumber());
                                                             float dirtyX = 0;
                                                             float dirtyY = 0;
                                                             auto dirtyWidth = (float) canvas_native_image_data_get_width(
                                                                     imageData->GetImageData());
                                                             auto dirtyHeight = (float) canvas_native_image_data_get_height(
                                                                     imageData->GetImageData());
                                                             canvas_native_context_put_image_data(
                                                                     this->GetContext(),
                                                                     imageData->GetImageData(), dx,
                                                                     dy, dirtyX, dirtyY,
                                                                     dirtyWidth, dirtyHeight);
                                                             this->UpdateInvalidateState();
                                                         } else if (count == 7) {
                                                             auto dx = static_cast<float>(arguments[1].asNumber());
                                                             auto dy = static_cast<float>(arguments[2].asNumber());
                                                             auto dirtyX = static_cast<float>(arguments[3].asNumber());
                                                             auto dirtyY = static_cast<float>(arguments[4].asNumber());
                                                             auto dirtyWidth = static_cast<float>(arguments[5].asNumber());
                                                             auto dirtyHeight = static_cast<float>(arguments[6].asNumber());
                                                             canvas_native_context_put_image_data(
                                                                     this->GetContext(),
                                                                     imageData->GetImageData(), dx,
                                                                     dy, dirtyX, dirtyY,
                                                                     dirtyWidth, dirtyHeight);
                                                             this->UpdateInvalidateState();
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "quadraticCurveTo") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 4) {
                                                             auto cpx = static_cast<float>(arguments[0].asNumber());
                                                             auto cpy = static_cast<float>(arguments[1].asNumber());
                                                             auto x = static_cast<float>(arguments[2].asNumber());
                                                             auto y = static_cast<float>(arguments[3].asNumber());
                                                             canvas_native_context_quadratic_curve_to(
                                                                     this->GetContext(), cpx, cpy,
                                                                     x, y);
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "roundRect") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 5) {
                                                             auto x = static_cast<float>(arguments[0].asNumber());
                                                             auto y = static_cast<float>(arguments[1].asNumber());
                                                             auto width = static_cast<float>(arguments[2].asNumber());
                                                             auto height = static_cast<float>(arguments[3].asNumber());
                                                             if (arguments[4].isObject()) {
                                                                 auto radii = arguments[4].asObject(
                                                                         runtime);
                                                                 if (radii.isArray(runtime)) {
                                                                     auto array = radii.getArray(
                                                                             runtime);
                                                                     auto size = array.size(
                                                                             runtime);
                                                                     if (size > 1) {
                                                                         rust::Vec<float> store;
                                                                         store.reserve(size);
                                                                         for (int i = 0;
                                                                              i < size; i++) {
                                                                             store[i] = (float) array.getValueAtIndex(
                                                                                     runtime,
                                                                                     i).asNumber();
                                                                         }

                                                                         rust::Slice<const float> buf(
                                                                                 store.data(),
                                                                                 store.size());
                                                                         canvas_native_context_round_rect(
                                                                                 this->GetContext(),
                                                                                 x, y,
                                                                                 width,
                                                                                 height, buf);

                                                                     }
                                                                 }
                                                             } else {
                                                                 auto radii = (float) arguments[4].asNumber();
                                                                 canvas_native_context_round_rect_tl_tr_br_bl(
                                                                         this->GetContext(), x, y,
                                                                         width,
                                                                         height, radii, radii,
                                                                         radii, radii);

                                                             }
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "rect") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 4) {
                                                             auto x = static_cast<float>(arguments[0].asNumber());
                                                             auto y = static_cast<float>(arguments[1].asNumber());
                                                             auto width = static_cast<float>(arguments[2].asNumber());
                                                             auto height = static_cast<float>(arguments[3].asNumber());
                                                             canvas_native_context_rect(
                                                                     this->GetContext(), x, y,
                                                                     width,
                                                                     height);
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "removeHitRegion") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [](jsi::Runtime &runtime,
                                                        const jsi::Value &thisValue,
                                                        const jsi::Value *arguments,
                                                        size_t count) -> jsi::Value {

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "resetTransform") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         canvas_native_context_reset_transform(
                                                                 this->GetContext());
                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "restore") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         canvas_native_context_restore(
                                                                 this->GetContext());
                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "rotate") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 1 &&
                                                             arguments[0].isNumber()) {
                                                             canvas_native_context_rotate(
                                                                     this->GetContext(),
                                                                     (float) arguments[0].asNumber());
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "save") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         canvas_native_context_save(
                                                                 this->GetContext());
                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "scale") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 2) {
                                                             auto x = static_cast<float>(arguments[0].asNumber());
                                                             auto y = static_cast<float>(arguments[1].asNumber());
                                                             canvas_native_context_scale(
                                                                     this->GetContext(), x, y);
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "scrollPathIntoView") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [](jsi::Runtime &runtime,
                                                        const jsi::Value &thisValue,
                                                        const jsi::Value *arguments,
                                                        size_t count) -> jsi::Value {
                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "setLineDash") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 1 &&
                                                             arguments[0].isObject()) {
                                                             auto vecObject = arguments[0].asObject(
                                                                     runtime);
                                                             if (vecObject.isArray(
                                                                     runtime)) {
                                                                 auto segments = vecObject.getArray(
                                                                         runtime);
                                                                 auto len = segments.size(runtime);
                                                                 std::vector<float> data;
                                                                 for (int i = 0; i < len; ++i) {
                                                                     auto item = segments.getValueAtIndex(
                                                                             runtime, i);
                                                                     data.push_back(
                                                                             static_cast<float>(item.asNumber()));
                                                                 }
                                                                 rust::Slice<const float> slice{
                                                                         data.data(), data.size()};
                                                                 canvas_native_context_set_line_dash(
                                                                         this->GetContext(), slice);
                                                             }
                                                         }
                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "setTransform") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     6,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (count == 1 &&
                                                             arguments[0].isObject()) {
                                                             auto matrix = getHostObject<MatrixImpl>(
                                                                     runtime, arguments[0]);
                                                             if (matrix != nullptr) {
                                                                 canvas_native_context_set_transform_matrix(
                                                                         this->GetContext(),
                                                                         matrix->GetMatrix());
                                                             }
                                                         } else if (count == 6) {
                                                             auto a = static_cast<float>(arguments[0].asNumber());
                                                             auto b = static_cast<float>(arguments[1].asNumber());
                                                             auto c = static_cast<float>(arguments[2].asNumber());
                                                             auto d = static_cast<float>(arguments[3].asNumber());
                                                             auto e = static_cast<float>(arguments[4].asNumber());
                                                             auto f = static_cast<float>(arguments[5].asNumber());
                                                             canvas_native_context_set_transform(
                                                                     this->GetContext(), a, b, c, d,
                                                                     e,
                                                                     f);
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "stroke") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 1 &&
                                                             arguments[0].isObject()) {
                                                             auto path = getHostObject<Path2D>(
                                                                     runtime, arguments[0]);
                                                             if (path != nullptr) {
                                                                 canvas_native_context_stroke_with_path(
                                                                         this->GetContext(),
                                                                         path->GetPath());
                                                                 this->UpdateInvalidateState();
                                                             }
                                                         } else {
                                                             canvas_native_context_stroke(
                                                                     this->GetContext());
                                                             this->UpdateInvalidateState();
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "strokeRect") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 4) {
                                                             auto x = static_cast<float>(arguments[0].asNumber());
                                                             auto y = static_cast<float>(arguments[1].asNumber());
                                                             auto width = static_cast<float>(arguments[2].asNumber());
                                                             auto height = static_cast<float>(arguments[3].asNumber());
                                                             canvas_native_context_stroke_rect(
                                                                     this->GetContext(), x, y,
                                                                     width,
                                                                     height);
                                                             this->UpdateInvalidateState();
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "strokeText") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count >= 3) {
                                                             auto text = arguments[0].asString(
                                                                     runtime).utf8(runtime);
                                                             auto x = static_cast<float>(arguments[1].asNumber());
                                                             auto y = static_cast<float>(arguments[2].asNumber());
                                                             float maxWidth = -1;

                                                             if (count > 3) {
                                                                 maxWidth = static_cast<float>(arguments[3].asNumber());
                                                             }

                                                             canvas_native_context_stroke_text(
                                                                     this->GetContext(),
                                                                     rust::Str(text.c_str()),
                                                                     x, y, maxWidth);
                                                             this->UpdateInvalidateState();
                                                         }
                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "transform") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     6,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 6) {
                                                             auto a = static_cast<float>(arguments[0].asNumber());
                                                             auto b = static_cast<float>(arguments[1].asNumber());
                                                             auto c = static_cast<float>(arguments[2].asNumber());
                                                             auto d = static_cast<float>(arguments[3].asNumber());
                                                             auto e = static_cast<float>(arguments[4].asNumber());
                                                             auto f = static_cast<float>(arguments[5].asNumber());
                                                             canvas_native_context_transform(
                                                                     this->GetContext(), a, b, c, d,
                                                                     e,
                                                                     f);
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "translate") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 2) {
                                                             auto x = static_cast<float>(arguments[0].asNumber());
                                                             auto y = static_cast<float>(arguments[1].asNumber());
                                                             canvas_native_context_translate(
                                                                     this->GetContext(), x, y);
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "__toDataURL") {

        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         std::string type("image/png");
                                                         int quality = 92;
                                                         if (arguments[0].isString()) {
                                                             type = arguments[0].asString(
                                                                     runtime).utf8(
                                                                     runtime);
                                                         }


                                                         if (arguments[1].isNumber()) {
                                                             quality = (int) arguments[1].asNumber();
                                                         }


                                                         rust::Str str(type);

                                                         auto data = canvas_native_to_data_url(
                                                                 this->GetContext(), str,
                                                                 quality);
                                                         return jsi::String::createFromAscii(
                                                                 runtime,
                                                                 data.data(),
                                                                 data.size());
                                                     }
        );
    }


    return jsi::Value::undefined();
}

CanvasRenderingContext2DImpl::~CanvasRenderingContext2DImpl() {
    auto raf = this->raf_.get();
    if (raf != nullptr) {
        canvas_native_raf_stop(raf->GetRaf());
    }
}

void CanvasRenderingContext2DImpl::UpdateInvalidateState() {
    auto raf = this->GetRaf();
    if (raf != nullptr) {
        if (!canvas_native_raf_get_started(raf->GetRaf())) {
            canvas_native_raf_start(raf->GetRaf());
        }
    }

    auto state = this->GetInvalidateState();
    this->SetInvalidateState((int) state | (int) InvalidateState::PENDING);
}

InvalidateState CanvasRenderingContext2DImpl::GetInvalidateState() const {
    return (InvalidateState)
            this->invalidateState_;
}

void CanvasRenderingContext2DImpl::SetInvalidateState(InvalidateState state) {
    this->invalidateState_ = (int) state;
}

void CanvasRenderingContext2DImpl::SetInvalidateState(int state) {
    this->invalidateState_ = state;
}


void CanvasRenderingContext2DImpl::Flush() {
    auto state = (int) this->GetInvalidateState() & (int) InvalidateState::PENDING;
    if (state == (int) InvalidateState::PENDING) {
        canvas_native_context_flush(this->GetContext());
        this->SetInvalidateState(InvalidateState::INVALIDATING);
        canvas_native_context_render(this->GetContext());
//        canvas_native_context_gl_make_current(this->GetContext());
//        canvas_native_context_gl_swap_buffers(this->GetContext());
        this->SetInvalidateState(InvalidateState::NONE);
    }
}

void CanvasRenderingContext2DImpl::Flush(intptr_t context) {
    auto ctx = reinterpret_cast<CanvasRenderingContext2DImpl *>(reinterpret_cast<intptr_t *>(context));
    if (ctx != nullptr) {
        ctx->Flush();
    }
}


CanvasRenderingContext2D &CanvasRenderingContext2DImpl::GetContext() {
    return *this->context_;
}
