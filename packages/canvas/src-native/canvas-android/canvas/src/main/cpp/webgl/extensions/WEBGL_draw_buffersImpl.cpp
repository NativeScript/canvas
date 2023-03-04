//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_draw_buffersImpl.h"

WEBGL_draw_buffersImpl::WEBGL_draw_buffersImpl(rust::Box<WEBGL_draw_buffers> buffers) : buffers_(
        std::move(buffers)) {

}

jsi::Value WEBGL_draw_buffersImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "drawBuffersWEBGL") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         if (arguments[0].isObject()) {
                                                             auto buffersObject = arguments[0].asObject(
                                                                     runtime);
                                                             if (buffersObject.isArray(runtime)) {
                                                                 auto buffers = buffersObject.getArray(
                                                                         runtime);
                                                                 auto len = buffers.size(runtime);
                                                                 rust::Vec<uint32_t> buf;
                                                                 buf.reserve(len);
                                                                 for (int j = 0; j < len; ++j) {
                                                                     auto item = buffers.getValueAtIndex(
                                                                             runtime, j);
                                                                     if (!item.isNumber()) {
                                                                         // todo verify
                                                                         buf.push_back(0);
                                                                     } else {
                                                                         buf.push_back(
                                                                                 (uint32_t) item.asNumber());
                                                                     }

                                                                 }
                                                                 rust::Slice<const uint32_t> slice(
                                                                         buf.data(), buf.size());
                                                                 canvas_native_webgl_draw_buffers_draw_buffers_webgl(
                                                                         slice,
                                                                         this->GetDrawBuffers());
                                                             }

                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "COLOR_ATTACHMENT0_WEBGL") {
        return {GL_COLOR_ATTACHMENT0_EXT};
    } else if (methodName == "COLOR_ATTACHMENT1_WEBGL") {
        return {GL_COLOR_ATTACHMENT1_EXT};
    } else if (methodName == "COLOR_ATTACHMENT2_WEBGL") {
        return {GL_COLOR_ATTACHMENT2_EXT};
    } else if (methodName == "COLOR_ATTACHMENT3_WEBGL") {
        return {GL_COLOR_ATTACHMENT3_EXT};
    } else if (methodName == "COLOR_ATTACHMENT4_WEBGL") {
        return {GL_COLOR_ATTACHMENT4_EXT};
    } else if (methodName == "COLOR_ATTACHMENT5_WEBGL") {
        return {GL_COLOR_ATTACHMENT5_EXT};
    } else if (methodName == "COLOR_ATTACHMENT6_WEBGL") {
        return {GL_COLOR_ATTACHMENT6_EXT};
    } else if (methodName == "COLOR_ATTACHMENT7_WEBGL") {
        return {GL_COLOR_ATTACHMENT7_EXT};
    } else if (methodName == "COLOR_ATTACHMENT8_WEBGL") {
        return {GL_COLOR_ATTACHMENT8_EXT};
    } else if (methodName == "COLOR_ATTACHMENT9_WEBGL") {
        return {GL_COLOR_ATTACHMENT9_EXT};
    } else if (methodName == "COLOR_ATTACHMENT10_WEBGL") {
        return {GL_COLOR_ATTACHMENT10_EXT};
    } else if (methodName == "COLOR_ATTACHMENT11_WEBGL") {
        return {GL_COLOR_ATTACHMENT11_EXT};
    } else if (methodName == "COLOR_ATTACHMENT12_WEBGL") {
        return {GL_COLOR_ATTACHMENT12_EXT};
    } else if (methodName == "COLOR_ATTACHMENT13_WEBGL") {
        return {GL_COLOR_ATTACHMENT13_EXT};
    } else if (methodName == "COLOR_ATTACHMENT14_WEBGL") {
        return {GL_COLOR_ATTACHMENT14_EXT};
    } else if (methodName == "COLOR_ATTACHMENT15_WEBGL") {
        return {GL_COLOR_ATTACHMENT15_EXT};
    } else if (methodName == "DRAW_BUFFER0_WEBGL") {
        return {GL_DRAW_BUFFER0_EXT};
    } else if (methodName == "DRAW_BUFFER1_WEBGL") {
        return {GL_DRAW_BUFFER1_EXT};
    } else if (methodName == "DRAW_BUFFER2_WEBGL") {
        return {GL_DRAW_BUFFER2_EXT};
    } else if (methodName == "DRAW_BUFFER3_WEBGL") {
        return {GL_DRAW_BUFFER3_EXT};
    } else if (methodName == "DRAW_BUFFER4_WEBGL") {
        return {GL_DRAW_BUFFER4_EXT};
    } else if (methodName == "DRAW_BUFFER5_WEBGL") {
        return {GL_DRAW_BUFFER5_EXT};
    } else if (methodName == "DRAW_BUFFER6_WEBGL") {
        return {GL_DRAW_BUFFER6_EXT};
    } else if (methodName == "DRAW_BUFFER0_WEBGL") {
        return {GL_DRAW_BUFFER0_EXT};
    } else if (methodName == "DRAW_BUFFER7_WEBGL") {
        return {GL_DRAW_BUFFER7_EXT};
    } else if (methodName == "DRAW_BUFFER8_WEBGL") {
        return {GL_DRAW_BUFFER8_EXT};
    } else if (methodName == "DRAW_BUFFER9_WEBGL") {
        return {GL_DRAW_BUFFER9_EXT};
    } else if (methodName == "DRAW_BUFFER10_WEBGL") {
        return {GL_DRAW_BUFFER10_EXT};
    } else if (methodName == "DRAW_BUFFER11_WEBGL") {
        return {GL_DRAW_BUFFER11_EXT};
    } else if (methodName == "DRAW_BUFFER12_WEBGL") {
        return {GL_DRAW_BUFFER12_EXT};
    } else if (methodName == "DRAW_BUFFER13_WEBGL") {
        return {GL_DRAW_BUFFER13_EXT};
    } else if (methodName == "DRAW_BUFFER14_WEBGL") {
        return {GL_DRAW_BUFFER14_EXT};
    } else if (methodName == "DRAW_BUFFER15_WEBGL") {
        return {GL_DRAW_BUFFER15_EXT};
    } else if (methodName == "MAX_COLOR_ATTACHMENTS_WEBGL") {
        return {GL_MAX_COLOR_ATTACHMENTS_EXT};
    } else if (methodName == "MAX_DRAW_BUFFERS_WEBGL") {
        return {GL_MAX_DRAW_BUFFERS_EXT};
    }

    return jsi::Value::undefined();
}

std::vector<jsi::PropNameID> WEBGL_draw_buffersImpl::getPropertyNames(jsi::Runtime &rt) {
    return {
            jsi::PropNameID::forUtf8(rt, std::string("drawBuffersWEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT0_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT1_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT2_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT3_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT4_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT5_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT6_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT7_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT8_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT9_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT10_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT11_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT12_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT13_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT14_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT15_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER0_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER1_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER2_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER3_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER4_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER5_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER6_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER7_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER8_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER9_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER10_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER11_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER12_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER13_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER14_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER15_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("MAX_COLOR_ATTACHMENTS_WEBGL")),
            jsi::PropNameID::forUtf8(rt, std::string("MAX_DRAW_BUFFERS_WEBGL")),
    };
}

