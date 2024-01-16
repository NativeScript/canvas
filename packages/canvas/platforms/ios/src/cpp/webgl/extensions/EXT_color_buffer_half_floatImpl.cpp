//
// Created by Osei Fortune on 28/04/2022.
//

#include "EXT_color_buffer_half_floatImpl.h"
#include "Caches.h"
#include "Helpers.h"

v8::Local<v8::FunctionTemplate> EXT_color_buffer_half_floatImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->EXT_color_buffer_half_floatTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "EXT_color_buffer_half_float"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->Set(ConvertToV8String(isolate, "RGBA16F_EXT"),
              v8::Integer::NewFromUnsigned(isolate, 0x881A));

    tmpl->Set(ConvertToV8String(isolate, "RGB16F_EXT"),
              v8::Integer::NewFromUnsigned(isolate, 0x881B));

    tmpl->Set(ConvertToV8String(isolate, "FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT"),
              v8::Integer::NewFromUnsigned(isolate, 0x8211));

    tmpl->Set(ConvertToV8String(isolate, "UNSIGNED_NORMALIZED_EXT"),
              v8::Integer::NewFromUnsigned(isolate, 0x8C17));

    tmpl->Set(ConvertToV8String(isolate, "ext_name"),
              ConvertToV8String(isolate, "EXT_color_buffer_half_float"));

    cache->EXT_color_buffer_half_floatTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}
