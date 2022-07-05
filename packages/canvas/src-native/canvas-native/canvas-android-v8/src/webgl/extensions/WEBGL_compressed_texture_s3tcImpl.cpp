//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_compressed_texture_s3tcImpl.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

v8::Local<v8::FunctionTemplate> WEBGL_compressed_texture_s3tcImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->WEBGL_compressed_texture_s3tcImplTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "WEBGL_compressed_texture_s3tc"));
    cache->WEBGL_compressed_texture_s3tcImplTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate,
                                                                                                          ctorTmpl);
    return ctorTmpl;
}

v8::Local<v8::Object> WEBGL_compressed_texture_s3tcImpl::NewInstance(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ctorFunc = GetCtor(isolate);
    auto result = ctorFunc->InstanceTemplate()->NewInstance(context).ToLocalChecked();
    Helpers::SetInstanceType(isolate, result, ObjectType::WEBGL_compressed_texture_s3tc);


    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_RGB_S3TC_DXT1_EXT"),
                v8::Int32::New(isolate, GL_COMPRESSED_RGB_S3TC_DXT1_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_RGBA_S3TC_DXT1_EXT"),
                v8::Int32::New(isolate, GL_COMPRESSED_RGBA_S3TC_DXT1_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_RGBA_S3TC_DXT3_EXT"),
                v8::Int32::New(isolate, GL_COMPRESSED_RGBA_S3TC_DXT3_EXT));
    result->Set(context, Helpers::ConvertToV8String(isolate, "COMPRESSED_RGBA_S3TC_DXT5_EXT"),
                v8::Int32::New(isolate, GL_COMPRESSED_RGBA_S3TC_DXT5_EXT));

    return handle_scope.Escape(result);
}