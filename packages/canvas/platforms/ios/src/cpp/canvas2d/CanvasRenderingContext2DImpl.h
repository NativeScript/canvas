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

class CanvasRenderingContext2DImpl {
public:
    CanvasRenderingContext2DImpl(CanvasRenderingContext2D* context);
    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static CanvasRenderingContext2DImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, CanvasRenderingContext2DImpl *renderingContext) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = CanvasRenderingContext2DImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::CanvasRenderingContext2D);
        auto ext = v8::External::New(isolate, renderingContext);
        object->SetInternalField(0, ext);
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

    static void ArcTo(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BeginPath(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void BezierCurveTo(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ClearHitRegions(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ClearRect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Clip(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ClosePath(const v8::FunctionCallbackInfo<v8::Value> &args);

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

    static void Ellipse(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Fill(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FillRect(const v8::FunctionCallbackInfo<v8::Value> &args);

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

    static void Rotate(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Save(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Scale(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ScrollPathIntoView(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetLineDash(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetTransform(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Stroke(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void StrokeRect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void StrokeText(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Transform(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Translate(const v8::FunctionCallbackInfo<v8::Value> &args);

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

    CanvasRenderingContext2D* GetContext();

private:
    CanvasRenderingContext2D* context_;

    int invalidateState_ = static_cast<int>(InvalidateState::InvalidateStateNone);

    std::shared_ptr<RafImpl> raf_;

};
