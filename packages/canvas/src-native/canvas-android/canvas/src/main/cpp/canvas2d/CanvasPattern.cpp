//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasPattern.h"
#include "canvas-cxx/src/lib.rs.h"
#include "Helpers.h"
#include "Caches.h"

CanvasPattern::CanvasPattern(rust::Box<PaintStyle> style) : style_(std::move(style)) {}

void CanvasPattern::Init(const v8::Local<v8::Object> &canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "CanvasPattern"), func);
}

CanvasPattern *CanvasPattern::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<CanvasPattern *>(ptr);
}

v8::Local<v8::FunctionTemplate> CanvasPattern::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->CanvasGradientTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "CanvasPattern"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);
    tmpl->Set(
            ConvertToV8String(isolate, "SetTransform"),
            v8::FunctionTemplate::New(isolate, &SetTransform));

    cache->CanvasGradientTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void CanvasPattern::SetTransform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasPattern *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];

    if (!value->IsNullOrUndefined()) {
        // todo
//        auto matrix =
//        if (matrix != nullptr) {
//            canvas_native_pattern_set_transform(
//                    ptr->GetPaintStyle(),
//                    matrix->GetMatrix());
//        }
    }


    args.GetReturnValue().SetUndefined();
}

PaintStyle &CanvasPattern::GetPaintStyle() {
    return *this->style_;
}
