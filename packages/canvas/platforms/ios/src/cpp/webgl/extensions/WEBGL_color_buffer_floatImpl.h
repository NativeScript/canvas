//
// Created by Osei Fortune on 29/04/2022.
//

#define once

#include "gl.h"
#include <vector>
#include "Helpers.h"
#include "Common.h"
#include "Caches.h"
#include "ObjectWrapperImpl.h"
class WEBGL_color_buffer_floatImpl: ObjectWrapperImpl {
public:
    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WEBGL_color_buffer_floatTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WEBGL_color_buffer_float"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);
        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "WEBGL_color_buffer_float"));

        tmpl->Set(ConvertToV8String(isolate, "RGBA32F_EXT"),
                  v8::Integer::NewFromUnsigned(isolate, GL_RGBA32F_EXT));
        tmpl->Set(ConvertToV8String(isolate, "RGB32F_EXT"),
                  v8::Integer::NewFromUnsigned(isolate, GL_RGB32F_EXT));
        tmpl->Set(ConvertToV8String(isolate, "FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT"),
                  v8::Integer::NewFromUnsigned(isolate, GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT));
        tmpl->Set(ConvertToV8String(isolate, "UNSIGNED_NORMALIZED_EXT"),
                  v8::Integer::NewFromUnsigned(isolate, GL_UNSIGNED_NORMALIZED_EXT));
        cache->WEBGL_color_buffer_floatTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, WEBGL_color_buffer_floatImpl *buffer) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WEBGL_color_buffer_floatImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( object, NativeType::WEBGL_color_buffer_float);
        object->SetAlignedPointerInInternalField(0, buffer);
        buffer->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WEBGL_color_buffer_floatImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WEBGL_color_buffer_floatImpl *>(ptr);
    }
};
