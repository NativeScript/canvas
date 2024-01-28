//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "gl.h"
#include "WebGLVertexArrayObject.h"
#include "Helpers.h"
#include <vector>
#include "ObjectWrapperImpl.h"

class OES_vertex_array_objectImpl : ObjectWrapperImpl {
public:
    OES_vertex_array_objectImpl(OES_vertex_array_object *object);

    static v8::CFunction fast_delete_vertex_array_oes_;

    static v8::CFunction fast_is_vertex_array_oes_;

    static v8::CFunction fast_bind_vertex_array_oes_;

    ~OES_vertex_array_objectImpl() {
        canvas_native_webgl_OES_vertex_array_object_destroy(this->GetVertexArrayObject());
        this->object_ = nullptr;
    }

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->OES_vertex_array_objectTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "OES_vertex_array_object"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);
        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "OES_vertex_array_object"));
        tmpl->Set(ConvertToV8String(isolate, "VERTEX_ARRAY_BINDING_OES"),
                  v8::Integer::NewFromUnsigned(isolate, GL_VERTEX_ARRAY_BINDING_OES));


        tmpl->Set(ConvertToV8String(isolate, "createVertexArrayOES"),
                  v8::FunctionTemplate::New(isolate, &CreateVertexArrayOES));


        SetFastMethod(isolate, tmpl, "deleteVertexArrayOES", DeleteVertexArrayOES,
                      &fast_delete_vertex_array_oes_, v8::Local<v8::Value>());


        SetFastMethod(isolate, tmpl, "isVertexArrayOES", IsVertexArrayOES,
                      &fast_is_vertex_array_oes_, v8::Local<v8::Value>());


        SetFastMethod(isolate, tmpl, "bindVertexArrayOES", BindVertexArrayOES,
                      &fast_bind_vertex_array_oes_, v8::Local<v8::Value>());


        cache->OES_vertex_array_objectTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, OES_vertex_array_objectImpl *vertexArrayObject) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = OES_vertex_array_objectImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::OES_vertex_array_object);
        object->SetAlignedPointerInInternalField(0, vertexArrayObject);
        vertexArrayObject->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static OES_vertex_array_objectImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<OES_vertex_array_objectImpl *>(ptr);
    }

    static void CreateVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastDeleteVertexArrayOES(
            v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> vertex_obj) {
        OES_vertex_array_objectImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(vertex_obj);

        if (type == NativeType::WebGLVertexArrayObject) {
            auto array_object = WebGLVertexArrayObject::GetPointer(vertex_obj);
            if (array_object != nullptr) {
                canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(
                        array_object->GetVertexArrayObject(),
                        ptr->GetVertexArrayObject()
                );
            }
        }
    }


    static void DeleteVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args);

    static bool FastIsVertexArrayOES(
            v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> vertex_obj) {
        OES_vertex_array_objectImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return false;
        }
        auto type = GetNativeType(vertex_obj);

        if (type == NativeType::WebGLVertexArrayObject) {
            auto array_object = WebGLVertexArrayObject::GetPointer(vertex_obj);
            if (array_object != nullptr) {
                auto ret = canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(
                        array_object->GetVertexArrayObject(),
                        ptr->GetVertexArrayObject()
                );

                return ret;
            }
        }

        return false;
    }

    static void BindVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastBindVertexArrayOES(
            v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> vertex_obj) {
        OES_vertex_array_objectImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(vertex_obj);

        if (type == NativeType::WebGLVertexArrayObject) {
            auto array_object = WebGLVertexArrayObject::GetPointer(vertex_obj);
            if (array_object != nullptr) {
                canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(
                        array_object->GetVertexArrayObject(),
                        ptr->GetVertexArrayObject()
                );
            }
        }
    }


    OES_vertex_array_object *GetVertexArrayObject() {
        return this->object_;
    }

private:
    OES_vertex_array_object *object_;
};
