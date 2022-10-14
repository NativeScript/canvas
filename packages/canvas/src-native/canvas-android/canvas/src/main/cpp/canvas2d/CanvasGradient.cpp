//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasGradient.h"


CanvasGradient::CanvasGradient(rust::Box<PaintStyle> style) : style_(std::move(style)) {}

void CanvasGradient::Init(v8::Isolate *isolate) {
    auto ctorFunc = GetCtorFunc(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, v8::String::NewFromUtf8(isolate, "CanvasGradient").ToLocalChecked(),
                ctorFunc->GetFunction(context).ToLocalChecked());
}

CanvasGradient *CanvasGradient::GetPointer(const v8::Local<v8::Object> &object) {
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
    auto ctorFunc = GetCtorFunc(isolate);
    CanvasGradient *gradient = new CanvasGradient(std::move(style));
    auto result = ctorFunc->InstanceTemplate()->NewInstance(isolate->GetCurrentContext()).ToLocalChecked();
    Helpers::SetInstanceType(isolate, result, ObjectType::CanvasGradient);
    AddWeakListener(isolate, result, gradient);
    return handle_scope.Escape(result);
}

void CanvasGradient::CreateCallback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Helpers::ThrowIllegalConstructor(args.GetIsolate());
}

void CanvasGradient::AddColorStop(const v8::FunctionCallbackInfo<v8::Value> &args) {
    if (args.Length() == 2 &&
        Helpers::IsNumber(args[0]) && Helpers::IsString(args[1])) {
        auto isolate = args.GetIsolate();
        auto ptr = GetPointer(args.This());
        auto context = isolate->GetCurrentContext();
        auto stop = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto color = Helpers::ConvertFromV8String(isolate, args[1]);
        canvas_native_gradient_add_color_stop(*ptr->style_, stop, rust::Str(color.c_str(), color.size()));
    }
}

v8::Local<v8::FunctionTemplate> CanvasGradient::GetCtorFunc(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto func = cache->CanvasGradientTmpl.get();
    if (func != nullptr) {
        return func->Get(isolate);
    }
    auto gradientTpl = v8::FunctionTemplate::New(isolate, &CreateCallback);
    gradientTpl->SetClassName(Helpers::ConvertToV8String(isolate, "CanvasGradient"));
    gradientTpl->InstanceTemplate()->SetInternalFieldCount(1);

    gradientTpl->PrototypeTemplate()->Set(
            v8::String::NewFromUtf8(isolate, "addColorStop").ToLocalChecked(),
            v8::FunctionTemplate::New(isolate, &AddColorStop)
    );

    cache->CanvasGradientTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, gradientTpl);

    return gradientTpl;
}


PaintStyle &CanvasGradient::GetPaintStyle() {
    return *this->style_;
}

