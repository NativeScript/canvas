//
// Created by Osei Fortune on 18/04/2022.
//

#include "TextMetricsImpl.h"

TextMetricsImpl::TextMetricsImpl(rust::Box <TextMetrics> metrics) : metrics_(std::move(metrics)) {

}

void TextMetricsImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "TextMetrics"), ctor);
}

void TextMetricsImpl::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto err = v8::Exception::TypeError(
            Helpers::ConvertToV8String(
                    isolate,
                    "Illegal constructor"
            )
    );
    isolate->ThrowException(err);
    return;
}

v8::Local<v8::Object> TextMetricsImpl::NewInstance(v8::Isolate *isolate, rust::Box <TextMetrics> metrics) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto ret = GetCtor(isolate);

    ret->SetPrivate(context, v8::Private::New(isolate, Helpers::ConvertToV8String(isolate, "class_name")),
                    Helpers::ConvertToV8String(isolate, "TextMetrics"));

    TextMetricsImpl *value = new TextMetricsImpl(std::move(metrics));
    auto ext = v8::External::New(isolate, value);
    ret->SetInternalField(0, ext);

    return handle_scope.Escape(ret);
}

TextMetricsImpl *TextMetricsImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<TextMetricsImpl *>(ptr);
}

v8::Local<v8::Object> TextMetricsImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto tmpl = cache->TextMetricsTmpl.get();
    auto context = isolate->GetCurrentContext();
    if (tmpl != nullptr) {
        return tmpl->Get(isolate);
    }
    auto textMetricsFuncTmpl = v8::FunctionTemplate::New(isolate, &Create);
    textMetricsFuncTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    textMetricsFuncTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "TextMetrics"));

    auto func = textMetricsFuncTmpl->GetFunction(context).ToLocalChecked();

    auto textMetricsTmpl = textMetricsFuncTmpl->InstanceTemplate();
    textMetricsTmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "width"), &GetWidth);

    textMetricsTmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "actualBoundingBoxLeft"), &ActualBoundingBoxLeft);
    textMetricsTmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "actualBoundingBoxRight"),
                                 &ActualBoundingBoxRight);

    textMetricsTmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "actualBoundingBoxAscent"),
                                 &ActualBoundingBoxAscent);
    textMetricsTmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "actualBoundingBoxDescent"),
                                 &ActualBoundingBoxDescent);

    textMetricsTmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "fontBoundingBoxAscent"), &FontBoundingBoxAscent);
    textMetricsTmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "fontBoundingBoxDescent"),
                                 &FontBoundingBoxDescent);

    textMetricsTmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "emHeightAscent"), &EmHeightAscent);
    textMetricsTmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "emHeightDescent"), &EmHeightDescent);
    textMetricsTmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "hangingBaseline"), &HangingBaseline);
    textMetricsTmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "alphabeticBaseline"), &AlphabeticBaseline);
    textMetricsTmpl->SetAccessor(Helpers::ConvertToV8String(isolate, "ideographicBaseline"), &IdeographicBaseline);

    cache->TextMetricsTmpl = std::make_unique<v8::Persistent<v8::Function>>(isolate, func);

    return func;
}

void TextMetricsImpl::GetWidth(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto width = static_cast<double>(canvas_native_text_metrics_get_width(*ptr->metrics_));
    info.GetReturnValue().Set(width);
}

void
TextMetricsImpl::ActualBoundingBoxLeft(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto width = static_cast<double>(canvas_native_text_metrics_get_actual_bounding_box_left(*ptr->metrics_));
    info.GetReturnValue().Set(width);
}

void
TextMetricsImpl::ActualBoundingBoxRight(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto width = static_cast<double>(canvas_native_text_metrics_get_actual_bounding_box_right(*ptr->metrics_));
    info.GetReturnValue().Set(width);
}

void
TextMetricsImpl::ActualBoundingBoxAscent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto width = static_cast<double>(canvas_native_text_metrics_get_actual_bounding_box_ascent(*ptr->metrics_));
    info.GetReturnValue().Set(width);
}

void
TextMetricsImpl::ActualBoundingBoxDescent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto width = static_cast<double>(canvas_native_text_metrics_get_actual_bounding_box_descent(*ptr->metrics_));
    info.GetReturnValue().Set(width);
}

void
TextMetricsImpl::FontBoundingBoxAscent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto width = static_cast<double>(canvas_native_text_metrics_get_font_bounding_box_ascent(*ptr->metrics_));
    info.GetReturnValue().Set(width);
}

void
TextMetricsImpl::FontBoundingBoxDescent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto width = static_cast<double>(canvas_native_text_metrics_get_font_bounding_box_descent(*ptr->metrics_));
    info.GetReturnValue().Set(width);
}

void TextMetricsImpl::EmHeightAscent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto width = static_cast<double>(canvas_native_text_metrics_get_em_height_ascent(*ptr->metrics_));
    info.GetReturnValue().Set(width);
}

void TextMetricsImpl::EmHeightDescent(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto width = static_cast<double>(canvas_native_text_metrics_get_em_height_descent(*ptr->metrics_));
    info.GetReturnValue().Set(width);
}

void TextMetricsImpl::HangingBaseline(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto width = static_cast<double>(canvas_native_text_metrics_get_hanging_baseline(*ptr->metrics_));
    info.GetReturnValue().Set(width);
}

void TextMetricsImpl::AlphabeticBaseline(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto width = static_cast<double>(canvas_native_text_metrics_get_alphabetic_baseline(*ptr->metrics_));
    info.GetReturnValue().Set(width);
}

void TextMetricsImpl::IdeographicBaseline(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto width = static_cast<double>(canvas_native_text_metrics_get_ideographic_baseline(*ptr->metrics_));
    info.GetReturnValue().Set(width);
}