//
// Created by Osei Fortune on 29/04/2022.
//

#include "OES_vertex_array_objectImpl.h"


OES_vertex_array_objectImpl::OES_vertex_array_objectImpl(OES_vertex_array_object* object)
        : object_(object) {

}

void OES_vertex_array_objectImpl::CreateVertexArrayOES(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    OES_vertex_array_objectImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }
    auto isolate = args.GetIsolate();

    auto ret = canvas_native_webgl_oes_vertex_array_object_create_vertex_array_oes(
            ptr->GetVertexArrayObject());
    auto vertex = WebGLVertexArrayObject::NewInstance(isolate, new WebGLVertexArrayObject(
            ret));

    args.GetReturnValue().Set(vertex);
}


void OES_vertex_array_objectImpl::DeleteVertexArrayOES(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    OES_vertex_array_objectImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();

    auto value = args[0];
    auto type = GetNativeType(isolate, value);

    if (type == NativeType::WebGLVertexArrayObject) {
        auto array_object = WebGLVertexArrayObject::GetPointer(value.As<v8::Object>());
        if (array_object != nullptr) {
            canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(
                    array_object->GetVertexArrayObject(),
                    ptr->GetVertexArrayObject()
            );
        }
    }
}

void OES_vertex_array_objectImpl::IsVertexArrayOES(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    OES_vertex_array_objectImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().Set(false);
        return;
    }
    auto isolate = args.GetIsolate();

    auto value = args[0];
    auto type = GetNativeType(isolate, value);

    if (type == NativeType::WebGLVertexArrayObject) {
        auto array_object = WebGLVertexArrayObject::GetPointer(value.As<v8::Object>());
        if (array_object != nullptr) {
            auto ret = canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(
                    array_object->GetVertexArrayObject(),
                    ptr->GetVertexArrayObject()
            );

            args.GetReturnValue().Set(ret);
            return;
        }
    }

    args.GetReturnValue().Set(false);
}


void OES_vertex_array_objectImpl::BindVertexArrayOES(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    OES_vertex_array_objectImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();

    auto value = args[0];
    auto type = GetNativeType(isolate, value);

    if (type == NativeType::WebGLVertexArrayObject) {
        auto array_object = WebGLVertexArrayObject::GetPointer(value.As<v8::Object>());
        if (array_object != nullptr) {
            canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(
                    array_object->GetVertexArrayObject(),
                    ptr->GetVertexArrayObject()
            );
        }
    }
}
