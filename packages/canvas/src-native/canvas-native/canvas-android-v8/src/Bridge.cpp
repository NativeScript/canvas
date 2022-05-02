//
// Created by Osei Fortune on 22/03/2022.
//

#include "Bridge.h"
void Init(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    isolate->SetMicrotasksPolicy(v8::MicrotasksPolicy::kAuto);

    auto len = args.Length();

    if (len != 5) {
        auto errMsg = v8::String::NewFromUtf8(isolate, "Wrong number of arguments (expected 5)");
        auto err = v8::Exception::Error(errMsg.ToLocalChecked());
        isolate->ThrowException(err);
        return;
    }
    auto context = isolate->GetCurrentContext();



    std::string arrayScript = R"(
        (function () {
            // Providing Array Helpers since v8 crashing for now

            global.__Array_Get = function(array, i){
                if(!Array.isArray){return undefined};
                return array[i];
            }

            global.__Array_Set = function(array, i, value){
                if(!Array.isArray){return};
                array[i] = value;
            }
        })();
    )";

    auto source = v8::String::NewFromUtf8(isolate, arrayScript.c_str()).ToLocalChecked();
    Local<Script> script;
    bool success = Script::Compile(context, source).ToLocal(&script);
    if(success && !script.IsEmpty()){
        // todo assert;
        Local<Value> result;
        script->Run(context).ToLocal(&result);
    }


    ImageAssetImpl::Init(isolate);
    TextDecoderImpl::Init(isolate);
    TextEncoderImpl::Init(isolate);
    Canvas2D::Init(isolate);
    WebGL::Init(isolate);
    WebGL2::Init(isolate);
}
