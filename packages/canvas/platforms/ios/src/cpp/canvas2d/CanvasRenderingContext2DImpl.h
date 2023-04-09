//
// Created by Osei Fortune on 22/03/2022.
//

#pragma once
#pragma process_pending_includes

#import <NativeScript/JSIRuntime.h>
#include "rust/cxx.h"
#include <vector>
#include <cstdint>
#include <memory>

#include "RafImpl.h"
#include "OnRafCallback.h"

#include "canvas-cxx/src/lib.rs.h"
#include "Helpers.h"

#include "CanvasGradient.h"
#include "CanvasPattern.h"
#include "ImageDataImpl.h"
#include "TextMetricsImpl.h"
#include "ImageAssetImpl.h"
#include "ImageBitmapImpl.h"
#include "Path2D.h"
#include "webgl/WebGLRenderingContextBase.h"


using namespace facebook;
using namespace org::nativescript::canvas;

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

    void SetInvalidateState(int state);

    void Flush();

    static void Flush(intptr_t context);

    void SetRaf(std::shared_ptr<RafImpl> raf);

    RafImpl *GetRaf();

    CanvasRenderingContext2D &GetContext();
    
    void StartRaf();
    
    void StopRaf();
    

private:
    rust::Box<CanvasRenderingContext2D> context_;

    int invalidateState_ = static_cast<int>(InvalidateState::NONE);

    std::shared_ptr<RafImpl> raf_;

};
