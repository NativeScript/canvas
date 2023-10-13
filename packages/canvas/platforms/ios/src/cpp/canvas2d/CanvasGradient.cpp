//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasGradient.h"
#include "Helpers.h"
#include "Caches.h"

CanvasGradient::CanvasGradient(rust::Box<PaintStyle> style) : style_(std::move(style)) {}

void CanvasGradient::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "CanvasGradient"), func);
}

CanvasGradient *CanvasGradient::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<CanvasGradient *>(ptr);
}

v8::Local<v8::FunctionTemplate> CanvasGradient::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->CanvasGradientTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "CanvasGradient"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);
    tmpl->Set(
            ConvertToV8String(isolate, "addColorStop"),
            v8::FunctionTemplate::New(isolate, &AddColorStop));

    cache->CanvasGradientTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void CanvasGradient::AddColorStop(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasGradient *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto stop = (float) args[0]->NumberValue(context).ToChecked();
    auto color = ConvertFromV8String(isolate, args[1]);
    canvas_native_gradient_add_color_stop(
            ptr->GetPaintStyle(), stop,
            rust::Str(color.c_str()));

    args.GetReturnValue().SetUndefined();
}

PaintStyle &CanvasGradient::GetPaintStyle() {
    return *this->style_;
}

