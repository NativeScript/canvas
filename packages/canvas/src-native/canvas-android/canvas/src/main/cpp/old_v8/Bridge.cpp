//
// Created by Osei Fortune on 22/03/2022.
//

#include "Bridge.h"
#include "OneByteStringResource.h"
#include "Configuration.h"
#include "rust/cxx.h"

void OneByteStringResourceInstance(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto string = args[0];

    if (Helpers::IsString(string)) {
        auto str = string.As<v8::String>();

        if (str->IsExternalOneByte()) {
            auto val = str->GetExternalOneByteStringResource();
            rust::String ret(val->data(), val->length());
            auto extString = new OneByteStringResource(std::move(
                    string_to_buf(ret)
            ));

            args.GetReturnValue().Set(
                    v8::String::NewExternalOneByte(isolate, extString).ToLocalChecked()
            );
        } else {
            auto val = Helpers::ConvertFromV8String(isolate, str);
            auto extString = new OneByteStringResource(
                    std::move(string_to_buf(rust::String(val.data(), val.length()))));

            args.GetReturnValue().Set(
                    v8::String::NewExternalOneByte(isolate, extString).ToLocalChecked()
            );
        }

        return;
    }

    auto extString = new OneByteStringResource(std::move(rust::Vec<uint8_t>()));

    args.GetReturnValue().Set(
            v8::String::NewExternalOneByte(isolate, extString).ToLocalChecked()
    );
}

void SetConfiguration(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto config = args[0];

    if (!config->IsNullOrUndefined() && config->IsObject()) {
        auto obj = config.As<v8::Object>();
        v8::Local<v8::Value> appBaseVal;
        if (obj->Get(context, Helpers::ConvertToV8String(isolate, "appBase")).ToLocal(
                &appBaseVal)) {
            if (appBaseVal->IsString()) {
                Configuration::SetAppBase(Helpers::ConvertFromV8String(isolate, appBaseVal));
            }
        }
    }
}

void Init(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto len = args.Length();

    if (len != 5) {
        auto errMsg = v8::String::NewFromUtf8(isolate, "Wrong number of arguments (expected 5)");
        auto err = v8::Exception::Error(errMsg.ToLocalChecked());
        isolate->ThrowException(err);
        return;
    }

    auto ctx = isolate->GetCurrentContext();

    auto exports = args[1].As<v8::Object>();

    auto propName = v8::String::NewFromUtf8(isolate, "dummy").ToLocalChecked();

    auto cache = Caches::Get(isolate);
    cache->SetContext(ctx);
    ImageAssetImpl::Init(isolate);
    ImageBitmapImpl::Init(isolate);
    TextDecoderImpl::Init(isolate);
    TextEncoderImpl::Init(isolate);
    Canvas2D::Init(isolate);
    WebGL::Init(isolate);
    WebGL2::Init(isolate);

    auto result = exports->Set(ctx, propName, v8::Null(isolate));

    auto global = ctx->Global();
    auto func = v8::FunctionTemplate::New(isolate, &OneByteStringResourceInstance);
    global->Set(ctx, Helpers::ConvertToV8String(isolate, "__getOneByteStringResource"),
                func->GetFunction(ctx).ToLocalChecked());

    auto configFunc = v8::FunctionTemplate::New(isolate, &SetConfiguration);
    global->Set(ctx, Helpers::ConvertToV8String(isolate, "__initAppConfiguration"),
                configFunc->GetFunction(ctx).ToLocalChecked());

}


extern "C" void NSMain(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Init(args);
}
