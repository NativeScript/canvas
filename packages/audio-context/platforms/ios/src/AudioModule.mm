#import "AudioModule.h"

#import <AVFoundation/AVFoundation.h>
#import <CoreFoundation/CoreFoundation.h>

#if __has_include(<NativeScript/runtime/Runtime.h>)
#import <NativeScript/runtime/Runtime.h>
#elif __has_include("Runtime.h")
#import "Runtime.h"
#else
#import <runtime/Runtime.h>
#endif

static void NSCAudioModuleCreateFloat32ArrayFromPCMBufferAddress(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() < 1 || !args[0]->IsString()) {
        args.GetReturnValue().Set(v8::Null(isolate));
        return;
    }

    v8::String::Utf8Value utf8(isolate, args[0]);
    if (*utf8 == nullptr || utf8.length() == 0) {
        args.GetReturnValue().Set(v8::Null(isolate));
        return;
    }

    char *end = nullptr;
    unsigned long long rawPtr = strtoull(*utf8, &end, 0);
    if (end == *utf8 || rawPtr == 0ULL) {
        args.GetReturnValue().Set(v8::Null(isolate));
        return;
    }

    auto *buffer = (__bridge AVAudioPCMBuffer *)(void *)(uintptr_t)rawPtr;
    if (buffer == nil) {
        args.GetReturnValue().Set(v8::Null(isolate));
        return;
    }

    NSInteger channel = 0;
    if (args.Length() > 1 && args[1]->IsNumber()) {
        channel = args[1]->Int32Value(context).FromMaybe(0);
    }

    float * const *channels = buffer.floatChannelData;
    if (!channels) {
        args.GetReturnValue().Set(v8::Null(isolate));
        return;
    }

    NSInteger channelCount = (NSInteger)buffer.format.channelCount;
    if (channel < 0 || channel >= channelCount) {
        args.GetReturnValue().Set(v8::Null(isolate));
        return;
    }

    AVAudioFrameCount frames = buffer.frameLength;
    float *channelPtr = channels[channel];

    if (!channelPtr || frames == 0) {
        auto emptyBuffer = v8::ArrayBuffer::New(isolate, 0);
        args.GetReturnValue().Set(v8::Float32Array::New(emptyBuffer, 0, 0));
        return;
    }

    size_t byteLength = (size_t)frames * sizeof(float);
    CFTypeRef retainedBuffer = CFRetain((__bridge CFTypeRef)buffer);

    auto backing = v8::ArrayBuffer::NewBackingStore(
        (void *)channelPtr,
        byteLength,
        [](void *data, size_t length, void *deleter_data) {
            if (deleter_data != nullptr) {
                CFRelease((CFTypeRef)deleter_data);
            }
        },
        (void *)retainedBuffer);

    auto arrayBuffer = v8::ArrayBuffer::New(isolate, std::move(backing));
    auto typedArray = v8::Float32Array::New(arrayBuffer, 0, frames);
    args.GetReturnValue().Set(typedArray);
}

@implementation AudioModule

+ (void)install {
    tns::Runtime *runtime = tns::Runtime::GetCurrentRuntime();
    if (!runtime) return;

    v8::Isolate *isolate = runtime->GetIsolate();
    if (!isolate) return;

    v8::Locker locker(isolate);
    v8::Isolate::Scope isolateScope(isolate);
    v8::HandleScope handleScope(isolate);

    auto context = isolate->GetCurrentContext();
    if (context.IsEmpty()) return;
    v8::Context::Scope contextScope(context);

    auto global = context->Global();
    auto moduleKey = v8::String::NewFromUtf8Literal(isolate, "AudioModule");

    v8::Local<v8::Value> moduleValue;
    if (!global->Get(context, moduleKey).ToLocal(&moduleValue) || !moduleValue->IsObject()) {
        moduleValue = v8::Object::New(isolate);
        global->Set(context, moduleKey, moduleValue).FromMaybe(false);
    }

    auto moduleObject = moduleValue.As<v8::Object>();
    auto helperKey = v8::String::NewFromUtf8Literal(isolate, "createFloat32ArrayFromPCMBufferAddress");

    v8::Local<v8::Value> existing;
    if (moduleObject->Get(context, helperKey).ToLocal(&existing) && existing->IsFunction()) {
        return;
    }

    auto helper = v8::FunctionTemplate::New(isolate, NSCAudioModuleCreateFloat32ArrayFromPCMBufferAddress)
                      ->GetFunction(context)
                      .ToLocalChecked();
    moduleObject->Set(context, helperKey, helper).FromMaybe(false);
}

@end
