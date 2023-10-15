//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasPattern.h"
#include "Helpers.h"
#include "Caches.h"

CanvasPattern::CanvasPattern(PaintStyle* style) : style_(style) {}

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

static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, CanvasPattern *pattern) {
    auto context = isolate->GetCurrentContext();
    v8::EscapableHandleScope scope(isolate);
    auto object = CanvasPattern::GetCtor(isolate)->GetFunction(
            context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
    SetNativeType(isolate, object, NativeType::CanvasPattern);
    auto ext = v8::External::New(isolate, pattern);
    object->SetInternalField(0, ext);
    return scope.Escape(object);
}

v8::Local<v8::FunctionTemplate> CanvasPattern::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->CanvasPatternTmpl.get();
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

    cache->CanvasPatternTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void CanvasPattern::SetTransform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasPattern *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();

    auto value = args[0];
    auto type = GetNativeType(isolate, value);

    if (type == NativeType::Matrix) {

        auto matrix = MatrixImpl::GetPointer(value.As<v8::Object>());
        if (matrix != nullptr) {
            canvas_native_pattern_set_transform(
                    ptr->GetPaintStyle(),
                    matrix->GetMatrix());
        }
    }

}

PaintStyle* CanvasPattern::GetPaintStyle() {
    return this->style_;
}
