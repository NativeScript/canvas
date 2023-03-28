//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_draw_buffersImpl.h"

WEBGL_draw_buffersImpl::WEBGL_draw_buffersImpl(rust::Box<WEBGL_draw_buffers> buffers) : buffers_(
        std::move(buffers)) {

}

std::vector<jsi::PropNameID> WEBGL_draw_buffersImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(36);
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("drawBuffersWEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT0_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT1_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT2_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT3_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT4_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT5_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT6_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT7_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT8_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT9_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT10_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT11_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT12_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT13_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT14_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("COLOR_ATTACHMENT15_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER0_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER1_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER2_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER3_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER4_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER5_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER6_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER7_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER8_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER9_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER10_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER11_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER12_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER13_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER14_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("DRAW_BUFFER15_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("MAX_COLOR_ATTACHMENTS_WEBGL")));
    ret.emplace_back(jsi::PropNameID::forUtf8(rt, std::string("MAX_DRAW_BUFFERS_WEBGL")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("ext_name")));
    return ret;
}

jsi::Value WEBGL_draw_buffersImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);

    if (methodName == "ext_name") {
        return jsi::String::createFromAscii(runtime, "WEBGL_draw_buffers");
    }

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
        return {GL_COLOR_ATTACHMENT0};
    } else if (methodName == "COLOR_ATTACHMENT1_WEBGL") {
        return {GL_COLOR_ATTACHMENT1};
    } else if (methodName == "COLOR_ATTACHMENT2_WEBGL") {
        return {GL_COLOR_ATTACHMENT2};
    } else if (methodName == "COLOR_ATTACHMENT3_WEBGL") {
        return {GL_COLOR_ATTACHMENT3};
    } else if (methodName == "COLOR_ATTACHMENT4_WEBGL") {
        return {GL_COLOR_ATTACHMENT4};
    } else if (methodName == "COLOR_ATTACHMENT5_WEBGL") {
        return {GL_COLOR_ATTACHMENT5};
    } else if (methodName == "COLOR_ATTACHMENT6_WEBGL") {
        return {GL_COLOR_ATTACHMENT6};
    } else if (methodName == "COLOR_ATTACHMENT7_WEBGL") {
        return {GL_COLOR_ATTACHMENT7};
    } else if (methodName == "COLOR_ATTACHMENT8_WEBGL") {
        return {GL_COLOR_ATTACHMENT8};
    } else if (methodName == "COLOR_ATTACHMENT9_WEBGL") {
        return {GL_COLOR_ATTACHMENT9};
    } else if (methodName == "COLOR_ATTACHMENT10_WEBGL") {
        return {GL_COLOR_ATTACHMENT10};
    } else if (methodName == "COLOR_ATTACHMENT11_WEBGL") {
        return {GL_COLOR_ATTACHMENT11};
    } else if (methodName == "COLOR_ATTACHMENT12_WEBGL") {
        return {GL_COLOR_ATTACHMENT12};
    } else if (methodName == "COLOR_ATTACHMENT13_WEBGL") {
        return {GL_COLOR_ATTACHMENT13};
    } else if (methodName == "COLOR_ATTACHMENT14_WEBGL") {
        return {GL_COLOR_ATTACHMENT14};
    } else if (methodName == "COLOR_ATTACHMENT15_WEBGL") {
        return {GL_COLOR_ATTACHMENT15};
    } else if (methodName == "DRAW_BUFFER0_WEBGL") {
        return {GL_DRAW_BUFFER0};
    } else if (methodName == "DRAW_BUFFER1_WEBGL") {
        return {GL_DRAW_BUFFER1};
    } else if (methodName == "DRAW_BUFFER2_WEBGL") {
        return {GL_DRAW_BUFFER2};
    } else if (methodName == "DRAW_BUFFER3_WEBGL") {
        return {GL_DRAW_BUFFER3};
    } else if (methodName == "DRAW_BUFFER4_WEBGL") {
        return {GL_DRAW_BUFFER4};
    } else if (methodName == "DRAW_BUFFER5_WEBGL") {
        return {GL_DRAW_BUFFER5};
    } else if (methodName == "DRAW_BUFFER6_WEBGL") {
        return {GL_DRAW_BUFFER6};
    } else if (methodName == "DRAW_BUFFER7_WEBGL") {
        return {GL_DRAW_BUFFER7};
    } else if (methodName == "DRAW_BUFFER8_WEBGL") {
        return {GL_DRAW_BUFFER8};
    } else if (methodName == "DRAW_BUFFER9_WEBGL") {
        return {GL_DRAW_BUFFER9};
    } else if (methodName == "DRAW_BUFFER10_WEBGL") {
        return {GL_DRAW_BUFFER10};
    } else if (methodName == "DRAW_BUFFER11_WEBGL") {
        return {GL_DRAW_BUFFER11};
    } else if (methodName == "DRAW_BUFFER12_WEBGL") {
        return {GL_DRAW_BUFFER12};
    } else if (methodName == "DRAW_BUFFER13_WEBGL") {
        return {GL_DRAW_BUFFER13};
    } else if (methodName == "DRAW_BUFFER14_WEBGL") {
        return {GL_DRAW_BUFFER14};
    } else if (methodName == "DRAW_BUFFER15_WEBGL") {
        return {GL_DRAW_BUFFER15};
    } else if (methodName == "MAX_COLOR_ATTACHMENTS_WEBGL") {
        return {GL_MAX_COLOR_ATTACHMENTS};
    } else if (methodName == "MAX_DRAW_BUFFERS_WEBGL") {
        return {GL_MAX_DRAW_BUFFERS};
    }

    return jsi::Value::undefined();
}


