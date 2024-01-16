//
// Created by Osei Fortune on 18/04/2022.
//

#include "TextMetricsImpl.h"
#include "Caches.h"

TextMetricsImpl::TextMetricsImpl(TextMetrics* metrics) : metrics_(metrics) {}

void TextMetricsImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "TextMetrics"), func);
}

TextMetricsImpl *TextMetricsImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<TextMetricsImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> TextMetricsImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->TextMetricsTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "TextMetrics"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "width"),
            GetWidth);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "actualBoundingBoxLeft"),
            GetActualBoundingBoxLeft);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "actualBoundingBoxRight"),
            GetActualBoundingBoxRight);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "actualBoundingBoxAscent"),
            GetActualBoundingBoxAscent);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "actualBoundingBoxDescent"),
            GetActualBoundingBoxDescent);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "fontBoundingBoxAscent"),
            GetFontBoundingBoxAscent);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "fontBoundingBoxDescent"),
            GetFontBoundingBoxDescent);


    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "emHeightAscent"),
            GetEmHeightAscent);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "emHeightDescent"),
            GetEmHeightDescent);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "hangingBaseline"),
            GetHangingBaseline);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "alphabeticBaseline"),
            GetAlphabeticBaseline);

    tmpl->SetLazyDataProperty(
            ConvertToV8String(isolate, "ideographicBaseline"),
            GetIdeographicBaseline);

    cache->TextMetricsTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}


void TextMetricsImpl::GetWidth(v8::Local<v8::Name> property,
                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    TextMetricsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_text_metrics_get_width(ptr->GetTextMetrics()));
}


void TextMetricsImpl::GetActualBoundingBoxLeft(v8::Local<v8::Name> property,
                                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    TextMetricsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_text_metrics_get_actual_bounding_box_left(
            ptr->GetTextMetrics()));
}


void TextMetricsImpl::GetActualBoundingBoxRight(v8::Local<v8::Name> property,
                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    TextMetricsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_text_metrics_get_actual_bounding_box_right(
            ptr->GetTextMetrics()));
}


void TextMetricsImpl::GetActualBoundingBoxAscent(v8::Local<v8::Name> property,
                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    TextMetricsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_text_metrics_get_actual_bounding_box_ascent(
            ptr->GetTextMetrics()));
}


void TextMetricsImpl::GetActualBoundingBoxDescent(v8::Local<v8::Name> property,
                                                  const v8::PropertyCallbackInfo<v8::Value> &info) {
    TextMetricsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_text_metrics_get_actual_bounding_box_descent(
            ptr->GetTextMetrics()));
}


void TextMetricsImpl::GetFontBoundingBoxAscent(v8::Local<v8::Name> property,
                                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    TextMetricsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_text_metrics_get_font_bounding_box_ascent(
            ptr->GetTextMetrics()));
}


void TextMetricsImpl::GetFontBoundingBoxDescent(v8::Local<v8::Name> property,
                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    TextMetricsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }

    info.GetReturnValue().Set((double) canvas_native_text_metrics_get_font_bounding_box_descent(
            ptr->GetTextMetrics()));
}


void TextMetricsImpl::GetEmHeightAscent(v8::Local<v8::Name> property,
                                        const v8::PropertyCallbackInfo<v8::Value> &info) {
    TextMetricsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }

    info.GetReturnValue().Set(
            (double) canvas_native_text_metrics_get_em_height_ascent(ptr->GetTextMetrics()));
}


void TextMetricsImpl::GetEmHeightDescent(v8::Local<v8::Name> property,
                                         const v8::PropertyCallbackInfo<v8::Value> &info) {
    TextMetricsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }

    info.GetReturnValue().Set(
            (double) canvas_native_text_metrics_get_em_height_descent(ptr->GetTextMetrics()));
}


void TextMetricsImpl::GetHangingBaseline(v8::Local<v8::Name> property,
                                         const v8::PropertyCallbackInfo<v8::Value> &info) {
    TextMetricsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }

    info.GetReturnValue().Set(
            (double) canvas_native_text_metrics_get_hanging_baseline(ptr->GetTextMetrics()));
}

void TextMetricsImpl::GetAlphabeticBaseline(v8::Local<v8::Name> property,
                                            const v8::PropertyCallbackInfo<v8::Value> &info) {
    TextMetricsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }

    info.GetReturnValue().Set(
            (double) canvas_native_text_metrics_get_alphabetic_baseline(ptr->GetTextMetrics()));
}

void TextMetricsImpl::GetIdeographicBaseline(v8::Local<v8::Name> property,
                                             const v8::PropertyCallbackInfo<v8::Value> &info) {
    TextMetricsImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }

    info.GetReturnValue().Set(
            (double) canvas_native_text_metrics_get_ideographic_baseline(ptr->GetTextMetrics()));
}

TextMetrics* TextMetricsImpl::GetTextMetrics() {
    return this->metrics_;
}
