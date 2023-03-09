//
// Created by Osei Fortune on 19/04/2022.
//

#include "TextEncoderImpl.h"

TextEncoderImpl::TextEncoderImpl(rust::Box<TextEncoder> encoder) : encoder_(std::move(encoder)) {}

std::vector<jsi::PropNameID> TextEncoderImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(2);
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("encoding")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("encode")));
    return ret;
}

jsi::Value TextEncoderImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "encoding") {
        auto encoding = canvas_native_text_encoder_get_encoding(this->GetTextEncoder());
        return jsi::String::createFromAscii(runtime, encoding.data(), encoding.size());
    } else if (methodName == "encode") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {


                                                         auto text = arguments[0].asString(
                                                                 runtime).utf8(runtime);

                                                         auto data = canvas_native_text_encoder_encode(
                                                                 this->GetTextEncoder(),
                                                                 rust::Str(text.c_str()));

                                                         auto buf = std::make_shared<VecMutableBuffer<uint8_t>>(
                                                                 std::move(data));
                                                         auto ab = jsi::ArrayBuffer(runtime, buf);
                                                         auto Uint8ClampedArray = runtime.global()
                                                                 .getProperty(runtime,
                                                                              "Uint8ClampedArray")
                                                                 .asObject(runtime)
                                                                 .asFunction(runtime);


                                                         return Uint8ClampedArray.callAsConstructor(
                                                                 runtime, ab);
                                                     }
        );
    }

    return jsi::Value::undefined();
}

TextEncoder &TextEncoderImpl::GetTextEncoder() {
    return *this->encoder_;
}
