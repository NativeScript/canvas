//
// Created by Osei Fortune on 19/04/2022.
//

#include "TextDecoderImpl.h"
#include "Caches.h"
#include "Helpers.h"
#include "OneByteStringResource.h"

TextDecoderImpl::TextDecoderImpl(TextDecoder* decoder) : decoder_(decoder) {}

void TextDecoderImpl::Init(const v8::Local<v8::Object> &canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

   canvasModule->Set(context, ConvertToV8String(isolate, "TextDecoder"), func);
}

TextDecoderImpl *TextDecoderImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<TextDecoderImpl *>(ptr);
}

TextDecoder* TextDecoderImpl::GetTextDecoder() {
    return this->decoder_;
}

v8::Local<v8::FunctionTemplate> TextDecoderImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->TextDecoderTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, Ctor);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "TextDecoder"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);
    tmpl->SetAccessor(
            ConvertToV8String(isolate, "encoding"),
            Encoding);
    tmpl->Set(
            ConvertToV8String(isolate, "decode"),
            v8::FunctionTemplate::New(isolate, &Decode));
    
    tmpl->Set(
            ConvertToV8String(isolate, "decodeAsync"),
            v8::FunctionTemplate::New(isolate, &DecodeAsync));
    
    
    cache->TextDecoderTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void TextDecoderImpl::Ctor(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto count = args.Length();
    auto value = args[0];
    auto isolate = args.GetIsolate();
    if (count == 1 && !value->IsString()) {
        auto label = value->ToString(isolate->GetCurrentContext()).ToLocalChecked();
        auto arg = ConvertFromV8String(isolate, label);
        auto error = "Failed to construct 'TextDecoder': The encoding label provided (" +
                     arg + "') is invalid";
        isolate->ThrowError(ConvertToV8String(isolate, error));
        return;
    }

    std::string encoding("utf-8");
    if (count == 1) {
        encoding = ConvertFromV8String(isolate, value);
    }
    auto encoder = canvas_native_text_decoder_create(encoding.c_str());

    auto ret = args.This();

    auto decoder = new TextDecoderImpl(encoder);

    ret->SetAlignedPointerInInternalField(0, decoder);

    decoder->BindFinalizer(isolate, ret);

    args.GetReturnValue().Set(ret);

}

void
TextDecoderImpl::Encoding(v8::Local<v8::String> name,
                          const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto isolate = info.GetIsolate();
        auto encoding = canvas_native_text_decoder_get_encoding(ptr->GetTextDecoder());
        info.GetReturnValue().Set(ConvertToV8String(isolate, encoding));
        canvas_native_string_destroy((char*)encoding);
        return;
    }
    info.GetReturnValue().SetEmptyString();
}

void TextDecoderImpl::Decode(const v8::FunctionCallbackInfo<v8::Value> &args) {
    TextDecoderImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetEmptyString();
        return;
    }
    auto isolate = args.GetIsolate();

    if (args.Length() == 1) {
        auto value = args[0];
        if (value->IsNull() ||
            value->IsUndefined() ||
            !value->IsObject()) {

            args.GetIsolate()->ThrowError(
                    "Failed to execute 'decode' on 'TextDecoder': The provided value is not of type '(ArrayBuffer or ArrayBufferView)'"
            );
            return;
        }

        auto buf = value.As<v8::Object>();

        if (buf->IsArrayBuffer()) {

            auto buffer = buf.As<v8::ArrayBuffer>();

            auto data = static_cast<u_int8_t *>(buffer->GetBackingStore()->Data());
            auto size = buffer->ByteLength();
            auto decoded = canvas_native_text_decoder_decode(
                    ptr->GetTextDecoder(),
                    data, size);
            //args.GetReturnValue().Set(ConvertToV8String(isolate, decoded.c_str()));
            auto returnValue = new OneByteStringResource((char *)decoded);
            auto ret = v8::String::NewExternalOneByte(isolate, returnValue);
            args.GetReturnValue().Set(ret.ToLocalChecked());

            return;
        }


        if (buf->IsArrayBufferView()) {

            auto buffer = buf.As<v8::ArrayBufferView>();


            auto store = buffer->Buffer()->GetBackingStore();
            auto buffer_data = static_cast<uint8_t *>(store->Data()) + buffer->ByteOffset();

            auto len = buffer->ByteLength();

//            v8::Local<v8::Value> size;
//
//            buf->Get(context, ConvertToV8String(isolate, "BYTES_PER_ELEMENT")).ToLocal(&size);
//
//            size_t len = 0;
//
//            if(!size.IsEmpty()){
//                len =  buffer->Length() * (size_t)size->NumberValue(context).FromMaybe(0);
//            }

          //  auto len = buffer->ByteLength();

            auto decoded = canvas_native_text_decoder_decode_as_cow(
                    ptr->GetTextDecoder(),
                                                             buffer_data, len);


            // args.GetReturnValue().Set(ConvertToV8String(isolate, decoded.c_str()));
            auto returnValue = new OneByteStringResource(decoded);
            auto ret = v8::String::NewExternalOneByte(isolate, returnValue);
            args.GetReturnValue().Set(ret.ToLocalChecked());
            return;
        }


        args.GetIsolate()->ThrowError(
                "Failed to execute 'decode' on 'TextDecoder': The provided value is not of type '(ArrayBuffer or ArrayBufferView)'");


    }

    args.GetReturnValue().SetEmptyString();
}

struct DecodeAsyncData {
    v8::Persistent<v8::Object>* buffer;
    uint8_t* data;
    size_t size;
};

void TextDecoderImpl::DecodeAsync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();

    auto resolver = v8::Promise::Resolver::New(isolate->GetCurrentContext()).ToLocalChecked();
    args.GetReturnValue().Set(resolver->GetPromise());
    
    auto context = isolate->GetCurrentContext();
    
    auto value = args[0];
    if (value->IsNull() ||
        value->IsUndefined() ||
        !value->IsObject()) {
        
        auto msg = v8::Exception::Error(ConvertToV8String(isolate, "Failed to execute 'decode' on 'TextDecoder': The provided value is not of type '(ArrayBuffer or ArrayBufferView)'"));
        
        resolver->Reject(context, msg);

        return;
    }
    
    TextDecoderImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        resolver->Resolve(context, v8::String::Empty(isolate));
        return;
    }
    
    if(!value->IsArrayBufferView() && !value->IsArrayBuffer()){
        
        auto msg = v8::Exception::Error(ConvertToV8String(isolate, "Failed to execute 'decode' on 'TextDecoder': The provided value is not of type '(ArrayBuffer or ArrayBufferView)'"));
        
        resolver->Reject(context, msg);
        return;
    }


    auto callback = new PromiseCallback{
            isolate,
            resolver,
            [](bool done, void *data) {
                auto async_data = static_cast<PromiseCallback *>(data);
                auto func = async_data->inner_;
                if (func != nullptr && func->isolate_ != nullptr) {
                    v8::Isolate *isolate = func->isolate_;
                    v8::Locker locker(isolate);
                    v8::Isolate::Scope isolate_scope(
                            isolate);
                    v8::HandleScope handle_scope(
                            isolate);
                    v8::Local<v8::Promise::Resolver> callback = func->callback_.Get(
                            isolate);
                    v8::Local<v8::Context> context = callback->GetCreationContextChecked();
                    v8::Context::Scope context_scope(
                            context);

                    auto funcData = func->getData();

                    if (funcData == nullptr) {
                        callback->Resolve(context, v8::String::Empty(isolate));
                    } else {
                        auto decoded = static_cast<CCow *>(funcData);

                        auto returnValue = new OneByteStringResource(decoded);
                        auto ret = v8::String::NewExternalOneByte(isolate, returnValue);
                        v8::Local<v8::Value> value;
                        func->setData(nullptr);

                        
                        if (ret.ToLocal(&value)){
                            callback->Resolve(context, value).IsJust();
                        } else {
                            callback->Resolve(context, v8::String::Empty(isolate)).IsJust();
                        }
                    }
                }

                delete static_cast<PromiseCallback *>(data);
            }
    };
    callback->prepare();
    
    
    auto buf = value.As<v8::Object>();
    
    auto bufferPer = new v8::Persistent<v8::Object>(isolate, buf);
    
    uint8_t* data = nullptr;
    size_t size;
    
    if (buf->IsArrayBuffer()) {
        auto buffer = buf.As<v8::ArrayBuffer>();
        data = static_cast<u_int8_t *>(buffer->GetBackingStore()->Data());
        size = buffer->ByteLength();
    }


    if (buf->IsArrayBufferView()) {
        auto buffer = buf.As<v8::ArrayBufferView>();

        auto store = buffer->Buffer()->GetBackingStore();
        data = static_cast<uint8_t *>(store->Data()) + buffer->ByteOffset();

        size = buffer->ByteLength();
    }

    DecodeAsyncData asyncData{
        bufferPer,
        data,
        size
    };
    

    std::thread thread(
            [callback, asyncData, ptr]() {
                if (callback->inner_ != nullptr) {
                    
                    auto decoded = canvas_native_text_decoder_decode_as_cow(
                            ptr->GetTextDecoder(),
                                                                            asyncData.data, asyncData.size);
                    
                    callback->inner_->setData(decoded);
                    callback->inner_->execute(true, callback);
                }
                asyncData.buffer->Reset();
                delete asyncData.buffer;
                
            });
    thread.detach();

}
