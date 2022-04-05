//
// Created by Osei Fortune on 22/03/2022.
//

#include "Helpers.h"

void Helpers::ThrowIllegalConstructor(v8::Isolate *isolate) {
    auto msg = ConvertToV8String(isolate, "Illegal constructor");
    auto err = v8::Exception::Error(msg);
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
