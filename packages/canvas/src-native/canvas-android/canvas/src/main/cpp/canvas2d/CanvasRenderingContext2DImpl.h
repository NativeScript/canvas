//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once
#pragma process_pending_includes

#include "v8runtime/V8Runtime.h"
#include "rust/cxx.h"
#include <vector>

#include "canvas-cxx/src/canvas2d.rs.h"
#include "canvas-cxx/src/webgl.rs.h"
#include "../Helpers.h"

#include "CanvasGradient.h"
#include "CanvasPattern.h"
#include "ImageDataImpl.h"
#include "TextMetricsImpl.h"
#include "ImageAssetImpl.h"
#include "ImageBitmapImpl.h"
#include "Path2D.h"
#include "OnRafCallback.h"
#include "RafImpl.h"
#include "webgl/WebGLRenderingContextBase.h"


using namespace facebook;

class JSI_EXPORT CanvasRenderingContext2DImpl : public facebook::jsi::HostObject {
public:
    CanvasRenderingContext2DImpl(rust::Box<CanvasRenderingContext2D> context);

    ~CanvasRenderingContext2DImpl();

    jsi::Value get(jsi::Runtime &, const jsi::PropNameID &name) override;

    void set(jsi::Runtime &, const jsi::PropNameID &name, const jsi::Value &value) override;

    std::vector<jsi::PropNameID> getPropertyNames(jsi::Runtime &rt) override;

    void UpdateInvalidateState();

    InvalidateState GetInvalidateState() const;

    void SetInvalidateState(InvalidateState state);

    void Flush();

    static void Flush(intptr_t context);

    CanvasRenderingContext2D &GetContext();

private:
    rust::Box<CanvasRenderingContext2D> context_;

    InvalidateState invalidateState_ = InvalidateState::NONE;

    std::shared_ptr<RafImpl> raf_;

};
