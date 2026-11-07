#pragma once

#include "gl.h"
#include "Helpers.h"
#include "Common.h"
#include "Caches.h"
#include "ObjectWrapperImpl.h"

class KHR_parallel_shader_compileImpl: ObjectWrapperImpl {
public:
    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->KHR_parallel_shader_compileTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "KHR_parallel_shader_compile"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);
        tmpl->Set(ConvertToV8String(isolate, "COMPLETION_STATUS_KHR"),
                  v8::Integer::NewFromUnsigned(isolate, 0x91B1));
        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "KHR_parallel_shader_compile"));

        cache->KHR_parallel_shader_compileTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, KHR_parallel_shader_compileImpl *parallelShaderCompile) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = KHR_parallel_shader_compileImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(parallelShaderCompile, NativeType::KHR_parallel_shader_compile);
        object->SetAlignedPointerInInternalField(0, parallelShaderCompile, ObjectWrapperImpl::kInternalFieldTag);
        parallelShaderCompile->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static KHR_parallel_shader_compileImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0, ObjectWrapperImpl::kInternalFieldTag);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<KHR_parallel_shader_compileImpl *>(ptr);
    }
};
