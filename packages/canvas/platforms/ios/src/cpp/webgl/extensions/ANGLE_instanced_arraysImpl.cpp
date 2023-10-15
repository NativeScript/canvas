//
// Created by Osei Fortune on 29/04/2022.
//

#include "ANGLE_instanced_arraysImpl.h"
#include "Caches.h"

ANGLE_instanced_arraysImpl::ANGLE_instanced_arraysImpl(ANGLE_instanced_arrays * arrays)
        : arrays_(arrays) {}

ANGLE_instanced_arraysImpl *
ANGLE_instanced_arraysImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<ANGLE_instanced_arraysImpl *>(ptr);
}

void ANGLE_instanced_arraysImpl::DrawArraysInstancedANGLE(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    ANGLE_instanced_arraysImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto mode = (uint32_t) args[0]->NumberValue(context).ToChecked();
    auto first = (int32_t) args[1]->NumberValue(context).ToChecked();
    auto count_ = (int32_t) args[2]->NumberValue(context).ToChecked();
    auto primcount = (int32_t) args[3]->NumberValue(context).ToChecked();

    canvas_native_webgl_angle_instanced_arrays_draw_arrays_instanced_angle(
            mode,
            first,
            count_,
            primcount,
            ptr->GetArrays()
    );
}

void ANGLE_instanced_arraysImpl::DrawElementsInstancedANGLE(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    ANGLE_instanced_arraysImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto mode = (uint32_t) args[0]->NumberValue(context).ToChecked();
    auto count_ = (int32_t) args[1]->NumberValue(context).ToChecked();
    auto type = (uint32_t) args[2]->NumberValue(context).ToChecked();
    auto offset = (int32_t) args[3]->NumberValue(context).ToChecked();
    auto primcount = (int32_t) args[4]->NumberValue(context).ToChecked();
    canvas_native_webgl_angle_instanced_arrays_draw_elements_instanced_angle(
            mode,
            count_,
            type,
            offset,
            primcount,
            ptr->GetArrays()
    );
}

void ANGLE_instanced_arraysImpl::VertexAttribDivisorANGLE(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    ANGLE_instanced_arraysImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto index = (u_int32_t) args[0]->NumberValue(context).ToChecked();
    auto divisor = (u_int32_t) args[1]->NumberValue(context).ToChecked();
    canvas_native_webgl_angle_instanced_arrays_vertex_attrib_divisor_angle(
            index,
            divisor,
            ptr->GetArrays()
    );
}


v8::Local<v8::FunctionTemplate> ANGLE_instanced_arraysImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->ANGLE_instanced_arraysTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "ANGLE_instanced_arrays"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);

    tmpl->Set(ConvertToV8String(isolate, "VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE"),
              v8::Number::New(isolate, 0x88FE));

    tmpl->Set(ConvertToV8String(isolate, "ext_name"),
              ConvertToV8String(isolate, "ANGLE_instanced_arrays"));

    tmpl->Set(
            ConvertToV8String(isolate, "drawArraysInstancedANGLE"),
            v8::FunctionTemplate::New(isolate, &DrawArraysInstancedANGLE));

    tmpl->Set(
            ConvertToV8String(isolate, "drawElementsInstancedANGLE"),
            v8::FunctionTemplate::New(isolate, &DrawElementsInstancedANGLE));

    tmpl->Set(
            ConvertToV8String(isolate, "vertexAttribDivisorANGLE"),
            v8::FunctionTemplate::New(isolate, &VertexAttribDivisorANGLE));


    cache->ANGLE_instanced_arraysTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

ANGLE_instanced_arrays* ANGLE_instanced_arraysImpl::GetArrays() {
    return this->arrays_;
}
