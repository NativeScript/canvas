//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasGradient.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

CanvasGradient::CanvasGradient(rust::Box<PaintStyle> style) : style_(std::move(style)) {}

void CanvasGradient::Init(v8::Isolate *isolate) {
    auto ctorFunc = GetCtorFunc(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, v8::String::NewFromUtf8(isolate, "CanvasGradient").ToLocalChecked(), ctorFunc->GetFunction(context).ToLocalChecked());
}

CanvasGradient *CanvasGradient::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<CanvasGradient *>(ptr);
}


v8::Local<v8::Object> CanvasGradient::NewInstance(v8::Isolate *isolate, rust::Box<PaintStyle> style) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto ctorFunc = CanvasGradient::GetCtorFunc(isolate);
    CanvasGradient *gradient = new CanvasGradient(std::move(style));
    auto result = ctorFunc->InstanceTemplate()->NewInstance(isolate->GetCurrentContext()).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result, "CanvasGradient");
    auto ext = v8::External::New(isolate, gradient);
    result->SetInternalField(0, ext);
    return handle_scope.Escape(result);
}

void CanvasGradient::CreateCallback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Helpers::ThrowIllegalConstructor(args.GetIsolate());
}

void CanvasGradient::AddColorStop(const v8::FunctionCallbackInfo<v8::Value> &args) {
    if (args.Length() == 2 &&
        ((args[0]->IsNumber() || args[0]->IsNumberObject()) && (args[1]->IsString() || args[1]->IsStringObject()))) {
        auto isolate = args.GetIsolate();
        auto ptr = GetPointer(args.Holder());
        auto context = isolate->GetCurrentContext();
        auto stop = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto color = Helpers::ConvertFromV8String(isolate, args[1]->ToString(context).ToLocalChecked());
        canvas_native_gradient_add_color_stop(*ptr->style_, stop, color);
    }
}

v8::Local<v8::FunctionTemplate> CanvasGradient::GetCtorFunc(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto func = cache->CanvasGradientTmpl.get();
    if (func != nullptr) {
        return func->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();
    auto gradientTpl = v8::FunctionTemplate::New(isolate, &CreateCallback);
    gradientTpl->SetClassName(Helpers::ConvertToV8String(isolate, "CanvasGradient"));
    gradientTpl->InstanceTemplate()->SetInternalFieldCount(1);

    gradientTpl->InstanceTemplate()->Set(
            v8::String::NewFromUtf8(isolate, "addColorStop").ToLocalChecked(),
            v8::FunctionTemplate::New(isolate, &AddColorStop)
    );

    cache->CanvasGradientTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, gradientTpl);

    return gradientTpl;
}


PaintStyle &CanvasGradient::GetPaintStyle() {
    return *this->style_;
}

