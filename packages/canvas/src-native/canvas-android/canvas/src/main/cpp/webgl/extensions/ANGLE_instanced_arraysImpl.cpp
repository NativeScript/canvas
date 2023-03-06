//
// Created by Osei Fortune on 29/04/2022.
//

#include "ANGLE_instanced_arraysImpl.h"

ANGLE_instanced_arraysImpl::ANGLE_instanced_arraysImpl(rust::Box<ANGLE_instanced_arrays> arrays)
        : arrays_(
        std::move(arrays)) {}

std::vector<jsi::PropNameID> ANGLE_instanced_arraysImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(4);
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("drawArraysInstancedANGLE")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("drawElementsInstancedANGLE")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("vertexAttribDivisorANGLE")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE")));
    return ret;
}

jsi::Value ANGLE_instanced_arraysImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE") {
        return {0x88FE};
    }
    if (methodName == "drawArraysInstancedANGLE") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     4,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         auto mode = (uint32_t) arguments[0].asNumber();
                                                         auto first = (int32_t) arguments[1].asNumber();
                                                         auto count_ = (int32_t) arguments[2].asNumber();
                                                         auto primcount = (int32_t) arguments[3].asNumber();

                                                         canvas_native_webgl_angle_instanced_arrays_draw_arrays_instanced_angle(
                                                                 mode,
                                                                 first,
                                                                 count_,
                                                                 primcount,
                                                                 this->GetArrays()
                                                         );

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "drawElementsInstancedANGLE") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     5,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto mode = (uint32_t) arguments[0].asNumber();
                                                         auto count_ = (int32_t) arguments[1].asNumber();
                                                         auto type = (uint32_t) arguments[2].asNumber();
                                                         auto offset = (int32_t) arguments[3].asNumber();
                                                         auto primcount = (int32_t) arguments[4].asNumber();
                                                         canvas_native_webgl_angle_instanced_arrays_draw_elements_instanced_angle(
                                                                 mode,
                                                                 count_,
                                                                 type,
                                                                 offset,
                                                                 primcount,
                                                                 this->GetArrays()
                                                         );

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "vertexAttribDivisorANGLE") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto index = (u_int32_t) arguments[0].asNumber();
                                                         auto divisor = (u_int32_t) arguments[1].asNumber();
                                                         canvas_native_webgl_angle_instanced_arrays_vertex_attrib_divisor_angle(
                                                                 index,
                                                                 divisor,
                                                                 this->GetArrays()
                                                         );

                                                         return jsi::Value::undefined();
                                                     }
        );
    }
    return jsi::Value::undefined();
}

ANGLE_instanced_arrays &ANGLE_instanced_arraysImpl::GetArrays() {
    return *this->arrays_;
}
