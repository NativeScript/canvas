//
// Created by Osei Fortune on 22/03/2022.
//

#include "Helpers.h"

void Helpers::ThrowIllegalConstructor(v8::Isolate *isolate) {
    auto msg = ConvertToV8String(isolate, "Illegal constructor");
    auto err = v8::Exception::TypeError(msg);
    isolate->ThrowException(err);
}

v8::Local<v8::String> Helpers::ConvertToV8String(v8::Isolate *isolate, const std::string &string) {
    return v8::String::NewFromUtf8(isolate, string.c_str()).ToLocalChecked();
}


rust::String Helpers::ConvertFromV8String(v8::Isolate *isolate, const v8::Local<v8::String> &value) {
    if (value.IsEmpty()) {
        return std::string();
    }
    v8::HandleScope handle_scope(isolate);
    auto len = value->Utf8Length(isolate);
    char buf[len + 1];
    auto wrote = value->WriteUtf8(isolate, buf);
    auto ret = to_rust_string(rust::Slice<const char>(buf, len + 1));
    return ret;
}

bool Helpers::IsInstanceOf(v8::Isolate *isolate, v8::Local<v8::Object> value, std::string clazz) {
    auto context = isolate->GetCurrentContext();
    auto key = v8::Private::New(isolate,
                                Helpers::ConvertToV8String(isolate,
                                                           "class_name"));
    auto instance = value->GetPrivate(context, key).FromMaybe(
            Helpers::ConvertToV8String(isolate, "").As<v8::Value>()
    );

    return std::strcmp(clazz.c_str(),
                       Helpers::ConvertFromV8String(isolate, instance->ToString(context).ToLocalChecked()).c_str()) ==
           0;
}

void Helpers::SetInternalClassName(v8::Isolate *isolate, v8::Local<v8::Object> value, std::string clazz) {
    auto context = isolate->GetCurrentContext();
    value->SetPrivate(context, v8::Private::New(isolate, Helpers::ConvertToV8String(isolate, "class_name")),
                      Helpers::ConvertToV8String(isolate, clazz));
}
