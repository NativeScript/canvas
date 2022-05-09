//
// Created by Osei Fortune on 19/04/2022.
//

#include "TextEncoderImpl.h"
#include "TextEncoderImplEntry.h"
#include "rust/cxx.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

TextEncoderImpl::TextEncoderImpl(rust::Box<TextEncoder> encoder) : encoder_(std::move(encoder)) {

}

void TextEncoderImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "TextEncoder"), ctor->GetFunction(context).ToLocalChecked());
}

void TextEncoderImpl::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();

    if (!args.IsConstructCall()) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(
                        isolate,
                        "Please use the 'new' operator, this object constructor cannot be called as a function."
                )
        );
        isolate->ThrowException(err);
        return;
    } else {
        if (args.Length() == 1 && !args[0]->IsString()) {
            auto err = v8::Exception::RangeError(
                    Helpers::ConvertToV8String(
                            isolate,
                            "Failed to construct 'TextEncoder': The encoding label provided ('" +
                            std::string(Helpers::ConvertFromV8String(isolate,args[0]->ToString(context).ToLocalChecked())) + "') is invalid"
                    )
            );
            isolate->ThrowException(err);
            return;
        }
        std::string encoding("utf-8");
        if (args.Length() == 1 && args[0]->IsString()) {
            encoding = std::string(Helpers::ConvertFromV8String(isolate, args[0]->ToString(context).ToLocalChecked()));
        }

        auto ret = args.This();
        Helpers::SetInternalClassName(isolate, ret, "TextEncoder");
        auto encoder = canvas_native_text_encoder_create(encoding);
        TextEncoderImpl *impl = new TextEncoderImpl(std::move(encoder));
        auto ext = v8::External::New(isolate, impl);

        if (ret->InternalFieldCount() > 0) {
            ret->SetInternalField(0, ext);
        } else {
            ret->SetPrivate(context, v8::Private::New(isolate, Helpers::ConvertToV8String(isolate, "rustPtr")),
                            ext);
        }

        args.GetReturnValue().Set(ret);
    }
}

TextEncoderImpl *TextEncoderImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<TextEncoderImpl *>(ptr);
}

void TextEncoderImpl::GetEncoding(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto self = info.Holder();
    auto ptr = GetPointer(self);
    std::string encoding;
    canvas_native_text_encoder_get_encoding(*ptr->encoder_, encoding);
    info.GetReturnValue().Set(
            Helpers::ConvertToV8String(isolate, encoding.c_str())
    );
}

v8::Local<v8::FunctionTemplate> TextEncoderImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->TextEncoderTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, &Create);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "TextEncoder"));

    auto tmpl = ctorTmpl->InstanceTemplate();

    tmpl->SetInternalFieldCount(1);
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "encode"), v8::FunctionTemplate::New(isolate, &Encode)
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "encoding"),
            &GetEncoding
    );

    cache->TextEncoderTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void TextEncoderImpl::Encode(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 1) {
        auto val = args[0]->ToString(context).ToLocalChecked();
        v8::String::Utf8Value utf8(isolate, val);
        std::string text(*utf8, utf8.length());

        auto data = canvas_native_text_encoder_encode(*ptr->encoder_, text);
        TextEncoderImplEntry *entry = new TextEncoderImplEntry(std::move(data));

        auto len = entry->GetSize();
        auto store_data = entry->GetData();

        auto callback = [](const v8::WeakCallbackInfo<TextEncoderImplEntry> &data) {
            auto value = data.GetParameter();
            auto cache = Caches::Get(data.GetIsolate());
            auto entry = cache->TextEncoderData->Get(value);
            if (entry.get() != nullptr) {
                entry->ClearWeak();
                entry->Reset();
            }
            cache->TextEncoderData->Remove(value);
            delete value;
        };

        auto buffer = v8::ArrayBuffer::New(isolate, store_data, len);
        auto array = v8::Uint8ClampedArray::New(buffer, 0, len);

        auto buf = std::make_shared<v8::Persistent<v8::Object>>(isolate, array);
        buf->SetWeak(entry, callback, v8::WeakCallbackType::kFinalizer);
        auto cache = Caches::Get(isolate);
        cache->TextEncoderData->Insert(entry,buf);
        args.GetReturnValue().Set(array);
        return;
    }
    args.GetReturnValue().Set(v8::Undefined(isolate));
}
