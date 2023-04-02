//
// Created by Osei Fortune on 28/04/2022.
//

#include "EXT_disjoint_timer_queryImpl.h"

EXT_disjoint_timer_queryImpl::EXT_disjoint_timer_queryImpl(
        rust::Box<EXT_disjoint_timer_query> query) : query_(
        std::move(query)) {}

std::vector<jsi::PropNameID> EXT_disjoint_timer_queryImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(17);
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("QUERY_COUNTER_BITS_EXT")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("CURRENT_QUERY_EXT")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("QUERY_RESULT_EXT")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("QUERY_RESULT_AVAILABLE_EXT")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("TIME_ELAPSED_EXT")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("TIMESTAMP_EXT")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("GPU_DISJOINT_EXT")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("createQueryExt")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("deleteQueryExt")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("isQueryExt")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("beginQueryExt")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("endQueryExt")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("queryCounterExt")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("getQueryExt")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("getQueryObjectExt")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("getQueryParameterExt")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("ext_name")));
    return ret;
}


jsi::Value EXT_disjoint_timer_queryImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);

    if (methodName == "ext_name") {
        return jsi::String::createFromAscii(runtime,"EXT_disjoint_timer_query");
    }

    if (methodName == "QUERY_COUNTER_BITS_EXT") {
        return {0x8864};
    } else if (methodName == "CURRENT_QUERY_EXT") {
        return {0x8865};
    } else if (methodName == "QUERY_RESULT_EXT") {
        return {0x8866};
    } else if (methodName == "QUERY_RESULT_AVAILABLE_EXT") {
        return {0x8867};
    } else if (methodName == "TIME_ELAPSED_EXT") {
        return {0x88BF};
    } else if (methodName == "TIMESTAMP_EXT") {
        return {0x8E28};
    } else if (methodName == "GPU_DISJOINT_EXT") {
        return {0x8FBB};
    } else if (methodName == "createQueryExt") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto ret = canvas_native_webgl_ext_disjoint_timer_query_create_query_ext(
                                                                 this->GetQuery());

                                                         auto object = std::make_shared<WebGLQuery>(
                                                                 ret);

                                                         return jsi::Object::createFromHostObject(
                                                                 runtime, object);
                                                     }
        );
    } else if (methodName == "deleteQueryExt") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (arguments[0].isObject()) {
                                                             auto query = arguments[0].asObject(
                                                                     runtime).asHostObject<WebGLQuery>(
                                                                     runtime);
                                                             if (query ==
                                                                 nullptr) { return jsi::Value::undefined(); }
                                                             canvas_native_webgl_ext_disjoint_timer_query_delete_query_ext(
                                                                     query->GetQuery(),
                                                                     this->GetQuery());
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "isQueryExt") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (arguments[0].isObject()) {
                                                             auto query = arguments[0].asObject(
                                                                     runtime).asHostObject<WebGLQuery>(
                                                                     runtime);
                                                             if (query == nullptr) {
                                                                 return {false};
                                                             }
                                                             auto ret = canvas_native_webgl_ext_disjoint_timer_query_is_query_ext(
                                                                     query->GetQuery(),
                                                                     this->GetQuery());

                                                             return {ret};
                                                         }

                                                         return {false};
                                                     }
        );
    } else if (methodName == "beginQueryExt") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         if (arguments[0].isNumber() &&
                                                             arguments[1].isObject()) {
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             auto query = arguments[1].asObject(
                                                                     runtime).asHostObject<WebGLQuery>(
                                                                     runtime);

                                                             canvas_native_webgl_ext_disjoint_timer_query_begin_query_ext(
                                                                     target,
                                                                     query->GetQuery(),
                                                                     this->GetQuery()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "endQueryExt") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (arguments[0].isNumber()) {
                                                             auto target = (uint32_t) arguments[0].asNumber();
                                                             canvas_native_webgl_ext_disjoint_timer_query_end_query_ext(
                                                                     target,
                                                                     this->GetQuery()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "queryCounterExt") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (arguments[1].isNumber() &&
                                                             arguments[0].isObject()) {
                                                             auto query = arguments[0].asObject(
                                                                     runtime).asHostObject<WebGLQuery>(
                                                                     runtime);
                                                             auto target = (uint32_t) arguments[1].isNumber();

                                                             canvas_native_webgl_ext_disjoint_timer_query_query_counter_ext(
                                                                     query->GetQuery(),
                                                                     target,
                                                                     this->GetQuery());
                                                         }
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "getQueryExt") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     2,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto target = (uint32_t) arguments[0].asNumber();
                                                         auto pname = (uint32_t) arguments[1].asNumber();
                                                         auto ret = canvas_native_webgl_ext_disjoint_timer_query_get_query_ext(
                                                                 target,
                                                                 pname,
                                                                 this->GetQuery()
                                                         );
                                                         return {ret};
                                                     }
        );
    } else if (methodName == "queryCounterEXT") {
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
    } else if (methodName == "getQueryObjectExt") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto query = arguments[0].asObject(
                                                                 runtime).asHostObject<WebGLQuery>(
                                                                 runtime);
                                                         auto pname = (uint32_t) arguments[1].asNumber();

                                                         auto ret = canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(
                                                                 query->GetQuery(),
                                                                 pname,
                                                                 this->GetQuery()
                                                         );

                                                         // GL_QUERY_RESULT_AVAILABLE_EXT
                                                         if (pname == 0x8867) {
                                                             return {canvas_native_webgl_result_get_bool(
                                                                     *ret)};
                                                         }

                                                         return {canvas_native_webgl_result_get_i32(
                                                                 *ret)};
                                                     }
        );

    } else if (methodName == "getQueryParameterExt") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto query = arguments[0].asObject(
                                                                 runtime).asHostObject<WebGLQuery>(
                                                                 runtime);
                                                         auto pname = (uint32_t) arguments[1].asNumber();

                                                         auto ret = canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(
                                                                 query->GetQuery(),
                                                                 pname,
                                                                 this->GetQuery()
                                                         );

                                                         // GL_QUERY_RESULT_AVAILABLE_EXT
                                                         if (pname == 0x8867) {
                                                             return {canvas_native_webgl_result_get_bool(
                                                                     *ret)};
                                                         }

                                                         return {canvas_native_webgl_result_get_i32(
                                                                 *ret)};
                                                     }
        );

    }
    return jsi::Value::undefined();
}

EXT_disjoint_timer_query &EXT_disjoint_timer_queryImpl::GetQuery() {
    return *this->query_;
}
