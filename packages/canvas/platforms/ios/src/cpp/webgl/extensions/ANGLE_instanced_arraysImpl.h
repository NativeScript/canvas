//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "Helpers.h"
#include <vector>
#include "ObjectWrapperImpl.h"

class ANGLE_instanced_arraysImpl: ObjectWrapperImpl {
public:
    ANGLE_instanced_arraysImpl(ANGLE_instanced_arrays *arrays);

    ANGLE_instanced_arrays *GetArrays();

    static ANGLE_instanced_arraysImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, ANGLE_instanced_arraysImpl *arrays) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = ANGLE_instanced_arraysImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( object, NativeType::ANGLE_instanced_arrays);
        object->SetAlignedPointerInInternalField(0, arrays);
        arrays->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void DrawArraysInstancedANGLE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawElementsInstancedANGLE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void VertexAttribDivisorANGLE(const v8::FunctionCallbackInfo<v8::Value> &args);

private:
    ANGLE_instanced_arrays *arrays_;
};

