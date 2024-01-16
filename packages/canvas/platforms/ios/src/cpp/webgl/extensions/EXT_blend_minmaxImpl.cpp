//
// Created by Osei Fortune on 28/04/2022.
//

#include "EXT_blend_minmaxImpl.h"
#include "Caches.h"
#include "Helpers.h"

v8::Local<v8::FunctionTemplate> EXT_blend_minmaxImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->EXT_blend_minmaxTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "EXT_blend_minmax"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->Set(ConvertToV8String(isolate, "MIN_EXT"),
              v8::Integer::NewFromUnsigned(isolate, 0x8007));

    tmpl->Set(ConvertToV8String(isolate, "MAX_EXT"),
              v8::Integer::NewFromUnsigned(isolate, 0x8008));

    tmpl->Set(ConvertToV8String(isolate, "ext_name"),
              ConvertToV8String(isolate, "EXT_blend_minmax"));

    cache->EXT_blend_minmaxTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}
