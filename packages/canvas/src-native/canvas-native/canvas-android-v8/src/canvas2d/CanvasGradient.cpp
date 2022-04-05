//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasGradient.h"

void CanvasGradient::Init(v8::Isolate *isolate) {
    auto ctorFunc = GetCtorFunc(isolate);
    auto context = isolate->GetCurrentContext()
    auto global = context->Global();
    global->Set(context, v8::String::NewFromUtf8(isolate, "CanvasGradient").ToLocalChecked(), ctorFunc);
}


v8::Local<v8::Object> CanvasGradient::Create(v8::Local<v8::Context> context, intptr_t gradient) {
    auto ctorFunc = CanvasGradient::GetCtorFunc(context->GetIsolate());
    auto result = ctorFunc->NewInstance(context).ToLocalChecked();
    auto ext = v8::External::New(context->GetIsolate(), reinterpret_cast<void *>(gradient));
    result->SetInternalField(0, ext);
    return result;
}

void CanvasGradient::CreateCallback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Helpers::ThrowIllegalConstructor(args.GetIsolate());
}

void CanvasGradient::AddColorStop(const v8::FunctionCallbackInfo<v8::Value> &args) {
    if (args.Length() == 2 &&
        ((args[0]->IsNumber() || args[0]->IsNumberObject()) && (args[1]->IsString() || args[1]->IsStringObject()))) {
        auto obj = args.Holder();
        auto ptr = static_cast<intptr_t>(reinterpret_cast<size_t>(obj->GetInternalField(
                0).As<v8::External>()->Value()));
        auto context = args.GetIsolate()->GetCurrentContext();
        auto stop = args[0]->NumberValue(context).ToChecked();
        v8::String::Utf8Value color(args[0]->ToString(context).ToLocalChecked());
        canvas_native_gradient_add_color_stop(ptr, stop, color);
    }
}

v8::Local<v8::Function> CanvasGradient::GetCtorFunc(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto func = cache->CanvasGradientCtorFunc.get();
    if (func != nullptr) {
        return func->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();
    auto gradientTpl = v8::ObjectTemplate::New(isolate);
    gradientTpl->SetInternalFieldCount(1);

    auto addColorStopFt = v8::FunctionTemplate::New(isolate, AddColorStop);
    addColorStopFt->SetClassName(Helpers::ConvertToV8String("CanvasGradient"));
    gradientTpl->Set(
            v8::String::NewFromUtf8(isolate, "addColorStop").ToLocalChecked(),
            addColorStopFt->GetFunction(context).ToLocalChecked()
    );

    Local <Object> obj = class_tpl->NewInstance(context).ToLocalChecked();

    auto tpl = v8::FunctionTemplate::New(isolate, CreateCallback);

    auto ctorFunc = tpl->GetFunction(context).ToLocalChecked();

    cache->CanvasGradientCtorFunc = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(ctorFunc);

    return ctorFunc;
}


