//
// Created by Osei Fortune on 22/03/2022.
//

#include "Helpers.h"
#include "rust/cxx.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

const char *Helpers::LOG_TAG = "JS";
int Helpers::m_maxLogcatObjectSize = 4096;

std::string Helpers::RGBAToHex(uint8_t r, uint8_t g, uint8_t b, uint8_t a) {

    std::stringstream ss;

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

void Helpers::ThrowIllegalConstructor(v8::Isolate *isolate) {
    auto msg = ConvertToV8String(isolate, "Illegal constructor");
    auto err = v8::Exception::TypeError(msg);
    isolate->ThrowException(err);
}

v8::Local<v8::String> Helpers::ConvertToV8String(v8::Isolate *isolate, const std::string &string) {
    return v8::String::NewFromUtf8(isolate, string.c_str(), v8::NewStringType::kNormal,
                                   string.length()).ToLocalChecked();
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

    const char *val = *result;

    if (val == nullptr) {
        return {};
    }

    return std::move(std::string(val, result.length()));
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

bool
Helpers::IsInstanceOf(v8::Isolate *isolate, const v8::Local<v8::Value> &value, const v8::Local<v8::Object> &instance) {
    return (NSCIsObjectInstanceOfFunc_(isolate, local_to_ptr(instance), local_to_ptr(value)));
}

bool Helpers::IsInstanceOf(v8::Isolate *isolate, const v8::Local<v8::Value> &value, const std::string &clazz) {
    auto context = isolate->GetCurrentContext();
    v8::TryCatch tryCatch(isolate);
    v8::Local<v8::Value> object;

    if (value.IsEmpty()) {
        return false;
    }

    Helpers::LogToConsole("IsInstanceOf inner");

    if (context->Global()
            ->GetRealNamedProperty(context, Helpers::ConvertToV8String(isolate, clazz))
            .ToLocal(&object)) {

        Helpers::LogToConsole(Helpers::ConvertFromV8String(isolate, value.As<v8::Object>()->GetConstructorName()));

        Helpers::LogToConsole(Helpers::ConvertFromV8String(isolate, object.As<v8::Object>()->GetConstructorName()));

        if (object->IsObject() && value->InstanceOf(context, object.As<v8::Object>())
                .FromMaybe(false)) {
            return true;
        }

//        if (!object.IsEmpty() && Helpers::IsObject(object) &&
//            (NSCIsObjectInstanceOfFunc_(isolate, local_to_ptr(object), local_to_ptr(value)))
//                ) {
//            return true;
//        }
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
    auto value = object->GetPrivate(context, key);
    if (value.IsEmpty()) {
        return v8::Undefined(isolate);
    } else {
        return value.ToLocalChecked();
    }
}


Helpers::NSCGetObjectValue Helpers::NSCGetObjectValueFunc_ = nullptr;
Helpers::NSCSetObjectValue Helpers::NSCSetObjectValueFunc_ = nullptr;
Helpers::NSCCallFunction Helpers::NSCCallFunctionFunc_ = nullptr;
Helpers::NSCScriptCompileAndRun Helpers::NSCScriptCompileAndRunFunc_ = nullptr;
Helpers::NSCGetObjectValueWithIndex Helpers::NSCGetObjectValueWithIndexFunc_ = nullptr;
Helpers::NSCSetObjectValueWithIndex Helpers::NSCSetObjectValueWithIndexFunc_ = nullptr;
Helpers::NSCIsObjectInstanceOf Helpers::NSCIsObjectInstanceOfFunc_ = nullptr;
Helpers::NSCCreateArrayBuffer Helpers::NSCCreateArrayBufferFunc_ = nullptr;

void InitRuntimeUtils() {
    if (Helpers::NSCGetObjectValueFunc_ == nullptr &&
        Helpers::NSCSetObjectValueFunc_ == nullptr &&
        Helpers::NSCGetObjectValueWithIndexFunc_ == nullptr &&
        Helpers::NSCSetObjectValueWithIndexFunc_ == nullptr &&
        Helpers::NSCCallFunctionFunc_ == nullptr &&
        Helpers::NSCScriptCompileAndRunFunc_ == nullptr &&
        Helpers::NSCIsObjectInstanceOfFunc_ == nullptr &&
        Helpers::NSCCreateArrayBufferFunc_ == nullptr) {

        void *lib = dlopen("libNativeScript.so", RTLD_NOW | RTLD_LOCAL);
        if (lib != nullptr) {
            Helpers::NSCGetObjectValueFunc_ = reinterpret_cast<Helpers::NSCGetObjectValue>(
                    dlsym(lib, "NSCGetObjectValue"));
            Helpers::NSCSetObjectValueFunc_ = reinterpret_cast<Helpers::NSCSetObjectValue>(
                    dlsym(lib, "NSCSetObjectValue"));
            Helpers::NSCGetObjectValueWithIndexFunc_ = reinterpret_cast<Helpers::NSCGetObjectValueWithIndex>(
                    dlsym(lib, "NSCGetObjectValueWithIndex"));
            Helpers::NSCSetObjectValueWithIndexFunc_ = reinterpret_cast<Helpers::NSCSetObjectValueWithIndex>(
                    dlsym(lib, "NSCSetObjectValueWithIndex"));
            Helpers::NSCCallFunctionFunc_ = reinterpret_cast<Helpers::NSCCallFunction>(
                    dlsym(lib, "NSCCallFunction"));
            Helpers::NSCScriptCompileAndRunFunc_ = reinterpret_cast<Helpers::NSCScriptCompileAndRun>(
                    dlsym(lib, "NSCScriptCompileAndRun"));

            Helpers::NSCIsObjectInstanceOfFunc_ = reinterpret_cast<Helpers::NSCIsObjectInstanceOf>(
                    dlsym(lib, "NSCIsObjectInstanceOf"));

            Helpers::NSCCreateArrayBufferFunc_ = reinterpret_cast<Helpers::NSCCreateArrayBuffer>(
                    dlsym(lib, "NSCCreateArrayBuffer"));

            assert(Helpers::NSCGetObjectValueFunc_);
            assert(Helpers::NSCSetObjectValueFunc_);
            assert(Helpers::NSCGetObjectValueWithIndexFunc_);
            assert(Helpers::NSCSetObjectValueWithIndexFunc_);
            assert(Helpers::NSCCallFunctionFunc_);
            assert(Helpers::NSCScriptCompileAndRunFunc_);
            assert(Helpers::NSCIsObjectInstanceOfFunc_);
            assert(Helpers::NSCCreateArrayBufferFunc_);

        }
    }
}

v8::Local<v8::Value>
Helpers::ObjectGet(const v8::Local<v8::Context> &context, const v8::Local<v8::Object> &object, uint32_t key) {
    InitRuntimeUtils();
    auto ptr = reinterpret_cast<v8::Value *>(NSCGetObjectValueWithIndexFunc_(local_to_ptr(object),
                                                                             local_to_ptr(context), key));
    auto ret = ptr_to_maybe_local(ptr);
    if (ret.IsEmpty()) {
        return v8::Undefined(context->GetIsolate());
    }
    return ret.ToLocalChecked();
}

v8::Local<v8::Value>
Helpers::ObjectGet(const v8::Local<v8::Context> &context, const v8::Local<v8::Object> &object,
                   const v8::Local<v8::Value> &key) {
    InitRuntimeUtils();
    auto ptr = reinterpret_cast<v8::Value *>(NSCGetObjectValueFunc_(local_to_ptr(object), local_to_ptr(context),
                                                                    local_to_ptr(key)));
    auto ret = ptr_to_maybe_local(ptr);
    if (ret.IsEmpty()) {
        return v8::Undefined(context->GetIsolate());
    }
    return ret.ToLocalChecked();
}


void
Helpers::ObjectSet(const v8::Local<v8::Context> &context, const v8::Local<v8::Object> &object, uint32_t key,
                   const v8::Local<v8::Value> &value) {
    InitRuntimeUtils();
    NSCSetObjectValueWithIndexFunc_(local_to_ptr(object), local_to_ptr(context), key, local_to_ptr(value));
}

void
Helpers::ObjectSet(const v8::Local<v8::Context> &context, const v8::Local<v8::Object> &object,
                   const v8::Local<v8::Value> &key,
                   const v8::Local<v8::Value> &value) {
    InitRuntimeUtils();
    NSCSetObjectValueFunc_(local_to_ptr(object), local_to_ptr(context), local_to_ptr(key), local_to_ptr(value));
}

v8::Local<v8::Value> Helpers::FunctionCall(v8::Local<v8::Function> func,
                                           v8::Local<v8::Context> context,
                                           v8::Local<v8::Value> recv,
                                           int argc,
                                           v8::Local<v8::Value> argv[]) {
    InitRuntimeUtils();
    auto ptr = reinterpret_cast<v8::Value *>(NSCCallFunctionFunc_(
            local_to_ptr(func),
            local_to_ptr(context),
            local_to_ptr(recv),
            argc,
            local_args_to_ptr(argv)
    ));
    return ptr_to_local(ptr);
}

v8::Local<v8::ArrayBuffer>
Helpers::CreateArrayBuffer(v8::Isolate *isolate, void *data, size_t size, v8::BackingStore::DeleterCallback callback,
                           void *deleter_data) {
    InitRuntimeUtils();
    auto ptr = reinterpret_cast<v8::ArrayBuffer *>(NSCCreateArrayBufferFunc_(isolate, data, size, callback,
                                                                             deleter_data));
    return ptr_to_local(ptr);
}


v8::Local<v8::Script> Helpers::ScriptCompileAndRun(v8::Isolate *isolate, const std::string &src) {
    InitRuntimeUtils();
    auto ptr = reinterpret_cast<v8::Script *>(NSCScriptCompileAndRunFunc_(
            src.c_str(), isolate
    ));
    return ptr_to_local(ptr);
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
    auto ret = Helpers::GetPrivate(isolate, object.As<v8::Object>(), "__instanceType");
    if (!ret->IsNumber()) {
        return ObjectType::Unknown;
    }

    auto res = ret->Uint32Value(context);
    if (res.IsNothing()) {
        return ObjectType::Unknown;
    }
    return (ObjectType) res.ToChecked();
}
