//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "../../Common.h"
#include "../../Caches.h"
#include "../../Helpers.h"

class OES_vertex_array_objectImpl {
public:
    OES_vertex_array_objectImpl(rust::Box<OES_vertex_array_object> object);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, rust::Box<OES_vertex_array_object> object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void CreateVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DeleteVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BindVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    rust::Box<OES_vertex_array_object> object_;

    static OES_vertex_array_objectImpl *GetPointer(v8::Local<v8::Object> object);
};
