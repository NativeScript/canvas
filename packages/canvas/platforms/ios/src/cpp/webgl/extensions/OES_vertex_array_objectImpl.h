//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "canvas-cxx/src/lib.rs.h"
#include "rust/cxx.h"
#include "gl.h"
#include "../../webgl2/WebGLVertexArrayObject.h"
#include "Helpers.h"
#include <vector>

using namespace org::nativescript::canvas;

class OES_vertex_array_objectImpl {
public:
    OES_vertex_array_objectImpl(OES_vertex_array_object* object);
    ~OES_vertex_array_objectImpl(){
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
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "OES_vertex_array_object"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(1);
        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "OES_vertex_array_object"));
        tmpl->Set(ConvertToV8String(isolate, "VERTEX_ARRAY_BINDING_OES"),
                  v8::Number::New(isolate, (double) GL_VERTEX_ARRAY_BINDING_OES));
        tmpl->Set(ConvertToV8String(isolate, "createVertexArrayOES"),
                  v8::FunctionTemplate::New(isolate, &CreateVertexArrayOES));
        tmpl->Set(ConvertToV8String(isolate, "deleteVertexArrayOES"),
                  v8::FunctionTemplate::New(isolate, &DeleteVertexArrayOES));
        tmpl->Set(ConvertToV8String(isolate, "isVertexArrayOES"),
                  v8::FunctionTemplate::New(isolate, &IsVertexArrayOES));
        tmpl->Set(ConvertToV8String(isolate, "bindVertexArrayOES"),
                  v8::FunctionTemplate::New(isolate, &BindVertexArrayOES));


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
        SetNativeType(isolate, object, NativeType::OES_vertex_array_object);
        auto ext = v8::External::New(isolate, vertexArrayObject);
        object->SetInternalField(0, ext);
        return scope.Escape(object);
    }

    static OES_vertex_array_objectImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<OES_vertex_array_objectImpl *>(ptr);
    }

    static void CreateVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DeleteVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BindVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args);


    OES_vertex_array_object * GetVertexArrayObject() {
        return this->object_;
    }

private:
    OES_vertex_array_object* object_;
};
