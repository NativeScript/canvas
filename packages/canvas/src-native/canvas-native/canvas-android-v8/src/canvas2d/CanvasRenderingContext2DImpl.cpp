//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasRenderingContext2DImpl.h"

CanvasRenderingContext2DImpl::CanvasRenderingContext2DImpl(rust::Box<CanvasRenderingContext2D> context) : context_(
        std::move(context)) {

}

CanvasRenderingContext2DImpl::~CanvasRenderingContext2DImpl() {
    auto raf = this->raf_.get();
    if (raf != nullptr) {
        canvas_native_raf_stop(raf->GetRaf());
    }
}

void CanvasRenderingContext2DImpl::UpdateInvalidateState() {
    auto raf = this->raf_.get();
    if (raf != nullptr) {
        if (!canvas_native_raf_get_started(raf->GetRaf())) {
            canvas_native_raf_start(raf->GetRaf());
        }
    }

    this->invalidateState_ = InvalidateState::PENDING;
}


InvalidateState CanvasRenderingContext2DImpl::GetInvalidateState() const {
    return this->invalidateState_;
}

void CanvasRenderingContext2DImpl::SetInvalidateState(InvalidateState state) {
    this->invalidateState_ = state;
}

void CanvasRenderingContext2DImpl::Flush() {
    if (this->GetInvalidateState() == InvalidateState::PENDING) {
        canvas_native_context_flush(*this->context_);
        this->SetInvalidateState(InvalidateState::INVALIDATING);
        auto current = canvas_native_context_gl_make_current(*this->context_);
        auto swapped = canvas_native_context_gl_swap_buffers(*this->context_);
        this->SetInvalidateState(InvalidateState::NONE);
    }
}

void CanvasRenderingContext2DImpl::Flush(intptr_t context) {
    auto ctx = reinterpret_cast<CanvasRenderingContext2DImpl *>(reinterpret_cast<intptr_t *>(context));
    if (ctx != nullptr) {
        ctx->Flush();
    }
}

void CanvasRenderingContext2DImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto ctorFunc = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "CanvasRenderingContext2D"),
                ctorFunc->GetFunction(context).ToLocalChecked());
    auto funcTpl = v8::FunctionTemplate::New(isolate, &InstanceFromPointer);
    global->Set(context, Helpers::ConvertToV8String(isolate, "__getCanvasRenderingContext2DImpl"),
                funcTpl->GetFunction(context).ToLocalChecked());
}

void CanvasRenderingContext2DImpl::InstanceFromPointer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    // sent as a string until we can get the java long from the js object
    if (args.Length() > 0 && args[0]->IsString()) {
        auto ptr_str = Helpers::ConvertFromV8String(isolate, args[0]->ToString(context).ToLocalChecked());
        auto ptr = std::atoll(ptr_str.c_str());
        if (ptr == 0) {
            args.GetReturnValue().Set(v8::Undefined(isolate));
        } else {
            auto cache = Caches::Get(isolate);
            auto ctx = canvas_native_context_create_with_wrapper(ptr);
            CanvasRenderingContext2DImpl *renderingContext = new CanvasRenderingContext2DImpl(std::move(ctx));
            auto ctx_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(renderingContext));
            auto raf_callback = new OnRafCallback(ctx_ptr, 0);
            auto raf_callback_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(raf_callback));
            auto raf = canvas_native_raf_create(raf_callback_ptr);
            renderingContext->raf_ = std::make_shared<RafImpl>(raf_callback, raf_callback_ptr, std::move(raf));

            auto ret = GetCtor(isolate)->InstanceTemplate()->NewInstance(context).ToLocalChecked();
            Helpers::SetInternalClassName(isolate, ret, "CanvasRenderingContext2D");
            auto ext = v8::External::New(isolate, renderingContext);
            ret->SetInternalField(0, ext);
            args.GetReturnValue().Set(ret);
        }
        return;
    }
    args.GetReturnValue().Set(v8::Undefined(isolate));
}

v8::Local<v8::Object>
CanvasRenderingContext2DImpl::NewInstance(v8::Isolate *isolate, rust::Box<CanvasRenderingContext2D> ctx) {
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    CanvasRenderingContext2DImpl *renderingContext = new CanvasRenderingContext2DImpl(std::move(ctx));
    auto ret = GetCtor(isolate)->InstanceTemplate()->NewInstance(context).ToLocalChecked();
    Helpers::SetInternalClassName(isolate, ret, "CanvasRenderingContext2D");
    auto ext = v8::External::New(isolate, renderingContext);
    ret->SetInternalField(0, ext);
    return handle_scope.Escape(ret);
}

void CanvasRenderingContext2DImpl::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Helpers::ThrowIllegalConstructor(args.GetIsolate());
}

CanvasRenderingContext2DImpl *CanvasRenderingContext2DImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<CanvasRenderingContext2DImpl *>(ptr);
}

void
CanvasRenderingContext2DImpl::GetGlobalAlpha(v8::Local<v8::String> name,
                                             const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto alpha = canvas_native_context_get_global_alpha(*ptr->context_);
    info.GetReturnValue().Set(static_cast<double>(alpha));
}

void CanvasRenderingContext2DImpl::SetGlobalAlpha(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                  const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto ptr = GetPointer(info.Holder());
        auto alpha = value->NumberValue(isolate->GetCurrentContext()).ToChecked();
        canvas_native_context_set_global_alpha(*ptr->context_, static_cast<float>(alpha));
    }
}

void CanvasRenderingContext2DImpl::GetImageSmoothingEnabled(v8::Local<v8::String> name,
                                                            const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    info.GetReturnValue().Set(canvas_native_context_get_image_smoothing_enabled(*ptr->context_));
}

void CanvasRenderingContext2DImpl::SetImageSmoothingEnabled(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                            const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsBoolean()) {
        auto isolate = info.GetIsolate();
        auto ptr = GetPointer(info.Holder());
        canvas_native_context_set_image_smoothing_enabled(*ptr->context_, value->BooleanValue(isolate));
    }
}

void CanvasRenderingContext2DImpl::GetImageSmoothingQuality(v8::Local<v8::String> name,
                                                            const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto quality = canvas_native_context_get_image_smoothing_quality(*ptr->context_);
    info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, quality.c_str()));
}

void CanvasRenderingContext2DImpl::SetImageSmoothingQuality(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                            const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsString()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        auto quality = Helpers::ConvertFromV8String(isolate,
                                                    value->ToString(context).ToLocalChecked());

        canvas_native_context_set_image_smoothing_quality(*ptr->context_, quality);
    }
}


void CanvasRenderingContext2DImpl::GetLineJoin(v8::Local<v8::String> name,
                                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto join = canvas_native_context_get_line_join(*ptr->context_);
    info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, join.c_str()));
}

void CanvasRenderingContext2DImpl::SetLineJoin(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                               const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsString()) {
        auto isolate = info.GetIsolate();
        auto ptr = GetPointer(info.Holder());
        auto join = Helpers::ConvertFromV8String(isolate,
                                                 value->ToString(isolate->GetCurrentContext()).ToLocalChecked());
        canvas_native_context_set_line_join(*ptr->context_, join);
    }
}


void CanvasRenderingContext2DImpl::GetLineCap(v8::Local<v8::String> name,
                                              const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto cap = canvas_native_context_get_line_cap(*ptr->context_);
    info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, cap.c_str()));
}

void CanvasRenderingContext2DImpl::SetLineCap(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                              const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsString()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());

        auto cap = Helpers::ConvertFromV8String(isolate, value->ToString(context).ToLocalChecked());

        canvas_native_context_set_line_join(*ptr->context_, cap);
    }
}

void CanvasRenderingContext2DImpl::GetMiterLimit(v8::Local<v8::String> name,
                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_context_get_miter_limit(*ptr->context_));
    info.GetReturnValue().Set(ret);
}

void CanvasRenderingContext2DImpl::SetMiterLimit(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                 const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_context_set_miter_limit(*ptr->context_,
                                              static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void CanvasRenderingContext2DImpl::GetShadowColor(v8::Local<v8::String> name,
                                                  const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto color = canvas_native_context_get_shadow_color(*ptr->context_);
    info.GetReturnValue().Set(
            Helpers::ConvertToV8String(
                    isolate, color.c_str()
            )
    );
}

void CanvasRenderingContext2DImpl::SetShadowColor(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                  const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsString()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());

        v8::String::Utf8Value val(isolate, value->ToString(context).ToLocalChecked());
        rust::Str color(*val, val.length());

        canvas_native_context_set_shadow_color(*ptr->context_, color);
    }
}

void CanvasRenderingContext2DImpl::GetShadowBlur(v8::Local<v8::String> name,
                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_context_get_shadow_blur(*ptr->context_));
    info.GetReturnValue().Set(ret);
}

void CanvasRenderingContext2DImpl::SetShadowBlur(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                 const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_context_set_shadow_blur(*ptr->context_,
                                              static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void CanvasRenderingContext2DImpl::GetShadowOffsetX(v8::Local<v8::String> name,
                                                    const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_context_get_shadow_offset_x(*ptr->context_));
    info.GetReturnValue().Set(ret);
}

void CanvasRenderingContext2DImpl::SetShadowOffsetX(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                    const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_context_set_shadow_offset_x(*ptr->context_,
                                                  static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void CanvasRenderingContext2DImpl::GetShadowOffsetY(v8::Local<v8::String> name,
                                                    const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_context_get_shadow_offset_y(*ptr->context_));
    info.GetReturnValue().Set(ret);
}

void CanvasRenderingContext2DImpl::SetShadowOffsetY(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                    const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_context_set_shadow_offset_y(*ptr->context_,
                                                  static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}

void CanvasRenderingContext2DImpl::GetTextAlign(v8::Local<v8::String> name,
                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto ret = canvas_native_context_get_text_align(*ptr->context_);
    info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, ret.c_str()));
}

void CanvasRenderingContext2DImpl::SetTextAlign(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsString()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        auto alignment = Helpers::ConvertFromV8String(isolate, value->ToString(context).ToLocalChecked());
        canvas_native_context_set_text_align(*ptr->context_, alignment);
    }
}

void CanvasRenderingContext2DImpl::GetGlobalCompositeOperation(v8::Local<v8::String> name,
                                                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.Holder());
    auto ret = canvas_native_context_get_global_composition(*ptr->context_);
    info.GetReturnValue().Set(
            Helpers::ConvertToV8String(isolate, ret.c_str())
    );
}

void CanvasRenderingContext2DImpl::SetGlobalCompositeOperation(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                               const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsString()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        auto str = value->ToString(context).ToLocalChecked();
        v8::String::Utf8Value val(isolate, str);
        rust::Str operation(*val, val.length());
        canvas_native_context_set_global_composition(*ptr->context_, operation);
    }
}

void CanvasRenderingContext2DImpl::GetFillStyle(v8::Local<v8::String> name,
                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(info.Holder());
    auto style = canvas_native_context_get_fill_style(*ptr->context_);
    PaintStyleType type = canvas_native_context_get_style_type(*style);

    switch (type) {
        case PaintStyleType::Color: {
            auto color = canvas_native_paint_style_get_color_string(*style);
            info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, color.c_str()));
            break;
        }
        case PaintStyleType::Gradient: {
            info.GetReturnValue().Set(CanvasGradient::NewInstance(isolate, std::move(style)));
            break;
        }
        case PaintStyleType::Pattern: {
            info.GetReturnValue().Set(CanvasPattern::NewInstance(isolate, std::move(style)));
            break;
        }
        case PaintStyleType::None: {
            // should not reach here .... throw ? run for your life
            info.GetReturnValue().Set(v8::Undefined(isolate));
            break;
        }
    }
}

void CanvasRenderingContext2DImpl::SetFillStyle(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                const v8::PropertyCallbackInfo<void> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(info.Holder());
    if (value->IsString()) {
        v8::String::Utf8Value utf8(isolate, value->ToString(context).ToLocalChecked());
        rust::Str color(*utf8, utf8.length());
        canvas_native_paint_style_set_fill_color_with_string(*ptr->context_, color);
    } else if (!value->IsNullOrUndefined() && value->IsObject()) {
        auto color = value.As<v8::Object>();
        if (Helpers::IsInstanceOf(isolate, color, "CanvasPattern")) {
            auto pattern_ptr = CanvasPattern::GetPointer(color);
            if (pattern_ptr != nullptr) {
                canvas_native_context_set_fill_style(*ptr->context_, pattern_ptr->GetPaintStyle());
            }
        }

        if (Helpers::IsInstanceOf(isolate, color, "CanvasGradient")) {
            auto gradient_ptr = CanvasGradient::GetPointer(color);
            if (gradient_ptr != nullptr) {
                canvas_native_context_set_fill_style(*ptr->context_, gradient_ptr->GetPaintStyle());
            }
        }
    }
}

void CanvasRenderingContext2DImpl::GetStrokeStyle(v8::Local<v8::String> name,
                                                  const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(info.Holder());
    auto style = canvas_native_context_get_stroke_style(*ptr->context_);
    PaintStyleType type = canvas_native_context_get_style_type(*style);
    switch (type) {
        case PaintStyleType::Color: {
            auto color = canvas_native_paint_style_get_color_string(*style);
            info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, color.c_str()));
            break;
        }
        case PaintStyleType::Gradient: {
            info.GetReturnValue().Set(CanvasGradient::NewInstance(isolate, std::move(style)));
            break;
        }
        case PaintStyleType::Pattern: {
            info.GetReturnValue().Set(CanvasPattern::NewInstance(isolate, std::move(style)));
            break;
        }
        case PaintStyleType::None: {
            // should not reach here .... throw ? run for your life
            info.GetReturnValue().Set(v8::Undefined(isolate));
            break;
        }
    }
}

void CanvasRenderingContext2DImpl::SetStrokeStyle(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                  const v8::PropertyCallbackInfo<void> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(info.Holder());
    if (value->IsString()) {
        v8::String::Utf8Value utf8(isolate, value->ToString(context).ToLocalChecked());
        rust::Str color(*utf8, utf8.length());
        canvas_native_paint_style_set_stroke_color_with_string(*ptr->context_, color);
    } else if (!value->IsNullOrUndefined() && value->IsObject()) {
        auto color = value.As<v8::Object>();
        if (Helpers::IsInstanceOf(isolate, color, "CanvasPattern")) {
            auto pattern_ptr = CanvasPattern::GetPointer(color);
            if (pattern_ptr != nullptr) {
                canvas_native_context_set_stroke_style(*ptr->context_, pattern_ptr->GetPaintStyle());
            }
        }

        if (Helpers::IsInstanceOf(isolate, color, "CanvasGradient")) {
            auto gradient_ptr = CanvasGradient::GetPointer(color);
            if (gradient_ptr != nullptr) {
                canvas_native_context_set_stroke_style(*ptr->context_, gradient_ptr->GetPaintStyle());
            }
        }
    }
}


void CanvasRenderingContext2DImpl::GetLineWidth(v8::Local<v8::String> name,
                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_context_get_line_width(*ptr->context_));
    info.GetReturnValue().Set(ret);
}

void CanvasRenderingContext2DImpl::SetLineWidth(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_context_set_line_width(*ptr->context_,
                                             static_cast<float>(value->NumberValue(context).ToChecked()));
    }
}


void CanvasRenderingContext2DImpl::GetLineDashOffset(v8::Local<v8::String> name,
                                                     const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    auto ret = static_cast<double>(canvas_native_context_get_line_dash_offset(*ptr->context_));
    info.GetReturnValue().Set(ret);
}

void CanvasRenderingContext2DImpl::SetLineDashOffset(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                                     const v8::PropertyCallbackInfo<void> &info) {
    if (value->IsNumber()) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.Holder());
        canvas_native_context_set_line_dash_offset(*ptr->context_,
                                                   static_cast<float>(value->NumberValue(context).ToChecked()));
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
            *ptr->context_,
            static_cast<float>(args[0]->NumberValue(context).ToChecked()),
            static_cast<float>(args[1]->NumberValue(context).ToChecked()),
            static_cast<float>(args[2]->NumberValue(context).ToChecked()),
            static_cast<float>(args[3]->NumberValue(context).ToChecked()),
            static_cast<float>(args[4]->NumberValue(context).ToChecked()),
            anti_clockwise
    );
}

void CanvasRenderingContext2DImpl::ArcTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());

    auto x1 = static_cast<float>(args[0]->NumberValue(context).ToChecked());
    auto y1 = static_cast<float>(args[1]->NumberValue(context).ToChecked());
    auto x2 = static_cast<float>(args[2]->NumberValue(context).ToChecked());
    auto y2 = static_cast<float>(args[3]->NumberValue(context).ToChecked());
    auto radius = static_cast<float>(args[4]->NumberValue(context).ToChecked());
    canvas_native_context_arc_to(
            *ptr->context_,
            x1,
            y1,
            x2,
            y2,
            radius
    );
}

void CanvasRenderingContext2DImpl::BeginPath(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.Holder());
    canvas_native_context_begin_path(*ptr->context_);
}

void CanvasRenderingContext2DImpl::BezierCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.Holder());
    auto context = args.GetIsolate()->GetCurrentContext();
    canvas_native_context_bezier_curve_to(
            *ptr->context_,
            static_cast<float>(args[0]->NumberValue(context).ToChecked()),
            static_cast<float>(args[1]->NumberValue(context).ToChecked()),
            static_cast<float>(args[2]->NumberValue(context).ToChecked()),
            static_cast<float>(args[3]->NumberValue(context).ToChecked()),
            static_cast<float>(args[4]->NumberValue(context).ToChecked()),
            static_cast<float>(args[5]->NumberValue(context).ToChecked())
    );
}

void CanvasRenderingContext2DImpl::ClearHitRegions(const v8::FunctionCallbackInfo<v8::Value> &args) {
// noop
}

void CanvasRenderingContext2DImpl::ClearRect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.Holder());
    auto context = args.GetIsolate()->GetCurrentContext();
    canvas_native_context_clear_rect(
            *ptr->context_,
            static_cast<float>(args[0]->NumberValue(context).ToChecked()),
            static_cast<float>(args[1]->NumberValue(context).ToChecked()),
            static_cast<float>(args[2]->NumberValue(context).ToChecked()),
            static_cast<float>(args[3]->NumberValue(context).ToChecked())
    );
    ptr->UpdateInvalidateState();
}

void CanvasRenderingContext2DImpl::Clip(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 0) {
        canvas_native_context_clip_rule(*ptr->context_, "nonzero");
    } else if (args[0]->IsString()) {
        // TODO throw for invalid enum
        auto value = Helpers::ConvertFromV8String(isolate, args[0]->ToString(context).ToLocalChecked());
        canvas_native_context_clip_rule(*ptr->context_, value);
    }
}

void CanvasRenderingContext2DImpl::ClosePath(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.Holder());
    canvas_native_context_close_path(*ptr->context_);
}

void CanvasRenderingContext2DImpl::CreateImageData(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 1 && args[0]->IsObject()) {

        auto object = args[0]->ToObject(context).ToLocalChecked();

        if (Helpers::IsInstanceOf(isolate, object, "ImageData")) {
            ImageDataImpl *data = ImageDataImpl::GetPointer(object);
            ImageDataImpl *cptr = new ImageDataImpl(
                    canvas_native_image_data_get_shared_instance(data->GetImageData()));
            auto ret = ImageDataImpl::NewInstance(isolate, cptr);
            args.GetReturnValue().Set(ret);
        }


    } else if (args.Length() > 1) {

        ImageDataImpl *imageData = new ImageDataImpl(std::move(canvas_native_image_data_create(
                args[0]->Int32Value(context).ToChecked(),
                args[1]->Int32Value(context).ToChecked()
        )));

        auto ret = ImageDataImpl::NewInstance(isolate, imageData);

        args.GetReturnValue().Set(ret);
    }
}


void CanvasRenderingContext2DImpl::CreateLinearGradient(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 4) {
        auto x0 = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y0 = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto x1 = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto y1 = static_cast<float>(args[3]->NumberValue(context).ToChecked());

        auto gradient = canvas_native_context_create_linear_gradient(*ptr->context_, x0, y0, x1, y1);
        args.GetReturnValue().Set(CanvasGradient::NewInstance(isolate, std::move(gradient)));
    }
}

void CanvasRenderingContext2DImpl::CreatePattern(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 2) {
        auto image = args[0]->ToObject(context).ToLocalChecked();
        // TODO handle other cases
        if (Helpers::IsInstanceOf(isolate, image, "ImageAsset")) {
            auto asset = ImageAssetImpl::GetPointer(image);
            auto rep = Helpers::ConvertFromV8String(isolate, args[1]->ToString(context).ToLocalChecked());
            rust::Box<PaintStyle> pattern = canvas_native_context_create_pattern_asset(*ptr->context_,
                                                                                       asset->GetImageAsset(), rep);
            auto type = canvas_native_context_get_style_type(*pattern);
            if (type == PaintStyleType::None) {
                args.GetReturnValue().Set(v8::Undefined(isolate));
            } else {
                args.GetReturnValue().Set(CanvasPattern::NewInstance(isolate, std::move(pattern)));
            }
            return;
        }

        args.GetReturnValue().Set(v8::Undefined(isolate));
    } else {
        args.GetReturnValue().Set(v8::Undefined(isolate));
    }
}

void CanvasRenderingContext2DImpl::CreateRadialGradient(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 6) {
        auto x0 = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y0 = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto r0 = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto x1 = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto y1 = static_cast<float>(args[4]->NumberValue(context).ToChecked());
        auto r1 = static_cast<float>(args[5]->NumberValue(context).ToChecked());

        auto gradient = canvas_native_context_create_radial_gradient(*ptr->context_, x0, y0, r0, x1, y1, r1);
        args.GetReturnValue().Set(CanvasGradient::NewInstance(isolate, std::move(gradient)));
    }
}

void CanvasRenderingContext2DImpl::DrawFocusIfNeeded(const v8::FunctionCallbackInfo<v8::Value> &args) {
    // NOOP
}


void CanvasRenderingContext2DImpl::DrawImage(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    // TODO handle other cases
    if (args.Length() == 3) {
        auto image = args[0].As<v8::Object>();
        auto dx = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto dy = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        if (Helpers::IsInstanceOf(isolate, image, "ImageAsset")) {
            auto asset = ImageAssetImpl::GetPointer(image);
            canvas_native_context_draw_image_dx_dy_asset(*ptr->context_, asset->GetImageAsset(), dx, dy);
            ptr->UpdateInvalidateState();
        }
    } else if (args.Length() == 5) {
        auto image = args[0]->ToObject(context).ToLocalChecked();
        auto dx = args[1]->NumberValue(context).ToChecked();
        auto dy = args[2]->NumberValue(context).ToChecked();
        auto dWidth = args[3]->NumberValue(context).ToChecked();
        auto dHeight = args[4]->NumberValue(context).ToChecked();
        if (Helpers::IsInstanceOf(isolate, image, "ImageAsset")) {
            auto asset = ImageAssetImpl::GetPointer(image);
            canvas_native_context_draw_image_dx_dy_dw_dh_asset(*ptr->context_, asset->GetImageAsset(), dx, dy, dWidth,
                                                               dHeight);
            ptr->UpdateInvalidateState();
        }
    } else if (args.Length() == 9) {
        auto image = args[0]->ToObject(context).ToLocalChecked();
        auto sx = args[1]->NumberValue(context).ToChecked();
        auto sy = args[2]->NumberValue(context).ToChecked();
        auto sWidth = args[3]->NumberValue(context).ToChecked();
        auto sHeight = args[4]->NumberValue(context).ToChecked();
        auto dx = args[5]->NumberValue(context).ToChecked();
        auto dy = args[6]->NumberValue(context).ToChecked();
        auto dWidth = args[7]->NumberValue(context).ToChecked();
        auto dHeight = args[8]->NumberValue(context).ToChecked();
        if (Helpers::IsInstanceOf(isolate, image, "ImageAsset")) {
            auto asset = ImageAssetImpl::GetPointer(image);
            canvas_native_context_draw_image_asset(*ptr->context_, asset->GetImageAsset(), sx, sy, sWidth, sHeight, dx,
                                                   dy, dWidth, dHeight);
            ptr->UpdateInvalidateState();
        }
    }
}


void CanvasRenderingContext2DImpl::Ellipse(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 8) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto radiusX = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto radiusY = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto rotation = static_cast<float>(args[4]->NumberValue(context).ToChecked());
        auto startAngle = static_cast<float>(args[5]->NumberValue(context).ToChecked());
        auto endAngle = static_cast<float>(args[6]->NumberValue(context).ToChecked());
        auto anticlockwise = false;
        if (args[7]->IsBoolean()) {
            anticlockwise = args[7]->BooleanValue(isolate);
        }

        canvas_native_context_ellipse(*ptr->context_, x, y, radiusX, radiusY, rotation, startAngle, endAngle,
                                      anticlockwise);
    }
}


void CanvasRenderingContext2DImpl::Fill(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 2) {
        auto object = args[0]->ToObject(context).ToLocalChecked();
        auto rule = args[1]->ToString(context).ToLocalChecked();
        if (Helpers::IsInstanceOf(isolate, object, "Path2D")) {
            auto path = Path2D::GetPointer(object);
            canvas_native_context_fill_with_path(*ptr->context_, path->GetPath(),
                                                 Helpers::ConvertFromV8String(isolate, rule));
            ptr->UpdateInvalidateState();
        }
    } else if (args.Length() == 1) {
        if (args[0]->IsString()) {
            canvas_native_context_fill(*ptr->context_, Helpers::ConvertFromV8String(isolate, args[0]->ToString(
                    context).ToLocalChecked()));
            ptr->UpdateInvalidateState();
        } else if (args[0]->IsObject()) {
            auto object = args[0]->ToObject(context).ToLocalChecked();
            if (Helpers::IsInstanceOf(isolate, object, "Path2D")) {
                auto path = Path2D::GetPointer(object);
                canvas_native_context_fill_with_path(*ptr->context_, path->GetPath(), "nonzero");
                ptr->UpdateInvalidateState();
            }
        }
    } else {
        canvas_native_context_fill(*ptr->context_, "nonzero");
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::FillRect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (ptr != nullptr) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto width = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto height = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        canvas_native_context_fill_rect(*ptr->context_, x, y, width, height);
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::FillText(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() >= 3) {
        auto text = Helpers::ConvertFromV8String(isolate, args[0]->ToString(context).ToLocalChecked());
        auto x = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        float width = -1;
        if (args[3]->IsNumber()) {
            width = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        }
        canvas_native_context_fill_text(*ptr->context_, text, x, y, width);
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::GetImageData(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 4) {
        auto sx = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto sy = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto sw = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto sh = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto data = canvas_native_context_get_image_data(*ptr->context_, sx, sy, sw, sh);
        ImageDataImpl *imageData = new ImageDataImpl(std::move(data));
        auto ret = ImageDataImpl::NewInstance(isolate, imageData);

        args.GetReturnValue().Set(ret);

    } else {
        args.GetReturnValue().Set(v8::Undefined(isolate));
    }
}

void CanvasRenderingContext2DImpl::GetLineDash(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    auto dash = canvas_native_context_get_line_dash(*ptr->context_);
    auto size = dash.size();
    auto array = v8::Array::New(isolate, size);
    for (int i = 0; i < size; ++i) {
        array->Set(context, i, v8::Number::New(isolate, static_cast<double>(dash[i])));
    }
    args.GetReturnValue().Set(array);
}

void CanvasRenderingContext2DImpl::IsPointInPath(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto ret = canvas_native_context_is_point_in_path(*ptr->context_, x, y, "nonzero");
        args.GetReturnValue().Set(ret);
    } else if (args.Length() == 3 && args[2]->IsString()) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto rule = Helpers::ConvertFromV8String(isolate, args[2]->ToString(context).ToLocalChecked());
        auto ret = canvas_native_context_is_point_in_path(*ptr->context_, x, y, rule);
        args.GetReturnValue().Set(ret);
    } else if (args.Length() == 4 && args[0]->IsObject() && args[3]->IsString()) {
        auto path = args[0]->ToObject(context).ToLocalChecked();
        auto x = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto rule = Helpers::ConvertFromV8String(isolate, args[3]->ToString(context).ToLocalChecked());
        if (Helpers::IsInstanceOf(isolate, path, "Path2D")) {
            auto path_ptr = Path2D::GetPointer(path);
            auto ret = canvas_native_context_is_point_in_path_with_path(*ptr->context_, path_ptr->GetPath(), x, y,
                                                                        rule);
            args.GetReturnValue().Set(ret);
        } else {
            args.GetReturnValue().Set(false);
        }
    } else {
        // TODO other checks ?
        args.GetReturnValue().Set(false);
    }
}

void CanvasRenderingContext2DImpl::IsPointInStroke(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto ret = canvas_native_context_is_point_in_stroke(*ptr->context_, x, y);
        args.GetReturnValue().Set(ret);
    } else if (args.Length() == 3 && args[0]->IsObject()) {
        auto path = args[0]->ToObject(context).ToLocalChecked();
        auto x = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        if (Helpers::IsInstanceOf(isolate, path, "Path2D")) {
            auto path_ptr = Path2D::GetPointer(path);
            auto ret = canvas_native_context_is_point_in_stroke_with_path(*ptr->context_, path_ptr->GetPath(), x, y);
            args.GetReturnValue().Set(ret);
        } else {
            args.GetReturnValue().Set(false);
        }
    } else {
        // TODO other checks ?
        args.GetReturnValue().Set(false);
    }
}


void CanvasRenderingContext2DImpl::LineTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        canvas_native_context_line_to(*ptr->context_, x, y);
    }
}

void CanvasRenderingContext2DImpl::MeasureText(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    auto text = Helpers::ConvertFromV8String(isolate, args[0]->ToString(context).ToLocalChecked());
    auto metrics = canvas_native_context_measure_text(*ptr->context_, text);
    args.GetReturnValue().Set(TextMetricsImpl::NewInstance(isolate, std::move(metrics)));
}


void CanvasRenderingContext2DImpl::MoveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        canvas_native_context_move_to(*ptr->context_, x, y);
    }
}

void CanvasRenderingContext2DImpl::PutImageData(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    auto imageData = args[0]->ToObject(context).ToLocalChecked();
    if (args.Length() == 3) {
        auto imageDataPtr = ImageDataImpl::GetPointer(imageData);
        auto dx = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto dy = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        float dirtyX = 0;
        float dirtyY = 0;
        auto dirtyWidth = imageDataPtr->GetWidth();
        auto dirtyHeight = imageDataPtr->GetHeight();
        canvas_native_context_put_image_data(*ptr->context_, imageDataPtr->GetImageData(), dx, dy, dirtyX, dirtyY,
                                             dirtyWidth, dirtyHeight);
        ptr->UpdateInvalidateState();
    } else if (args.Length() == 7) {
        auto imageDataPtr = ImageDataImpl::GetPointer(imageData);
        auto dx = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto dy = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        float dirtyX = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        float dirtyY = static_cast<float>(args[4]->NumberValue(context).ToChecked());;
        auto dirtyWidth = static_cast<float>(args[5]->NumberValue(context).ToChecked());
        auto dirtyHeight = static_cast<float>(args[6]->NumberValue(context).ToChecked());
        canvas_native_context_put_image_data(*ptr->context_, imageDataPtr->GetImageData(), dx, dy, dirtyX, dirtyY,
                                             dirtyWidth, dirtyHeight);
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::QuadraticCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 4) {
        auto cpx = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto cpy = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto x = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        canvas_native_context_quadratic_curve_to(*ptr->context_, cpx, cpy, x, y);
    }
}

void CanvasRenderingContext2DImpl::Rect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 4) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto width = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto height = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        canvas_native_context_rect(*ptr->context_, x, y, width, height);
    }
}


void CanvasRenderingContext2DImpl::RemoveHitRegion(const v8::FunctionCallbackInfo<v8::Value> &args) {
    // NOOP
}


void CanvasRenderingContext2DImpl::ResetTransform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    canvas_native_context_reset_transform(*ptr->context_);
}


void CanvasRenderingContext2DImpl::Restore(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.Holder());
    canvas_native_context_restore(*ptr->context_);
}


void CanvasRenderingContext2DImpl::Rotate(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 1 && args[0]->IsNumber()) {
        canvas_native_context_rotate(*ptr->context_, args[0]->NumberValue(context).ToChecked());
    }
}

void CanvasRenderingContext2DImpl::Save(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.Holder());
    canvas_native_context_save(*ptr->context_);
}

void CanvasRenderingContext2DImpl::Scale(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        canvas_native_context_scale(*ptr->context_, x, y);
    }
}

void CanvasRenderingContext2DImpl::ScrollPathIntoView(const v8::FunctionCallbackInfo<v8::Value> &args) {
// NOOP
}

void CanvasRenderingContext2DImpl::SetLineDash(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 1 && args[0]->IsArray()) {
        auto segments = args[0].As<v8::Array>();
        auto len = segments->Length();
        std::vector<float> data;
        for (int i = 0; i < len; ++i) {
            data.push_back(
                    static_cast<float>(segments->Get(context, i).ToLocalChecked()->NumberValue(context).ToChecked()));
        }
        rust::Slice<const float> slice{data.data(), data.size()};
        canvas_native_context_set_line_dash(*ptr->context_, slice);
    }
}


void CanvasRenderingContext2DImpl::SetTransform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 1 && args[0]->IsObject()) {
        auto matrix = args[0]->ToObject(context).ToLocalChecked();
        if (Helpers::IsInstanceOf(isolate, matrix, "DOMMatrix")) {
            auto matrix_ptr = MatrixImpl::GetPointer(matrix);
            canvas_native_context_set_transform_matrix(*ptr->context_, matrix_ptr->GetMatrix());
        }
    } else if (args.Length() == 6) {
        auto a = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto b = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto c = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto d = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto e = static_cast<float>(args[4]->NumberValue(context).ToChecked());
        auto f = static_cast<float>(args[5]->NumberValue(context).ToChecked());
        canvas_native_context_set_transform(*ptr->context_, a, b, c, d, e, f);
    }
}


void CanvasRenderingContext2DImpl::Stroke(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 1 && args[0]->IsObject()) {
        auto path = args[0]->ToObject(context).ToLocalChecked();
        if (Helpers::IsInstanceOf(isolate, path, "Path2D")) {
            auto path_ptr = Path2D::GetPointer(path);
            canvas_native_context_stroke_with_path(*ptr->context_, path_ptr->GetPath());
            ptr->UpdateInvalidateState();
        }
    } else {
        canvas_native_context_stroke(*ptr->context_);
        ptr->UpdateInvalidateState();
    }
}


void CanvasRenderingContext2DImpl::StrokeRect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 4) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto width = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto height = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        canvas_native_context_stroke_rect(*ptr->context_, x, y, width, height);
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::StrokeText(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() >= 3) {
        auto text = Helpers::ConvertFromV8String(isolate, args[0]->ToString(context).ToLocalChecked());
        auto x = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        float maxWidth = static_cast<float>(args[3]->NumberValue(context).FromMaybe(-1));
        canvas_native_context_stroke_text(*ptr->context_, text, x, y, maxWidth);
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::Transform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 6) {
        auto a = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto b = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto c = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto d = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto e = static_cast<float>(args[4]->NumberValue(context).ToChecked());
        auto f = static_cast<float>(args[5]->NumberValue(context).ToChecked());
        canvas_native_context_transform(*ptr->context_, a, b, c, d, e, f);
    }
}

void CanvasRenderingContext2DImpl::Translate(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.Holder());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        canvas_native_context_translate(*ptr->context_, x, y);
    }
}


v8::Local<v8::FunctionTemplate> CanvasRenderingContext2DImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto tmpl = cache->CanvasRenderingContext2DTmpl.get();
    if (tmpl != nullptr) {
        return tmpl->Get(isolate);
    }

    auto context = isolate->GetCurrentContext();
    auto canvasRenderingContextFunc = v8::FunctionTemplate::New(isolate, &Create);
    canvasRenderingContextFunc->SetClassName(Helpers::ConvertToV8String(isolate, "CanvasRenderingContext2D"));
    canvasRenderingContextFunc->InstanceTemplate()->SetInternalFieldCount(1);

    auto func = canvasRenderingContextFunc->GetFunction(context).ToLocalChecked();

    auto canvasRenderingContextTpl = canvasRenderingContextFunc->InstanceTemplate();

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

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "addHitRegion"),
            v8::FunctionTemplate::New(isolate, &AddHitRegion)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "arc"),
            v8::FunctionTemplate::New(isolate, &Arc)
    );


    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "arcTo"),
            v8::FunctionTemplate::New(isolate, &ArcTo)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "beginPath"),
            v8::FunctionTemplate::New(isolate, &BeginPath)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "bezierCurveTo"),
            v8::FunctionTemplate::New(isolate, &BezierCurveTo)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "clearHitRegions"),
            v8::FunctionTemplate::New(isolate, &ClearHitRegions)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "clearRect"),
            v8::FunctionTemplate::New(isolate, &ClearRect)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "clip"),
            v8::FunctionTemplate::New(isolate, &Clip)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "closePath"),
            v8::FunctionTemplate::New(isolate, &ClosePath)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createImageData"),
            v8::FunctionTemplate::New(isolate, &CreateImageData)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createLinearGradient"),
            v8::FunctionTemplate::New(isolate, &CreateLinearGradient)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createPattern"),
            v8::FunctionTemplate::New(isolate, &CreatePattern)
    );


    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "createRadialGradient"),
            v8::FunctionTemplate::New(isolate, &CreateRadialGradient)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "drawFocusIfNeeded"),
            v8::FunctionTemplate::New(isolate, &DrawFocusIfNeeded)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "drawImage"),
            v8::FunctionTemplate::New(isolate, &DrawImage)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "ellipse"),
            v8::FunctionTemplate::New(isolate, &Ellipse)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "fill"),
            v8::FunctionTemplate::New(isolate, &Fill)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "fillRect"),
            v8::FunctionTemplate::New(isolate, &FillRect)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "fillText"),
            v8::FunctionTemplate::New(isolate, &FillText)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getImageData"),
            v8::FunctionTemplate::New(isolate, &GetImageData)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "getLineDash"),
            v8::FunctionTemplate::New(isolate, &GetLineDash)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "isPointInPath"),
            v8::FunctionTemplate::New(isolate, &IsPointInPath)
    );


    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "isPointInStroke"),
            v8::FunctionTemplate::New(isolate, &IsPointInStroke)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "lineTo"),
            v8::FunctionTemplate::New(isolate, &LineTo)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "measureText"),
            v8::FunctionTemplate::New(isolate, &MeasureText)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "moveTo"),
            v8::FunctionTemplate::New(isolate, &MoveTo)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "putImageData"),
            v8::FunctionTemplate::New(isolate, &PutImageData)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "quadraticCurveTo"),
            v8::FunctionTemplate::New(isolate, &QuadraticCurveTo)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "rect"),
            v8::FunctionTemplate::New(isolate, &Rect)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "removeHitRegion"),
            v8::FunctionTemplate::New(isolate, &RemoveHitRegion)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "resetTransform"),
            v8::FunctionTemplate::New(isolate, &ResetTransform)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "restore"),
            v8::FunctionTemplate::New(isolate, &Restore)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "rotate"),
            v8::FunctionTemplate::New(isolate, &Rotate)
    );


    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "save"),
            v8::FunctionTemplate::New(isolate, &Save)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "scale"),
            v8::FunctionTemplate::New(isolate, &Scale)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "scrollPathIntoView"),
            v8::FunctionTemplate::New(isolate, &ScrollPathIntoView)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "setLineDash"),
            v8::FunctionTemplate::New(isolate, &SetLineDash)
    );


    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "setTransform"),
            v8::FunctionTemplate::New(isolate, &SetTransform)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "stroke"),
            v8::FunctionTemplate::New(isolate, &Stroke)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "strokeRect"),
            v8::FunctionTemplate::New(isolate, &StrokeRect)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "strokeText"),
            v8::FunctionTemplate::New(isolate, &StrokeText)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "transform"),
            v8::FunctionTemplate::New(isolate, &Transform)
    );

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "translate"),
            v8::FunctionTemplate::New(isolate, &Translate)
    );

    cache->CanvasRenderingContext2DTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate,
                                                                                                 canvasRenderingContextFunc);
    return canvasRenderingContextFunc;
}
