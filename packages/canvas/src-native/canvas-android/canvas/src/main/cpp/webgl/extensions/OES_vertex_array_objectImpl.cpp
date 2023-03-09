//
// Created by Osei Fortune on 29/04/2022.
//

#include "OES_vertex_array_objectImpl.h"


OES_vertex_array_objectImpl::OES_vertex_array_objectImpl(rust::Box<OES_vertex_array_object> object)
        : object_(
        std::move(object)) {

}

jsi::Value OES_vertex_array_objectImpl::get(jsi::Runtime &runtime, const jsi::PropNameID &name) {
    auto methodName = name.utf8(runtime);
    if (methodName == "VERTEX_ARRAY_BINDING_OES") {
        return {GL_VERTEX_ARRAY_BINDING_OES};
    } else if (methodName == "createVertexArrayOES") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     0,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {
                                                         auto ret = canvas_native_webgl_oes_vertex_array_object_create_vertex_array_oes(
                                                                 this->GetVertexArrayObject());
                                                         auto vertex = std::make_shared<WebGLVertexArrayObject>(
                                                                 ret);
                                                         return jsi::Object::createFromHostObject(
                                                                 runtime, vertex);
                                                     }
        );
    } else if (methodName == "deleteVertexArrayOES") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto array_object = getHostObject<WebGLVertexArrayObject>(
                                                                 runtime, arguments[0]);
                                                         if (array_object != nullptr) {
                                                             canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(
                                                                     array_object->GetVertexArrayObject(),
                                                                     this->GetVertexArrayObject()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    } else if (methodName == "isVertexArrayOES") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto vertexArray = getHostObject<WebGLVertexArrayObject>(
                                                                 runtime, arguments[0]);
                                                         if (vertexArray != nullptr) {
                                                             auto ret = canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(
                                                                     vertexArray->GetVertexArrayObject(),
                                                                     this->GetVertexArrayObject()
                                                             );
                                                             return {ret};
                                                         }

                                                         return {false};
                                                     }
        );
    } else if (methodName == "bindVertexArrayOES") {
        return jsi::Function::createFromHostFunction(runtime,
                                                     jsi::PropNameID::forAscii(runtime, methodName),
                                                     1,
                                                     [this](jsi::Runtime &runtime,
                                                            const jsi::Value &thisValue,
                                                            const jsi::Value *arguments,
                                                            size_t count) -> jsi::Value {

                                                         auto vertexArray = getHostObject<WebGLVertexArrayObject>(
                                                                 runtime, arguments[0]);
                                                         if (vertexArray != nullptr) {
                                                             canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(
                                                                     vertexArray->GetVertexArrayObject(),
                                                                     this->GetVertexArrayObject()
                                                             );
                                                         }

                                                         return jsi::Value::undefined();
                                                     }
        );
    }
    return jsi::Value::undefined();
}

std::vector<jsi::PropNameID> OES_vertex_array_objectImpl::getPropertyNames(jsi::Runtime &rt) {
    std::vector<jsi::PropNameID> ret;
    ret.reserve(5);
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("VERTEX_ARRAY_BINDING_OES")));

    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("createVertexArrayOES")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("deleteVertexArrayOES")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("isVertexArrayOES")));
    ret.emplace_back(
            jsi::PropNameID::forUtf8(rt, std::string("bindVertexArrayOES")));

    return ret;
}
