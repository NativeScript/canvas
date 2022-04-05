//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasRenderingContext2DImpl.h"

CanvasRenderingContext2DImpl::CanvasRenderingContext2DImpl(rust::Box<CanvasRenderingContext2D> context): context_(std::move(context)) {}

void CanvasRenderingContext2DImpl::Init(v8::Isolate *isolate) {
    auto ctorFunc = GetCtorFunc(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "CanvasRenderingContext2DImpl"), ctorFunc);
}

v8::Local<v8::Function> CanvasRenderingContext2DImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto context = isolate->GetCurrentContext();
    auto tmpl = cache->CanvasRenderingContext2DCtor.get();
    if (tmpl != nullptr) {
        return tmpl->Get(isolate);
    }
    auto canvasRenderingContextFunc = v8::FunctionTemplate::New(isolate);

    auto canvasRenderingContextTpl = canvasRenderingContextFunc->InstanceTemplate();

    canvasRenderingContextTpl->SetInternalFieldCount(1);

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "globalAlpha"),
            &GetGlobalAlpha,
            &SetGlobalAlpha
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "imageSmoothingEnabled"),
            &GetImageSmoothingEnabled,
            &SetImageSmoothingEnabled
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "imageSmoothingQuality"),
            &GetImageSmoothingQuality,
            &SetImageSmoothingQuality
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "lineDashOffset"),
            &GetLineDashOffset,
            &SetLineDashOffset
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "lineJoin"),
            &GetLineJoin,
            &SetLineJoin
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "lineCap"),
            &GetLineCap,
            &SetLineCap
    );


    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "miterLimit"),
            &GetMiterLimit,
            &SetMiterLimit
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "shadowColor"),
            &GetShadowColor,
            &SetShadowColor
    );


    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "shadowBlur"),
            &GetShadowBlur,
            &SetShadowBlur
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "shadowOffsetX"),
            &GetShadowOffsetX,
            &SetShadowOffsetX
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "shadowOffsetY"),
            &GetShadowOffsetY,
            &SetShadowOffsetY
    );


    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "textAlign"),
            &GetTextAlign,
            &SetTextAlign
    );


    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "globalCompositeOperation"),
            &GetGlobalCompositeOperation,
            &SetGlobalCompositeOperation
    );


    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "fillStyle"),
            &GetFillStyle,
            &SetFillStyle
    );


    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "strokeStyle"),
            &GetStrokeStyle,
            &SetStrokeStyle
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "lineWidth"),
            &GetLineWidth,
            &SetLineWidth
    );


    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "lineDash"),
            &GetLineDash,
            &SetLineDash
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String("addHitRegion"),
            v8::FunctionTemplate::New(isolate, &AddHitRegion)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String("arc"),
            v8::FunctionTemplate::New(isolate, &Arc)
    );


    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String("arcTo"),
            v8::FunctionTemplate::New(isolate, &ArcTo)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String("beginPath"),
            v8::FunctionTemplate::New(isolate, &BeginPath)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String("bezierCurveTo"),
            v8::FunctionTemplate::New(isolate, &BezierCurveTo)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String("clearHitRegions"),
            v8::FunctionTemplate::New(isolate, &ClearHitRegions)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String("clearRect"),
            v8::FunctionTemplate::New(isolate, &ClearRect)
    );

    auto func = canvasRenderingContextFunc->GetFunction(context).ToLocalChecked();

    cache->CanvasRenderingContext2DCtor = std::make_unique<v8::Persistent<v8::Function>>(isolate, func);

    return func;
}

void CanvasRenderingContext2DImpl::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ctorFunc = GetCtor(context->GetIsolate());
    auto result = ctorFunc->NewInstance(context).ToLocalChecked();
    auto ptr = new CanvasRenderingContext2DImpl();
    auto ext = v8::External::New(context->GetIsolate(), renderingContext);
    result->SetInternalField(0, ext);
    v8::Persistent<v8::Object> handle(isolate, ctorFunc->NewInstance(context).ToLocalChecked());
    handle.SetWeak(renderingContext, &Dispose);
    return result;
}

CanvasRenderingContext2DImpl *CanvasRenderingContext2DImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr
    }

    return static_cast<CanvasRenderingContext2DImpl *>(ptr);
}

void
CanvasRenderingContext2DImpl::GetGlobalAlpha(v8::Local<v8::String> name,
                                             const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto alpha = canvas_native_context_get_global_alpha(ptr);
    info.GetReturnValue().Set(static_cast<double>(alpha));
}

void CanvasRenderingContext2DImpl::SetGlobalAlpha(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                  const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber())) {
        auto isolate = info.GetIsolate();
        auto ptr = GetPointer(info.Holder());
        auto alpha = value->NumberValue(isolate->GetCurrentContext()).ToChecked();
        canvas_native_context_set_global_alpha(ptr, static_cast<float>(alpha));
    }
}

void CanvasRenderingContext2DImpl::GetImageSmoothingEnabled(v8::Local<v8::String> name,
                                                            const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    info.GetReturnValue().Set(canvas_native_context_get_image_smoothing_enabled(ptr));
}

void CanvasRenderingContext2DImpl::SetImageSmoothingEnabled(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                            const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsBoolean())) {
        auto isolate = info.GetIsolate();
        auto ptr = GetPointer(info.Holder());
        canvas_native_context_set_image_smoothing_enabled(ptr, value->BooleanValue(isolate));
    }
}

void CanvasRenderingContext2DImpl::GetImageSmoothingQuality(v8::Local<v8::String> name,
                                                            const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto quality = canvas_native_context_get_image_smoothing_quality(ptr);
    info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, quality));
}

void CanvasRenderingContext2DImpl::SetImageSmoothingQuality(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                            const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsString())) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        auto quality = Helpers::ConvertFromV8String(isolate,
                                                    value->ToString(context).ToLocalChecked());

        canvas_native_context_set_image_smoothing_quality(ptr, Helpers::ConvertFromV8String(isolate,val));

    }
}


void CanvasRenderingContext2DImpl::GetLineJoin(v8::Local<v8::String> name,
                                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto join = canvas_native_context_get_line_join(ptr);
    info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, join));
}

void CanvasRenderingContext2DImpl::SetLineJoin(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                               const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsString())) {
        auto isolate = info.GetIsolate();
        auto ptr = GetPointer(info.Holder());
        auto join = Helpers::ConvertFromV8String(isolate, value->ToString().ToLocalChecked());
        canvas_native_context_set_line_join(ptr, join);
    }
}


void CanvasRenderingContext2DImpl::GetLineCap(v8::Local<v8::String> name,
                                              const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto cap = canvas_native_context_get_line_cap(ptr);
    info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, cap));
}

void CanvasRenderingContext2DImpl::SetLineCap(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                              const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsString())) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());

        auto cap = Helpers::ConvertFromV8String(isolate, value->ToString().ToLocalChecked());

        canvas_native_context_set_line_join(ptr, cap);
    }
}

void CanvasRenderingContext2DImpl::GetMiterLimit(v8::Local<v8::String> name,
                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_context_get_miter_limit(ptr));
    info.GetReturnValue().Set(ret);
}

void CanvasRenderingContext2DImpl::SetMiterLimit(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                 const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber())) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_context_set_miter_limit(ptr, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void CanvasRenderingContext2DImpl::GetShadowColor(v8::Local<v8::String> name,
                                                  const const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto color = canvas_native_get_shadow_color(ptr);
    info.GetReturnValue().Set(
            Helpers::ConvertToV8String(
                    isolate, color
            )
    );
}

void CanvasRenderingContext2DImpl::SetShadowColor(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                  const const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsString()) {
        auto isolate = info.GetIsolate();
        auto ptr = GetPointer(info.Holder());
        auto color = ConvertFromV8String(isolate, value->ToString(isolate->GetCurrentContext()).ToLocalChecked());
        canvas_native_set_shadow_color_string(ptr, color.c_str());
    }
}

void CanvasRenderingContext2DImpl::GetShadowBlur(v8::Local<v8::String> name,
                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_context_get_shadow_blur(ptr));
    info.GetReturnValue().Set(ret);
}

void CanvasRenderingContext2DImpl::SetShadowBlur(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                 const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber())) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_context_set_shadow_blur(ptr, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void CanvasRenderingContext2DImpl::GetShadowOffsetX(v8::Local<v8::String> name,
                                                    const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_context_get_shadow_offset_x(ptr));
    info.GetReturnValue().Set(ret);
}

void CanvasRenderingContext2DImpl::SetShadowOffsetX(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                    const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber())) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_context_set_shadow_offset_x(ptr, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void CanvasRenderingContext2DImpl::GetShadowOffsetY(v8::Local<v8::String> name,
                                                    const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_context_get_shadow_offset_y(ptr));
    info.GetReturnValue().Set(ret);
}

void CanvasRenderingContext2DImpl::SetShadowOffsetY(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                    const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber())) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_context_set_shadow_offset_y(ptr, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void CanvasRenderingContext2DImpl::GetTextAlign(v8::Local<v8::String> name,
                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto ret = canvas_native_context_get_text_align(ptr);
    info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, ret));
}

void CanvasRenderingContext2DImpl::SetTextAlign(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsString())) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        auto alignment = Helpers::ConvertFromV8String(isolate, value->ToString(context).ToLocalChecked());
        canvas_native_context_set_text_align(ptr, alignment);
    }
}

void CanvasRenderingContext2DImpl::GetGlobalCompositeOperation(v8::Local<v8::String> name,
                                                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto ret: std::string;
    switch (canvas_native_context_get_global_composite_operation(ptr)) {
        case 0:
            ret = "source-over";
            break;
        case 1:
            ref = "source-in";
            break;
        case 2:
            ref = "source-out";
            break;
        case 3:
            ref = "source-atop";
            break;
        case 4:
            ref = "destination-over";
            break;
        case 5:
            ref = "destination-in";
            break;
        case 6:
            ref = "destination-out";
            break;
        case 7:
            ref = "destination-atop";
            break;
        case 8:
            ref = "lighter";
            break;
        case 9:
            ref = "copy";
            break;
        case 10:
            ref = "xor";
            break;
        case 11:
            ref = "multiply";
            break;
        case 12:
            ref = "screen";
            break;
        case 13:
            ref = "overlay";
            break;
        case 14:
            ref = "darken";
            break;
        case 15
            ref = "lighten";
            break;
        case 16:
            ref = "color-dodge";
            break;
        case 17:
            ref = "color-burn";
            break;
        case 18:
            ref = "hard-light";
            break;
        case 19:
            ref = "soft-light";
            break;
        case 20:
            ref = "difference";
            break;
        case 21:
            ref = "exclusion";
            break;
        case 22:
            ref = "hue";
            break;
        case 23:
            ref = "saturation";
            break;
        case 24:
            ref = "color";
            break;
        case 25:
            ref = "luminosity";
            break;
    }
    info.GetReturnValue().Set(
            Helpers::ConvertToV8String(isolate, ret)
    );
}

void CanvasRenderingContext2DImpl::SetGlobalCompositeOperation(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                               const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsString())) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        auto str = value->ToString(context).ToLocalChecked();
        auto composite = tolower(Helpers::ConvertFromV8String(isolate, str));
    }
}

void CanvasRenderingContext2DImpl::GetFillStyle(v8::Local<v8::String> name,
                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(info.Holder());
    auto style = canvas_native_context_get_fill_style(ptr);
    auto type = canvas_native_context_get_style_type(style);
    switch (type) {
        case 0:
            auto color = canvas_native_paint_style_get_color_string(style);
            std::string val(color);
            info.GetReturnValue().Set(Helpers::ConvertToV8String(val));
            canvas_native_destroy_string(color);
            break;
        case 1:
            info.GetReturnValue().Set(CanvasGradient::Create(context, style));
            break;
        case 2:
            info.GetReturnValue().Set(CanvasPattern::Create(context, style));
            break;
    }
}

void CanvasRenderingContext2DImpl::SetFillStyle(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                const v8::PropertyCallbackInfo<void> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(info.Holder());
    if (value->IsString())) {
        auto color = Helpers::ConvertFromV8String(isolate, value->ToString(context).ToLocalChecked());
        canvas_native_paint_style_set_fill_color_with_string(ptr, color.c_str());
    }else if (!value->IsNullOrUndefined() && value->IsObject()) {
        auto global = context->Global();
        auto pattern = global->Get(context, Helpers::ConvertToV8String(isolate, "CanvasPattern")).ToLocalChecked();
        auto gradient = global->Get(context, Helpers::ConvertToV8String(isolate, "CanvasGradient")).ToLocalChecked();
        auto color = value.As<v8::Object>();
        if (color->InstanceOf(context, pattern)) {
            auto pattern_ptr = color->GetInternalField(0).As<v8::External>()->Value();
            if (pattern_ptr != nullptr) {
                canvas_native_context_set_fill_style(ptr, static_cast<intptr_t>(static_cast<size_t>(pattern_ptr)))
            }
        }

        if (color->InstanceOf(context, gradient)) {
            auto gradient_ptr = color->GetInternalField(0).As<v8::External>()->Value();
            if (gradient_ptr != nullptr) {
                canvas_native_context_set_fill_style(ptr, static_cast<intptr_t>(static_cast<size_t>(gradient_ptr)))
            }
        }
    }
}

void CanvasRenderingContext2DImpl::GetStrokeStyle(v8::Local<v8::String> name,
                                                  const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(info.Holder());
    auto style = canvas_native_context_get_stroke_style(ptr);
    auto type = canvas_native_context_get_style_type(style);
    switch (type) {
        case 0:
            auto color = canvas_native_paint_style_get_color_string(ptr);
            std::string val(color);
            info.GetReturnValue().Set(Helpers::ConvertToV8String(val));
            canvas_native_destroy_string(color);
            break;
        case 1:
            info.GetReturnValue().Set(CanvasGradient::Create(context, style));
            break;
        case 2:
            info.GetReturnValue().Set(CanvasPattern::Create(context, style));
            break;
    }
}

void CanvasRenderingContext2DImpl::SetStrokeStyle(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                  const v8::PropertyCallbackInfo<void> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(info.Holder());
    if (value->IsString())) {
        auto color = Helpers::ConvertFromV8String(isolate, value->ToString(context).ToLocalChecked());
        canvas_native_paint_style_set_stroke_color_with_string(ptr, color.c_str());
    }else if (!value->IsNullOrUndefined() && value->IsObject()) {
        auto global = context->Global();
        auto pattern = global->Get(context, Helpers::ConvertToV8String(isolate, "CanvasPattern")).ToLocalChecked();
        auto gradient = global->Get(context, Helpers::ConvertToV8String(isolate, "CanvasGradient")).ToLocalChecked();
        auto color = value.As<v8::Object>();
        if (color->InstanceOf(context, pattern)) {
            auto pattern_ptr = color->GetInternalField(0).As<v8::External>()->Value();
            if (pattern_ptr != nullptr) {
                canvas_native_context_set_stroke_style(ptr, static_cast<intptr_t>(static_cast<size_t>(pattern_ptr)))
            }
        }

        if (color->InstanceOf(context, gradient)) {
            auto gradient_ptr = color->GetInternalField(0).As<v8::External>()->Value();
            if (gradient_ptr != nullptr) {
                canvas_native_context_set_stroke_style(ptr, static_cast<intptr_t>(static_cast<size_t>(gradient_ptr)))
            }
        }
    }
}


void CanvasRenderingContext2DImpl::GetLineWidth(v8::Local<v8::String> name,
                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_context_get_line_width(ptr));
    info.GetReturnValue().Set(ret);
}

void CanvasRenderingContext2DImpl::SetLineWidth(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber())) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_context_set_line_width(ptr, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}


void CanvasRenderingContext2DImpl::GetLineDashOffset(v8::Local<v8::String> name,
                                                     const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_context_get_line_dash_offset(ptr));
    info.GetReturnValue().Set(ret);
}

void CanvasRenderingContext2DImpl::SetLineDashOffset(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                     const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber())) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_context_set_line_dash_offset(ptr, static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}


void CanvasRenderingContext2DImpl::GetLineDash(v8::Local<v8::String> name,
                                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(info.Holder());
    auto ret = canvas_native_context_get_line_dash(ptr);
    auto data = canvas_native_get_f32_array_data(ret);
    auto data_len = canvas_native_get_f32_array_data_len(ret);
    auto array = v8::Array::New(isolate, data_len);
    auto size = sizeof(float);
    for (int i = 0; i < data_len; ++i) {
        auto item = data[i * size];
        array->Set(context, i, v8::Number::New(isolate, static_cast<double>(item)));
    }
    canvas_native_destroy_f32_array(ret);
    info.GetReturnValue().Set(array);
}

void CanvasRenderingContext2DImpl::SetLineDash(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                               const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsArray())) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto array = value.As<v8::Array>();
        auto length = array->Length();
        auto size = sizeof(float);
        auto buf_size = length * size;
        float buf[buf_size];
        for (int i = 0; i < length; i++) {
            auto item = array->Get(context, i).ToLocalChecked();
            auto offset = static_cast<float>(item->NumberValue(context).ToChecked());
            buf[i * size] = offset;
        }
        auto ptr = GetPointer(info.Holder());
        canvas_native_context_set_line_dash(ptr, &buf, buf_size);
    }
}

void CanvasRenderingContext2DImpl::AddHitRegion(const v8::FunctionCallbackInfo<v8::Value> &args) {
    // noop .. todo ?
}

void CanvasRenderingContext2DImpl::Arc(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    auto anti_clockwise = false;
    if (args.Length() == 6) {
        anti_clockwise = args[5]->BooleanValue(isolate);
    }
    canvas_native_context_arc(
            ptr,
            static_cast<float>(args[0]->ToNumber(context).ToLocalChecked()),
            static_cast<float>(args[1]->ToNumber(context).ToLocalChecked()),
            static_cast<float>(args[2]->ToNumber(context).ToLocalChecked()),
            static_cast<float>(args[3]->ToNumber(context).ToLocalChecked()),
            static_cast<float>(args[4]->ToNumber(context).ToLocalChecked()),
            anti_clockwise
    );
}

void CanvasRenderingContext2DImpl::ArcTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());

    canvas_native_context_arc_to(
            ptr,
            static_cast<float>(args[0]->ToNumber(context).ToLocalChecked()),
            static_cast<float>(args[1]->ToNumber(context).ToLocalChecked()),
            static_cast<float>(args[2]->ToNumber(context).ToLocalChecked()),
            static_cast<float>(args[3]->ToNumber(context).ToLocalChecked()),
            static_cast<float>(args[4]->ToNumber(context).ToLocalChecked()),
    );
}

void CanvasRenderingContext2DImpl::BeginPath(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.Holder());
    canvas_native_context_begin_path(ptr);
}

void CanvasRenderingContext2DImpl::BezierCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.Holder());
    canvas_native_context_bezier_curve_to(
            ptr,
            static_cast<float>(args[0]->ToNumber(context).ToLocalChecked()),
            static_cast<float>(args[1]->ToNumber(context).ToLocalChecked()),
            static_cast<float>(args[2]->ToNumber(context).ToLocalChecked()),
            static_cast<float>(args[3]->ToNumber(context).ToLocalChecked()),
            static_cast<float>(args[4]->ToNumber(context).ToLocalChecked()),
            static_cast<float>(args[5]->ToNumber(context).ToLocalChecked()),
    );
}

void CanvasRenderingContext2DImpl::ClearHitRegions(const v8::FunctionCallbackInfo<v8::Value> &args) {
// noop
}

void CanvasRenderingContext2DImpl::ClearRect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.Holder());
    canvas_native_context_clear_rect(
            ptr,
            static_cast<float>(args[0]->ToNumber(context).ToLocalChecked()),
            static_cast<float>(args[1]->ToNumber(context).ToLocalChecked()),
            static_cast<float>(args[2]->ToNumber(context).ToLocalChecked()),
            static_cast<float>(args[3]->ToNumber(context).ToLocalChecked()),
    );
}

void CanvasRenderingContext2DImpl::Clip(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 0) {
        canvas_native_context_clip_rule(ptr, 0);
    } else if (args[0]->IsString()) {
        // TODO throw for invalid enum
        auto value = Helpers::ConvertFromV8String(args[0]->ToString(context).ToLocalChecked());
        switch (value) {
            case "nonzero":
                canvas_native_context_clip_rule(ptr, 0);
                break;
            case "evenodd":
                canvas_native_context_clip_rule(ptr, 1)
                break;
            default:
                // throw
                break;
        }
    }
}

void CanvasRenderingContext2DImpl::ClosePath(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.Holder());
    canvas_native_context_close_path(ptr);
}

void CanvasRenderingContext2DImpl::CreateImageData(const v8::FunctionCallbackInfo<v8::Value> &args) {

}

CanvasRenderingContext2DImpl::~CanvasRenderingContext2DImpl() {}

