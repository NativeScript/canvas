//
// Created by Osei Fortune on 19/04/2022.
//

#include "TextDecoderImpl.h"
#include "rust/cxx.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

TextDecoderImpl::TextDecoderImpl(rust::Box<TextDecoder> decoder) : decoder_(std::move(decoder)) {

}

void TextDecoderImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "TextDecoder"),
                ctor->GetFunction(context).ToLocalChecked());
}

void TextDecoderImpl::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
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
                            "Failed to construct 'TextDecoder': The encoding label provided ('" +
                            std::string(Helpers::ConvertFromV8String(isolate,
                                                                     args[0]->ToString(context).ToLocalChecked())) +
                            "') is invalid"
                    )
            );
            isolate->ThrowException(err);
            return;
        }
        std::string decoding("utf-8");
        if (args.Length() == 1 && Helpers::IsString(args[0])) {
            decoding = Helpers::GetString(isolate, args[0]);
        }

        auto ret = args.This();
        Helpers::SetInstanceType(isolate, ret, ObjectType::TextDecoder);
        auto decoder = canvas_native_text_decoder_create(rust::Str(decoding.c_str(), decoding.size()));
        TextDecoderImpl *impl = new TextDecoderImpl(std::move(decoder));
        AddWeakListener(isolate, ret, impl);
        args.GetReturnValue().Set(ret);
    }
}

TextDecoderImpl *TextDecoderImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<TextDecoderImpl *>(ptr);
}

void TextDecoderImpl::GetEncoding(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.This());
    auto encoding = canvas_native_text_decoder_get_encoding(*ptr->decoder_);
    info.GetReturnValue().Set(
            Helpers::ConvertToV8String(isolate, encoding.c_str())
    );
}


v8::Local<v8::FunctionTemplate> TextDecoderImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->TextDecoderTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, &Create);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "TextDecoder"));

    auto tmpl = ctorTmpl->PrototypeTemplate();

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "decode"), v8::FunctionTemplate::New(isolate, &Decode)
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "encoding"),
            &GetEncoding
    );

    cache->TextDecoderTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}


void TextDecoderImpl::Decode(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 1) {
        auto buf = args[0];
        if (!buf->IsArrayBuffer() && !buf->IsArrayBufferView() && !buf->IsTypedArray()) {

            auto err = v8::Exception::TypeError(
                    Helpers::ConvertToV8String(
                            isolate,
                            "Failed to execute 'decode' on 'TextDecoder': The provided value is not of type '(ArrayBuffer or ArrayBufferView)'"
                    )
            );
            isolate->ThrowException(err);
            return;
        }
        if (buf->IsArrayBuffer()) {
            auto buffer = buf.As<v8::ArrayBuffer>();
            auto store = buffer->GetBackingStore();
            auto data = static_cast<std::uint8_t *>(store->Data());
            auto size = store->ByteLength();
            rust::Slice<const uint8_t> slice{data, size};
            auto decoded = canvas_native_text_decoder_decode(*ptr->decoder_, slice);
            args.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, decoded.c_str()));
        } else if (buf->IsArrayBufferView() || buf->IsTypedArray()) {
            auto buffer = buf.As<v8::TypedArray>();
            auto offset = buffer->ByteOffset();
            auto store = buffer->Buffer()->GetBackingStore();
            auto data = static_cast<std::uint8_t *>(store->Data()) + offset;
            auto size = store->ByteLength();
            rust::Slice<const uint8_t> slice{data, size};
            auto decoded = canvas_native_text_decoder_decode(*ptr->decoder_, slice);
            args.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, decoded.c_str()));
        }

        return;
    }
    args.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, ""));
}