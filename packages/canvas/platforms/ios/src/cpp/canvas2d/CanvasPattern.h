//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once

#include "MatrixImpl.h"
#include <vector>
#include "ObjectWrapperImpl.h"

class CanvasPattern : ObjectWrapperImpl {
public:
    explicit CanvasPattern(PaintStyle *style);

    ~CanvasPattern() {
        canvas_native_paint_style_release(this->GetPaintStyle());
        this->style_ = nullptr;
    }

    PaintStyle *GetPaintStyle();

    static v8::CFunction fast_set_transform_;

    static void Init(const v8::Local<v8::Object> &canvasModule, v8::Isolate *isolate);

    static CanvasPattern *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, CanvasPattern *pattern) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = CanvasPattern::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(pattern, NativeType::CanvasPattern);
        object->SetAlignedPointerInInternalField(0, pattern);
        pattern->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void SetTransform(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastSetTransform(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> matrix_obj) {
        CanvasPattern *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(matrix_obj);

        if (type == NativeType::Matrix) {
            auto matrix = MatrixImpl::GetPointer(matrix_obj);
            if (matrix != nullptr) {
                canvas_native_pattern_set_transform(
                        ptr->GetPaintStyle(),
                        matrix->GetMatrix());
            }
        }

    }

private:
    PaintStyle *style_;
};

