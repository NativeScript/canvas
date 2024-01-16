//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "gl.h"
#include <vector>
#include "Common.h"
#include "Caches.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class WEBGL_draw_buffersImpl: ObjectWrapperImpl {
public:
    WEBGL_draw_buffersImpl(WEBGL_draw_buffers *buffers);

    ~WEBGL_draw_buffersImpl() {
        if(this->buffers_ != nullptr){
            canvas_native_webgl_WEBGL_draw_buffers_destroy(this->buffers_);
            this->buffers_ = nullptr;
        }
    }

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WEBGL_draw_buffersTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WEBGL_draw_buffers"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);
        tmpl->Set(ConvertToV8String(isolate, "drawBuffersWEBGL"),
                  v8::FunctionTemplate::New(isolate, &DrawBuffersWEBGL));


        tmpl->Set(ConvertToV8String(isolate, "COLOR_ATTACHMENT0_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COLOR_ATTACHMENT0_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COLOR_ATTACHMENT1_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COLOR_ATTACHMENT1_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COLOR_ATTACHMENT2_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COLOR_ATTACHMENT2_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COLOR_ATTACHMENT3_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COLOR_ATTACHMENT3_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COLOR_ATTACHMENT4_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COLOR_ATTACHMENT4_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COLOR_ATTACHMENT5_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COLOR_ATTACHMENT5_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COLOR_ATTACHMENT6_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COLOR_ATTACHMENT6_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COLOR_ATTACHMENT7_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COLOR_ATTACHMENT7_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COLOR_ATTACHMENT8_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COLOR_ATTACHMENT8_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COLOR_ATTACHMENT9_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COLOR_ATTACHMENT9_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COLOR_ATTACHMENT10_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COLOR_ATTACHMENT10_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COLOR_ATTACHMENT11_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COLOR_ATTACHMENT11_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COLOR_ATTACHMENT12_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COLOR_ATTACHMENT12_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COLOR_ATTACHMENT13_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COLOR_ATTACHMENT13_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COLOR_ATTACHMENT14_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COLOR_ATTACHMENT14_EXT));
        tmpl->Set(ConvertToV8String(isolate, "COLOR_ATTACHMENT15_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_COLOR_ATTACHMENT15_EXT));
        tmpl->Set(ConvertToV8String(isolate, "DRAW_BUFFER0_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_DRAW_BUFFER0_EXT));
        tmpl->Set(ConvertToV8String(isolate, "DRAW_BUFFER1_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_DRAW_BUFFER1_EXT));
        tmpl->Set(ConvertToV8String(isolate, "DRAW_BUFFER2_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_DRAW_BUFFER2_EXT));
        tmpl->Set(ConvertToV8String(isolate, "DRAW_BUFFER3_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_DRAW_BUFFER3_EXT));
        tmpl->Set(ConvertToV8String(isolate, "DRAW_BUFFER4_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_DRAW_BUFFER4_EXT));
        tmpl->Set(ConvertToV8String(isolate, "DRAW_BUFFER5_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_DRAW_BUFFER5_EXT));
        tmpl->Set(ConvertToV8String(isolate, "DRAW_BUFFER6_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_DRAW_BUFFER6_EXT));
        tmpl->Set(ConvertToV8String(isolate, "DRAW_BUFFER7_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_DRAW_BUFFER7_EXT));
        tmpl->Set(ConvertToV8String(isolate, "DRAW_BUFFER8_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_DRAW_BUFFER8_EXT));
        tmpl->Set(ConvertToV8String(isolate, "DRAW_BUFFER9_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_DRAW_BUFFER9_EXT));
        tmpl->Set(ConvertToV8String(isolate, "DRAW_BUFFER10_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_DRAW_BUFFER10_EXT));
        tmpl->Set(ConvertToV8String(isolate, "DRAW_BUFFER11_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_DRAW_BUFFER11_EXT));
        tmpl->Set(ConvertToV8String(isolate, "DRAW_BUFFER12_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_DRAW_BUFFER12_EXT));
        tmpl->Set(ConvertToV8String(isolate, "DRAW_BUFFER13_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_DRAW_BUFFER13_EXT));
        tmpl->Set(ConvertToV8String(isolate, "DRAW_BUFFER14_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_DRAW_BUFFER14_EXT));
        tmpl->Set(ConvertToV8String(isolate, "DRAW_BUFFER15_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_DRAW_BUFFER15_EXT));

        tmpl->Set(ConvertToV8String(isolate, "MAX_COLOR_ATTACHMENTS_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_MAX_COLOR_ATTACHMENTS_EXT));
        tmpl->Set(ConvertToV8String(isolate, "MAX_DRAW_BUFFERS_WEBGL"),
                  v8::Integer::NewFromUnsigned(isolate, GL_MAX_DRAW_BUFFERS_EXT));

        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "WEBGL_draw_buffers"));

        cache->WEBGL_draw_buffersTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, WEBGL_draw_buffersImpl *buffers) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WEBGL_draw_buffersImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( object, NativeType::WEBGL_draw_buffers);
        object->SetAlignedPointerInInternalField(0, buffers);
        buffers->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WEBGL_draw_buffersImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WEBGL_draw_buffersImpl *>(ptr);
    }

    static void DrawBuffersWEBGL(const v8::FunctionCallbackInfo<v8::Value> &args);

    WEBGL_draw_buffers *GetDrawBuffers() {
        return this->buffers_;
    }

private:
    WEBGL_draw_buffers *buffers_;
};
