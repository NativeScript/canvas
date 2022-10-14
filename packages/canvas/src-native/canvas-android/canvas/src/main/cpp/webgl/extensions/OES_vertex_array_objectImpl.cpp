//
// Created by Osei Fortune on 29/04/2022.
//

#include "OES_vertex_array_objectImpl.h"


OES_vertex_array_objectImpl::OES_vertex_array_objectImpl(rust::Box<OES_vertex_array_object> object) : object_(
        std::move(object)) {

}

OES_vertex_array_objectImpl *OES_vertex_array_objectImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<OES_vertex_array_objectImpl *>(ptr);
}

v8::Local<v8::Object>
OES_vertex_array_objectImpl::NewInstance(v8::Isolate *isolate, rust::Box<OES_vertex_array_object> object) {
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    OES_vertex_array_objectImpl *objectImpl = new OES_vertex_array_objectImpl(std::move(object));
    auto result = ctorFunc->InstanceTemplate()->NewInstance(isolate->GetCurrentContext()).ToLocalChecked();
    Helpers::SetInstanceType(isolate, result, ObjectType::OES_vertex_array_object);

    AddWeakListener(isolate, result, objectImpl);

    result->Set(context, Helpers::ConvertToV8String(isolate, "VERTEX_ARRAY_BINDING_OES"),
                v8::Uint32::New(isolate, GL_VERTEX_ARRAY_BINDING_OES));

    return handle_scope.Escape(result);
}

v8::Local<v8::FunctionTemplate> OES_vertex_array_objectImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->OES_vertex_array_objectImplTmpl.get();

    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "OES_vertex_array_object"));

    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);

    auto tmpl = ctorTmpl->PrototypeTemplate();

    tmpl->Set(Helpers::ConvertToV8String(isolate, "createVertexArrayOES"),
              v8::FunctionTemplate::New(isolate, &CreateVertexArrayOES));
    tmpl->Set(Helpers::ConvertToV8String(isolate, "deleteVertexArrayOES"),
              v8::FunctionTemplate::New(isolate, &DeleteVertexArrayOES));
    tmpl->Set(Helpers::ConvertToV8String(isolate, "isVertexArrayOES"),
              v8::FunctionTemplate::New(isolate, &IsVertexArrayOES));
    tmpl->Set(Helpers::ConvertToV8String(isolate, "bindVertexArrayOES"),
              v8::FunctionTemplate::New(isolate, &BindVertexArrayOES));

    cache->OES_vertex_array_objectImplTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void OES_vertex_array_objectImpl::CreateVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.This());
    args.GetReturnValue().Set(
            WebGLVertexArrayObject::NewInstance(isolate,
                                                canvas_native_webgl_oes_vertex_array_object_create_vertex_array_oes(
                                                        *ptr->object_))
    );
}

void OES_vertex_array_objectImpl::DeleteVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto array_object = args[0];
    if (Helpers::GetInstanceType(isolate, array_object) == ObjectType::WebGLVertexArrayObject) {
        auto vertexArrayValue = Helpers::GetPrivate(isolate, array_object.As<v8::Object>(), "instance")->ToUint32(
                context);
        canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(
                vertexArrayValue.ToLocalChecked()->Uint32Value(context).ToChecked(),
                *ptr->object_
        );
    }
}

void OES_vertex_array_objectImpl::IsVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto array_object = args[0];
    if (Helpers::GetInstanceType(isolate, array_object) == ObjectType::WebGLVertexArrayObject) {
        auto vertexArrayValue = Helpers::GetPrivate(isolate, array_object.As<v8::Object>(), "instance")->ToUint32(
                context);
        args.GetReturnValue().Set(
                canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(
                        vertexArrayValue.ToLocalChecked()->Uint32Value(context).ToChecked(),
                        *ptr->object_
                )
        );
    }
}

void OES_vertex_array_objectImpl::BindVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto array_object = args[0];
    if (Helpers::GetInstanceType(isolate, array_object) == ObjectType::WebGLVertexArrayObject) {
        auto vertexArrayValue = Helpers::GetPrivate(isolate, array_object.As<v8::Object>(), "instance")->ToUint32(
                context);
        canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(
                vertexArrayValue.ToLocalChecked()->Uint32Value(context).ToChecked(),
                *ptr->object_
        );
    }
}
