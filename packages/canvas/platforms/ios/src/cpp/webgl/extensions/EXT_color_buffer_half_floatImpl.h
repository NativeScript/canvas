//
// Created by Osei Fortune on 28/04/2022.
//

#pragma once

#include "Common.h"
#include <vector>
#include "Helpers.h"
#include "ObjectWrapperImpl.h"
class EXT_color_buffer_half_floatImpl: ObjectWrapperImpl {
public:
    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, EXT_color_buffer_half_floatImpl *buffer) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = EXT_color_buffer_half_floatImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( buffer, NativeType::EXT_color_buffer_half_float);
        object->SetAlignedPointerInInternalField(0, buffer);
        object->Set(context, ConvertToV8String(isolate, "ext_name"),
                    ConvertToV8String(isolate, "EXT_color_buffer_half_float"));
        buffer->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static EXT_color_buffer_half_floatImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<EXT_color_buffer_half_floatImpl *>(ptr);
    }
};

