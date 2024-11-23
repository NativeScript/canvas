//
// Created by Osei Fortune on 25/02/2023.
//

#pragma once

#include <memory>
#include "Common.h"
#include "OneByteStringResource.h"
#include "v8-fast-api-calls.h"
#include "AsyncCallback.h"
#include "PromiseCallback.h"
//#ifdef __APPLE__
//#ifdef __OBJC__
//#include <Foundation/Foundation.h>
//#else
//#include <CoreFoundation/CoreFoundation.h>
//extern "C" void NSLog(CFStringRef format, ...);
//#endif
//#endif

static const char *LOG_TAG = "JS";
static int m_maxLogcatObjectSize = 4096;

#ifdef __ANDROID__

static void sendToADBLogcat(const std::string &message, android_LogPriority logPriority) {
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
        __android_log_write(logPriority, LOG_TAG, messageToLog.c_str());
    } else {
        for (int i = 0; i < messageLength; i += maxStringLength) {
            auto messagePart = messageToLog.substr(i, maxStringLength);

            __android_log_write(logPriority, LOG_TAG, messagePart.c_str());
        }
    }
}

#endif

//#ifdef __APPLE__
//#ifndef __OBJC__
//#define Log(fmt, ...) NSLog(CFSTR(fmt), ##__VA_ARGS__)
//#else
//#define Log(...) NSLog(__VA_ARGS__)
//#endif
//#endif



static void LogToConsole(const std::string &message) {
#ifdef __ANDROID__
    sendToADBLogcat(message, android_LogPriority::ANDROID_LOG_INFO);
#endif

#ifdef __APPLE__
    // Log("%s", message.c_str());
#endif
}

inline static v8::Local<v8::String>
ConvertToV8OneByteString(v8::Isolate *isolate, std::string string) {
    auto value = new OneByteStringResource(std::move(string));
    auto ret = v8::String::NewExternalOneByte(isolate, value);
    return ret.ToLocalChecked();
}


inline static v8::Local<v8::String>
ConvertToV8OneByteString(v8::Isolate *isolate, char *string) {
    auto value = new OneByteStringResource(string);
    auto ret = v8::String::NewExternalOneByte(isolate, value);
    return ret.ToLocalChecked();
}

inline static v8::Local<v8::String>
ConvertToV8String(v8::Isolate *isolate, const std::string &string) {
    return v8::String::NewFromUtf8(isolate, string.c_str()).ToLocalChecked();
}

inline static std::string
ConvertFromV8String(v8::Isolate *isolate, const v8::Local<v8::Value> &value) {
    if (value.IsEmpty()) {
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

    return {*result};
}

inline static std::string_view
ConvertFromV8StringView(v8::Isolate *isolate, const v8::Local<v8::Value> &value) {
    if (value.IsEmpty()) {
        return {};
    }

    if (value->IsStringObject()) {
        v8::Local<v8::String> obj = value.As<v8::StringObject>()->ValueOf();
        return ConvertFromV8StringView(isolate, obj);
    }

    v8::String::Utf8Value result(isolate, value);

    const char *val = *result;

    if (val == nullptr) {
        return {};
    }

    return {*result};
}

inline static std::string_view
ConvertFromV8StringViewValue(v8::Isolate *isolate, v8::Local<v8::Value> value) {
    if (value.IsEmpty()) {
        return {};
    }

    if (value->IsStringObject()) {
        v8::Local<v8::String> obj = value.As<v8::StringObject>()->ValueOf();
        return ConvertFromV8StringViewValue(isolate, obj);
    }

    v8::String::Utf8Value result(isolate, value);

    const char *val = *result;

    if (val == nullptr) {
        return {};
    }

    return {*result};
}


static void SetFastMethod(v8::Isolate *isolate,
                          v8::Local<v8::Template> that,
                          const char *name,
                          v8::FunctionCallback slow_callback,
                          const v8::CFunction *c_function,
                          v8::Local<v8::Value> data) {
    v8::Local<v8::FunctionTemplate> t =
            v8::FunctionTemplate::New(isolate,
                                      slow_callback,
                                      data,
                                      v8::Local<v8::Signature>(),
                                      0,
                                      v8::ConstructorBehavior::kThrow,
                                      v8::SideEffectType::kHasSideEffect,
                                      c_function);
    // kInternalized strings are created in the old space.
    const v8::NewStringType type = v8::NewStringType::kInternalized;
    v8::Local<v8::String> name_string =
            v8::String::NewFromUtf8(isolate, name, type).ToLocalChecked();
    that->Set(name_string, t);
}


#define NUM(a) (sizeof(a) / sizeof(*a))

static void SetFastMethodWithOverLoads(v8::Isolate *isolate,
                                       v8::Local<v8::Template> that,
                                       const char *name,
                                       v8::FunctionCallback slow_callback,
                                       const v8::CFunction *method_overloads,
                                       v8::Local<v8::Value> data) {

    auto len = NUM(&method_overloads);
    v8::Local<v8::FunctionTemplate> t =
            v8::FunctionTemplate::NewWithCFunctionOverloads(isolate,
                                                            slow_callback,
                                                            data,
                                                            v8::Local<v8::Signature>(),
                                                            0,
                                                            v8::ConstructorBehavior::kThrow,
                                                            v8::SideEffectType::kHasSideEffect,
                                                            {method_overloads, len});
    // kInternalized strings are created in the old space.
    const v8::NewStringType type = v8::NewStringType::kInternalized;
    v8::Local<v8::String> name_string =
            v8::String::NewFromUtf8(isolate, name, type).ToLocalChecked();
    that->Set(name_string, t);
}


static void SetFastMethod(v8::Local<v8::Context> context,
                          v8::Local<v8::Object> that,
                          const char *name,
                          v8::FunctionCallback slow_callback,
                          const v8::CFunction *c_function,
                          v8::Local<v8::Value> data = v8::Local<v8::Value>()) {
    v8::Isolate *isolate = context->GetIsolate();
    v8::Local<v8::Function> function =
            v8::FunctionTemplate::New(isolate,
                                      slow_callback,
                                      data,
                                      v8::Local<v8::Signature>(),
                                      0,
                                      v8::ConstructorBehavior::kThrow,
                                      v8::SideEffectType::kHasSideEffect,
                                      c_function)
                    ->GetFunction(context)
                    .ToLocalChecked();
    const v8::NewStringType type = v8::NewStringType::kInternalized;
    v8::Local<v8::String> name_string =
            v8::String::NewFromUtf8(isolate, name, type).ToLocalChecked();
    that->Set(context, name_string, function).Check();
}

static void SetFastMethodNoSideEffect(v8::Local<v8::Context> context,
                                      v8::Local<v8::Object> that,
                                      const char *name,
                                      v8::FunctionCallback slow_callback,
                                      const v8::CFunction *c_function,
                                      v8::Local<v8::Value> data) {
    v8::Isolate *isolate = context->GetIsolate();
    v8::Local<v8::Function> function =
            v8::FunctionTemplate::New(isolate,
                                      slow_callback,
                                      data,
                                      v8::Local<v8::Signature>(),
                                      0,
                                      v8::ConstructorBehavior::kThrow,
                                      v8::SideEffectType::kHasNoSideEffect,
                                      c_function)
                    ->GetFunction(context)
                    .ToLocalChecked();
    const v8::NewStringType type = v8::NewStringType::kInternalized;
    v8::Local<v8::String> name_string =
            v8::String::NewFromUtf8(isolate, name, type).ToLocalChecked();
    that->Set(context, name_string, function).Check();
}

static void SetFastMethodNoSideEffect(v8::Isolate *isolate,
                                      v8::Local<v8::Template> that,
                                      const char *name,
                                      v8::FunctionCallback slow_callback,
                                      const v8::CFunction *c_function,
                                      v8::Local<v8::Value> data) {
    v8::Local<v8::FunctionTemplate> t =
            v8::FunctionTemplate::New(isolate,
                                      slow_callback,
                                      data,
                                      v8::Local<v8::Signature>(),
                                      0,
                                      v8::ConstructorBehavior::kThrow,
                                      v8::SideEffectType::kHasNoSideEffect,
                                      c_function);
    // kInternalized strings are created in the old space.
    const v8::NewStringType type = v8::NewStringType::kInternalized;
    v8::Local<v8::String> name_string =
            v8::String::NewFromUtf8(isolate, name, type).ToLocalChecked();
    that->Set(name_string, t);
}
