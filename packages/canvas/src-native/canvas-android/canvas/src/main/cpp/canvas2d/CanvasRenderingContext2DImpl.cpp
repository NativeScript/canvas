//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasRenderingContext2DImpl.h"
#include "../webgl/WebGLRenderingContextBase.h"
#include "../OneByteStringResource.h"
#include "RafImpl.h"

CanvasRenderingContext2DImpl::CanvasRenderingContext2DImpl(
        rust::Box <CanvasRenderingContext2D> context) : context_(
        std::move(context)) {
    this->buf_ = new char[1024];
}

CanvasRenderingContext2DImpl::~CanvasRenderingContext2DImpl() {
    auto raf = this->raf_.get();
    if (raf != nullptr) {
        canvas_native_raf_stop(raf->GetRaf());
    }
    delete this->buf_;
    auto isolate = v8::Isolate::GetCurrent();
    if (isolate != nullptr) {
        isolate->AdjustAmountOfExternalAllocatedMemory(sizeof(this));
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
    auto ctorFunc = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    global->Set(context, Helpers::ConvertToV8String(isolate, "CanvasRenderingContext2D"),
                ctorFunc->GetFunction(context).ToLocalChecked());
    auto funcTpl = v8::FunctionTemplate::New(isolate, &InstanceFromPointer);
    global->Set(context, Helpers::ConvertToV8String(isolate, "__getCanvasRenderingContext2DImpl"),
                funcTpl->GetFunction(context).ToLocalChecked());
}

void
CanvasRenderingContext2DImpl::InstanceFromPointer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    if (args.Length() > 0) {
        auto width = args[0];
        auto height = args[1];
        auto density = args[2];
        auto fontColor = args[3];
        auto ppi = args[4];
        auto direction = args[5];
        auto alpha = args[6];
        auto offscreenValue = args[7];
        bool offscreen = false;
        if (args.Length() < 6) {
            args.GetReturnValue().SetUndefined();
            return;
        } else {
            auto cache = Caches::Get(isolate);

            if (offscreenValue->IsBoolean() || offscreenValue->IsBooleanObject()) {
                offscreen = offscreenValue->BooleanValue(isolate);
            }

            if (offscreen) {
                auto ctx = canvas_native_context_create_gl_no_window(
                        static_cast<float>(width->NumberValue(context).ToChecked()),
                        static_cast<float>(height->NumberValue(context).ToChecked()),
                        static_cast<float>(density->NumberValue(context).ToChecked()),
                        fontColor->Int32Value(context).ToChecked(),
                        static_cast<float>(ppi->NumberValue(context).ToChecked()),
                        direction->Uint32Value(context).ToChecked(),
                        alpha->BooleanValue(isolate)
                );
                auto ret = NewInstance(isolate, std::move(ctx));
                args.GetReturnValue().Set(ret);
                return;
            } else {
                auto ctx = canvas_native_context_create_with_current(
                        static_cast<float>(width->NumberValue(context).ToChecked()),
                        static_cast<float>(height->NumberValue(context).ToChecked()),
                        static_cast<float>(density->NumberValue(context).ToChecked()),
                        fontColor->Int32Value(context).ToChecked(),
                        static_cast<float>(ppi->NumberValue(context).ToChecked()),
                        direction->Uint32Value(context).ToChecked(),
                        alpha->BooleanValue(isolate)
                );
                auto ret = NewInstance(isolate, std::move(ctx));
                args.GetReturnValue().Set(ret);
                return;
            }

        }
    }
    args.GetReturnValue().SetUndefined();
}

v8::Local<v8::Object>
CanvasRenderingContext2DImpl::NewInstance(v8::Isolate *isolate,
                                          rust::Box <CanvasRenderingContext2D> ctx) {
    v8::EscapableHandleScope handle_scope(isolate);
    auto context = isolate->GetCurrentContext();
    auto *renderingContext = new CanvasRenderingContext2DImpl(std::move(ctx));


    auto ctx_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(renderingContext));
    auto raf_callback = new OnRafCallback(ctx_ptr, 0);
    auto raf_callback_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(raf_callback));
    auto raf = canvas_native_raf_create(raf_callback_ptr);
    renderingContext->raf_ = std::make_shared<RafImpl>(raf_callback, raf_callback_ptr,
                                                       std::move(raf));


    auto _raf = renderingContext->raf_.get();
    canvas_native_raf_start(_raf->GetRaf());


    auto ret = GetCtor(isolate)->InstanceTemplate()->NewInstance(context).ToLocalChecked();
    Helpers::SetInstanceType(isolate, ret, ObjectType::CanvasRenderingContext2D);
    AddWeakListener(isolate, ret, renderingContext);
    return handle_scope.Escape(ret);
}

void CanvasRenderingContext2DImpl::Resize(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto width = args[0];
    auto height = args[1];
    canvas_native_context_resize(ptr->GetContext(),
                                 static_cast<float>(width->NumberValue(context).FromMaybe(1)),
                                 static_cast<float>(width->NumberValue(context).FromMaybe(1)));

}


void CanvasRenderingContext2DImpl::Create(const v8::FunctionCallbackInfo<v8::Value> &args) {
    Helpers::ThrowIllegalConstructor(args.GetIsolate());
}

CanvasRenderingContext2DImpl *
CanvasRenderingContext2DImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    return static_cast<CanvasRenderingContext2DImpl *>(ptr);
}

void
CanvasRenderingContext2DImpl::GetFont(v8::Local<v8::String> name,
                                      const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.This());
    auto font = canvas_native_context_get_font(ptr->GetContext());
    info.GetReturnValue().Set(
            Helpers::ConvertToV8String(isolate, std::string(font.c_str(), font.size()))
    );
}

void CanvasRenderingContext2DImpl::SetFont(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                           const v8::PropertyCallbackInfo<void> &info) {
    if (Helpers::IsString(value)) {
        auto isolate = info.GetIsolate();
        auto ptr = GetPointer(info.This());
        auto val = Helpers::ConvertFromV8String(isolate, value);
        canvas_native_context_set_font(ptr->GetContext(), rust::Str(val.c_str(), val.size()));
    }
}


void
CanvasRenderingContext2DImpl::GetGlobalAlpha(v8::Local<v8::String> name,
                                             const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.This());
    auto alpha = canvas_native_context_get_global_alpha(ptr->GetContext());
    info.GetReturnValue().Set(static_cast<double>(alpha));
}

void
CanvasRenderingContext2DImpl::SetGlobalAlpha(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                             const v8::PropertyCallbackInfo<void> &info) {
    if (Helpers::IsNumber(value)) {
        auto isolate = info.GetIsolate();
        auto ptr = GetPointer(info.This());
        auto alpha = value->NumberValue(isolate->GetCurrentContext()).ToChecked();
        canvas_native_context_set_global_alpha(ptr->GetContext(), static_cast<float>(alpha));
    }
}

void CanvasRenderingContext2DImpl::GetImageSmoothingEnabled(v8::Local<v8::String> name,
                                                            const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.This());
    info.GetReturnValue().Set(canvas_native_context_get_image_smoothing_enabled(ptr->GetContext()));
}

void CanvasRenderingContext2DImpl::SetImageSmoothingEnabled(v8::Local<v8::String> name,
                                                            v8::Local<v8::Value> value,
                                                            const v8::PropertyCallbackInfo<void> &info) {
    if (Helpers::IsBoolean(value)) {
        auto isolate = info.GetIsolate();
        auto ptr = GetPointer(info.This());
        canvas_native_context_set_image_smoothing_enabled(ptr->GetContext(),
                                                          value->BooleanValue(isolate));
    }
}

void CanvasRenderingContext2DImpl::GetImageSmoothingQuality(v8::Local<v8::String> name,
                                                            const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.This());
    rust::Str quality = canvas_native_context_get_image_smoothing_quality(ptr->GetContext());
    info.GetReturnValue().Set(
            Helpers::ConvertToV8String(isolate, std::string(quality.data(), quality.size())));
}

void CanvasRenderingContext2DImpl::SetImageSmoothingQuality(v8::Local<v8::String> name,
                                                            v8::Local<v8::Value> value,
                                                            const v8::PropertyCallbackInfo<void> &info) {
    if (Helpers::IsString(value)) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.This());
        auto quality = Helpers::ConvertFromV8String(isolate,
                                                    value->ToString(context).ToLocalChecked());

        canvas_native_context_set_image_smoothing_quality(ptr->GetContext(),
                                                          rust::Str(quality.c_str(),
                                                                    quality.size()));
    }
}


void CanvasRenderingContext2DImpl::GetLineJoin(v8::Local<v8::String> name,
                                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.This());
    auto join = canvas_native_context_get_line_join(ptr->GetContext());
    info.GetReturnValue().Set(
            Helpers::ConvertToV8String(isolate, std::string(join.data(), join.size())));
}

void
CanvasRenderingContext2DImpl::SetLineJoin(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                          const v8::PropertyCallbackInfo<void> &info) {
    if (Helpers::IsString(value)) {
        auto isolate = info.GetIsolate();
        auto ptr = GetPointer(info.This());
        auto join = Helpers::ConvertFromV8String(isolate,
                                                 value->ToString(
                                                         isolate->GetCurrentContext()).ToLocalChecked());
        canvas_native_context_set_line_join(ptr->GetContext(),
                                            rust::Str(join.c_str(), join.size()));
    }
}


void CanvasRenderingContext2DImpl::GetLineCap(v8::Local<v8::String> name,
                                              const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.This());
    auto cap = canvas_native_context_get_line_cap(ptr->GetContext());
    info.GetReturnValue().Set(
            Helpers::ConvertToV8String(isolate, std::string(cap.data(), cap.size())));
}

void
CanvasRenderingContext2DImpl::SetLineCap(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                         const v8::PropertyCallbackInfo<void> &info) {
    if (Helpers::IsString(value)) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.This());

        auto cap = Helpers::ConvertFromV8String(isolate, value->ToString(context).ToLocalChecked());

        canvas_native_context_set_line_cap(ptr->GetContext(), rust::Str(cap.c_str(), cap.size()));
    }
}

void CanvasRenderingContext2DImpl::GetMiterLimit(v8::Local<v8::String> name,
                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto ret = static_cast<double>(canvas_native_context_get_miter_limit(ptr->GetContext()));
    info.GetReturnValue().Set(ret);
}

void
CanvasRenderingContext2DImpl::SetMiterLimit(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                            const v8::PropertyCallbackInfo<void> &info) {
    if (Helpers::IsNumber(value)) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.This());
        canvas_native_context_set_miter_limit(ptr->GetContext(),
                                              static_cast<float>(value->NumberValue(
                                                      context).ToChecked()));
    }
}

void CanvasRenderingContext2DImpl::GetShadowColor(v8::Local<v8::String> name,
                                                  const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.This());

    uint8_t r;
    uint8_t g;
    uint8_t b;
    uint8_t a;
//
//    canvas_native_context_get_shadow_color_rgba(ptr->GetContext(), r, g, b, a);
//
//    auto color = Helpers::RGBAToHex(r, g, b, a);
//
//    auto ret = Helpers::ConvertToV8String(isolate, color);
//
//    info.GetReturnValue().Set(ret);


    auto color = canvas_native_context_get_shadow_color_buf(ptr->GetContext());

    auto extString = new OneByteStringResource(std::move(color));

    info.GetReturnValue().Set(
            v8::String::NewExternalOneByte(isolate, extString).ToLocalChecked()
    );

}

void
CanvasRenderingContext2DImpl::SetShadowColor(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                             const v8::PropertyCallbackInfo<void> &info) {
    if (Helpers::IsString(value)) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.This());
        auto color = Helpers::ConvertFromV8String(isolate, value);

        uint8_t r;
        uint8_t g;
        uint8_t b;
        uint8_t a;

        if (canvas_native_parse_css_color_rgba(rust::Str(color.c_str(), color.size()), r, g, b,
                                               a)) {
            canvas_native_context_set_shadow_color_rgba(ptr->GetContext(), r, g, b, a);
        }
    }
}

void CanvasRenderingContext2DImpl::GetShadowBlur(v8::Local<v8::String> name,
                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto ret = static_cast<double>(canvas_native_context_get_shadow_blur(ptr->GetContext()));
    info.GetReturnValue().Set(ret);
}

void
CanvasRenderingContext2DImpl::SetShadowBlur(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                            const v8::PropertyCallbackInfo<void> &info) {
    if (Helpers::IsNumber(value)) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.This());
        canvas_native_context_set_shadow_blur(ptr->GetContext(),
                                              static_cast<float>(value->NumberValue(
                                                      context).ToChecked()));
    }
}

void CanvasRenderingContext2DImpl::GetShadowOffsetX(v8::Local<v8::String> name,
                                                    const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto ret = static_cast<double>(canvas_native_context_get_shadow_offset_x(ptr->GetContext()));
    info.GetReturnValue().Set(ret);
}

void CanvasRenderingContext2DImpl::SetShadowOffsetX(v8::Local<v8::String> name,
                                                    v8::Local<v8::Value> value,
                                                    const v8::PropertyCallbackInfo<void> &info) {
    if (Helpers::IsNumber(value)) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.This());
        canvas_native_context_set_shadow_offset_x(ptr->GetContext(),
                                                  static_cast<float>(value->NumberValue(
                                                          context).ToChecked()));
    }
}

void CanvasRenderingContext2DImpl::GetShadowOffsetY(v8::Local<v8::String> name,
                                                    const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto ret = static_cast<double>(canvas_native_context_get_shadow_offset_y(ptr->GetContext()));
    info.GetReturnValue().Set(ret);
}

void CanvasRenderingContext2DImpl::SetShadowOffsetY(v8::Local<v8::String> name,
                                                    v8::Local<v8::Value> value,
                                                    const v8::PropertyCallbackInfo<void> &info) {
    if (Helpers::IsNumber(value)) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.This());
        canvas_native_context_set_shadow_offset_y(ptr->GetContext(),
                                                  static_cast<float>(value->NumberValue(
                                                          context).ToChecked()));
    }
}

void CanvasRenderingContext2DImpl::GetTextAlign(v8::Local<v8::String> name,
                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.This());
    auto align = canvas_native_context_get_text_align(ptr->GetContext());
    info.GetReturnValue().Set(
            Helpers::ConvertToV8String(isolate, std::string(align.data(), align.size())));
}

void
CanvasRenderingContext2DImpl::SetTextAlign(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                           const v8::PropertyCallbackInfo<void> &info) {
    if (Helpers::IsString(value)) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.This());
        auto align = Helpers::ConvertFromV8String(isolate, value);
        canvas_native_context_set_text_align(ptr->GetContext(),
                                             rust::Str(align.c_str(), align.size()));
    }
}

void CanvasRenderingContext2DImpl::GetGlobalCompositeOperation(v8::Local<v8::String> name,
                                                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.This());
    auto composite = canvas_native_context_get_global_composition(ptr->GetContext());
    info.GetReturnValue().Set(
            Helpers::ConvertToV8String(isolate, std::string(composite.data(), composite.size()))
    );
}

void CanvasRenderingContext2DImpl::SetGlobalCompositeOperation(v8::Local<v8::String> name,
                                                               v8::Local<v8::Value> value,
                                                               const v8::PropertyCallbackInfo<void> &info) {
    if (Helpers::IsString(value)) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.This());
        auto val = Helpers::ConvertFromV8String(isolate, value);
        canvas_native_context_set_global_composition(ptr->GetContext(),
                                                     rust::Str(val.c_str(), val.size()));
    }
}

void CanvasRenderingContext2DImpl::GetFillStyle(v8::Local<v8::String> name,
                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.This());
    PaintStyleType type = canvas_native_context_get_current_fill_style_type(ptr->GetContext());

    switch (type) {
        case PaintStyleType::Color: {
//            uint8_t r;
//            uint8_t g;
//            uint8_t b;
//            uint8_t a;
//
//            canvas_native_paint_style_get_current_fill_color_r_g_b_a(ptr->GetContext(), r, g, b, a);
//
//            auto color = Helpers::RGBAToHex(r, g, b, a);
//
//            auto ret = Helpers::ConvertToV8String(isolate, color);

            auto color = canvas_native_paint_style_get_current_fill_color_buf(ptr->GetContext());


            auto extString = new OneByteStringResource(std::move(color));

            info.GetReturnValue().Set(
                    v8::String::NewExternalOneByte(isolate, extString).ToLocalChecked()
            );
            break;
        }
        case PaintStyleType::Gradient: {
            auto style = canvas_native_context_get_fill_style(ptr->GetContext());
            info.GetReturnValue().Set(CanvasGradient::NewInstance(isolate, std::move(style)));
            break;
        }
        case PaintStyleType::Pattern: {
            auto style = canvas_native_context_get_fill_style(ptr->GetContext());
            info.GetReturnValue().Set(CanvasPattern::NewInstance(isolate, std::move(style)));
            break;
        }
        case PaintStyleType::None: {
            // should not reach here .... throw ? run for your life
            info.GetReturnValue().SetUndefined();
            break;
        }
    }
}

void
CanvasRenderingContext2DImpl::SetFillStyle(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                           const v8::PropertyCallbackInfo<void> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(info.This());
    if (ptr == nullptr) { return; }
    if (Helpers::IsString(value)) {

        uint8_t r;
        uint8_t g;
        uint8_t b;
        uint8_t a;


        auto string = value->ToString(context).ToLocalChecked();
        if (string->IsExternalOneByte()) {
            auto resource = string->GetExternalOneByteStringResource();
            if (canvas_native_parse_css_color_rgba(rust::Str(resource->data(), resource->length()),
                                                   r, g, b, a)) {
                canvas_native_paint_style_set_fill_color_with_rgba(ptr->GetContext(), r, g, b, a);
            }
        } else {

            v8::String::Value stringValue(isolate, value);
            auto str = rust::Slice<const uint16_t>((uint16_t * ) * stringValue,
                                                   stringValue.length());
            canvas_native_paint_style_set_fill_color_with_utf16(ptr->GetContext(), str);
        }

//        auto color = CSSColorParser::parse(val);
//
//        if (!color.isValid) {
//            return;
//        }
//
//        canvas_native_paint_style_set_fill_color_with_rgba(ptr->GetContext(), color.r, color.g, color.b,
//                                                           (uint8_t)(color.a * 255));
    } else if (!value->IsNullOrUndefined() && value->IsObject()) {
        auto color = value.As<v8::Object>();
        if (Helpers::GetInstanceType(isolate, color) == ObjectType::CanvasPattern) {
            auto pattern_ptr = CanvasPattern::GetPointer(color);
            if (pattern_ptr != nullptr) {
                canvas_native_context_set_fill_style(ptr->GetContext(),
                                                     pattern_ptr->GetPaintStyle());
            }
        }
        if (Helpers::GetInstanceType(isolate, color) == ObjectType::CanvasGradient) {
            auto gradient_ptr = CanvasGradient::GetPointer(color);
            if (gradient_ptr != nullptr) {
                canvas_native_context_set_fill_style(ptr->GetContext(),
                                                     gradient_ptr->GetPaintStyle());
            }
        }
    }
}

void CanvasRenderingContext2DImpl::GetStrokeStyle(v8::Local<v8::String> name,
                                                  const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ptr = GetPointer(info.This());
    PaintStyleType type = canvas_native_context_get_current_stroke_style_type(ptr->GetContext());
    switch (type) {
        case PaintStyleType::Color: {
//            uint8_t r;
//            uint8_t g;
//            uint8_t b;
//            uint8_t a;
//
//            canvas_native_paint_style_get_current_stroke_color_r_g_b_a(ptr->GetContext(), r, g, b, a);
//
//            auto color = Helpers::RGBAToHex(r, g, b, a);
//
//            auto ret = Helpers::ConvertToV8String(isolate, color);
//
//            info.GetReturnValue().Set(ret);


            auto color = canvas_native_paint_style_get_current_stroke_color_buf(ptr->GetContext());
            auto extString = new OneByteStringResource(std::move(color));

            info.GetReturnValue().Set(
                    v8::String::NewExternalOneByte(isolate, extString).ToLocalChecked()
            );
            break;
        }
        case PaintStyleType::Gradient: {
            auto style = canvas_native_context_get_stroke_style(ptr->GetContext());
            info.GetReturnValue().Set(CanvasGradient::NewInstance(isolate, std::move(style)));
            break;
        }
        case PaintStyleType::Pattern: {
            auto style = canvas_native_context_get_stroke_style(ptr->GetContext());
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

void
CanvasRenderingContext2DImpl::SetStrokeStyle(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                             const v8::PropertyCallbackInfo<void> &info) {
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(info.This());
    if (Helpers::IsString(value)) {
        auto val = Helpers::ConvertFromV8String(isolate, value);
        uint8_t r;
        uint8_t g;
        uint8_t b;
        uint8_t a;

        auto string = value->ToString(context).ToLocalChecked();
        if (string->IsExternalOneByte()) {
            auto resource = string->GetExternalOneByteStringResource();
            if (canvas_native_parse_css_color_rgba(rust::Str(resource->data(), resource->length()),
                                                   r, g, b, a)) {
                canvas_native_paint_style_set_stroke_color_with_rgba(ptr->GetContext(), r, g, b, a);
            }
        } else {
//            auto val = Helpers::ConvertFromV8String(isolate, string);
//            if (canvas_native_parse_css_color_rgba(rust::Str(val.data(), val.size()), r, g, b, a)) {
//                canvas_native_paint_style_set_stroke_color_with_rgba(ptr->GetContext(), r, g, b, a);
//            }

            v8::String::Value stringValue(isolate, value);
            auto str = rust::Slice<const uint16_t>(*stringValue, stringValue.length());
            canvas_native_paint_style_set_stroke_color_with_utf16(ptr->GetContext(), str);
        }

//        auto color = CSSColorParser::parse(val);
//        if (!color.isValid) {
//            return;
//        }
//
//        canvas_native_paint_style_set_stroke_color_with_rgba(ptr->GetContext(), color.r, color.g, color.b,
//                                                             (uint8_t)(color.a * 255));
    } else if (Helpers::IsObject(value)) {
        auto color = value.As<v8::Object>();
        if (Helpers::GetInstanceType(isolate, color) == ObjectType::CanvasPattern) {
            auto pattern_ptr = CanvasPattern::GetPointer(color);
            if (pattern_ptr != nullptr) {
                canvas_native_context_set_stroke_style(ptr->GetContext(),
                                                       pattern_ptr->GetPaintStyle());
            }
        }

        if (Helpers::GetInstanceType(isolate, color) == ObjectType::CanvasGradient) {
            auto gradient_ptr = CanvasGradient::GetPointer(color);
            if (gradient_ptr != nullptr) {
                canvas_native_context_set_stroke_style(ptr->GetContext(),
                                                       gradient_ptr->GetPaintStyle());
            }
        }
    }
}


void CanvasRenderingContext2DImpl::GetLineWidth(v8::Local<v8::String> name,
                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto ret = static_cast<double>(canvas_native_context_get_line_width(ptr->GetContext()));
    info.GetReturnValue().Set(ret);
}

void
CanvasRenderingContext2DImpl::SetLineWidth(v8::Local<v8::String> name, v8::Local<v8::Value> value,
                                           const v8::PropertyCallbackInfo<void> &info) {
    if (Helpers::IsNumber(value)) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.This());
        canvas_native_context_set_line_width(ptr->GetContext(),
                                             static_cast<float>(value->NumberValue(
                                                     context).ToChecked()));
    }
}


void CanvasRenderingContext2DImpl::GetLineDashOffset(v8::Local<v8::String> name,
                                                     const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto ret = (double) canvas_native_context_get_line_dash_offset(ptr->GetContext());
    info.GetReturnValue().Set(ret);
}

void CanvasRenderingContext2DImpl::SetLineDashOffset(v8::Local<v8::String> name,
                                                     v8::Local<v8::Value> value,
                                                     const v8::PropertyCallbackInfo<void> &info) {
    if (Helpers::IsNumber(value)) {
        auto isolate = info.GetIsolate();
        auto context = isolate->GetCurrentContext();
        auto ptr = GetPointer(info.This());
        canvas_native_context_set_line_dash_offset(ptr->GetContext(),
                                                   static_cast<float>(value->NumberValue(
                                                           context).ToChecked()));
    }
}


void CanvasRenderingContext2DImpl::AddHitRegion(const v8::FunctionCallbackInfo<v8::Value> &args) {
    // noop .. todo ?
}

void CanvasRenderingContext2DImpl::Arc(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto anti_clockwise = false;
    if (args.Length() == 6) {
        anti_clockwise = args[5]->BooleanValue(isolate);
    }
    canvas_native_context_arc(
            ptr->GetContext(),
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
    auto ptr = GetPointer(args.This());

    auto x1 = static_cast<float>(args[0]->NumberValue(context).ToChecked());
    auto y1 = static_cast<float>(args[1]->NumberValue(context).ToChecked());
    auto x2 = static_cast<float>(args[2]->NumberValue(context).ToChecked());
    auto y2 = static_cast<float>(args[3]->NumberValue(context).ToChecked());
    auto radius = static_cast<float>(args[4]->NumberValue(context).ToChecked());
    canvas_native_context_arc_to(
            ptr->GetContext(),
            x1,
            y1,
            x2,
            y2,
            radius
    );
}

void CanvasRenderingContext2DImpl::BeginPath(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    canvas_native_context_begin_path(ptr->GetContext());
}

void CanvasRenderingContext2DImpl::BezierCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    auto context = args.GetIsolate()->GetCurrentContext();
    canvas_native_context_bezier_curve_to(
            ptr->GetContext(),
            static_cast<float>(args[0]->NumberValue(context).ToChecked()),
            static_cast<float>(args[1]->NumberValue(context).ToChecked()),
            static_cast<float>(args[2]->NumberValue(context).ToChecked()),
            static_cast<float>(args[3]->NumberValue(context).ToChecked()),
            static_cast<float>(args[4]->NumberValue(context).ToChecked()),
            static_cast<float>(args[5]->NumberValue(context).ToChecked())
    );
}

void
CanvasRenderingContext2DImpl::ClearHitRegions(const v8::FunctionCallbackInfo<v8::Value> &args) {
// noop
}

void CanvasRenderingContext2DImpl::ClearRect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    auto context = args.GetIsolate()->GetCurrentContext();
    canvas_native_context_clear_rect(
            ptr->GetContext(),
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
    auto ptr = GetPointer(args.This());
    if (args.Length() == 0) {
        std::string rule("nonzero");
        canvas_native_context_clip_rule(ptr->GetContext(), rust::Str(rule.c_str(), rule.size()));
    } else if (Helpers::IsString(args[0])) {
        // TODO throw for invalid enum
        auto val = Helpers::ConvertFromV8String(isolate, args[0]);
        canvas_native_context_clip_rule(ptr->GetContext(), rust::Str(val.c_str(), val.size()));
    }
}

void CanvasRenderingContext2DImpl::ClosePath(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    canvas_native_context_close_path(ptr->GetContext());
}

void
CanvasRenderingContext2DImpl::CreateImageData(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 1 && args[0]->IsObject()) {

        auto object = args[0]->ToObject(context).ToLocalChecked();

        if (Helpers::GetInstanceType(isolate, object) == ObjectType::ImageData) {
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


void CanvasRenderingContext2DImpl::CreateLinearGradient(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 4) {
        auto x0 = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y0 = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto x1 = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto y1 = static_cast<float>(args[3]->NumberValue(context).ToChecked());

        auto gradient = canvas_native_context_create_linear_gradient(ptr->GetContext(), x0, y0, x1,
                                                                     y1);
        args.GetReturnValue().Set(CanvasGradient::NewInstance(isolate, std::move(gradient)));
    }
}

void CanvasRenderingContext2DImpl::CreatePattern(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2) {
        auto image = args[0]->ToObject(context).ToLocalChecked();
        // TODO handle other cases
        auto type = Helpers::GetInstanceType(isolate, image);
        if (type == ObjectType::ImageAsset) {
            auto asset = ImageAssetImpl::GetPointer(image);
            auto rep = Helpers::ConvertFromV8String(isolate, args[1]);
            rust::Box <PaintStyle> pattern = canvas_native_context_create_pattern_asset(
                    ptr->GetContext(),
                    asset->GetImageAsset(),
                    rust::Str(rep.c_str(),
                              rep.size()));
            auto type = canvas_native_context_get_style_type(*pattern);
            if (type == PaintStyleType::None) {
                args.GetReturnValue().SetUndefined();
            } else {
                args.GetReturnValue().Set(CanvasPattern::NewInstance(isolate, std::move(pattern)));
            }
            return;
        } else if (type == ObjectType::CanvasRenderingContext2D) {
            auto source = CanvasRenderingContext2DImpl::GetPointer(image);
            auto rep = Helpers::ConvertFromV8String(isolate, args[1]);
            rust::Box <PaintStyle> pattern = canvas_native_context_create_pattern_canvas2d(
                    *source->context_,
                    ptr->GetContext(),
                    rust::Str(rep.c_str(),
                              rep.size()));
            auto type = canvas_native_context_get_style_type(*pattern);
            if (type == PaintStyleType::None) {
                args.GetReturnValue().SetUndefined();
            } else {
                args.GetReturnValue().Set(CanvasPattern::NewInstance(isolate, std::move(pattern)));
            }
            return;
        } else if (type == ObjectType::WebGLRenderingContext ||
                   type == ObjectType::WebGL2RenderingContext) {
            auto source = WebGLRenderingContextBase::GetPointerBase(image);
            auto rep = Helpers::ConvertFromV8String(isolate, args[1]);
            rust::Box <PaintStyle> pattern = canvas_native_context_create_pattern_webgl(
                    source->GetState(),
                    ptr->GetContext(),
                    rust::Str(rep.c_str(),
                              rep.size()));
            auto type = canvas_native_context_get_style_type(*pattern);
            if (type == PaintStyleType::None) {
                args.GetReturnValue().SetUndefined();
            } else {
                args.GetReturnValue().Set(CanvasPattern::NewInstance(isolate, std::move(pattern)));
            }
            return;
        }

        args.GetReturnValue().SetUndefined();
    } else {
        args.GetReturnValue().SetUndefined();
    }
}

void CanvasRenderingContext2DImpl::CreatePatternAndroid(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2 && Helpers::IsString(args[0])) {
        auto bytes = Helpers::GetString(isolate, args[0]);
        char *endptr;
        auto bytes_addr = std::strtoll(bytes.c_str(), &endptr, 10);
        if (bytes_addr == 0) {
            args.GetReturnValue().SetUndefined();
            return;
        }

        auto rep = Helpers::ConvertFromV8String(isolate, args[1]);
        rust::Box <PaintStyle> pattern = canvas_native_context_create_pattern_bytes(
                ptr->GetContext(),
                bytes_addr,
                rust::Str(rep.c_str(),
                          rep.size()));
        auto type = canvas_native_context_get_style_type(*pattern);
        if (type == PaintStyleType::None) {
            args.GetReturnValue().SetUndefined();
        } else {
            args.GetReturnValue().Set(CanvasPattern::NewInstance(isolate, std::move(pattern)));
        }

        args.GetReturnValue().SetUndefined();
    } else {
        args.GetReturnValue().SetUndefined();
    }
}

void CanvasRenderingContext2DImpl::CreateRadialGradient(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 6) {
        auto x0 = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y0 = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto r0 = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto x1 = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto y1 = static_cast<float>(args[4]->NumberValue(context).ToChecked());
        auto r1 = static_cast<float>(args[5]->NumberValue(context).ToChecked());

        auto gradient = canvas_native_context_create_radial_gradient(ptr->GetContext(), x0, y0, r0,
                                                                     x1, y1, r1);
        args.GetReturnValue().Set(CanvasGradient::NewInstance(isolate, std::move(gradient)));
    }
}

void
CanvasRenderingContext2DImpl::DrawFocusIfNeeded(const v8::FunctionCallbackInfo<v8::Value> &args) {
    // NOOP
}


void CanvasRenderingContext2DImpl::DrawImage(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    // TODO handle other cases
    if (args.Length() == 3) {
        auto image = args[0].As<v8::Object>();
        auto dx = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto dy = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto type = Helpers::GetInstanceType(isolate, image);
        if (type == ObjectType::ImageAsset) {
            auto asset = ImageAssetImpl::GetPointer(image);
            canvas_native_context_draw_image_dx_dy_asset(ptr->GetContext(), asset->GetImageAsset(),
                                                         dx, dy);
            ptr->UpdateInvalidateState();
        } else if (type == ObjectType::ImageBitmap) {
            auto asset = ImageAssetImpl::GetPointer(image);
            canvas_native_context_draw_image_dx_dy_asset(ptr->GetContext(), asset->GetImageAsset(),
                                                         dx, dy);
            ptr->UpdateInvalidateState();
        }
    } else if (args.Length() == 5) {
        auto image = args[0]->ToObject(context).ToLocalChecked();
        auto dx = args[1]->NumberValue(context).ToChecked();
        auto dy = args[2]->NumberValue(context).ToChecked();
        auto dWidth = args[3]->NumberValue(context).ToChecked();
        auto dHeight = args[4]->NumberValue(context).ToChecked();

        auto type = Helpers::GetInstanceType(isolate, image);
        if (type == ObjectType::ImageAsset) {
            auto asset = ImageAssetImpl::GetPointer(image);
            canvas_native_context_draw_image_dx_dy_dw_dh_asset(ptr->GetContext(),
                                                               asset->GetImageAsset(), dx, dy,
                                                               dWidth,
                                                               dHeight);
            ptr->UpdateInvalidateState();
        } else if (type == ObjectType::ImageBitmap) {
            auto asset = ImageAssetImpl::GetPointer(image);
            canvas_native_context_draw_image_dx_dy_dw_dh_asset(ptr->GetContext(),
                                                               asset->GetImageAsset(), dx, dy,
                                                               dWidth,
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

        auto type = Helpers::GetInstanceType(isolate, image);

        if (type == ObjectType::ImageAsset) {
            auto asset = ImageAssetImpl::GetPointer(image);
            canvas_native_context_draw_image_asset(ptr->GetContext(), asset->GetImageAsset(), sx,
                                                   sy, sWidth, sHeight,
                                                   dx,
                                                   dy, dWidth, dHeight);
            ptr->UpdateInvalidateState();
        } else if (type == ObjectType::ImageBitmap) {
            auto asset = ImageAssetImpl::GetPointer(image);
            canvas_native_context_draw_image_asset(ptr->GetContext(), asset->GetImageAsset(), sx,
                                                   sy, sWidth, sHeight,
                                                   dx,
                                                   dy, dWidth, dHeight);
            ptr->UpdateInvalidateState();
        }
    }
}


void CanvasRenderingContext2DImpl::Ellipse(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
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

        canvas_native_context_ellipse(ptr->GetContext(), x, y, radiusX, radiusY, rotation,
                                      startAngle, endAngle,
                                      anticlockwise);
    }
}


void CanvasRenderingContext2DImpl::Fill(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2) {
        auto object = args[0]->ToObject(context).ToLocalChecked();
        auto rule = args[1];
        if (Helpers::GetInstanceType(isolate, object) == ObjectType::Path2D) {
            auto path = Path2D::GetPointer(object);
            auto value = Helpers::ConvertFromV8String(isolate, rule);
            canvas_native_context_fill_with_path(ptr->GetContext(), path->GetPath(),
                                                 rust::Str(value.c_str(), value.size()));
            ptr->UpdateInvalidateState();
        }
    } else if (args.Length() == 1) {
        if (args[0]->IsString()) {
            auto value = Helpers::ConvertFromV8String(isolate, args[0]);
            canvas_native_context_fill(ptr->GetContext(), rust::Str(value.c_str(), value.size()));
            ptr->UpdateInvalidateState();
        } else if (args[0]->IsObject()) {
            auto object = args[0]->ToObject(context).ToLocalChecked();
            if (Helpers::GetInstanceType(isolate, object) == ObjectType::Path2D) {
                auto path = Path2D::GetPointer(object);
                std::string rule("nonzero");
                canvas_native_context_fill_with_path(ptr->GetContext(), path->GetPath(),
                                                     rust::Str(rule.c_str(), rule.size()));
                ptr->UpdateInvalidateState();
            }
        }
    } else {
        std::string rule("nonzero");
        canvas_native_context_fill(ptr->GetContext(), rust::Str(rule.c_str(), rule.size()));
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::FillRect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto width = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto height = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        canvas_native_context_fill_rect(ptr->GetContext(), x, y, width, height);
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::FillText(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() >= 3) {
        auto text = Helpers::ConvertFromV8String(isolate, args[0]);
        auto x = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        float width = -1;
        if (args[3]->IsNumber()) {
            width = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        }
        canvas_native_context_fill_text(ptr->GetContext(), rust::Str(text.c_str(), text.size()), x,
                                        y, width);
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::GetImageData(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 4) {
        auto sx = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto sy = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto sw = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto sh = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto data = canvas_native_context_get_image_data(ptr->GetContext(), sx, sy, sw, sh);
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
    auto ptr = GetPointer(args.This());
    auto dash = canvas_native_context_get_line_dash(ptr->GetContext());
    auto size = dash.size();
    auto array = v8::Array::New(isolate, size);
    for (int i = 0; i < size; ++i) {
        array->Set(context, i, v8::Number::New(isolate, (double) dash[i]));
    }
    args.GetReturnValue().Set(array);
}

void CanvasRenderingContext2DImpl::IsPointInPath(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        std::string rule("nonzero");
        auto ret = canvas_native_context_is_point_in_path(ptr->GetContext(), x, y,
                                                          rust::Str(rule.c_str(), rule.size()));
        args.GetReturnValue().Set(ret);
    } else if (args.Length() == 3 && args[2]->IsString()) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto rule = Helpers::ConvertFromV8String(isolate, args[2]);
        auto ret = canvas_native_context_is_point_in_path(ptr->GetContext(), x, y,
                                                          rust::Str(rule.c_str(), rule.size()));
        args.GetReturnValue().Set(ret);
    } else if (args.Length() == 4 && args[0]->IsObject() && args[3]->IsString()) {
        auto path = args[0]->ToObject(context).ToLocalChecked();
        auto x = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto rule = Helpers::ConvertFromV8String(isolate, args[3]);
        if (Helpers::GetInstanceType(isolate, path) == ObjectType::Path2D) {
            auto path_ptr = Path2D::GetPointer(path);
            auto ret = canvas_native_context_is_point_in_path_with_path(ptr->GetContext(),
                                                                        path_ptr->GetPath(), x, y,
                                                                        rust::Str(rule.c_str(),
                                                                                  rule.size()));
            args.GetReturnValue().Set(ret);
        } else {
            args.GetReturnValue().Set(false);
        }
    } else {
        // TODO other checks ?
        args.GetReturnValue().Set(false);
    }
}

void
CanvasRenderingContext2DImpl::IsPointInStroke(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto ret = canvas_native_context_is_point_in_stroke(ptr->GetContext(), x, y);
        args.GetReturnValue().Set(ret);
    } else if (args.Length() == 3 && args[0]->IsObject()) {
        auto path = args[0]->ToObject(context).ToLocalChecked();
        auto x = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        if (Helpers::GetInstanceType(isolate, path) == ObjectType::Path2D) {
            auto path_ptr = Path2D::GetPointer(path);
            auto ret = canvas_native_context_is_point_in_stroke_with_path(ptr->GetContext(),
                                                                          path_ptr->GetPath(), x,
                                                                          y);
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
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        canvas_native_context_line_to(ptr->GetContext(), x, y);
    }
}

void CanvasRenderingContext2DImpl::MeasureText(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto text = Helpers::ConvertFromV8String(isolate, args[0]);
    auto metrics = canvas_native_context_measure_text(ptr->GetContext(),
                                                      rust::Str(text.c_str(), text.size()));
    args.GetReturnValue().Set(TextMetricsImpl::NewInstance(isolate, std::move(metrics)));
}


void CanvasRenderingContext2DImpl::MoveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        canvas_native_context_move_to(ptr->GetContext(), x, y);
    }
}

void CanvasRenderingContext2DImpl::PutImageData(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    auto imageData = args[0]->ToObject(context).ToLocalChecked();
    if (args.Length() == 3) {
        auto imageDataPtr = ImageDataImpl::GetPointer(imageData);
        auto dx = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto dy = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        float dirtyX = 0;
        float dirtyY = 0;
        auto dirtyWidth = imageDataPtr->GetWidth();
        auto dirtyHeight = imageDataPtr->GetHeight();
        canvas_native_context_put_image_data(ptr->GetContext(), imageDataPtr->GetImageData(), dx,
                                             dy, dirtyX, dirtyY,
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
        canvas_native_context_put_image_data(ptr->GetContext(), imageDataPtr->GetImageData(), dx,
                                             dy, dirtyX, dirtyY,
                                             dirtyWidth, dirtyHeight);
        ptr->UpdateInvalidateState();
    }
}

void
CanvasRenderingContext2DImpl::QuadraticCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 4) {
        auto cpx = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto cpy = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto x = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        canvas_native_context_quadratic_curve_to(ptr->GetContext(), cpx, cpy, x, y);
    }
}

void CanvasRenderingContext2DImpl::Rect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 4) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto width = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto height = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        canvas_native_context_rect(ptr->GetContext(), x, y, width, height);
    }
}


void
CanvasRenderingContext2DImpl::RemoveHitRegion(const v8::FunctionCallbackInfo<v8::Value> &args) {
    // NOOP
}


void CanvasRenderingContext2DImpl::ResetTransform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    canvas_native_context_reset_transform(ptr->GetContext());
}


void CanvasRenderingContext2DImpl::Restore(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.This());
    canvas_native_context_restore(ptr->GetContext());
}


void CanvasRenderingContext2DImpl::Rotate(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 1 && args[0]->IsNumber()) {
        canvas_native_context_rotate(ptr->GetContext(), args[0]->NumberValue(context).ToChecked());
    }
}

void CanvasRenderingContext2DImpl::Save(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.This());
    canvas_native_context_save(ptr->GetContext());
}

void CanvasRenderingContext2DImpl::Scale(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        canvas_native_context_scale(ptr->GetContext(), x, y);
    }
}

void
CanvasRenderingContext2DImpl::ScrollPathIntoView(const v8::FunctionCallbackInfo<v8::Value> &args) {
// NOOP
}

void CanvasRenderingContext2DImpl::SetLineDash(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 1) {
        auto vec = args[0];
        if (vec->IsArray()) {
            auto segments = vec.As<v8::Array>();
            auto len = segments->Length();
            std::vector<float> data;
            auto context = isolate->GetCurrentContext();
            for (int i = 0; i < len; ++i) {
                auto item = segments->Get(context, i);
                data.push_back(static_cast<float>(item.ToLocalChecked()->NumberValue(context).ToChecked()));
            }
            rust::Slice<const float> slice{data.data(), data.size()};
            canvas_native_context_set_line_dash(ptr->GetContext(), slice);
        }
    }
}

void CanvasRenderingContext2DImpl::SetTransform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 1 && args[0]->IsObject()) {
        auto matrix = args[0]->ToObject(context).ToLocalChecked();
        if (Helpers::GetInstanceType(isolate, matrix) == ObjectType::Matrix) {
            auto matrix_ptr = MatrixImpl::GetPointer(matrix);
            canvas_native_context_set_transform_matrix(ptr->GetContext(), matrix_ptr->GetMatrix());
        }
    } else if (args.Length() == 6) {
        auto a = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto b = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto c = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto d = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto e = static_cast<float>(args[4]->NumberValue(context).ToChecked());
        auto f = static_cast<float>(args[5]->NumberValue(context).ToChecked());
        canvas_native_context_set_transform(ptr->GetContext(), a, b, c, d, e, f);
    }
}

void CanvasRenderingContext2DImpl::Stroke(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 1 && args[0]->IsObject()) {
        auto path = args[0]->ToObject(context).ToLocalChecked();
        if (Helpers::GetInstanceType(isolate, path) == ObjectType::Path2D) {
            auto path_ptr = Path2D::GetPointer(path);
            canvas_native_context_stroke_with_path(ptr->GetContext(), path_ptr->GetPath());
            ptr->UpdateInvalidateState();
        }
    } else {
        canvas_native_context_stroke(ptr->GetContext());
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::StrokeRect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 4) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto width = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto height = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        canvas_native_context_stroke_rect(ptr->GetContext(), x, y, width, height);
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::StrokeText(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() >= 3) {
        auto text = Helpers::ConvertFromV8String(isolate, args[0]);
        auto x = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        float maxWidth = static_cast<float>(args[3]->NumberValue(context).FromMaybe(-1));
        canvas_native_context_stroke_text(ptr->GetContext(), rust::Str(text.c_str(), text.size()),
                                          x, y, maxWidth);
        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::Transform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 6) {
        auto a = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto b = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto c = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto d = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto e = static_cast<float>(args[4]->NumberValue(context).ToChecked());
        auto f = static_cast<float>(args[5]->NumberValue(context).ToChecked());
        canvas_native_context_transform(ptr->GetContext(), a, b, c, d, e, f);
    }
}

void CanvasRenderingContext2DImpl::Translate(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        canvas_native_context_translate(ptr->GetContext(), x, y);
    }
}

void CanvasRenderingContext2DImpl::ToDataURL(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    std::string type("image/png");
    int quality = 92;
    auto typeVal = args[0];
    auto qualityVal = args[1];

    if (typeVal->IsStringObject()) {
        type = Helpers::ConvertFromV8String(isolate, typeVal.As<v8::StringObject>()->ValueOf());
    }

    if (typeVal->IsString()) {
        type = Helpers::ConvertFromV8String(isolate, typeVal);
    }

    if (qualityVal->IsNumberObject()) {
        quality = static_cast<int32_t>(qualityVal.As<v8::NumberObject>()->ValueOf());
    }

    if (qualityVal->IsNumber()) {
        quality = qualityVal->Int32Value(context).FromMaybe(quality);
    }

    auto data = canvas_native_to_data_url(ptr->GetContext(), rust::Str(type.c_str(), type.size()),
                                          quality);
    args.GetReturnValue().Set(
            Helpers::ConvertToV8String(isolate, std::string(data.c_str(), data.size())));
}

v8::Local<v8::FunctionTemplate> CanvasRenderingContext2DImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto tmpl = cache->CanvasRenderingContext2DTmpl.get();
    if (tmpl != nullptr) {
        return tmpl->Get(isolate);
    }

    auto context = isolate->GetCurrentContext();
    auto canvasRenderingContextFunc = v8::FunctionTemplate::New(isolate, &Create);
    canvasRenderingContextFunc->SetClassName(
            Helpers::ConvertToV8String(isolate, "CanvasRenderingContext2D"));
    canvasRenderingContextFunc->InstanceTemplate()->SetInternalFieldCount(1);

    auto canvasRenderingContextTpl = canvasRenderingContextFunc->PrototypeTemplate();


    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "__resize"),
            v8::FunctionTemplate::New(isolate, &Resize)
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "globalAlpha"),
            &GetGlobalAlpha,
            &SetGlobalAlpha
    );

    canvasRenderingContextTpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "font"),
            &GetFont,
            &SetFont
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
            Helpers::ConvertToV8String(isolate, "__createPatternWithBitmap"),
            v8::FunctionTemplate::New(isolate, &CreatePatternAndroid)
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

    canvasRenderingContextTpl->Set(
            Helpers::ConvertToV8String(isolate, "__toDataURL"),
            v8::FunctionTemplate::New(isolate, &ToDataURL)
    );

    cache->CanvasRenderingContext2DTmpl = std::make_unique<v8::Persistent<v8::FunctionTemplate>>(
            isolate,
            canvasRenderingContextFunc);
    return canvasRenderingContextFunc;
}

CanvasRenderingContext2D &CanvasRenderingContext2DImpl::GetContext() {
    return *this->context_;
}
