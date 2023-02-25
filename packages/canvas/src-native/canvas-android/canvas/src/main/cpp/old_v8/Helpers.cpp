//
// Created by Osei Fortune on 22/03/2022.
//

#include "Helpers.h"
#include "rust/cxx.h"
#include "canvas-android/src/lib.rs.h"

const char *Helpers::LOG_TAG = "JS";
int Helpers::m_maxLogcatObjectSize = 4096;

std::string Helpers::RGBAToHex(uint8_t r, uint8_t g, uint8_t b, uint8_t a) {
    std::ostringstream ss;

    if (a < 255) {
        ss << "rgba(";
        ss << static_cast<int>(r);
        ss << ",";
        ss << static_cast<int>(g);
        ss << ",";
        ss << static_cast<int>(b);
        ss << ",";
        ss << ((float) (a) / (float) 255);
        ss << ")";
    } else {
        ss << "#";
        ss << std::hex << std::setfill('0');
        ss << std::hex << std::setw(2) << static_cast<int>(r);
        ss << std::hex << std::setw(2) << static_cast<int>(g);
        ss << std::hex << std::setw(2) << static_cast<int>(b);
    }

    return ss.str();
}

void Helpers::sendToADBLogcat(const std::string &message, android_LogPriority logPriority) {
    // limit the size of the message that we send to logcat using the predefined value in package.json
    auto messageToLog = message;
    if (messageToLog.length() > m_maxLogcatObjectSize) {
        messageToLog = messageToLog.erase(m_maxLogcatObjectSize, std::string::npos);
        messageToLog = messageToLog + "...";
    }

    // split strings into chunks of 4000 characters
    // __android_log_write can't send more than 4000 to the stdout at a time
    auto messageLength = messageToLog.length();
    int maxStringLength = 4000;

    if (messageLength < maxStringLength) {
        __android_log_write(logPriority, Helpers::LOG_TAG, messageToLog.c_str());
    } else {
        for (int i = 0; i < messageLength; i += maxStringLength) {
            auto messagePart = messageToLog.substr(i, maxStringLength);

            __android_log_write(logPriority, Helpers::LOG_TAG, messagePart.c_str());
        }
    }
}

void Helpers::LogToConsole(const std::string &message) {
    sendToADBLogcat(message, android_LogPriority::ANDROID_LOG_INFO);
}


void Helpers::LogWarningToConsole(const std::string &message) {
    sendToADBLogcat(message, android_LogPriority::ANDROID_LOG_WARN);
}

void Helpers::ThrowIllegalConstructor(v8::Isolate *isolate) {
    auto msg = ConvertToV8String(isolate, "Illegal constructor");
    auto err = v8::Exception::TypeError(msg);
    isolate->ThrowException(err);
}

v8::Local<v8::String> Helpers::ConvertToV8String(v8::Isolate *isolate, const std::string &string) {
    return v8::String::NewFromUtf8(isolate, string.c_str(), v8::NewStringType::kNormal,
                                   string.length()).ToLocalChecked();
}


v8::Local<v8::String> Helpers::ConvertToV8String(v8::Isolate *isolate, const char *string) {
    return v8::String::NewFromUtf8(isolate, string).ToLocalChecked();
}

std::string Helpers::ConvertFromV8String(v8::Isolate *isolate, const v8::Local<v8::Value> &value) {
    if (value.IsEmpty()) {
        return {};
    }

    if (!value->IsString()) {
        return {};
    }

    if (value->IsStringObject()) {
        v8::Local<v8::String> obj = value.As<v8::StringObject>()->ValueOf();
        return ConvertFromV8String(isolate, obj);
    }

    v8::String::Utf8Value result(isolate, value);

    if (result.length() == 0) {
        return {};
    }

    const char *val = *result;

    if (val == nullptr) {
        return {};
    }

    std::ostringstream ss;
    ss << val;

    return ss.str();
}

rust::Str Helpers::ConvertFromV8StringToRust(v8::Isolate *isolate, const v8::Local<v8::Value> &value) {
    if (value.IsEmpty()) {
        return rust::Str(0);
    }

    if (!value->IsString()) {
        return rust::Str(0);
    }

    if (value->IsStringObject()) {
        v8::Local<v8::String> obj = value.As<v8::StringObject>()->ValueOf();
        return ConvertFromV8StringToRust(isolate, obj);
    }

    v8::String::Utf8Value result(isolate, value);

    const char *val = *result;

    if (val == nullptr) {
        return rust::Str(0);
    }

    return rust::Str(val, result.length());
}

bool Helpers::IsInstanceOf(v8::Isolate *isolate, const v8::Local<v8::Value> &value, const std::string &clazz) {
    auto context = isolate->GetCurrentContext();
    v8::TryCatch tryCatch(isolate);
    v8::Local<v8::Value> object;

    if (value.IsEmpty()) {
        return false;
    }

    if (context->Global()
            ->GetRealNamedProperty(context, Helpers::ConvertToV8String(isolate, clazz))
            .ToLocal(&object)) {

        if (object->IsObject() && value->InstanceOf(context, object.As<v8::Object>())
                .FromMaybe(false)) {
            return true;
        }
    }

    if (tryCatch.HasCaught()) tryCatch.Reset();
    return false;
}

void Helpers::SetPrivate(v8::Isolate *isolate, const v8::Local<v8::Object> &object, const std::string &property,
                         const v8::Local<v8::Value> &value) {
    auto context = isolate->GetCurrentContext();
    auto key = v8::Private::ForApi(isolate, Helpers::ConvertToV8String(isolate, property));
    object->SetPrivate(context, key, value);
}

v8::Local<v8::Value>
Helpers::GetPrivate(v8::Isolate *isolate, const v8::Local<v8::Object> &object, const std::string &property) {
    auto context = isolate->GetCurrentContext();
    auto key = v8::Private::ForApi(isolate, Helpers::ConvertToV8String(isolate, property));
    auto has = object->HasPrivate(context, key).FromMaybe(false);

    if (!has) {
        return v8::Undefined(isolate);
    }

    auto value = object->GetPrivate(context, key);

    if (value.IsEmpty()) {
        return v8::Undefined(isolate);
    } else {
        return value.ToLocalChecked();
    }
}


void Helpers::SetInstanceType(v8::Isolate *isolate, const v8::Local<v8::Object> &object, ObjectType type) {
    Helpers::SetPrivate(isolate, object, "__instanceType", v8::Uint32::New(isolate, (uint32_t) type));
}

ObjectType Helpers::GetInstanceType(v8::Isolate *isolate, const v8::Local<v8::Value> &object) {
    auto context = isolate->GetCurrentContext();

    if (object->IsNullOrUndefined()) {
        return ObjectType::Unknown;
    }

    if (object.IsEmpty()) {
        return ObjectType::Unknown;
    }

    auto ret = GetPrivate(isolate, object->ToObject(context).ToLocalChecked(), "__instanceType");

    if (!ret->IsNumber()) {
        return ObjectType::Unknown;
    }

    auto res = ret->Uint32Value(context);

    if (res.IsNothing()) {
        return ObjectType::Unknown;
    }

    auto val = (ObjectType) res.ToChecked();
    return val;
}
