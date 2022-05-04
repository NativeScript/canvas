//
// Created by Osei Fortune on 22/03/2022.
//

#include "Bridge.h"

void Init(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
   // isolate->SetMicrotasksPolicy(v8::MicrotasksPolicy::kAuto);

    auto len = args.Length();

    if (len != 5) {
        auto errMsg = v8::String::NewFromUtf8(isolate, "Wrong number of arguments (expected 5)");
        auto err = v8::Exception::Error(errMsg.ToLocalChecked());
        isolate->ThrowException(err);
        return;
    }

//    v8::TryCatch tryCatch(isolate);
//    v8::
//    auto context = isolate->GetCurrentContext();
//    std::string arrayScript = R"(
//        (function(){
//        })();
//    )";
//    console_log("1");
//    auto source = v8::String::NewFromUtf8(isolate, arrayScript.c_str()).ToLocalChecked();
//    console_log("2");
//    auto val = Helpers::ConvertFromV8String(isolate, source);
//    console_log(val);
//    v8::Local<v8::Script> script;
//    bool success = v8::Script::Compile(context, source).ToLocal(&script);
//    console_log("3");
//    if (success && !script.IsEmpty()) {
//        // todo assert;
//        v8::Local<v8::Value> result;
//        script->Run(context).ToLocal(&result);
//        console_log("4");
//    }else {
//        if(tryCatch.HasCaught()){
//            console_log("HasCaught");
//        }
//    }

    ImageAssetImpl::Init(isolate);
    TextDecoderImpl::Init(isolate);
    TextEncoderImpl::Init(isolate);
    Canvas2D::Init(isolate);
    WebGL::Init(isolate);
    WebGL2::Init(isolate);
}
