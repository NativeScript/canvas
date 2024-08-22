//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once
#pragma process_pending_includes

#include <vector>
#include <cstdint>
#include <memory>

#include "RafImpl.h"
#include "OnRafCallback.h"

#include "Helpers.h"

#include "CanvasGradient.h"
#include "CanvasPattern.h"
#include "ImageDataImpl.h"
#include "TextMetricsImpl.h"
#include "ImageAssetImpl.h"
#include "ImageBitmapImpl.h"
#include "Path2D.h"
#include "WebGLRenderingContextBase.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class CanvasRenderingContext2DImpl : ObjectWrapperImpl {
public:

    static v8::CFunction fast_start_raf_;

    static v8::CFunction fast_stop_raf_;

    static v8::CFunction fast_draw_point_;

    static v8::CFunction fast_arc_;

    static v8::CFunction fast_arc_to_;

    static v8::CFunction fast_begin_path_;

    static v8::CFunction fast_bezier_curve_to_;

    static v8::CFunction fast_clear_rect_;

    static v8::CFunction fast_clip_rule_;

    static v8::CFunction fast_clip_path_;

    static v8::CFunction fast_clip_;

    static v8::CFunction fast_close_path_;

    static v8::CFunction fast_draw_image_dx_dy_;

    static v8::CFunction fast_draw_image_dx_dy_dw_dh_;

    static v8::CFunction fast_draw_image_;

    static v8::CFunction fast_ellipse_;

    static v8::CFunction fast_fill_;

    static v8::CFunction fast_fill_path_;

    static v8::CFunction fast_fill_rule_;

    static v8::CFunction fast_is_point_in_path_xy_;

    static v8::CFunction fast_is_point_in_path_xy_rule_;

    static v8::CFunction fast_is_point_in_path_;

    static v8::CFunction fast_is_point_in_stroke_xy_;

    static v8::CFunction fast_is_point_in_stroke_;

    static v8::CFunction fast_line_to_;

    static v8::CFunction fast_move_to_;

    static v8::CFunction fast_put_image_data_dx_dy_;

    static v8::CFunction fast_put_image_data_;

    static v8::CFunction fast_quadratic_curve_to_;

    static v8::CFunction fast_round_rect_array_;

    static v8::CFunction fast_round_rect_;

    static v8::CFunction fast_rect_;

    static v8::CFunction fast_scale_;

    static v8::CFunction fast_set_line_dash_;

    static v8::CFunction fast_set_transform_;

    static v8::CFunction fast_set_transform_abcdef_;

    static v8::CFunction fast_transform_;

    static v8::CFunction fast_fill_path_rule_;

    static v8::CFunction fast_fill_rect_;

    static v8::CFunction fast_fill_oval_;

    static v8::CFunction fast_stroke_;

    static v8::CFunction fast_stroke_path_;

    static v8::CFunction fast_stroke_rect_;

    static v8::CFunction fast_stroke_oval_;

    static v8::CFunction fast_rotate_;

    static v8::CFunction fast_reset_transform_;

    static v8::CFunction fast_restore_;

    static v8::CFunction fast_save_;

    static v8::CFunction fast_translate_;

    static v8::CFunction fast_draw_atlas_;


    CanvasRenderingContext2DImpl(CanvasRenderingContext2D *context);

    void StartRaf();

    void StopRaf();

    static void __StartRaf(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void __FastStartRaf(v8::Local<v8::Object> receiver_obj) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        ptr->StartRaf();

    }

    static void __StopRaf(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void __FastStopRaf(v8::Local<v8::Object> receiver_obj) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        ptr->StopRaf();

    }

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static CanvasRenderingContext2DImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, CanvasRenderingContext2DImpl *renderingContext) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = CanvasRenderingContext2DImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(renderingContext, NativeType::CanvasRenderingContext2D);
        object->SetAlignedPointerInInternalField(0, renderingContext);
        renderingContext->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void DrawPoint(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void FastDrawPoint(v8::Local<v8::Object> receiver_obj, double x, double y) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_draw_point(
                ptr->GetContext(), static_cast<float>(x), static_cast<float>(y));
        ptr->UpdateInvalidateState();
    }

    static void DrawPoints(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawPaint(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void __MakeDirty(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void __GetPointer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void __Resize(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void SetContinuousRenderMode(v8::Local<v8::String> property,
                                        v8::Local<v8::Value> value,
                                        const v8::PropertyCallbackInfo<void> &info);

    static void GetContinuousRenderMode(v8::Local<v8::String> property,
                                        const v8::PropertyCallbackInfo<v8::Value> &info);


    static void GetFilter(v8::Local<v8::String> property,
                          const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetFilter(v8::Local<v8::String> property,
                          v8::Local<v8::Value> value,
                          const v8::PropertyCallbackInfo<void> &info);

    static void GetFont(v8::Local<v8::String> property,
                        const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetFont(v8::Local<v8::String> property,
                        v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info);


    static void GetLetterSpacing(v8::Local<v8::String> property,
                                 const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetLetterSpacing(v8::Local<v8::String> property,
                                 v8::Local<v8::Value> value,
                                 const v8::PropertyCallbackInfo<void> &info);

    static void GetWordSpacing(v8::Local<v8::String> property,
                               const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetWordSpacing(v8::Local<v8::String> property,
                               v8::Local<v8::Value> value,
                               const v8::PropertyCallbackInfo<void> &info);

    static void SetGlobalAlpha(v8::Local<v8::String> property,
                               v8::Local<v8::Value> value,
                               const v8::PropertyCallbackInfo<void> &info);

    static void GetGlobalAlpha(v8::Local<v8::String> property,
                               const v8::PropertyCallbackInfo<v8::Value> &info);


    static void SetImageSmoothingEnabled(v8::Local<v8::String> property,
                                         v8::Local<v8::Value> value,
                                         const v8::PropertyCallbackInfo<void> &info);

    static void GetImageSmoothingEnabled(v8::Local<v8::String> property,
                                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetImageSmoothingQuality(v8::Local<v8::String> property,
                                         v8::Local<v8::Value> value,
                                         const v8::PropertyCallbackInfo<void> &info);

    static void GetImageSmoothingQuality(v8::Local<v8::String> property,
                                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetLineDashOffset(v8::Local<v8::String> property,
                                  v8::Local<v8::Value> value,
                                  const v8::PropertyCallbackInfo<void> &info);

    static void GetLineDashOffset(v8::Local<v8::String> property,
                                  const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetLineJoin(v8::Local<v8::String> property,
                            const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetLineJoin(v8::Local<v8::String> property,
                            v8::Local<v8::Value> value,
                            const v8::PropertyCallbackInfo<void> &info);

    static void GetLineCap(v8::Local<v8::String> property,
                           const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetLineCap(v8::Local<v8::String> property,
                           v8::Local<v8::Value> value,
                           const v8::PropertyCallbackInfo<void> &info);

    static void GetMiterLimit(v8::Local<v8::String> property,
                              const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetMiterLimit(v8::Local<v8::String> property,
                              v8::Local<v8::Value> value,
                              const v8::PropertyCallbackInfo<void> &info);

    static void GetShadowColor(v8::Local<v8::String> property,
                               const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetShadowColor(v8::Local<v8::String> property,
                               v8::Local<v8::Value> value,
                               const v8::PropertyCallbackInfo<void> &info);

    static void GetShadowBlur(v8::Local<v8::String> property,
                              const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetShadowBlur(v8::Local<v8::String> property,
                              v8::Local<v8::Value> value,
                              const v8::PropertyCallbackInfo<void> &info);

    static void GetShadowOffsetX(v8::Local<v8::String> property,
                                 const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetShadowOffsetX(v8::Local<v8::String> property,
                                 v8::Local<v8::Value> value,
                                 const v8::PropertyCallbackInfo<void> &info);

    static void GetShadowOffsetY(v8::Local<v8::String> property,
                                 const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetShadowOffsetY(v8::Local<v8::String> property,
                                 v8::Local<v8::Value> value,
                                 const v8::PropertyCallbackInfo<void> &info);

    static void GetTextAlign(v8::Local<v8::String> property,
                             const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetTextAlign(v8::Local<v8::String> property,
                             v8::Local<v8::Value> value,
                             const v8::PropertyCallbackInfo<void> &info);

    static void GetTextBaseline(v8::Local<v8::String> property,
                                const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetTextBaseline(v8::Local<v8::String> property,
                                v8::Local<v8::Value> value,
                                const v8::PropertyCallbackInfo<void> &info);

    static void GetGlobalCompositeOperation(v8::Local<v8::String> property,
                                            const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetGlobalCompositeOperation(v8::Local<v8::String> property,
                                            v8::Local<v8::Value> value,
                                            const v8::PropertyCallbackInfo<void> &info);

    static void GetFillStyle(v8::Local<v8::String> property,
                             const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetFillStyle(v8::Local<v8::String> property,
                             v8::Local<v8::Value> value,
                             const v8::PropertyCallbackInfo<void> &info);

    static void GetStrokeStyle(v8::Local<v8::String> property,
                               const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetStrokeStyle(v8::Local<v8::String> property,
                               v8::Local<v8::Value> value,
                               const v8::PropertyCallbackInfo<void> &info);

    static void GetLineWidth(v8::Local<v8::String> property,
                             const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetLineWidth(v8::Local<v8::String> property,
                             v8::Local<v8::Value> value,
                             const v8::PropertyCallbackInfo<void> &info);

    static void GetLineDash(v8::Local<v8::String> property,
                            const v8::PropertyCallbackInfo<v8::Value> &info);

    static void SetLineDash(v8::Local<v8::String> property,
                            v8::Local<v8::Value> value,
                            const v8::PropertyCallbackInfo<void> &info);


    static void AddHitRegion(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Arc(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void
    ArcImpl(CanvasRenderingContext2D *receiver_obj, double x, double y, double radius,
            double startAngle,
            double endAngle, bool anti_clockwise) {

        canvas_native_context_arc(
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
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        ArcImpl(
                ptr->GetContext(),
                x,
                y,
                radius,
                startAngle,
                endAngle,
                anti_clockwise
        );
    }


    static void ArcTo(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastArcTo(v8::Local<v8::Object> receiver_obj, double x1,
                          double y1,
                          double x2,
                          double y2,
                          double radius) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_arc_to(
                ptr->GetContext(), static_cast<float>(x1),
                static_cast<float>(y1),
                static_cast<float>(x2),
                static_cast<float>(y2),
                static_cast<float>(radius)
        );
    }

    static void BeginPath(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BeginPathImpl(CanvasRenderingContext2D *receiver_obj) {
        canvas_native_context_begin_path(receiver_obj);
    }

    static void FastBeginPath(v8::Local<v8::Object> receiver_obj) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        BeginPathImpl(ptr->GetContext());
    }

    static void BezierCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void FastBezierCurveTo(v8::Local<v8::Object> receiver_obj, double cp1x,
                                  double cp1y,
                                  double cp2x,
                                  double cp2y,
                                  double x,
                                  double y) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_bezier_curve_to(
                ptr->GetContext(),
                static_cast<float>(cp1x),
                static_cast<float>(cp1y),
                static_cast<float>(cp2x),
                static_cast<float>(cp2y),
                static_cast<float>(x),
                static_cast<float>(y)
        );
    }

    static void ClearHitRegions(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ClearRect(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void FastClearRect(v8::Local<v8::Object> receiver_obj, double x, double y, double width,
                              double height) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_clear_rect(
                ptr->GetContext(),
                static_cast<float>(x),
                static_cast<float>(y),
                static_cast<float>(width),
                static_cast<float>(height)
        );
        ptr->UpdateInvalidateState();
    }

    static void Clip(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastClipRule(v8::Local<v8::Object> receiver_obj, uint32_t rule) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        switch (rule) {
            case 0:
                canvas_native_context_clip_rule(
                        ptr->GetContext(), CanvasFillRuleNonZero);
                break;
            case 1:
                canvas_native_context_clip_rule(
                        ptr->GetContext(), CanvasFillRuleEvenOdd);
                break;
            default:
                break;
        }
    }

    static void FastClipPath(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> path_obj) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        if (GetNativeType(path_obj) == NativeType::Path2D) {
            auto path = Path2D::GetPointer(path_obj);
            canvas_native_context_clip(
                    ptr->GetContext(), path->GetPath(), CanvasFillRuleNonZero);
        }


    }

    static void
    FastClip(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> path_obj, uint32_t rule) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        if (GetNativeType(path_obj) == NativeType::Path2D) {
            auto path = Path2D::GetPointer(path_obj);
            switch (rule) {
                case 0:
                    canvas_native_context_clip(
                            ptr->GetContext(), path->GetPath(), CanvasFillRuleNonZero);
                    break;
                case 1:
                    canvas_native_context_clip(
                            ptr->GetContext(), path->GetPath(), CanvasFillRuleEvenOdd);
                    break;
                default:
                    break;
            }
        }

    }

    static void ClosePath(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ClosePathImpl(CanvasRenderingContext2D *receiver_obj) {
        canvas_native_context_close_path(receiver_obj);
    }

    static void FastClosePath(v8::Local<v8::Object> receiver_obj) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        ClosePathImpl(ptr->GetContext());
    }

    static void CreateImageData(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreatePattern(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateLinearGradient(
            const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateConicGradient(
            const v8::FunctionCallbackInfo<v8::Value> &args);

    static void __CreatePatternWithNative(
            const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CreateRadialGradient(
            const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawFocusIfNeeded(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void DrawAtlas(const v8::FunctionCallbackInfo<v8::Value> &args);

/*
    static void FastDrawAtlas(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> image_obj, v8::Local<v8::Array> xform, v8::Local<v8::Array> tex, v8::Local<v8::Array> xform) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto len = array->Length();
        std::vector<float> buf;
        buf.reserve(len);

        auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                array, buf.data(), len);

        if (copied) {

        auto imageType = GetNativeType(image_obj);
        switch (imageType) {
            case NativeType::ImageAsset: {
                auto asset = ImageAssetImpl::GetPointer(image_obj);

            }
                break;
            case NativeType::ImageBitmap: {
                auto asset = ImageBitmapImpl::GetPointer(image_obj);
            }
                break;
            case NativeType::CanvasRenderingContext2D: {
                auto context = CanvasRenderingContext2DImpl::GetPointer(image_obj);

            }
                break;
            default:
                break;
        }


        ptr->UpdateInvalidateState();
    }
        */

    static void DrawImage(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    DrawImageDxDyAssetImpl(CanvasRenderingContext2DImpl *ptr, const ImageAsset *asset, double dx,
                           double dy) {
        if (asset != nullptr) {
            canvas_native_context_draw_image_dx_dy_asset(
                    ptr->GetContext(),
                    asset,
                    static_cast<float>(dx), static_cast<float>(dy));
            ptr->UpdateInvalidateState();
        }
    }

    static void
    DrawImageDxDyContext2DImpl(CanvasRenderingContext2DImpl *ptr,
                               CanvasRenderingContext2D *source, double dx, double dy) {
        if (source != nullptr) {
            canvas_native_context_draw_image_dx_dy_context(
                    ptr->GetContext(),
                    source,
                    static_cast<float>(dx), static_cast<float>(dy));
            ptr->UpdateInvalidateState();
        }
    }


    static void
    FastDrawImageDxDy(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> image_obj,
                      double dx, double dy) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto imageType = GetNativeType(image_obj);
        switch (imageType) {
            case NativeType::ImageAsset: {
                auto asset = ImageAssetImpl::GetPointer(image_obj);
                DrawImageDxDyAssetImpl(ptr, asset->GetImageAsset(), dx, dy);
            }
                break;
            case NativeType::ImageBitmap: {
                auto asset = ImageBitmapImpl::GetPointer(image_obj);
                DrawImageDxDyAssetImpl(ptr, asset->GetImageAsset(), dx, dy);
            }
                break;
            case NativeType::CanvasRenderingContext2D: {
                auto context = CanvasRenderingContext2DImpl::GetPointer(image_obj);
                DrawImageDxDyContext2DImpl(ptr, context->GetContext(), dx, dy);
            }
                break;
            default:
                break;
        }
    }

    static void DrawImageDxDyDwDhAssetImpl(CanvasRenderingContext2DImpl *ptr, const ImageAsset *asset,
                                           double dx, double dy, double dw, double dh) {


        if (asset != nullptr) {
            canvas_native_context_draw_image_dx_dy_dw_dh_asset(
                    ptr->GetContext(),
                    asset,
                    static_cast<float>(dx), static_cast<float>(dy),
                    static_cast<float>(dw),
                    static_cast<float>(dh));
            ptr->UpdateInvalidateState();
        }
    }


    static void DrawImageDxDyDwDhContextImpl(CanvasRenderingContext2DImpl *ptr,
                                             CanvasRenderingContext2D *source,
                                             double dx, double dy, double dw, double dh) {

        if (source != nullptr) {
            canvas_native_context_draw_image_dx_dy_dw_dh_context(
                    ptr->GetContext(),
                    source,
                    static_cast<float>(dx), static_cast<float>(dy),
                    static_cast<float>(dw),
                    static_cast<float>(dh));
            ptr->UpdateInvalidateState();
        }
    }


    static void
    FastDrawImageDxDyDwDh(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> image_obj,
                          double dx, double dy, double dw, double dh) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto imageType = GetNativeType(image_obj);

        switch (imageType) {
            case NativeType::ImageAsset: {
                auto image_asset = ImageAssetImpl::GetPointer(image_obj);

                if (image_asset != nullptr) {
                    DrawImageDxDyDwDhAssetImpl(
                            ptr,
                            image_asset->GetImageAsset(),
                            dx, dy,
                            dw,
                            dh);
                    ptr->UpdateInvalidateState();
                }
            }
                break;
            case NativeType::ImageBitmap: {
                auto image_asset = ImageBitmapImpl::GetPointer(image_obj);

                if (image_asset != nullptr) {
                    DrawImageDxDyDwDhAssetImpl(
                            ptr,
                            image_asset->GetImageAsset(),
                            dx, dy,
                            dw,
                            dh);
                    ptr->UpdateInvalidateState();
                }
            }
                break;
            case NativeType::CanvasRenderingContext2D: {
                auto context = CanvasRenderingContext2DImpl::GetPointer(image_obj);

                if (context != nullptr) {
                    DrawImageDxDyDwDhContextImpl(
                            ptr,
                            context->GetContext(),
                            dx, dy,
                            dw,
                            dh);
                    ptr->UpdateInvalidateState();
                }
            }
                break;
            default:
                break;
        }
    }

    static void
    DrawImageAssetImpl(CanvasRenderingContext2DImpl *ptr,
                       const ImageAsset *asset, double sx,
                       double sy, double sw, double sh, double dx, double dy, double dw,
                       double dh) {

        canvas_native_context_draw_image_asset(
                ptr->GetContext(),
                asset,
                static_cast<float>(sx),
                static_cast<float>(sy), static_cast<float>(sw), static_cast<float>(sh),
                static_cast<float>(dx),
                static_cast<float>(dy), static_cast<float>(dw), static_cast<float>(dh));

        ptr->UpdateInvalidateState();
    }


    static void
    DrawImageContextImpl(CanvasRenderingContext2DImpl *ptr,
                         CanvasRenderingContext2D *source, double sx,
                         double sy, double sw, double sh, double dx, double dy, double dw,
                         double dh) {
        canvas_native_context_draw_image_context(
                ptr->GetContext(),
                source,
                static_cast<float>(sx),
                static_cast<float>(sy), static_cast<float>(sw), static_cast<float>(sh),
                static_cast<float>(dx),
                static_cast<float>(dy), static_cast<float>(dw), static_cast<float>(dh));
        ptr->UpdateInvalidateState();
    }

    static void
    FastDrawImage(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> image_obj, double sx,
                  double sy, double sw, double sh, double dx, double dy, double dw,
                  double dh) {

        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto imageType = GetNativeType(image_obj);

        switch (imageType) {
            case NativeType::ImageAsset: {
                auto image_asset = ImageAssetImpl::GetPointer(image_obj);

                if (image_asset != nullptr) {
                    DrawImageAssetImpl(ptr, image_asset->GetImageAsset(), sx, sy, sw, sh, dx, dy,
                                       dw, dh);
                }
            }
                break;
            case NativeType::ImageBitmap: {
                auto image_bitmap = ImageBitmapImpl::GetPointer(image_obj);
                if (image_bitmap != nullptr) {
                    DrawImageAssetImpl(ptr, image_bitmap->GetImageAsset(), sx, sy, sw, sh, dx, dy,
                                       dw, dh);
                }
            }
                break;
            case NativeType::CanvasRenderingContext2D: {
                auto image_canvas = CanvasRenderingContext2DImpl::GetPointer(image_obj);
                if (image_canvas != nullptr) {
                    DrawImageContextImpl(ptr, image_canvas->GetContext(), sx, sy, sw, sh, dx, dy,
                                         dw, dh);
                }
            }
                break;
            default:
                break;
        }

    }


    static void Ellipse(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastEllipse(v8::Local<v8::Object> receiver_obj, double x,
                            double y,
                            double radius_x,
                            double radius_y,
                            double rotation,
                            double start_angle,
                            double end_angle,
                            bool anticlockwise) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_ellipse(
                ptr->GetContext(), x, y,
                radius_x,
                radius_y, rotation,
                start_angle, end_angle,
                anticlockwise);
    }


    static void Fill(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastFillPathRule(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> path,
                     uint32_t rule) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto object = Path2D::GetPointer(path);

        if (object != nullptr) {
            switch (rule) {
                case 0:
                    canvas_native_context_fill_with_path(
                            ptr->GetContext(),
                            object->GetPath(),
                            CanvasFillRuleNonZero);
                    break;
                case 1:
                    canvas_native_context_fill_with_path(
                            ptr->GetContext(),
                            object->GetPath(),
                            CanvasFillRuleEvenOdd);
                    break;
                default:
                    break;
            }

            ptr->UpdateInvalidateState();
        }
    }


    static void FastFillPath(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> path) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto object = Path2D::GetPointer(path);


        canvas_native_context_fill_with_path(
                ptr->GetContext(),
                object->GetPath(), CanvasFillRuleNonZero);
        ptr->UpdateInvalidateState();
    }

    static void FastFillRule(v8::Local<v8::Object> receiver_obj, uint32_t rule) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        switch (rule) {
            case 0:
                canvas_native_context_fill(
                        ptr->GetContext(), CanvasFillRuleNonZero);
                break;
            case 1:
                canvas_native_context_fill(
                        ptr->GetContext(), CanvasFillRuleEvenOdd);
                break;
            default:
                break;
        }


        ptr->UpdateInvalidateState();
    }

    static void FastFill(v8::Local<v8::Object> receiver_obj) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_fill(
                ptr->GetContext(), CanvasFillRuleNonZero);
        ptr->UpdateInvalidateState();
    }

    static void FillRect(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void FastFillRect(v8::Local<v8::Object> receiver_obj, double x, double y, double width,
                             double height) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_fill_rect(
                ptr->GetContext(),
                static_cast<float>(x),
                static_cast<float>(y),
                static_cast<float>(width),
                static_cast<float>(height)
        );
        ptr->UpdateInvalidateState();
    }

    static void FillText(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void FillOval(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void FastFillOval(v8::Local<v8::Object> receiver_obj, double x, double y, double width,
                             double height) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_fill_oval(
                ptr->GetContext(),
                static_cast<float>(x),
                static_cast<float>(y),
                static_cast<float>(width),
                static_cast<float>(height)
        );
        ptr->UpdateInvalidateState();
    }

    static void GetImageData(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetLineDash(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsPointInPath(const v8::FunctionCallbackInfo<v8::Value> &args);

    static bool FastIsPointInPathXY(v8::Local<v8::Object> receiver_obj, double x, double y) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);

        if (ptr == nullptr) {
            return false;
        }

        return canvas_native_context_is_point_in_path(
                ptr->GetContext(), static_cast<float>(x), static_cast<float>(y),
                CanvasFillRuleNonZero);
    }

    static bool
    FastIsPointInPathXYRule(v8::Local<v8::Object> receiver_obj, double x, double y, uint32_t rule) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);

        if (ptr == nullptr) {
            return false;
        }


        return canvas_native_context_is_point_in_path(
                ptr->GetContext(), static_cast<float>(x), static_cast<float>(y),
                rule == 0 ? CanvasFillRuleNonZero : CanvasFillRuleEvenOdd);
    }

    static bool
    FastIsPointInPath(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> path_obj, double x,
                      double y, uint32_t rule) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);

        if (ptr == nullptr) {
            return false;
        }


        bool ret = false;

        auto type = GetNativeType(path_obj);

        if (type == NativeType::Path2D) {
            auto path = Path2D::GetPointer(path_obj);

            switch (rule) {
                case 0:

                    ret = canvas_native_context_is_point_in_path_with_path(
                            ptr->GetContext(),
                            path->GetPath(), (float) x, (float) y, CanvasFillRuleNonZero);
                    break;
                case 1:

                    ret = canvas_native_context_is_point_in_path_with_path(
                            ptr->GetContext(),
                            path->GetPath(), (float) x, (float) y, CanvasFillRuleEvenOdd);
                    break;
                default:
                    break;
            }

        }
        return ret;
    }


    static void IsPointInStroke(const v8::FunctionCallbackInfo<v8::Value> &args);


    static bool FastIsPointInStrokeXY(v8::Local<v8::Object> receiver_obj, double x, double y) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);

        if (ptr == nullptr) {
            return false;
        }


        return canvas_native_context_is_point_in_stroke(
                ptr->GetContext(), static_cast<float>(x), static_cast<float>(y));
    }

    static bool
    FastIsPointInStroke(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> path_obj,
                        double x, double y) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);

        if (ptr == nullptr) {
            return false;
        }

        auto type = GetNativeType(path_obj);

        if (type == NativeType::Path2D) {
            auto path = Path2D::GetPointer(path_obj);

            return canvas_native_context_is_point_in_stroke_with_path(
                    ptr->GetContext(),
                    path->GetPath(), static_cast<float>(x), static_cast<float>(y));
        }

        return false;
    }

    static void LineTo(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void FastLineTo(v8::Local<v8::Object> receiver_obj, float x, float y) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_line_to(
                ptr->GetContext(), x, y);
    }


    static void MeasureText(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void MoveTo(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastMoveTo(v8::Local<v8::Object> receiver_obj, float x, float y) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_move_to(
                ptr->GetContext(), x, y);
    }

    static void PutImageData(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastPutImageDataDxDy(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> image_data_obj,
                         float dx, float dy) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto type = GetNativeType(image_data_obj);
        if (type == NativeType::ImageData) {
            auto imageData = ImageDataImpl::GetPointer(image_data_obj);
            auto dirtyWidth = (float) canvas_native_image_data_get_width(
                    imageData->GetImageData());
            auto dirtyHeight = (float) canvas_native_image_data_get_height(
                    imageData->GetImageData());
            canvas_native_context_put_image_data(
                    ptr->GetContext(),
                    imageData->GetImageData(), dx,
                    dy, 0, 0,
                    dirtyWidth, dirtyHeight);
            ptr->UpdateInvalidateState();
        }

    }

    static void
    FastPutImageData(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> image_data_obj,
                     float dx, float dy, float dirtyX, float dirtyY, float dirtyWidth,
                     float dirtyHeight) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(image_data_obj);
        if (type == NativeType::ImageData) {

            auto imageData = ImageDataImpl::GetPointer(image_data_obj);
            canvas_native_context_put_image_data(
                    ptr->GetContext(),
                    imageData->GetImageData(), dx,
                    dy, dirtyX, dirtyY,
                    dirtyWidth, dirtyHeight);
            ptr->UpdateInvalidateState();
        }

    }

    static void QuadraticCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void FastQuadraticCurveTo(v8::Local<v8::Object> receiver_obj, float cpx,
                                     float cpy,
                                     float x,
                                     float y) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);

        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_quadratic_curve_to(
                ptr->GetContext(), cpx, cpy,
                x, y);

    }


    static void RoundRect(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void FastRoundRectArray(v8::Local<v8::Object> receiver_obj, float x,
                                   float y,
                                   float width,
                                   float height, v8::Local<v8::Array> array) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);

        if (ptr == nullptr) { return; }


        auto len = array->Length();
        std::vector<float> buf;
        buf.reserve(len);

        auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                array, buf.data(), len);

        if (copied) {
            canvas_native_context_round_rect(
                    ptr->GetContext(),
                    x, y,
                    width,
                    height, buf.data(), buf.size());
        }
    }

    static void FastRoundRect(v8::Local<v8::Object> receiver_obj, float x,
                              float y,
                              float width,
                              float height, float radii) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);

        if (ptr == nullptr) { return; }


        canvas_native_context_round_rect_tl_tr_br_bl(
                ptr->GetContext(), x, y,
                width,
                height, radii, radii,
                radii, radii);

    }

    static void Rect(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void FastRect(v8::Local<v8::Object> receiver_obj, float x,
                         float y,
                         float width,
                         float height) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);

        if (ptr == nullptr) { return; }

        canvas_native_context_rect(
                ptr->GetContext(), x, y,
                width,
                height);

    }

    static void RemoveHitRegion(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ResetTransform(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastResetTransform(v8::Local<v8::Object> receiver_obj) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_reset_transform(
                ptr->GetContext());
    }

    static void Restore(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastRestore(v8::Local<v8::Object> receiver_obj) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_restore(
                ptr->GetContext());
    }

    static void Rotate(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastRotate(v8::Local<v8::Object> receiver_obj, float angle) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }
        canvas_native_context_rotate(
                ptr->GetContext(), angle);
    }


    static void Save(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastSave(v8::Local<v8::Object> receiver_obj) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_save(
                ptr->GetContext());
    }

    static void Scale(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastScale(v8::Local<v8::Object> receiver_obj, float x, float y) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_scale(
                ptr->GetContext(), x, y);
    }

    static void ScrollPathIntoView(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetLineDash(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void FastSetLineDash(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Array> array) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        auto len = array->Length();
        std::vector<float> buf;
        buf.reserve(len);

        auto copied = v8::TryToCopyAndConvertArrayToCppBuffer<v8::CTypeInfoBuilder<float>::Build().GetId(), float>(
                array, buf.data(), len);

        if (copied) {
            canvas_native_context_set_line_dash(
                    ptr->GetContext(), buf.data(), buf.size());
        }
    }


    static void SetTransform(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastSetTransform(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> matrix_obj) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto type = GetNativeType(matrix_obj);

        if (type == NativeType::Matrix) {
            auto matrix = MatrixImpl::GetPointer(matrix_obj);
            if (matrix != nullptr) {
                canvas_native_context_set_transform_matrix(
                        ptr->GetContext(),
                        matrix->GetMatrix());
            }
        }
    }


    static void
    FastSetTransformABCDEF(v8::Local<v8::Object> receiver_obj, float a, float b, float c, float d,
                           float e, float f) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_set_transform(
                ptr->GetContext(), a, b, c, d,
                e,
                f);
    }


    static void GetTransform(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Stroke(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastStroke(v8::Local<v8::Object> receiver_obj) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);

        canvas_native_context_stroke(
                ptr->GetContext());
        ptr->UpdateInvalidateState();
    }

    static void FastStrokePath(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> path_obj) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);

        auto path = Path2D::GetPointer(path_obj);
        if (path != nullptr) {
            canvas_native_context_stroke_with_path(
                    ptr->GetContext(),
                    path->GetPath());
            ptr->UpdateInvalidateState();
        }
    }

    static void StrokeRect(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void FastStrokeRect(v8::Local<v8::Object> receiver_obj, double x, double y, double width,
                               double height) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_stroke_rect(
                ptr->GetContext(), static_cast<float>(x),
                static_cast<float>(y),
                static_cast<float>(width),
                static_cast<float>(height));
        ptr->UpdateInvalidateState();
    }


    static void StrokeText(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void StrokeOval(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void FastStrokeOval(v8::Local<v8::Object> receiver_obj, double x, double y, double width,
                               double height) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_stroke_oval(
                ptr->GetContext(), static_cast<float>(x),
                static_cast<float>(y),
                static_cast<float>(width),
                static_cast<float>(height));
        ptr->UpdateInvalidateState();
    }


    static void Transform(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    FastTransform(v8::Local<v8::Object> receiver_obj, float a, float b, float c, float d,
                  float e, float f) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }


        canvas_native_context_transform(
                ptr->GetContext(), a, b, c, d,
                e,
                f);
    }

    static void Translate(const v8::FunctionCallbackInfo<v8::Value> &args);


    static void FastTranslate(v8::Local<v8::Object> receiver_obj, float x, float y) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        canvas_native_context_translate(
                ptr->GetContext(), x, y);
    }


    static void __ToDataURL(const v8::FunctionCallbackInfo<v8::Value> &args);

    ~CanvasRenderingContext2DImpl();

    void UpdateInvalidateState();

    InvalidateState GetInvalidateState() const;

    void SetInvalidateState(InvalidateState state);

    void SetInvalidateState(int state);

    void Flush();

    static void Flush(intptr_t context);

    void SetRaf(std::shared_ptr<RafImpl> raf);

    RafImpl *GetRaf();

    CanvasRenderingContext2D *GetContext();

private:
    CanvasRenderingContext2D *context_;

    int invalidateState_ = static_cast<int>(InvalidateState::InvalidateStateNone);

    std::shared_ptr<RafImpl> raf_;

    bool continuousRender_ = true;

};
