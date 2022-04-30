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

std::string Helpers::ConvertFromV8StringToString(v8::Isolate *isolate, const v8::Local<v8::String> &value) {
    if (value.IsEmpty()) {
        return std::string();
    }
    v8::HandleScope handle_scope(isolate);
    auto len = value->Utf8Length(isolate);
    char buf[len + 1];
    auto wrote = value->WriteUtf8(isolate, buf);
    std::string buffer;
    write_to_string(rust::Slice<const char>(buf, len + 1), buffer);
    return buffer;
}


bool Helpers::IsInstanceOf(v8::Isolate *isolate, v8::Local<v8::Value> value, std::string clazz) {
    auto context = isolate->GetCurrentContext();

    if (value.IsEmpty()) {
        return false;
    }

    if (value->IsNullOrUndefined()) {
        return false;
    }

    if (!value->IsObject()) {
        return false;
    }

//    auto key = v8::Private::New(isolate,
//                                Helpers::ConvertToV8String(isolate,
//                                                           "class_name"));
//    auto instance = value->GetPrivate(context, key);
//    if(instance.IsEmpty()){
//        return false;
//    }
//
//    auto to_cmp = Helpers::ConvertFromV8String(isolate, instance.ToLocalChecked()->ToString(context).ToLocalChecked());
//    return std::strcmp(clazz.c_str(), to_cmp.c_str()) == 0;

    v8::TryCatch tryCatch(isolate);
    v8::Local<v8::Value> object;

    if (context->Global()
            ->GetRealNamedProperty(context, Helpers::ConvertToV8String(isolate, clazz))
            .ToLocal(&object)) {

        if (object->IsFunction()) {
            auto name = object.As<v8::Function>()->GetName();
            if (value->IsFunction()) {
                auto value_name = value.As<v8::Function>()->GetName();
                if (std::strcmp(
                        Helpers::ConvertFromV8String(isolate, name.As<v8::String>()).c_str(),
                        Helpers::ConvertFromV8String(isolate, value_name.As<v8::String>()).c_str()
                ) !=
                    0) {
                    return false;
                }
            }

            if (name->IsString()) {
                if (std::strcmp(Helpers::ConvertFromV8String(isolate, name.As<v8::String>()).c_str(), clazz.c_str()) ==
                    0) {
                    return true;
                }
            }
        }
        if (object->IsObject() &&
            value->ToObject(context).ToLocalChecked()->InstanceOf(context, object.As<v8::Object>())
                    .FromMaybe(false)) {
            return true;
        }
    }

    if (tryCatch.HasCaught()) tryCatch.Reset();
    return false;
}

void Helpers::SetInternalClassName(v8::Isolate *isolate, v8::Local<v8::Object> value, std::string clazz) {
    auto context = isolate->GetCurrentContext();
    value->SetPrivate(context, v8::Private::New(isolate, Helpers::ConvertToV8String(isolate, "class_name")),
                      Helpers::ConvertToV8String(isolate, clazz));
}

void Helpers::SetPrivate(v8::Isolate *isolate, v8::Local<v8::Object> object, std::string property,
                         v8::Local<v8::Value> value) {
    auto context = isolate->GetCurrentContext();
    auto key = v8::Private::ForApi(isolate, Helpers::ConvertToV8String(isolate, property));
    object->SetPrivate(context, key, value);
}

v8::Local<v8::Value> Helpers::GetPrivate(v8::Isolate *isolate, v8::Local<v8::Object> object, std::string property) {
    auto context = isolate->GetCurrentContext();
    auto key = v8::Private::ForApi(isolate, Helpers::ConvertToV8String(isolate, property));
    auto value = object->GetPrivate(context, key);
    if (value.IsEmpty()) {
        return v8::Undefined(isolate);
    } else {
        return value.ToLocalChecked();
    }
}
