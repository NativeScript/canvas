//
// Created by Osei Fortune on 19/04/2022.
//

#include "TextEncoderImpl.h"
#include "TextEncoderImplEntry.h"
#include "rust/cxx.h"
#include "canvas-android/src/lib.rs.h"

TextEncoderImpl::TextEncoderImpl(rust::Box<TextEncoder> encoder) : encoder_(std::move(encoder)) {

}

void TextEncoderImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "TextEncoder"),
                ctor->GetFunction(context).ToLocalChecked());
}

void TextEncoderImpl::Create(const v8::FunctionCallbackInfo <v8::Value> &args) {
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
                            std::string(Helpers::ConvertFromV8String(isolate,
                                                                     args[0]->ToString(
                                                                             context).ToLocalChecked())) +
                            "') is invalid"
                    )
            );
            isolate->ThrowException(err);
            return;
        }
        std::string encoding("utf-8");
        if (args.Length() == 1 && Helpers::IsString(args[0])) {
            encoding = Helpers::GetString(isolate, args[0]);
        }

        auto ret = args.This();
        Helpers::SetInstanceType(isolate, ret, ObjectType::TextEncoder);
        auto encoder = canvas_native_text_encoder_create(
                rust::Str(encoding.c_str(), encoding.size()));
        TextEncoderImpl *impl = new TextEncoderImpl(std::move(encoder));
        AddWeakListener(isolate, ret, impl);
        args.GetReturnValue().Set(ret);
    }
}

TextEncoderImpl *TextEncoderImpl::GetPointer(const v8::Local <v8::Object> &object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<TextEncoderImpl *>(ptr);
}

void TextEncoderImpl::GetEncoding(v8::Local <v8::String> name,
                                  const v8::PropertyCallbackInfo <v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.This());
    auto encoding = canvas_native_text_encoder_get_encoding(*ptr->encoder_);
    info.GetReturnValue().Set(
            Helpers::ConvertToV8String(isolate, encoding.c_str())
    );
}

v8::Local <v8::FunctionTemplate> TextEncoderImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->TextEncoderTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local <v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, &Create);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "TextEncoder"));

    auto tmpl = ctorTmpl->PrototypeTemplate();

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "encode"),
            v8::FunctionTemplate::New(isolate, &Encode)
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "encoding"),
            &GetEncoding
    );

    cache->TextEncoderTmpl =
            std::make_unique < v8::Persistent < v8::FunctionTemplate >> (isolate, ctorTmpl);
    return ctorTmpl;
}

void TextEncoderImpl::Encode(const v8::FunctionCallbackInfo <v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 1) {
        auto val = args[0]->ToString(context).ToLocalChecked();
        v8::String::Utf8Value utf8(isolate, val);
        std::string text(*utf8, utf8.length());

        auto data = canvas_native_text_encoder_encode(*ptr->encoder_,
                                                      rust::Str(text.c_str(), text.size()));
        isolate->AdjustAmountOfExternalAllocatedMemory(data.size());

        TextEncoderImplEntry *entry = new TextEncoderImplEntry(std::move(data));

        auto len = entry->GetSize();
        auto store_data = entry->GetData();

        auto callback = [](const v8::WeakCallbackInfo <TextEncoderImplEntry> &data) {
            auto isolate = data.GetIsolate();
            auto value = data.GetParameter();
            auto cache = Caches::Get(isolate);
            auto entry = cache->TextEncoderData->Get(value);
            auto size = value->GetSize();
            if (entry.get() != nullptr) {
                entry->ClearWeak();
                entry->Reset();
            }
            cache->TextEncoderData->Remove(value);
            delete value;

            if (size != 0) {
                isolate->AdjustAmountOfExternalAllocatedMemory(-size);
            }
        };

        auto buffer = v8::ArrayBuffer::New(isolate, store_data, len);
        auto array = v8::Uint8ClampedArray::New(buffer, 0, len);

        auto buf = std::make_shared < v8::Persistent < v8::Object >> (isolate, array);
        buf->SetWeak(entry, callback, v8::WeakCallbackType::kFinalizer);
        auto cache = Caches::Get(isolate);
        cache->TextEncoderData->Insert(entry, buf);
        args.GetReturnValue().Set(array);
        return;
    }
    args.GetReturnValue().SetUndefined();
}
