//
// Created by Osei Fortune on 28/03/2022.
//

#pragma once

#include "Common.h"
#include "Helpers.h"
#include <vector>
#include "ObjectWrapperImpl.h"

class Path2D : ObjectWrapperImpl {
public:
    Path2D(Path *path);

    ~Path2D() {
        canvas_native_path_destroy(this->GetPath());
        this->path_ = nullptr;
    }

    Path *GetPath();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static Path2D *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void Ctor(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void AddPath(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastAddPath(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> obj) {
        Path2D *ptr = Path2D::GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        Path2D *src = Path2D::GetPointer(obj);
        if (src == nullptr) {
            return;
        }

        AddPathImpl(ptr->GetPath(), src->GetPath());
    }

    static void AddPathImpl(Path *receiver_obj, Path *obj) {
        canvas_native_path_add_path(
                receiver_obj,
                obj);
    }

    static v8::CFunction fast_add_path_;
    static v8::CFunction fast_arc_;
    static v8::CFunction fast_arc_to_;
    static v8::CFunction fast_bezier_curve_to_;
    static v8::CFunction fast_close_path_;
    static v8::CFunction fast_ellipse_;
    static v8::CFunction fast_line_to_;
    static v8::CFunction fast_move_to_;
    static v8::CFunction fast_quadratic_curve_to_;
    static v8::CFunction fast_rect_;
    static v8::CFunction fast_round_rect_;
    static v8::CFunction fast_round_rect_array_;
    // static v8::CFunction fast_to_svg_; // todo after v8 upgrade

    static v8::CFunction fast_trim_;


    static void Arc(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    ArcImpl(Path *receiver_obj, double x, double y, double radius, double startAngle,
            double endAngle, bool anti_clockwise) {

        canvas_native_path_arc(
                receiver_obj,
                static_cast<float>(x),
                static_cast<float>(y),
                static_cast<float>(radius),
                static_cast<float>(startAngle),
                static_cast<float>(endAngle),
                anti_clockwise
        );
    }


    static void
    FastArc(v8::Local<v8::Object> receiver_obj, double x, double y, double radius,
            double startAngle,
            double endAngle, bool anti_clockwise) {
        Path2D *ptr = Path2D::GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        ArcImpl(
                ptr->GetPath(),
                static_cast<float>(x),
                static_cast<float>(y),
                static_cast<float>(radius),
                static_cast<float>(startAngle),
                static_cast<float>(endAngle),
                anti_clockwise
        );
    }


    static void
    ArcToImpl(Path *receiver_obj, double x1, double y1, double x2, double y2, double radius) {
        canvas_native_path_arc_to(
                receiver_obj,
                static_cast<float>(x1),
                static_cast<float>(y1),
                static_cast<float>(x2),
                static_cast<float>(y2),
                static_cast<float>(radius)
        );
    }


    static void
    FastArcTo(v8::Local<v8::Object> receiver_obj, double x1, double y1, double x2, double y2,
              double radius) {
        Path2D *ptr = Path2D::GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        ArcToImpl(ptr->GetPath(), x1, y1, x2, y2, radius);
    }

    static void ArcTo(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void BezierCurveToImpl(Path *receiver_obj,
                                  double cp1x,
                                  double cp1y,
                                  double cp2x,
                                  double cp2y,
                                  double x,
                                  double y) {


        canvas_native_path_bezier_curve_to(
                receiver_obj,
                static_cast<float>(cp1x),
                static_cast<float>(cp1y),
                static_cast<float>(cp2x),
                static_cast<float>(cp2y),
                static_cast<float>(x),
                static_cast<float>(y)
        );
    }


    static void FastBezierCurveTo(v8::Local<v8::Object> receiver_obj,
                                  double cp1x,
                                  double cp1y,
                                  double cp2x,
                                  double cp2y,
                                  double x,
                                  double y) {
        Path2D *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        BezierCurveToImpl(
                ptr->GetPath(),
                static_cast<float>(cp1x),
                static_cast<float>(cp1y),
                static_cast<float>(cp2x),
                static_cast<float>(cp2y),
                static_cast<float>(x),
                static_cast<float>(y)
        );
    }


    static void ClosePathImpl(Path *receiver_obj) {
        canvas_native_path_close_path(receiver_obj);
    }

    static void FastClosePath(v8::Local<v8::Object> receiver_obj) {
        Path2D *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        ClosePathImpl(ptr->GetPath());
    }

    static void EllipseImpl(Path *receiver_obj,
                            double x,
                            double y,
                            double radius_x,
                            double radius_y,
                            double rotation,
                            double start_angle,
                            double end_angle,
                            double anticlockwise) {

        canvas_native_path_ellipse(
                receiver_obj,
                static_cast<float>(x),
                static_cast<float>(y),
                static_cast<float>(radius_x),
                static_cast<float>(radius_y),
                static_cast<float>(rotation),
                static_cast<float>(start_angle),
                static_cast<float>(end_angle),
                anticlockwise
        );
    }

    static void FastEllipse(v8::Local<v8::Object> receiver_obj,
                            double x,
                            double y,
                            double radius_x,
                            double radius_y,
                            double rotation,
                            double start_angle,
                            double end_angle,
                            double anticlockwise) {
        Path2D *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        EllipseImpl(
                ptr->GetPath(),
                x,
                y,
                radius_x,
                radius_y,
                rotation,
                start_angle,
                end_angle,
                anticlockwise
        );
    }

    static void LineToImpl(Path *receiver_obj, double x, double y) {
        canvas_native_path_line_to(receiver_obj, static_cast<float>(x), static_cast<float>(y));
    }


    static void FastLineTo(v8::Local<v8::Object> receiver_obj, double x, double y) {
        Path2D *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }
        LineToImpl(ptr->GetPath(), x, y);
    }


    static void MoveToImpl(Path *receiver_obj, double x, double y) {
        canvas_native_path_move_to(receiver_obj, static_cast<float>(x), static_cast<float>(y));
    }

    static void FastMoveTo(v8::Local<v8::Object> receiver_obj, double x, double y) {
        Path2D *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }
        MoveToImpl(ptr->GetPath(), x, y);
    }

    static void QuadraticCurveToImpl(Path *receiver_obj, double cpx,
                                     double cpy,
                                     double x,
                                     double y) {

        canvas_native_path_quadratic_curve_to(
                receiver_obj,
                static_cast<float>(cpx),
                static_cast<float>(cpy),
                static_cast<float>(x),
                static_cast<float>(y)
        );
    }


    static void FastQuadraticCurveTo(v8::Local<v8::Object> receiver_obj, double cpx,
                                     double cpy,
                                     double x,
                                     double y) {
        Path2D *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        QuadraticCurveToImpl(
                ptr->GetPath(),
                cpx,
                cpy,
                x,
                y
        );
    }

    static void RectImpl(Path *receiver_obj, double x, double y, double width, double height) {
        canvas_native_path_rect(
                receiver_obj,
                static_cast<float>(x),
                static_cast<float>(y),
                static_cast<float>(width),
                static_cast<float>(height)
        );
    }

    static void
    FastRect(v8::Local<v8::Object> receiver_obj, double x, double y, double width, double height) {
        Path2D *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        RectImpl(
                ptr->GetPath(),
                x,
                y,
                width,
                height
        );
    }


    static void FastRoundRect(v8::Local<v8::Object> receiver_obj, double x, double y, double width,
                              double height, double radii) {
        Path2D *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_path_round_rect_tl_tr_br_bl(
                ptr->GetPath(), static_cast<float>(x), static_cast<float>(y),
                static_cast<float>(width),
                static_cast<float>(height), static_cast<float>(radii), static_cast<float>(radii),
                static_cast<float>(radii), static_cast<float>(radii));

    }

    static void
    FastRoundRectArray(v8::Local<v8::Object> receiver_obj, double x, double y, double width,
                       double height, v8::Local<v8::Array> value) {
        Path2D *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto len = value->Length();
        std::vector<float> buf;
        buf.reserve(len);

        auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                value, nullptr, len);

        if (copied) {
            if (len > 1) {

                canvas_native_path_round_rect(
                        ptr->GetPath(),
                        x, y,
                        width,
                        height, buf.data(),
                        buf.size());

            }
        }
    }


    /*

    static void ToSVGImpl(Path *receiver_obj) {
        Path2D *ptr = GetPointer(args.This());
        if (ptr == nullptr) {
            args.GetReturnValue().SetEmptyString();
            return;
        }

        auto isolate = args.GetIsolate();

        auto d = canvas_native_path_to_string(
                ptr->GetPath());

//    args.GetReturnValue().Set(ConvertToV8String(isolate, d.c_str()));

        auto value = new OneByteStringResource((char *) d);
        auto ret = v8::String::NewExternalOneByte(isolate, value);
        args.GetReturnValue().Set(ret.ToLocalChecked());
    }


    static v8::String::One FastToSVG(Path *receiver_obj) {
        Path2D *ptr = GetPointer(args.This());
        if (ptr == nullptr) {
            args.GetReturnValue().SetEmptyString();
            return;
        }

        auto isolate = args.GetIsolate();

        auto d = canvas_native_path_to_string(
                ptr->GetPath());

//    args.GetReturnValue().Set(ConvertToV8String(isolate, d.c_str()));

        auto value = new OneByteStringResource((char *) d);
        auto ret = v8::String::NewExternalOneByte(isolate, value);
        args.GetReturnValue().Set(ret.ToLocalChecked());
    }

    */

    static void BezierCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ClosePath(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Ellipse(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void LineTo(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void MoveTo(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void QuadraticCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Rect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RoundRect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Trim(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastTrim(v8::Local<v8::Object> receiver_obj, double start, double end) {

        Path2D *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_path_trim(
                ptr->GetPath(),
                static_cast<float>(start),
                static_cast<float>(end)
        );
    }

    static void __toSVG(const v8::FunctionCallbackInfo<v8::Value> &args);


private:
    Path *path_;
};
