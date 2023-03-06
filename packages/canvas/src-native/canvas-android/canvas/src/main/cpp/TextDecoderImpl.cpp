//
// Created by Osei Fortune on 19/04/2022.
//

#include "TextDecoderImpl.h"

TextDecoderImpl::TextDecoderImpl(rust::Box<TextDecoder> decoder) : decoder_(std::move(decoder)) {}

std::vector<jsi::PropNameID> TextDecoderImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(2);
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("encoding")));
    ret.push_back(jsi::PropNameID::forUtf8(rt, std::string("decode")));
    return ret;
}

jsi::Value TextDecoderImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "encoding") {
        auto encoding = canvas_native_text_decoder_get_encoding(this->GetTextDecoder());
        return jsi::String::createFromAscii(runtime, encoding.data(), encoding.size());
    } else if (methodName == "decode") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (count == 1) {
                                                             auto buf = &arguments[0];
                                                             if (buf->isNull() ||
                                                                 buf->isUndefined() ||
                                                                 !buf->isObject() || !buf->asObject(
                                                                     runtime).isArrayBuffer(
                                                                     runtime)) {

                                                                 throw jsi::JSINativeException(
                                                                         "Failed to execute 'decode' on 'TextDecoder': The provided value is not of type '(ArrayBuffer or ArrayBufferView)'");
                                                             }

                                                             auto buffer = buf->asObject(
                                                                     runtime).getArrayBuffer(
                                                                     runtime);

                                                             auto data = buffer.data(runtime);
                                                             auto size = buffer.size(runtime);
                                                             rust::Slice<const uint8_t> slice{data,
                                                                                              size};
                                                             auto decoded = canvas_native_text_decoder_decode(
                                                                     this->GetTextDecoder(), slice);
                                                             return jsi::String::createFromAscii(
                                                                     runtime, decoded.data(),
                                                                     decoded.size());
                                                         }

                                                         return jsi::String::createFromUtf8(runtime,
                                                                                            "");
                                                     }
        );
    }

    return jsi::Value::undefined();
}

TextDecoder &TextDecoderImpl::GetTextDecoder() {
    return *this->decoder_;
}
