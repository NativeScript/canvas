//
// Created by Osei Fortune on 22/03/2022.
//

#include "CanvasRenderingContext2DImpl.h"
#include "Caches.h"
#include "OneByteStringResource.h"


v8::CFunction CanvasRenderingContext2DImpl::fast_ellipse_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastEllipse));

v8::CFunction CanvasRenderingContext2DImpl::fast_bezier_curve_to_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastBezierCurveTo));


v8::CFunction CanvasRenderingContext2DImpl::fast_clip_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastClip));

v8::CFunction CanvasRenderingContext2DImpl::fast_clip_rule_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastClipRule));

v8::CFunction CanvasRenderingContext2DImpl::fast_clip_path_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastClipPath));


const v8::CFunction fast_clip_overloads_[] = {
        CanvasRenderingContext2DImpl::fast_clip_,
        CanvasRenderingContext2DImpl::fast_clip_path_,
        CanvasRenderingContext2DImpl::fast_clip_rule_
};


v8::CFunction CanvasRenderingContext2DImpl::fast_close_path_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastClosePath));

v8::CFunction CanvasRenderingContext2DImpl::fast_begin_path_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastBeginPath));

v8::CFunction CanvasRenderingContext2DImpl::fast_arc_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastArc));

v8::CFunction CanvasRenderingContext2DImpl::fast_arc_to_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastArcTo));


v8::CFunction CanvasRenderingContext2DImpl::fast_draw_image_dx_dy_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastDrawImageDxDy));


v8::CFunction CanvasRenderingContext2DImpl::fast_draw_image_dx_dy_dw_dh_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastDrawImageDxDyDwDh));

v8::CFunction CanvasRenderingContext2DImpl::fast_draw_image_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastDrawImage));


const v8::CFunction fast_draw_overloads_[] = {
        CanvasRenderingContext2DImpl::fast_draw_image_,
        CanvasRenderingContext2DImpl::fast_draw_image_dx_dy_dw_dh_,
        CanvasRenderingContext2DImpl::fast_draw_image_dx_dy_
};


v8::CFunction CanvasRenderingContext2DImpl::fast_save_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastSave));

v8::CFunction CanvasRenderingContext2DImpl::fast_restore_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastRestore));


v8::CFunction CanvasRenderingContext2DImpl::fast_translate_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastTranslate));

v8::CFunction CanvasRenderingContext2DImpl::fast_clear_rect_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastClearRect));

v8::CFunction CanvasRenderingContext2DImpl::fast_fill_rect_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastFillRect));

v8::CFunction CanvasRenderingContext2DImpl::fast_stroke_rect_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastStrokeRect));

v8::CFunction CanvasRenderingContext2DImpl::fast_rotate_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastRotate));


v8::CFunction CanvasRenderingContext2DImpl::fast_fill_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastFill));

v8::CFunction CanvasRenderingContext2DImpl::fast_fill_path_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastFillPath));

v8::CFunction CanvasRenderingContext2DImpl::fast_fill_rule_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastFillRule));

v8::CFunction CanvasRenderingContext2DImpl::fast_fill_path_rule_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastFillPathRule));

const v8::CFunction fast_fill_overloads_[] = {
        CanvasRenderingContext2DImpl::fast_fill_path_rule_,
        CanvasRenderingContext2DImpl::fast_fill_path_,
        CanvasRenderingContext2DImpl::fast_fill_rule_,
        CanvasRenderingContext2DImpl::fast_fill_,

};


v8::CFunction CanvasRenderingContext2DImpl::fast_put_image_data_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastPutImageData));

v8::CFunction CanvasRenderingContext2DImpl::fast_put_image_data_dx_dy_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastPutImageDataDxDy));

const v8::CFunction fast_put_image_data_overloads_[] = {
        CanvasRenderingContext2DImpl::fast_put_image_data_,
        CanvasRenderingContext2DImpl::fast_put_image_data_dx_dy_

};

v8::CFunction CanvasRenderingContext2DImpl::fast_stroke_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastStroke));

v8::CFunction CanvasRenderingContext2DImpl::fast_stroke_path_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastStrokePath));

const v8::CFunction fast_stroke_overloads_[] = {
        CanvasRenderingContext2DImpl::fast_stroke_,
        CanvasRenderingContext2DImpl::fast_stroke_path_
};

v8::CFunction CanvasRenderingContext2DImpl::fast_is_point_in_path_xy_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastIsPointInPathXY));

v8::CFunction CanvasRenderingContext2DImpl::fast_is_point_in_path_xy_rule_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastIsPointInPathXYRule));

v8::CFunction CanvasRenderingContext2DImpl::fast_is_point_in_path_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastIsPointInPath));

const v8::CFunction fast_is_point_in_path_overloads_[] = {
        CanvasRenderingContext2DImpl::fast_is_point_in_path_,
        CanvasRenderingContext2DImpl::fast_is_point_in_path_xy_rule_,
        CanvasRenderingContext2DImpl::fast_is_point_in_path_xy_
};


v8::CFunction CanvasRenderingContext2DImpl::fast_is_point_in_stroke_xy_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastIsPointInStrokeXY));

v8::CFunction CanvasRenderingContext2DImpl::fast_is_point_in_stroke_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastIsPointInStroke));

const v8::CFunction fast_is_point_in_stroke_overloads_[] = {
        CanvasRenderingContext2DImpl::fast_is_point_in_stroke_,
        CanvasRenderingContext2DImpl::fast_is_point_in_stroke_xy_
};

v8::CFunction CanvasRenderingContext2DImpl::fast_line_to_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastLineTo));

v8::CFunction CanvasRenderingContext2DImpl::fast_move_to_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastMoveTo));


v8::CFunction CanvasRenderingContext2DImpl::fast_quadratic_curve_to_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastQuadraticCurveTo));


v8::CFunction CanvasRenderingContext2DImpl::fast_round_rect_array_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastRoundRectArray));

v8::CFunction CanvasRenderingContext2DImpl::fast_round_rect_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastRoundRect));

const v8::CFunction fast_round_rect_overloads_[] = {
        CanvasRenderingContext2DImpl::fast_round_rect_array_,
        CanvasRenderingContext2DImpl::fast_round_rect_
};


v8::CFunction CanvasRenderingContext2DImpl::fast_rect_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastRect));

v8::CFunction CanvasRenderingContext2DImpl::fast_scale_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastRect));

v8::CFunction CanvasRenderingContext2DImpl::fast_set_line_dash_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastSetLineDash));


v8::CFunction CanvasRenderingContext2DImpl::fast_set_transform_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastSetTransform));

v8::CFunction CanvasRenderingContext2DImpl::fast_set_transform_abcdef_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastSetTransformABCDEF));

const v8::CFunction fast_set_transform_overloads_[] = {
        CanvasRenderingContext2DImpl::fast_set_transform_abcdef_,
        CanvasRenderingContext2DImpl::fast_set_transform_
};

v8::CFunction CanvasRenderingContext2DImpl::fast_transform_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastTransform));


v8::CFunction CanvasRenderingContext2DImpl::fast_reset_transform_(
        v8::CFunction::Make(CanvasRenderingContext2DImpl::FastResetTransform));


CanvasRenderingContext2DImpl::CanvasRenderingContext2DImpl(
        CanvasRenderingContext2D *context) : context_(context) {

    auto ctx_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(this));
    auto raf_callback = new OnRafCallback(ctx_ptr, 0);
    auto raf_callback_ptr = reinterpret_cast<intptr_t>(reinterpret_cast<intptr_t *>(raf_callback));
    auto raf = canvas_native_raf_create(raf_callback_ptr, OnRafCallbackOnFrame);

    this->SetRaf(std::make_shared<RafImpl>(raf_callback, raf_callback_ptr, raf));

    auto _raf = this->GetRaf();

    if (_raf != nullptr) {
        canvas_native_raf_start(_raf->GetRaf());
    }

}


void CanvasRenderingContext2DImpl::Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    canvasModule->Set(context, ConvertToV8String(isolate, "CanvasRenderingContext2D"), func);
}

CanvasRenderingContext2DImpl *
CanvasRenderingContext2DImpl::GetPointer(const v8::Local<v8::Object> &object) {
    auto ptr = object->GetAlignedPointerFromInternalField(0);
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<CanvasRenderingContext2DImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> CanvasRenderingContext2DImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->CanvasRenderingContext2DTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
    ctorTmpl->SetClassName(ConvertToV8String(isolate, "CanvasRenderingContext2D"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(2);

    tmpl->Set(ConvertToV8String(isolate, "drawPoint"),
              v8::FunctionTemplate::New(isolate, &DrawPoint));
    tmpl->Set(ConvertToV8String(isolate, "drawPoints"),
              v8::FunctionTemplate::New(isolate, &DrawPoints));
    tmpl->Set(ConvertToV8String(isolate, "drawPaint"),
              v8::FunctionTemplate::New(isolate, &DrawPaint));


    tmpl->Set(ConvertToV8String(isolate, "__makeDirty"),
              v8::FunctionTemplate::New(isolate, __MakeDirty));
    tmpl->Set(ConvertToV8String(isolate, "__getPointer"),
              v8::FunctionTemplate::New(isolate, __GetPointer));
    tmpl->Set(ConvertToV8String(isolate, "__resize"), v8::FunctionTemplate::New(isolate, __Resize));

    tmpl->SetAccessor(ConvertToV8String(isolate, "filter"), GetFilter, SetFilter);
    tmpl->SetAccessor(ConvertToV8String(isolate, "font"), GetFont, SetFont);
    tmpl->SetAccessor(ConvertToV8String(isolate, "letterSpacing"), GetLetterSpacing,
                      SetLetterSpacing);
    tmpl->SetAccessor(ConvertToV8String(isolate, "wordSpacing"), GetWordSpacing, SetWordSpacing);

    tmpl->SetAccessor(ConvertToV8String(isolate, "globalAlpha"), GetGlobalAlpha, SetGlobalAlpha);
    tmpl->SetAccessor(ConvertToV8String(isolate, "imageSmoothingEnabled"), GetImageSmoothingEnabled,
                      SetImageSmoothingEnabled);
    tmpl->SetAccessor(ConvertToV8String(isolate, "imageSmoothingQuality"), GetImageSmoothingQuality,
                      SetImageSmoothingQuality);
    tmpl->SetAccessor(ConvertToV8String(isolate, "lineDashOffset"), GetLineDashOffset,
                      SetLineDashOffset);
    tmpl->SetAccessor(ConvertToV8String(isolate, "lineJoin"), GetLineJoin, SetLineJoin);
    tmpl->SetAccessor(ConvertToV8String(isolate, "lineCap"), GetLineCap, SetLineCap);
    tmpl->SetAccessor(ConvertToV8String(isolate, "miterLimit"), GetMiterLimit, SetMiterLimit);
    tmpl->SetAccessor(ConvertToV8String(isolate, "shadowColor"), GetShadowColor, SetShadowColor);
    tmpl->SetAccessor(ConvertToV8String(isolate, "shadowBlur"), GetShadowBlur, SetShadowBlur);
    tmpl->SetAccessor(ConvertToV8String(isolate, "shadowOffsetX"), GetShadowOffsetX,
                      SetShadowOffsetX);
    tmpl->SetAccessor(ConvertToV8String(isolate, "shadowOffsetY"), GetShadowOffsetY,
                      SetShadowOffsetY);
    tmpl->SetAccessor(ConvertToV8String(isolate, "textAlign"), GetTextAlign, SetTextAlign);
    tmpl->SetAccessor(ConvertToV8String(isolate, "textBaseline"), GetTextBaseline,
                      SetTextBaseline);
    tmpl->SetAccessor(ConvertToV8String(isolate, "globalCompositeOperation"),
                      GetGlobalCompositeOperation, SetGlobalCompositeOperation);
    tmpl->SetAccessor(ConvertToV8String(isolate, "fillStyle"), GetFillStyle, SetFillStyle);
    tmpl->SetAccessor(ConvertToV8String(isolate, "strokeStyle"), GetStrokeStyle, SetStrokeStyle);
    tmpl->SetAccessor(ConvertToV8String(isolate, "lineWidth"), GetLineWidth, SetLineWidth);


    tmpl->Set(ConvertToV8String(isolate, "addHitRegion"),
              v8::FunctionTemplate::New(isolate, &AddHitRegion));

    SetFastMethod(isolate, tmpl, "arc", Arc, &fast_arc_, v8::Local<v8::Value>());

    SetFastMethod(isolate, tmpl, "arcTo", ArcTo, &fast_arc_to_, v8::Local<v8::Value>());

    SetFastMethod(isolate, tmpl, "beginPath", BeginPath, &fast_begin_path_, v8::Local<v8::Value>());


    SetFastMethod(isolate, tmpl, "bezierCurveTo", BezierCurveTo, &fast_bezier_curve_to_,
                  v8::Local<v8::Value>());


    tmpl->Set(ConvertToV8String(isolate, "clearHitRegions"),
              v8::FunctionTemplate::New(isolate, &ClearHitRegions));


    SetFastMethod(isolate, tmpl, "clearRect", ClearRect, &fast_clear_rect_, v8::Local<v8::Value>());


    SetFastMethodWithOverLoads(isolate, tmpl, "clip", Clip, fast_clip_overloads_,
                               v8::Local<v8::Value>());

    SetFastMethod(isolate, tmpl, "closePath", ClosePath, &fast_close_path_, v8::Local<v8::Value>());


    tmpl->Set(ConvertToV8String(isolate, "createImageData"),
              v8::FunctionTemplate::New(isolate, &CreateImageData));
    tmpl->Set(ConvertToV8String(isolate, "createPattern"),
              v8::FunctionTemplate::New(isolate, &CreatePattern));
    tmpl->Set(ConvertToV8String(isolate, "createLinearGradient"),
              v8::FunctionTemplate::New(isolate, &CreateLinearGradient));
    tmpl->Set(ConvertToV8String(isolate, "createConicGradient"),
              v8::FunctionTemplate::New(isolate, &CreateConicGradient));
    tmpl->Set(ConvertToV8String(isolate, "__createPatternWithNative"),
              v8::FunctionTemplate::New(isolate, &__CreatePatternWithNative));
    tmpl->Set(ConvertToV8String(isolate, "createRadialGradient"),
              v8::FunctionTemplate::New(isolate, &CreateRadialGradient));
    tmpl->Set(ConvertToV8String(isolate, "drawFocusIfNeeded"),
              v8::FunctionTemplate::New(isolate, &DrawFocusIfNeeded));


    SetFastMethodWithOverLoads(isolate, tmpl, "drawImage", DrawImage, fast_draw_overloads_,
                               v8::Local<v8::Value>());

    SetFastMethod(isolate, tmpl, "ellipse", Ellipse, &fast_ellipse_, v8::Local<v8::Value>());

    SetFastMethodWithOverLoads(isolate, tmpl, "fill", Fill, fast_fill_overloads_,
                               v8::Local<v8::Value>());

    SetFastMethod(isolate, tmpl, "fillRect", FillRect, &fast_fill_rect_, v8::Local<v8::Value>());

    tmpl->Set(ConvertToV8String(isolate, "fillText"),
              v8::FunctionTemplate::New(isolate, &FillText));
    tmpl->Set(ConvertToV8String(isolate, "getImageData"),
              v8::FunctionTemplate::New(isolate, &GetImageData));
    tmpl->Set(ConvertToV8String(isolate, "getLineDash"),
              v8::FunctionTemplate::New(isolate, &GetLineDash));

    SetFastMethodWithOverLoads(isolate, tmpl, "isPointInPath", IsPointInPath,
                               fast_is_point_in_path_overloads_,
                               v8::Local<v8::Value>());


    SetFastMethodWithOverLoads(isolate, tmpl, "isPointInStroke", IsPointInStroke,
                               fast_is_point_in_stroke_overloads_,
                               v8::Local<v8::Value>());


    SetFastMethod(isolate, tmpl, "lineTo", LineTo, &fast_line_to_, v8::Local<v8::Value>());


    tmpl->Set(ConvertToV8String(isolate, "measureText"),
              v8::FunctionTemplate::New(isolate, &MeasureText));

    SetFastMethod(isolate, tmpl, "moveTo", MoveTo, &fast_move_to_, v8::Local<v8::Value>());


    SetFastMethodWithOverLoads(isolate, tmpl, "putImageData", PutImageData,
                               fast_put_image_data_overloads_,
                               v8::Local<v8::Value>());


    SetFastMethod(isolate, tmpl, "quadraticCurveTo", QuadraticCurveTo, &fast_quadratic_curve_to_,
                  v8::Local<v8::Value>());

    SetFastMethodWithOverLoads(isolate, tmpl, "roundRect", RoundRect, fast_round_rect_overloads_,
                               v8::Local<v8::Value>());

    SetFastMethod(isolate, tmpl, "rect", Rect, &fast_rect_, v8::Local<v8::Value>());


    tmpl->Set(ConvertToV8String(isolate, "removeHitRegion"),
              v8::FunctionTemplate::New(isolate, &RemoveHitRegion));


    SetFastMethod(isolate, tmpl, "resetTransform", ResetTransform, &fast_reset_transform_,
                  v8::Local<v8::Value>());

    SetFastMethod(isolate, tmpl, "restore", Restore, &fast_restore_, v8::Local<v8::Value>());

    SetFastMethod(isolate, tmpl, "rotate", Rotate, &fast_rotate_, v8::Local<v8::Value>());

    SetFastMethod(isolate, tmpl, "save", Save, &fast_save_, v8::Local<v8::Value>());

    SetFastMethod(isolate, tmpl, "scale", Scale, &fast_scale_, v8::Local<v8::Value>());


    tmpl->Set(ConvertToV8String(isolate, "scrollPathIntoView"),
              v8::FunctionTemplate::New(isolate, &ScrollPathIntoView));

    SetFastMethod(isolate, tmpl, "setLineDash", SetLineDash, &fast_set_line_dash_,
                  v8::Local<v8::Value>());

    SetFastMethodWithOverLoads(isolate, tmpl, "setTransform", SetTransform,
                               fast_set_transform_overloads_, v8::Local<v8::Value>());


    SetFastMethodWithOverLoads(isolate, tmpl, "stroke", Stroke, fast_stroke_overloads_,
                               v8::Local<v8::Value>());


    SetFastMethod(isolate, tmpl, "strokeRect", StrokeRect, &fast_stroke_rect_,
                  v8::Local<v8::Value>());


    tmpl->Set(ConvertToV8String(isolate, "strokeText"),
              v8::FunctionTemplate::New(isolate, &StrokeText));

    SetFastMethod(isolate, tmpl, "transform", Transform, &fast_transform_, v8::Local<v8::Value>());


    SetFastMethod(isolate, tmpl, "translate", Translate, &fast_translate_, v8::Local<v8::Value>());


    tmpl->Set(ConvertToV8String(isolate, "__toDataURL"),
              v8::FunctionTemplate::New(isolate, &__ToDataURL));

    cache->CanvasRenderingContext2DTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;
}

void CanvasRenderingContext2DImpl::SetRaf(std::shared_ptr<RafImpl> raf) {
    this->raf_ = std::move(raf);
}

RafImpl *CanvasRenderingContext2DImpl::GetRaf() {
    return this->raf_.get();
}

/* Non Standard 2D */

void CanvasRenderingContext2DImpl::DrawPoint(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto x = (float) args[0]->NumberValue(context).ToChecked();
    auto y = (float) args[1]->NumberValue(context).ToChecked();

    canvas_native_context_draw_point(
            ptr->GetContext(), x, y);
    ptr->UpdateInvalidateState();
}

void CanvasRenderingContext2DImpl::DrawPoints(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto mode = ConvertFromV8String(isolate, args[0]);
    auto points = args[1].As<v8::Array>();
    auto size = points->Length();


    if ((size % 2) == 0) {
        int32_t pointMode = -1;
        if (mode == "points") {
            pointMode = 0;
        } else if (mode == "lines") {
            pointMode = 1;
        } else if (mode == "polygon") {
            pointMode = 2;
        }
        if (pointMode == -1) {
            return;
        }
        std::vector<float> store;
        store.reserve(size);
        int next = 0;
        for (int i = 0; i < size; i++) {

            auto object = points->Get(
                    context, i).ToLocalChecked().As<v8::Object>();

            auto x = object->Get(context,
                                 ConvertToV8String(isolate, "x")).ToLocalChecked()->NumberValue(
                    context).ToChecked();
            auto y = object->Get(context,
                                 ConvertToV8String(isolate, "y")).ToLocalChecked()->NumberValue(
                    context).ToChecked();
            store[next] = (float) x;
            store[next + 1] = (float) y;

            next = i + 2;
        }

        canvas_native_context_draw_points(
                ptr->GetContext(), pointMode,
                store.data(), store.size());


        ptr->UpdateInvalidateState();
    }
}

void CanvasRenderingContext2DImpl::DrawPaint(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();


    auto color = ConvertFromV8String(isolate, args[0]);
    canvas_native_context_draw_paint(
            ptr->GetContext(), color.c_str());
    ptr->UpdateInvalidateState();
}

/* Non Standard 2D */


void CanvasRenderingContext2DImpl::__MakeDirty(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    ptr->UpdateInvalidateState();
}

void CanvasRenderingContext2DImpl::__GetPointer(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetEmptyString();
        return;
    }

    auto isolate = args.GetIsolate();

    auto pointer = (intptr_t *) ptr->GetContext();
    auto ret = std::to_string((intptr_t) pointer);
    args.GetReturnValue().Set(ConvertToV8String(isolate, ret));
}

void CanvasRenderingContext2DImpl::__Resize(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto width = args[0]->NumberValue(context).ToChecked();
    auto height = args[1]->NumberValue(context).ToChecked();
    canvas_native_context_resize(
            ptr->GetContext(),
            static_cast<float>(width),
            static_cast<float>(height));
}


void CanvasRenderingContext2DImpl::GetFilter(v8::Local<v8::String> property,
                                             const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetEmptyString();
        return;
    }
    auto isolate = info.GetIsolate();
    auto filter = canvas_native_context_get_filter(ptr->GetContext());
    info.GetReturnValue().Set(ConvertToV8String(isolate, filter));
    canvas_native_string_destroy((char *) filter);
}

void CanvasRenderingContext2DImpl::SetFilter(v8::Local<v8::String> property,
                                             v8::Local<v8::Value> value,
                                             const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto val = ConvertFromV8String(isolate, value);
    canvas_native_context_set_filter(ptr->GetContext(), val.c_str());
}

void CanvasRenderingContext2DImpl::GetFont(v8::Local<v8::String> property,
                                           const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetEmptyString();
        return;
    }
    auto isolate = info.GetIsolate();
    auto font = canvas_native_context_get_font(ptr->GetContext());
    info.GetReturnValue().Set(ConvertToV8String(isolate, font));

    canvas_native_string_destroy((char *) font);
}

void CanvasRenderingContext2DImpl::SetFont(v8::Local<v8::String> property,
                                           v8::Local<v8::Value> value,
                                           const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto val = ConvertFromV8String(isolate, value);
    canvas_native_context_set_font(ptr->GetContext(), val.c_str());
}


void CanvasRenderingContext2DImpl::GetLetterSpacing(v8::Local<v8::String> property,
                                                    const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetEmptyString();
        return;
    }
    auto isolate = info.GetIsolate();
    auto font = canvas_native_context_get_letter_spacing(ptr->GetContext());
    info.GetReturnValue().Set(ConvertToV8String(isolate, font));

    canvas_native_string_destroy((char *) font);
}

void CanvasRenderingContext2DImpl::SetLetterSpacing(v8::Local<v8::String> property,
                                                    v8::Local<v8::Value> value,
                                                    const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto val = ConvertFromV8String(isolate, value);
    canvas_native_context_set_letter_spacing(ptr->GetContext(), val.c_str());
}


void CanvasRenderingContext2DImpl::GetWordSpacing(v8::Local<v8::String> property,
                                                  const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetEmptyString();
        return;
    }
    auto isolate = info.GetIsolate();
    auto font = canvas_native_context_get_word_spacing(ptr->GetContext());
    info.GetReturnValue().Set(ConvertToV8String(isolate, font));

    canvas_native_string_destroy((char *) font);
}

void CanvasRenderingContext2DImpl::SetWordSpacing(v8::Local<v8::String> property,
                                                  v8::Local<v8::Value> value,
                                                  const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto val = ConvertFromV8String(isolate, value);
    canvas_native_context_set_word_spacing(ptr->GetContext(), val.c_str());
}


void CanvasRenderingContext2DImpl::GetGlobalAlpha(v8::Local<v8::String> property,
                                                  const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(1);
        return;
    }
    auto alpha = canvas_native_context_get_global_alpha(ptr->GetContext());
    info.GetReturnValue().Set((double) alpha);
}

void CanvasRenderingContext2DImpl::SetGlobalAlpha(v8::Local<v8::String> property,
                                                  v8::Local<v8::Value> value,
                                                  const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_context_set_global_alpha(ptr->GetContext(),
                                           (float) value->NumberValue(context).ToChecked());
}

void CanvasRenderingContext2DImpl::GetImageSmoothingEnabled(v8::Local<v8::String> property,
                                                            const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(false);
        return;
    }
    auto enabled = canvas_native_context_get_image_smoothing_enabled(ptr->GetContext());
    info.GetReturnValue().Set(enabled);
}

void CanvasRenderingContext2DImpl::SetImageSmoothingEnabled(v8::Local<v8::String> property,
                                                            v8::Local<v8::Value> value,
                                                            const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    canvas_native_context_set_image_smoothing_enabled(ptr->GetContext(),
                                                      value->BooleanValue(isolate));
}

void CanvasRenderingContext2DImpl::GetImageSmoothingQuality(v8::Local<v8::String> property,
                                                            const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetEmptyString();
        return;
    }
    auto isolate = info.GetIsolate();
    auto quality = canvas_native_context_get_image_smoothing_quality(ptr->GetContext());
    info.GetReturnValue().Set(
            ConvertToV8String(isolate, quality));

    canvas_native_string_destroy((char *) quality);

}

void CanvasRenderingContext2DImpl::SetImageSmoothingQuality(v8::Local<v8::String> property,
                                                            v8::Local<v8::Value> value,
                                                            const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto quality = ConvertFromV8String(isolate, value);
    canvas_native_context_set_image_smoothing_quality(ptr->GetContext(), quality.c_str());
}

void CanvasRenderingContext2DImpl::GetLineDashOffset(v8::Local<v8::String> property,
                                                     const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }
    auto offset = canvas_native_context_get_line_dash_offset(ptr->GetContext());
    info.GetReturnValue().Set((double) offset);
}

void CanvasRenderingContext2DImpl::SetLineDashOffset(v8::Local<v8::String> property,
                                                     v8::Local<v8::Value> value,
                                                     const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_context_set_line_dash_offset(ptr->GetContext(),
                                               (float) value->NumberValue(context).ToChecked());
}


void CanvasRenderingContext2DImpl::GetLineJoin(v8::Local<v8::String> property,
                                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }
    auto isolate = info.GetIsolate();
    auto join = canvas_native_context_get_line_join(ptr->GetContext());
    info.GetReturnValue().Set(ConvertToV8String(isolate, join));

    canvas_native_string_destroy((char *) join);
}

void CanvasRenderingContext2DImpl::SetLineJoin(v8::Local<v8::String> property,
                                               v8::Local<v8::Value> value,
                                               const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto join = ConvertFromV8String(isolate, value);
    canvas_native_context_set_line_join(ptr->GetContext(), join.c_str());
}

void CanvasRenderingContext2DImpl::GetLineCap(v8::Local<v8::String> property,
                                              const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }
    auto isolate = info.GetIsolate();
    auto cap = canvas_native_context_get_line_cap(ptr->GetContext());
    info.GetReturnValue().Set(ConvertToV8String(isolate, cap));

    canvas_native_string_destroy((char *) cap);
}

void CanvasRenderingContext2DImpl::SetLineCap(v8::Local<v8::String> property,
                                              v8::Local<v8::Value> value,
                                              const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto cap = ConvertFromV8String(isolate, value);
    canvas_native_context_set_line_cap(ptr->GetContext(), cap.c_str());
}


void CanvasRenderingContext2DImpl::GetMiterLimit(v8::Local<v8::String> property,
                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }
    auto limit = canvas_native_context_get_miter_limit(ptr->GetContext());
    info.GetReturnValue().Set((double) limit);
}

void CanvasRenderingContext2DImpl::SetMiterLimit(v8::Local<v8::String> property,
                                                 v8::Local<v8::Value> value,
                                                 const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_context_set_miter_limit(ptr->GetContext(),
                                          (float) value->NumberValue(context).ToChecked());
}


void CanvasRenderingContext2DImpl::GetShadowColor(v8::Local<v8::String> property,
                                                  const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }
    auto isolate = info.GetIsolate();
    auto color = canvas_native_context_get_shadow_color(ptr->GetContext());
    info.GetReturnValue().Set(ConvertToV8String(isolate, color));
    canvas_native_string_destroy((char *) color);
}

void CanvasRenderingContext2DImpl::SetShadowColor(v8::Local<v8::String> property,
                                                  v8::Local<v8::Value> value,
                                                  const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto color = ConvertFromV8String(isolate, value);
    canvas_native_context_set_shadow_color(ptr->GetContext(), color.c_str());
}


void CanvasRenderingContext2DImpl::GetShadowBlur(v8::Local<v8::String> property,
                                                 const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }
    auto blur = canvas_native_context_get_shadow_blur(ptr->GetContext());
    info.GetReturnValue().Set((double) blur);
}

void CanvasRenderingContext2DImpl::SetShadowBlur(v8::Local<v8::String> property,
                                                 v8::Local<v8::Value> value,
                                                 const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_context_set_shadow_blur(ptr->GetContext(),
                                          (float) value->NumberValue(context).ToChecked());
}

void CanvasRenderingContext2DImpl::GetShadowOffsetX(v8::Local<v8::String> property,
                                                    const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }
    auto x = canvas_native_context_get_shadow_offset_x(ptr->GetContext());
    info.GetReturnValue().Set((double) x);
}

void CanvasRenderingContext2DImpl::SetShadowOffsetX(v8::Local<v8::String> property,
                                                    v8::Local<v8::Value> value,
                                                    const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_context_set_shadow_offset_x(ptr->GetContext(),
                                              (float) value->NumberValue(context).ToChecked());
}


void CanvasRenderingContext2DImpl::GetShadowOffsetY(v8::Local<v8::String> property,
                                                    const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }
    auto y = canvas_native_context_get_shadow_offset_y(ptr->GetContext());
    info.GetReturnValue().Set((double) y);
}

void CanvasRenderingContext2DImpl::SetShadowOffsetY(v8::Local<v8::String> property,
                                                    v8::Local<v8::Value> value,
                                                    const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    canvas_native_context_set_shadow_offset_y(ptr->GetContext(),
                                              (float) value->NumberValue(context).ToChecked());
}


void CanvasRenderingContext2DImpl::GetTextAlign(v8::Local<v8::String> property,
                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }
    auto isolate = info.GetIsolate();
    auto alignment = canvas_native_context_get_text_align(ptr->GetContext());
    info.GetReturnValue().Set(
            ConvertToV8String(isolate, alignment));
    canvas_native_string_destroy((char *) alignment);
}

void CanvasRenderingContext2DImpl::SetTextAlign(v8::Local<v8::String> property,
                                                v8::Local<v8::Value> value,
                                                const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto alignment = ConvertFromV8String(isolate, value);
    canvas_native_context_set_text_align(ptr->GetContext(), alignment.c_str());
}

void CanvasRenderingContext2DImpl::GetTextBaseline(v8::Local<v8::String> property,
                                                   const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    auto isolate = info.GetIsolate();
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }
    auto baseline = canvas_native_context_get_text_baseline(ptr->GetContext());
    info.GetReturnValue().Set(
            ConvertToV8String(isolate, baseline));
    canvas_native_string_destroy((char *) baseline);
}

void CanvasRenderingContext2DImpl::SetTextBaseline(v8::Local<v8::String> property,
                                                   v8::Local<v8::Value> value,
                                                   const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {

        return;
    }
    auto isolate = info.GetIsolate();
    auto baseline = ConvertFromV8String(isolate, value);
    canvas_native_context_set_text_baseline(ptr->GetContext(), baseline.c_str());
}


void CanvasRenderingContext2DImpl::GetGlobalCompositeOperation(v8::Local<v8::String> property,
                                                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }
    auto operation = canvas_native_context_get_global_composition(ptr->GetContext());
    info.GetReturnValue().Set(
            ConvertToV8String(info.GetIsolate(), operation));

    canvas_native_string_destroy((char *) operation);
}

void CanvasRenderingContext2DImpl::SetGlobalCompositeOperation(v8::Local<v8::String> property,
                                                               v8::Local<v8::Value> value,
                                                               const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto operation = ConvertFromV8String(isolate, value);
    canvas_native_context_set_global_composition(ptr->GetContext(), operation.c_str());
}


void CanvasRenderingContext2DImpl::GetFillStyle(v8::Local<v8::String> property,
                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }
    auto isolate = info.GetIsolate();

    PaintStyleType type = canvas_native_context_get_current_fill_style_type(ptr->GetContext());

    switch (type) {
        case PaintStyleType::PaintStyleTypeColor: {
            auto color = canvas_native_paint_style_get_current_fill_color_string(
                    ptr->GetContext());


            auto value = new OneByteStringResource((char *) color);
            auto ret = v8::String::NewExternalOneByte(isolate, value);
            info.GetReturnValue().Set(ret.ToLocalChecked());

            //info.GetReturnValue().Set(ConvertToV8String(isolate, color.c_str()));
            break;
        }
        case PaintStyleType::PaintStyleTypeGradient: {
            auto style = CanvasGradient::NewInstance(isolate, new CanvasGradient(
                    canvas_native_context_get_fill_style(ptr->GetContext())));

            info.GetReturnValue().Set(style);

            break;
        }
        case PaintStyleType::PaintStyleTypePattern: {
            auto style = CanvasPattern::NewInstance(isolate, new CanvasPattern(
                    canvas_native_context_get_fill_style(ptr->GetContext())));

            info.GetReturnValue().Set(style);
            break;
        }
        case PaintStyleType::PaintStyleTypeNone: {
            info.GetReturnValue().SetUndefined();
            break;
        }
    }

}

void CanvasRenderingContext2DImpl::SetFillStyle(v8::Local<v8::String> property,
                                                v8::Local<v8::Value> value,
                                                const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();

    if (value->IsString()) {
        auto style = ConvertFromV8String(isolate, value);
        canvas_native_paint_style_set_fill_color_with_c_string(ptr->GetContext(),
                                                               style.c_str());
    } else if (value->IsObject()) {

        auto type = GetNativeType(value);

        switch (type) {
            case NativeType::CanvasGradient: {
                auto gradient = CanvasGradient::GetPointer(value.As<v8::Object>());
                canvas_native_context_set_fill_style(ptr->GetContext(),
                                                     gradient->GetPaintStyle());
            }
                break;
            case NativeType::CanvasPattern: {
                auto pattern = CanvasPattern::GetPointer(value.As<v8::Object>());
                canvas_native_context_set_fill_style(ptr->GetContext(),
                                                     pattern->GetPaintStyle());
            }
                break;
            default:
                // noop
                break;
        }
    }
}

void CanvasRenderingContext2DImpl::GetStrokeStyle(v8::Local<v8::String> property,
                                                  const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().SetUndefined();
        return;
    }
    auto isolate = info.GetIsolate();

    PaintStyleType type = canvas_native_context_get_current_stroke_style_type(ptr->GetContext());

    switch (type) {
        case PaintStyleType::PaintStyleTypeColor: {
            auto color = canvas_native_paint_style_get_current_stroke_color_string(
                    ptr->GetContext());

            auto value = new OneByteStringResource((char *) color);
            auto ret = v8::String::NewExternalOneByte(isolate, value);
            info.GetReturnValue().Set(ret.ToLocalChecked());

            //info.GetReturnValue().Set(ConvertToV8String(isolate, color.c_str()));
            break;
        }
        case PaintStyleType::PaintStyleTypeGradient: {
            auto style = CanvasGradient::NewInstance(isolate, new CanvasGradient(
                    canvas_native_context_get_stroke_style(ptr->GetContext())));

            info.GetReturnValue().Set(style);

            break;
        }
        case PaintStyleType::PaintStyleTypePattern: {
            auto style = CanvasPattern::NewInstance(isolate, new CanvasPattern(
                    canvas_native_context_get_stroke_style(ptr->GetContext())));

            info.GetReturnValue().Set(style);
            break;
        }
        case PaintStyleType::PaintStyleTypeNone: {
            info.GetReturnValue().SetUndefined();
            break;
        }
    }

}


void CanvasRenderingContext2DImpl::SetStrokeStyle(v8::Local<v8::String> property,
                                                  v8::Local<v8::Value> value,
                                                  const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();

    if (value->IsString()) {
        auto style = ConvertFromV8String(isolate, value);
        canvas_native_paint_style_set_stroke_color_with_c_string(ptr->GetContext(),
                                                                 style.c_str());
    } else if (value->IsObject()) {

        auto type = GetNativeType(value);

        switch (type) {
            case NativeType::CanvasGradient: {
                auto gradient = CanvasGradient::GetPointer(value.As<v8::Object>());
                canvas_native_context_set_stroke_style(ptr->GetContext(),
                                                       gradient->GetPaintStyle());
            }
                break;
            case NativeType::CanvasPattern: {
                auto pattern = CanvasPattern::GetPointer(value.As<v8::Object>());
                canvas_native_context_set_stroke_style(ptr->GetContext(),
                                                       pattern->GetPaintStyle());
            }
                break;
            default:
                break;
        }
    }
}


void CanvasRenderingContext2DImpl::GetLineWidth(v8::Local<v8::String> property,
                                                const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }

    auto lineWidth = canvas_native_context_get_line_width(ptr->GetContext());

    info.GetReturnValue().Set((double) lineWidth);

}


void CanvasRenderingContext2DImpl::SetLineWidth(v8::Local<v8::String> property,
                                                v8::Local<v8::Value> value,
                                                const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto lineWidth = (float) value->NumberValue(context).ToChecked();
    canvas_native_context_set_line_width(ptr->GetContext(), lineWidth);
}

void CanvasRenderingContext2DImpl::GetLineDash(v8::Local<v8::String> property,
                                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        info.GetReturnValue().Set(0);
        return;
    }

    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto lineDash = canvas_native_context_get_line_dash(ptr->GetContext());
    auto buf = canvas_native_f32_buffer_get_bytes(lineDash);
    auto size = canvas_native_f32_buffer_get_length(lineDash);
    auto ret = v8::Array::New(isolate, (int) size);
    for (int i = 0; i < size; i++) {
        auto item = buf[i];
        ret->Set(context, i, v8::Number::New(isolate, (double) item));
    }

}


void CanvasRenderingContext2DImpl::SetLineDash(v8::Local<v8::String> property,
                                               v8::Local<v8::Value> value,
                                               const v8::PropertyCallbackInfo<void> &info) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(info.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (!value->IsNullOrUndefined() && value->IsObject()) {
        auto lineDashObject = value.As<v8::Object>();
        if (lineDashObject->IsArray()) {
            auto lineDash = lineDashObject.As<v8::Array>();
            auto size = lineDash->Length();
            std::vector<float> dash;
            dash.reserve(size);
            for (int i = 0; i < size; ++i) {
                auto val = lineDash->Get(context, i).ToLocalChecked();
                dash[i] = (float) val->NumberValue(context).ToChecked();
            }
            canvas_native_context_set_line_dash(ptr->GetContext(), dash.data(),
                                                dash.size());
        }
    }
}

void CanvasRenderingContext2DImpl::AddHitRegion(const v8::FunctionCallbackInfo<v8::Value> &args) {
    // todo
    //    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    //    if (ptr == nullptr) {
    //        return;
    //    }
}

void CanvasRenderingContext2DImpl::Arc(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

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
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

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
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_context_begin_path(
            ptr->GetContext()
    );
}

void CanvasRenderingContext2DImpl::BezierCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

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
    // TODO
    //    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    //    if (ptr == nullptr) {
    //        return;
    //    }
}

void CanvasRenderingContext2DImpl::ClearRect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

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
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto count = args.Length();
    if (count == 0) {
        canvas_native_context_clip_rule(
                ptr->GetContext(), "nonzero");
    } else if (count == 1 && args[0]->IsInt32()) {
        auto val = args[0]->Int32Value(context);
        switch (val.ToChecked()) {
            case 0:
                canvas_native_context_clip_rule(
                        ptr->GetContext(), "nonzero");
                break;
            case 1:
                canvas_native_context_clip_rule(
                        ptr->GetContext(), "evenodd");
                break;
        }
    } else if (count == 1 && args[0]->IsObject()) {
        auto type = GetNativeType(args[0].As<v8::Object>());
        if (type == NativeType::Path2D) {
            auto object = Path2D::GetPointer(args[0].As<v8::Object>());

            if (object != nullptr) {
                canvas_native_context_clip(
                        ptr->GetContext(), object->GetPath(), "nonzero");
            }
        }
    } else if (count >= 2 && args[0]->IsObject() && args[1]->IsInt32()) {
        auto type = GetNativeType(args[0].As<v8::Object>());
        if (type == NativeType::Path2D) {
            auto object = Path2D::GetPointer(args[0].As<v8::Object>());
            auto rule = args[1]->Int32Value(context).ToChecked();

            if (object != nullptr) {
                switch (rule) {
                    case 0:
                        canvas_native_context_clip(
                                ptr->GetContext(), object->GetPath(), "nonzero");
                        break;
                    case 1:
                        canvas_native_context_clip(
                                ptr->GetContext(), object->GetPath(), "evenodd");
                        break;
                    default:
                        break;
                }

            }
        }
    }
}

void CanvasRenderingContext2DImpl::ClosePath(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_context_close_path(
            ptr->GetContext());
}

void
CanvasRenderingContext2DImpl::CreateImageData(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto count = args.Length();
    auto value = args[0];
    if (count == 1 && value->IsObject()) {

        auto typeValue = GetNativeType(value);

        if (typeValue == NativeType::ImageData) {

            auto object = ImageDataImpl::GetPointer(value.As<v8::Object>());

            auto width = canvas_native_image_data_get_width(
                    object->GetImageData());
            auto height = canvas_native_image_data_get_height(
                    object->GetImageData());
            auto data = new ImageDataImpl(
                    canvas_native_image_data_create(
                            width, height));
            auto ret = ImageDataImpl::GetCtor(isolate)->GetFunction(
                    context).ToLocalChecked()->NewInstance(context).ToLocalChecked();


            ret->SetAlignedPointerInInternalField(0, data);

            SetNativeType(ret, NativeType::ImageData);

            args.GetReturnValue().Set(ret);

            return;
        }

    } else if (count > 1) {

        auto width = (int) args[0]->NumberValue(context).ToChecked();
        auto height = (int) args[1]->NumberValue(context).ToChecked();
        auto data = new ImageDataImpl(
                canvas_native_image_data_create(
                        width, height));

        auto ret = ImageDataImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();

        ret->SetAlignedPointerInInternalField(0, data);

        SetNativeType(ret, NativeType::ImageData);

        args.GetReturnValue().Set(ret);
    }

    args.GetReturnValue().SetUndefined();
}

void
CanvasRenderingContext2DImpl::CreatePattern(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }
    auto isolate = args.GetIsolate();

    if (args.Length() > 1) {
        auto value = args[0];
        auto valueType = GetNativeType(value);

        switch (valueType) {
            case NativeType::ImageAsset: {
                auto image_asset = ImageAssetImpl::GetPointer(value.As<v8::Object>());
                if (image_asset != nullptr) {
                    auto rep = ConvertFromV8String(isolate, args[1]);
                    auto pattern = canvas_native_context_create_pattern_asset(
                            ptr->GetContext(),
                            image_asset->GetImageAsset(), rep.c_str());
                    auto style_type = canvas_native_context_get_style_type(pattern);
                    if (style_type ==
                        PaintStyleType::PaintStyleTypeNone) {
                        args.GetReturnValue().SetUndefined();
                        canvas_native_paint_style_destroy(pattern);
                    } else {
                        auto data = CanvasPattern::NewInstance(isolate, new CanvasPattern(pattern));
                        args.GetReturnValue().Set(data);
                    }
                }
                return;
            }
            case NativeType::ImageBitmap: {
                auto image_bitmap = ImageBitmapImpl::GetPointer(value.As<v8::Object>());
                if (image_bitmap != nullptr) {
                    auto rep = ConvertFromV8String(isolate, args[1]);
                    auto pattern = canvas_native_context_create_pattern_asset(
                            ptr->GetContext(),
                            image_bitmap->GetImageAsset(), rep.c_str());
                    auto style_type = canvas_native_context_get_style_type(pattern);
                    if (style_type ==
                        PaintStyleType::PaintStyleTypeNone) {
                        args.GetReturnValue().SetUndefined();
                        canvas_native_paint_style_destroy(pattern);
                    } else {
                        auto data = CanvasPattern::NewInstance(isolate, new CanvasPattern(pattern));
                        args.GetReturnValue().Set(data);
                    }
                }
                return;
            }
            case NativeType::CanvasRenderingContext2D: {
                auto canvas_2d = CanvasRenderingContext2DImpl::GetPointer(
                        value.As<v8::Object>());
                if (canvas_2d != nullptr) {
                    auto rep = ConvertFromV8String(isolate, args[1]);
                    auto pattern = canvas_native_context_create_pattern_canvas2d(
                            canvas_2d->GetContext(),
                            ptr->GetContext(), rep.c_str());
                    auto style_type = canvas_native_context_get_style_type(pattern);
                    if (style_type ==
                        PaintStyleType::PaintStyleTypeNone) {
                        args.GetReturnValue().SetUndefined();
                        canvas_native_paint_style_destroy(pattern);
                    } else {
                        auto data = CanvasPattern::NewInstance(isolate, new CanvasPattern(pattern));
                        args.GetReturnValue().Set(data);
                    }
                }
                return;
            }
            case NativeType::WebGLRenderingContextBase: {
                auto webgl = WebGLRenderingContextBase::GetPointer(value.As<v8::Object>());
                if (webgl != nullptr) {
                    auto rep = ConvertFromV8String(isolate, args[1]);
                    auto pattern = canvas_native_context_create_pattern_webgl(
                            webgl->GetState(),
                            ptr->GetContext(), rep.c_str());
                    auto type = canvas_native_context_get_style_type(pattern);
                    if (type ==
                        PaintStyleType::PaintStyleTypeNone) {
                        args.GetReturnValue().SetNull();
                        canvas_native_paint_style_destroy(pattern);
                        return;
                    } else {
                        auto ret = CanvasPattern::NewInstance(isolate, new CanvasPattern(
                                pattern));
                        args.GetReturnValue().Set(ret);
                        return;
                    }
                }
                return;
            }
            default:
                args.GetReturnValue().SetNull();
                return;
        }

        return;
    }

    args.GetReturnValue().SetNull();
}

void
CanvasRenderingContext2DImpl::CreateLinearGradient(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() == 4) {
        auto x0 = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y0 = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto x1 = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto y1 = static_cast<float>(args[3]->NumberValue(context).ToChecked());

        auto gradient = canvas_native_context_create_linear_gradient(
                ptr->GetContext(), x0, y0, x1,
                y1);

        auto data = CanvasGradient::NewInstance(isolate, new CanvasGradient(
                gradient));

        args.GetReturnValue().Set(data);

        return;
    }

    args.GetReturnValue().SetNull();
}


void CanvasRenderingContext2DImpl::CreateConicGradient(
        const v8::FunctionCallbackInfo<v8::Value> &args) {

    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() == 3) {

        auto startAngle = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto x = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[2]->NumberValue(context).ToChecked());


        auto gradient = canvas_native_context_create_conic_gradient(
                ptr->GetContext(), startAngle, x, y);

        auto data = CanvasGradient::NewInstance(isolate, new CanvasGradient(
                gradient));

        args.GetReturnValue().Set(data);

        return;
    }

    args.GetReturnValue().SetNull();
}

void
CanvasRenderingContext2DImpl::__CreatePatternWithNative(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto pointer = args[0]->ToBigInt(context).ToLocalChecked();

    auto pattern = canvas_native_pattern_from_ptr(pointer->Int64Value());
    auto type = canvas_native_context_get_style_type(pattern);
    if (type == PaintStyleType::PaintStyleTypeNone) {
        args.GetReturnValue().SetUndefined();
        canvas_native_paint_style_destroy(pattern);
    } else {
        auto data = CanvasPattern::NewInstance(isolate, new CanvasPattern(pattern));

        args.GetReturnValue().Set(data);
    }

}

void
CanvasRenderingContext2DImpl::CreateRadialGradient(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetNull();
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() == 6) {
        auto x0 = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y0 = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto r0 = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto x1 = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto y1 = static_cast<float>(args[4]->NumberValue(context).ToChecked());
        auto r1 = static_cast<float>(args[5]->NumberValue(context).ToChecked());

        auto gradient = canvas_native_context_create_radial_gradient(
                ptr->GetContext(), x0, y0, r0,
                x1, y1, r1);

        auto data = CanvasGradient::NewInstance(isolate, new CanvasGradient(gradient));

        args.GetReturnValue().Set(data);
        return;
    }

    args.GetReturnValue().SetNull();
}


void
CanvasRenderingContext2DImpl::DrawFocusIfNeeded(const v8::FunctionCallbackInfo<v8::Value> &args) {
    // TODO
    //    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    //    if (ptr == nullptr) {
    //        return;
    //    }
}


void
CanvasRenderingContext2DImpl::DrawImage(const v8::FunctionCallbackInfo<v8::Value> &args) {

    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    auto count = args.Length();
    auto value = args[0];

    if (value->IsNullOrUndefined() || !value->IsObject()) {
        return;
    }

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto image = value.As<v8::Object>();
    auto imageType = GetNativeType(image);
    if (count == 3) {
        auto dx = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto dy = static_cast<float>(args[2]->NumberValue(context).ToChecked());

        switch (imageType) {
            case NativeType::ImageAsset: {
                auto image_asset = ImageAssetImpl::GetPointer(image);

                if (image_asset != nullptr) {
                    canvas_native_context_draw_image_dx_dy_asset(
                            ptr->GetContext(),
                            image_asset->GetImageAsset(),
                            dx, dy);
                    ptr->UpdateInvalidateState();
                }
            }
                break;
            case NativeType::ImageBitmap: {
                auto image_bitmap = ImageBitmapImpl::GetPointer(image);
                if (image_bitmap != nullptr) {
                    canvas_native_context_draw_image_dx_dy_asset(
                            ptr->GetContext(),
                            image_bitmap->GetImageAsset(),
                            dx, dy);
                    ptr->UpdateInvalidateState();
                }
            }
                break;
            case NativeType::CanvasRenderingContext2D: {
                auto image_canvas = CanvasRenderingContext2DImpl::GetPointer(image);
                if (image_canvas != nullptr) {
                    canvas_native_context_draw_image_dx_dy_context(
                            ptr->GetContext(),
                            image_canvas->GetContext(),
                            dx, dy);
                    ptr->UpdateInvalidateState();
                }
            }
                break;
            default:
                break;
        }
    } else if (count == 5) {
        auto dx = (float) args[1]->NumberValue(context).ToChecked();
        auto dy = (float) args[2]->NumberValue(context).ToChecked();
        auto dWidth = (float) args[3]->NumberValue(context).ToChecked();
        auto dHeight = (float) args[4]->NumberValue(context).ToChecked();

        switch (imageType) {
            case NativeType::ImageAsset: {
                auto image_asset = ImageAssetImpl::GetPointer(image);

                if (image_asset != nullptr) {
                    canvas_native_context_draw_image_dx_dy_dw_dh_asset(
                            ptr->GetContext(),
                            image_asset->GetImageAsset(),
                            dx, dy,
                            dWidth,
                            dHeight);
                    ptr->UpdateInvalidateState();
                }
            }
                break;
            case NativeType::ImageBitmap: {
                auto image_bitmap = ImageBitmapImpl::GetPointer(image);
                if (image_bitmap != nullptr) {
                    canvas_native_context_draw_image_dx_dy_dw_dh_asset(
                            ptr->GetContext(),
                            image_bitmap->GetImageAsset(),
                            dx, dy,
                            dWidth,
                            dHeight);
                    ptr->UpdateInvalidateState();
                }
            }
                break;
            case NativeType::CanvasRenderingContext2D: {
                auto image_canvas = CanvasRenderingContext2DImpl::GetPointer(image);
                if (image_canvas != nullptr) {
                    canvas_native_context_draw_image_dx_dy_dw_dh_context(
                            ptr->GetContext(),
                            image_canvas->GetContext(),
                            dx, dy,
                            dWidth,
                            dHeight);
                    ptr->UpdateInvalidateState();
                }
            }
                break;
            default:
                break;
        }

    } else if (count == 9) {
        auto sx = (float) args[1]->NumberValue(context).ToChecked();
        auto sy = (float) args[2]->NumberValue(context).ToChecked();
        auto sWidth = (float) args[3]->NumberValue(context).ToChecked();
        auto sHeight = (float) args[4]->NumberValue(context).ToChecked();
        auto dx = (float) args[5]->NumberValue(context).ToChecked();
        auto dy = (float) args[6]->NumberValue(context).ToChecked();
        auto dWidth = (float) args[7]->NumberValue(context).ToChecked();
        auto dHeight = (float) args[8]->NumberValue(context).ToChecked();

        switch (imageType) {
            case NativeType::ImageAsset: {
                auto image_asset = ImageAssetImpl::GetPointer(image);

                if (image_asset != nullptr) {
                    canvas_native_context_draw_image_asset(
                            ptr->GetContext(),
                            image_asset->GetImageAsset(),
                            sx,
                            sy, sWidth, sHeight,
                            dx,
                            dy, dWidth, dHeight);
                    ptr->UpdateInvalidateState();
                }
            }
                break;
            case NativeType::ImageBitmap: {
                auto image_bitmap = ImageBitmapImpl::GetPointer(image);
                if (image_bitmap != nullptr) {
                    canvas_native_context_draw_image_asset(
                            ptr->GetContext(),
                            image_bitmap->GetImageAsset(),
                            sx,
                            sy, sWidth, sHeight,
                            dx,
                            dy, dWidth, dHeight);
                    ptr->UpdateInvalidateState();
                }
            }
                break;
            case NativeType::CanvasRenderingContext2D: {
                auto image_canvas = CanvasRenderingContext2DImpl::GetPointer(image);
                if (image_canvas != nullptr) {
                    canvas_native_context_draw_image_context(
                            ptr->GetContext(),
                            image_canvas->GetContext(),
                            sx,
                            sy, sWidth, sHeight,
                            dx,
                            dy, dWidth, dHeight);
                    ptr->UpdateInvalidateState();
                }
            }
                break;
            default:
                break;
        }
    }
}


void
CanvasRenderingContext2DImpl::Ellipse(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

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
        canvas_native_context_ellipse(
                ptr->GetContext(), x, y,
                radiusX,
                radiusY, rotation,
                startAngle, endAngle,
                anticlockwise);
    }
}

void
CanvasRenderingContext2DImpl::Fill(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto count = args.Length();
    auto value = args[0];
    if (count == 2) {
        auto type = GetNativeType(value.As<v8::Object>());
        if (type == NativeType::Path2D) {
            auto object = Path2D::GetPointer(value.As<v8::Object>());

            if (object != nullptr) {
                auto rule = args[1]->Int32Value(context).ToChecked();
                switch (rule) {
                    case 0:
                        canvas_native_context_fill_with_path(
                                ptr->GetContext(),
                                object->GetPath(),
                                "nonzero");
                        ptr->UpdateInvalidateState();
                        break;
                    case 1:
                        canvas_native_context_fill_with_path(
                                ptr->GetContext(),
                                object->GetPath(),
                                "evenodd");
                        ptr->UpdateInvalidateState();
                        break;
                    default:
                        break;
                }

            }
        }
    } else if (count == 1) {
        if (value->IsInt32()) {
            auto rule = args[1]->Int32Value(context).ToChecked();
            switch (rule) {
                case 0:
                    canvas_native_context_fill(
                            ptr->GetContext(),
                            "nonzero");
                    ptr->UpdateInvalidateState();
                    break;
                case 1:
                    canvas_native_context_fill(
                            ptr->GetContext(),
                            "evenodd");
                    ptr->UpdateInvalidateState();
                    break;
                default:
                    break;
            }
        } else if (value->IsObject()) {
            auto type = GetNativeType(value.As<v8::Object>());
            if (type == NativeType::Path2D) {
                auto object = Path2D::GetPointer(value.As<v8::Object>());

                canvas_native_context_fill_with_path(
                        ptr->GetContext(),
                        object->GetPath(), "nonzero");
                ptr->UpdateInvalidateState();
            }
        }
    } else {
        canvas_native_context_fill(
                ptr->GetContext(), "nonzero");
        ptr->UpdateInvalidateState();
    }
}

void
CanvasRenderingContext2DImpl::FillRect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
    auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
    auto width = static_cast<float>(args[2]->NumberValue(context).ToChecked());
    auto height = static_cast<float>(args[3]->NumberValue(context).ToChecked());
    canvas_native_context_fill_rect(
            ptr->GetContext(), x, y, width,
            height);
    ptr->UpdateInvalidateState();
}

void
CanvasRenderingContext2DImpl::FillText(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto text = ConvertFromV8String(isolate, args[0]);

    auto x = static_cast<float>(args[1]->NumberValue(context).ToChecked());
    auto y = static_cast<float>(args[2]->NumberValue(context).ToChecked());

    if (args[3]->IsNumber()) {
        float width = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        canvas_native_context_fill_text_width(
                ptr->GetContext(), text.c_str(), x,
                y, width);
    } else {
        canvas_native_context_fill_text(
                ptr->GetContext(), text.c_str(), x,
                y);
    }

    ptr->UpdateInvalidateState();

}

void
CanvasRenderingContext2DImpl::GetImageData(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        args.GetReturnValue().SetUndefined();
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() == 4) {
        auto sx = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto sy = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto sw = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto sh = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto imageData = canvas_native_context_get_image_data(
                ptr->GetContext(), sx, sy, sw,
                sh);

        auto data = new ImageDataImpl(imageData);

        auto ret = ImageDataImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();

        ret->SetAlignedPointerInInternalField(0, data);

        SetNativeType(ret, NativeType::ImageData);

        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetUndefined();


}

void
CanvasRenderingContext2DImpl::GetLineDash(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();

    if (ptr == nullptr) {
        auto ret = v8::Array::New(isolate);
        args.GetReturnValue().Set(ret);
        return;
    }
    auto context = isolate->GetCurrentContext();


    auto dash = canvas_native_context_get_line_dash(
            ptr->GetContext());
    auto buf = canvas_native_f32_buffer_get_bytes(dash);
    auto size = canvas_native_f32_buffer_get_length(dash);
    auto array = v8::Array::New(isolate, (int) size);
    for (int i = 0; i < size; ++i) {
        array->Set(context, i,
                   v8::Number::New(isolate,
                                   (double) buf[i]));
    }

    args.GetReturnValue().Set(array);
}

void
CanvasRenderingContext2DImpl::IsPointInPath(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto count = args.Length();

    if (count == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto ret = canvas_native_context_is_point_in_path(
                ptr->GetContext(), x, y, "nonzero");
        args.GetReturnValue().Set(ret);
        return;
    } else if (count == 3 &&
               args[2]->IsInt32()) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto rule = args[2]->Int32Value(context).ToChecked();
        bool ret = false;
        switch (rule) {
            case 0:
                ret = canvas_native_context_is_point_in_path(
                        ptr->GetContext(), x, y, "nonzero");
                break;
            case 1:
                ret = canvas_native_context_is_point_in_path(
                        ptr->GetContext(), x, y, "evenodd");
                break;
        }
        args.GetReturnValue().Set(ret);
        return;
    } else if (count == 4 &&
               args[0]->IsObject() &&
               args[3]->IsInt32()) {

        auto value = args[0];
        auto type = GetNativeType(value);

        if (type == NativeType::Path2D) {
            auto path = Path2D::GetPointer(value.As<v8::Object>());
            auto x = static_cast<float>(args[1]->NumberValue(context).ToChecked());
            auto y = static_cast<float>(args[2]->NumberValue(context).ToChecked());
            auto rule = args[3]->Int32Value(context).ToChecked();
            bool ret = false;

            if (path != nullptr) {

                switch (rule) {
                    case 0:
                        ret = canvas_native_context_is_point_in_path_with_path(
                                ptr->GetContext(),
                                path->GetPath(), x, y, "nonzero");
                        break;
                    case 1:
                        ret = canvas_native_context_is_point_in_path_with_path(
                                ptr->GetContext(),
                                path->GetPath(), x, y, "evenodd");
                        break;
                }

                args.GetReturnValue().Set(ret);
                return;
            }
        }

    }


    args.GetReturnValue().Set(false);
}

void
CanvasRenderingContext2DImpl::IsPointInStroke(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto count = args.Length();


    if (count == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto ret = canvas_native_context_is_point_in_stroke(
                ptr->GetContext(), x, y);

        args.GetReturnValue().Set(ret);
        return;
    } else if (count == 3 &&
               args[0]->IsObject()) {


        auto value = args[0];
        auto type = GetNativeType(value);

        if (type == NativeType::Path2D) {
            auto path = Path2D::GetPointer(value.As<v8::Object>());
            auto x = static_cast<float>(args[1]->NumberValue(context).ToChecked());
            auto y = static_cast<float>(args[2]->NumberValue(context).ToChecked());


            if (path != nullptr) {
                auto ret = canvas_native_context_is_point_in_stroke_with_path(
                        ptr->GetContext(),
                        path->GetPath(), x, y);
                args.GetReturnValue().Set(ret);
                return;
            }
        }
    }

    args.GetReturnValue().Set(false);
}

void
CanvasRenderingContext2DImpl::LineTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        canvas_native_context_line_to(
                ptr->GetContext(), x, y);
    }
}

void
CanvasRenderingContext2DImpl::MeasureText(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto text = ConvertFromV8String(isolate, args[0]);

    auto metrics = canvas_native_context_measure_text(
            ptr->GetContext(), text.c_str());

    auto data = new TextMetricsImpl(metrics);

    auto ret = TextMetricsImpl::GetCtor(isolate)->GetFunction(
            context).ToLocalChecked()->NewInstance(context).ToLocalChecked();

    ret->SetAlignedPointerInInternalField(0, data);

    data->BindFinalizer(isolate, ret);

    SetNativeType(ret, NativeType::TextMetrics);

    args.GetReturnValue().Set(ret);

}

void
CanvasRenderingContext2DImpl::MoveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() > 1) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        canvas_native_context_move_to(
                ptr->GetContext(), x, y);
    }
}

void
CanvasRenderingContext2DImpl::PutImageData(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    auto value = args[0];
    auto typeValue = GetPrivateValue(isolate, value.As<v8::Object>(),
                                     ConvertToV8String(isolate, "__type"));
    auto count = args.Length();
    if (!typeValue.IsEmpty()) {
        auto type = (NativeType) typeValue->Int32Value(context).ToChecked();
        if (type == NativeType::ImageData) {


            auto imageData = ImageDataImpl::GetPointer(value.As<v8::Object>());
            if (count == 3) {
                auto dx = static_cast<float>(args[1]->NumberValue(context).ToChecked());
                auto dy = static_cast<float>(args[2]->NumberValue(context).ToChecked());
                float dirtyX = 0;
                float dirtyY = 0;
                auto dirtyWidth = (float) canvas_native_image_data_get_width(
                        imageData->GetImageData());
                auto dirtyHeight = (float) canvas_native_image_data_get_height(
                        imageData->GetImageData());
                canvas_native_context_put_image_data(
                        ptr->GetContext(),
                        imageData->GetImageData(), dx,
                        dy, dirtyX, dirtyY,
                        dirtyWidth, dirtyHeight);
                ptr->UpdateInvalidateState();
            } else if (count == 7) {
                auto dx = static_cast<float>(args[1]->NumberValue(context).ToChecked());
                auto dy = static_cast<float>(args[2]->NumberValue(context).ToChecked());
                auto dirtyX = static_cast<float>(args[3]->NumberValue(context).ToChecked());
                auto dirtyY = static_cast<float>(args[4]->NumberValue(context).ToChecked());
                auto dirtyWidth = static_cast<float>(args[5]->NumberValue(context).ToChecked());
                auto dirtyHeight = static_cast<float>(args[6]->NumberValue(context).ToChecked());
                canvas_native_context_put_image_data(
                        ptr->GetContext(),
                        imageData->GetImageData(), dx,
                        dy, dirtyX, dirtyY,
                        dirtyWidth, dirtyHeight);
                ptr->UpdateInvalidateState();
            }
        }
    }

}

void
CanvasRenderingContext2DImpl::QuadraticCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() == 4) {
        auto cpx = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto cpy = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto x = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        canvas_native_context_quadratic_curve_to(
                ptr->GetContext(), cpx, cpy,
                x, y);
    }

}

void
CanvasRenderingContext2DImpl::RoundRect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() == 5) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto width = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto height = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        if (args[4]->IsObject()) {
            auto radii = args[4];
            if (radii->IsArray()) {
                auto array = radii.As<v8::Array>();
                auto size = array->Length();
                if (size > 1) {
                    std::vector<float> store;
                    store.reserve(size);
                    for (int i = 0;
                         i < size; i++) {
                        store[i] = (float) array->Get(
                                context,
                                i).ToLocalChecked()->NumberValue(context).ToChecked();
                    }


                    canvas_native_context_round_rect(
                            ptr->GetContext(),
                            x, y,
                            width,
                            height, store.data(), store.size());

                }
            }
        } else {
            auto radii = (float) args[4]->NumberValue(context).ToChecked();
            canvas_native_context_round_rect_tl_tr_br_bl(
                    ptr->GetContext(), x, y,
                    width,
                    height, radii, radii,
                    radii, radii);

        }
    }

}

void
CanvasRenderingContext2DImpl::Rect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() == 4) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto width = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto height = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        canvas_native_context_rect(
                ptr->GetContext(), x, y,
                width,
                height);
    }

}

void
CanvasRenderingContext2DImpl::RemoveHitRegion(const v8::FunctionCallbackInfo<v8::Value> &args) {
    // CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
}

void
CanvasRenderingContext2DImpl::ResetTransform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_context_reset_transform(
            ptr->GetContext());
}

void
CanvasRenderingContext2DImpl::Restore(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_context_restore(
            ptr->GetContext());
}

void
CanvasRenderingContext2DImpl::Rotate(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];

    if (args.Length() == 1 && value->IsNumber()) {
        canvas_native_context_rotate(
                ptr->GetContext(),
                (float) args[0]->NumberValue(context).ToChecked());
    }
}

void
CanvasRenderingContext2DImpl::Save(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }

    canvas_native_context_save(
            ptr->GetContext());
}

void
CanvasRenderingContext2DImpl::Scale(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        canvas_native_context_scale(
                ptr->GetContext(), x, y);
    }
}

void
CanvasRenderingContext2DImpl::ScrollPathIntoView(const v8::FunctionCallbackInfo<v8::Value> &args) {
    // CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
}

void
CanvasRenderingContext2DImpl::SetLineDash(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];

    if (value->IsArray()) {
        auto segments = value.As<v8::Array>();
        auto len = segments->Length();
        std::vector<float> data;
        for (int i = 0; i < len; ++i) {
            auto item = segments->Get(
                    context, i).ToLocalChecked()->NumberValue(context).ToChecked();
            data.push_back(
                    static_cast<float>(item));
        }

        canvas_native_context_set_line_dash(
                ptr->GetContext(), data.data(), data.size());
    }
}

void
CanvasRenderingContext2DImpl::SetTransform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto count = args.Length();
    auto value = args[0];
    if (count == 1 && value->IsObject()) {

        auto typeValue = GetPrivateValue(isolate, value.As<v8::Object>(),
                                         ConvertToV8String(isolate, "__type"));

        if (!typeValue.IsEmpty()) {
            auto type = (NativeType) typeValue->Int32Value(context).ToChecked();

            if (type == NativeType::Matrix) {
                auto matrix = MatrixImpl::GetPointer(value.As<v8::Object>());
                if (matrix != nullptr) {
                    canvas_native_context_set_transform_matrix(
                            ptr->GetContext(),
                            matrix->GetMatrix());
                }
            }

        }
    } else if (count == 6) {
        auto a = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto b = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto c = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto d = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto e = static_cast<float>(args[4]->NumberValue(context).ToChecked());
        auto f = static_cast<float>(args[5]->NumberValue(context).ToChecked());
        canvas_native_context_set_transform(
                ptr->GetContext(), a, b, c, d,
                e,
                f);
    }
}

void
CanvasRenderingContext2DImpl::Stroke(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();

    auto value = args[0];
    auto type = GetNativeType(value);
    if (type == NativeType::Path2D) {
        auto path = Path2D::GetPointer(value.As<v8::Object>());
        if (path != nullptr) {
            canvas_native_context_stroke_with_path(
                    ptr->GetContext(),
                    path->GetPath());
            ptr->UpdateInvalidateState();
        }
    } else {
        canvas_native_context_stroke(
                ptr->GetContext());
        ptr->UpdateInvalidateState();
    }
}

void
CanvasRenderingContext2DImpl::StrokeRect(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    if (args.Length() == 4) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto width = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto height = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        canvas_native_context_stroke_rect(
                ptr->GetContext(), x, y,
                width,
                height);
        ptr->UpdateInvalidateState();
    }
}

void
CanvasRenderingContext2DImpl::StrokeText(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto count = args.Length();
    if (count >= 3) {
        auto text = ConvertFromV8String(isolate, args[0]);
        auto x = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        if (count > 3) {
            float maxWidth = static_cast<float>(args[3]->NumberValue(context).ToChecked());
            canvas_native_context_stroke_text_width(
                    ptr->GetContext(), text.c_str(),
                    x, y, maxWidth);
        } else {
            canvas_native_context_stroke_text(
                    ptr->GetContext(), text.c_str(),
                    x, y);
        }


        ptr->UpdateInvalidateState();
    }
}

void
CanvasRenderingContext2DImpl::Transform(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() == 6) {
        auto a = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto b = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        auto c = static_cast<float>(args[2]->NumberValue(context).ToChecked());
        auto d = static_cast<float>(args[3]->NumberValue(context).ToChecked());
        auto e = static_cast<float>(args[4]->NumberValue(context).ToChecked());
        auto f = static_cast<float>(args[5]->NumberValue(context).ToChecked());
        canvas_native_context_transform(
                ptr->GetContext(), a, b, c, d,
                e,
                f);
    }
}

void
CanvasRenderingContext2DImpl::Translate(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    if (args.Length() == 2) {
        auto x = static_cast<float>(args[0]->NumberValue(context).ToChecked());
        auto y = static_cast<float>(args[1]->NumberValue(context).ToChecked());
        canvas_native_context_translate(
                ptr->GetContext(), x, y);
    }
}

void
CanvasRenderingContext2DImpl::__ToDataURL(const v8::FunctionCallbackInfo<v8::Value> &args) {
    CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());

    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();


    std::string type("image/png");
    int quality = 92;
    if (args[0]->IsString()) {
        type = ConvertFromV8String(isolate, args[0]);
    }


    if (args[1]->IsNumber()) {
        quality = (int) args[1]->NumberValue(context).ToChecked();
    }


    auto data = canvas_native_to_data_url(
            ptr->GetContext(), type.c_str(),
            quality);

    auto value = new OneByteStringResource((char *) data);
    auto ret = v8::String::NewExternalOneByte(isolate, value);
    args.GetReturnValue().Set(ret.ToLocalChecked());

    // args.GetReturnValue().Set(ConvertToV8String(isolate, data.c_str()));
}

CanvasRenderingContext2DImpl::~CanvasRenderingContext2DImpl() {
    auto raf = this->raf_.get();
    if (raf != nullptr) {
        canvas_native_raf_stop(raf->GetRaf());
    }

    canvas_native_context_destroy(this->GetContext());
    this->context_ = nullptr;
}

void CanvasRenderingContext2DImpl::UpdateInvalidateState() {
    auto raf = this->GetRaf();
    if (raf != nullptr) {
        if (!canvas_native_raf_get_started(raf->GetRaf())) {
            canvas_native_raf_start(raf->GetRaf());
        }
    }

    auto state = this->GetInvalidateState();
    this->SetInvalidateState((int) state | (int) InvalidateState::InvalidateStatePending);
}

InvalidateState CanvasRenderingContext2DImpl::GetInvalidateState() const {
    return (InvalidateState)
            this->invalidateState_;
}

void CanvasRenderingContext2DImpl::SetInvalidateState(InvalidateState state) {
    this->invalidateState_ = (int) state;
}

void CanvasRenderingContext2DImpl::SetInvalidateState(int state) {
    this->invalidateState_ = state;
}


void CanvasRenderingContext2DImpl::Flush() {
    auto state = (int) this->GetInvalidateState() & (int) InvalidateState::InvalidateStatePending;
    if (state == (int) InvalidateState::InvalidateStatePending) {
        this->SetInvalidateState(InvalidateState::InvalidateStateInvalidating);
        //   canvas_native_context_flush(ptr->GetContext());

        canvas_native_context_render(this->GetContext());
        //        canvas_native_context_gl_make_current(ptr->GetContext());
        //        canvas_native_context_gl_swap_buffers(ptr->GetContext());
        this->SetInvalidateState(InvalidateState::InvalidateStateNone);
    }
}

void CanvasRenderingContext2DImpl::Flush(intptr_t context) {
    auto ctx = reinterpret_cast<CanvasRenderingContext2DImpl *>(reinterpret_cast<intptr_t *>(context));
    if (ctx != nullptr) {
        ctx->Flush();
    }
}


CanvasRenderingContext2D *CanvasRenderingContext2DImpl::GetContext() {
    return this->context_;
}
