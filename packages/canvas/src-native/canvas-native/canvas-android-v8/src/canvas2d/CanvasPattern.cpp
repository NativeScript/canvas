//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasPattern.h"
#include "canvas-android-v8/src/bridges/context.rs.h"

CanvasPattern::CanvasPattern(rust::Box<PaintStyle> style) : style_(std::move(style)) {}

void CanvasPattern::Init(v8::Isolate *isolate) {
    auto ctorFunc = GetCtorFunc(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "CanvasPattern"),
                ctorFunc->GetFunction(context).ToLocalChecked());
}

CanvasPattern *CanvasPattern::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<CanvasPattern *>(ptr);
}

v8::Local<v8::Object> CanvasPattern::NewInstance(v8::Isolate *isolate, rust::Box<PaintStyle> style) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto ctorFunc = GetCtorFunc(isolate);
    CanvasPattern *gradient = new CanvasPattern(std::move(style));
    auto result = ctorFunc->InstanceTemplate()->NewInstance(isolate->GetCurrentContext()).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, result, "CanvasPattern");
    auto ext = v8::External::New(isolate, gradient);
    result->SetInternalField(0, ext);
    return handle_scope.Escape(result);
}

void CanvasPattern::CreateCallback(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Helpers::ThrowIllegalConstructor(args.GetIsolate());
}

void CanvasPattern::SetTransform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    if (args.Length() == 1) {
        auto isolate = args.GetIsolate();
        auto context = isolate->GetCurrentContext();
        if (args[0]->IsObject()) {
            auto matrix = args[0]->ToObject(context).ToLocalChecked();
            if (Helpers::IsInstanceOf(isolate, matrix, "DOMMatrix")) {
                auto ptr = GetPointer(args.Holder());
                auto matrix_ptr = MatrixImpl::GetPointer(matrix);
                canvas_native_pattern_set_transform(ptr->GetPaintStyle(), matrix_ptr->GetMatrix());
            }
        }
    }
}

v8::Local<v8::FunctionTemplate> CanvasPattern::GetCtorFunc(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto func = cache->CanvasPatternTmpl.get();
    if (func != nullptr) {
        return func->Get(isolate);
    }
    auto context = isolate->GetCurrentContext();
    auto patternTpl = v8::FunctionTemplate::New(isolate, &CreateCallback);
    patternTpl->SetClassName(Helpers::ConvertToV8String(isolate, "CanvasPattern"));
    patternTpl->InstanceTemplate()->SetInternalFieldCount(1);

    patternTpl->InstanceTemplate()->Set(
            Helpers::ConvertToV8String(isolate, "setTransform"),
            v8::FunctionTemplate::New(isolate, &SetTransform)
    );

    cache->CanvasPatternTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, patternTpl);

    return patternTpl;
}

PaintStyle &CanvasPattern::GetPaintStyle() {
    return *this->style_;
}