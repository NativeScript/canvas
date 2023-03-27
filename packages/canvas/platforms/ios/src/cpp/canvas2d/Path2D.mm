//
// Created by Osei Fortune on 28/03/2022.
//

#include "Path2D.h"


Path2D::Path2D(rust::Box<Path> path)
        : path_(std::move(path)) {}

std::vector<jsi::PropNameID> Path2D::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(11);
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("addPath")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("arc")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("arcTo")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("bezierCurveTo")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("closePath")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("ellipse")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("lineTo")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("moveTo")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("quadraticCurveTo")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("rect")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("roundRect")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("__toSVG")));
    return ret;
}

jsi::Value Path2D::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "addPath") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         auto object = getHostObject<Path2D>(
                                                                 runtime, arguments[0]);
                                                         if (object != nullptr) {
                                                             canvas_native_path_add_path(
                                                                     this->GetPath(),
                                                                     object->GetPath());
                                                         }
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

                                                         canvas_native_path_arc(
                                                                 this->GetPath(),
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


                                                         canvas_native_path_arc_to(
                                                                 this->GetPath(),
                                                                 static_cast<float>(arguments[0].asNumber()),
                                                                 static_cast<float>(arguments[1].asNumber()),
                                                                 static_cast<float>(arguments[2].asNumber()),
                                                                 static_cast<float>(arguments[3].asNumber()),
                                                                 static_cast<float>(arguments[4].asNumber())
                                                         );

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


                                                         canvas_native_path_bezier_curve_to(
                                                                 this->GetPath(),
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

    } else if (methodName == "closePath") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         canvas_native_path_close_path(
                                                                 this->GetPath());

                                                         return jsi::Value::undefined();
                                                     }
        );

    } else if (methodName == "ellipse") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     7,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         auto anti_clockwise = false;
                                                         if (count > 7) {
                                                             anti_clockwise = arguments[7].asBool();
                                                         }
                                                         canvas_native_path_ellipse(
                                                                 this->GetPath(),
                                                                 static_cast<float>(arguments[0].asNumber()),
                                                                 static_cast<float>(arguments[1].asNumber()),
                                                                 static_cast<float>(arguments[2].asNumber()),
                                                                 static_cast<float>(arguments[3].asNumber()),
                                                                 static_cast<float>(arguments[4].asNumber()),
                                                                 static_cast<float>(arguments[5].asNumber()),
                                                                 static_cast<float>(arguments[6].asNumber()),
                                                                 anti_clockwise
                                                         );

                                                         return jsi::Value::undefined();
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


                                                         canvas_native_path_line_to(this->GetPath(),
                                                                                    static_cast<float>(arguments[0].asNumber()),
                                                                                    static_cast<float>(arguments[1].asNumber()));

                                                         return jsi::Value::undefined();
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


                                                         canvas_native_path_move_to(this->GetPath(),
                                                                                    static_cast<float>(arguments[0].asNumber()),
                                                                                    static_cast<float>(arguments[1].asNumber()));

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


                                                         canvas_native_path_quadratic_curve_to(
                                                                 this->GetPath(),
                                                                 static_cast<float>(arguments[0].asNumber()),
                                                                 static_cast<float>(arguments[1].asNumber()),
                                                                 static_cast<float>(arguments[2].asNumber()),
                                                                 static_cast<float>(arguments[3].asNumber())
                                                         );

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


                                                         canvas_native_path_rect(
                                                                 this->GetPath(),
                                                                 static_cast<float>(arguments[0].asNumber()),
                                                                 static_cast<float>(arguments[1].asNumber()),
                                                                 static_cast<float>(arguments[2].asNumber()),
                                                                 static_cast<float>(arguments[3].asNumber())
                                                         );

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
                                                                         canvas_native_path_round_rect(
                                                                                 this->GetPath(),
                                                                                 x, y,
                                                                                 width,
                                                                                 height, buf);

                                                                     }
                                                                 }
                                                             } else {
                                                                 auto radii = (float) arguments[4].asNumber();
                                                                 canvas_native_path_round_rect_tl_tr_br_bl(
                                                                         this->GetPath(), x, y,
                                                                         width,
                                                                         height, radii, radii,
                                                                         radii, radii);

                                                             }
                                                         }

                                                         return jsi::Value::undefined();

                                                     }
        );
    } else if (methodName == "__toSVG") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         auto d = canvas_native_path_to_string(
                                                                 this->GetPath());
                                                         return jsi::String::createFromAscii(
                                                                 runtime, d.data(), d.size());
                                                     }
        );

    }

    return jsi::Value::undefined();
}

Path &Path2D::GetPath() {
    return *this->path_;
}

