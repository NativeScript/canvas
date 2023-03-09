//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_lose_contextImpl.h"


WEBGL_lose_contextImpl::WEBGL_lose_contextImpl(rust::Box<WEBGL_lose_context> context) : context_(
        std::move(context)) {}

jsi::Value WEBGL_lose_contextImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "loseContext") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         canvas_native_webgl_lose_context_lose_context(
                                                                 this->GetContext());
                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "restoreContext") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         canvas_native_webgl_lose_context_restore_context(
                                                                 this->GetContext());

                                                         return jsi::Value::undefined();
                                                     }
        );
    }
    return jsi::Value::undefined();
}

std::vector<jsi::PropNameID> WEBGL_lose_contextImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(2);
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("loseContext")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("restoreContext")));
    return ret;
}
