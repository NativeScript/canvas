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
    CanvasRenderingContext2DImpl(CanvasRenderingContext2D *context);

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static CanvasRenderingContext2DImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, CanvasRenderingContext2DImpl *renderingContext) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = CanvasRenderingContext2DImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(object, NativeType::CanvasRenderingContext2D);
        object->SetAlignedPointerInInternalField(0, renderingContext);
        renderingContext->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void DrawPoint(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawPoints(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawPaint(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void __MakeDirty(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void __GetPointer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void __Resize(const v8::FunctionCallbackInfo<v8::Value> &args);

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

    static v8::CFunction fast_draw_image_dx_dy_;

    static v8::CFunction fast_draw_image_dx_dy_dw_dh_;

    static v8::CFunction fast_draw_image_;

    static v8::CFunction fast_close_path_;
    static v8::CFunction fast_begin_path_;

    static v8::CFunction fast_arc_;
    static v8::CFunction fast_arc_to_;
    static v8::CFunction fast_clear_rect_;

    static v8::CFunction fast_fill_;
    static v8::CFunction fast_fill_one_path_;

    static v8::CFunction fast_fill_rect_;

    static v8::CFunction fast_stroke_;
    static v8::CFunction fast_stroke_path_;

    static v8::CFunction fast_stroke_rect_;

    static v8::CFunction fast_rotate_;

    static v8::CFunction fast_restore_;

    static v8::CFunction fast_save_;

    static v8::CFunction fast_translate_;


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

    static void DrawImage(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    DrawImageDxDyAssetImpl(CanvasRenderingContext2DImpl *ptr, ImageAsset *asset, double dx,
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

    static void DrawImageDxDyDwDhAssetImpl(CanvasRenderingContext2DImpl *ptr, ImageAsset *asset,
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
                       ImageAsset *asset, double sx,
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

    static void Fill(const v8::FunctionCallbackInfo<v8::Value> &args);

    /* todo when fast string is supported
    static void FastFillTwo(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> path, string) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto object = Path2D::GetPointer(path);

        if (object != nullptr) {
            auto data = ConvertFromV8String(isolate, args[1]);
            canvas_native_context_fill_with_path(
                    ptr->GetContext(),
                    object->GetPath(),
                    data.c_str());
            ptr->UpdateInvalidateState();
        }
    }

    */

    static void FastFillOnePath(v8::Local<v8::Object> receiver_obj, v8::Local<v8::Object> path) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        auto object = Path2D::GetPointer(path);

        std::string rule("nonzero");
        canvas_native_context_fill_with_path(
                ptr->GetContext(),
                object->GetPath(), rule.c_str());
        ptr->UpdateInvalidateState();
    }

    // todo when fast string is supported
    /*
    static void FastFillOneString(v8::Local<v8::Object> receiver_obj) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(args.This());
        if (ptr == nullptr) {
            return;
        }
        auto isolate = args.GetIsolate();

        auto count = args.Length();
        auto value = args[0];
        if (count == 2) {
            auto type = GetNativeType( value.As<v8::Object>());
            if (type == NativeType::Path2D) {
                auto object = Path2D::GetPointer(value.As<v8::Object>());

                if (object != nullptr) {
                    auto data = ConvertFromV8String(isolate, args[1]);
                    canvas_native_context_fill_with_path(
                            ptr->GetContext(),
                            object->GetPath(),
                            data.c_str());
                    ptr->UpdateInvalidateState();
                }
            }

        } else if (count == 1) {
            if (value->IsString()) {
                auto rule = ConvertFromV8String(isolate, value);
                canvas_native_context_fill(
                        ptr->GetContext(), rule.c_str());
                ptr->UpdateInvalidateState();
            } else if (value->IsObject()) {
                auto type = GetNativeType( value.As<v8::Object>());
                if (type == NativeType::Path2D) {
                    auto object = Path2D::GetPointer(value.As<v8::Object>());

                    std::string rule("nonzero");
                    canvas_native_context_fill_with_path(
                            ptr->GetContext(),
                            object->GetPath(), rule.c_str());
                    ptr->UpdateInvalidateState();
                }
            }
        } else {
            std::string rule("nonzero");
            canvas_native_context_fill(
                    ptr->GetContext(), rule.c_str());
            ptr->UpdateInvalidateState();
        }
    }
*/
    static void FastFill(v8::Local<v8::Object> receiver_obj) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
        if (ptr == nullptr) {
            return;
        }

        std::string rule("nonzero");
        canvas_native_context_fill(
                ptr->GetContext(), rule.c_str());
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

    static void GetImageData(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetLineDash(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsPointInPath(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsPointInStroke(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void LineTo(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void MeasureText(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void MoveTo(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void PutImageData(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void QuadraticCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RoundRect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Rect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RemoveHitRegion(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ResetTransform(const v8::FunctionCallbackInfo<v8::Value> &args);

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

    static void FastRotate(v8::Local<v8::Object> receiver_obj, double angle) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
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

    static void ScrollPathIntoView(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetLineDash(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetTransform(const v8::FunctionCallbackInfo<v8::Value> &args);

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

        canvas_native_context_stroke_rect(
                ptr->GetContext(), static_cast<float>(x),
                static_cast<float>(y),
                static_cast<float>(width),
                static_cast<float>(height));
        ptr->UpdateInvalidateState();
    }


    static void StrokeText(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Transform(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Translate(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FastTranslate(v8::Local<v8::Object> receiver_obj, double x, double y) {
        CanvasRenderingContext2DImpl *ptr = GetPointer(receiver_obj);
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

};
