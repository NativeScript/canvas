//
// Created by Osei Fortune on 29/04/2022.
//

#include "OES_vertex_array_objectImpl.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

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
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    OES_vertex_array_objectImpl *objectImpl = new OES_vertex_array_objectImpl(std::move(object));
    auto result = ctorFunc->InstanceTemplate()->NewInstance(isolate->GetCurrentContext()).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result, "OES_vertex_array_object");
    auto ext = v8::External::New(isolate, objectImpl);
    result->SetInternalField(0, ext);

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

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);

    tmpl->Set(Helpers::ConvertToV8String(isolate, "createVertexArrayOES"),
              v8::FunctionTemplate::New(isolate, &CreateVertexArrayOES));
    tmpl->Set(Helpers::ConvertToV8String(isolate, "deleteVertexArrayOES"),
              v8::FunctionTemplate::New(isolate, &DeleteVertexArrayOES));
    tmpl->Set(Helpers::ConvertToV8String(isolate, "isVertexArrayOES"),
              v8::FunctionTemplate::New(isolate, &IsVertexArrayOES));
    tmpl->Set(Helpers::ConvertToV8String(isolate, "BindVertexArrayOES"),
              v8::FunctionTemplate::New(isolate, &BindVertexArrayOES));

    cache->OES_vertex_array_objectImplTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void OES_vertex_array_objectImpl::CreateVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.Holder());
    args.GetReturnValue().Set(
            canvas_native_webgl_oes_vertex_array_object_create_vertex_array_oes(*ptr->object_)
    );
}

void OES_vertex_array_objectImpl::DeleteVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    auto array_object = args[0];
    canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(
            array_object->Uint32Value(context).ToChecked(),
            *ptr->object_
    );
}

void OES_vertex_array_objectImpl::IsVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    auto array_object = args[0];
    args.GetReturnValue().Set(
            canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(
                    array_object->Uint32Value(context).ToChecked(),
                    *ptr->object_
            )
    );
}

void OES_vertex_array_objectImpl::BindVertexArrayOES(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    auto array_object = args[0];
    canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(
            array_object->Uint32Value(context).ToChecked(),
            *ptr->object_
    );
}
