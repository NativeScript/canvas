//
// Created by Osei Fortune on 29/04/2022.
//

#pragma

#include "gl.h"
#include <vector>
#include "Helpers.h"
#include "Caches.h"
#include "Common.h"
#include "ObjectWrapperImpl.h"

class WEBGL_depth_textureImpl: ObjectWrapperImpl {
public:
    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WEBGL_depth_textureTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WEBGL_depth_texture"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);

        tmpl->Set(ConvertToV8String(isolate, "UNSIGNED_INT_24_8_WEBGL"), v8::Integer::NewFromUnsigned(isolate, 0x84FA));

        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "WEBGL_depth_texture"));

        cache->WEBGL_depth_textureTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, WEBGL_depth_textureImpl *depthTexture) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WEBGL_depth_textureImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( object, NativeType::WEBGL_depth_texture);
        object->SetAlignedPointerInInternalField(0, depthTexture);
        depthTexture->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WEBGL_depth_textureImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WEBGL_depth_textureImpl *>(ptr);
    }
};
