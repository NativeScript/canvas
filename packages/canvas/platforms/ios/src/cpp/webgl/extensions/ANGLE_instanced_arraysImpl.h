//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "Helpers.h"
#include <vector>
#include "ObjectWrapperImpl.h"

class ANGLE_instanced_arraysImpl : ObjectWrapperImpl {
public:
    ANGLE_instanced_arraysImpl(ANGLE_instanced_arrays *arrays);

    static v8::CFunction fast_draw_arrays_instanced_angle_;

    static v8::CFunction fast_draw_elements_instanced_angle_;

    static v8::CFunction fast_vertex_attrib_divisor_angle_;

    ANGLE_instanced_arrays *GetArrays();

    static ANGLE_instanced_arraysImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, ANGLE_instanced_arraysImpl *arrays) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = ANGLE_instanced_arraysImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::ANGLE_instanced_arrays);
        object->SetAlignedPointerInInternalField(0, arrays);
        arrays->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void DrawArraysInstancedANGLE(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastDrawArraysInstancedANGLE(v8::Local<v8::Object> receiver_obj, uint32_t mode, int32_t first,
                                 int32_t count, int32_t primcount) {
        ANGLE_instanced_arraysImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_angle_instanced_arrays_draw_arrays_instanced_angle(
                mode,
                first,
                count,
                primcount,
                ptr->GetArrays()
        );

    }

    static void DrawElementsInstancedANGLE(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastDrawElementsInstancedANGLE(v8::Local<v8::Object> receiver_obj, uint32_t mode, int32_t count,
                                   int32_t type, int32_t offset, int32_t primcount) {
        ANGLE_instanced_arraysImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_angle_instanced_arrays_draw_elements_instanced_angle(
                mode,
                count,
                type,
                offset,
                primcount,
                ptr->GetArrays()
        );

    }

    static void VertexAttribDivisorANGLE(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    FastVertexAttribDivisorANGLE(v8::Local<v8::Object> receiver_obj, uint32_t index,
                                 uint32_t divisor) {
        ANGLE_instanced_arraysImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_webgl_angle_instanced_arrays_vertex_attrib_divisor_angle(
                index,
                divisor,
                ptr->GetArrays()
        );

    }

private:
    ANGLE_instanced_arrays *arrays_;
};

