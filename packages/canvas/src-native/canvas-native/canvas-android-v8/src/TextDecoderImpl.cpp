//
// Created by Osei Fortune on 19/04/2022.
//

#include "TextDecoderImpl.h"

TextDecoderImpl::TextDecoderImpl(rust::Box<TextDecoder> decoder) : decoder_(std::move(decoder)) {

}

void TextDecoderImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "TextDecoder"), ctor->GetFunction(context).ToLocalChecked());
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
                                    std::string(Helpers::ConvertFromV8String(isolate,args[0]->ToString(context).ToLocalChecked())) + "') is invalid"
                    )
            );
            isolate->ThrowException(err);
            return;
        }
        std::string decoding("utf-8");
        if (args.Length() == 1 && args[0]->IsString()) {
            decoding = std::string(Helpers::ConvertFromV8String(isolate, args[0]->ToString(context).ToLocalChecked()));
        }

        auto ret = args.This();
        Helpers::SetInternalClassName(isolate, ret, "TextDecoder");
        auto decoder = canvas_native_text_decoder_create(decoding);
        TextDecoderImpl *impl = new TextDecoderImpl(std::move(decoder));
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

TextDecoderImpl *TextDecoderImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<TextDecoderImpl *>(ptr);
}

void TextDecoderImpl::GetEncoding(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto self = info.Holder();
    auto ptr = GetPointer(self);
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
    auto context = isolate->GetCurrentContext();

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, &Create);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "TextDecoder"));


    auto tmpl = ctorTmpl->InstanceTemplate();

    tmpl->SetInternalFieldCount(1);
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
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 1) {
        if (!args[0]->IsArrayBuffer() || !args[0]->IsArrayBufferView()) {

            auto err = v8::Exception::TypeError(
                    Helpers::ConvertToV8String(
                            isolate,
                            "Failed to execute 'decode' on 'TextDecoder': The provided value is not of type '(ArrayBuffer or ArrayBufferView)'"
                    )
            );
            isolate->ThrowException(err);
            return;
        }
        if (args[0]->IsArrayBuffer()) {
            auto buf = args[0].As<v8::ArrayBuffer>();
            auto store = buf->GetBackingStore();
            auto data = static_cast<std::uint8_t *>(store->Data());
            auto size = store->ByteLength();
            rust::Slice<const uint8_t> slice{data, size};
            auto decoded = canvas_native_text_decoder_decode(*ptr->decoder_, slice);
            args.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, decoded.c_str()));
        } else if (args[0]->IsArrayBufferView()) {
            auto buf = args[0].As<v8::ArrayBufferView>();

            auto offset = buf->ByteOffset();
            auto store = buf->Buffer()->GetBackingStore();

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