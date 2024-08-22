//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasPattern.h"
#include "Helpers.h"
#include "Caches.h"

CanvasPattern::CanvasPattern(PaintStyle *style) : style_(style) {}

void CanvasPattern::Init(const v8::Local<v8::Object> &canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "CanvasPattern"), func).FromJust();
}

v8::CFunction CanvasPattern::fast_set_transform_(
        v8::CFunction::Make(CanvasPattern::FastSetTransform));

CanvasPattern *CanvasPattern::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<CanvasPattern *>(ptr);
}

v8::Local<v8::FunctionTemplate> CanvasPattern::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->CanvasPatternTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "CanvasPattern"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    SetFastMethod(isolate, tmpl, "setTransform", SetTransform, &fast_set_transform_,
                  v8::Local<v8::Value>());

    cache->CanvasPatternTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void CanvasPattern::SetTransform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasPattern *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto value = args[0];
    auto type = GetNativeType(value);

    if (type == NativeType::Matrix) {

        auto matrix = MatrixImpl::GetPointer(value.As<v8::Object>());
        if (matrix != nullptr) {
            canvas_native_pattern_set_transform(
                    ptr->GetPaintStyle(),
                    matrix->GetMatrix());
        }
    }

}

PaintStyle *CanvasPattern::GetPaintStyle() {
    return this->style_;
}
