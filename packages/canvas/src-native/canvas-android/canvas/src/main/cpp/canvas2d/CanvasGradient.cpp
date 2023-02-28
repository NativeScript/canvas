//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasGradient.h"


CanvasGradient::CanvasGradient(rust::Box<PaintStyle> style) : style_(std::move(style)) {}


std::vector<jsi::PropNameID> CanvasGradient::getPropertyNames(jsi::Runtime &rt) {
    return {jsi::PropNameID::forUtf8(rt, std::string("addColorStop"))};
}

jsi::Value CanvasGradient::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "addColorStop") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         auto stop = (float) arguments[0].asNumber();
                                                         auto color = arguments[1].asString(
                                                                 runtime).utf8(runtime);
                                                         canvas_native_gradient_add_color_stop(
                                                                 this->GetPaintStyle(), stop,
                                                                 rust::Str(color.c_str(),
                                                                           color.size()));

                                                         return jsi::Value::undefined();
                                                     });
    }
    return jsi::Value::undefined();
}

PaintStyle &CanvasGradient::GetPaintStyle() {
    return *this->style_;
}

