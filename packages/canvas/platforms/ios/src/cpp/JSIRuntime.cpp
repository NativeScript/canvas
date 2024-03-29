//
// Created by Osei Fortune on 14/12/2022.
//

#include "CanvasJSIModule.h"
#include "JSIRuntime.h"


extern "C" void NSMain(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto len = args.Length();

    if (len != 5) {
        auto errMsg = v8::String::NewFromUtf8(isolate, "Wrong number of arguments (expected 5)");
        auto err = v8::Exception::Error(errMsg.ToLocalChecked());
        isolate->ThrowException(err);
        return;
    }

    CanvasJSIModule::install(isolate);
}
