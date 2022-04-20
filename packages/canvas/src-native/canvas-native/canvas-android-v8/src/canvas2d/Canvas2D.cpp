//
// Created by Osei Fortune on 22/03/2022.
//

#include "Canvas2D.h"

void Canvas2D::Init(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    cache->SetContext(isolate->GetCurrentContext());
    CanvasGradient::Init(isolate);
    CanvasPattern::Init(isolate);
    ImageDataImpl::Init(isolate);
    Path2D::Init(isolate);
    TextMetricsImpl::Init(isolate);
    MatrixImpl::Init(isolate);
    CanvasRenderingContext2DImpl::Init(isolate);
}
