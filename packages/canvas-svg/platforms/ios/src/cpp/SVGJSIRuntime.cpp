// NSMain: NativeScript's entry point for system_lib:// native modules on Android.
//
// Android only, and the guard matters: on Apple the module is installed through
// `CanvasSVGModule.install()` (see helpers.ts) and this entry point is never called. But every
// plugin's bridge defines `NSMain`, so compiling it here collides at link time with the one in
// canvas's own bridge the moment both plugins are in the same app.
#include "SVGJSIModule.h"

#ifdef __ANDROID__

extern "C" void NSMain(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto len = args.Length();

    if (len != 5) {
        auto errMsg = v8::String::NewFromUtf8(isolate, "Wrong number of arguments (expected 5)");
        auto err = v8::Exception::Error(errMsg.ToLocalChecked());
        isolate->ThrowException(err);
        return;
    }

    SVGJSIModule::install(isolate);
}

#endif // __ANDROID__
