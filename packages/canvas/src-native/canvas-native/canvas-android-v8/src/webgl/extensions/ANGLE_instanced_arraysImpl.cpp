//
// Created by Osei Fortune on 29/04/2022.
//

#include "ANGLE_instanced_arraysImpl.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

ANGLE_instanced_arraysImpl::ANGLE_instanced_arraysImpl(rust::Box<ANGLE_instanced_arrays> arrays) : arrays_(
        std::move(arrays)) {

}


ANGLE_instanced_arraysImpl *ANGLE_instanced_arraysImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<ANGLE_instanced_arraysImpl *>(ptr);
}

v8::Local<v8::Object>
ANGLE_instanced_arraysImpl::NewInstance(v8::Isolate *isolate, rust::Box<ANGLE_instanced_arrays> arrays) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    ANGLE_instanced_arraysImpl *arraysImpl = new ANGLE_instanced_arraysImpl(std::move(arrays));
    auto result = ctorFunc->InstanceTemplate()->NewInstance(isolate->GetCurrentContext()).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result, "ANGLE_instanced_arrays");
    auto ext = v8::External::New(isolate, arraysImpl);
    result->SetInternalField(0, ext);

    result->Set(context, Helpers::ConvertToV8String(isolate, "VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE"),
                v8::Uint32::New(isolate, GL_VERTEX_ATTRIB_ARRAY_DIVISOR_EXT));

    return handle_scope.Escape(result);
}

v8::Local<v8::FunctionTemplate> ANGLE_instanced_arraysImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->ANGLE_instanced_arraysImplTmpl.get();

    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "ANGLE_instanced_arrays"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);

    tmpl->Set(Helpers::ConvertToV8String(isolate, "drawArraysInstancedANGLE"),
              v8::FunctionTemplate::New(isolate, &DrawArraysInstancedANGLE));
    tmpl->Set(Helpers::ConvertToV8String(isolate, "drawElementsInstancedANGLE"),
              v8::FunctionTemplate::New(isolate, &DrawElementsInstancedANGLE));
    tmpl->Set(Helpers::ConvertToV8String(isolate, "vertexAttribDivisorANGLE"),
              v8::FunctionTemplate::New(isolate, &VertexAttribDivisorANGLE));

    cache->ANGLE_instanced_arraysImplTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void ANGLE_instanced_arraysImpl::DrawArraysInstancedANGLE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    auto mode = args[0];
    auto first = args[1];
    auto count = args[2];
    auto primcount = args[3];

    canvas_native_webgl_angle_instanced_arrays_draw_arrays_instanced_angle(
            mode->Uint32Value(context).ToChecked(),
            first->Int32Value(context).ToChecked(),
            count->Int32Value(context).ToChecked(),
            primcount->Int32Value(context).ToChecked(),
            *ptr->arrays_
    );
}


void ANGLE_instanced_arraysImpl::DrawElementsInstancedANGLE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());

    auto mode = args[0];
    auto count = args[1];
    auto type = args[2];
    auto offset = args[3];
    auto primcount = args[4];
    canvas_native_webgl_angle_instanced_arrays_draw_elements_instanced_angle(
            mode->Uint32Value(context).ToChecked(),
            count->Int32Value(context).ToChecked(),
            type->Uint32Value(context).ToChecked(),
            offset->Int32Value(context).ToChecked(),
            primcount->Int32Value(context).ToChecked(),
            *ptr->arrays_
    );
}


void ANGLE_instanced_arraysImpl::VertexAttribDivisorANGLE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());

    auto index = args[0];
    auto divisor = args[1];
    canvas_native_webgl_angle_instanced_arrays_vertex_attrib_divisor_angle(
            index->Uint32Value(context).ToChecked(),
            divisor->Uint32Value(context).ToChecked(),
            *ptr->arrays_
    );
}
