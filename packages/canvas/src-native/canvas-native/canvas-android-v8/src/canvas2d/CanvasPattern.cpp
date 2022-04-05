//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasPattern.h"

void CanvasPattern::Init(v8::Isolate *isolate) {
    auto ctorFunc = GetCtorFunc(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, v8::String::NewFromUtf8(isolate, "CanvasPattern").ToLocalChecked(), ctorFunc);
}

v8::Local<v8::Function> CanvasPattern::GetCtorFunc(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto func = cache->CanvasPatternCtorFunc.get();
    if (func != nullptr) {
        return func->Get(isolate);
    }

    auto context = isolate->GetCurrentContext();
    auto patternTpl = v8::ObjectTemplate::New(isolate);
    patternTpl->SetInternalFieldCount(1);

    auto setTransformFunc = v8::FunctionTemplate::New(isolate, SetTransform);

    patternTpl->Set(
            v8::String::NewFromUtf8(isolate, "setTransform").ToLocalChecked(),
            setTransformFunc->GetFunction(context).ToLocalChecked()
    );

    Local <Object> obj = patternTpl->NewInstance(context).ToLocalChecked();

    auto tpl = v8::FunctionTemplate::New(isolate, CreateCallback);

    tpl->SetClassName(Helpers::ConvertToV8String(isolate, "CanvasPattern"));

    auto ctorFunc = tpl->GetFunction(context).ToLocalChecked();

    cache->CanvasPatternCtorFunc = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(ctorFunc);

    return ctorFunc;
}

v8::Local<v8::Object> CanvasPattern::Create(v8::Local<v8::Context> context, intptr_t pattern) {
    auto ctorFunc = CanvasPattern::GetCtorFunc(context->GetIsolate());
    auto result = ctorFunc->NewInstance(context).ToLocalChecked();
    auto ext = v8::External::New(context->GetIsolate(), reinterpret_cast<void *>(pattern));
    result->SetInternalField(0, ext);
    return result;
}

void CanvasPattern::CreateCallback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Helpers::ThrowIllegalConstructor(args.GetIsolate());
}

void CanvasPattern::SetTransform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    if (args.Length() == 1) {
        // todo check if instanceof matrix
        auto obj = args.Holder();
        auto ptr = static_cast<intptr_t>(reinterpret_cast<size_t>(obj->GetInternalField(
                0).As<v8::External>()->Value()));
        auto context = args.GetIsolate()->GetCurrentContext();
        if (args[0]->IsNullOrUndefined()) {
            return;
        }
        auto matrix = args[0]->ToObject(context).ToLocalChecked();
        auto matrix_ptr = static_cast<intptr_t>(reinterpret_cast<size_t>(matrix->GetInternalField(
                0).As<v8::External>()->Value()));
        canvas_native_pattern_set_transform(ptr, matrix_ptr);
    }
}
