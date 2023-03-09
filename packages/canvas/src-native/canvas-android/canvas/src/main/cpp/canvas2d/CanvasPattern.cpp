//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasPattern.h"
#include "canvas-cxx/src/lib.rs.h"

CanvasPattern::CanvasPattern(rust::Box<PaintStyle> style) : style_(std::move(style)) {}

std::vector<jsi::PropNameID> CanvasPattern::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.push_back(jsi::PropNameID::forAscii(rt, std::string("setTransform")));
    return ret;
}

jsi::Value CanvasPattern::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "setTransform") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (!arguments[0].isUndefined() &&
                                                             !arguments[0].isNull()) {
                                                             auto matrix = arguments[0].asObject(
                                                                     runtime).asHostObject<MatrixImpl>(
                                                                     runtime);
                                                             if (matrix != nullptr) {
                                                                 canvas_native_pattern_set_transform(
                                                                         this->GetPaintStyle(),
                                                                         matrix->GetMatrix());
                                                             }
                                                         }

                                                         return jsi::Value::undefined();
                                                     });
    }

    return jsi::Value::undefined();
}

PaintStyle &CanvasPattern::GetPaintStyle() {
    return *this->style_;
}
